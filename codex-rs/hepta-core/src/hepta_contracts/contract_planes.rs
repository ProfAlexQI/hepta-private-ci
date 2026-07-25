pub fn node_device_pairing_qr_contract_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane(
        "node-device-pairing-qr-contract-plane",
        "Node / devices / pairing / QR contract plane",
        sample_run,
        vec![
            row(
                "node",
                "headless-node-service",
                "service lifecycle is metadata-only",
                sample_run,
            ),
            row(
                "nodes",
                "gateway-owned-node-actions",
                "camera/screen/location/notify actions require explicit live adapter confirmation",
                sample_run,
            ),
            row(
                "devices",
                "device-pairing-token-inventory",
                "token inventory is redacted and local-only",
                sample_run,
            ),
            row(
                "pairing",
                "secure-dm-pairing",
                "approve/reject mutations are not performed in report mode",
                sample_run,
            ),
            row(
                "qr",
                "mobile-setup-qr",
                "QR payload shape is classified without token materialization",
                sample_run,
            ),
        ],
        &[
            ("device_action_performed", false),
            ("camera_capture_performed", false),
            ("screen_capture_performed", false),
            ("location_read_performed", false),
            ("notification_sent", false),
            ("pairing_state_mutated", false),
            ("qr_token_materialized", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_send", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn config_update_security_secrets_lifecycle_dry_run_map(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane(
        "config-update-security-secrets-lifecycle-dry-run-map",
        "Config / update / security / secrets lifecycle dry-run map",
        sample_run,
        vec![
            row(
                "config",
                "validate/get/set/unset/file",
                "write requires explicit config patch/apply flow",
                sample_run,
            ),
            row(
                "configure",
                "interactive-credential-onboarding",
                "plan-only; no credential prompt in report mode",
                sample_run,
            ),
            row(
                "setup",
                "workspace-initialization",
                "no directories or files created in report mode",
                sample_run,
            ),
            row(
                "onboard",
                "gateway-workspace-skills-onboarding",
                "dry-run plan only",
                sample_run,
            ),
            row(
                "update",
                "self-update",
                "update.run remains blocked unless explicitly requested",
                sample_run,
            ),
            row(
                "security",
                "local-config-audit",
                "audit shape only; no privileged action",
                sample_run,
            ),
            row(
                "secrets",
                "runtime-secret-reload",
                "secret values are never read",
                sample_run,
            ),
            row(
                "doctor",
                "health-check-and-repair",
                "repair preview only; no package manager",
                sample_run,
            ),
            row(
                "exec-policy",
                "approval-policy-sync",
                "approval sync not performed in report mode",
                sample_run,
            ),
            row(
                "approvals",
                "approval-state-inspection",
                "approval mutation not performed",
                sample_run,
            ),
            row(
                "reset",
                "local-config-state-reset",
                "destructive reset intentionally report-only",
                sample_run,
            ),
            row(
                "crestodian",
                "ring-zero-setup-repair",
                "operator-only actions represented as blocked dry-run",
                sample_run,
            ),
        ],
        &[
            ("config_written", false),
            ("state_reset_performed", false),
            ("package_manager_invoked", false),
            ("hepta_update_invoked", false),
            ("secret_file_read", false),
            ("credential_value_read", false),
            ("approval_state_mutated", false),
            ("privileged_repair_performed", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn channel_message_directory_webhook_exact_parity_map(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane(
        "channel-message-directory-webhook-exact-parity-map",
        "Channels / message / directory / webhooks exact parity map",
        sample_run,
        vec![
            row(
                "channels",
                "login/status/list/logout",
                "channel auth and live login are out-of-band",
                sample_run,
            ),
            row(
                "message",
                "send/read/react/pin/thread/poll",
                "external send requires explicit confirmation",
                sample_run,
            ),
            row(
                "directory",
                "self/peer/group lookup",
                "metadata lookup redacts raw provider ids",
                sample_run,
            ),
            row(
                "webhooks",
                "register/list/test payload",
                "webhook ack/send not performed",
                sample_run,
            ),
            row(
                "status",
                "channel health/recent recipients",
                "status is metadata-only",
                sample_run,
            ),
            row(
                "telegram",
                "topic/progress/reply routing",
                "covered by adapter-owned route parser",
                sample_run,
            ),
            row(
                "discord",
                "thread/heartbeat/steer mention gates",
                "auth/mention gate represented as contract",
                sample_run,
            ),
            row(
                "feishu",
                "chat/topic hydration",
                "topic starter shape only",
                sample_run,
            ),
            row(
                "imessage",
                "chat db/local-probe/live-send",
                "live-send blocked without confirm",
                sample_run,
            ),
            row(
                "slack-matrix-msteams-line-whatsapp",
                "threaded route shapes",
                "raw target ids not logged",
                sample_run,
            ),
        ],
        &[
            ("channel_send_performed", false),
            ("message_provider_api_called", false),
            ("webhook_registered", false),
            ("webhook_ack_performed", false),
            ("directory_live_lookup_performed", false),
            ("raw_channel_id_logged", false),
            ("raw_chat_id_logged", false),
            ("raw_message_id_logged", false),
            ("credential_value_read", false),
            ("external_send", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn acp_agent_sandbox_infer_bridge_matrix(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane(
        "acp-agent-sandbox-infer-bridge-matrix",
        "ACP / agent / sandbox / infer runtime bridge matrix",
        sample_run,
        vec![
            row(
                "acp",
                "ACP harness routing",
                "session spawn/telephone-game flow remains explicit",
                sample_run,
            ),
            row(
                "agent",
                "single agent turn via gateway",
                "provider call not performed in report mode",
                sample_run,
            ),
            row(
                "agents",
                "isolated agents/workspaces/auth/routing",
                "workspace mutation not performed",
                sample_run,
            ),
            row(
                "capability",
                "provider-backed inference alias",
                "provider selection shape only",
                sample_run,
            ),
            row(
                "infer",
                "provider-backed inference",
                "no model request sent",
                sample_run,
            ),
            row(
                "sandbox",
                "containerized execution isolation",
                "container not started",
                sample_run,
            ),
            row(
                "exec-policy",
                "host approvals sync",
                "approval sync blocked in report mode",
                sample_run,
            ),
        ],
        &[
            ("acp_session_started", false),
            ("agent_turn_started", false),
            ("provider_call_performed", false),
            ("sandbox_container_started", false),
            ("approval_policy_mutated", false),
            ("workspace_mutated", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn operational_utility_contract_map(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane(
        "operational-utility-contract-map",
        "Backup / docs / logs / migrate / skills / system / tasks operational utility map",
        sample_run,
        vec![
            row(
                "backup",
                "archive-create/verify",
                "archive write not performed",
                sample_run,
            ),
            row(
                "completion",
                "shell completion generation",
                "stdout contract only",
                sample_run,
            ),
            row(
                "dns",
                "tailscale/coredns helpers",
                "network config not changed",
                sample_run,
            ),
            row(
                "docs",
                "docs search/open",
                "no live docs fetch in report mode",
                sample_run,
            ),
            row(
                "logs",
                "gateway log tail",
                "log file not tailed",
                sample_run,
            ),
            row(
                "mcp",
                "MCP config/channel bridge",
                "MCP server not started",
                sample_run,
            ),
            row(
                "migrate",
                "foreign agent state import",
                "source state not read",
                sample_run,
            ),
            row(
                "proxy",
                "debug proxy capture",
                "listener not started",
                sample_run,
            ),
            row("skills", "skill list/inspect", "metadata-only", sample_run),
            row(
                "system",
                "system events/heartbeat/presence",
                "event not emitted",
                sample_run,
            ),
            row(
                "tasks",
                "durable task state",
                "task mutation not performed",
                sample_run,
            ),
            row(
                "terminal",
                "local terminal UI alias",
                "TUI not launched",
                sample_run,
            ),
            row(
                "tui",
                "gateway/local terminal UI",
                "TUI not launched",
                sample_run,
            ),
            row(
                "uninstall",
                "service/data uninstall",
                "intentionally unsupported destructive action",
                sample_run,
            ),
            row(
                "gateway",
                "gateway runtime control",
                "gateway RPC not performed",
                sample_run,
            ),
            row(
                "health",
                "gateway health fetch",
                "gateway RPC not performed",
                sample_run,
            ),
            row(
                "dashboard",
                "Control UI open",
                "browser not launched",
                sample_run,
            ),
            row(
                "hooks",
                "internal hook management",
                "hook queue not mutated",
                sample_run,
            ),
            row(
                "commitments",
                "follow-up commitments",
                "commitment state not mutated",
                sample_run,
            ),
            row(
                "clawbot",
                "legacy aliases",
                "legacy compatibility documented only",
                sample_run,
            ),
            row(
                "daemon",
                "legacy gateway alias",
                "service lifecycle not invoked",
                sample_run,
            ),
            row("chat", "TUI local alias", "TUI not launched", sample_run),
        ],
        &[
            ("archive_written", false),
            ("listener_started", false),
            ("browser_opened", false),
            ("gateway_rpc_performed", false),
            ("mcp_server_started", false),
            ("source_state_imported", false),
            ("task_state_mutated", false),
            ("hook_queue_mutated", false),
            ("destructive_uninstall_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn vendored_hepta_sidecar_runtime_rpc_contract(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane(
        "vendored-hepta-js-sidecar-runtime-rpc-contract",
        "Vendored Hepta JS plugin sidecar runtime RPC contract",
        sample_run,
        vec![
            row(
                "sidecar-inventory",
                "plugin inventory RPC",
                "metadata-only inventory; no plugin import or runtime start",
                sample_run,
            ),
            row(
                "sidecar-receive",
                "channel receive RPC",
                "receive shape gated by explicit token source and redacted ledger",
                sample_run,
            ),
            row(
                "sidecar-send",
                "channel send RPC",
                "send remains confirmation-gated and dry-run by default",
                sample_run,
            ),
            row(
                "sidecar-tool",
                "tool invocation RPC",
                "tool calls require Hepta policy/approval envelope",
                sample_run,
            ),
            row(
                "sidecar-provider",
                "provider command RPC",
                "provider calls remain disabled in report mode",
                sample_run,
            ),
            row(
                "sidecar-ledger",
                "redacted runtime ledger",
                "Hepta owns redaction, provenance, and side-effect ledger",
                sample_run,
            ),
        ],
        &[
            ("sidecar_process_started", false),
            ("plugin_import_attempted", false),
            ("plugin_runtime_started", false),
            ("channel_receive_performed", false),
            ("channel_send_performed", false),
            ("tool_invoked", false),
            ("provider_call_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn hepta_2026_5_6_hardening_regressions(sample_run: bool) -> HeptaContractPlaneReport {
    let doctor = DoctorOpenAiRouteNoRewriteGuardReport::synthetic_noop();
    let guarded_fetch = sanitize_guarded_fetch_headers(&synthetic_metadata_headers());
    let debug_proxy = normalize_debug_proxy_replay_headers(&synthetic_metadata_headers());
    let timeout_cleanup = simulate_guarded_dispatcher_timeout_lane_cleanup(1);

    contract_plane(
        "hepta-2026-5-6-hardening-regressions",
        "Hepta 2026.5.6 doctor/fetch/proxy/timeout hardening regressions",
        sample_run,
        vec![
            row(
                "doctor",
                "doctor-openai-route-no-rewrite-guard",
                "doctor repair cannot silently rewrite existing OpenAI-compatible routes",
                sample_run,
            ),
            row(
                "plugins-runtime-fetch",
                "guarded-fetch-header-symbol-scrubber",
                "symbol/non-string/prototype metadata is dropped before native Headers construction",
                sample_run,
            ),
            row(
                "proxy",
                "debug-proxy-replay-header-normalization",
                "captured headers are normalized before replay envelopes are built",
                sample_run,
            ),
            row(
                "web-fetch",
                "guarded-dispatcher-timeout-lane-cleanup",
                "timed-out guarded dispatch returns structured error and releases active lane",
                sample_run,
            ),
        ],
        &[
            ("openai_route_rewritten", doctor.route_rewritten),
            ("doctor_fix_executed", doctor.doctor_fix_executed),
            ("doctor_secret_value_read", doctor.secret_value_read),
            (
                "header_symbol_metadata_forwarded",
                guarded_fetch.symbol_metadata_forwarded,
            ),
            ("native_headers_safe", guarded_fetch.native_headers_safe),
            (
                "debug_proxy_replay_network_performed",
                debug_proxy.replay_network_performed,
            ),
            (
                "debug_proxy_metadata_forwarded",
                debug_proxy.captured_metadata_forwarded,
            ),
            ("dispatcher_lane_leaked", timeout_cleanup.lane_leaked),
            ("timeout_cleanup_bounded", timeout_cleanup.cleanup_bounded),
            ("provider_call_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn hepta_2026_5_7_delta_regressions(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-2026-5-7-delta-regressions",
        "Hepta 2026.5.7 exact delta regressions for owner/authz, context, delivery, cron, sessions, plugins, providers, and channel edges",
        sample_run,
        vec![
            row(
                "native-commands",
                "owner-enforced-native-command-handlers",
                "native command handlers check owner scope before any handler side effect",
                sample_run,
            ),
            row(
                "auto-reply/skills",
                "before-tool-call-authorization-for-inline-skill-dispatch",
                "auto-reply inline skill dispatch is gated by before-tool-call authorization hooks",
                sample_run,
            ),
            row(
                "agents/context",
                "assembled-context-cache-invalidation-on-shrink-or-failure",
                "assembled context views are invalidated when source history shrinks or assembly fails",
                sample_run,
            ),
            row(
                "agents/compaction",
                "summary-reserve-clamped-to-model-output-limit",
                "compaction summary reserve tokens are clamped to the active model output limit",
                sample_run,
            ),
            row(
                "agent-delivery",
                "empty-adapter-result-marks-delivery-failed",
                "empty outbound adapter results produce deliverySucceeded=false instead of false success",
                sample_run,
            ),
            row(
                "cron/isolated",
                "delivery-last-preflight-before-model-execution",
                "delivery.channel=last without a previous route fails before model execution",
                sample_run,
            ),
            row(
                "gateway/sessions",
                "daily-rollover-generated-transcript-persistence",
                "daily gateway-agent session rollover creates a generated transcript while preserving custom transcript paths",
                sample_run,
            ),
            row(
                "plugins/install",
                "absolute-posix-npm-lifecycle-shell-consistency",
                "managed plugin install, rollback, repair, and uninstall use the same absolute POSIX npm lifecycle shell",
                sample_run,
            ),
            row(
                "plugins/channel-setup",
                "external-set-channel-runtime-forwarding",
                "non-bundled external plugin setup forwards setChannelRuntime before startup polling",
                sample_run,
            ),
            row(
                "providers/channels",
                "provider-channel-edge-normalization-pack",
                "APNG, Gemini 3 signatures, __env__ keys, snake_case transcripts, WhatsApp LID, captioned MEDIA, and Discord voice capability edges are normalized without live calls",
                sample_run,
            ),
        ],
        &[
            ("owner_enforcement_bypassed", false),
            ("before_tool_call_hook_skipped", false),
            ("stale_context_view_reused", false),
            ("compaction_max_tokens_invalid", false),
            ("delivery_success_claimed_on_empty_adapter_result", false),
            ("cron_model_executed_before_delivery_preflight", false),
            ("rollover_transcript_missing", false),
            ("npm_lifecycle_shell_inconsistent", false),
            ("external_set_channel_runtime_dropped", false),
            ("provider_channel_edge_regression_missing", false),
            ("provider_call_performed", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("external_send", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_2026_5_7_delta_checks(sample_run),
    )
}

pub fn hepta_2026_5_7_polish_regressions(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-2026-5-7-polish-regressions",
        "Hepta 2026.5.7 polish regressions for release publishing, sanitizers, cron repair, Telegram auth/callbacks, subagent TTL, and Discord voice config",
        sample_run,
        vec![
            row(
                "release/clawhub",
                "clawhub-publish-retry-and-version-verification",
                "ClawHub publish maintenance retries transient dependency installs, tolerates isolated preview-cell flakes, and verifies expected package versions after publish without publishing by default",
                sample_run,
            ),
            row(
                "commands/btw",
                "btw-placeholder-bracket-preserving-sanitizer",
                "/btw missing-question usage placeholder keeps bracketed text after outbound sanitization",
                sample_run,
            ),
            row(
                "cron/doctor",
                "payload-model-exact-repair-fixture",
                "doctor repair removes persisted cron payload.model values stored as default/null/blank while keeping runtime model validation strict",
                sample_run,
            ),
            row(
                "telegram/authz",
                "accessgroup-before-numeric-sender-id-fixture",
                "Telegram DMs, groups, native commands, and callbacks honor accessGroup allowlists before numeric sender-id fallback",
                sample_run,
            ),
            row(
                "agents/subagents",
                "archive-after-minutes-exact-ttl-fixture",
                "completed session-mode subagent registry rows use agents.defaults.subagents.archiveAfterMinutes rather than a hardcoded 5-minute TTL",
                sample_run,
            ),
            row(
                "discord/voice",
                "voice-capture-silence-grace-config-fixture",
                "Discord voice capture defaults post-speech silence grace to 2.5s and parses bounded voice.captureSilenceGraceMs overrides",
                sample_run,
            ),
            row(
                "telegram/models",
                "dotted-provider-id-callback-parser-fixture",
                "Telegram /models inline keyboard callbacks preserve dotted provider ids such as hf.co",
                sample_run,
            ),
            row(
                "release/plugins",
                "redacted-publishing-evidence-ledger-shape",
                "release/plugin publishing stores redacted retry and version-check evidence without registry credentials, prompts, or network writes in report mode",
                sample_run,
            ),
        ],
        &[
            ("clawhub_publish_performed", false),
            ("btw_placeholder_brackets_stripped", false),
            ("cron_payload_model_bad_override_preserved", false),
            ("telegram_accessgroup_bypassed", false),
            ("subagent_ttl_hardcoded_five_minutes", false),
            ("discord_voice_silence_default_missing", false),
            ("telegram_dotted_provider_id_broken", false),
            ("release_evidence_contains_secret", false),
            ("registry_credential_value_read", false),
            ("provider_call_performed", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("external_send", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_2026_5_7_polish_checks(sample_run),
    )
}

pub fn hepta_unreleased_channel_streaming_delivery_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-channel-streaming-delivery-regressions",
        "Hepta upstream-unreleased channel streaming and delivery edge regressions",
        sample_run,
        vec![
            row(
                "channels/streaming",
                "progress-draft-label-scroll-contract",
                "progress draft labels scroll with progress lines and stale pre-tool labels are not reused",
                sample_run,
            ),
            row(
                "channels/streaming",
                "compact-structured-tool-row-rendering",
                "structured tool rows render compact emoji/title/details without raw tool payloads",
                sample_run,
            ),
            row(
                "tools/web-search",
                "native-web-search-query-rendering",
                "provider-native web-search argument shapes render as redacted structured query rows",
                sample_run,
            ),
            row(
                "discord/streaming",
                "discord-apply-patch-empty-start-suppression",
                "empty apply-patch starts are suppressed until a patch summary exists",
                sample_run,
            ),
            row(
                "telegram/poll",
                "telegram-poll-option-cap-preflight",
                "Telegram poll fixtures over the 10-option channel cap are rejected before send",
                sample_run,
            ),
            row(
                "telegram/delivery",
                "telegram-same-chat-success-suppresses-fallback",
                "successful same-chat message tool sends suppress the rewritten silent fallback",
                sample_run,
            ),
            row(
                "telegram/topics",
                "telegram-numeric-forum-topic-plugin-owned",
                "numeric forum-topic targets are parsed as plugin-owned topic routes, not legacy raw ids",
                sample_run,
            ),
            row(
                "telegram/streaming",
                "telegram-stable-runtime-alias-chunking",
                "reply-dispatch chunks keep stable runtime aliases during updates",
                sample_run,
            ),
            row(
                "discord/streaming",
                "discord-progress-draft-preview-default",
                "Discord progress draft previews are enabled by default and respect explicit disable",
                sample_run,
            ),
            row(
                "telegram/streaming",
                "telegram-draft-preview-rotation-after-output",
                "Telegram draft preview rotation invalidates stale pre-tool previews after tool/media output",
                sample_run,
            ),
            row(
                "whatsapp/newsletter",
                "whatsapp-channel-newsletter-targets",
                "WhatsApp @newsletter targets route as channel/newsletter targets instead of DMs",
                sample_run,
            ),
            row(
                "slack/streaming",
                "slack-rich-progress-draft-trimming",
                "Slack rich progress drafts preserve shape while trimming newest progress lines",
                sample_run,
            ),
            row(
                "discord/message",
                "discord-provider-prefixed-channel-route",
                "provider-prefixed discord:channel targets do not misroute into legacy DMs",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("channel_send_performed", false),
            ("provider_call_performed", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("external_send", false),
            ("credential_value_read", false),
            ("private_config_read", false),
            ("private_chat_history_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
            ("raw_channel_id_logged", false),
            ("raw_message_id_logged", false),
            ("raw_tool_payload_logged", false),
        ],
        synthetic_hepta_unreleased_channel_streaming_delivery_checks(sample_run),
    )
}

pub fn hepta_unreleased_codex_acp_approval_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-codex-acp-approval-regressions",
        "Hepta upstream-unreleased Codex, ACP, and approval lifecycle regressions",
        sample_run,
        vec![
            row(
                "codex/app-server",
                "codex-harness-version-and-dynamic-tools",
                "managed Codex harness is pinned and dynamic tools defer behind tool search by default",
                sample_run,
            ),
            row(
                "codex/app-server",
                "codex-post-tool-watchdog-idle-contract",
                "post-tool completion watchdog disarms after current-turn activity and exposes idle timeout diagnostics",
                sample_run,
            ),
            row(
                "codex/approvals",
                "codex-native-permissionrequest-policy",
                "Codex reviewer handles safe native PermissionRequest payloads before Hepta surfaces approval",
                sample_run,
            ),
            row(
                "codex/approvals",
                "codex-allow-always-active-session-scope",
                "allow-always decisions are bounded to identical payloads in the active session window",
                sample_run,
            ),
            row(
                "codex/plugins",
                "codex-plugin-approval-action-shape",
                "plugin approval requests render only the actual allowed decisions",
                sample_run,
            ),
            row(
                "codex/plugins",
                "openai-curated-plugin-thread-contract",
                "source-installed openai-curated Codex plugins share the harness thread with cached readiness",
                sample_run,
            ),
            row(
                "codex/plugins",
                "codex-plugin-destructive-policy-delegation",
                "destructive policy is delegated to Codex app-level destructive_enabled config",
                sample_run,
            ),
            row(
                "acpx/codex",
                "trusted-project-declaration-preservation",
                "isolated Codex ACP launches preserve trusted project declarations",
                sample_run,
            ),
            row(
                "acpx/codex",
                "stale-acpx-process-tree-reaping",
                "Hepta-owned stale ACPX/Codex process trees are reaped on startup/session close",
                sample_run,
            ),
            row(
                "acp/bridge",
                "stable-session-list-resume-close-handlers",
                "ACP bridge exposes stable session list, resume, and close handlers",
                sample_run,
            ),
            row(
                "acp/sessions",
                "parent-owned-cross-agent-visibility",
                "parent agents may inspect/message only their own spawned cross-agent ACP sessions",
                sample_run,
            ),
            row(
                "openai/codex-media",
                "codex-audio-transcription-routing",
                "Codex audio transcription metadata routes active chat models to transcription defaults",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("codex_prompt_uploaded", false),
            ("acp_process_spawned", false),
            ("approval_decision_persisted", false),
            ("provider_call_performed", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_codex_acp_approval_checks(sample_run),
    )
}

pub fn hepta_unreleased_talk_voice_controller_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-talk-voice-controller-regressions",
        "Hepta upstream-unreleased Talk, voice, realtime, and telephony controller regressions",
        sample_run,
        vec![
            row(
                "talk/session",
                "shared-talk-session-controller-rpc",
                "Talk, transcription relay, managed rooms, Voice Call, Google Meet, VoiceClaw, and native clients share talk.session RPC shape",
                sample_run,
            ),
            row(
                "diagnostics/talk",
                "bounded-talk-lifecycle-audio-metrics",
                "Talk lifecycle/audio metrics export bounded redacted OTLP/Prometheus shapes",
                sample_run,
            ),
            row(
                "logging/talk",
                "redacted-talk-lifecycle-logs",
                "Talk lifecycle logs exclude transcripts, audio payloads, turn ids, call ids, and provider item ids",
                sample_run,
            ),
            row(
                "openai/realtime",
                "ga-realtime-default-voice-shape",
                "OpenAI realtime defaults to gpt-realtime-2 and GA WebSocket session shape",
                sample_run,
            ),
            row(
                "google-meet/voice-call",
                "realtime-gemini-bridge-pacing",
                "Meet/Voice Call realtime Gemini bridge uses paced audio and backpressure-aware buffering",
                sample_run,
            ),
            row(
                "voice-call/realtime",
                "voice-context-capsule-cadence",
                "agent voice context capsules and consult-cadence guidance are opt-in and bounded",
                sample_run,
            ),
            row(
                "tts/telephony",
                "telephony-provider-voice-model-overrides",
                "telephony synthesis logs honor provider voice/model overrides",
                sample_run,
            ),
            row(
                "discord/voice",
                "discord-voice-stt-preview-verbose-log",
                "Discord verbose voice logs include a bounded one-line STT preview",
                sample_run,
            ),
            row(
                "discord/voice",
                "elevenlabs-direct-tts-playback",
                "ElevenLabs TTS streams directly into Discord playback with latency optimization",
                sample_run,
            ),
            row(
                "discord/voice",
                "tts-playback-capture-barge-in-guard",
                "TTS playback continues while new capture is ignored to avoid feedback loops",
                sample_run,
            ),
            row(
                "discord/voice",
                "voice-channel-permission-probe-shape",
                "channels capabilities/status probe audits Connect/Speak/Read Message History",
                sample_run,
            ),
            row(
                "google-meet",
                "silent-intro-empty-string-preservation",
                "realtime.introMessage empty string remains silent instead of restoring default intro",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("audio_payload_captured", false),
            ("transcript_text_logged", false),
            ("meet_joined", false),
            ("twilio_call_started", false),
            ("discord_voice_connected", false),
            ("provider_call_performed", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("credential_value_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_talk_voice_controller_checks(sample_run),
    )
}

pub fn hepta_unreleased_gateway_session_task_performance_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-gateway-session-task-performance-regressions",
        "Hepta upstream-unreleased gateway, sessions, tasks, startup, and performance regressions",
        sample_run,
        vec![
            row(
                "gateway/tasks",
                "stale-cli-run-context-reconciliation",
                "stale CLI run-context tasks reconcile when live context disappears even if child session remains",
                sample_run,
            ),
            row(
                "gateway/reload",
                "bounded-channel-hot-reload-deferral",
                "channel hot reloads apply bounded default reload deferral timeouts",
                sample_run,
            ),
            row(
                "gateway/sessions",
                "atomic-session-store-index-writes",
                "session-store index writes are atomic while durable fsync is skipped inside writer lock",
                sample_run,
            ),
            row(
                "sessions/cli",
                "qualified-model-ref-fast-path",
                "session-list rows fast-path already-qualified model refs",
                sample_run,
            ),
            row(
                "sessions/cli",
                "selected-agent-runtime-column",
                "sessions tables include selected agent runtime",
                sample_run,
            ),
            row(
                "gateway/startup",
                "startup-phase-span-diagnostics",
                "startup phase spans, active work labels, stale bridge markers, and sync-I/O traces are bounded",
                sample_run,
            ),
            row(
                "gateway/startup",
                "nonreadiness-sidecar-deferral",
                "non-readiness sidecars defer until after ready signal",
                sample_run,
            ),
            row(
                "gateway/performance",
                "plugin-metadata-snapshot-reuse",
                "compatible/current plugin metadata snapshots are reused during dashboard and channel turns",
                sample_run,
            ),
            row(
                "gateway/performance",
                "plugin-auto-enable-single-resolution",
                "plugin auto-enable metadata is not resolved twice in one runtime config pass",
                sample_run,
            ),
            row(
                "gateway/performance",
                "native-loadable-plugin-no-jiti-fast-path",
                "native-loadable plugins avoid jiti import unless fallback loading is required",
                sample_run,
            ),
            row(
                "plugins/loader",
                "compiled-plugin-error-preservation",
                "native fast path preserves real compiled plugin module evaluation errors",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("gateway_restart_performed", false),
            ("session_store_mutated", false),
            ("task_registry_mutated", false),
            ("plugin_runtime_started", false),
            ("provider_call_performed", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_gateway_session_task_performance_checks(sample_run),
    )
}

pub fn hepta_unreleased_plugin_install_sdk_fssafe_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-plugin-install-sdk-fssafe-regressions",
        "Hepta upstream-unreleased plugin install, SDK lifecycle, and fs-safe regressions",
        sample_run,
        vec![
            row(
                "plugins/install",
                "npm-pack-managed-install-path",
                "npm-pack:<path.tgz> installs use managed npm-root, install records, and dependency scanning",
                sample_run,
            ),
            row(
                "plugins/install",
                "local-pack-lockfile-verification",
                "local npm pack artifacts pass lockfile verification and dependency scan before install-record publication",
                sample_run,
            ),
            row(
                "channels/plugins",
                "official-external-channel-missing-plugin-status",
                "configured official external channels render missing-plugin status rows with exact repair commands",
                sample_run,
            ),
            row(
                "doctor/plugins",
                "plugin-owned-legacy-config-repair-order",
                "doctor discovers plugin-owned legacy repair contracts before validation",
                sample_run,
            ),
            row(
                "plugin-skills/windows",
                "plugin-skill-junction-registration",
                "plugin-provided skill dirs register as Windows junctions when symlink is unavailable",
                sample_run,
            ),
            row(
                "plugins/lifecycle",
                "absolute-posix-managed-npm-shell",
                "managed plugin npm operations use an absolute POSIX lifecycle shell",
                sample_run,
            ),
            row(
                "plugin-sdk/channel-message",
                "channel-message-sdk-lifecycle-helpers",
                "SDK exposes channel-message lifecycle helpers without requiring plugin runtime startup",
                sample_run,
            ),
            row(
                "plugin-sdk/fs-safe",
                "staged-external-output-writes",
                "browser/media/channel/QA outputs are staged through fs-safe writes before publication",
                sample_run,
            ),
            row(
                "plugin-sdk/temp-workspace",
                "temp-workspace-helper-rename",
                "public temp workspace helpers use tempWorkspace/withTempWorkspace naming",
                sample_run,
            ),
            row(
                "plugins/loader",
                "compiled-module-error-preservation",
                "compiled plugin loader preserves real module evaluation errors",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("plugin_installed", false),
            ("package_manager_invoked", false),
            ("plugin_runtime_started", false),
            ("external_output_published", false),
            ("filesystem_mutated", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_plugin_install_sdk_fssafe_checks(sample_run),
    )
}

pub fn synthetic_metadata_headers() -> Vec<FetchHeaderEntry> {
    vec![
        FetchHeaderEntry::plain("Content-Type", "application/json"),
        FetchHeaderEntry::plain("X-Trace-Id", "trace-redacted"),
        FetchHeaderEntry::metadata(
            "Symbol(nodejs.util.inspect.custom)",
            "metadata",
            HeaderEntryKind::SymbolMetadata,
        ),
        FetchHeaderEntry::metadata("__proto__", "poison", HeaderEntryKind::PrototypeMetadata),
        FetchHeaderEntry::metadata(
            "sdk-internal-retry-state",
            "opaque",
            HeaderEntryKind::NonStringMetadata,
        ),
    ]
}

fn valid_http_header_name(name: &str) -> bool {
    let trimmed = name.trim();
    !trimmed.is_empty()
        && trimmed.bytes().all(|byte| {
            matches!(
                byte,
                b'a'..=b'z'
                    | b'A'..=b'Z'
                    | b'0'..=b'9'
                    | b'!'
                    | b'#'
                    | b'$'
                    | b'%'
                    | b'&'
                    | b'\''
                    | b'*'
                    | b'+'
                    | b'-'
                    | b'.'
                    | b'^'
                    | b'_'
                    | b'`'
                    | b'|'
                    | b'~'
            )
        })
}

pub fn gateway_session_task_liveness_plane(sample_run: bool) -> HeptaContractPlaneReport {
    let stale_run_context = synthetic_stale_run_context_reconciliation_check(sample_run);
    let bounded_reload_deferral = synthetic_bounded_reload_deferral_check(sample_run);
    contract_plane_with_checks(
        "gateway-session-task-liveness-plane",
        "Gateway session/task liveness, starvation, and stale-run cleanup plane",
        sample_run,
        vec![
            row(
                "gateway/tasks",
                "stale-cli-run-context-reconciliation",
                "stale run-context tasks are reconciled even when child session rows remain",
                sample_run,
            ),
            row(
                "channels/reload",
                "bounded-channel-hot-reload-deferral",
                "channel hot reload cannot be blocked forever by stale task records",
                sample_run,
            ),
            row(
                "gateway/sessions",
                "atomic-session-index-write-no-fsync-lock",
                "session index writes are atomic and never hold writer locks across durable fsync",
                sample_run,
            ),
            row(
                "sessions",
                "qualified-model-ref-fast-path",
                "already-qualified model refs skip heavyweight model resolution in session rows",
                sample_run,
            ),
            row(
                "sessions-cli",
                "selected-agent-runtime-visible",
                "session summaries expose selected agent runtime without widening visibility",
                sample_run,
            ),
        ],
        &[
            ("stale_run_context_left_active", false),
            ("reload_deferral_unbounded", false),
            ("session_index_write_non_atomic", false),
            ("fsync_inside_writer_lock", false),
            ("model_ref_heavy_resolution_required", false),
            ("runtime_visibility_missing", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        vec![stale_run_context, bounded_reload_deferral],
    )
}

pub fn channel_delivery_streaming_parity_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "channel-delivery-streaming-parity-plane",
        "Channel delivery, streaming, target-routing, and progress parity plane",
        sample_run,
        vec![
            row(
                "telegram/message",
                "same-chat-outbound-delivery-suppresses-fallback",
                "successful same-chat sends count as delivered before silent fallback rewriting",
                sample_run,
            ),
            row(
                "telegram/forum-topic",
                "plugin-owned-numeric-topic-targets",
                "numeric forum-topic targets preserve plugin ownership and stable runtime aliases",
                sample_run,
            ),
            row(
                "telegram/streaming",
                "draft-preview-rotation-after-tool-output",
                "post-tool previews do not reuse stale pre-tool assistant drafts",
                sample_run,
            ),
            row(
                "discord/message",
                "provider-prefixed-channel-target-routing",
                "discord:channel:<id> parses as a channel send, never a misleading DM target",
                sample_run,
            ),
            row(
                "discord/thread-reply",
                "thread-attachment-path-preservation",
                "filePath/path attachments survive thread-reply routing envelopes",
                sample_run,
            ),
            row(
                "whatsapp/channel",
                "explicit-newsletter-target-metadata",
                "@newsletter outbound targets carry channel/newsletter session metadata instead of DM routing",
                sample_run,
            ),
            row(
                "slack/streaming",
                "rich-progress-line-trimming-and-caps",
                "rich progress drafts retain newest bounded lines and avoid jumpy reflow",
                sample_run,
            ),
        ],
        &[
            ("duplicate_silent_fallback_emitted", false),
            ("provider_prefixed_channel_misrouted_to_dm", false),
            ("thread_attachment_path_dropped", false),
            ("newsletter_target_routed_as_dm", false),
            ("stale_pre_tool_preview_reused", false),
            ("progress_draft_unbounded", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_send", false),
            ("side_effects_performed", false),
        ],
        synthetic_channel_target_parser_checks(sample_run),
    )
}

pub fn plugin_install_secret_contract_lifecycle_plane(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "plugin-install-secret-contract-lifecycle-plane",
        "Plugin install/update, SecretRef contract, and migration lifecycle plane",
        sample_run,
        vec![
            row(
                "plugins/install",
                "npm-pack-managed-install-provenance",
                "npm-pack:<path.tgz> uses managed npm-root, lockfile verification, dependency scan, and install-record ledger",
                sample_run,
            ),
            row(
                "secrets/external-channel-contracts",
                "dist-sidecar-secret-contract-lookup",
                "externalized channel secret-contract-api sidecars resolve from rootDir/dist as well as root",
                sample_run,
            ),
            row(
                "secrets/apply",
                "auth-profile-keyref-tokenref-preservation",
                "keyRef/tokenRef metadata survives plaintext secret scrubbing",
                sample_run,
            ),
            row(
                "plugins/update",
                "official-externalized-plugin-trust-marker",
                "official externalized bundled npm migrations and ClawHub fallbacks remain source-linked",
                sample_run,
            ),
            row(
                "plugins/clawhub",
                "rate-limit-reset-and-signin-hint",
                "429 errors expose redacted Retry-After/RateLimit-Reset guidance and sign-in hint",
                sample_run,
            ),
            row(
                "plugins/migration",
                "catalog-backed-install-hints",
                "valid official missing plugins produce install hints instead of invalid removal advice",
                sample_run,
            ),
            row(
                "plugins/loader",
                "compiled-plugin-error-preservation",
                "native compiled module evaluation errors are preserved instead of hidden as transform fallback misses",
                sample_run,
            ),
        ],
        &[
            ("npm_pack_installed", false),
            ("dependency_scan_skipped", false),
            ("secret_contract_dist_lookup_missing", false),
            ("plaintext_secret_preserved", false),
            ("keyref_tokenref_metadata_dropped", false),
            ("official_plugin_trust_marker_missing", false),
            ("compiled_plugin_error_erased", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
        synthetic_secretref_contract_checks(sample_run),
    )
}

pub fn acp_codex_approval_lifecycle_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "acp-codex-approval-lifecycle-plane",
        "ACP/Codex approval, watchdog, trust, process, and parent-visibility lifecycle plane",
        sample_run,
        vec![
            row(
                "codex/app-server",
                "post-tool-idle-watchdog-disarm",
                "post-tool completion watchdog disarms after current-turn activity and exposes idle timeout diagnostics",
                sample_run,
            ),
            row(
                "codex/approvals",
                "native-permission-hook-not-preinstalled",
                "Codex native PermissionRequest hook is not preinstalled before Codex reviewer can approve safe commands",
                sample_run,
            ),
            row(
                "codex/approvals",
                "allow-always-identical-payload-session-memory",
                "identical native allow-always decisions are remembered only within the active session window",
                sample_run,
            ),
            row(
                "plugin-approvals",
                "actual-allowed-decision-rendering",
                "plugin approval requests render only actual allowed decisions, preventing stale native UI actions",
                sample_run,
            ),
            row(
                "acpx/codex",
                "trusted-project-declaration-preservation",
                "isolated Codex ACP sessions preserve trusted project declarations without interactive prompts",
                sample_run,
            ),
            row(
                "acp-sessions",
                "owned-process-reap-and-parent-scoped-visibility",
                "stale Hepta-owned process trees are reaped while parent agents only inspect/message their own spawned ACP sessions",
                sample_run,
            ),
        ],
        &[
            ("post_tool_watchdog_left_armed", false),
            ("native_permission_hook_preinstalled", false),
            ("allow_always_scope_unbounded", false),
            ("stale_approval_action_rendered", false),
            ("trusted_project_prompt_required", false),
            ("stale_acp_process_left_running", false),
            ("broad_agent_visibility_enabled", false),
            ("credential_value_read", false),
            ("external_process_started", false),
            ("side_effects_performed", false),
        ],
        synthetic_acp_codex_approval_lifecycle_checks(sample_run),
    )
}

pub fn cli_status_auth_parity_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "cli-status-auth-parity-plane",
        "CLI/status/auth JSON shape parity plane",
        sample_run,
        vec![
            row(
                "channels/list",
                "channel-only-all-installed-configured-enabled-shape",
                "channels list is channel-only and exposes --all installed/configured/enabled/origin fields",
                sample_run,
            ),
            row(
                "cron/list-show",
                "computed-status-json-field",
                "cron list/show JSON includes computed disabled/running/ok/error/skipped/idle status",
                sample_run,
            ),
            row(
                "cron/list-agent",
                "agent-filter-normalization",
                "cron list --agent normalizes requested agent id while keeping unfiltered default behavior",
                sample_run,
            ),
            row(
                "models/auth",
                "redacted-auth-profile-list",
                "models auth list exposes profile metadata without dumping secret values",
                sample_run,
            ),
            row(
                "status",
                "gateway-and-host-uptime-fields",
                "status surfaces compact gateway process uptime and host uptime",
                sample_run,
            ),
            row(
                "channels/status",
                "degraded-transport-and-event-loop-starvation-signals",
                "status/deep status expose degraded Discord transport and event-loop starvation signals",
                sample_run,
            ),
            row(
                "models/openai",
                "moving-alias-and-transcription-route-contract",
                "openai/chat-latest stays explicit and Codex audio transcription routes to transcription defaults",
                sample_run,
            ),
        ],
        &[
            ("auth_provider_usage_in_channels_list", false),
            ("cron_computed_status_missing", false),
            ("cron_agent_filter_ignored", false),
            ("auth_secret_value_dumped", false),
            ("uptime_fields_missing", false),
            ("degraded_transport_hidden", false),
            ("moving_model_alias_made_default", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
        vec![synthetic_catalog_auth_redaction_check(sample_run)],
    )
}

pub fn gateway_plugin_startup_diagnostics_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "gateway-plugin-startup-diagnostics-plane",
        "Gateway/plugin startup performance, metadata cache, and diagnostics plane",
        sample_run,
        vec![
            row(
                "gateway/startup",
                "readiness-before-nonreadiness-sidecars",
                "non-readiness sidecars are deferred until after the gateway ready signal",
                sample_run,
            ),
            row(
                "plugins/startup",
                "native-loadable-no-jiti-fast-path",
                "native-loadable plugin startup avoids hot-path jiti/source-transform imports",
                sample_run,
            ),
            row(
                "plugins/metadata",
                "compatible-metadata-snapshot-reuse",
                "dashboard/channel turns reuse compatible plugin metadata snapshots",
                sample_run,
            ),
            row(
                "provider-activation",
                "root-scoped-auth-env-bundle-cache",
                "provider activation/auth/env/bundle metadata cache refuses stale unscoped roots",
                sample_run,
            ),
            row(
                "plugins/auto-enable",
                "single-pass-auto-enable-resolution",
                "plugin auto-enable metadata is not resolved twice in one runtime config pass",
                sample_run,
            ),
            row(
                "diagnostics/startup",
                "startup-phase-spans-active-work-and-sync-io-tracing",
                "startup phase spans, active work labels, stale bridge markers, and sync-I/O traces are bounded",
                sample_run,
            ),
        ],
        &[
            ("nonreadiness_sidecar_started_before_ready", false),
            ("hot_path_jiti_imported", false),
            ("metadata_snapshot_recomputed_per_turn", false),
            ("stale_unscoped_cache_reused", false),
            ("auto_enable_resolved_twice", false),
            ("diagnostic_payload_unbounded", false),
            ("plugin_runtime_started", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
        synthetic_startup_diagnostics_checks(sample_run),
    )
}

pub fn talk_session_controller_contract_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane(
        "talk-session-controller-contract-plane",
        "Talk/voice shared session controller, telemetry, and audio queue contract plane",
        sample_run,
        vec![
            row(
                "talk/session",
                "shared-session-controller-state-machine",
                "realtime, transcription, room handoff, Voice Call, Google Meet, VoiceClaw, and native clients share a controller contract",
                sample_run,
            ),
            row(
                "talk/rpc",
                "gateway-managed-talk-session-rpc-surface",
                "talk.session.* RPC remains bounded and explicit before live bridges are enabled",
                sample_run,
            ),
            row(
                "diagnostics/talk",
                "privacy-bounded-lifecycle-audio-metrics",
                "OTel/Prometheus metrics omit transcripts, audio payloads, room ids, turn ids, and session ids",
                sample_run,
            ),
            row(
                "logging/talk",
                "privacy-bounded-lifecycle-logs",
                "file/OTLP logs omit transcript text, audio payloads, call ids, and provider item ids",
                sample_run,
            ),
            row(
                "voice-call/realtime",
                "paced-audio-backpressure-bargein-queue",
                "paced audio queues are bounded and overload closes realtime streams before provider audio piles up",
                sample_run,
            ),
            row(
                "voice-context",
                "context-capsule-consult-cadence-and-duplicate-coalescing",
                "voice context capsules, consult cadence, same-session consult routing, and duplicate-consult coalescing are explicit",
                sample_run,
            ),
        ],
        &[
            ("live_voice_bridge_started", false),
            ("transcript_or_audio_logged", false),
            ("talk_metric_contains_raw_session_id", false),
            ("audio_queue_unbounded", false),
            ("duplicate_consult_not_coalesced", false),
            ("twiml_fallback_required", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_send", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn qa_live_proof_harness_contract_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane(
        "qa-live-proof-harness-contract-plane",
        "QA/Mantis live behavior proof harness and artifact-redaction contract plane",
        sample_run,
        vec![
            row(
                "qa/mantis/slack-desktop-smoke",
                "desktop-vnc-screenshot-artifact-schema",
                "Slack desktop smoke artifacts use bounded screenshot paths and redacted metadata",
                sample_run,
            ),
            row(
                "qa/mantis/discord-thread-attachment",
                "before-after-thread-attachment-evidence-schema",
                "Discord thread attachment before/after scenarios preserve file evidence without live execution by default",
                sample_run,
            ),
            row(
                "qa/mantis/visual-desktop",
                "mp4-screenshot-image-assertion-artifact-contract",
                "visual desktop MP4/screenshot/image-understanding artifacts are preserved with redaction gates",
                sample_run,
            ),
            row(
                "qa/whatsapp",
                "live-dm-canary-pairing-gate-contract",
                "WhatsApp live DM canaries require explicit linked-session credential pool and pairing gates",
                sample_run,
            ),
            row(
                "qa/codex-harness",
                "docker-testbox-auth-cache-protocol-diagnostics",
                "Codex harness diagnostics expose Docker/Testbox/auth/cache/protocol checkout readiness without leaking credentials",
                sample_run,
            ),
        ],
        &[
            ("live_qa_executed", false),
            ("desktop_browser_started", false),
            ("credential_pool_accessed", false),
            ("artifact_contains_secret", false),
            ("private_endpoint_logged", false),
            ("external_network_read", false),
            ("external_send", false),
            ("credential_value_read", false),
            ("side_effects_performed", false),
        ],
    )
}


pub fn hepta_unreleased_model_auth_provider_catalog_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-model-auth-provider-catalog-regressions",
        "Hepta upstream-unreleased model auth, provider catalog, model ref, and provider-route regressions",
        sample_run,
        vec![
            row(
                "models/auth",
                "models-auth-list-redacted-profile-inspection",
                "models auth list exposes profile metadata without secret values",
                sample_run,
            ),
            row(
                "auth/providers",
                "workspace-scoped-provider-id-resolution",
                "provider-id resolution receives config/workspaceDir and resolves workspace-scoped aliases without global leakage",
                sample_run,
            ),
            row(
                "providers/openrouter",
                "openrouter-cache-header-route-verification",
                "OpenRouter cache headers are attached only to verified OpenRouter routes",
                sample_run,
            ),
            row(
                "providers/openrouter",
                "openrouter-auto-canonical-picker-dedupe",
                "openrouter/auto remains canonical while pickers avoid openrouter/openrouter/auto",
                sample_run,
            ),
            row(
                "agents/models",
                "legacy-anthropic-cli-model-ref-resolution",
                "legacy anthropic-cli/* refs resolve as Claude CLI runtime refs",
                sample_run,
            ),
            row(
                "doctor/openai-codex",
                "codex-model-ref-doctor-preserves-oauth-profile",
                "doctor rewrites stale openai-codex refs while preserving Codex OAuth auth profiles",
                sample_run,
            ),
            row(
                "openai/codex",
                "stale-openai-codex-model-suppression",
                "stale GPT-5.1/5.2/5.3 OpenAI-Codex refs are suppressed in favor of current routes",
                sample_run,
            ),
            row(
                "providers/openrouter",
                "deepseek-v4-openrouter-reasoning-effort",
                "OpenRouter DeepSeek V4 reasoning_effort=max maps to supported xhigh",
                sample_run,
            ),
            row(
                "providers/media",
                "provider-audio-transcription-catalog-route",
                "transcription metadata routes separately from chat model ids",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("provider_call_performed", false),
            ("model_catalog_network_fetch_performed", false),
            ("oauth_or_device_flow_started", false),
            ("credential_value_read", false),
            ("secret_value_logged", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_model_auth_provider_catalog_checks(sample_run),
    )
}

pub fn hepta_unreleased_security_boundary_redaction_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-security-boundary-redaction-regressions",
        "Hepta upstream-unreleased security, boundary, SecretRef, proxy, approvals, and redaction regressions",
        sample_run,
        vec![
            row(
                "docker/gateway",
                "docker-gateway-capability-drop-contract",
                "Gateway container drops NET_RAW/NET_ADMIN and enables no-new-privileges",
                sample_run,
            ),
            row(
                "secrets/channels",
                "external-secret-contract-dist-sidecar",
                "external channel secret-contract sidecars resolve from rootDir/dist without secret reads",
                sample_run,
            ),
            row(
                "secrets/apply",
                "secrets-apply-secretref-preservation",
                "secrets apply preserves keyRef/tokenRef while scrubbing plaintext",
                sample_run,
            ),
            row(
                "plugin-sdk/security-runtime",
                "plugin-sdk-security-runtime-atomic-replacement",
                "shared fs-safe atomic replacement, sibling-temp writes, and cross-device fallback are exposed",
                sample_run,
            ),
            row(
                "exec/approvals",
                "tree-sitter-shell-command-explainer",
                "tree-sitter shell command explainer produces bounded review shape without execution",
                sample_run,
            ),
            row(
                "proxy/managed",
                "proxy-loopback-mode-control-plane",
                "proxy.loopbackMode governs Gateway loopback control-plane traffic",
                sample_run,
            ),
            row(
                "doctor/secrets",
                "doctor-secrets-safe-passenv-names",
                "safe inherited SecretRef passEnv names are allowed while dangerous hooks stay blocked",
                sample_run,
            ),
            row(
                "tools/tavily",
                "tavily-credential-resolution-secretref",
                "Tavily dedicated credentials resolve from runtime config snapshot without exposing SecretRefs",
                sample_run,
            ),
            row(
                "control-ui/approvals",
                "control-ui-approval-replay-binding",
                "trusted backend node approvals after request reconnect keep node/cwd/env/replay bindings",
                sample_run,
            ),
            row(
                "plugins/install",
                "managed-npm-root-security-overrides",
                "managed plugin npm roots inherit host npm security overrides",
                sample_run,
            ),
            row(
                "memory/wiki",
                "memory-wiki-empty-related-block-skip",
                "Memory Wiki skips empty/whitespace pages when refreshing Related blocks",
                sample_run,
            ),
            row(
                "exec/approvals/windows",
                "windows-exec-approval-storage-guarded-copy",
                "Windows exec-approval storage uses guarded copy fallback with link/permission safeguards",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("container_started", false),
            ("proxy_request_performed", false),
            ("approval_state_mutated", false),
            ("provider_call_performed", false),
            ("credential_value_read", false),
            ("secret_value_logged", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_security_boundary_redaction_checks(sample_run),
    )
}

pub fn hepta_unreleased_cli_doctor_observability_update_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-cli-doctor-observability-update-regressions",
        "Hepta upstream-unreleased CLI, doctor, observability, status, cron, and update regression contracts",
        sample_run,
        vec![
            row(
                "cron/cli",
                "cron-json-computed-status",
                "cron list/show JSON exposes computed status",
                sample_run,
            ),
            row(
                "cron/cli",
                "cron-list-agent-filter-normalization",
                "cron list --agent normalizes agent ids and includes default-agent jobs",
                sample_run,
            ),
            row(
                "channels/cli",
                "channels-list-channel-only-all-origin-installed",
                "channels list is channel-only and --all shows installed/configured/enabled origin metadata",
                sample_run,
            ),
            row(
                "channels/plugins",
                "channel-plugin-missing-repair-command",
                "missing official external channels show exact install/doctor repair commands",
                sample_run,
            ),
            row(
                "sessions/cli",
                "sessions-table-selected-agent-runtime",
                "sessions table includes selected agent runtime",
                sample_run,
            ),
            row(
                "status/uptime",
                "status-gateway-host-uptime",
                "status exposes compact Gateway process uptime and host uptime",
                sample_run,
            ),
            row(
                "discord/status",
                "discord-degraded-transport-starvation-signal",
                "Discord degraded transport and event-loop starvation surface in status/deep logs",
                sample_run,
            ),
            row(
                "doctor/plugins",
                "doctor-plugin-update-repair-hints",
                "doctor/plugin update hints remain exact and redacted",
                sample_run,
            ),
            row(
                "diagnostics/otel",
                "observability-redacted-otlp-prometheus-shape",
                "diagnostics export bounded redacted OTLP/Prometheus shapes",
                sample_run,
            ),
            row(
                "cli/update",
                "update-cli-dry-run-evidence-ledger",
                "update CLI evidence ledger is redacted and dry-run by default",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("cron_registry_mutated", false),
            ("channel_probe_performed", false),
            ("package_manager_invoked", false),
            ("provider_call_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_cli_doctor_observability_update_checks(sample_run),
    )
}

pub fn hepta_unreleased_agents_tools_subagents_failover_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-agents-tools-subagents-failover-regressions",
        "Hepta upstream-unreleased agents, tools, subagents, media completion, and failover regressions",
        sample_run,
        vec![
            row(
                "agents/failover",
                "state-aware-lane-suspension-quota-resume",
                "quota resume transitions restore lane concurrency and preserve non-quota failure reasons",
                sample_run,
            ),
            row(
                "agents/subagents",
                "grouped-child-result-preservation",
                "grouped child results survive direct completion fallback",
                sample_run,
            ),
            row(
                "agents/subagents",
                "parent-wake-announce-retry-cooldown",
                "parent wake announces retry after transient fallback cooldown exhaustion",
                sample_run,
            ),
            row(
                "agents/tools",
                "exec-node-disconnected-preflight",
                "exec host=node fails before system.run when node is known disconnected",
                sample_run,
            ),
            row(
                "agents/tools",
                "restrictive-profile-tool-warning-scope",
                "restrictive-profile warnings stay scoped to missing configured sections",
                sample_run,
            ),
            row(
                "agents/tools",
                "messaging-only-agent-no-global-fs-exec-warning",
                "messaging-only agents avoid inherited global fs/exec warnings",
                sample_run,
            ),
            row(
                "agents/verbose",
                "compact-explain-tool-summaries-default",
                "verbose/progress drafts use compact explain-mode tool summaries by default",
                sample_run,
            ),
            row(
                "agents/subagents",
                "spawn-system-prompt-override-task-preservation",
                "spawned target agents with systemPromptOverride still receive delegated task prompt",
                sample_run,
            ),
            row(
                "agents/media",
                "generated-media-async-completion-dedup",
                "async media completions avoid duplicate raw media while announce-agent run is pending",
                sample_run,
            ),
            row(
                "agents/diagnostics",
                "model-failover-otlp-event-shape",
                "model failover events export bounded diagnostics OTLP metadata",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("subagent_spawned", false),
            ("node_invoke_performed", false),
            ("media_generation_performed", false),
            ("provider_call_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_agents_tools_subagents_failover_checks(sample_run),
    )
}

pub fn hepta_unreleased_qa_mantis_live_proof_harness_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-qa-mantis-live-proof-harness-regressions",
        "Hepta upstream-unreleased QA/Mantis, Crabbox/Testbox, screenshot, MP4, and live proof harness contracts",
        sample_run,
        vec![
            row(
                "qa/mantis",
                "slack-desktop-crabbox-smoke-artifacts",
                "Slack desktop smoke records VNC screenshot artifact paths",
                sample_run,
            ),
            row(
                "qa/mantis",
                "discord-thread-attachment-before-after",
                "Discord thread attachment scenario captures before/after evidence shape",
                sample_run,
            ),
            row(
                "qa/mantis",
                "visual-desktop-mp4-screenshot-assertions",
                "visual desktop tasks preserve MP4/screenshot/image-assertion artifact manifests",
                sample_run,
            ),
            row(
                "qa/whatsapp",
                "whatsapp-live-dm-canary-pairing-gate",
                "WhatsApp live DM canary and pairing-gate coverage are represented as credential-pool contracts",
                sample_run,
            ),
            row(
                "qa/crabbox",
                "crabbox-env-passthrough-parent-immutable",
                "desktop-browser Crabbox child commands receive env without mutating parent",
                sample_run,
            ),
            row(
                "qa/mantis",
                "failure-screenshot-path-returned",
                "Slack desktop screenshot path is returned even when remote QA fails",
                sample_run,
            ),
            row(
                "qa/testbox",
                "blacksmith-tbx-lease-id-acceptance",
                "Blacksmith tbx lease ids are accepted before inspect/run",
                sample_run,
            ),
            row(
                "qa/codex",
                "codex-docker-testbox-diagnostics-preflight",
                "Codex live Docker/Testbox diagnostics expose auth/cache/checkout preflight shape",
                sample_run,
            ),
            row(
                "ci/crabbox",
                "aws-standard-multiregion-fallback",
                "Crabbox owned AWS fallback defaults to standard multi-region broker hints",
                sample_run,
            ),
            row(
                "qa/mantis",
                "slack-desktop-hydrate-phase-timing",
                "Slack desktop smoke hydrate modes expose phase timing reports",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("crabbox_allocated", false),
            ("testbox_allocated", false),
            ("screenshot_captured", false),
            ("mp4_captured", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_qa_mantis_live_proof_harness_checks(sample_run),
    )
}

pub fn hepta_unreleased_control_ui_operator_chat_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-control-ui-operator-chat-regressions",
        "Hepta upstream-unreleased Control UI, operator chat, responsive controls, QR/relink, and context pressure contracts",
        sample_run,
        vec![
            row(
                "control-ui/chat",
                "agent-first-chat-session-picker",
                "chat session picker supports agent-first filtering",
                sample_run,
            ),
            row(
                "control-ui/chat",
                "responsive-chat-controls-composer",
                "chat controls/composer stay responsive across phone/tablet/desktop",
                sample_run,
            ),
            row(
                "control-ui/chat",
                "desktop-controls-one-row-scroll-hide",
                "desktop controls remain one row and hide while scrolling down transcript",
                sample_run,
            ),
            row(
                "control-ui/chat",
                "initial-avatar-refresh-dedup",
                "initial chat load avoids duplicate avatar refreshes",
                sample_run,
            ),
            row(
                "control-ui/chat",
                "duplicate-text-bubble-count-collapse",
                "consecutive duplicate text messages collapse into counted bubble",
                sample_run,
            ),
            row(
                "control-ui/sessions",
                "inherited-thinking-default-label",
                "inherited thinking defaults are labeled separately from explicit overrides",
                sample_run,
            ),
            row(
                "control-ui/whatsapp",
                "whatsapp-show-qr-relink-wait-scan-state",
                "WhatsApp Show QR/Relink/Wait-for-scan states are explicit",
                sample_run,
            ),
            row(
                "control-ui/header",
                "active-agent-breadcrumb-without-session-key",
                "dashboard breadcrumbs show active agent name without crowding session key",
                sample_run,
            ),
            row(
                "control-ui/plugins",
                "external-plugin-install-hints",
                "config-referenced uninstalled external plugins show install hints",
                sample_run,
            ),
            row(
                "control-ui/debug",
                "long-animation-frame-debug-log",
                "debug log records long animation frame/task entries where supported",
                sample_run,
            ),
            row(
                "control-ui/context",
                "compact-context-usage-indicator",
                "compact context usage indicator appears before high-pressure warning",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("browser_automation_performed", false),
            ("real_qr_generated", false),
            ("private_session_content_read", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_control_ui_operator_chat_checks(sample_run),
    )
}

pub fn hepta_unreleased_memory_active_compaction_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-memory-active-compaction-regressions",
        "Hepta upstream-unreleased memory, active-memory, dreaming, compaction, context cache, and transcript regressions",
        sample_run,
        vec![
            row(
                "plugins/active-memory",
                "scoped-channel-id-skip-for-recall-subagent",
                "active-memory skips channel entries containing scoped ids when resolving recall subagent channel",
                sample_run,
            ),
            row(
                "google-chat/memory",
                "spaces-conversation-id-scoped-target",
                "Google Chat spaces conversation ids are treated as scoped targets, not runnable channel names",
                sample_run,
            ),
            row(
                "active-memory/status",
                "active-memory-status-agent-allowlist",
                "active-memory status honors configured agent allowlist",
                sample_run,
            ),
            row(
                "active-memory/admin",
                "global-active-memory-admin-toggle",
                "global active-memory toggles require admin scope",
                sample_run,
            ),
            row(
                "memory/dreaming",
                "openai-output-text-narrative-subagent",
                "Dream Diary reads OpenAI-style output_text assistant parts",
                sample_run,
            ),
            row(
                "agents/compaction",
                "compaction-output-reserve-model-cap",
                "compaction output reserve tokens are capped to selected model maxTokens",
                sample_run,
            ),
            row(
                "agents/compaction",
                "safeguard-compaction-visible-anchor-types",
                "safeguard compaction treats visible custom-message/bash/branch-summary entries as anchors",
                sample_run,
            ),
            row(
                "telegram/compaction",
                "telegram-preview-replay-stale-guard",
                "Telegram draft preview rotation avoids stale pre-tool previews after compaction replay",
                sample_run,
            ),
            row(
                "webchat/context",
                "persistent-context-usage-indicator",
                "persistent context usage indicator remains available in WebChat",
                sample_run,
            ),
            row(
                "memory/wiki",
                "memory-wiki-whitespace-related-skip",
                "Memory Wiki skips whitespace-only pages during Related refresh",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("private_memory_corpus_read", false),
            ("recall_subagent_run", false),
            ("session_store_mutated", false),
            ("provider_call_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_memory_active_compaction_checks(sample_run),
    )
}

pub fn hepta_unreleased_multi_channel_longtail_receipts_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-multi-channel-longtail-receipts-regressions",
        "Hepta upstream-unreleased multi-channel long-tail message lifecycle, receipts, preview finalization, and SDK regression contracts",
        sample_run,
        vec![
            row(
                "plugin-sdk/channel-message",
                "channel-message-lifecycle-helper-receipts",
                "channel-message SDK lifecycle helpers produce normalized receipt shapes",
                sample_run,
            ),
            row(
                "channels/reply",
                "legacy-channel-reply-pipeline-wrapper",
                "legacy channel-reply-pipeline wraps shared reply core",
                sample_run,
            ),
            row(
                "discord/slack/mattermost/matrix",
                "live-preview-finalization-shared-sdk",
                "live-preview finalization moves onto shared channel-message SDK",
                sample_run,
            ),
            row(
                "telegram/teams",
                "finalized-preview-native-stream-receipts",
                "Telegram finalized previews and Teams native stream finals attach message receipts",
                sample_run,
            ),
            row(
                "slack/performance",
                "slack-message-preparation-thread-context-fast-path",
                "Slack message preparation, recipient lookup, and thread-context allocation are bounded",
                sample_run,
            ),
            row(
                "discord/status",
                "discord-degraded-transport-receipt-status",
                "Discord degraded transport surfaces in delivery/status receipt metadata",
                sample_run,
            ),
            row(
                "channels/plugins",
                "official-external-channel-missing-plugin-receipts",
                "official external missing-plugin rows include repair commands and receipt-safe errors",
                sample_run,
            ),
            row(
                "channels/longtail",
                "bluebubbles-feishu-googlechat-imessage-contracts",
                "BlueBubbles/Feishu/Google Chat/iMessage route and receipt metadata are contract-only",
                sample_run,
            ),
            row(
                "channels/longtail",
                "irc-line-matrix-nextcloud-qq-signal-contracts",
                "IRC/LINE/Matrix/Nextcloud Talk/QQ Bot/Signal route and receipt metadata are contract-only",
                sample_run,
            ),
            row(
                "channels/longtail",
                "synology-tlon-twitch-zalo-contracts",
                "Synology Chat/Tlon/Twitch/Zalo route and receipt metadata are contract-only",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("channel_send_performed", false),
            ("external_channel_started", false),
            ("history_read", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_multi_channel_longtail_receipts_checks(sample_run),
    )
}

pub fn hepta_unreleased_imessage_imsg_bluebubbles_parity_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-imessage-imsg-bluebubbles-parity-regressions",
        "Hepta upstream-unreleased iMessage imsg JSON-RPC and BlueBubbles replacement parity contracts",
        sample_run,
        vec![
            row(
                "channels/imessage",
                "imsg-json-rpc-capability-map",
                "imsg JSON-RPC exposes BlueBubbles replacement capability metadata without reading Messages history",
                sample_run,
            ),
            row(
                "channels/imessage",
                "bluebubbles-to-imessage-migration-contract",
                "channels.bluebubbles migration maps to channels.imessage with explicit service/node boundaries",
                sample_run,
            ),
            row(
                "channels/imessage",
                "imsg-send-reply-react-edit-unsend-effects-shape",
                "send/reply/react/edit/unsend/effects/group action shapes are represented as redacted contracts",
                sample_run,
            ),
            row(
                "channels/imessage",
                "signed-in-mac-node-route-fail-closed",
                "missing signed-in Mac/node route fails closed before JSON-RPC mutation",
                sample_run,
            ),
            row(
                "channels/imessage",
                "redacted-local-probe-version-help-db",
                "local probe reports imsg help/version and Messages DB existence without chat reads",
                sample_run,
            ),
            row(
                "channels/imessage",
                "live-send-confirmation-gate",
                "live-send path requires explicit confirm-send and target/text boundaries",
                sample_run,
            ),
            row(
                "channels/imessage",
                "bluebubbles-ssrf-guard-carry-forward",
                "BlueBubbles reply-context URL guard carries forward during migration",
                sample_run,
            ),
            row(
                "channels/imessage",
                "receipt-thread-service-normalization",
                "receipt metadata normalizes chat id, thread id, service, and provider message id without raw identifiers",
                sample_run,
            ),
            row(
                "channels/imessage",
                "history-readiness-no-history-read",
                "history readiness reports capability state without dumping or scanning chat history",
                sample_run,
            ),
            row(
                "channels/imessage",
                "node-host-jsonrpc-timeout-boundary",
                "node-host JSON-RPC timeout/error shape is bounded and redacted",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("imsg_rpc_called", false),
            ("message_send_performed", false),
            ("chat_history_read", false),
            ("messages_db_read", false),
            ("bluebubbles_network_called", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_imessage_imsg_bluebubbles_parity_checks(sample_run),
    )
}

pub fn hepta_unreleased_plugin_update_externalized_lifecycle_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-plugin-update-externalized-lifecycle-regressions",
        "Hepta upstream-unreleased plugin update, externalized bundled plugin, managed npm-root, and lifecycle repair contracts",
        sample_run,
        vec![
            row(
                "plugins/update",
                "official-externalized-bundled-plugin-migration",
                "official externalized bundled plugins migrate from bundled paths to trusted source-linked installs",
                sample_run,
            ),
            row(
                "plugins/update",
                "clawhub-preferred-after-npm-fallback",
                "temporary npm fallback does not permanently displace ClawHub preferred source",
                sample_run,
            ),
            row(
                "plugins/update",
                "stale-bundled-load-path-cleanup",
                "stale bundled load paths are cleaned for pinned npm/ClawHub plugins",
                sample_run,
            ),
            row(
                "plugins/update",
                "managed-npm-root-peer-repair",
                "managed npm-root repairs missing peer packages without leaking global package state",
                sample_run,
            ),
            row(
                "plugins/update",
                "peer-link-reassertion-after-update",
                "peer links are reasserted after plugin update/repair",
                sample_run,
            ),
            row(
                "plugins/update",
                "rollback-repair-uninstall-legacy-peer-resolution",
                "rollback/repair/uninstall resolve legacy peer packages deterministically",
                sample_run,
            ),
            row(
                "plugins/update",
                "package-lock-safety-and-version-verification",
                "package-lock safety and expected version verification are represented before install",
                sample_run,
            ),
            row(
                "plugins/update",
                "beta-default-line-fallback",
                "beta/default release-line fallback is explicit and redacted",
                sample_run,
            ),
            row(
                "plugin-sdk/lifecycle",
                "absolute-posix-lifecycle-shell-path",
                "plugin lifecycle shell paths are absolute POSIX paths and never implicit shell lookups",
                sample_run,
            ),
            row(
                "plugins/update",
                "update-evidence-ledger-redacted",
                "plugin update evidence ledger stores only redacted package/source fingerprints",
                sample_run,
            ),
            row(
                "plugins/update",
                "stale-managed-npm-root-repair-plan",
                "stale managed npm roots produce exact repair plans without invoking package managers",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("package_manager_invoked", false),
            ("plugin_installed", false),
            ("plugin_updated", false),
            ("plugin_uninstalled", false),
            ("managed_npm_root_mutated", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_plugin_update_externalized_lifecycle_checks(sample_run),
    )
}

pub fn hepta_unreleased_runtime_install_platform_floor_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    let host_node_major = 25_u32;
    let host_node_minor = 9_u32;
    let node_floor_major = 22_u32;
    let node_floor_minor = 16_u32;
    let node_floor_satisfied =
        (host_node_major, host_node_minor) >= (node_floor_major, node_floor_minor);
    contract_plane_with_checks(
        "hepta-unreleased-runtime-install-platform-floor-regressions",
        "Hepta upstream-unreleased runtime install, Node platform floor, node:sqlite metadata, and platform compatibility contracts",
        sample_run,
        vec![
            row(
                "runtime/install",
                "node-22-16-minimum-floor",
                "Node 22.16+ minimum floor is represented while Node 24 remains recommended",
                sample_run,
            ),
            row(
                "runtime/install",
                "node-sqlite-statement-metadata-capability",
                "node:sqlite statement metadata availability is modeled as a capability",
                sample_run,
            ),
            row(
                "runtime/install",
                "node-24-recommendation-not-hard-requirement",
                "Node 24 recommendation does not reject supported Node 22.16+ runtimes",
                sample_run,
            ),
            row(
                "runtime/install",
                "hepta-plugin-bridge-floor-hints",
                "plugin/runtime bridges expose exact update hints when Node floor is not satisfied",
                sample_run,
            ),
            row(
                "gateway/windows",
                "windows-loopback-bind-127001-contract",
                "Windows default loopback listener binds 127.0.0.1 instead of dual-stack ::1",
                sample_run,
            ),
            row(
                "security/windows",
                "windows-exec-approval-guarded-copy-storage",
                "Windows exec-approval storage uses guarded copy fallback with link/permission safeguards",
                sample_run,
            ),
            row(
                "runtime/install",
                "node-manager-no-mutation-sample-mode",
                "platform floor checks do not install or mutate Node managers in sample mode",
                sample_run,
            ),
            row(
                "runtime/install",
                "native-sqlite-query-handling-version-gate",
                "native SQLite query handling is gated by statement metadata presence, not optimistic version strings",
                sample_run,
            ),
            row(
                "runtime/install",
                "runtime-floor-diagnostics-redacted",
                "platform floor diagnostics redact PATH/home-specific details",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("node_floor_satisfied", node_floor_satisfied),
            ("node_install_performed", false),
            ("package_manager_invoked", false),
            ("runtime_config_mutated", false),
            ("filesystem_mutated", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_runtime_install_platform_floor_checks(sample_run),
    )
}

pub fn hepta_unreleased_discord_voice_live_tts_stt_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-discord-voice-live-tts-stt-regressions",
        "Hepta upstream-unreleased Discord voice live TTS/STT, permission audit, diagnostics, and feedback-loop contracts",
        sample_run,
        vec![
            row(
                "discord/voice",
                "voice-channel-permission-audit-shape",
                "Connect/Speak/Read Message History permission audit is represented for voice targets",
                sample_run,
            ),
            row(
                "discord/voice",
                "bounded-stt-preview-redaction",
                "verbose voice logs include only bounded one-line STT previews",
                sample_run,
            ),
            row(
                "discord/voice",
                "elevenlabs-direct-playback-latency-query",
                "ElevenLabs TTS direct playback and latency optimization query shape are represented",
                sample_run,
            ),
            row(
                "discord/voice",
                "playback-continues-while-capture-ignored",
                "TTS playback continues while new speaker capture is ignored to avoid feedback loops",
                sample_run,
            ),
            row(
                "discord/voice",
                "expected-receive-stream-abort-verbose",
                "expected receive-stream aborts downgrade to verbose diagnostics",
                sample_run,
            ),
            row(
                "discord/voice",
                "capture-silence-grace-bounds",
                "voice.captureSilenceGraceMs parser bounds noisy-session overrides",
                sample_run,
            ),
            row(
                "discord/streaming",
                "progress-draft-default-for-voice-replies",
                "Discord replies default to progress draft previews unless streaming mode is off",
                sample_run,
            ),
            row(
                "discord/status",
                "degraded-transport-voice-status",
                "degraded Discord transport and event-loop starvation are included in voice diagnostics",
                sample_run,
            ),
            row(
                "discord/voice",
                "spoken-output-prompt-tightening",
                "spoken-output prompt keeps live STT fragments bounded",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("discord_connected", false),
            ("voice_capture_started", false),
            ("tts_synthesis_performed", false),
            ("audio_payload_logged", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_discord_voice_live_tts_stt_checks(sample_run),
    )
}

pub fn hepta_unreleased_talk_meet_voicecall_realtime_productization_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-talk-meet-voicecall-realtime-productization-regressions",
        "Hepta upstream-unreleased Talk, Google Meet, Voice Call, Twilio, and realtime productization contracts",
        sample_run,
        vec![
            row(
                "google-meet/realtime",
                "empty-intro-message-preservation",
                "realtime.introMessage empty string is preserved for silent Chrome joins",
                sample_run,
            ),
            row(
                "voice-call/realtime",
                "agent-voice-context-capsule-shape",
                "opt-in agent voice context capsules are bounded and transcript-free",
                sample_run,
            ),
            row(
                "voice-call/realtime",
                "consult-cadence-guidance-bounds",
                "consult cadence guidance avoids full agent consult on ordinary turns",
                sample_run,
            ),
            row(
                "twilio/realtime",
                "paced-audio-queue-backpressure-close",
                "paced Twilio audio queue is bounded and overloaded realtime streams close before backlog piles up",
                sample_run,
            ),
            row(
                "google-meet/voice-call",
                "same-session-agent-consult-routing",
                "same-session agent consult routing and duplicate-consult coalescing are represented",
                sample_run,
            ),
            row(
                "talk/session",
                "shared-talk-session-controller-rpc-surface",
                "Talk session controller RPC shape covers realtime relay, transcription relay, managed room, Meet, VoiceClaw, and native clients",
                sample_run,
            ),
            row(
                "diagnostics/talk",
                "bounded-talk-lifecycle-otel-prometheus",
                "Talk lifecycle/audio metrics export redacted OTLP/Prometheus shapes",
                sample_run,
            ),
            row(
                "tts/telephony",
                "provider-voice-model-override-log-alignment",
                "telephony synthesis logs match selected provider voice/model override metadata",
                sample_run,
            ),
            row(
                "google-meet/voice-call",
                "twilio-gemini-paced-audio-buffer-contract",
                "Twilio dial-in speaks through realtime Gemini bridge with pacing and backpressure metadata",
                sample_run,
            ),
            row(
                "talk/privacy",
                "no-transcript-audio-room-id-logging",
                "Talk logs exclude transcript text, audio payloads, room ids, turn ids, and provider item ids",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("meeting_joined", false),
            ("twilio_call_started", false),
            ("realtime_provider_connected", false),
            ("audio_payload_logged", false),
            ("transcript_text_logged", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_talk_meet_voicecall_realtime_productization_checks(sample_run),
    )
}

pub fn hepta_unreleased_qa_mantis_exact_proof_harness_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-qa-mantis-exact-proof-harness-regressions",
        "Hepta upstream-unreleased QA/Mantis exact-confirmation live proof harness, artifact redaction, and credential-pool contracts",
        sample_run,
        vec![
            row(
                "qa/mantis",
                "exact-proof-artifact-manifest-schema",
                "screenshot/MP4/image assertion evidence manifests are redacted and typed",
                sample_run,
            ),
            row(
                "qa/mantis",
                "slack-desktop-smoke-confirmation-gate",
                "Slack desktop live smoke requires explicit confirmation and credential pool preflight",
                sample_run,
            ),
            row(
                "qa/mantis",
                "discord-thread-attachment-before-after-confirmation",
                "Discord thread attachment before/after proof is exact-confirmation gated",
                sample_run,
            ),
            row(
                "qa/whatsapp",
                "whatsapp-live-dm-canary-pairing-gate",
                "WhatsApp live DM canary requires pairing gate and credential-pool readiness",
                sample_run,
            ),
            row(
                "qa/crabbox",
                "crabbox-testbox-no-allocation-sample-mode",
                "Crabbox/Testbox leases are never allocated in sample mode",
                sample_run,
            ),
            row(
                "qa/testbox",
                "codex-docker-testbox-diagnostics-redacted",
                "Codex Docker/Testbox diagnostics expose auth/cache/checkout shape without starting containers",
                sample_run,
            ),
            row(
                "qa/mantis",
                "failure-screenshot-copy-redacted-path",
                "failure screenshot copied-path evidence is redacted and bounded",
                sample_run,
            ),
            row(
                "qa/mantis",
                "hydrate-phase-timing-evidence-ledger",
                "Slack desktop hydrate cold/warm timing records store redacted phase timing",
                sample_run,
            ),
            row(
                "qa/proof",
                "external-contributor-private-info-redaction-reminder",
                "external proof ingestion reminds contributors to redact IPs, keys, phone numbers, and private endpoints",
                sample_run,
            ),
            row(
                "qa/proof",
                "proof-supplied-sufficient-label-separation",
                "proof supplied/sufficient labels remain distinct until exact proof passes",
                sample_run,
            ),
            row(
                "qa/proof",
                "credential-pool-preflight-no-secret-read",
                "credential-pool preflight reports availability without reading token values",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("crabbox_allocated", false),
            ("testbox_allocated", false),
            ("browser_started", false),
            ("screenshot_captured", false),
            ("mp4_captured", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_qa_mantis_exact_proof_harness_checks(sample_run),
    )
}


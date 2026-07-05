use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeptaLatestAbsorptionPriority {
    P0,
    P0P1,
    P1,
    P1P2,
    P2,
    Guardrail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaLatestAbsorptionContract {
    pub id: String,
    pub priority: HeptaLatestAbsorptionPriority,
    pub title: String,
    pub source_files: Vec<String>,
    pub source_commits_or_prs: Vec<String>,
    pub hepta_native_surfaces: Vec<String>,
    pub absorbed_capabilities: Vec<String>,
    pub validation_gate: String,
    pub contract_ready: bool,
    pub local_only_audit: bool,
    pub report_only: bool,
    pub mutates_runtime_state: bool,
    pub external_network_read: bool,
    pub external_send: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub not_claimed: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaLatestAbsorptionReport {
    pub product: String,
    pub status: String,
    pub source_agent: String,
    pub source_head: String,
    pub release_anchor: String,
    pub git_describe: String,
    pub baseline: String,
    pub contract_count: usize,
    pub ready_contract_count: usize,
    pub p0_contract_count: usize,
    pub p0_ready_contract_count: usize,
    pub p1_or_later_contract_count: usize,
    pub p1_or_later_ready_contract_count: usize,
    pub absorption_complete: bool,
    pub p0_absorption_complete: bool,
    pub local_only_audit: bool,
    pub report_only: bool,
    pub side_effects_performed: bool,
    pub mutates_runtime_state: bool,
    pub external_network_read: bool,
    pub external_send: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub unstable_goal_checklist_absorbed_as_guardrail: bool,
    pub contracts: Vec<HeptaLatestAbsorptionContract>,
}

impl HeptaLatestAbsorptionReport {
    pub fn native_default() -> Self {
        Self::from_contracts(default_hepta_latest_contracts())
    }

    pub fn from_contracts(contracts: Vec<HeptaLatestAbsorptionContract>) -> Self {
        let contract_count = contracts.len();
        let ready_contract_count = contracts
            .iter()
            .filter(|contract| contract.contract_ready)
            .count();
        let is_p0 = |priority: HeptaLatestAbsorptionPriority| {
            matches!(
                priority,
                HeptaLatestAbsorptionPriority::P0
                    | HeptaLatestAbsorptionPriority::P0P1
                    | HeptaLatestAbsorptionPriority::Guardrail
            )
        };
        let p0_contract_count = contracts
            .iter()
            .filter(|contract| is_p0(contract.priority))
            .count();
        let p0_ready_contract_count = contracts
            .iter()
            .filter(|contract| is_p0(contract.priority) && contract.contract_ready)
            .count();
        let p1_or_later_contract_count = contract_count.saturating_sub(p0_contract_count);
        let p1_or_later_ready_contract_count =
            ready_contract_count.saturating_sub(p0_ready_contract_count);
        let absorption_complete = contract_count > 0 && ready_contract_count == contract_count;
        let p0_absorption_complete =
            p0_contract_count > 0 && p0_ready_contract_count == p0_contract_count;
        let unstable_goal_checklist_absorbed_as_guardrail = contracts.iter().any(|contract| {
            contract.id == "goal-checklist-subgoal-rollback-guard"
                && contract.contract_ready
                && contract.priority == HeptaLatestAbsorptionPriority::Guardrail
        });

        Self {
            product: "Hepta".into(),
            status: if absorption_complete {
                "ready"
            } else {
                "attention"
            }
            .into(),
            source_agent: "hepta-agent".into(),
            source_head: "88a2ce4a".into(),
            release_anchor: "v2026.5.7 / RELEASE_v0.13.0".into(),
            git_describe: "v2026.5.7-432-g88a2ce4a".into(),
            baseline: "Hepta latest delta audit 2026-05-12".into(),
            contract_count,
            ready_contract_count,
            p0_contract_count,
            p0_ready_contract_count,
            p1_or_later_contract_count,
            p1_or_later_ready_contract_count,
            absorption_complete,
            p0_absorption_complete,
            local_only_audit: true,
            report_only: true,
            side_effects_performed: false,
            mutates_runtime_state: false,
            external_network_read: false,
            external_send: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            unstable_goal_checklist_absorbed_as_guardrail,
            contracts,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn contract(
    id: &str,
    priority: HeptaLatestAbsorptionPriority,
    title: &str,
    source_files: &[&str],
    source_commits_or_prs: &[&str],
    hepta_native_surfaces: &[&str],
    absorbed_capabilities: &[&str],
    validation_gate: &str,
    not_claimed: &str,
) -> HeptaLatestAbsorptionContract {
    HeptaLatestAbsorptionContract {
        id: id.into(),
        priority,
        title: title.into(),
        source_files: source_files.iter().map(|value| (*value).into()).collect(),
        source_commits_or_prs: source_commits_or_prs
            .iter()
            .map(|value| (*value).into())
            .collect(),
        hepta_native_surfaces: hepta_native_surfaces
            .iter()
            .map(|value| (*value).into())
            .collect(),
        absorbed_capabilities: absorbed_capabilities
            .iter()
            .map(|value| (*value).into())
            .collect(),
        validation_gate: validation_gate.into(),
        contract_ready: true,
        local_only_audit: true,
        report_only: true,
        mutates_runtime_state: false,
        external_network_read: false,
        external_send: false,
        credential_value_read: false,
        secret_file_read: false,
        provider_invoked: false,
        channel_delivery_performed: false,
        gateway_rpc_performed: false,
        not_claimed: not_claimed.into(),
    }
}

fn default_hepta_latest_contracts() -> Vec<HeptaLatestAbsorptionContract> {
    vec![
        contract(
            "task-board-v0-durable-kanban",
            HeptaLatestAbsorptionPriority::P0,
            "Durable multi-agent execution board",
            &[
                "hepta_cli/kanban.py",
                "hepta_cli/kanban_db.py",
                "hepta_cli/kanban_diagnostics.py",
            ],
            &["RELEASE_v0.13.0.md", "v2026.5.7-432-g88a2ce4a"],
            &[
                "/task-board",
                "/worker-tasks",
                "/runtime-event-plane --task-status-runtime",
                "/runtime-event-plane --task-registry-runtime",
            ],
            &[
                "durable multi-board task DB contract",
                "parent/child dependency graph",
                "worker heartbeat and stale-claim reclaim",
                "zombie detection",
                "per-task retry budgets",
                "worker logs and run records",
                "auto-block on incomplete worker exit",
                "diagnostics for hallucinated cards, phantom refs, repeated failures, stranded ready tasks, and stuck blocked tasks",
            ],
            "cargo test -p hepta-core hepta_latest_absorption_covers_full_priority_queue_without_side_effects --quiet",
            "live task DB migration, worker spawn, task registry mutation, queue mutation, or autonomous execution",
        ),
        contract(
            "operator-security-session-safety-pack",
            HeptaLatestAbsorptionPriority::P0,
            "Gateway/session safety and operator policy hardening",
            &[
                "gateway/slash_access.py",
                "gateway/shutdown_forensics.py",
                "gateway/platforms/msgraph_webhook.py",
            ],
            &["976d8e27", "9520a1cc", "f6736ced", "cede6129", "#23647"],
            &[
                "/operator-security",
                "/runtime-event-plane --system-event-queue",
                "/operator-dashboard-polish",
                "/ui-action-plan --dry-run",
            ],
            &[
                "auto-resume interrupted sessions after gateway restart",
                "preserved pending update prompts and thread routing",
                "allowed_channels / allowed_chats / allowed_rooms allowlists",
                "platform admin/user split for slash commands",
                "default-on debug-share redaction",
                "quick-command env sanitization and output redaction",
                "sudo -S and stdin password guessing guard",
                "shutdown forensics with phase timing and stale-unit warning",
            ],
            "cargo test -p hepta-core operator_security_report_reaches_local_100_without_external_claims --quiet",
            "gateway restart, slash command dispatch, channel route mutation, process execution, credential reads, or external sends",
        ),
        contract(
            "plugin-provider-abi-ctx-llm-transform-hook",
            HeptaLatestAbsorptionPriority::P0,
            "Plugin provider ABI and plugin-local LLM calls",
            &[
                "agent/plugin_llm.py",
                "agent/think_scrubber.py",
                "agent/error_classifier.py",
                "hepta_cli/profile_distribution.py",
            ],
            &["5aa755e4", "a479ec01", "7026af4e"],
            &[
                "/provider-registration",
                "/native-model-provider",
                "/plugin-migration-audit",
                "/model-catalog-manifest",
            ],
            &[
                "ProviderProfile-style typed provider plugin registry",
                "auth-readiness probe contract without secret reads",
                "provider picker filtering by available credential labels",
                "plugin-local ctx.llm bounded-call envelope",
                "transform_llm_output lifecycle hook for response shaping/filtering",
                "thinking/hidden-reasoning scrubber boundary",
            ],
            "cargo test -p hepta-cli provider_registration_command_reports_provider_registration_contracts --quiet",
            "provider/plugin runtime start, prompt submission, model call, token refresh, hidden reasoning capture, or plugin import",
        ),
        contract(
            "mcp-media-tool-result-robustness-pack",
            HeptaLatestAbsorptionPriority::P0P1,
            "MCP and tool-result robustness pack",
            &["tools/mcp_tool.py", "tools/vision_tools.py"],
            &["#20298", "#20358", "#20281"],
            &[
                "/tools-invoke",
                "/media-generation-plane",
                "/message-contract-map",
                "/runtime-event-plane --tool-event",
            ],
            &[
                "MCP SSE transport contract with OAuth forwarding metadata",
                "sse_read_timeout independent from tool timeout",
                "stale-pipe retry as session-expired",
                "image tool results surfaced as MEDIA lines",
                "keepalive during long lifecycle waits",
                "numeric arg coercion",
                "capability-gated utility stubs",
                "video_analyze surface contract",
                "[[as_document]] media-routing directive",
            ],
            "cargo test -p hepta-cli runtime_event -- --nocapture",
            "MCP connection, SDK-dependent stream read, OAuth token forwarding, media upload, model video analysis, or channel delivery",
        ),
        contract(
            "post-write-delta-lint-pack",
            HeptaLatestAbsorptionPriority::P1,
            "Post-write delta lint for file edits",
            &["tools/edit.py", "tools/patch.py", "tools/write_file.py"],
            &["RELEASE_v0.13.0.md"],
            &[
                "/diffs",
                "/filesystem-plane",
                "/runtime-event-plane --write-event",
            ],
            &[
                "immediate Python lint after write/patch",
                "JSON/YAML/TOML syntax gate after write/patch",
                "delta-only diagnostics returned to operator",
                "fail-closed write-quality report before claiming success",
            ],
            "cargo test -p hepta-cli diffs --quiet",
            "file writes, patch application, formatter invocation, dependency installation, or repository mutation",
        ),
        contract(
            "cron-no-agent-watchers-pack",
            HeptaLatestAbsorptionPriority::P1,
            "Cron no-agent watchdog and watcher skill pattern",
            &["cron", "optional-skills/watchers"],
            &["ea8e6088"],
            &[
                "/runtime-event-plane --cron-event",
                "/lifecycle-dry-run-map",
                "/operational-utility-contract-map",
            ],
            &[
                "script-only no_agent cron jobs",
                "empty stdout stays silent",
                "non-empty stdout delivered verbatim",
                "cheap health checks without model calls",
                "RSS / HTTP JSON / GitHub polling watcher pattern",
            ],
            "cargo test -p hepta-cli runtime_event -- --nocapture",
            "cron materialization, timer registration, script execution, stdout delivery, network polling, or model invocation",
        ),
        contract(
            "media-delivery-directives-video-analyze-pack",
            HeptaLatestAbsorptionPriority::P1,
            "Media delivery directives and video analysis pack",
            &["tools/vision_tools.py", "tools/video_analyze.py"],
            &["4ed293b3", "#20332"],
            &[
                "/media-generation-plane",
                "/message-contract-map",
                "/runtime-event-plane --assistant-output-directive-runtime",
            ],
            &[
                "video_analyze multimodal model contract",
                "[[as_document]] delivery directive",
                "Telegram draft streaming envelope",
                "overflow-safe Telegram edit/split policy",
                "UTF-16 split boundary accounting",
            ],
            "cargo test -p hepta-core media_delivery_contract_covers_multi_image_and_centralized_audio_without_side_effects --quiet",
            "provider video analysis, draft streaming, message edit, Telegram API call, or media upload",
        ),
        contract(
            "i18n-static-message-catalog-pack",
            HeptaLatestAbsorptionPriority::P1,
            "Static-message i18n catalog",
            &[
                "agent/i18n.py",
                "locales/af.yaml",
                "locales/de.yaml",
                "locales/en.yaml",
                "locales/es.yaml",
                "locales/fr.yaml",
                "locales/ga.yaml",
                "locales/hu.yaml",
                "locales/it.yaml",
                "locales/ja.yaml",
                "locales/ko.yaml",
                "locales/pt.yaml",
                "locales/ru.yaml",
                "locales/tr.yaml",
                "locales/uk.yaml",
                "locales/zh-hant.yaml",
                "locales/zh.yaml",
            ],
            &["c3916845"],
            &[
                "/native-capabilities",
                "/ops-status",
                "/operator-dashboard-polish",
            ],
            &[
                "typed static-message catalog contract",
                "locale fallback chain",
                "CLI/gateway/dashboard message key coverage",
                "missing-key diagnostic surface",
            ],
            "cargo test -p hepta-core hepta_latest_absorption_covers_full_priority_queue_without_side_effects --quiet",
            "runtime locale switching, translated text generation, external translation API calls, or private payload localization",
        ),
        contract(
            "profile-distribution-session-handoff-pack",
            HeptaLatestAbsorptionPriority::P1,
            "Profile distribution and session handoff/resume pack",
            &["hepta_cli/profile_distribution.py", "tui_gateway"],
            &["f209a358", "878611a7", "00ce5f04", "09a49146"],
            &[
                "/thread-binding",
                "/session-orchestration",
                "/runtime-event-plane --session-command-export",
            ],
            &[
                "git-shareable profile distribution metadata",
                "redacted profile packaging contract",
                "live cross-platform /handoff envelope",
                "TUI /sessions browse/resume contract",
                "resume/delete guardrails",
            ],
            "cargo test -p hepta-cli session_orchestration --quiet",
            "git push/pull, session transfer mutation, transcript export, TUI process start, profile secret read, or cross-platform send",
        ),
        contract(
            "search-skill-catalog-refresh-pack",
            HeptaLatestAbsorptionPriority::P1P2,
            "Search and optional skill ecosystem refresh",
            &[
                "optional-skills/search/searxng",
                "optional-skills/watchers",
                "optional-skills/finance/stocks",
                "optional-skills/api-testing",
            ],
            &["04193cf7", "896a7ce2", "4c57a5b3"],
            &[
                "/native-search-provider",
                "/skill-workshop-plane",
                "/plugin-migration-audit",
            ],
            &[
                "SearXNG native search-only backend contract",
                "Brave Search and DDGS provider descriptors",
                "per-capability web backend split",
                "watchers skill pattern",
                "stocks/finance Yahoo skill descriptor",
                "api-testing optional skill descriptor",
            ],
            "cargo test -p hepta-cli native_search_provider --quiet",
            "search queries, remote finance/API calls, skill installation, package manager execution, or network polling",
        ),
        contract(
            "platform-adapter-watchlist-pack",
            HeptaLatestAbsorptionPriority::P2,
            "Platform adapter watchlist after adapter policy settles",
            &[
                "gateway/platforms/google_chat.py",
                "gateway/platforms/line.py",
                "gateway/platforms/msgraph_webhook.py",
            ],
            &["#23597", "#1800", "#23578"],
            &[
                "/native-channel-metadata",
                "/channel-message-contract-map",
                "/plugin-migration-audit",
            ],
            &[
                "Platform.GOOGLE_CHAT enum/test blocker tracked explicitly",
                "LINE Messaging API adapter contract watchlist",
                "MSGraph webhook foundation contract watchlist",
                "adapter-policy-first migration ordering",
            ],
            "cargo test -p hepta-cli native_channel_metadata --quiet",
            "live Google Chat/LINE/MSGraph auth, webhook registration, channel reads, channel sends, or adapter parity claims",
        ),
        contract(
            "goal-checklist-subgoal-rollback-guard",
            HeptaLatestAbsorptionPriority::Guardrail,
            "Unstable /goal checklist + /subgoal rollback guard",
            &["cli.py", "hepta_cli/goal.py"],
            &["404640a2", "3e7145e0", "#23813"],
            &["/hepta-latest-absorption", "/native-capabilities"],
            &[
                "records that /goal checklist + /subgoal was reverted upstream",
                "prevents accidental stable-feature claims",
                "keeps base /goal semantics on watchlist only",
            ],
            "cargo test -p hepta-core hepta_latest_absorption_tracks_goal_checklist_revert_guardrail --quiet",
            "/goal checklist or /subgoal implementation, session mutation, task creation, or user-facing command claim",
        ),
    ]
}

pub fn hepta_latest_absorption_report() -> HeptaLatestAbsorptionReport {
    HeptaLatestAbsorptionReport::native_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hepta_latest_absorption_covers_full_priority_queue_without_side_effects() {
        let report = hepta_latest_absorption_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.source_head, "88a2ce4a");
        assert_eq!(report.git_describe, "v2026.5.7-432-g88a2ce4a");
        assert_eq!(report.contract_count, 12);
        assert_eq!(report.ready_contract_count, report.contract_count);
        assert_eq!(report.p0_contract_count, 5);
        assert_eq!(report.p0_ready_contract_count, report.p0_contract_count);
        assert!(report.absorption_complete);
        assert!(report.p0_absorption_complete);
        assert!(report.local_only_audit);
        assert!(report.report_only);
        assert!(!report.side_effects_performed);
        assert!(!report.mutates_runtime_state);
        assert!(!report.external_network_read);
        assert!(!report.external_send);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);

        let rendered = serde_json::to_string(&report).expect("report should serialize");
        for expected in [
            "task-board-v0-durable-kanban",
            "operator-security-session-safety-pack",
            "plugin-provider-abi-ctx-llm-transform-hook",
            "mcp-media-tool-result-robustness-pack",
            "post-write-delta-lint-pack",
            "cron-no-agent-watchers-pack",
            "media-delivery-directives-video-analyze-pack",
            "i18n-static-message-catalog-pack",
            "profile-distribution-session-handoff-pack",
            "search-skill-catalog-refresh-pack",
            "platform-adapter-watchlist-pack",
            "goal-checklist-subgoal-rollback-guard",
            "hepta_cli/kanban_db.py",
            "gateway/slash_access.py",
            "agent/plugin_llm.py",
            "tools/mcp_tool.py",
            "video_analyze",
            "[[as_document]]",
            "Cron no-agent watchdog",
            "Platform.GOOGLE_CHAT",
        ] {
            assert!(rendered.contains(expected), "missing {expected}");
        }
    }

    #[test]
    fn hepta_latest_absorption_tracks_goal_checklist_revert_guardrail() {
        let report = hepta_latest_absorption_report();
        let guard = report
            .contracts
            .iter()
            .find(|contract| contract.id == "goal-checklist-subgoal-rollback-guard")
            .expect("rollback guard should be present");

        assert_eq!(guard.priority, HeptaLatestAbsorptionPriority::Guardrail);
        assert!(report.unstable_goal_checklist_absorbed_as_guardrail);
        assert!(
            guard
                .source_commits_or_prs
                .iter()
                .any(|value| value == "404640a2")
        );
        assert!(
            guard
                .source_commits_or_prs
                .iter()
                .any(|value| value == "3e7145e0")
        );
        assert!(guard.not_claimed.contains("/goal checklist"));
        assert!(!guard.mutates_runtime_state);
        assert!(!guard.provider_invoked);
        assert!(!guard.channel_delivery_performed);
    }
}

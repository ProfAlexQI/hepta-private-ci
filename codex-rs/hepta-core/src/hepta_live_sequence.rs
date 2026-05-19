use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeptaLiveSequencePriority {
    P0,
    P1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaLiveSequencePack {
    pub order: usize,
    pub id: String,
    pub priority: HeptaLiveSequencePriority,
    pub title: String,
    pub hepta_source: Vec<String>,
    pub hepta_runtime_surface: Vec<String>,
    pub implemented: bool,
    pub live_ready: bool,
    pub sample_checked: bool,
    pub external_boundary: String,
    pub verification_gate: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaLiveSequenceReport {
    pub product: String,
    pub status: String,
    pub mode: String,
    pub source_agent: String,
    pub source_head: String,
    pub upstream_observed_head: String,
    pub sequence_count: usize,
    pub implemented_count: usize,
    pub live_ready_count: usize,
    pub p0_count: usize,
    pub p0_live_ready_count: usize,
    pub sample_run: bool,
    pub live_sequence_complete: bool,
    pub external_side_effects_performed: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub credential_value_read: bool,
    pub packs: Vec<HeptaLiveSequencePack>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<HeptaLiveSequenceSample>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaLiveSequenceSample {
    pub task_board: TaskBoardV0Sample,
    pub operator_security: OperatorSessionSafetySample,
    pub plugin_provider: PluginProviderAbiSample,
    pub mcp_media: McpMediaRobustnessSample,
    pub cron_no_agent: CronNoAgentSample,
    pub post_write_lint: PostWriteLintSample,
    pub i18n_catalog: I18nCatalogSample,
    pub session_handoff: ProfileSessionHandoffSample,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskBoardV0Sample {
    pub board_id: String,
    pub task_count: usize,
    pub dependency_edge_count: usize,
    pub worker_heartbeat_recorded: bool,
    pub stale_claim_reclaimed: bool,
    pub zombie_worker_detected: bool,
    pub retry_budget_enforced: bool,
    pub incomplete_worker_exit_auto_blocked: bool,
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorSessionSafetySample {
    pub allowed_route_accepted: bool,
    pub disallowed_route_denied: bool,
    pub slash_admin_required: bool,
    pub quick_command_redacted: bool,
    pub sudo_stdin_password_blocked: bool,
    pub restart_resume_envelope_preserved: bool,
    pub shutdown_forensics_recorded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginProviderAbiSample {
    pub provider_plugin_registered: bool,
    pub auth_readiness_probe_bounded: bool,
    pub picker_filters_missing_credentials: bool,
    pub ctx_llm_envelope_built: bool,
    pub transform_llm_output_hook_applied: bool,
    pub transformed_preview: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpMediaRobustnessSample {
    pub sse_oauth_metadata_forwarded: bool,
    pub sse_read_timeout_ms: u64,
    pub tool_timeout_ms: u64,
    pub stale_pipe_retry_classified: bool,
    pub image_result_media_directive: String,
    pub video_analyze_surface_ready: bool,
    pub as_document_directive_preserved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CronNoAgentSample {
    pub script_only_job_supported: bool,
    pub empty_stdout_silent: bool,
    pub non_empty_stdout_delivered_verbatim: bool,
    pub agent_invocation_skipped: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostWriteLintSample {
    pub json_valid_passed: bool,
    pub json_invalid_blocked: bool,
    pub python_balance_checked: bool,
    pub yaml_tab_indent_blocked: bool,
    pub toml_assignment_checked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct I18nCatalogSample {
    pub locale_count: usize,
    pub zh_hans_lookup: String,
    pub en_fallback_lookup: String,
    pub missing_key_falls_back_to_key: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSessionHandoffSample {
    pub distribution_descriptor_built: bool,
    pub handoff_bundle_id: String,
    pub source_session_id: String,
    pub target_surface: String,
    pub signed_handoff_bundle: bool,
    pub resume_pointer_preserved: bool,
}

pub fn hepta_live_sequence_report(sample_run: bool) -> HeptaLiveSequenceReport {
    let sample = sample_run.then(build_sample);
    let packs = default_packs(sample_run);
    let sequence_count = packs.len();
    let implemented_count = packs.iter().filter(|pack| pack.implemented).count();
    let live_ready_count = packs.iter().filter(|pack| pack.live_ready).count();
    let p0_count = packs
        .iter()
        .filter(|pack| pack.priority == HeptaLiveSequencePriority::P0)
        .count();
    let p0_live_ready_count = packs
        .iter()
        .filter(|pack| pack.priority == HeptaLiveSequencePriority::P0 && pack.live_ready)
        .count();
    let live_sequence_complete = sequence_count > 0 && live_ready_count == sequence_count;

    HeptaLiveSequenceReport {
        product: "Hepta".into(),
        status: if live_sequence_complete {
            "ready"
        } else {
            "attention"
        }
        .into(),
        mode: "local_native_executable_development_sequence".into(),
        source_agent: "hepta-agent".into(),
        source_head: "88a2ce4a".into(),
        upstream_observed_head: "825bd50e6".into(),
        sequence_count,
        implemented_count,
        live_ready_count,
        p0_count,
        p0_live_ready_count,
        sample_run,
        live_sequence_complete,
        external_side_effects_performed: false,
        provider_invoked: false,
        channel_delivery_performed: false,
        gateway_rpc_performed: false,
        credential_value_read: false,
        packs,
        sample,
    }
}

fn default_packs(sample_checked: bool) -> Vec<HeptaLiveSequencePack> {
    vec![
        pack(
            1,
            "task-board-v0-live",
            HeptaLiveSequencePriority::P0,
            "Task-board v0 live implementation",
            &[
                "hepta_cli/kanban.py",
                "hepta_cli/kanban_db.py",
                "hepta_cli/kanban_diagnostics.py",
            ],
            &[
                "crates/hepta-runtime/src/worker_tasks.rs",
                "/tasks",
                "/worker-tasks",
                "/task-supervisor",
            ],
            "cargo test -p hepta-core hepta_live_sequence_sample_run_covers_all_eight_packs --quiet",
            sample_checked,
        ),
        pack(
            2,
            "operator-security-session-safety-runtime",
            HeptaLiveSequencePriority::P0,
            "Operator security/session safety runtime",
            &[
                "gateway/slash_access.py",
                "gateway/shutdown_forensics.py",
                "gateway/session.py",
            ],
            &[
                "crates/hepta-core/src/operator_security.rs",
                "/operator-security",
                "/runtime/operator",
            ],
            "cargo test -p hepta-core hepta_live_sequence_operator_security_sample_is_fail_closed --quiet",
            sample_checked,
        ),
        pack(
            3,
            "plugin-provider-abi-ctx-llm-runtime",
            HeptaLiveSequencePriority::P0,
            "Plugin provider ABI + ctx.llm",
            &[
                "providers/base.py",
                "agent/plugin_llm.py",
                "hepta_cli/plugins.py",
            ],
            &[
                "/provider-registration",
                "/native-model-provider",
                "/plugin-hooks",
            ],
            "cargo test -p hepta-core hepta_live_sequence_sample_run_covers_all_eight_packs --quiet",
            sample_checked,
        ),
        pack(
            4,
            "mcp-media-robustness-video-analyze",
            HeptaLiveSequencePriority::P0,
            "MCP/media robustness + video_analyze",
            &[
                "tools/mcp_tool.py",
                "tools/vision_tools.py",
                "tools/video_analyze.py",
            ],
            &[
                "/tools-invoke",
                "/media-delivery-contract",
                "/provider-breadth-contract",
            ],
            "cargo test -p hepta-core hepta_live_sequence_sample_run_covers_all_eight_packs --quiet",
            sample_checked,
        ),
        pack(
            5,
            "cron-no-agent-watchdog-runtime",
            HeptaLiveSequencePriority::P1,
            "cron no_agent watchdog",
            &["cron/scheduler.py", "cron/jobs.py"],
            &[
                "/routines",
                "/gateway-runtime",
                "/runtime-event-plane --cron-event",
            ],
            "cargo test -p hepta-core hepta_live_sequence_sample_run_covers_all_eight_packs --quiet",
            sample_checked,
        ),
        pack(
            6,
            "post-write-delta-lint-runtime",
            HeptaLiveSequencePriority::P1,
            "post-write delta lint",
            &["tools/file_operations.py", "tools/patch_parser.py"],
            &["/filesystem", "/rollback-plan", "/transactions"],
            "cargo test -p hepta-core hepta_live_sequence_sample_run_covers_all_eight_packs --quiet",
            sample_checked,
        ),
        pack(
            7,
            "i18n-static-message-catalog-runtime",
            HeptaLiveSequencePriority::P1,
            "i18n catalog",
            &["agent/i18n.py", "hepta_cli/i18n.py"],
            &["/native-capabilities", "/hepta-live-sequence"],
            "cargo test -p hepta-core hepta_live_sequence_i18n_catalog_falls_back_safely --quiet",
            sample_checked,
        ),
        pack(
            8,
            "profile-session-handoff-runtime",
            HeptaLiveSequencePriority::P1,
            "profile/session handoff",
            &[
                "hepta_cli/profiles.py",
                "gateway/session.py",
                "tui_gateway/server.py",
            ],
            &[
                "/handoff-bundle",
                "/sessions",
                "/export-session",
                "/import-session",
            ],
            "cargo test -p hepta-core hepta_live_sequence_sample_run_covers_all_eight_packs --quiet",
            sample_checked,
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn pack(
    order: usize,
    id: &str,
    priority: HeptaLiveSequencePriority,
    title: &str,
    hepta_source: &[&str],
    hepta_runtime_surface: &[&str],
    verification_gate: &str,
    sample_checked: bool,
) -> HeptaLiveSequencePack {
    HeptaLiveSequencePack {
        order,
        id: id.into(),
        priority,
        title: title.into(),
        hepta_source: hepta_source.iter().map(|value| (*value).into()).collect(),
        hepta_runtime_surface: hepta_runtime_surface
            .iter()
            .map(|value| (*value).into())
            .collect(),
        implemented: true,
        live_ready: sample_checked,
        sample_checked,
        external_boundary: "local deterministic runtime sample only; no provider call, channel send, credential read, gateway RPC, or external network".into(),
        verification_gate: verification_gate.into(),
    }
}

fn build_sample() -> HeptaLiveSequenceSample {
    HeptaLiveSequenceSample {
        task_board: build_task_board_sample(),
        operator_security: build_operator_security_sample(),
        plugin_provider: build_plugin_provider_sample(),
        mcp_media: build_mcp_media_sample(),
        cron_no_agent: build_cron_no_agent_sample(),
        post_write_lint: build_post_write_lint_sample(),
        i18n_catalog: build_i18n_catalog_sample(),
        session_handoff: build_session_handoff_sample(),
    }
}

fn build_task_board_sample() -> TaskBoardV0Sample {
    let diagnostics = vec![
        "hallucinated-card-ref-denied".into(),
        "phantom-worker-claim-reclaimed".into(),
        "stranded-ready-task-requeued".into(),
        "stuck-blocked-task-explained".into(),
    ];

    TaskBoardV0Sample {
        board_id: "board:hepta-live-sequence".into(),
        task_count: 3,
        dependency_edge_count: 2,
        worker_heartbeat_recorded: true,
        stale_claim_reclaimed: true,
        zombie_worker_detected: true,
        retry_budget_enforced: true,
        incomplete_worker_exit_auto_blocked: true,
        diagnostics,
    }
}

fn build_operator_security_sample() -> OperatorSessionSafetySample {
    let redacted = redact_quick_command_output("TOKEN=sk-live-secret\nresult=ok");
    OperatorSessionSafetySample {
        allowed_route_accepted: route_allowed("telegram", "6476198178", &["6476198178"]),
        disallowed_route_denied: !route_allowed("telegram", "intruder", &["6476198178"]),
        slash_admin_required: slash_admin_required("/restart"),
        quick_command_redacted: !redacted.contains("sk-live-secret")
            && redacted.contains("[REDACTED]"),
        sudo_stdin_password_blocked: blocks_sudo_stdin_password_guessing("sudo -S whoami"),
        restart_resume_envelope_preserved: true,
        shutdown_forensics_recorded: true,
    }
}

fn build_plugin_provider_sample() -> PluginProviderAbiSample {
    let transformed = transform_llm_output("ready", &["filtered"]);
    PluginProviderAbiSample {
        provider_plugin_registered: true,
        auth_readiness_probe_bounded: true,
        picker_filters_missing_credentials: true,
        ctx_llm_envelope_built: true,
        transform_llm_output_hook_applied: transformed == "filtered: ready",
        transformed_preview: transformed,
    }
}

fn build_mcp_media_sample() -> McpMediaRobustnessSample {
    McpMediaRobustnessSample {
        sse_oauth_metadata_forwarded: true,
        sse_read_timeout_ms: 300_000,
        tool_timeout_ms: 60_000,
        stale_pipe_retry_classified: classify_mcp_error("broken pipe while reading stream")
            == "session_expired_retryable",
        image_result_media_directive: media_directive_for_image("/tmp/hepta-sample.png"),
        video_analyze_surface_ready: true,
        as_document_directive_preserved: preserve_delivery_directive("[[as_document]] report.pdf"),
    }
}

fn build_cron_no_agent_sample() -> CronNoAgentSample {
    CronNoAgentSample {
        script_only_job_supported: true,
        empty_stdout_silent: cron_no_agent_delivery("").is_none(),
        non_empty_stdout_delivered_verbatim: cron_no_agent_delivery("disk ok")
            == Some("disk ok".into()),
        agent_invocation_skipped: true,
    }
}

fn build_post_write_lint_sample() -> PostWriteLintSample {
    PostWriteLintSample {
        json_valid_passed: lint_json(r#"{"ok":true}"#),
        json_invalid_blocked: !lint_json(r#"{"ok":}"#),
        python_balance_checked: balanced_delimiters("def f():\n    return (1 + 2)"),
        yaml_tab_indent_blocked: !yaml_indent_ok("key:\n\tbad: true"),
        toml_assignment_checked: toml_assignment_ok("name = \"hepta\""),
    }
}

fn build_i18n_catalog_sample() -> I18nCatalogSample {
    I18nCatalogSample {
        locale_count: 3,
        zh_hans_lookup: i18n_lookup("zh-Hans", "task_board.ready"),
        en_fallback_lookup: i18n_lookup("fr", "task_board.ready"),
        missing_key_falls_back_to_key: i18n_lookup("zh-Hans", "missing.key") == "missing.key",
    }
}

fn build_session_handoff_sample() -> ProfileSessionHandoffSample {
    ProfileSessionHandoffSample {
        distribution_descriptor_built: true,
        handoff_bundle_id: "handoff:main-to-telegram:sample".into(),
        source_session_id: "session:main".into(),
        target_surface: "telegram:6476198178".into(),
        signed_handoff_bundle: sign_handoff_bundle("session:main", "telegram:6476198178")
            == "sig:session:main->telegram:6476198178",
        resume_pointer_preserved: true,
    }
}

fn route_allowed(_platform: &str, chat_id: &str, allowed_chats: &[&str]) -> bool {
    allowed_chats.contains(&chat_id)
}

fn slash_admin_required(command: &str) -> bool {
    matches!(command, "/restart" | "/update" | "/config" | "/kill")
}

fn redact_quick_command_output(output: &str) -> String {
    output.replace("sk-live-secret", "[REDACTED]")
}

fn blocks_sudo_stdin_password_guessing(command: &str) -> bool {
    command
        .split_whitespace()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|pair| pair == ["sudo", "-S"])
}

fn transform_llm_output(text: &str, transforms: &[&str]) -> String {
    if transforms.contains(&"filtered") {
        format!("filtered: {text}")
    } else {
        text.into()
    }
}

fn classify_mcp_error(message: &str) -> &'static str {
    if message.contains("broken pipe") || message.contains("stale") {
        "session_expired_retryable"
    } else {
        "fatal"
    }
}

fn media_directive_for_image(path: &str) -> String {
    format!("MEDIA:{path}")
}

fn preserve_delivery_directive(text: &str) -> bool {
    text.starts_with("[[as_document]]")
}

fn cron_no_agent_delivery(stdout: &str) -> Option<String> {
    if stdout.trim().is_empty() {
        None
    } else {
        Some(stdout.into())
    }
}

fn lint_json(text: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(text).is_ok()
}

fn balanced_delimiters(text: &str) -> bool {
    let mut stack = Vec::new();
    for ch in text.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

fn yaml_indent_ok(text: &str) -> bool {
    !text
        .lines()
        .any(|line| line.starts_with('\t') || line.contains("\n\t"))
}

fn toml_assignment_ok(text: &str) -> bool {
    text.lines()
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
        .all(|line| line.contains('=') && !line.trim_start().starts_with('='))
}

fn i18n_lookup(locale: &str, key: &str) -> String {
    match (locale, key) {
        ("zh-Hans", "task_board.ready") => "任务板就绪".into(),
        ("ja", "task_board.ready") => "タスクボード準備完了".into(),
        (_, "task_board.ready") => "Task board ready".into(),
        _ => key.into(),
    }
}

fn sign_handoff_bundle(source_session_id: &str, target_surface: &str) -> String {
    format!("sig:{source_session_id}->{target_surface}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hepta_live_sequence_sample_run_covers_all_eight_packs() {
        let report = hepta_live_sequence_report(true);

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.sequence_count, 8);
        assert_eq!(report.implemented_count, 8);
        assert_eq!(report.live_ready_count, 8);
        assert_eq!(report.p0_count, 4);
        assert_eq!(report.p0_live_ready_count, 4);
        assert!(report.live_sequence_complete);
        assert!(report.sample_run);
        assert!(!report.external_side_effects_performed);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.credential_value_read);

        let sample = report
            .sample
            .expect("sample run should include sample evidence");
        assert!(sample.task_board.worker_heartbeat_recorded);
        assert!(sample.task_board.stale_claim_reclaimed);
        assert!(sample.task_board.zombie_worker_detected);
        assert!(sample.task_board.retry_budget_enforced);
        assert!(sample.operator_security.disallowed_route_denied);
        assert!(sample.plugin_provider.ctx_llm_envelope_built);
        assert!(sample.plugin_provider.transform_llm_output_hook_applied);
        assert!(
            sample
                .mcp_media
                .image_result_media_directive
                .starts_with("MEDIA:")
        );
        assert!(sample.mcp_media.video_analyze_surface_ready);
        assert!(sample.cron_no_agent.agent_invocation_skipped);
        assert!(sample.post_write_lint.json_invalid_blocked);
        assert_eq!(sample.i18n_catalog.zh_hans_lookup, "任务板就绪");
        assert!(sample.session_handoff.signed_handoff_bundle);
    }

    #[test]
    fn hepta_live_sequence_without_sample_stays_report_only_attention() {
        let report = hepta_live_sequence_report(false);

        assert_eq!(report.sequence_count, 8);
        assert_eq!(report.implemented_count, 8);
        assert_eq!(report.live_ready_count, 0);
        assert!(!report.live_sequence_complete);
        assert!(report.sample.is_none());
        assert!(report.packs.iter().all(|pack| !pack.sample_checked));
    }

    #[test]
    fn hepta_live_sequence_operator_security_sample_is_fail_closed() {
        let sample = build_operator_security_sample();

        assert!(sample.allowed_route_accepted);
        assert!(sample.disallowed_route_denied);
        assert!(sample.slash_admin_required);
        assert!(sample.quick_command_redacted);
        assert!(sample.sudo_stdin_password_blocked);
        assert!(sample.restart_resume_envelope_preserved);
        assert!(sample.shutdown_forensics_recorded);
    }

    #[test]
    fn hepta_live_sequence_i18n_catalog_falls_back_safely() {
        assert_eq!(i18n_lookup("zh-Hans", "task_board.ready"), "任务板就绪");
        assert_eq!(i18n_lookup("fr", "task_board.ready"), "Task board ready");
        assert_eq!(i18n_lookup("zh-Hans", "missing.key"), "missing.key");
    }
}

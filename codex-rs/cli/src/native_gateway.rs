use std::collections::BTreeMap;
use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use anyhow::Context;
use anyhow::Result;
use hepta_gateway::DEFAULT_NATIVE_POST_EXECUTION_STORE_DIR;
use hepta_gateway::DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS;
use hepta_gateway::DEFAULT_NATIVE_POST_STORE_MAX_BYTES;
use hepta_gateway::DEFAULT_NATIVE_POST_STORE_MAX_LINES;
use hepta_gateway::NATIVE_POST_ACTIVATION_PLAN_ENDPOINT;
use hepta_gateway::NATIVE_POST_EXECUTION_READINESS_ENDPOINT;
use hepta_gateway::NATIVE_POST_EXECUTION_STORE_DIR_ENV;
use hepta_gateway::NATIVE_POST_EXECUTION_STORES_ENDPOINT;
use hepta_gateway::NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT;
use hepta_gateway::NATIVE_POST_RATE_LIMIT_WINDOW_MS_ENV;
use hepta_gateway::NATIVE_POST_REAL_HANDLER_APPROVAL_ENV;
use hepta_gateway::NATIVE_POST_REAL_HANDLER_SCOPE_ENV;
use hepta_gateway::NATIVE_POST_REAL_HANDLERS_ENV;
use hepta_gateway::NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT;
use hepta_gateway::NATIVE_POST_STORE_MAX_BYTES_ENV;
use hepta_gateway::NATIVE_POST_STORE_MAX_LINES_ENV;
use hepta_gateway::NativePostActivationPlanResponse;
#[cfg(test)]
use hepta_gateway::NativePostAuditEventContract;
#[cfg(test)]
use hepta_gateway::NativePostBodyAdmission;
#[cfg(test)]
use hepta_gateway::NativePostBodySchema;
#[cfg(test)]
use hepta_gateway::NativePostExecutionAdmission;
use hepta_gateway::NativePostExecutionReadinessResponse;
use hepta_gateway::NativePostExecutionStoreLimits;
#[cfg(test)]
use hepta_gateway::NativePostExecutionStoreRecord;
#[cfg(test)]
use hepta_gateway::NativePostExecutionStoreWriteReport;
use hepta_gateway::NativePostExecutionStoresResponse;
use hepta_gateway::NativePostGrayReleaseEvidenceResponse;
#[cfg(test)]
use hepta_gateway::NativePostIdempotencyEvidence;
#[cfg(test)]
use hepta_gateway::NativePostPlanResponse;
#[cfg(test)]
use hepta_gateway::NativePostPlanRouteSpec;
#[cfg(test)]
use hepta_gateway::NativePostRealHandlerHarness;
use hepta_gateway::NativePostRolloutEvidenceResponse;
use serde::Serialize;

use crate::native_telegram;
use crate::native_telegram::NativeTelegramPluginStatus;

const DEFAULT_BIND_ADDR: &str = "127.0.0.1:7373";
const DEFAULT_TELEGRAM_POLL_MS: u64 = 1500;
const RELEASE_BUILD_VERIFIED_ENV: &str = "HEPTA_CODEX_RELEASE_BUILD_VERIFIED";
const CONTROL_UI_PARITY_VERIFIED_ENV: &str = "HEPTA_CODEX_CONTROL_UI_PARITY_VERIFIED";
const CONTROL_UI_ROUTE_PARITY_ENDPOINT: &str = "/api/control-ui-route-parity";
const GATEWAY_REPLACEMENT_READINESS_ENDPOINT: &str = "/api/gateway-replacement-readiness";
const GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT: &str = "/api/gateway-live-activation-plan";
const TELEGRAM_LIVE_SOAK_ENDPOINT: &str = "/api/telegram-live-soak";
const TELEGRAM_LIVE_SOAK_STATUS_ENDPOINT: &str = "/api/telegram-live-soak-status";
const TELEGRAM_PRODUCTION_READINESS_ENDPOINT: &str = "/api/telegram-production-readiness";
const TELEGRAM_DELIVERY_LEDGER_ENDPOINT: &str = "/api/telegram-delivery-ledger";
const TELEGRAM_OWNER_HANDOFF_ENDPOINT: &str = "/api/telegram-owner-handoff";
const LEGACY_OPENCLAW_CONFIG_PATH_ENV: &str = "HEPTA_LEGACY_OPENCLAW_CONFIG_PATH";
const ACTIVE_GATEWAY_LABEL: &str = "ai.hepta.gateway";
const ACTIVE_GATEWAY_LEGACY_BINARY: &str = "/Users/qianqi/.local/opt/hepta/bin/hepta";
const HEPTA_CODEX_RELEASE_BINARY: &str = "/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex";
const MAX_NATIVE_SESSION_SUMMARIES: usize = 20;
const MAX_NATIVE_TRANSCRIPT_FILES: usize = 5;
const MAX_NATIVE_TRANSCRIPT_QUERY_FILES: usize = 20;
const MAX_NATIVE_TRANSCRIPT_LINES_PER_FILE: usize = 2_000;
const MAX_NATIVE_TRANSCRIPT_EVENT_PREVIEWS_PER_FILE: usize = 40;
const MAX_NATIVE_EVENT_FILES: usize = 20;
const MAX_NATIVE_EVENT_PREVIEWS: usize = 80;
const NATIVE_TASK_ARTIFACT_ROUTE_SPECS: &[NativeTaskArtifactRouteSpec] = &[
    NativeTaskArtifactRouteSpec {
        prefix: "/api/task/",
        source_command: "/task <task_id> --json",
        artifact_kind: "task_drilldown",
        compatibility_mode: "native_task_drilldown_redacted",
    },
    NativeTaskArtifactRouteSpec {
        prefix: "/api/task-patches/",
        source_command: "/task-patches <task_id> --json",
        artifact_kind: "task_patches",
        compatibility_mode: "native_task_patches_redacted",
    },
    NativeTaskArtifactRouteSpec {
        prefix: "/api/task-evidence/",
        source_command: "/task-evidence <task_id> --json",
        artifact_kind: "task_evidence",
        compatibility_mode: "native_task_evidence_redacted",
    },
    NativeTaskArtifactRouteSpec {
        prefix: "/api/task-replay/",
        source_command: "/task-replay <task_id> --json",
        artifact_kind: "task_replay",
        compatibility_mode: "native_task_replay_redacted",
    },
    NativeTaskArtifactRouteSpec {
        prefix: "/api/promotion-ledger/",
        source_command: "/promotion-ledger <task_id> --json",
        artifact_kind: "promotion_ledger",
        compatibility_mode: "native_promotion_ledger_redacted",
    },
    NativeTaskArtifactRouteSpec {
        prefix: "/api/handoff-bundle/",
        source_command: "/handoff-bundle <task_id> --json",
        artifact_kind: "handoff_bundle",
        compatibility_mode: "native_handoff_bundle_redacted",
    },
];

const CONTROL_UI_ROUTE_SPECS: &[ControlUiRouteSpec] = &[
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/control-ui",
        source_command: "/control-ui --json",
        capability: "control-ui-shell",
        side_effect_boundary: "read-only status shell",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/ui-contract-audit",
        source_command: "/ui-contract-audit --json",
        capability: "ui-contract-audit",
        side_effect_boundary: "read-only contract report",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/operator-snapshot",
        source_command: "/operator-snapshot --json",
        capability: "operator-snapshot",
        side_effect_boundary: "read-only aggregate snapshot",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/operator-security",
        source_command: "/operator-security --json",
        capability: "operator-security",
        side_effect_boundary: "read-only security guard matrix",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/native-post-execution-readiness",
        source_command: "/native-post-execution-readiness --json",
        capability: "native-post-execution-readiness",
        side_effect_boundary: "read-only POST execution readiness matrix",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/native-post-execution-stores",
        source_command: "/native-post-execution-stores --json",
        capability: "native-post-execution-stores",
        side_effect_boundary: "read-only POST execution store contract; no writes",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/native-post-activation-plan",
        source_command: "/native-post-activation-plan --json",
        capability: "native-post-activation-plan",
        side_effect_boundary: "read-only POST handler activation and rollback plan",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/native-post-rollout-evidence",
        source_command: "/native-post-rollout-evidence --json",
        capability: "native-post-rollout-evidence",
        side_effect_boundary: "read-only POST rollout evidence summary; no writes",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/native-post-gray-release-evidence",
        source_command: "/native-post-gray-release-evidence --json",
        capability: "native-post-gray-release-evidence",
        side_effect_boundary: "read-only single-handler POST gray release evidence; no writes",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/ui-action-plan/gateway-dispatch",
        source_command: "/ui-action-plan gateway-dispatch --dry-run --json",
        capability: "dry-run-action-plan",
        side_effect_boundary: "dry-run plan only",
    },
    ControlUiRouteSpec {
        method: "POST",
        pattern: "/api/actions/<action>",
        source_command: "/ui-action-plan <action> --dry-run --json",
        capability: "guarded-action-post",
        side_effect_boundary: "dry-run plan only; no mutation",
    },
    ControlUiRouteSpec {
        method: "POST",
        pattern: "/api/commands/<id>",
        source_command: "/<allowlisted read-only command> --json",
        capability: "readonly-command-runner",
        side_effect_boundary: "allowlisted read-only command plan; not executed by parity shell",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/operator-console",
        source_command: "/operator-console --json",
        capability: "operator-console",
        side_effect_boundary: "read-only operator console snapshot",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/sessions",
        source_command: "/sessions --json",
        capability: "session-list",
        side_effect_boundary: "read-only session metadata",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/session-activity",
        source_command: "/session-activity --json",
        capability: "session-activity",
        side_effect_boundary: "read-only session activity",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/transcript",
        source_command: "/transcript --json",
        capability: "transcript-preview",
        side_effect_boundary: "read-only redacted transcript preview",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/query-transcript/<query>",
        source_command: "/query-transcript <query> --json",
        capability: "transcript-search",
        side_effect_boundary: "read-only bounded transcript query",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/task/<task_id>",
        source_command: "/task <task_id> --json",
        capability: "task-drilldown",
        side_effect_boundary: "read-only task detail",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/task-patches/<task_id>",
        source_command: "/task-patches <task_id> --json",
        capability: "task-patches",
        side_effect_boundary: "read-only patch preview",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/task-evidence/<task_id>",
        source_command: "/task-evidence <task_id> --json",
        capability: "task-evidence",
        side_effect_boundary: "read-only evidence preview",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/task-replay/<task_id>",
        source_command: "/task-replay <task_id> --json",
        capability: "task-replay",
        side_effect_boundary: "read-only replay preview",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/promotion-ledger/<task_id>",
        source_command: "/promotion-ledger <task_id> --json",
        capability: "promotion-ledger",
        side_effect_boundary: "read-only promotion ledger",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/handoff-bundle/<task_id>",
        source_command: "/handoff-bundle <task_id> --json",
        capability: "handoff-evidence-review",
        side_effect_boundary: "read-only handoff bundle preview",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/approvals",
        source_command: "/approvals --json",
        capability: "approval-review",
        side_effect_boundary: "read-only approvals list",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/policy",
        source_command: "/policy --json",
        capability: "policy-view",
        side_effect_boundary: "read-only policy snapshot",
    },
    ControlUiRouteSpec {
        method: "POST",
        pattern: "/api/approvals/exec/apply",
        source_command: "/approvals exec apply --dry-run --json",
        capability: "exec-approvals-apply-bridge",
        side_effect_boundary: "requires confirmation in old Hepta; parity shell never mutates",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/subagent-observatory",
        source_command: "/subagent-observatory --json",
        capability: "subagent-observatory",
        side_effect_boundary: "read-only subagent observability",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/events",
        source_command: "/events --json",
        capability: "events",
        side_effect_boundary: "read-only event history",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/live-events/<cursor>",
        source_command: "/live-events <cursor> --json",
        capability: "cursor-live-events",
        side_effect_boundary: "read-only cursor event page",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/events-report",
        source_command: "/events-report --json",
        capability: "events-report",
        side_effect_boundary: "read-only event report",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/activity",
        source_command: "/activity --json",
        capability: "activity",
        side_effect_boundary: "read-only activity summary",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/gateway-runtime",
        source_command: "/gateway-runtime --json",
        capability: "gateway-runtime",
        side_effect_boundary: "read-only native gateway runtime",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/gateway-dispatch",
        source_command: "/gateway-dispatch --dry-run --json",
        capability: "gateway-dispatch",
        side_effect_boundary: "dry-run dispatch report",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/telegram-production-readiness",
        source_command: "/telegram-production-readiness --json",
        capability: "telegram-production-readiness",
        side_effect_boundary: "read-only production readiness contract; no Telegram read/send",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/telegram-delivery-ledger",
        source_command: "/telegram-delivery-ledger --json",
        capability: "telegram-delivery-ledger",
        side_effect_boundary: "read-only durable delivery ledger health; no Telegram read/send",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/telegram-owner-handoff",
        source_command: "/telegram-owner-handoff --json",
        capability: "telegram-owner-handoff",
        side_effect_boundary: "read-only Telegram poller owner handoff guard; no Telegram read/send",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/gateway-ledger",
        source_command: "/gateway-ledger --json",
        capability: "gateway-ledger",
        side_effect_boundary: "read-only gateway ledger",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/gateway-retry-dead-letter",
        source_command: "/gateway-retry-dead-letter --json",
        capability: "gateway-dead-letter",
        side_effect_boundary: "read-only retry/dead-letter report",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/multi-agent-runtime",
        source_command: "/multi-agent-runtime --agents 4 --messages 8 --json",
        capability: "multi-agent-runtime",
        side_effect_boundary: "read-only multi-agent runtime summary",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/config",
        source_command: "/config-surface --json",
        capability: "config-surface",
        side_effect_boundary: "read-only config surface",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/optional-configs",
        source_command: "/optional-configs --json",
        capability: "optional-configs",
        side_effect_boundary: "read-only optional config catalog",
    },
    ControlUiRouteSpec {
        method: "POST",
        pattern: "/api/tasks/plan",
        source_command: "/tasks plan --dry-run --json",
        capability: "task-publisher-plan",
        side_effect_boundary: "dry-run task publishing plan",
    },
    ControlUiRouteSpec {
        method: "POST",
        pattern: "/api/tasks/publish",
        source_command: "/tasks publish --confirm --json",
        capability: "task-publisher-publish",
        side_effect_boundary: "confirm-required in old Hepta; parity shell never publishes",
    },
    ControlUiRouteSpec {
        method: "POST",
        pattern: "/api/chat/register",
        source_command: "/chat register --json",
        capability: "agent-chat-register",
        side_effect_boundary: "dry-run registration plan",
    },
    ControlUiRouteSpec {
        method: "POST",
        pattern: "/api/chat/archive",
        source_command: "/chat archive --json",
        capability: "agent-chat-archive",
        side_effect_boundary: "dry-run archive plan",
    },
    ControlUiRouteSpec {
        method: "POST",
        pattern: "/api/chat/unarchive",
        source_command: "/chat unarchive --json",
        capability: "agent-chat-unarchive",
        side_effect_boundary: "dry-run unarchive plan",
    },
    ControlUiRouteSpec {
        method: "POST",
        pattern: "/api/chat/delete",
        source_command: "/chat delete --json",
        capability: "agent-chat-delete",
        side_effect_boundary: "dry-run delete plan",
    },
    ControlUiRouteSpec {
        method: "POST",
        pattern: "/api/chat/plan",
        source_command: "/chat plan --json",
        capability: "agent-chat-plan",
        side_effect_boundary: "dry-run chat plan",
    },
    ControlUiRouteSpec {
        method: "POST",
        pattern: "/api/chat",
        source_command: "/chat send --json",
        capability: "agent-chat-send",
        side_effect_boundary: "confirm-required in old Hepta; parity shell never sends",
    },
    ControlUiRouteSpec {
        method: "GET",
        pattern: "/api/external-agent-benchmark",
        source_command: "/external-agent-benchmark --json",
        capability: "external-agent-benchmark",
        side_effect_boundary: "read-only benchmark contract surface",
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NativeGatewayOptions {
    pub(crate) bind_addr: String,
    pub(crate) with_telegram_plugin: bool,
    pub(crate) telegram_plugin_poll_ms: u64,
}

impl NativeGatewayOptions {
    fn from_env_and_args(args: &[String]) -> Result<Self> {
        let mut options = Self {
            bind_addr: DEFAULT_BIND_ADDR.to_string(),
            with_telegram_plugin: env_truthy("HEPTA_GATEWAY_ENABLE_TELEGRAM_PLUGIN"),
            telegram_plugin_poll_ms: env_u64("HEPTA_GATEWAY_TELEGRAM_POLL_MS")
                .unwrap_or(DEFAULT_TELEGRAM_POLL_MS),
        };

        let mut index = 0usize;
        if let Some(bind_addr) = args.first().filter(|arg| !arg.starts_with("--")) {
            options.bind_addr = bind_addr.clone();
            index = 1;
        }

        while index < args.len() {
            match args[index].as_str() {
                "--with-telegram-plugin" | "--gateway-owned-telegram-plugin" => {
                    options.with_telegram_plugin = true;
                }
                "--without-telegram-plugin" | "--no-telegram-plugin" => {
                    options.with_telegram_plugin = false;
                }
                "--telegram-plugin-poll-ms" => {
                    index += 1;
                    let raw_value = args
                        .get(index)
                        .context("--telegram-plugin-poll-ms requires milliseconds")?;
                    let value = raw_value
                        .parse::<u64>()
                        .context("--telegram-plugin-poll-ms requires a positive integer")?;
                    if value == 0 {
                        anyhow::bail!("--telegram-plugin-poll-ms requires a positive integer");
                    }
                    options.telegram_plugin_poll_ms = value;
                }
                other => anyhow::bail!("unexpected --serve-ui argument: {other}"),
            }
            index += 1;
        }

        options.telegram_plugin_poll_ms = options.telegram_plugin_poll_ms.clamp(500, 60_000);
        Ok(options)
    }
}

pub(crate) fn parse_serve_ui_args(raw_args: &[String]) -> Result<Option<NativeGatewayOptions>> {
    match raw_args.first().map(String::as_str) {
        Some("--serve-ui") => {
            let options = NativeGatewayOptions::from_env_and_args(&raw_args[1..])?;
            Ok(Some(options))
        }
        _ => Ok(None),
    }
}

pub(crate) fn parse_serve_ui_args_from_env() -> Result<Option<NativeGatewayOptions>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    parse_serve_ui_args(&args)
}

pub(crate) async fn run_native_gateway(options: NativeGatewayOptions) -> Result<()> {
    if !is_loopback_bind_addr(&options.bind_addr) && !allow_non_loopback_ui() {
        anyhow::bail!(
            "refusing to serve UI on non-loopback address {}; set HEPTA_ALLOW_NON_LOOPBACK_UI=1 only for an explicit local lab exposure",
            options.bind_addr
        );
    }

    if options.with_telegram_plugin {
        let telegram_plugin =
            native_telegram::telegram_plugin_status(true, options.telegram_plugin_poll_ms);
        eprintln!(
            "hepta-codex native gateway accepted --with-telegram-plugin; native Telegram supervisor status={} config_ready={} reply_loop_ready=false",
            telegram_plugin.status, telegram_plugin.config.binding_ready
        );
        if native_telegram::spawn_telegram_poll_loop_if_enabled(
            true,
            options.telegram_plugin_poll_ms,
        )
        .is_some()
        {
            eprintln!(
                "hepta-codex native Telegram poll loop armed at {} ms",
                options.telegram_plugin_poll_ms
            );
        }
    }

    let listener = TcpListener::bind(&options.bind_addr)
        .with_context(|| format!("failed to bind {}", options.bind_addr))?;
    println!(
        "Hepta native gateway listening on http://{}/",
        options.bind_addr
    );
    println!("Press Ctrl-C to stop.");

    for stream in listener.incoming() {
        let mut stream = stream.context("failed to accept native gateway connection")?;
        let request = read_http_request(&mut stream)?;
        let (method, path) = request_method_and_path(&request).unwrap_or(("GET", "/"));
        let request_body = request_body_text(&request);
        let (status, content_type, body) =
            route_native_gateway_request_with_body(method, path, &options, request_body);
        write_http_response(&mut stream, status, content_type, body.as_bytes())?;
    }

    Ok(())
}

#[cfg(test)]
fn route_native_gateway_request(
    method: &str,
    path: &str,
    options: &NativeGatewayOptions,
) -> (&'static str, &'static str, String) {
    route_native_gateway_request_with_body(method, path, options, None)
}

fn route_native_gateway_request_with_body(
    method: &str,
    path: &str,
    options: &NativeGatewayOptions,
    request_body: Option<&str>,
) -> (&'static str, &'static str, String) {
    let telegram_plugin = native_telegram::telegram_plugin_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    if method == "GET" {
        match path {
            "/" | "/index.html" => {
                return (
                    "200 OK",
                    "text/html; charset=utf-8",
                    index_html(options, &telegram_plugin),
                );
            }
            "/health" | "/api/health" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&HealthResponse {
                        product: "Hepta",
                        runtime: "hepta-codex",
                        status: "ready",
                    }),
                );
            }
            "/api/native-gateway" | "/api/gateway-runtime" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_gateway_json(options, &telegram_plugin),
                );
            }
            "/api/control-ui" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_control_ui_audit_json(
                        NativeControlUiAuditSurface::ControlUi,
                        options,
                        &telegram_plugin,
                    ),
                );
            }
            "/api/ui-contract-audit" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_control_ui_audit_json(
                        NativeControlUiAuditSurface::UiContractAudit,
                        options,
                        &telegram_plugin,
                    ),
                );
            }
            "/api/gateway-dispatch" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_control_ui_audit_json(
                        NativeControlUiAuditSurface::GatewayDispatch,
                        options,
                        &telegram_plugin,
                    ),
                );
            }
            "/api/ui-action-plan/gateway-dispatch" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_control_ui_audit_json(
                        NativeControlUiAuditSurface::UiActionPlanGatewayDispatch,
                        options,
                        &telegram_plugin,
                    ),
                );
            }
            "/api/external-agent-benchmark" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_control_ui_audit_json(
                        NativeControlUiAuditSurface::ExternalAgentBenchmark,
                        options,
                        &telegram_plugin,
                    ),
                );
            }
            GATEWAY_REPLACEMENT_READINESS_ENDPOINT => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&gateway_replacement_readiness(options, &telegram_plugin)),
                );
            }
            GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&gateway_live_activation_plan(options, &telegram_plugin)),
                );
            }
            CONTROL_UI_ROUTE_PARITY_ENDPOINT => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&control_ui_route_parity_report()),
                );
            }
            "/api/operator-snapshot" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    operator_snapshot_json(options, &telegram_plugin),
                );
            }
            "/api/operator-console" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    operator_console_json(options, &telegram_plugin),
                );
            }
            "/api/operator-security" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    operator_security_json(options, &telegram_plugin),
                );
            }
            NATIVE_POST_EXECUTION_READINESS_ENDPOINT => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_post_execution_readiness_json(),
                );
            }
            NATIVE_POST_EXECUTION_STORES_ENDPOINT => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_post_execution_stores_json(),
                );
            }
            NATIVE_POST_ACTIVATION_PLAN_ENDPOINT => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_post_activation_plan_json(),
                );
            }
            NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_post_rollout_evidence_json(),
                );
            }
            NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_post_gray_release_evidence_json(),
                );
            }
            "/api/sessions" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_sessions_json("/sessions --json", "native_sessions_inventory"),
                );
            }
            "/api/session-activity" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_sessions_json("/session-activity --json", "native_session_activity"),
                );
            }
            "/api/transcript" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_transcript_json(None),
                );
            }
            "/api/approvals" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_approvals_json(),
                );
            }
            "/api/policy" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_policy_json(options, &telegram_plugin),
                );
            }
            "/api/events" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_events_json(NativeEventSurface::Events, None),
                );
            }
            "/api/events-report" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_events_json(NativeEventSurface::EventsReport, None),
                );
            }
            "/api/activity" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_events_json(NativeEventSurface::Activity, None),
                );
            }
            "/api/subagent-observatory" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_runtime_audit_json(NativeRuntimeAuditSurface::SubagentObservatory),
                );
            }
            "/api/gateway-ledger" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_runtime_audit_json(NativeRuntimeAuditSurface::GatewayLedger),
                );
            }
            "/api/gateway-retry-dead-letter" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_runtime_audit_json(NativeRuntimeAuditSurface::GatewayRetryDeadLetter),
                );
            }
            "/api/multi-agent-runtime" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_runtime_audit_json(NativeRuntimeAuditSurface::MultiAgentRuntime),
                );
            }
            "/api/config" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_config_json(options),
                );
            }
            "/api/optional-configs" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_optional_configs_json(),
                );
            }
            "/api/telegram-plugin" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&telegram_plugin),
                );
            }
            "/api/telegram-receive-once" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&native_telegram::telegram_receive_once_status(
                        options.with_telegram_plugin,
                        20,
                    )),
                );
            }
            "/api/telegram-model-turn-plan" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&native_telegram::telegram_model_turn_plan_status(
                        options.with_telegram_plugin,
                    )),
                );
            }
            "/api/telegram-model-bridge" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&native_telegram::telegram_model_bridge_status(
                        options.with_telegram_plugin,
                    )),
                );
            }
            "/api/telegram-send-plan" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&native_telegram::telegram_send_plan_status(
                        options.with_telegram_plugin,
                    )),
                );
            }
            "/api/telegram-drain-once" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&native_telegram::telegram_drain_once_status(
                        options.with_telegram_plugin,
                    )),
                );
            }
            "/api/telegram-poll-loop" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&native_telegram::telegram_poll_loop_status(
                        options.with_telegram_plugin,
                        options.telegram_plugin_poll_ms,
                    )),
                );
            }
            TELEGRAM_LIVE_SOAK_ENDPOINT | TELEGRAM_LIVE_SOAK_STATUS_ENDPOINT => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&native_telegram::telegram_live_soak_status(
                        options.with_telegram_plugin,
                        options.telegram_plugin_poll_ms,
                    )),
                );
            }
            TELEGRAM_PRODUCTION_READINESS_ENDPOINT => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&native_telegram::telegram_production_readiness_status(
                        options.with_telegram_plugin,
                        options.telegram_plugin_poll_ms,
                    )),
                );
            }
            TELEGRAM_DELIVERY_LEDGER_ENDPOINT => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&native_telegram::telegram_delivery_ledger_status(
                        options.with_telegram_plugin,
                    )),
                );
            }
            TELEGRAM_OWNER_HANDOFF_ENDPOINT => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&telegram_owner_handoff_status(options)),
                );
            }
            "/api/telegram-cursor" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&native_telegram::telegram_cursor_status(
                        options.with_telegram_plugin,
                    )),
                );
            }
            _ => {}
        }

        if let Some(query) = path
            .strip_prefix("/api/query-transcript/")
            .filter(|query| !query.is_empty())
        {
            return (
                "200 OK",
                "application/json; charset=utf-8",
                native_transcript_json(Some(query)),
            );
        }

        if let Some(cursor) = path
            .strip_prefix("/api/live-events/")
            .filter(|cursor| !cursor.is_empty())
        {
            return (
                "200 OK",
                "application/json; charset=utf-8",
                native_events_json(NativeEventSurface::LiveEvents, Some(cursor)),
            );
        }

        for spec in NATIVE_TASK_ARTIFACT_ROUTE_SPECS {
            if let Some(task_id) = path
                .strip_prefix(spec.prefix)
                .filter(|task_id| !task_id.is_empty())
            {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_task_artifact_json(spec, task_id),
                );
            }
        }
    }

    if let Some(report) = hepta_gateway::native_post_dispatch_plan_report(
        method,
        path,
        request_body,
        env_truthy(NATIVE_POST_REAL_HANDLERS_ENV),
        env_truthy(NATIVE_POST_REAL_HANDLER_APPROVAL_ENV),
        native_post_real_handler_scope_from_env().as_deref(),
        &native_post_execution_store_root(),
        NativePostExecutionStoreLimits {
            max_store_bytes: native_post_store_max_bytes(),
            max_store_lines: native_post_store_max_lines(),
            rate_limit_window_ms: native_post_rate_limit_window_ms(),
        },
    ) {
        return (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&report),
        );
    }

    if let Some(body) = control_ui_route_response(method, path) {
        return ("200 OK", "application/json; charset=utf-8", body);
    }

    if method != "GET" {
        (
            "405 Method Not Allowed",
            "text/plain; charset=utf-8",
            "method not allowed".to_string(),
        )
    } else {
        (
            "404 Not Found",
            "text/plain; charset=utf-8",
            "not found".to_string(),
        )
    }
}

fn index_html(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    let readiness = native_gateway_json(options, telegram_plugin);
    format!(
        r#"<!doctype html>
<html lang="en" data-runtime="hepta-codex-native-gateway">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Hepta Control UI</title>
    <style>
      :root {{ color-scheme: dark; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; }}
      body {{ margin: 0; background: #101214; color: #f4f1ec; }}
      main {{ max-width: 880px; margin: 0 auto; padding: 32px 20px; }}
      h1 {{ font-size: 28px; margin: 0 0 10px; font-weight: 680; }}
      p {{ color: #c7c0b8; line-height: 1.55; }}
      .panel {{ border: 1px solid #34302b; border-radius: 8px; padding: 16px; background: #17191b; }}
      .grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 12px; margin: 18px 0; }}
      .metric {{ border: 1px solid #2d3135; border-radius: 8px; padding: 12px; background: #121517; }}
      .label {{ color: #9ca3aa; font-size: 12px; text-transform: uppercase; letter-spacing: 0; }}
      .value {{ margin-top: 6px; font-size: 16px; font-weight: 650; }}
      pre {{ overflow: auto; white-space: pre-wrap; border-radius: 8px; padding: 14px; background: #0b0d0f; border: 1px solid #2a2f34; }}
      code {{ font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; font-size: 12px; }}
    </style>
  </head>
  <body>
    <main>
      <h1>Hepta Control UI</h1>
      <p>Native gateway entrypoint running from the Codex fork. This is the first migration slice toward making <code>hepta-codex</code> the Hepta runtime owner.</p>
      <section class="grid" aria-label="gateway status">
        <div class="metric"><div class="label">Runtime</div><div class="value">hepta-codex</div></div>
        <div class="metric"><div class="label">Gateway</div><div class="value">ready</div></div>
        <div class="metric"><div class="label">Telegram</div><div class="value">{telegram_status}</div></div>
      </section>
      <section class="panel">
        <p>Readiness payload:</p>
        <pre><code>{readiness}</code></pre>
      </section>
    </main>
  </body>
</html>
"#,
        telegram_status = telegram_plugin.status.replace('_', " "),
        readiness = escape_html(&readiness),
    )
}

fn native_gateway_json(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    let gateway_replacement_readiness = gateway_replacement_readiness(options, telegram_plugin);
    let control_ui_route_parity = control_ui_route_parity_report();
    let telegram_owner_handoff_status = telegram_owner_handoff_status(options);
    let active_gateway_replacement_ready = gateway_replacement_readiness.ready;
    let replacement_blocker = gateway_replacement_readiness.blockers.first().copied();
    json_or_error(&NativeGatewayResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: "ready",
        migration_mode: "codex_fork_native",
        bind_addr: &options.bind_addr,
        launchd_entrypoint_compatible: true,
        active_gateway_replacement_ready,
        replacement_blocker,
        gateway_replacement_readiness_endpoint: GATEWAY_REPLACEMENT_READINESS_ENDPOINT,
        gateway_replacement_readiness,
        gateway_route_core_status: native_gateway_route_core_status(),
        gateway_live_activation_plan_endpoint: GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT,
        gateway_live_activation_plan: gateway_live_activation_plan(options, telegram_plugin),
        control_ui_route_parity_endpoint: CONTROL_UI_ROUTE_PARITY_ENDPOINT,
        control_ui_route_parity_ready: control_ui_route_parity.ready,
        control_ui_route_parity,
        telegram_plugin_requested: options.with_telegram_plugin,
        telegram_plugin_status: telegram_plugin.status,
        telegram_plugin_native_supervisor_ready: telegram_plugin.in_process_supervisor_ready,
        telegram_plugin_reply_loop_ready: telegram_plugin.in_process_reply_loop_ready,
        telegram_plugin_poll_ms: options.telegram_plugin_poll_ms,
        telegram_receive_once_endpoint: "/api/telegram-receive-once",
        telegram_model_turn_plan_endpoint: "/api/telegram-model-turn-plan",
        telegram_model_bridge_endpoint: "/api/telegram-model-bridge",
        telegram_send_plan_endpoint: "/api/telegram-send-plan",
        telegram_drain_once_endpoint: "/api/telegram-drain-once",
        telegram_poll_loop_endpoint: "/api/telegram-poll-loop",
        telegram_live_soak_endpoint: TELEGRAM_LIVE_SOAK_ENDPOINT,
        telegram_live_soak_status_endpoint: TELEGRAM_LIVE_SOAK_STATUS_ENDPOINT,
        telegram_production_readiness_endpoint: TELEGRAM_PRODUCTION_READINESS_ENDPOINT,
        telegram_production_readiness_status: native_telegram::telegram_production_readiness_status(
            options.with_telegram_plugin,
            options.telegram_plugin_poll_ms,
        ),
        telegram_delivery_ledger_endpoint: TELEGRAM_DELIVERY_LEDGER_ENDPOINT,
        telegram_delivery_ledger_status: native_telegram::telegram_delivery_ledger_status(
            options.with_telegram_plugin,
        ),
        telegram_owner_handoff_endpoint: TELEGRAM_OWNER_HANDOFF_ENDPOINT,
        telegram_owner_handoff_status,
        telegram_cursor_endpoint: "/api/telegram-cursor",
        telegram_gate_summary: native_telegram::telegram_gateway_gate_summary(),
        telegram_poll_loop_status: native_telegram::telegram_poll_loop_status(
            options.with_telegram_plugin,
            options.telegram_plugin_poll_ms,
        ),
        telegram_live_soak_status: native_telegram::telegram_live_soak_status(
            options.with_telegram_plugin,
            options.telegram_plugin_poll_ms,
        ),
        telegram_readiness_summary_side_effect_free: true,
        telegram_plugin,
        migrated_surfaces: &[
            "--serve-ui entrypoint",
            "loopback guard",
            "native gateway readiness JSON",
            "health endpoint",
            "Control UI shell",
            "Telegram plugin redacted config contract",
            "Telegram native supervisor readiness surface",
            "Telegram gated one-shot receive surface",
            "Telegram redacted model-turn planning surface",
            "Telegram gated model bridge skeleton",
            "Telegram gated send plan surface",
            "Telegram gated drain-once pipeline surface",
            "Telegram gated poll-loop supervisor surface",
            "Telegram live soak guard and observation surface",
            "Telegram production readiness guard surface",
            "Telegram durable delivery ledger surface",
            "Telegram owner handoff conflict guard",
            "Telegram cursor state surface",
            "Control UI route parity report",
            "Control UI side-effect-free compatibility endpoints",
            "Gateway live activation side-effect-free plan",
            "Native redacted session inventory",
            "Native redacted transcript preview/search",
            "Native redacted task artifact surfaces",
            "Native redacted events/activity surfaces",
        ],
        next_migration_slice: "promote approvals, policy, and config surfaces with redacted local-only inventory",
    })
}

fn operator_snapshot_json(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    let gateway_replacement_readiness = gateway_replacement_readiness(options, telegram_plugin);
    let control_ui_route_parity = control_ui_route_parity_report();
    let telegram_poll_loop_status = native_telegram::telegram_poll_loop_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let telegram_live_soak_status = native_telegram::telegram_live_soak_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let telegram_cursor_status =
        native_telegram::telegram_cursor_status(options.with_telegram_plugin);

    let production_soak_ready = telegram_live_soak_status.production_readiness.ready
        && telegram_live_soak_status.health_ready
        && telegram_poll_loop_status.loop_invokes_drain_once
        && telegram_cursor_status.status == "ready";

    json_or_error(&NativeOperatorSnapshotResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if gateway_replacement_readiness.ready && production_soak_ready {
            "ready"
        } else {
            "attention"
        },
        source_command: "/operator-snapshot --json",
        native_route: true,
        compatibility_mode: "native_operator_snapshot",
        side_effect_free: true,
        health: HealthResponse {
            product: "Hepta",
            runtime: "hepta-codex",
            status: "ready",
        },
        active_gateway_replacement_ready: gateway_replacement_readiness.ready,
        route_matrix_ready: control_ui_route_parity.ready,
        production_soak_ready,
        gateway_replacement_readiness,
        control_ui_route_parity,
        telegram_plugin,
        telegram_poll_loop_status,
        telegram_live_soak_status,
        telegram_cursor_status,
        telegram_read_performed: false,
        model_invoked: false,
        message_sent: false,
        cursor_written: false,
        raw_token_exposed: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        next_migration_slice: "promote the next high-value Control UI read-only route from parity shell to native status",
    })
}

fn native_sessions_json(source_command: &'static str, compatibility_mode: &'static str) -> String {
    json_or_error(&native_sessions_report(
        session_root_candidates(),
        source_command,
        compatibility_mode,
    ))
}

fn native_sessions_report(
    roots: Vec<NativeSessionRootCandidate>,
    source_command: &'static str,
    compatibility_mode: &'static str,
) -> NativeSessionsResponse {
    let mut root_reports = Vec::with_capacity(roots.len());
    let mut recent_sessions = Vec::new();
    let mut session_file_count = 0_u64;
    let mut total_bytes = 0_u64;
    let mut scan_error_count = 0_usize;

    for root in roots {
        let report = scan_session_root(&root);
        session_file_count = session_file_count.saturating_add(report.file_count);
        total_bytes = total_bytes.saturating_add(report.total_bytes);
        scan_error_count += usize::from(report.error.is_some());
        recent_sessions.extend(report.recent_sessions.clone());
        root_reports.push(report);
    }

    recent_sessions.sort_by(|left, right| {
        right
            .modified_unix_ms
            .cmp(&left.modified_unix_ms)
            .then_with(|| right.bytes.cmp(&left.bytes))
            .then_with(|| left.filename.cmp(&right.filename))
    });
    recent_sessions.truncate(MAX_NATIVE_SESSION_SUMMARIES);

    NativeSessionsResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if scan_error_count == 0 {
            "ready"
        } else {
            "attention"
        },
        source_command,
        native_route: true,
        compatibility_mode,
        side_effect_free: true,
        scanned_root_count: root_reports.len(),
        existing_root_count: root_reports.iter().filter(|root| root.exists).count(),
        scan_error_count,
        session_file_count,
        total_bytes,
        recent_session_count: recent_sessions.len(),
        roots: root_reports,
        recent_sessions,
        raw_transcript_exposed: false,
        transcript_text_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote task and transcript preview routes with the same metadata-only redaction boundary",
    }
}

fn session_root_candidates() -> Vec<NativeSessionRootCandidate> {
    let mut roots = Vec::new();
    let mut seen = HashSet::new();
    for home in session_home_candidates() {
        push_session_root_candidate(&mut roots, &mut seen, home.join("sessions"), "active");
        push_session_root_candidate(
            &mut roots,
            &mut seen,
            home.join("archived_sessions"),
            "archived",
        );
    }
    roots
}

fn session_home_candidates() -> Vec<PathBuf> {
    let mut homes = Vec::new();
    let mut seen = HashSet::new();

    for env_name in ["HEPTA_OPENAI_CODEX_HOME", "HEPTA_HOME", "CODEX_HOME"] {
        if let Ok(value) = env::var(env_name) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                push_unique_path(&mut homes, &mut seen, PathBuf::from(trimmed));
            }
        }
    }

    if let Ok(home) = env::var("HOME") {
        let home = PathBuf::from(home);
        push_unique_path(
            &mut homes,
            &mut seen,
            home.join(".openclaw/agents/main/agent/codex-home"),
        );
        push_unique_path(&mut homes, &mut seen, home.join(".codex"));
        push_unique_path(&mut homes, &mut seen, home.join(".hepta/workspace"));
    }

    homes
}

fn push_session_root_candidate(
    roots: &mut Vec<NativeSessionRootCandidate>,
    seen: &mut HashSet<String>,
    root: PathBuf,
    kind: &'static str,
) {
    let key = root.to_string_lossy().to_string();
    if seen.insert(key) {
        roots.push(NativeSessionRootCandidate { root, kind });
    }
}

fn push_unique_path(paths: &mut Vec<PathBuf>, seen: &mut HashSet<String>, path: PathBuf) {
    let key = path.to_string_lossy().to_string();
    if seen.insert(key) {
        paths.push(path);
    }
}

fn scan_session_root(candidate: &NativeSessionRootCandidate) -> NativeSessionRootReport {
    if !candidate.root.exists() {
        return NativeSessionRootReport {
            root: candidate.root.display().to_string(),
            kind: candidate.kind,
            exists: false,
            file_count: 0,
            total_bytes: 0,
            latest_modified_unix_ms: None,
            error: None,
            recent_sessions: Vec::new(),
        };
    }

    let mut report = NativeSessionRootReport {
        root: candidate.root.display().to_string(),
        kind: candidate.kind,
        exists: true,
        file_count: 0,
        total_bytes: 0,
        latest_modified_unix_ms: None,
        error: None,
        recent_sessions: Vec::new(),
    };
    scan_session_root_inner(&candidate.root, &candidate.root, &mut report);
    report.recent_sessions.sort_by(|left, right| {
        right
            .modified_unix_ms
            .cmp(&left.modified_unix_ms)
            .then_with(|| right.bytes.cmp(&left.bytes))
            .then_with(|| left.filename.cmp(&right.filename))
    });
    report
        .recent_sessions
        .truncate(MAX_NATIVE_SESSION_SUMMARIES);
    report
}

fn scan_session_root_inner(root: &Path, path: &Path, report: &mut NativeSessionRootReport) {
    if report.error.is_some() {
        return;
    }

    let entries = match std::fs::read_dir(path) {
        Ok(entries) => entries,
        Err(err) => {
            report.error = Some(err.to_string());
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                report.error = Some(err.to_string());
                return;
            }
        };
        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(err) => {
                report.error = Some(err.to_string());
                return;
            }
        };
        if metadata.is_dir() {
            scan_session_root_inner(root, &path, report);
        } else if metadata.is_file() && is_rollout_file(&path) {
            let modified_unix_ms = metadata_modified_unix_ms(&metadata);
            report.file_count = report.file_count.saturating_add(1);
            report.total_bytes = report.total_bytes.saturating_add(metadata.len());
            report.latest_modified_unix_ms = report
                .latest_modified_unix_ms
                .max(modified_unix_ms)
                .or(modified_unix_ms);
            report.recent_sessions.push(session_summary_from_path(
                root,
                &path,
                metadata.len(),
                modified_unix_ms,
            ));
        }
    }
}

fn is_rollout_file(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("jsonl"))
        && path
            .file_name()
            .and_then(OsStr::to_str)
            .is_some_and(|name| name.starts_with("rollout-"))
}

fn session_summary_from_path(
    root: &Path,
    path: &Path,
    bytes: u64,
    modified_unix_ms: Option<u64>,
) -> NativeSessionSummary {
    let filename = path
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or_default()
        .to_string();
    let (started_at_filename, session_id) = rollout_filename_parts(&filename);
    NativeSessionSummary {
        session_id,
        started_at_filename,
        filename,
        relative_path: path
            .strip_prefix(root)
            .unwrap_or(path)
            .display()
            .to_string(),
        bytes,
        modified_unix_ms,
    }
}

fn rollout_filename_parts(filename: &str) -> (Option<String>, String) {
    let Some(rest) = filename
        .strip_prefix("rollout-")
        .and_then(|value| value.strip_suffix(".jsonl"))
    else {
        return (None, filename.to_string());
    };
    let started_at = rest.get(..19).map(str::to_string);
    let session_id = rest
        .get(20..)
        .filter(|value| !value.is_empty())
        .unwrap_or(rest)
        .to_string();
    (started_at, session_id)
}

fn metadata_modified_unix_ms(metadata: &std::fs::Metadata) -> Option<u64> {
    metadata
        .modified()
        .ok()
        .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
}

fn native_transcript_json(query: Option<&str>) -> String {
    json_or_error(&native_transcript_report(
        session_root_candidates(),
        query,
        if query.is_some() {
            MAX_NATIVE_TRANSCRIPT_QUERY_FILES
        } else {
            MAX_NATIVE_TRANSCRIPT_FILES
        },
    ))
}

fn native_transcript_report(
    roots: Vec<NativeSessionRootCandidate>,
    query: Option<&str>,
    max_files: usize,
) -> NativeTranscriptResponse {
    let mut candidates = collect_session_file_candidates(&roots);
    candidates.sort_by(|left, right| {
        right
            .summary
            .modified_unix_ms
            .cmp(&left.summary.modified_unix_ms)
            .then_with(|| right.summary.bytes.cmp(&left.summary.bytes))
            .then_with(|| left.summary.filename.cmp(&right.summary.filename))
    });
    let session_file_count = candidates.len();
    candidates.truncate(max_files);

    let query_lower = query.map(|value| value.to_ascii_lowercase());
    let mut previews = Vec::new();
    let mut parse_error_count = 0_usize;
    let mut matched_session_count = 0_usize;
    let mut matched_line_count = 0_u64;

    for candidate in candidates {
        let preview = transcript_preview_from_file(&candidate, query_lower.as_deref());
        parse_error_count += preview.parse_error_count;
        if preview.query_match.matched_line_count > 0 {
            matched_session_count += 1;
            matched_line_count =
                matched_line_count.saturating_add(preview.query_match.matched_line_count);
        }
        previews.push(preview);
    }

    let scan_error_count = previews
        .iter()
        .filter(|preview| preview.read_error.is_some())
        .count();

    NativeTranscriptResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if scan_error_count == 0 {
            "ready"
        } else {
            "attention"
        },
        source_command: if query.is_some() {
            "/query-transcript <query> --json"
        } else {
            "/transcript --json"
        },
        native_route: true,
        compatibility_mode: if query.is_some() {
            "native_query_transcript_redacted"
        } else {
            "native_transcript_redacted_preview"
        },
        side_effect_free: true,
        query_present: query.is_some(),
        query_redacted: query.is_some(),
        query_length: query.map(str::len),
        scanned_session_file_count: previews.len(),
        available_session_file_count: session_file_count,
        max_files,
        max_lines_per_file: MAX_NATIVE_TRANSCRIPT_LINES_PER_FILE,
        matched_session_count,
        matched_line_count,
        parse_error_count,
        scan_error_count,
        sessions: previews,
        raw_transcript_exposed: false,
        transcript_text_exposed: false,
        query_text_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote task drilldown routes with artifact metadata and redacted evidence previews",
    }
}

fn native_task_artifact_json(spec: &NativeTaskArtifactRouteSpec, task_id: &str) -> String {
    json_or_error(&native_task_artifact_report(spec, task_id))
}

fn native_task_artifact_report(
    spec: &NativeTaskArtifactRouteSpec,
    task_id: &str,
) -> NativeTaskArtifactResponse {
    let evidence_search = native_transcript_report(
        session_root_candidates(),
        Some(task_id),
        MAX_NATIVE_TRANSCRIPT_QUERY_FILES,
    );
    let status = if evidence_search.scan_error_count == 0 {
        "ready"
    } else {
        "attention"
    };

    NativeTaskArtifactResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status,
        source_command: spec.source_command,
        native_route: true,
        compatibility_mode: spec.compatibility_mode,
        side_effect_free: true,
        artifact_kind: spec.artifact_kind,
        task_id_redacted: true,
        task_id_length: task_id.len(),
        evidence_found: evidence_search.matched_line_count > 0,
        matched_session_count: evidence_search.matched_session_count,
        matched_line_count: evidence_search.matched_line_count,
        evidence_search,
        raw_task_id_exposed: false,
        raw_transcript_exposed: false,
        transcript_text_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "replace redacted task evidence search with structured task registry storage when available",
    }
}

#[cfg(test)]
fn native_post_plan_route_specs() -> &'static [NativePostPlanRouteSpec] {
    hepta_gateway::native_post_plan_route_specs()
}

#[cfg(test)]
fn native_post_plan_report(
    spec: &NativePostPlanRouteSpec,
    parameter: Option<&str>,
    request_body: Option<&str>,
) -> NativePostPlanResponse {
    hepta_gateway::native_post_plan_report(
        spec,
        parameter,
        request_body,
        env_truthy(NATIVE_POST_REAL_HANDLERS_ENV),
        env_truthy(NATIVE_POST_REAL_HANDLER_APPROVAL_ENV),
        native_post_real_handler_scope_from_env().as_deref(),
        &native_post_execution_store_root(),
        NativePostExecutionStoreLimits {
            max_store_bytes: native_post_store_max_bytes(),
            max_store_lines: native_post_store_max_lines(),
            rate_limit_window_ms: native_post_rate_limit_window_ms(),
        },
    )
}

#[cfg(test)]
fn native_post_body_schema(plan_kind: &str, body_read_during_plan: bool) -> NativePostBodySchema {
    hepta_gateway::native_post_body_schema(plan_kind, body_read_during_plan)
}

#[cfg(test)]
fn native_post_body_admission(
    spec: &NativePostPlanRouteSpec,
    schema: &NativePostBodySchema,
    request_body: Option<&str>,
) -> NativePostBodyAdmission {
    hepta_gateway::native_post_body_admission(spec, schema, request_body)
}

#[cfg(test)]
fn native_post_idempotency_evidence(
    spec: &NativePostPlanRouteSpec,
    body_admission: &NativePostBodyAdmission,
) -> NativePostIdempotencyEvidence {
    hepta_gateway::native_post_idempotency_evidence(spec, body_admission)
}

#[cfg(test)]
fn native_post_audit_event_contract(
    spec: &NativePostPlanRouteSpec,
    body_schema: &NativePostBodySchema,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
) -> NativePostAuditEventContract {
    hepta_gateway::native_post_audit_event_contract(
        spec,
        body_schema,
        body_admission,
        idempotency_evidence,
    )
}

#[cfg(test)]
fn native_post_execution_admission_with_gates(
    spec: &NativePostPlanRouteSpec,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
    audit_event_contract: &NativePostAuditEventContract,
    enablement_gate_enabled: bool,
    operator_approval_enabled: bool,
) -> NativePostExecutionAdmission {
    native_post_execution_admission_with_scope(
        spec,
        body_admission,
        idempotency_evidence,
        audit_event_contract,
        enablement_gate_enabled,
        operator_approval_enabled,
        Some(spec.plan_kind),
    )
}

#[cfg(test)]
fn native_post_execution_admission_with_scope(
    spec: &NativePostPlanRouteSpec,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
    audit_event_contract: &NativePostAuditEventContract,
    enablement_gate_enabled: bool,
    operator_approval_enabled: bool,
    handler_scope: Option<&str>,
) -> NativePostExecutionAdmission {
    hepta_gateway::native_post_execution_admission_with_scope(
        spec,
        body_admission,
        idempotency_evidence,
        audit_event_contract,
        enablement_gate_enabled,
        operator_approval_enabled,
        handler_scope,
    )
}

#[cfg(test)]
fn native_post_plan_kind_has_real_handler(plan_kind: &str) -> bool {
    hepta_gateway::native_post_plan_kind_has_real_handler(plan_kind)
}

fn native_post_real_handler_scope_from_env() -> Option<String> {
    env::var(NATIVE_POST_REAL_HANDLER_SCOPE_ENV)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

#[cfg(test)]
fn native_post_real_handler_harness(
    spec: &NativePostPlanRouteSpec,
    body_schema: &NativePostBodySchema,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
    audit_event_contract: &NativePostAuditEventContract,
    execution_admission: &NativePostExecutionAdmission,
    store_root: &Path,
) -> NativePostRealHandlerHarness {
    hepta_gateway::native_post_real_handler_harness(
        spec,
        body_schema,
        body_admission,
        idempotency_evidence,
        audit_event_contract,
        execution_admission,
        store_root,
        NativePostExecutionStoreLimits {
            max_store_bytes: native_post_store_max_bytes(),
            max_store_lines: native_post_store_max_lines(),
            rate_limit_window_ms: native_post_rate_limit_window_ms(),
        },
    )
}

fn native_post_execution_readiness_json() -> String {
    json_or_error(&native_post_execution_readiness_report())
}

fn native_post_activation_plan_json() -> String {
    json_or_error(&native_post_activation_plan_report())
}

fn native_post_rollout_evidence_json() -> String {
    json_or_error(&native_post_rollout_evidence_report())
}

fn native_post_gray_release_evidence_json() -> String {
    json_or_error(&native_post_gray_release_evidence_report())
}

fn native_post_gray_release_evidence_report() -> NativePostGrayReleaseEvidenceResponse {
    hepta_gateway::native_post_gray_release_evidence_report(
        &native_post_execution_store_root(),
        native_post_store_max_bytes(),
        native_post_store_max_lines(),
        native_post_real_handler_scope_from_env().as_deref(),
        env_truthy(NATIVE_POST_REAL_HANDLERS_ENV),
        env_truthy(NATIVE_POST_REAL_HANDLER_APPROVAL_ENV),
    )
}

fn native_post_rollout_evidence_report() -> NativePostRolloutEvidenceResponse {
    hepta_gateway::native_post_rollout_evidence_report(
        &native_post_execution_store_root(),
        native_post_store_max_bytes(),
        native_post_store_max_lines(),
        native_post_real_handler_scope_from_env().as_deref(),
    )
}

fn native_post_activation_plan_report() -> NativePostActivationPlanResponse {
    hepta_gateway::native_post_activation_plan_report(
        &native_post_execution_store_root(),
        native_post_store_max_bytes(),
        native_post_store_max_lines(),
        env_truthy(NATIVE_POST_REAL_HANDLERS_ENV),
        env_truthy(NATIVE_POST_REAL_HANDLER_APPROVAL_ENV),
        native_post_real_handler_scope_from_env().as_deref(),
    )
}

fn native_post_execution_readiness_report() -> NativePostExecutionReadinessResponse {
    hepta_gateway::native_post_execution_readiness_report(
        env_truthy(NATIVE_POST_REAL_HANDLERS_ENV),
        native_post_real_handler_scope_from_env().as_deref(),
    )
}

fn native_post_execution_stores_json() -> String {
    json_or_error(&native_post_execution_stores_report())
}

fn native_post_execution_stores_report() -> NativePostExecutionStoresResponse {
    hepta_gateway::native_post_execution_stores_report(
        &native_post_execution_store_root(),
        native_post_store_max_bytes(),
        native_post_store_max_lines(),
    )
}

fn native_post_execution_store_root() -> PathBuf {
    if let Ok(value) = env::var(NATIVE_POST_EXECUTION_STORE_DIR_ENV) {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }
    env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(DEFAULT_NATIVE_POST_EXECUTION_STORE_DIR)
}

fn native_post_store_max_bytes() -> u64 {
    env_u64(NATIVE_POST_STORE_MAX_BYTES_ENV)
        .map(|bytes| bytes.clamp(1_024, 1024 * 1024 * 1024))
        .unwrap_or(DEFAULT_NATIVE_POST_STORE_MAX_BYTES)
}

fn native_post_store_max_lines() -> u64 {
    env_u64(NATIVE_POST_STORE_MAX_LINES_ENV)
        .map(|lines| lines.clamp(1, 10_000_000))
        .unwrap_or(DEFAULT_NATIVE_POST_STORE_MAX_LINES)
}

#[cfg(test)]
fn native_post_execution_store_record(
    spec: &NativePostPlanRouteSpec,
    body_schema: &NativePostBodySchema,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
    audit_event_contract: &NativePostAuditEventContract,
    current_plan_executes_real_handler: bool,
) -> NativePostExecutionStoreRecord {
    hepta_gateway::native_post_execution_store_record(
        spec,
        body_schema,
        body_admission,
        idempotency_evidence,
        audit_event_contract,
        current_plan_executes_real_handler,
    )
}

#[cfg(test)]
fn native_post_execution_store_capacity_allows_append_with_limits(
    root: &Path,
    record: &NativePostExecutionStoreRecord,
    max_store_bytes: u64,
    max_store_lines: u64,
) -> Result<bool, String> {
    hepta_gateway::native_post_execution_store_capacity_allows_append_with_limits(
        root,
        record,
        max_store_bytes,
        max_store_lines,
    )
}

#[cfg(test)]
fn persist_native_post_execution_store_record(
    root: &Path,
    record: &NativePostExecutionStoreRecord,
) -> Result<NativePostExecutionStoreWriteReport, String> {
    hepta_gateway::persist_native_post_execution_store_record(root, record)
}

fn native_post_rate_limit_window_ms() -> u64 {
    env::var(NATIVE_POST_RATE_LIMIT_WINDOW_MS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS)
}

fn native_events_json(surface: NativeEventSurface, cursor: Option<&str>) -> String {
    json_or_error(&native_events_report(
        session_root_candidates(),
        surface,
        cursor,
    ))
}

fn native_events_report(
    roots: Vec<NativeSessionRootCandidate>,
    surface: NativeEventSurface,
    cursor: Option<&str>,
) -> NativeEventsResponse {
    let transcript = native_transcript_report(roots.clone(), None, MAX_NATIVE_EVENT_FILES);
    let activity_sessions = if surface.includes_activity_sessions() {
        Some(native_sessions_report(
            roots,
            "/activity sessions --json",
            "native_activity_session_inventory",
        ))
    } else {
        None
    };

    let mut event_type_counts = BTreeMap::<String, u64>::new();
    let mut recent_events = Vec::<NativeEventPreview>::new();
    let mut total_line_count = 0_u64;
    let mut parsed_json_line_count = 0_u64;
    let mut truncated_session_count = 0_usize;

    for session in &transcript.sessions {
        total_line_count = total_line_count.saturating_add(session.line_count);
        parsed_json_line_count =
            parsed_json_line_count.saturating_add(session.parsed_json_line_count);
        truncated_session_count += usize::from(session.truncated);
        for count in &session.event_type_counts {
            *event_type_counts
                .entry(count.event_type.clone())
                .or_default() += count.count;
        }
        for event in &session.redacted_events {
            if recent_events.len() >= MAX_NATIVE_EVENT_PREVIEWS {
                break;
            }
            recent_events.push(NativeEventPreview {
                root_kind: session.root_kind,
                session_id: session.session_id.clone(),
                started_at_filename: session.started_at_filename.clone(),
                relative_path: session.relative_path.clone(),
                line_number: event.line_number,
                event_type: event.event_type.clone(),
                role: event.role.clone(),
                has_text_fields: event.has_text_fields,
                redacted: true,
            });
        }
    }

    let activity_scan_errors = activity_sessions
        .as_ref()
        .map(|sessions| sessions.scan_error_count)
        .unwrap_or_default();
    let scan_error_count = transcript.scan_error_count + activity_scan_errors;
    let status = if scan_error_count == 0 {
        "ready"
    } else {
        "attention"
    };

    NativeEventsResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status,
        source_command: surface.source_command(),
        native_route: true,
        compatibility_mode: surface.compatibility_mode(),
        side_effect_free: true,
        event_surface: surface.event_surface(),
        cursor_present: cursor.is_some(),
        cursor_redacted: cursor.is_some(),
        cursor_length: cursor.map(str::len),
        cursor_parseable_as_u64: cursor.and_then(|value| value.parse::<u64>().ok()).is_some(),
        scanned_session_file_count: transcript.scanned_session_file_count,
        available_session_file_count: transcript.available_session_file_count,
        max_files: transcript.max_files,
        max_lines_per_file: transcript.max_lines_per_file,
        total_line_count,
        parsed_json_line_count,
        parse_error_count: transcript.parse_error_count,
        scan_error_count,
        truncated_session_count,
        event_type_count: event_type_counts.len(),
        event_type_counts: event_type_counts
            .into_iter()
            .map(|(event_type, count)| NativeTranscriptEventCount { event_type, count })
            .collect(),
        recent_event_count: recent_events.len(),
        recent_events,
        activity_sessions,
        raw_transcript_exposed: false,
        transcript_text_exposed: false,
        raw_cursor_exposed: false,
        cursor_text_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote approvals, policy, and config surfaces with redacted local-only inventory",
    }
}

fn native_runtime_audit_json(surface: NativeRuntimeAuditSurface) -> String {
    json_or_error(&native_runtime_audit_report(
        session_root_candidates(),
        surface,
    ))
}

fn native_runtime_audit_report(
    roots: Vec<NativeSessionRootCandidate>,
    surface: NativeRuntimeAuditSurface,
) -> NativeRuntimeAuditResponse {
    let sessions = native_sessions_report(
        roots.clone(),
        surface.source_command(),
        "native_runtime_audit_session_inventory",
    );
    let events = native_events_report(roots, NativeEventSurface::EventsReport, None);
    let control_ui_route_parity = control_ui_route_parity_report();
    let approvals = native_approvals_report();
    let subagent_event_count =
        count_event_types_matching(&events.event_type_counts, runtime_event_type_is_subagent);
    let retry_or_error_event_count = count_event_types_matching(
        &events.event_type_counts,
        runtime_event_type_is_retry_or_error,
    );
    let multi_agent_event_count =
        count_event_types_matching(&events.event_type_counts, runtime_event_type_is_multi_agent);
    let ready = sessions.scan_error_count == 0
        && events.scan_error_count == 0
        && control_ui_route_parity.ready
        && approvals.status == "ready";

    NativeRuntimeAuditResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if ready { "ready" } else { "attention" },
        source_command: surface.source_command(),
        native_route: true,
        compatibility_mode: surface.compatibility_mode(),
        side_effect_free: true,
        audit_surface: surface.audit_surface(),
        event_focus: surface.event_focus(),
        agent_limit: surface.agent_limit(),
        message_limit: surface.message_limit(),
        route_matrix_ready: control_ui_route_parity.ready,
        route_count: control_ui_route_parity.route_count,
        missing_route_count: control_ui_route_parity.missing_route_count,
        approval_route_count: approvals.approval_route_count,
        guarded_approval_route_count: approvals.guarded_route_count,
        session_file_count: sessions.session_file_count,
        recent_session_count: sessions.recent_session_count,
        session_scan_error_count: sessions.scan_error_count,
        event_type_count: events.event_type_count,
        recent_event_count: events.recent_event_count,
        event_scan_error_count: events.scan_error_count,
        subagent_event_count,
        retry_or_error_event_count,
        multi_agent_event_count,
        sessions,
        events,
        redaction: NativeRuntimeAuditRedaction {
            raw_transcript_exposed: false,
            transcript_text_exposed: false,
            raw_agent_payload_exposed: false,
            raw_error_payload_exposed: false,
            raw_gateway_ledger_payload_exposed: false,
        },
        side_effects: NativeRuntimeAuditSideEffects {
            model_invoked: false,
            external_side_effects: false,
            gateway_mutation_performed: false,
            telegram_read_performed: false,
            message_sent: false,
            cursor_written: false,
        },
        next_migration_slice: "promote remaining runtime automation routes with the same local-only redacted audit boundary",
    }
}

fn count_event_types_matching(
    event_type_counts: &[NativeTranscriptEventCount],
    matches_event_type: fn(&str) -> bool,
) -> u64 {
    event_type_counts
        .iter()
        .filter(|count| matches_event_type(&count.event_type))
        .map(|count| count.count)
        .sum()
}

fn runtime_event_type_is_subagent(event_type: &str) -> bool {
    let event_type = event_type.to_ascii_lowercase();
    event_type.contains("subagent")
        || event_type.contains("sub_agent")
        || event_type.contains("agent_message")
}

fn runtime_event_type_is_retry_or_error(event_type: &str) -> bool {
    let event_type = event_type.to_ascii_lowercase();
    event_type.contains("retry")
        || event_type.contains("dead")
        || event_type.contains("error")
        || event_type.contains("failed")
        || event_type.contains("failure")
}

fn runtime_event_type_is_multi_agent(event_type: &str) -> bool {
    let event_type = event_type.to_ascii_lowercase();
    event_type.contains("multi_agent")
        || event_type.contains("multi-agent")
        || event_type.contains("subagent")
        || event_type.contains("agent_message")
}

fn native_control_ui_audit_json(
    surface: NativeControlUiAuditSurface,
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    json_or_error(&native_control_ui_audit_report(
        surface,
        options,
        telegram_plugin,
    ))
}

fn native_control_ui_audit_report(
    surface: NativeControlUiAuditSurface,
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> NativeControlUiAuditResponse {
    let route_matrix = control_ui_route_parity_report();
    let approvals = native_approvals_report();
    let gateway_replacement = gateway_replacement_readiness(options, telegram_plugin);
    let get_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.method == "GET")
        .count();
    let post_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.method == "POST")
        .count();
    let guarded_post_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.method == "POST")
        .filter(|route| post_route_is_guarded(route))
        .count();
    let dry_run_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| {
            route.side_effect_boundary.contains("dry-run")
                || route.side_effect_boundary.contains("plan only")
        })
        .count();
    let read_only_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.side_effect_boundary.contains("read-only"))
        .count();
    let ready = route_matrix.ready
        && approvals.status == "ready"
        && guarded_post_route_count == post_route_count;

    NativeControlUiAuditResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if ready { "ready" } else { "attention" },
        source_command: surface.source_command(),
        native_route: true,
        compatibility_mode: surface.compatibility_mode(),
        side_effect_free: true,
        control_surface: surface.control_surface(),
        plan_target: surface.plan_target(),
        dry_run_only: surface.dry_run_only(),
        read_only: surface.read_only(),
        confirmation_required_for_real_mutation: false,
        route_matrix_ready: route_matrix.ready,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        get_route_count,
        post_route_count,
        dry_run_route_count,
        read_only_route_count,
        guarded_post_route_count,
        approval_route_count: approvals.approval_route_count,
        guarded_approval_route_count: approvals.guarded_route_count,
        gateway_replacement_ready: gateway_replacement.ready,
        gateway_replacement_blocker_count: gateway_replacement.blocker_count,
        external_agent_benchmark_executed: false,
        external_agent_spawned: false,
        action_dispatched: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        route_matrix,
        redaction: NativeControlUiAuditRedaction {
            raw_transcript_exposed: false,
            transcript_text_exposed: false,
            raw_token_exposed: false,
            raw_action_payload_exposed: false,
            raw_agent_payload_exposed: false,
        },
        side_effects: NativeControlUiAuditSideEffects {
            model_invoked: false,
            external_side_effects: false,
            gateway_mutation_performed: false,
            telegram_read_performed: false,
            message_sent: false,
            cursor_written: false,
        },
        next_migration_slice: "turn guarded POST action routes into explicit native dry-run planners before enabling any mutation",
    }
}

fn native_approvals_json() -> String {
    json_or_error(&native_approvals_report())
}

fn native_approvals_report() -> NativeApprovalsResponse {
    let approval_routes = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.method == "POST")
        .map(|route| NativeApprovalRoute {
            method: route.method,
            pattern: route.pattern,
            capability: route.capability,
            source_command: route.source_command,
            side_effect_boundary: route.side_effect_boundary,
            dry_run_only: route.side_effect_boundary.contains("dry-run")
                || route.side_effect_boundary.contains("plan only"),
            guarded: post_route_is_guarded(route),
            confirmation_required_for_real_mutation: !route
                .side_effect_boundary
                .contains("read-only"),
        })
        .collect::<Vec<_>>();
    let guarded_route_count = approval_routes.iter().filter(|route| route.guarded).count();
    let pending_approval_count = 0usize;
    let ready = guarded_route_count == approval_routes.len();

    NativeApprovalsResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if ready { "ready" } else { "attention" },
        source_command: "/approvals --json",
        native_route: true,
        compatibility_mode: "native_approvals_redacted",
        side_effect_free: true,
        pending_approval_count,
        approval_route_count: approval_routes.len(),
        guarded_route_count,
        approval_routes,
        raw_command_payload_exposed: false,
        raw_approval_payload_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote approvals exec apply as an explicit dry-run plan before enabling mutation",
    }
}

fn native_policy_json(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    json_or_error(&native_policy_report(options, telegram_plugin))
}

fn native_policy_report(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> NativePolicyResponse {
    let gateway_replacement_readiness = gateway_replacement_readiness(options, telegram_plugin);
    let approvals = native_approvals_report();
    let loopback_bound = is_loopback_bind_addr(&options.bind_addr);
    let ready =
        loopback_bound && gateway_replacement_readiness.ready && approvals.status == "ready";

    NativePolicyResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if ready { "ready" } else { "attention" },
        source_command: "/policy --json",
        native_route: true,
        compatibility_mode: "native_policy_snapshot",
        side_effect_free: true,
        loopback_bind_required: true,
        loopback_bound,
        non_loopback_override_enabled: allow_non_loopback_ui(),
        bind_addr: options.bind_addr.clone(),
        telegram_gate_summary: native_telegram::telegram_gateway_gate_summary(),
        gateway_replacement_ready: gateway_replacement_readiness.ready,
        gateway_replacement_blocker_count: gateway_replacement_readiness.blocker_count,
        approval_route_count: approvals.approval_route_count,
        guarded_approval_route_count: approvals.guarded_route_count,
        raw_token_exposed: false,
        raw_transcript_exposed: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "thread policy snapshot into operator console once config and optional-configs are native",
    }
}

fn native_config_json(options: &NativeGatewayOptions) -> String {
    json_or_error(&native_config_report(options))
}

fn native_config_report(options: &NativeGatewayOptions) -> NativeConfigResponse {
    let config_roots = session_home_candidates()
        .into_iter()
        .map(|path| NativeConfigPathStatus {
            label: "session_home_candidate",
            path: path.display().to_string(),
            exists: path.exists(),
            is_dir: path.is_dir(),
            bytes: path
                .metadata()
                .ok()
                .filter(|meta| meta.is_file())
                .map(|meta| meta.len()),
        })
        .collect::<Vec<_>>();

    NativeConfigResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: "ready",
        source_command: "/config-surface --json",
        native_route: true,
        compatibility_mode: "native_config_surface_redacted",
        side_effect_free: true,
        bind_addr: options.bind_addr.clone(),
        telegram_plugin_requested: options.with_telegram_plugin,
        telegram_plugin_poll_ms: options.telegram_plugin_poll_ms,
        default_model_present: env::var("HEPTA_DEFAULT_MODEL")
            .ok()
            .is_some_and(|value| !value.trim().is_empty()),
        telegram_model_present: env::var("HEPTA_TELEGRAM_MODEL")
            .ok()
            .is_some_and(|value| !value.trim().is_empty()),
        openai_codex_home_present: env::var("HEPTA_OPENAI_CODEX_HOME")
            .ok()
            .is_some_and(|value| !value.trim().is_empty()),
        gateway_token_file_present: env::var("HEPTA_GATEWAY_TOKEN_FILE")
            .ok()
            .is_some_and(|value| !value.trim().is_empty()),
        release_build_verified: env_truthy(RELEASE_BUILD_VERIFIED_ENV),
        control_ui_parity_verified: env_truthy(CONTROL_UI_PARITY_VERIFIED_ENV),
        telegram_gate_summary: native_telegram::telegram_gateway_gate_summary(),
        config_root_count: config_roots.len(),
        config_roots,
        raw_env_exposed: false,
        raw_token_exposed: false,
        raw_config_value_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote optional config catalog and config edit dry-run plans without exposing raw config values",
    }
}

fn native_optional_configs_json() -> String {
    json_or_error(&native_optional_configs_report())
}

fn native_optional_configs_report() -> NativeOptionalConfigsResponse {
    let workspace_root = env::var("HOME")
        .map(|home| PathBuf::from(home).join(".openclaw/workspace"))
        .unwrap_or_else(|_| PathBuf::from("."));
    let today = "2026-05-18";
    let yesterday = "2026-05-17";
    let candidates = [
        ("agents", workspace_root.join("AGENTS.md"), true),
        ("soul", workspace_root.join("SOUL.md"), true),
        ("user", workspace_root.join("USER.md"), true),
        ("tools", workspace_root.join("TOOLS.md"), false),
        ("heartbeat", workspace_root.join("HEARTBEAT.md"), false),
        ("long_term_memory", workspace_root.join("MEMORY.md"), true),
        (
            "today_memory",
            workspace_root.join(format!("memory/{today}.md")),
            true,
        ),
        (
            "yesterday_memory",
            workspace_root.join(format!("memory/{yesterday}.md")),
            false,
        ),
    ];
    let configs = candidates
        .into_iter()
        .map(|(label, path, expected)| {
            let metadata = path.metadata().ok();
            NativeOptionalConfigStatus {
                label,
                path: path.display().to_string(),
                expected,
                exists: metadata.is_some(),
                is_file: metadata.as_ref().is_some_and(std::fs::Metadata::is_file),
                bytes: metadata
                    .as_ref()
                    .filter(|meta| meta.is_file())
                    .map(|meta| meta.len()),
                content_exposed: false,
            }
        })
        .collect::<Vec<_>>();
    let missing_expected_count = configs
        .iter()
        .filter(|config| config.expected && !config.exists)
        .count();
    NativeOptionalConfigsResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if missing_expected_count == 0 {
            "ready"
        } else {
            "attention"
        },
        source_command: "/optional-configs --json",
        native_route: true,
        compatibility_mode: "native_optional_configs_redacted",
        side_effect_free: true,
        config_count: configs.len(),
        missing_expected_count,
        configs,
        raw_config_value_exposed: false,
        config_content_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote config edit and approval apply endpoints as explicit dry-run/confirm-required plans",
    }
}

fn collect_session_file_candidates(
    roots: &[NativeSessionRootCandidate],
) -> Vec<NativeSessionFileCandidate> {
    let mut files = Vec::new();
    for root in roots {
        if root.root.exists() {
            collect_session_file_candidates_inner(&root.root, &root.root, root.kind, &mut files);
        }
    }
    files
}

fn collect_session_file_candidates_inner(
    root: &Path,
    path: &Path,
    root_kind: &'static str,
    files: &mut Vec<NativeSessionFileCandidate>,
) {
    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        if metadata.is_dir() {
            collect_session_file_candidates_inner(root, &path, root_kind, files);
        } else if metadata.is_file() && is_rollout_file(&path) {
            files.push(NativeSessionFileCandidate {
                path: path.clone(),
                root_kind,
                summary: session_summary_from_path(
                    root,
                    &path,
                    metadata.len(),
                    metadata_modified_unix_ms(&metadata),
                ),
            });
        }
    }
}

fn transcript_preview_from_file(
    candidate: &NativeSessionFileCandidate,
    query_lower: Option<&str>,
) -> NativeTranscriptSessionPreview {
    let mut preview = NativeTranscriptSessionPreview {
        root_kind: candidate.root_kind,
        session_id: candidate.summary.session_id.clone(),
        started_at_filename: candidate.summary.started_at_filename.clone(),
        filename: candidate.summary.filename.clone(),
        relative_path: candidate.summary.relative_path.clone(),
        bytes: candidate.summary.bytes,
        modified_unix_ms: candidate.summary.modified_unix_ms,
        line_count: 0,
        parsed_json_line_count: 0,
        parse_error_count: 0,
        truncated: false,
        event_type_counts: Vec::new(),
        redacted_events: Vec::new(),
        query_match: NativeTranscriptQueryMatch {
            matched_line_count: 0,
            first_match_line: None,
            matched_event_type_counts: Vec::new(),
        },
        read_error: None,
    };

    let file = match std::fs::File::open(&candidate.path) {
        Ok(file) => file,
        Err(err) => {
            preview.read_error = Some(err.to_string());
            return preview;
        }
    };

    let mut event_type_counts = BTreeMap::<String, u64>::new();
    let mut matched_event_type_counts = BTreeMap::<String, u64>::new();

    for (index, line) in BufReader::new(file).lines().enumerate() {
        if index >= MAX_NATIVE_TRANSCRIPT_LINES_PER_FILE {
            preview.truncated = true;
            break;
        }

        let line_number = index + 1;
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                preview.read_error = Some(err.to_string());
                break;
            }
        };
        preview.line_count += 1;

        let value = match serde_json::from_str::<serde_json::Value>(&line) {
            Ok(value) => {
                preview.parsed_json_line_count += 1;
                value
            }
            Err(_) => {
                preview.parse_error_count += 1;
                continue;
            }
        };

        let event_type = transcript_event_type(&value);
        *event_type_counts.entry(event_type.clone()).or_default() += 1;

        let query_matched = query_lower
            .map(|query| line.to_ascii_lowercase().contains(query))
            .unwrap_or(false);
        if query_matched {
            preview.query_match.matched_line_count += 1;
            if preview.query_match.first_match_line.is_none() {
                preview.query_match.first_match_line = Some(line_number);
            }
            *matched_event_type_counts
                .entry(event_type.clone())
                .or_default() += 1;
        }

        if preview.redacted_events.len() < MAX_NATIVE_TRANSCRIPT_EVENT_PREVIEWS_PER_FILE {
            preview.redacted_events.push(redacted_transcript_event(
                line_number,
                &event_type,
                &value,
            ));
        }
    }

    preview.event_type_counts = event_type_counts
        .into_iter()
        .map(|(event_type, count)| NativeTranscriptEventCount { event_type, count })
        .collect();
    preview.query_match.matched_event_type_counts = matched_event_type_counts
        .into_iter()
        .map(|(event_type, count)| NativeTranscriptEventCount { event_type, count })
        .collect();
    preview
}

fn transcript_event_type(value: &serde_json::Value) -> String {
    let top_level = value
        .get("type")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");
    let payload_type = value
        .get("payload")
        .and_then(|payload| payload.get("type"))
        .and_then(serde_json::Value::as_str);
    match payload_type {
        Some(payload_type) => format!("{top_level}:{payload_type}"),
        None => top_level.to_string(),
    }
}

fn redacted_transcript_event(
    line_number: usize,
    event_type: &str,
    value: &serde_json::Value,
) -> NativeTranscriptEventPreview {
    let payload = value.get("payload");
    NativeTranscriptEventPreview {
        line_number,
        event_type: event_type.to_string(),
        role: payload
            .and_then(|payload| payload.get("role"))
            .and_then(serde_json::Value::as_str)
            .map(str::to_string),
        has_text_fields: json_contains_text_field(value),
        redacted: true,
    }
}

fn json_contains_text_field(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, value)| {
            key == "text" || key == "message" || key == "content" || json_contains_text_field(value)
        }),
        serde_json::Value::Array(values) => values.iter().any(json_contains_text_field),
        _ => false,
    }
}

fn operator_console_json(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    let gateway_replacement_readiness = gateway_replacement_readiness(options, telegram_plugin);
    let control_ui_route_parity = control_ui_route_parity_report();
    let sessions = native_sessions_report(
        session_root_candidates(),
        "/sessions --json",
        "native_sessions_inventory",
    );
    let telegram_live_soak_status = native_telegram::telegram_live_soak_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let telegram_poll_loop_status = native_telegram::telegram_poll_loop_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );

    let ready = gateway_replacement_readiness.ready
        && control_ui_route_parity.ready
        && sessions.scan_error_count == 0
        && telegram_live_soak_status.health_ready;

    json_or_error(&NativeOperatorConsoleResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if ready { "ready" } else { "attention" },
        source_command: "/operator-console --json",
        native_route: true,
        compatibility_mode: "native_operator_console",
        side_effect_free: true,
        health: HealthResponse {
            product: "Hepta",
            runtime: "hepta-codex",
            status: "ready",
        },
        operator_snapshot_endpoint: "/api/operator-snapshot",
        operator_security_endpoint: "/api/operator-security",
        sessions_endpoint: "/api/sessions",
        session_activity_endpoint: "/api/session-activity",
        gateway_replacement_readiness,
        control_ui_route_parity,
        sessions,
        telegram_plugin,
        telegram_poll_loop_status,
        telegram_live_soak_status,
        raw_transcript_exposed: false,
        raw_token_exposed: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        model_invoked: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote transcript and task drilldown previews with explicit redaction contracts",
    })
}

fn operator_security_json(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    let control_ui_route_parity = control_ui_route_parity_report();
    let gateway_replacement_readiness = gateway_replacement_readiness(options, telegram_plugin);
    let post_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.method == "POST")
        .count();
    let dry_run_post_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.method == "POST")
        .filter(|route| {
            route.side_effect_boundary.contains("dry-run")
                || route.side_effect_boundary.contains("plan only")
        })
        .count();
    let guarded_post_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.method == "POST")
        .filter(|route| post_route_is_guarded(route))
        .count();
    let telegram_production_readiness_status =
        native_telegram::telegram_production_readiness_status(
            options.with_telegram_plugin,
            options.telegram_plugin_poll_ms,
        );
    let telegram_owner_handoff_status = telegram_owner_handoff_status(options);
    let post_execution_readiness = native_post_execution_readiness_report();
    let post_execution_stores = native_post_execution_stores_report();
    let post_activation_plan = native_post_activation_plan_report();
    let post_gray_release_evidence = native_post_gray_release_evidence_report();
    let post_execution_stores_ready = post_execution_stores.persistence_implementation_ready
        && post_execution_stores.idempotency_store_ready
        && post_execution_stores.audit_store_ready
        && post_execution_stores.rollback_store_ready
        && post_execution_stores.rate_limit_store_ready
        && post_execution_stores.store_jsonl_valid
        && post_execution_stores.store_capacity_ok;
    let post_activation_plan_ready = post_activation_plan.activation_preflight_ready
        && post_activation_plan.rollback_ready
        && !post_activation_plan.activation_currently_enabled;
    let post_gray_release_evidence_ready = !post_activation_plan.activation_currently_enabled
        || post_gray_release_evidence.gray_release_evidence_ready;
    let production_soak_ready = telegram_production_readiness_status.ready;
    let loopback_bound = is_loopback_bind_addr(&options.bind_addr);
    let ready = control_ui_route_parity.ready
        && gateway_replacement_readiness.ready
        && production_soak_ready
        && post_execution_readiness.all_evidence_contracts_ready
        && post_execution_stores_ready
        && post_activation_plan_ready
        && post_gray_release_evidence_ready
        && telegram_owner_handoff_status.conflict_free
        && loopback_bound
        && guarded_post_route_count == post_route_count;

    json_or_error(&NativeOperatorSecurityResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if ready { "ready" } else { "attention" },
        source_command: "/operator-security --json",
        native_route: true,
        compatibility_mode: "native_operator_security",
        side_effect_free: true,
        loopback_bind_required: true,
        loopback_bound,
        non_loopback_override_enabled: allow_non_loopback_ui(),
        bind_addr: options.bind_addr.clone(),
        control_ui_route_parity,
        gateway_replacement_readiness,
        post_route_count,
        dry_run_post_route_count,
        guarded_post_route_count,
        post_execution_readiness_endpoint: NATIVE_POST_EXECUTION_READINESS_ENDPOINT,
        post_execution_stores_endpoint: NATIVE_POST_EXECUTION_STORES_ENDPOINT,
        post_activation_plan_endpoint: NATIVE_POST_ACTIVATION_PLAN_ENDPOINT,
        post_execution_readiness,
        post_execution_stores_ready,
        post_execution_stores,
        post_activation_plan_ready,
        post_activation_plan,
        post_gray_release_evidence_endpoint: NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT,
        post_gray_release_evidence_ready,
        post_gray_release_evidence,
        production_soak_ready,
        telegram_gate_summary: native_telegram::telegram_gateway_gate_summary(),
        telegram_production_readiness_status,
        telegram_owner_handoff_endpoint: TELEGRAM_OWNER_HANDOFF_ENDPOINT,
        telegram_owner_handoff_status,
        telegram_plugin_requested: options.with_telegram_plugin,
        telegram_plugin_status: telegram_plugin.status,
        redaction: NativeOperatorSecurityRedaction {
            raw_transcript_exposed: false,
            raw_token_exposed: false,
            raw_update_payload_exposed: false,
            raw_prompt_text_exposed: false,
            raw_response_text_exposed: false,
            raw_idempotency_key_exposed: false,
            raw_audit_payload_exposed: false,
        },
        side_effects: NativeOperatorSecuritySideEffects {
            external_side_effects: false,
            gateway_mutation_performed: false,
            telegram_read_performed: false,
            model_invoked: false,
            message_sent: false,
            cursor_written: false,
        },
        next_migration_slice: "keep POST routes dry-run until one selected handler is wired through the native execution stores with operator approval",
    })
}

fn telegram_owner_handoff_status(
    options: &NativeGatewayOptions,
) -> NativeTelegramOwnerHandoffStatus {
    let legacy_config_path = legacy_openclaw_config_path();
    let (legacy_config_found, legacy_config_parse_ok, legacy_telegram_enabled, error) =
        read_legacy_openclaw_telegram_enabled(legacy_config_path.as_deref());
    let poll_loop_status = native_telegram::telegram_poll_loop_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let gate_summary = native_telegram::telegram_gateway_gate_summary();

    telegram_owner_handoff_status_from_inputs(NativeTelegramOwnerHandoffInputs {
        legacy_config_path: legacy_config_path.map(|path| path.display().to_string()),
        legacy_config_found,
        legacy_config_parse_ok,
        legacy_telegram_enabled,
        legacy_config_error: error,
        hepta_telegram_requested: options.with_telegram_plugin,
        hepta_poll_loop_armed: poll_loop_status.status == "armed"
            && poll_loop_status.loop_invokes_drain_once,
        hepta_poll_loop_gate_enabled: poll_loop_status.poll_loop_gate_enabled,
        hepta_delivery_approval_gate_enabled: gate_summary.delivery_approval_gate_enabled,
    })
}

fn telegram_owner_handoff_status_from_inputs(
    inputs: NativeTelegramOwnerHandoffInputs,
) -> NativeTelegramOwnerHandoffStatus {
    let legacy_telegram_enabled = inputs.legacy_telegram_enabled;
    let legacy_telegram_enabled_explicit = legacy_telegram_enabled.is_some();
    let legacy_enabled = legacy_telegram_enabled == Some(true);
    let double_poller_risk = legacy_enabled && inputs.hepta_poll_loop_armed;
    let conflict_free = inputs.legacy_config_parse_ok && !double_poller_risk;
    let hepta_takeover_ready = conflict_free
        && inputs.hepta_telegram_requested
        && inputs.hepta_poll_loop_armed
        && !legacy_enabled;

    let active_owner = if double_poller_risk {
        "conflict_risk"
    } else if inputs.hepta_poll_loop_armed {
        "hepta"
    } else if legacy_enabled {
        "legacy_openclaw"
    } else {
        "none"
    };

    let status = if !inputs.legacy_config_parse_ok {
        "attention"
    } else if double_poller_risk {
        "conflict_risk"
    } else if hepta_takeover_ready {
        "hepta_takeover_ready"
    } else if legacy_enabled {
        "legacy_owner"
    } else {
        "handoff_pending"
    };

    let mut takeover_blockers = Vec::new();
    if !inputs.legacy_config_parse_ok {
        takeover_blockers.push("legacy_openclaw_config_unreadable");
    }
    if legacy_enabled {
        takeover_blockers.push("legacy_openclaw_telegram_enabled");
    }
    if !inputs.hepta_telegram_requested {
        takeover_blockers.push("hepta_telegram_not_requested");
    }
    if !inputs.hepta_delivery_approval_gate_enabled {
        takeover_blockers.push("hepta_delivery_approval_gate_disabled");
    }
    if !inputs.hepta_poll_loop_gate_enabled {
        takeover_blockers.push("hepta_poll_loop_gate_disabled");
    }
    if !inputs.hepta_poll_loop_armed {
        takeover_blockers.push("hepta_poll_loop_not_armed");
    }

    NativeTelegramOwnerHandoffStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        status,
        endpoint: TELEGRAM_OWNER_HANDOFF_ENDPOINT,
        ready: conflict_free,
        conflict_free,
        hepta_takeover_ready,
        side_effect_free: true,
        active_owner,
        legacy_config_path: inputs.legacy_config_path,
        legacy_config_found: inputs.legacy_config_found,
        legacy_config_parse_ok: inputs.legacy_config_parse_ok,
        legacy_telegram_enabled,
        legacy_telegram_enabled_explicit,
        hepta_telegram_requested: inputs.hepta_telegram_requested,
        hepta_poll_loop_armed: inputs.hepta_poll_loop_armed,
        hepta_poll_loop_gate_enabled: inputs.hepta_poll_loop_gate_enabled,
        hepta_delivery_approval_gate_enabled: inputs.hepta_delivery_approval_gate_enabled,
        double_poller_risk,
        takeover_blockers,
        legacy_config_error: inputs.legacy_config_error,
        raw_token_exposed: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        next_migration_slice: if hepta_takeover_ready {
            "legacy Telegram polling is disabled and Hepta is the only armed poller; continue live soak and ledger freshness checks"
        } else if double_poller_risk {
            "disable the legacy OpenClaw Telegram plugin before arming Hepta polling to avoid Bot API 409 conflicts"
        } else if legacy_enabled {
            "keep legacy OpenClaw as owner until the controlled flip disables its Telegram plugin and arms Hepta gates"
        } else {
            "arm Hepta Telegram gates only after confirming legacy OpenClaw Telegram polling is disabled"
        },
    }
}

fn legacy_openclaw_config_path() -> Option<PathBuf> {
    if let Ok(path) = env::var(LEGACY_OPENCLAW_CONFIG_PATH_ENV) {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Some(path);
        }
    }

    env::var_os("HOME").map(|home| PathBuf::from(home).join(".openclaw/openclaw.json"))
}

fn read_legacy_openclaw_telegram_enabled(
    path: Option<&Path>,
) -> (bool, bool, Option<bool>, Option<String>) {
    let Some(path) = path else {
        return (false, true, None, None);
    };
    if !path.is_file() {
        return (false, true, None, None);
    }
    let raw = match fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(error) => {
            return (
                true,
                false,
                None,
                Some(format!("failed to read legacy OpenClaw config: {error}")),
            );
        }
    };
    let value = match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(value) => value,
        Err(error) => {
            return (
                true,
                false,
                None,
                Some(format!("failed to parse legacy OpenClaw config: {error}")),
            );
        }
    };

    (true, true, legacy_openclaw_telegram_enabled(&value), None)
}

fn legacy_openclaw_telegram_enabled(value: &serde_json::Value) -> Option<bool> {
    value
        .pointer("/plugins/entries/telegram/enabled")
        .and_then(serde_json::Value::as_bool)
        .or_else(|| {
            value
                .pointer("/channels/telegram/enabled")
                .and_then(serde_json::Value::as_bool)
        })
}

fn post_route_is_guarded(route: &ControlUiRouteSpec) -> bool {
    let boundary = route.side_effect_boundary;
    boundary.contains("dry-run")
        || boundary.contains("plan only")
        || boundary.contains("not executed")
        || boundary.contains("confirm-required")
        || boundary.contains("requires confirmation")
        || boundary.contains("never mutates")
        || boundary.contains("never publishes")
        || boundary.contains("never sends")
}

fn gateway_replacement_readiness(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> NativeGatewayReplacementReadiness {
    let telegram_gate_summary = native_telegram::telegram_gateway_gate_summary();
    let telegram_poll_loop_status = native_telegram::telegram_poll_loop_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let telegram_owner_handoff_status = telegram_owner_handoff_status(options);
    let control_ui_route_parity = control_ui_route_parity_report();
    let release_build_verified = env_truthy(RELEASE_BUILD_VERIFIED_ENV);
    let control_ui_parity_verified =
        control_ui_route_parity.ready && env_truthy(CONTROL_UI_PARITY_VERIFIED_ENV);
    let in_process_model_runner_ready =
        env_truthy(native_telegram::TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV);
    let side_effect_free = true;

    let checks = vec![
        NativeGatewayReplacementCheck {
            name: "launchd_entrypoint_compatible",
            ready: true,
            detail: "serve-ui loopback entrypoint is available in hepta-codex",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_plugin_requested",
            ready: options.with_telegram_plugin,
            detail: "production replacement requires --with-telegram-plugin",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_config_binding_ready",
            ready: telegram_plugin.config.binding_ready,
            detail: "Telegram config and secret binding are redacted and resolvable",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_native_supervisor_ready",
            ready: telegram_plugin.in_process_supervisor_ready,
            detail: "native supervisor can load without OpenClaw gateway runtime dependency",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_delivery_approval_gate_enabled",
            ready: telegram_gate_summary.delivery_approval_gate_enabled,
            detail: "HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED must be enabled before live poll loop drain side effects",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_live_read_gate_enabled",
            ready: telegram_gate_summary.live_read_gate_enabled,
            detail: "HEPTA_NATIVE_TELEGRAM_LIVE_READ must be enabled for active polling",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_model_turn_gate_enabled",
            ready: telegram_gate_summary.model_turn_gate_enabled,
            detail: "HEPTA_NATIVE_TELEGRAM_MODEL_TURN must be enabled for active replies",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_send_gate_enabled",
            ready: telegram_gate_summary.send_gate_enabled,
            detail: "HEPTA_NATIVE_TELEGRAM_SEND must be enabled for active delivery",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_poll_loop_gate_enabled",
            ready: telegram_poll_loop_status.poll_loop_gate_enabled,
            detail: "HEPTA_NATIVE_TELEGRAM_POLL_LOOP must be enabled for background draining",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_poll_loop_invokes_drain_once",
            ready: telegram_poll_loop_status.loop_invokes_drain_once,
            detail: "poll loop must route through the gated drain-once pipeline",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_owner_handoff_ready",
            ready: telegram_owner_handoff_status.hepta_takeover_ready,
            detail: "legacy OpenClaw Telegram polling must be disabled before Hepta becomes the armed Telegram owner",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_cursor_policy_ready",
            ready: telegram_plugin.cursor_plan.duplicate_suppression_ready
                && telegram_plugin.cursor_plan.commit_offset_after_delivery
                && !telegram_plugin.cursor_plan.raw_update_payload_persisted,
            detail: "cursor commits only after delivery or duplicate suppression without raw payload persistence",
        },
        NativeGatewayReplacementCheck {
            name: "in_process_model_runner_ready",
            ready: in_process_model_runner_ready,
            detail: "HEPTA_NATIVE_TELEGRAM_IN_PROCESS_MODEL_RUNNER must be enabled after in-process runner smoke passes",
        },
        NativeGatewayReplacementCheck {
            name: "release_build_verified",
            ready: release_build_verified,
            detail: "release fat-LTO build must pass before replacing the active gateway",
        },
        NativeGatewayReplacementCheck {
            name: "control_ui_route_matrix_ready",
            ready: control_ui_route_parity.ready,
            detail: "hepta-codex must expose the old Hepta Control UI HTTP route matrix",
        },
        NativeGatewayReplacementCheck {
            name: "control_ui_route_parity_verified",
            ready: control_ui_parity_verified,
            detail: "HEPTA_CODEX_CONTROL_UI_PARITY_VERIFIED must be enabled after route parity smoke passes",
        },
        NativeGatewayReplacementCheck {
            name: "readiness_report_side_effect_free",
            ready: side_effect_free,
            detail: "readiness report does not read Telegram, invoke model, send message, or write cursor",
        },
    ];

    let blockers = checks
        .iter()
        .filter(|check| !check.ready)
        .map(|check| check.name)
        .collect::<Vec<_>>();
    let ready = blockers.is_empty();
    let status = if ready { "ready" } else { "blocked" };

    let next_migration_slice = if ready {
        "active replacement gates are satisfied; keep /api/telegram-live-soak green before broadening traffic or relaxing guards"
    } else {
        "run explicit live Telegram gate smoke only with operator approval, then replace the active gateway"
    };

    NativeGatewayReplacementReadiness {
        product: "Hepta",
        runtime: "hepta-codex",
        status,
        ready,
        active_install_allowed: ready,
        side_effect_free,
        blocker_count: blockers.len(),
        blockers,
        checks,
        required_env_gates: NativeGatewayReplacementEnvGates {
            delivery_approval: NativeGatewayReplacementGate {
                env: native_telegram::TELEGRAM_DELIVERY_APPROVED_ENV,
                enabled: telegram_gate_summary.delivery_approval_gate_enabled,
            },
            live_read: NativeGatewayReplacementGate {
                env: native_telegram::TELEGRAM_LIVE_READ_ENV,
                enabled: telegram_gate_summary.live_read_gate_enabled,
            },
            model_turn: NativeGatewayReplacementGate {
                env: native_telegram::TELEGRAM_MODEL_TURN_GATE_ENV,
                enabled: telegram_gate_summary.model_turn_gate_enabled,
            },
            send: NativeGatewayReplacementGate {
                env: native_telegram::TELEGRAM_SEND_GATE_ENV,
                enabled: telegram_gate_summary.send_gate_enabled,
            },
            poll_loop: NativeGatewayReplacementGate {
                env: native_telegram::TELEGRAM_POLL_LOOP_ENV,
                enabled: telegram_poll_loop_status.poll_loop_gate_enabled,
            },
            in_process_model_runner: NativeGatewayReplacementGate {
                env: native_telegram::TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV,
                enabled: in_process_model_runner_ready,
            },
            release_build_verified: NativeGatewayReplacementGate {
                env: RELEASE_BUILD_VERIFIED_ENV,
                enabled: release_build_verified,
            },
            control_ui_parity_verified: NativeGatewayReplacementGate {
                env: CONTROL_UI_PARITY_VERIFIED_ENV,
                enabled: control_ui_parity_verified,
            },
        },
        telegram_owner_handoff_endpoint: TELEGRAM_OWNER_HANDOFF_ENDPOINT,
        telegram_owner_handoff_status,
        control_ui_route_parity,
        next_migration_slice,
    }
}

fn gateway_live_activation_plan(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> NativeGatewayLiveActivationPlan {
    let readiness = gateway_replacement_readiness(options, telegram_plugin);
    let telegram_gate_summary = native_telegram::telegram_gateway_gate_summary();
    let poll_loop_status = native_telegram::telegram_poll_loop_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let status = if readiness.ready && poll_loop_status.loop_invokes_drain_once {
        "active_live_soak_ready"
    } else if readiness.ready {
        "ready_for_operator_live_smoke"
    } else if telegram_gate_summary.delivery_approval_gate_enabled {
        "blocked_after_operator_approval"
    } else {
        "operator_approval_required"
    };

    NativeGatewayLiveActivationPlan {
        product: "Hepta",
        runtime: "hepta-codex",
        status,
        endpoint: GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT,
        operator_approval_required: !telegram_gate_summary.delivery_approval_gate_enabled,
        active_install_allowed: readiness.ready,
        readiness_blocker_count: readiness.blocker_count,
        readiness_blockers: readiness.blockers.clone(),
        active_gateway_label: ACTIVE_GATEWAY_LABEL,
        current_legacy_binary: ACTIVE_GATEWAY_LEGACY_BINARY,
        replacement_binary: HEPTA_CODEX_RELEASE_BINARY,
        bind_addr: options.bind_addr.clone(),
        launch_arguments: vec![
            HEPTA_CODEX_RELEASE_BINARY.to_string(),
            "--serve-ui".to_string(),
            options.bind_addr.clone(),
            "--with-telegram-plugin".to_string(),
            "--telegram-plugin-poll-ms".to_string(),
            options.telegram_plugin_poll_ms.to_string(),
        ],
        required_env_gates: vec![
            NativeGatewayLiveActivationEnv {
                env: native_telegram::TELEGRAM_DELIVERY_APPROVED_ENV,
                enabled: telegram_gate_summary.delivery_approval_gate_enabled,
                purpose: "explicit operator approval before any live Telegram drain side effects",
            },
            NativeGatewayLiveActivationEnv {
                env: native_telegram::TELEGRAM_LIVE_READ_ENV,
                enabled: telegram_gate_summary.live_read_gate_enabled,
                purpose: "allow one-shot getUpdates and poll-loop live reads",
            },
            NativeGatewayLiveActivationEnv {
                env: native_telegram::TELEGRAM_MODEL_TURN_GATE_ENV,
                enabled: telegram_gate_summary.model_turn_gate_enabled,
                purpose: "allow model execution for redacted Telegram candidates",
            },
            NativeGatewayLiveActivationEnv {
                env: native_telegram::TELEGRAM_SEND_GATE_ENV,
                enabled: telegram_gate_summary.send_gate_enabled,
                purpose: "allow sendMessage only after model success and reply-target validation",
            },
            NativeGatewayLiveActivationEnv {
                env: native_telegram::TELEGRAM_POLL_LOOP_ENV,
                enabled: poll_loop_status.poll_loop_gate_enabled,
                purpose: "allow supervised background drain loop from --serve-ui",
            },
            NativeGatewayLiveActivationEnv {
                env: native_telegram::TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV,
                enabled: env_truthy(native_telegram::TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV),
                purpose: "use library-backed in-process runner instead of child exec fallback",
            },
            NativeGatewayLiveActivationEnv {
                env: RELEASE_BUILD_VERIFIED_ENV,
                enabled: env_truthy(RELEASE_BUILD_VERIFIED_ENV),
                purpose: "operator asserts current release binary passed the fat-LTO build gate",
            },
            NativeGatewayLiveActivationEnv {
                env: CONTROL_UI_PARITY_VERIFIED_ENV,
                enabled: env_truthy(CONTROL_UI_PARITY_VERIFIED_ENV),
                purpose: "operator asserts Control UI route parity smoke passed",
            },
        ],
        live_smoke_sequence: &[
            "start isolated hepta-codex release binary on a non-production loopback port",
            "GET /api/gateway-replacement-readiness and require active_install_allowed=false until delivery approval is explicit",
            "GET /api/control-ui-route-parity and require missing_route_count=0",
            "GET /api/telegram-poll-loop and require no status-triggered external read/send",
            "with explicit approval gates only, call /api/telegram-drain-once once and inspect redacted status",
            "allow production replacement only if readiness has no blockers after the smoke",
        ],
        production_replacement_sequence: &[
            "keep the old Hepta gateway binary and launchd label as rollback anchors",
            "install the verified hepta-codex release binary under the isolated hepta-codex path",
            "switch the active launchd ProgramArguments to hepta-codex --serve-ui loopback with Telegram plugin flags",
            "set only the audited HEPTA_NATIVE_* and HEPTA_CODEX_* gate env vars",
            "kickstart the gateway service and verify /health plus /api/native-gateway",
            "rollback by restoring the old ProgramArguments/binary and kickstarting ai.hepta.gateway",
        ],
        safety: NativeGatewayLiveActivationSafety {
            side_effect_free: true,
            status_probe_reads_telegram: false,
            status_probe_invokes_model: false,
            status_probe_sends_message: false,
            status_probe_writes_cursor: false,
            raw_token_exposed: false,
            raw_update_payload_exposed: false,
            raw_prompt_text_exposed: false,
            raw_response_text_exposed: false,
        },
        next_migration_slice: if readiness.ready {
            "active gateway is live; continue soak, keep rollback anchors, and watch /api/telegram-live-soak"
        } else {
            "perform an explicit operator-approved live Telegram drain smoke, then replace the active gateway if readiness is green"
        },
    }
}

fn json_or_error<T: Serialize>(value: &T) -> String {
    match serde_json::to_string(value) {
        Ok(json) => json,
        Err(err) => format!(r#"{{"error":"native gateway serialization failed: {err}"}}"#),
    }
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeGatewayResponse<'a> {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    migration_mode: &'static str,
    bind_addr: &'a str,
    launchd_entrypoint_compatible: bool,
    active_gateway_replacement_ready: bool,
    replacement_blocker: Option<&'static str>,
    gateway_replacement_readiness_endpoint: &'static str,
    gateway_replacement_readiness: NativeGatewayReplacementReadiness,
    gateway_route_core_status: NativeGatewayRouteCoreStatus,
    gateway_live_activation_plan_endpoint: &'static str,
    gateway_live_activation_plan: NativeGatewayLiveActivationPlan,
    control_ui_route_parity_endpoint: &'static str,
    control_ui_route_parity_ready: bool,
    control_ui_route_parity: ControlUiRouteParityReport,
    telegram_plugin_requested: bool,
    telegram_plugin_status: &'static str,
    telegram_plugin_native_supervisor_ready: bool,
    telegram_plugin_reply_loop_ready: bool,
    telegram_plugin_poll_ms: u64,
    telegram_receive_once_endpoint: &'static str,
    telegram_model_turn_plan_endpoint: &'static str,
    telegram_model_bridge_endpoint: &'static str,
    telegram_send_plan_endpoint: &'static str,
    telegram_drain_once_endpoint: &'static str,
    telegram_poll_loop_endpoint: &'static str,
    telegram_live_soak_endpoint: &'static str,
    telegram_live_soak_status_endpoint: &'static str,
    telegram_production_readiness_endpoint: &'static str,
    telegram_production_readiness_status: native_telegram::NativeTelegramProductionReadinessStatus,
    telegram_delivery_ledger_endpoint: &'static str,
    telegram_delivery_ledger_status: native_telegram::NativeTelegramDeliveryLedgerStatus,
    telegram_owner_handoff_endpoint: &'static str,
    telegram_owner_handoff_status: NativeTelegramOwnerHandoffStatus,
    telegram_cursor_endpoint: &'static str,
    telegram_gate_summary: native_telegram::NativeTelegramGatewayGateSummary,
    telegram_poll_loop_status: native_telegram::NativeTelegramPollLoopStatus,
    telegram_live_soak_status: native_telegram::NativeTelegramLiveSoakStatus,
    telegram_readiness_summary_side_effect_free: bool,
    telegram_plugin: &'a NativeTelegramPluginStatus,
    migrated_surfaces: &'static [&'static str],
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeGatewayRouteCoreStatus {
    source_crate: &'static str,
    route_core_ready: bool,
    surface_id: String,
    session_key: String,
    transport: &'static str,
    normalized_text: String,
    supported_transports: &'static [&'static str],
    side_effect_free: bool,
}

fn native_gateway_route_core_status() -> NativeGatewayRouteCoreStatus {
    let surface = hepta_gateway::GatewaySurface;
    let envelope = hepta_gateway::GatewayEnvelope::new(
        "hepta",
        "operator",
        hepta_gateway::GatewayTransport::Webhook,
        "  /status --json  ",
    )
    .with_session_hint("hepta:operator");
    let plan = surface.route_plan(&envelope);

    NativeGatewayRouteCoreStatus {
        source_crate: "hepta-gateway",
        route_core_ready: surface.supports_transport(envelope.transport)
            && !plan.session_key.trim().is_empty()
            && !plan.normalized_text.trim().is_empty(),
        surface_id: plan.surface_id,
        session_key: plan.session_key,
        transport: gateway_transport_label(plan.transport),
        normalized_text: plan.normalized_text,
        supported_transports: &["cli", "webhook", "queue"],
        side_effect_free: true,
    }
}

fn gateway_transport_label(transport: hepta_gateway::GatewayTransport) -> &'static str {
    match transport {
        hepta_gateway::GatewayTransport::Cli => "cli",
        hepta_gateway::GatewayTransport::Webhook => "webhook",
        hepta_gateway::GatewayTransport::Queue => "queue",
    }
}

#[derive(Debug, Serialize)]
struct NativeOperatorSnapshotResponse<'a> {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    health: HealthResponse,
    active_gateway_replacement_ready: bool,
    route_matrix_ready: bool,
    production_soak_ready: bool,
    gateway_replacement_readiness: NativeGatewayReplacementReadiness,
    control_ui_route_parity: ControlUiRouteParityReport,
    telegram_plugin: &'a NativeTelegramPluginStatus,
    telegram_poll_loop_status: native_telegram::NativeTelegramPollLoopStatus,
    telegram_live_soak_status: native_telegram::NativeTelegramLiveSoakStatus,
    telegram_cursor_status: native_telegram::NativeTelegramCursorStatus,
    telegram_read_performed: bool,
    model_invoked: bool,
    message_sent: bool,
    cursor_written: bool,
    raw_token_exposed: bool,
    raw_update_payload_exposed: bool,
    raw_prompt_text_exposed: bool,
    raw_response_text_exposed: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeSessionsResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    scanned_root_count: usize,
    existing_root_count: usize,
    scan_error_count: usize,
    session_file_count: u64,
    total_bytes: u64,
    recent_session_count: usize,
    roots: Vec<NativeSessionRootReport>,
    recent_sessions: Vec<NativeSessionSummary>,
    raw_transcript_exposed: bool,
    transcript_text_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Clone)]
struct NativeSessionRootCandidate {
    root: PathBuf,
    kind: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct NativeSessionRootReport {
    root: String,
    kind: &'static str,
    exists: bool,
    file_count: u64,
    total_bytes: u64,
    latest_modified_unix_ms: Option<u64>,
    error: Option<String>,
    recent_sessions: Vec<NativeSessionSummary>,
}

#[derive(Debug, Clone, Serialize)]
struct NativeSessionSummary {
    session_id: String,
    started_at_filename: Option<String>,
    filename: String,
    relative_path: String,
    bytes: u64,
    modified_unix_ms: Option<u64>,
}

#[derive(Debug, Clone)]
struct NativeSessionFileCandidate {
    path: PathBuf,
    root_kind: &'static str,
    summary: NativeSessionSummary,
}

#[derive(Debug, Serialize)]
struct NativeTranscriptResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    query_present: bool,
    query_redacted: bool,
    query_length: Option<usize>,
    scanned_session_file_count: usize,
    available_session_file_count: usize,
    max_files: usize,
    max_lines_per_file: usize,
    matched_session_count: usize,
    matched_line_count: u64,
    parse_error_count: usize,
    scan_error_count: usize,
    sessions: Vec<NativeTranscriptSessionPreview>,
    raw_transcript_exposed: bool,
    transcript_text_exposed: bool,
    query_text_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeTranscriptSessionPreview {
    root_kind: &'static str,
    session_id: String,
    started_at_filename: Option<String>,
    filename: String,
    relative_path: String,
    bytes: u64,
    modified_unix_ms: Option<u64>,
    line_count: u64,
    parsed_json_line_count: u64,
    parse_error_count: usize,
    truncated: bool,
    event_type_counts: Vec<NativeTranscriptEventCount>,
    redacted_events: Vec<NativeTranscriptEventPreview>,
    query_match: NativeTranscriptQueryMatch,
    read_error: Option<String>,
}

#[derive(Debug, Serialize)]
struct NativeTranscriptEventCount {
    event_type: String,
    count: u64,
}

#[derive(Debug, Serialize)]
struct NativeTranscriptEventPreview {
    line_number: usize,
    event_type: String,
    role: Option<String>,
    has_text_fields: bool,
    redacted: bool,
}

#[derive(Debug, Serialize)]
struct NativeTranscriptQueryMatch {
    matched_line_count: u64,
    first_match_line: Option<usize>,
    matched_event_type_counts: Vec<NativeTranscriptEventCount>,
}

#[derive(Debug)]
struct NativeTaskArtifactRouteSpec {
    prefix: &'static str,
    source_command: &'static str,
    artifact_kind: &'static str,
    compatibility_mode: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeEventSurface {
    Events,
    LiveEvents,
    EventsReport,
    Activity,
}

impl NativeEventSurface {
    fn source_command(self) -> &'static str {
        match self {
            Self::Events => "/events --json",
            Self::LiveEvents => "/live-events <cursor> --json",
            Self::EventsReport => "/events-report --json",
            Self::Activity => "/activity --json",
        }
    }

    fn compatibility_mode(self) -> &'static str {
        match self {
            Self::Events => "native_events_redacted",
            Self::LiveEvents => "native_live_events_redacted",
            Self::EventsReport => "native_events_report_redacted",
            Self::Activity => "native_activity_redacted",
        }
    }

    fn event_surface(self) -> &'static str {
        match self {
            Self::Events => "events",
            Self::LiveEvents => "live_events",
            Self::EventsReport => "events_report",
            Self::Activity => "activity",
        }
    }

    fn includes_activity_sessions(self) -> bool {
        self == Self::Activity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeRuntimeAuditSurface {
    SubagentObservatory,
    GatewayLedger,
    GatewayRetryDeadLetter,
    MultiAgentRuntime,
}

impl NativeRuntimeAuditSurface {
    fn source_command(self) -> &'static str {
        match self {
            Self::SubagentObservatory => "/subagent-observatory --json",
            Self::GatewayLedger => "/gateway-ledger --json",
            Self::GatewayRetryDeadLetter => "/gateway-retry-dead-letter --json",
            Self::MultiAgentRuntime => "/multi-agent-runtime --agents 4 --messages 8 --json",
        }
    }

    fn compatibility_mode(self) -> &'static str {
        match self {
            Self::SubagentObservatory => "native_subagent_observatory_redacted",
            Self::GatewayLedger => "native_gateway_ledger_redacted",
            Self::GatewayRetryDeadLetter => "native_gateway_retry_dead_letter_redacted",
            Self::MultiAgentRuntime => "native_multi_agent_runtime_redacted",
        }
    }

    fn audit_surface(self) -> &'static str {
        match self {
            Self::SubagentObservatory => "subagent_observatory",
            Self::GatewayLedger => "gateway_ledger",
            Self::GatewayRetryDeadLetter => "gateway_retry_dead_letter",
            Self::MultiAgentRuntime => "multi_agent_runtime",
        }
    }

    fn event_focus(self) -> &'static str {
        match self {
            Self::SubagentObservatory => "subagent event type counters and redacted previews",
            Self::GatewayLedger => {
                "gateway route matrix, approvals, session inventory, and event counters"
            }
            Self::GatewayRetryDeadLetter => {
                "retry, dead-letter, failure, and error event type counters"
            }
            Self::MultiAgentRuntime => "bounded multi-agent session and event inventory",
        }
    }

    fn agent_limit(self) -> Option<usize> {
        (self == Self::MultiAgentRuntime).then_some(4)
    }

    fn message_limit(self) -> Option<usize> {
        (self == Self::MultiAgentRuntime).then_some(8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeControlUiAuditSurface {
    ControlUi,
    UiContractAudit,
    GatewayDispatch,
    UiActionPlanGatewayDispatch,
    ExternalAgentBenchmark,
}

impl NativeControlUiAuditSurface {
    fn source_command(self) -> &'static str {
        match self {
            Self::ControlUi => "/control-ui --json",
            Self::UiContractAudit => "/ui-contract-audit --json",
            Self::GatewayDispatch => "/gateway-dispatch --dry-run --json",
            Self::UiActionPlanGatewayDispatch => {
                "/ui-action-plan gateway-dispatch --dry-run --json"
            }
            Self::ExternalAgentBenchmark => "/external-agent-benchmark --json",
        }
    }

    fn compatibility_mode(self) -> &'static str {
        match self {
            Self::ControlUi => "native_control_ui_shell_snapshot",
            Self::UiContractAudit => "native_ui_contract_audit",
            Self::GatewayDispatch => "native_gateway_dispatch_dry_run",
            Self::UiActionPlanGatewayDispatch => "native_ui_action_plan_gateway_dispatch",
            Self::ExternalAgentBenchmark => "native_external_agent_benchmark_redacted",
        }
    }

    fn control_surface(self) -> &'static str {
        match self {
            Self::ControlUi => "control_ui",
            Self::UiContractAudit => "ui_contract_audit",
            Self::GatewayDispatch => "gateway_dispatch",
            Self::UiActionPlanGatewayDispatch => "ui_action_plan_gateway_dispatch",
            Self::ExternalAgentBenchmark => "external_agent_benchmark",
        }
    }

    fn plan_target(self) -> Option<&'static str> {
        match self {
            Self::GatewayDispatch | Self::UiActionPlanGatewayDispatch => Some("gateway-dispatch"),
            Self::ExternalAgentBenchmark => Some("external-agent-benchmark"),
            Self::ControlUi | Self::UiContractAudit => None,
        }
    }

    fn dry_run_only(self) -> bool {
        matches!(
            self,
            Self::GatewayDispatch
                | Self::UiActionPlanGatewayDispatch
                | Self::ExternalAgentBenchmark
        )
    }

    fn read_only(self) -> bool {
        !self.dry_run_only()
    }
}

#[derive(Debug, Serialize)]
struct NativeTaskArtifactResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    artifact_kind: &'static str,
    task_id_redacted: bool,
    task_id_length: usize,
    evidence_found: bool,
    matched_session_count: usize,
    matched_line_count: u64,
    evidence_search: NativeTranscriptResponse,
    raw_task_id_exposed: bool,
    raw_transcript_exposed: bool,
    transcript_text_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeEventsResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    event_surface: &'static str,
    cursor_present: bool,
    cursor_redacted: bool,
    cursor_length: Option<usize>,
    cursor_parseable_as_u64: bool,
    scanned_session_file_count: usize,
    available_session_file_count: usize,
    max_files: usize,
    max_lines_per_file: usize,
    total_line_count: u64,
    parsed_json_line_count: u64,
    parse_error_count: usize,
    scan_error_count: usize,
    truncated_session_count: usize,
    event_type_count: usize,
    event_type_counts: Vec<NativeTranscriptEventCount>,
    recent_event_count: usize,
    recent_events: Vec<NativeEventPreview>,
    activity_sessions: Option<NativeSessionsResponse>,
    raw_transcript_exposed: bool,
    transcript_text_exposed: bool,
    raw_cursor_exposed: bool,
    cursor_text_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeEventPreview {
    root_kind: &'static str,
    session_id: String,
    started_at_filename: Option<String>,
    relative_path: String,
    line_number: usize,
    event_type: String,
    role: Option<String>,
    has_text_fields: bool,
    redacted: bool,
}

#[derive(Debug, Serialize)]
struct NativeRuntimeAuditResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    audit_surface: &'static str,
    event_focus: &'static str,
    agent_limit: Option<usize>,
    message_limit: Option<usize>,
    route_matrix_ready: bool,
    route_count: usize,
    missing_route_count: usize,
    approval_route_count: usize,
    guarded_approval_route_count: usize,
    session_file_count: u64,
    recent_session_count: usize,
    session_scan_error_count: usize,
    event_type_count: usize,
    recent_event_count: usize,
    event_scan_error_count: usize,
    subagent_event_count: u64,
    retry_or_error_event_count: u64,
    multi_agent_event_count: u64,
    sessions: NativeSessionsResponse,
    events: NativeEventsResponse,
    redaction: NativeRuntimeAuditRedaction,
    side_effects: NativeRuntimeAuditSideEffects,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeRuntimeAuditRedaction {
    raw_transcript_exposed: bool,
    transcript_text_exposed: bool,
    raw_agent_payload_exposed: bool,
    raw_error_payload_exposed: bool,
    raw_gateway_ledger_payload_exposed: bool,
}

#[derive(Debug, Serialize)]
struct NativeRuntimeAuditSideEffects {
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
}

#[derive(Debug, Serialize)]
struct NativeControlUiAuditResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    control_surface: &'static str,
    plan_target: Option<&'static str>,
    dry_run_only: bool,
    read_only: bool,
    confirmation_required_for_real_mutation: bool,
    route_matrix_ready: bool,
    route_count: usize,
    implemented_route_count: usize,
    missing_route_count: usize,
    get_route_count: usize,
    post_route_count: usize,
    dry_run_route_count: usize,
    read_only_route_count: usize,
    guarded_post_route_count: usize,
    approval_route_count: usize,
    guarded_approval_route_count: usize,
    gateway_replacement_ready: bool,
    gateway_replacement_blocker_count: usize,
    external_agent_benchmark_executed: bool,
    external_agent_spawned: bool,
    action_dispatched: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    route_matrix: ControlUiRouteParityReport,
    redaction: NativeControlUiAuditRedaction,
    side_effects: NativeControlUiAuditSideEffects,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeControlUiAuditRedaction {
    raw_transcript_exposed: bool,
    transcript_text_exposed: bool,
    raw_token_exposed: bool,
    raw_action_payload_exposed: bool,
    raw_agent_payload_exposed: bool,
}

#[derive(Debug, Serialize)]
struct NativeControlUiAuditSideEffects {
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
}

#[derive(Debug, Serialize)]
struct NativeApprovalsResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    pending_approval_count: usize,
    approval_route_count: usize,
    guarded_route_count: usize,
    approval_routes: Vec<NativeApprovalRoute>,
    raw_command_payload_exposed: bool,
    raw_approval_payload_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeApprovalRoute {
    method: &'static str,
    pattern: &'static str,
    capability: &'static str,
    source_command: &'static str,
    side_effect_boundary: &'static str,
    dry_run_only: bool,
    guarded: bool,
    confirmation_required_for_real_mutation: bool,
}

#[derive(Debug, Serialize)]
struct NativePolicyResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    loopback_bind_required: bool,
    loopback_bound: bool,
    non_loopback_override_enabled: bool,
    bind_addr: String,
    telegram_gate_summary: native_telegram::NativeTelegramGatewayGateSummary,
    gateway_replacement_ready: bool,
    gateway_replacement_blocker_count: usize,
    approval_route_count: usize,
    guarded_approval_route_count: usize,
    raw_token_exposed: bool,
    raw_transcript_exposed: bool,
    raw_update_payload_exposed: bool,
    raw_prompt_text_exposed: bool,
    raw_response_text_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeConfigResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    bind_addr: String,
    telegram_plugin_requested: bool,
    telegram_plugin_poll_ms: u64,
    default_model_present: bool,
    telegram_model_present: bool,
    openai_codex_home_present: bool,
    gateway_token_file_present: bool,
    release_build_verified: bool,
    control_ui_parity_verified: bool,
    telegram_gate_summary: native_telegram::NativeTelegramGatewayGateSummary,
    config_root_count: usize,
    config_roots: Vec<NativeConfigPathStatus>,
    raw_env_exposed: bool,
    raw_token_exposed: bool,
    raw_config_value_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeConfigPathStatus {
    label: &'static str,
    path: String,
    exists: bool,
    is_dir: bool,
    bytes: Option<u64>,
}

#[derive(Debug, Serialize)]
struct NativeOptionalConfigsResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    config_count: usize,
    missing_expected_count: usize,
    configs: Vec<NativeOptionalConfigStatus>,
    raw_config_value_exposed: bool,
    config_content_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeOptionalConfigStatus {
    label: &'static str,
    path: String,
    expected: bool,
    exists: bool,
    is_file: bool,
    bytes: Option<u64>,
    content_exposed: bool,
}

#[derive(Debug, Serialize)]
struct NativeOperatorConsoleResponse<'a> {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    health: HealthResponse,
    operator_snapshot_endpoint: &'static str,
    operator_security_endpoint: &'static str,
    sessions_endpoint: &'static str,
    session_activity_endpoint: &'static str,
    gateway_replacement_readiness: NativeGatewayReplacementReadiness,
    control_ui_route_parity: ControlUiRouteParityReport,
    sessions: NativeSessionsResponse,
    telegram_plugin: &'a NativeTelegramPluginStatus,
    telegram_poll_loop_status: native_telegram::NativeTelegramPollLoopStatus,
    telegram_live_soak_status: native_telegram::NativeTelegramLiveSoakStatus,
    raw_transcript_exposed: bool,
    raw_token_exposed: bool,
    raw_update_payload_exposed: bool,
    raw_prompt_text_exposed: bool,
    raw_response_text_exposed: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    model_invoked: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeOperatorSecurityResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    loopback_bind_required: bool,
    loopback_bound: bool,
    non_loopback_override_enabled: bool,
    bind_addr: String,
    control_ui_route_parity: ControlUiRouteParityReport,
    gateway_replacement_readiness: NativeGatewayReplacementReadiness,
    post_route_count: usize,
    dry_run_post_route_count: usize,
    guarded_post_route_count: usize,
    post_execution_readiness_endpoint: &'static str,
    post_execution_stores_endpoint: &'static str,
    post_activation_plan_endpoint: &'static str,
    post_execution_readiness: NativePostExecutionReadinessResponse,
    post_execution_stores_ready: bool,
    post_execution_stores: NativePostExecutionStoresResponse,
    post_activation_plan_ready: bool,
    post_activation_plan: NativePostActivationPlanResponse,
    post_gray_release_evidence_endpoint: &'static str,
    post_gray_release_evidence_ready: bool,
    post_gray_release_evidence: NativePostGrayReleaseEvidenceResponse,
    production_soak_ready: bool,
    telegram_gate_summary: native_telegram::NativeTelegramGatewayGateSummary,
    telegram_production_readiness_status: native_telegram::NativeTelegramProductionReadinessStatus,
    telegram_owner_handoff_endpoint: &'static str,
    telegram_owner_handoff_status: NativeTelegramOwnerHandoffStatus,
    telegram_plugin_requested: bool,
    telegram_plugin_status: &'static str,
    redaction: NativeOperatorSecurityRedaction,
    side_effects: NativeOperatorSecuritySideEffects,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeOperatorSecurityRedaction {
    raw_transcript_exposed: bool,
    raw_token_exposed: bool,
    raw_update_payload_exposed: bool,
    raw_prompt_text_exposed: bool,
    raw_response_text_exposed: bool,
    raw_idempotency_key_exposed: bool,
    raw_audit_payload_exposed: bool,
}

#[derive(Debug, Serialize)]
struct NativeOperatorSecuritySideEffects {
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    model_invoked: bool,
    message_sent: bool,
    cursor_written: bool,
}

#[derive(Debug, Clone)]
struct NativeTelegramOwnerHandoffInputs {
    legacy_config_path: Option<String>,
    legacy_config_found: bool,
    legacy_config_parse_ok: bool,
    legacy_telegram_enabled: Option<bool>,
    legacy_config_error: Option<String>,
    hepta_telegram_requested: bool,
    hepta_poll_loop_armed: bool,
    hepta_poll_loop_gate_enabled: bool,
    hepta_delivery_approval_gate_enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
struct NativeTelegramOwnerHandoffStatus {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    endpoint: &'static str,
    ready: bool,
    conflict_free: bool,
    hepta_takeover_ready: bool,
    side_effect_free: bool,
    active_owner: &'static str,
    legacy_config_path: Option<String>,
    legacy_config_found: bool,
    legacy_config_parse_ok: bool,
    legacy_telegram_enabled: Option<bool>,
    legacy_telegram_enabled_explicit: bool,
    hepta_telegram_requested: bool,
    hepta_poll_loop_armed: bool,
    hepta_poll_loop_gate_enabled: bool,
    hepta_delivery_approval_gate_enabled: bool,
    double_poller_risk: bool,
    takeover_blockers: Vec<&'static str>,
    legacy_config_error: Option<String>,
    raw_token_exposed: bool,
    raw_update_payload_exposed: bool,
    raw_prompt_text_exposed: bool,
    raw_response_text_exposed: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeGatewayReplacementReadiness {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    ready: bool,
    active_install_allowed: bool,
    side_effect_free: bool,
    blocker_count: usize,
    blockers: Vec<&'static str>,
    checks: Vec<NativeGatewayReplacementCheck>,
    required_env_gates: NativeGatewayReplacementEnvGates,
    telegram_owner_handoff_endpoint: &'static str,
    telegram_owner_handoff_status: NativeTelegramOwnerHandoffStatus,
    control_ui_route_parity: ControlUiRouteParityReport,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeGatewayReplacementCheck {
    name: &'static str,
    ready: bool,
    detail: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeGatewayReplacementEnvGates {
    delivery_approval: NativeGatewayReplacementGate,
    live_read: NativeGatewayReplacementGate,
    model_turn: NativeGatewayReplacementGate,
    send: NativeGatewayReplacementGate,
    poll_loop: NativeGatewayReplacementGate,
    in_process_model_runner: NativeGatewayReplacementGate,
    release_build_verified: NativeGatewayReplacementGate,
    control_ui_parity_verified: NativeGatewayReplacementGate,
}

#[derive(Debug, Serialize)]
struct NativeGatewayReplacementGate {
    env: &'static str,
    enabled: bool,
}

#[derive(Debug, Serialize)]
struct NativeGatewayLiveActivationPlan {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    endpoint: &'static str,
    operator_approval_required: bool,
    active_install_allowed: bool,
    readiness_blocker_count: usize,
    readiness_blockers: Vec<&'static str>,
    active_gateway_label: &'static str,
    current_legacy_binary: &'static str,
    replacement_binary: &'static str,
    bind_addr: String,
    launch_arguments: Vec<String>,
    required_env_gates: Vec<NativeGatewayLiveActivationEnv>,
    live_smoke_sequence: &'static [&'static str],
    production_replacement_sequence: &'static [&'static str],
    safety: NativeGatewayLiveActivationSafety,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeGatewayLiveActivationEnv {
    env: &'static str,
    enabled: bool,
    purpose: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeGatewayLiveActivationSafety {
    side_effect_free: bool,
    status_probe_reads_telegram: bool,
    status_probe_invokes_model: bool,
    status_probe_sends_message: bool,
    status_probe_writes_cursor: bool,
    raw_token_exposed: bool,
    raw_update_payload_exposed: bool,
    raw_prompt_text_exposed: bool,
    raw_response_text_exposed: bool,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct ControlUiRouteSpec {
    method: &'static str,
    pattern: &'static str,
    source_command: &'static str,
    capability: &'static str,
    side_effect_boundary: &'static str,
}

#[derive(Debug, Serialize)]
struct ControlUiRouteParityReport {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    ready: bool,
    route_count: usize,
    implemented_route_count: usize,
    missing_route_count: usize,
    missing_routes: Vec<String>,
    side_effect_free: bool,
    legacy_source: &'static str,
    routes: &'static [ControlUiRouteSpec],
}

#[derive(Debug, Serialize)]
struct ControlUiRouteCompatibilityResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    method: &'static str,
    pattern: &'static str,
    path: String,
    source_command: &'static str,
    capability: &'static str,
    side_effect_boundary: &'static str,
    compatibility_mode: &'static str,
    dry_run_only: bool,
    confirmation_required_for_real_mutation: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    model_invoked: bool,
    message_sent: bool,
    cursor_written: bool,
}

fn control_ui_route_parity_report() -> ControlUiRouteParityReport {
    let missing_routes = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| !control_ui_route_has_handler(route))
        .map(|route| format!("{} {}", route.method, route.pattern))
        .collect::<Vec<_>>();
    let implemented_route_count = CONTROL_UI_ROUTE_SPECS.len() - missing_routes.len();
    let ready = missing_routes.is_empty();
    ControlUiRouteParityReport {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if ready { "ready" } else { "blocked" },
        ready,
        route_count: CONTROL_UI_ROUTE_SPECS.len(),
        implemented_route_count,
        missing_route_count: missing_routes.len(),
        missing_routes,
        side_effect_free: true,
        legacy_source: "Hepta Control UI live operator DevEx 100 route matrix and hepta-core::control_ui markers",
        routes: CONTROL_UI_ROUTE_SPECS,
    }
}

fn control_ui_route_response(method: &str, path: &str) -> Option<String> {
    let route = control_ui_route_spec_for(method, path)?;
    Some(json_or_error(&ControlUiRouteCompatibilityResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if route.method == "POST" {
            "dry_run_compatibility"
        } else {
            "ready"
        },
        method: route.method,
        pattern: route.pattern,
        path: path.to_string(),
        source_command: route.source_command,
        capability: route.capability,
        side_effect_boundary: route.side_effect_boundary,
        compatibility_mode: "native_control_ui_route_parity_shell",
        dry_run_only: route.method == "POST",
        confirmation_required_for_real_mutation: route.method == "POST"
            && !route.side_effect_boundary.contains("read-only"),
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        model_invoked: false,
        message_sent: false,
        cursor_written: false,
    }))
}

fn control_ui_route_has_handler(route: &ControlUiRouteSpec) -> bool {
    let sample_path = control_ui_sample_path(route.pattern);
    control_ui_route_spec_for(route.method, &sample_path).is_some()
}

fn control_ui_route_spec_for(method: &str, path: &str) -> Option<&'static ControlUiRouteSpec> {
    CONTROL_UI_ROUTE_SPECS.iter().find(|route| {
        route.method == method && control_ui_route_pattern_matches(route.pattern, path)
    })
}

fn control_ui_route_pattern_matches(pattern: &str, path: &str) -> bool {
    if let Some(start) = pattern.find("/<") {
        let prefix = &pattern[..start + 1];
        path.starts_with(prefix) && path.len() > prefix.len()
    } else {
        pattern == path
    }
}

fn control_ui_sample_path(pattern: &str) -> String {
    pattern
        .replace("<action>", "gateway-dispatch")
        .replace("<id>", "gateway-status")
        .replace("<query>", "sample")
        .replace("<task_id>", "sample-task")
        .replace("<cursor>", "0")
}

fn allow_non_loopback_ui() -> bool {
    env_truthy("HEPTA_ALLOW_NON_LOOPBACK_UI")
}

fn env_truthy(name: &str) -> bool {
    env::var(name)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn env_u64(name: &str) -> Option<u64> {
    env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
}

fn is_loopback_bind_addr(bind_addr: &str) -> bool {
    let host = bind_addr
        .rsplit_once(':')
        .map(|(host, _)| host.trim_matches(['[', ']']))
        .unwrap_or(bind_addr)
        .trim();
    matches!(host, "127.0.0.1" | "localhost" | "::1")
}

fn read_http_request(stream: &mut TcpStream) -> Result<String> {
    let mut buffer = [0_u8; 8192];
    let mut bytes = Vec::new();
    let first_read = stream.read(&mut buffer).context("read request")?;
    bytes.extend_from_slice(&buffer[..first_read]);
    let content_length = String::from_utf8_lossy(&bytes)
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            if name.eq_ignore_ascii_case("content-length") {
                value.trim().parse::<usize>().ok()
            } else {
                None
            }
        })
        .unwrap_or(0);
    while request_body_bytes(&bytes).len() < content_length {
        let read = stream.read(&mut buffer).context("read request body")?;
        if read == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..read]);
    }
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

fn request_body_bytes(bytes: &[u8]) -> &[u8] {
    bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| &bytes[index + 4..])
        .unwrap_or(&[])
}

fn request_body_text(request: &str) -> Option<&str> {
    request
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .filter(|body| !body.is_empty())
}

fn request_method_and_path(request: &str) -> Option<(&str, &str)> {
    let first_line = request.lines().next()?;
    let mut parts = first_line.split_whitespace();
    let method = parts.next()?;
    let raw_path = parts.next()?;
    Some((method, raw_path.split('?').next().unwrap_or(raw_path)))
}

fn write_http_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> Result<()> {
    let header = format!(
        "HTTP/1.1 {status}\r\ncontent-type: {content_type}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
        body.len()
    );
    stream
        .write_all(header.as_bytes())
        .context("write header")?;
    stream.write_all(body).context("write body")?;
    stream.flush().context("flush response")?;
    Ok(())
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_serve_ui_defaults_to_loopback() {
        let args = vec!["--serve-ui".to_string()];
        let options = parse_serve_ui_args(&args)
            .expect("parse")
            .expect("serve ui options");
        assert_eq!(options.bind_addr, DEFAULT_BIND_ADDR);
        assert!(!options.with_telegram_plugin);
        assert_eq!(options.telegram_plugin_poll_ms, DEFAULT_TELEGRAM_POLL_MS);
    }

    #[test]
    fn parse_serve_ui_accepts_launchd_gateway_flags() {
        let args = vec![
            "--serve-ui".to_string(),
            "127.0.0.1:7777".to_string(),
            "--with-telegram-plugin".to_string(),
            "--telegram-plugin-poll-ms".to_string(),
            "250".to_string(),
        ];
        let options = parse_serve_ui_args(&args)
            .expect("parse")
            .expect("serve ui options");
        assert_eq!(options.bind_addr, "127.0.0.1:7777");
        assert!(options.with_telegram_plugin);
        assert_eq!(options.telegram_plugin_poll_ms, 500);
    }

    #[test]
    fn parse_serve_ui_rejects_unknown_args() {
        let args = vec!["--serve-ui".to_string(), "--unknown".to_string()];
        let err = parse_serve_ui_args(&args).expect_err("unknown arg should fail");
        assert!(err.to_string().contains("unexpected --serve-ui argument"));
    }

    #[test]
    fn native_gateway_readiness_exposes_pending_telegram_migration() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let telegram_plugin =
            native_telegram::telegram_plugin_status(true, options.telegram_plugin_poll_ms);
        let body = native_gateway_json(&options, &telegram_plugin);
        assert!(body.contains(r#""runtime":"hepta-codex""#));
        assert!(body.contains(r#""launchd_entrypoint_compatible":true"#));
        assert!(body.contains(r#""active_gateway_replacement_ready":false"#));
        assert!(body.contains(
            r#""gateway_replacement_readiness_endpoint":"/api/gateway-replacement-readiness""#
        ));
        assert!(body.contains(r#""gateway_route_core_status":{"source_crate":"hepta-gateway""#));
        assert!(body.contains(r#""route_core_ready":true"#));
        assert!(body.contains(
            r#""gateway_live_activation_plan_endpoint":"/api/gateway-live-activation-plan""#
        ));
        assert!(body.contains(r#""operator_approval_required":true"#));
        assert!(
            body.contains(r#""control_ui_route_parity_endpoint":"/api/control-ui-route-parity""#)
        );
        assert!(body.contains(r#""control_ui_route_parity_ready":true"#));
        assert!(body.contains(r#""status":"blocked""#));
        assert!(body.contains(r#""active_install_allowed":false"#));
        assert!(body.contains(r#""in_process_model_runner_ready""#));
        assert!(body.contains(r#""release_build_verified""#));
        assert!(body.contains(r#""control_ui_route_matrix_ready""#));
        assert!(body.contains(r#""control_ui_route_parity_verified""#));
        assert!(body.contains(r#""telegram_plugin_native_supervisor_ready":"#));
        assert!(body.contains(r#""telegram_receive_once_endpoint":"/api/telegram-receive-once""#));
        assert!(body.contains(r#""telegram_model_bridge_endpoint":"/api/telegram-model-bridge""#));
        assert!(body.contains(r#""telegram_send_plan_endpoint":"/api/telegram-send-plan""#));
        assert!(body.contains(r#""telegram_drain_once_endpoint":"/api/telegram-drain-once""#));
        assert!(body.contains(r#""telegram_poll_loop_endpoint":"/api/telegram-poll-loop""#));
        assert!(body.contains(r#""telegram_live_soak_endpoint":"/api/telegram-live-soak""#));
        assert!(
            body.contains(
                r#""telegram_live_soak_status_endpoint":"/api/telegram-live-soak-status""#
            )
        );
        assert!(body.contains(
            r#""telegram_production_readiness_endpoint":"/api/telegram-production-readiness""#
        ));
        assert!(
            body.contains(r#""telegram_delivery_ledger_endpoint":"/api/telegram-delivery-ledger""#)
        );
        assert!(
            body.contains(r#""telegram_owner_handoff_endpoint":"/api/telegram-owner-handoff""#)
        );
        assert!(body.contains(r#""side_effect_free":true"#));
        assert!(body.contains(r#""production_guards""#));
        assert!(body.contains(r#""poll_loop_gate_env":"HEPTA_NATIVE_TELEGRAM_POLL_LOOP""#));
        assert!(body.contains(r#""worker_spawned_by_status":false"#));
        assert!(body.contains(r#""telegram_cursor_endpoint":"/api/telegram-cursor""#));
        assert!(body.contains(r#""telegram_readiness_summary_side_effect_free":true"#));
        assert!(body.contains(r#""readiness_summary_performs_live_read":false"#));
        assert!(body.contains(r#""readiness_summary_invokes_model":false"#));
        assert!(body.contains(r#""readiness_summary_sends_message":false"#));
        assert!(!body.contains("pending_migration"));
    }

    #[test]
    fn telegram_live_soak_endpoint_is_side_effect_free() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", "/api/telegram-live-soak", &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect("live soak json");
        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["raw_update_payload_exposed"], false);
        assert_eq!(value["raw_prompt_text_exposed"], false);
        assert_eq!(value["raw_response_text_exposed"], false);
        assert_eq!(value["raw_token_exposed"], false);
        assert_eq!(value["poll_loop_status"]["worker_spawned_by_status"], false);
        assert_eq!(
            value["production_guards"]["retry_transient_send_errors"],
            true
        );
    }

    #[test]
    fn telegram_live_soak_status_alias_matches_primary_endpoint() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let primary = route_native_gateway_request("GET", TELEGRAM_LIVE_SOAK_ENDPOINT, &options);
        let alias =
            route_native_gateway_request("GET", TELEGRAM_LIVE_SOAK_STATUS_ENDPOINT, &options);

        assert_eq!(primary.0, "200 OK");
        assert_eq!(alias.0, "200 OK");
        assert_eq!(primary.1, alias.1);

        let primary_json: serde_json::Value =
            serde_json::from_str(&primary.2).expect("primary live soak json");
        let alias_json: serde_json::Value =
            serde_json::from_str(&alias.2).expect("alias live soak json");
        assert_eq!(alias_json["endpoint"], TELEGRAM_LIVE_SOAK_ENDPOINT);
        assert_eq!(alias_json["side_effect_free"], true);
        assert_eq!(alias_json["raw_token_exposed"], false);
        assert_eq!(
            primary_json["production_guards"],
            alias_json["production_guards"]
        );
    }

    #[test]
    fn telegram_production_readiness_endpoint_is_side_effect_free() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", TELEGRAM_PRODUCTION_READINESS_ENDPOINT, &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value =
            serde_json::from_str(&body).expect("production readiness json");
        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["raw_update_payload_exposed"], false);
        assert_eq!(value["raw_prompt_text_exposed"], false);
        assert_eq!(value["raw_response_text_exposed"], false);
        assert_eq!(value["raw_token_exposed"], false);
        assert_eq!(
            value["min_poll_iterations_env"],
            "HEPTA_NATIVE_TELEGRAM_SOAK_MIN_POLLS"
        );
        assert_eq!(
            value["max_attention_count_env"],
            "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_ATTENTION"
        );
    }

    #[test]
    fn telegram_delivery_ledger_endpoint_is_read_only_and_redacted() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", TELEGRAM_DELIVERY_LEDGER_ENDPOINT, &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect("delivery ledger json");
        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["requested"], true);
        assert_eq!(
            value["ledger_path"],
            ".hepta/telegram/delivery-ledger.jsonl"
        );
        assert_eq!(value["raw_response_text_logged"], false);
        assert_eq!(value["raw_chat_id_logged"], false);
        assert_eq!(value["raw_message_id_logged"], false);
        assert_eq!(value["raw_token_logged"], false);
    }

    #[test]
    fn telegram_owner_handoff_endpoint_is_side_effect_free() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", TELEGRAM_OWNER_HANDOFF_ENDPOINT, &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect("owner handoff json");
        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["endpoint"], TELEGRAM_OWNER_HANDOFF_ENDPOINT);
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["raw_token_exposed"], false);
        assert_eq!(value["raw_update_payload_exposed"], false);
        assert_eq!(value["raw_prompt_text_exposed"], false);
        assert_eq!(value["raw_response_text_exposed"], false);
        assert!(value["takeover_blockers"].is_array());
    }

    #[test]
    fn telegram_owner_handoff_detects_double_poller_risk() {
        let status = telegram_owner_handoff_status_from_inputs(NativeTelegramOwnerHandoffInputs {
            legacy_config_path: Some("/tmp/openclaw.json".to_string()),
            legacy_config_found: true,
            legacy_config_parse_ok: true,
            legacy_telegram_enabled: Some(true),
            legacy_config_error: None,
            hepta_telegram_requested: true,
            hepta_poll_loop_armed: true,
            hepta_poll_loop_gate_enabled: true,
            hepta_delivery_approval_gate_enabled: true,
        });

        assert_eq!(status.status, "conflict_risk");
        assert_eq!(status.active_owner, "conflict_risk");
        assert!(!status.ready);
        assert!(!status.conflict_free);
        assert!(!status.hepta_takeover_ready);
        assert!(status.double_poller_risk);
        assert!(
            status
                .takeover_blockers
                .contains(&"legacy_openclaw_telegram_enabled")
        );
        assert!(!status.raw_token_exposed);
    }

    #[test]
    fn telegram_owner_handoff_allows_hepta_only_after_legacy_disabled() {
        let status = telegram_owner_handoff_status_from_inputs(NativeTelegramOwnerHandoffInputs {
            legacy_config_path: Some("/tmp/openclaw.json".to_string()),
            legacy_config_found: true,
            legacy_config_parse_ok: true,
            legacy_telegram_enabled: Some(false),
            legacy_config_error: None,
            hepta_telegram_requested: true,
            hepta_poll_loop_armed: true,
            hepta_poll_loop_gate_enabled: true,
            hepta_delivery_approval_gate_enabled: true,
        });

        assert_eq!(status.status, "hepta_takeover_ready");
        assert_eq!(status.active_owner, "hepta");
        assert!(status.ready);
        assert!(status.conflict_free);
        assert!(status.hepta_takeover_ready);
        assert!(!status.double_poller_risk);
        assert!(status.takeover_blockers.is_empty());
    }

    #[test]
    fn gateway_replacement_readiness_endpoint_reports_blockers_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", "/api/gateway-replacement-readiness", &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect("readiness json");
        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["ready"], false);
        assert_eq!(value["active_install_allowed"], false);
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(
            value["required_env_gates"]["live_read"]["env"],
            native_telegram::TELEGRAM_LIVE_READ_ENV
        );
        let blockers = value["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .filter_map(|value| value.as_str())
            .collect::<Vec<_>>();
        assert!(blockers.contains(&"in_process_model_runner_ready"));
        assert!(blockers.contains(&"release_build_verified"));
        assert!(blockers.contains(&"control_ui_route_parity_verified"));
        assert!(!blockers.contains(&"control_ui_route_matrix_ready"));
        assert!(value["telegram_owner_handoff_status"].is_object());
        assert_eq!(
            value["telegram_owner_handoff_endpoint"],
            TELEGRAM_OWNER_HANDOFF_ENDPOINT
        );
        assert_eq!(value["control_ui_route_parity"]["ready"], true);
        assert!(
            value["control_ui_route_parity"]["route_count"]
                .as_u64()
                .expect("route count")
                >= 40
        );
    }

    #[test]
    fn gateway_live_activation_plan_is_side_effect_free_and_lists_operator_gates() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT, &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value = serde_json::from_str(&body).expect("activation plan json");
        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["operator_approval_required"], true);
        assert_eq!(value["active_gateway_label"], ACTIVE_GATEWAY_LABEL);
        assert_eq!(value["replacement_binary"], HEPTA_CODEX_RELEASE_BINARY);
        assert_eq!(
            value["safety"]["status_probe_reads_telegram"], false,
            "activation planning must not read Telegram"
        );
        assert_eq!(value["safety"]["status_probe_invokes_model"], false);
        assert_eq!(value["safety"]["status_probe_sends_message"], false);
        assert_eq!(value["safety"]["status_probe_writes_cursor"], false);
        let envs = value["required_env_gates"]
            .as_array()
            .expect("required env gates")
            .iter()
            .filter_map(|item| item["env"].as_str())
            .collect::<Vec<_>>();
        assert!(envs.contains(&native_telegram::TELEGRAM_DELIVERY_APPROVED_ENV));
        assert!(envs.contains(&native_telegram::TELEGRAM_LIVE_READ_ENV));
        assert!(envs.contains(&native_telegram::TELEGRAM_MODEL_TURN_GATE_ENV));
        assert!(envs.contains(&native_telegram::TELEGRAM_SEND_GATE_ENV));
        assert!(envs.contains(&native_telegram::TELEGRAM_POLL_LOOP_ENV));
        assert!(envs.contains(&native_telegram::TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV));
        assert!(envs.contains(&RELEASE_BUILD_VERIFIED_ENV));
        assert!(envs.contains(&CONTROL_UI_PARITY_VERIFIED_ENV));
    }

    #[test]
    fn control_ui_route_parity_report_covers_old_hepta_routes() {
        let report = control_ui_route_parity_report();
        assert!(report.ready);
        assert_eq!(report.missing_route_count, 0);
        assert!(report.route_count >= 40);
        let routes = report
            .routes
            .iter()
            .map(|route| format!("{} {}", route.method, route.pattern))
            .collect::<Vec<_>>();
        assert!(routes.contains(&"GET /api/operator-console".to_string()));
        assert!(routes.contains(&"GET /api/query-transcript/<query>".to_string()));
        assert!(routes.contains(&"POST /api/commands/<id>".to_string()));
        assert!(routes.contains(&"POST /api/actions/<action>".to_string()));
        assert!(routes.contains(&"POST /api/chat".to_string()));
        assert!(routes.contains(&"GET /api/external-agent-benchmark".to_string()));
        assert!(routes.contains(&"GET /api/telegram-production-readiness".to_string()));
        assert!(routes.contains(&"GET /api/telegram-delivery-ledger".to_string()));
    }

    #[test]
    fn control_ui_legacy_routes_are_reachable_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        for (method, path) in [
            ("GET", "/api/operator-console"),
            ("GET", "/api/query-transcript/sample"),
            ("GET", "/api/task/sample-task"),
            ("GET", "/api/live-events/0"),
            ("GET", "/api/external-agent-benchmark"),
            ("POST", "/api/actions/gateway-dispatch"),
            ("POST", "/api/commands/gateway-status"),
            ("POST", "/api/chat"),
        ] {
            let (status, content_type, body) = route_native_gateway_request(method, path, &options);
            assert_eq!(status, "200 OK", "{method} {path}");
            assert_eq!(content_type, "application/json; charset=utf-8");
            let value: serde_json::Value = serde_json::from_str(&body).expect("compat route json");
            assert_eq!(value["runtime"], "hepta-codex");
            assert_eq!(value["external_side_effects"], false);
            assert_eq!(value["gateway_mutation_performed"], false);
            assert_eq!(value["telegram_read_performed"], false);
            assert_eq!(value["model_invoked"], false);
            assert_eq!(value["message_sent"], false);
            assert_eq!(value["cursor_written"], false);
        }
    }

    #[test]
    fn operator_snapshot_returns_native_aggregate_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", "/api/operator-snapshot", &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect("operator snapshot json");
        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], "native_operator_snapshot");
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["telegram_read_performed"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
        assert_eq!(value["raw_token_exposed"], false);
        assert_eq!(value["health"]["status"], "ready");
        assert!(value["gateway_replacement_readiness"].is_object());
        assert!(value["control_ui_route_parity"].is_object());
        assert!(value["telegram_plugin"].is_object());
        assert!(value["telegram_live_soak_status"].is_object());
        assert_ne!(
            value["compatibility_mode"],
            "native_control_ui_route_parity_shell"
        );
    }

    #[test]
    fn native_sessions_inventory_scans_metadata_without_transcript_text() {
        let temp = tempfile::tempdir().expect("tempdir");
        let sessions = temp.path().join("sessions/2026/05/18");
        std::fs::create_dir_all(&sessions).expect("create sessions dir");
        std::fs::write(
            sessions.join("rollout-2026-05-18T10-31-22-019e38e5-4a20-7000-a111-222222222222.jsonl"),
            r#"{"item":{"type":"message","text":"do-not-expose-transcript"}}"#,
        )
        .expect("write rollout");
        std::fs::write(sessions.join("ignored.jsonl"), "{}").expect("write ignored");

        let report = native_sessions_report(
            vec![NativeSessionRootCandidate {
                root: temp.path().join("sessions"),
                kind: "active",
            }],
            "/sessions --json",
            "native_sessions_inventory",
        );
        let body = serde_json::to_string(&report).expect("serialize sessions report");

        assert_eq!(report.status, "ready");
        assert_eq!(report.session_file_count, 1);
        assert_eq!(report.recent_session_count, 1);
        assert_eq!(report.raw_transcript_exposed, false);
        assert_eq!(report.transcript_text_exposed, false);
        assert_eq!(
            report.recent_sessions[0].session_id,
            "019e38e5-4a20-7000-a111-222222222222"
        );
        assert_eq!(
            report.recent_sessions[0].started_at_filename.as_deref(),
            Some("2026-05-18T10-31-22")
        );
        assert!(!body.contains("do-not-expose-transcript"));
    }

    #[test]
    fn sessions_routes_return_native_inventory_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        for (path, mode) in [
            ("/api/sessions", "native_sessions_inventory"),
            ("/api/session-activity", "native_session_activity"),
        ] {
            let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
            assert_eq!(status, "200 OK", "{path}");
            assert_eq!(content_type, "application/json; charset=utf-8");
            let value: serde_json::Value =
                serde_json::from_str(&body).expect("sessions route json");
            assert_eq!(value["runtime"], "hepta-codex");
            assert_eq!(value["native_route"], true);
            assert_eq!(value["compatibility_mode"], mode);
            assert_eq!(value["side_effect_free"], true);
            assert_eq!(value["external_side_effects"], false);
            assert_eq!(value["gateway_mutation_performed"], false);
            assert_eq!(value["telegram_read_performed"], false);
            assert_eq!(value["model_invoked"], false);
            assert_eq!(value["message_sent"], false);
            assert_eq!(value["cursor_written"], false);
            assert_eq!(value["raw_transcript_exposed"], false);
            assert_eq!(value["transcript_text_exposed"], false);
            assert_ne!(
                value["compatibility_mode"],
                "native_control_ui_route_parity_shell"
            );
        }
    }

    #[test]
    fn transcript_preview_redacts_text_and_query() {
        let temp = tempfile::tempdir().expect("tempdir");
        let sessions = temp.path().join("sessions/2026/05/18");
        std::fs::create_dir_all(&sessions).expect("create sessions dir");
        std::fs::write(
            sessions.join(
                "rollout-2026-05-18T11-12-03-019e38f3-1111-7000-a111-333333333333.jsonl",
            ),
            concat!(
                r#"{"timestamp":"2026-05-18T03:12:03Z","type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"super-secret-query-marker"}]}}"#,
                "\n",
                r#"{"timestamp":"2026-05-18T03:12:04Z","type":"event_msg","payload":{"type":"token_count","info":null}}"#,
                "\n",
            ),
        )
        .expect("write rollout");

        let report = native_transcript_report(
            vec![NativeSessionRootCandidate {
                root: temp.path().join("sessions"),
                kind: "active",
            }],
            Some("super-secret-query-marker"),
            5,
        );
        let body = serde_json::to_string(&report).expect("serialize transcript report");

        assert_eq!(report.status, "ready");
        assert_eq!(report.query_present, true);
        assert_eq!(report.query_redacted, true);
        assert_eq!(report.query_length, Some("super-secret-query-marker".len()));
        assert_eq!(report.matched_session_count, 1);
        assert_eq!(report.matched_line_count, 1);
        assert_eq!(report.raw_transcript_exposed, false);
        assert_eq!(report.transcript_text_exposed, false);
        assert_eq!(report.query_text_exposed, false);
        assert_eq!(report.sessions[0].line_count, 2);
        assert_eq!(report.sessions[0].redacted_events[0].redacted, true);
        assert_eq!(report.sessions[0].redacted_events[0].has_text_fields, true);
        assert!(!body.contains("super-secret-query-marker"));
        assert!(!body.contains("input_text"));
    }

    #[test]
    fn transcript_routes_return_native_redacted_preview_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        for (path, mode, query_present) in [
            (
                "/api/transcript",
                "native_transcript_redacted_preview",
                false,
            ),
            (
                "/api/query-transcript/sample-secret-query",
                "native_query_transcript_redacted",
                true,
            ),
        ] {
            let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
            assert_eq!(status, "200 OK", "{path}");
            assert_eq!(content_type, "application/json; charset=utf-8");
            assert!(!body.contains("sample-secret-query"));
            let value: serde_json::Value =
                serde_json::from_str(&body).expect("transcript route json");
            assert_eq!(value["runtime"], "hepta-codex");
            assert_eq!(value["native_route"], true);
            assert_eq!(value["compatibility_mode"], mode);
            assert_eq!(value["side_effect_free"], true);
            assert_eq!(value["query_present"], query_present);
            assert_eq!(value["raw_transcript_exposed"], false);
            assert_eq!(value["transcript_text_exposed"], false);
            assert_eq!(value["query_text_exposed"], false);
            assert_eq!(value["external_side_effects"], false);
            assert_eq!(value["gateway_mutation_performed"], false);
            assert_eq!(value["telegram_read_performed"], false);
            assert_eq!(value["model_invoked"], false);
            assert_eq!(value["message_sent"], false);
            assert_eq!(value["cursor_written"], false);
            assert_ne!(
                value["compatibility_mode"],
                "native_control_ui_route_parity_shell"
            );
        }
    }

    #[test]
    fn task_artifact_report_redacts_task_id_and_transcript_text() {
        let temp = tempfile::tempdir().expect("tempdir");
        let sessions = temp.path().join("sessions/2026/05/18");
        std::fs::create_dir_all(&sessions).expect("create sessions dir");
        std::fs::write(
            sessions.join(
                "rollout-2026-05-18T11-40-00-019e38f8-2222-7000-a111-444444444444.jsonl",
            ),
            concat!(
                r#"{"timestamp":"2026-05-18T03:40:00Z","type":"event_msg","payload":{"type":"agent_message","message":"task-secret-123 produced confidential patch text"}}"#,
                "\n",
            ),
        )
        .expect("write rollout");

        let transcript = native_transcript_report(
            vec![NativeSessionRootCandidate {
                root: temp.path().join("sessions"),
                kind: "active",
            }],
            Some("task-secret-123"),
            20,
        );
        let response = NativeTaskArtifactResponse {
            product: "Hepta",
            runtime: "hepta-codex",
            status: "ready",
            source_command: "/task <task_id> --json",
            native_route: true,
            compatibility_mode: "native_task_drilldown_redacted",
            side_effect_free: true,
            artifact_kind: "task_drilldown",
            task_id_redacted: true,
            task_id_length: "task-secret-123".len(),
            evidence_found: transcript.matched_line_count > 0,
            matched_session_count: transcript.matched_session_count,
            matched_line_count: transcript.matched_line_count,
            evidence_search: transcript,
            raw_task_id_exposed: false,
            raw_transcript_exposed: false,
            transcript_text_exposed: false,
            model_invoked: false,
            external_side_effects: false,
            gateway_mutation_performed: false,
            telegram_read_performed: false,
            message_sent: false,
            cursor_written: false,
            next_migration_slice: "test",
        };
        let body = serde_json::to_string(&response).expect("serialize task response");

        assert!(response.evidence_found);
        assert_eq!(response.matched_line_count, 1);
        assert_eq!(response.task_id_redacted, true);
        assert_eq!(response.raw_task_id_exposed, false);
        assert_eq!(response.transcript_text_exposed, false);
        assert!(!body.contains("task-secret-123"));
        assert!(!body.contains("confidential patch text"));
    }

    #[test]
    fn task_artifact_routes_return_native_redacted_search_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        for (path, mode, artifact_kind) in [
            (
                "/api/task/sample-secret-task",
                "native_task_drilldown_redacted",
                "task_drilldown",
            ),
            (
                "/api/task-patches/sample-secret-task",
                "native_task_patches_redacted",
                "task_patches",
            ),
            (
                "/api/task-evidence/sample-secret-task",
                "native_task_evidence_redacted",
                "task_evidence",
            ),
            (
                "/api/task-replay/sample-secret-task",
                "native_task_replay_redacted",
                "task_replay",
            ),
            (
                "/api/promotion-ledger/sample-secret-task",
                "native_promotion_ledger_redacted",
                "promotion_ledger",
            ),
            (
                "/api/handoff-bundle/sample-secret-task",
                "native_handoff_bundle_redacted",
                "handoff_bundle",
            ),
        ] {
            let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
            assert_eq!(status, "200 OK", "{path}");
            assert_eq!(content_type, "application/json; charset=utf-8");
            assert!(!body.contains("sample-secret-task"));
            let value: serde_json::Value =
                serde_json::from_str(&body).expect("task artifact route json");
            assert_eq!(value["runtime"], "hepta-codex");
            assert_eq!(value["native_route"], true);
            assert_eq!(value["compatibility_mode"], mode);
            assert_eq!(value["artifact_kind"], artifact_kind);
            assert_eq!(value["side_effect_free"], true);
            assert_eq!(value["task_id_redacted"], true);
            assert_eq!(value["raw_task_id_exposed"], false);
            assert_eq!(value["raw_transcript_exposed"], false);
            assert_eq!(value["transcript_text_exposed"], false);
            assert_eq!(value["external_side_effects"], false);
            assert_eq!(value["gateway_mutation_performed"], false);
            assert_eq!(value["telegram_read_performed"], false);
            assert_eq!(value["model_invoked"], false);
            assert_eq!(value["message_sent"], false);
            assert_eq!(value["cursor_written"], false);
            assert_eq!(value["evidence_search"]["query_text_exposed"], false);
            assert_ne!(
                value["compatibility_mode"],
                "native_control_ui_route_parity_shell"
            );
        }
    }

    #[test]
    fn event_report_redacts_cursor_and_transcript_text() {
        let temp = tempfile::tempdir().expect("tempdir");
        let sessions = temp.path().join("sessions/2026/05/18");
        std::fs::create_dir_all(&sessions).expect("create sessions dir");
        std::fs::write(
            sessions.join(
                "rollout-2026-05-18T12-10-00-019e3900-3333-7000-a111-555555555555.jsonl",
            ),
            concat!(
                r#"{"timestamp":"2026-05-18T04:10:00Z","type":"event_msg","payload":{"type":"agent_message","role":"assistant","message":"confidential event text"}}"#,
                "\n",
                r#"{"timestamp":"2026-05-18T04:10:01Z","type":"response_item","payload":{"type":"token_count","count":7}}"#,
                "\n",
            ),
        )
        .expect("write rollout");

        let report = native_events_report(
            vec![NativeSessionRootCandidate {
                root: temp.path().join("sessions"),
                kind: "active",
            }],
            NativeEventSurface::LiveEvents,
            Some("secret-live-cursor"),
        );
        let body = serde_json::to_string(&report).expect("serialize events report");

        assert_eq!(report.status, "ready");
        assert_eq!(report.native_route, true);
        assert_eq!(report.compatibility_mode, "native_live_events_redacted");
        assert_eq!(report.cursor_present, true);
        assert_eq!(report.cursor_redacted, true);
        assert_eq!(report.cursor_length, Some("secret-live-cursor".len()));
        assert_eq!(report.raw_cursor_exposed, false);
        assert_eq!(report.cursor_text_exposed, false);
        assert_eq!(report.raw_transcript_exposed, false);
        assert_eq!(report.transcript_text_exposed, false);
        assert_eq!(report.total_line_count, 2);
        assert_eq!(report.parsed_json_line_count, 2);
        assert_eq!(report.recent_event_count, 2);
        assert!(
            report
                .event_type_counts
                .iter()
                .any(|count| count.event_type == "event_msg:agent_message" && count.count == 1)
        );
        assert!(!body.contains("secret-live-cursor"));
        assert!(!body.contains("confidential event text"));
    }

    #[test]
    fn event_and_activity_routes_return_native_redacted_views_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        for (path, mode, surface, cursor_present) in [
            ("/api/events", "native_events_redacted", "events", false),
            (
                "/api/live-events/sample-secret-cursor",
                "native_live_events_redacted",
                "live_events",
                true,
            ),
            (
                "/api/events-report",
                "native_events_report_redacted",
                "events_report",
                false,
            ),
            (
                "/api/activity",
                "native_activity_redacted",
                "activity",
                false,
            ),
        ] {
            let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
            assert_eq!(status, "200 OK", "{path}");
            assert_eq!(content_type, "application/json; charset=utf-8");
            assert!(!body.contains("sample-secret-cursor"));
            let value: serde_json::Value = serde_json::from_str(&body).expect("events route json");
            assert_eq!(value["runtime"], "hepta-codex");
            assert_eq!(value["native_route"], true);
            assert_eq!(value["compatibility_mode"], mode);
            assert_eq!(value["event_surface"], surface);
            assert_eq!(value["side_effect_free"], true);
            assert_eq!(value["cursor_present"], cursor_present);
            assert_eq!(value["raw_cursor_exposed"], false);
            assert_eq!(value["cursor_text_exposed"], false);
            assert_eq!(value["raw_transcript_exposed"], false);
            assert_eq!(value["transcript_text_exposed"], false);
            assert_eq!(value["external_side_effects"], false);
            assert_eq!(value["gateway_mutation_performed"], false);
            assert_eq!(value["telegram_read_performed"], false);
            assert_eq!(value["model_invoked"], false);
            assert_eq!(value["message_sent"], false);
            assert_eq!(value["cursor_written"], false);
            assert_ne!(
                value["compatibility_mode"],
                "native_control_ui_route_parity_shell"
            );
            if path == "/api/activity" {
                assert!(value["activity_sessions"].is_object());
            } else {
                assert!(value["activity_sessions"].is_null());
            }
        }
    }

    #[test]
    fn runtime_audit_report_counts_error_like_events_without_payloads() {
        let temp = tempfile::tempdir().expect("tempdir");
        let sessions = temp.path().join("sessions/2026/05/18");
        std::fs::create_dir_all(&sessions).expect("create sessions dir");
        std::fs::write(
            sessions.join(
                "rollout-2026-05-18T13-40-00-019e3900-3333-7000-a111-666666666666.jsonl",
            ),
            concat!(
                r#"{"timestamp":"2026-05-18T05:40:00Z","type":"event_msg","payload":{"type":"runtime_error","message":"super-secret-error-payload"}}"#,
                "\n",
                r#"{"timestamp":"2026-05-18T05:40:01Z","type":"event_msg","payload":{"type":"agent_message","role":"assistant","message":"subagent-secret-text"}}"#,
                "\n",
            ),
        )
        .expect("write rollout");

        let report = native_runtime_audit_report(
            vec![NativeSessionRootCandidate {
                root: temp.path().join("sessions"),
                kind: "active",
            }],
            NativeRuntimeAuditSurface::GatewayRetryDeadLetter,
        );
        let body = serde_json::to_string(&report).expect("serialize runtime audit report");

        assert_eq!(report.status, "ready");
        assert_eq!(report.native_route, true);
        assert_eq!(
            report.compatibility_mode,
            "native_gateway_retry_dead_letter_redacted"
        );
        assert_eq!(report.audit_surface, "gateway_retry_dead_letter");
        assert_eq!(report.retry_or_error_event_count, 1);
        assert_eq!(report.subagent_event_count, 1);
        assert_eq!(report.redaction.raw_error_payload_exposed, false);
        assert_eq!(report.redaction.raw_agent_payload_exposed, false);
        assert_eq!(report.redaction.transcript_text_exposed, false);
        assert_eq!(report.side_effects.gateway_mutation_performed, false);
        assert_eq!(report.side_effects.telegram_read_performed, false);
        assert_eq!(report.side_effects.model_invoked, false);
        assert_eq!(report.side_effects.message_sent, false);
        assert_eq!(report.side_effects.cursor_written, false);
        assert!(!body.contains("super-secret-error-payload"));
        assert!(!body.contains("subagent-secret-text"));
    }

    #[test]
    fn runtime_audit_routes_return_native_redacted_views_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        for (path, mode, audit_surface, agent_limit, message_limit) in [
            (
                "/api/subagent-observatory",
                "native_subagent_observatory_redacted",
                "subagent_observatory",
                None,
                None,
            ),
            (
                "/api/gateway-ledger",
                "native_gateway_ledger_redacted",
                "gateway_ledger",
                None,
                None,
            ),
            (
                "/api/gateway-retry-dead-letter",
                "native_gateway_retry_dead_letter_redacted",
                "gateway_retry_dead_letter",
                None,
                None,
            ),
            (
                "/api/multi-agent-runtime",
                "native_multi_agent_runtime_redacted",
                "multi_agent_runtime",
                Some(4),
                Some(8),
            ),
        ] {
            let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
            assert_eq!(status, "200 OK", "{path}");
            assert_eq!(content_type, "application/json; charset=utf-8");
            let value: serde_json::Value =
                serde_json::from_str(&body).expect("runtime audit route json");

            assert_eq!(value["runtime"], "hepta-codex");
            assert_eq!(value["native_route"], true);
            assert_eq!(value["compatibility_mode"], mode);
            assert_eq!(value["audit_surface"], audit_surface);
            assert_eq!(value["side_effect_free"], true);
            assert_eq!(value["sessions"]["native_route"], true);
            assert_eq!(value["events"]["native_route"], true);
            assert_eq!(value["redaction"]["raw_transcript_exposed"], false);
            assert_eq!(value["redaction"]["transcript_text_exposed"], false);
            assert_eq!(value["redaction"]["raw_agent_payload_exposed"], false);
            assert_eq!(value["redaction"]["raw_error_payload_exposed"], false);
            assert_eq!(
                value["redaction"]["raw_gateway_ledger_payload_exposed"],
                false
            );
            assert_eq!(value["side_effects"]["external_side_effects"], false);
            assert_eq!(value["side_effects"]["gateway_mutation_performed"], false);
            assert_eq!(value["side_effects"]["telegram_read_performed"], false);
            assert_eq!(value["side_effects"]["model_invoked"], false);
            assert_eq!(value["side_effects"]["message_sent"], false);
            assert_eq!(value["side_effects"]["cursor_written"], false);
            assert_eq!(
                value["agent_limit"],
                agent_limit
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null)
            );
            assert_eq!(
                value["message_limit"],
                message_limit
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null)
            );
            assert_ne!(
                value["compatibility_mode"],
                "native_control_ui_route_parity_shell"
            );
        }
    }

    #[test]
    fn approvals_policy_and_config_routes_are_native_redacted_views() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        for (path, mode) in [
            ("/api/approvals", "native_approvals_redacted"),
            ("/api/policy", "native_policy_snapshot"),
            ("/api/config", "native_config_surface_redacted"),
            ("/api/optional-configs", "native_optional_configs_redacted"),
        ] {
            let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
            assert_eq!(status, "200 OK", "{path}");
            assert_eq!(content_type, "application/json; charset=utf-8");
            let value: serde_json::Value =
                serde_json::from_str(&body).expect("redacted config route json");
            assert_eq!(value["runtime"], "hepta-codex");
            assert_eq!(value["native_route"], true);
            assert_eq!(value["compatibility_mode"], mode);
            assert_eq!(value["side_effect_free"], true);
            assert_eq!(value["external_side_effects"], false);
            assert_eq!(value["gateway_mutation_performed"], false);
            assert_eq!(value["telegram_read_performed"], false);
            assert_eq!(value["model_invoked"], false);
            assert_eq!(value["message_sent"], false);
            assert_eq!(value["cursor_written"], false);
            assert_ne!(
                value["compatibility_mode"],
                "native_control_ui_route_parity_shell"
            );
        }
    }

    #[test]
    fn approvals_report_keeps_mutating_routes_guarded_without_payloads() {
        let report = native_approvals_report();
        let body = serde_json::to_string(&report).expect("serialize approvals report");

        assert_eq!(report.status, "ready");
        assert_eq!(report.native_route, true);
        assert_eq!(report.pending_approval_count, 0);
        assert_eq!(report.approval_route_count, report.guarded_route_count);
        assert_eq!(report.raw_command_payload_exposed, false);
        assert_eq!(report.raw_approval_payload_exposed, false);
        assert!(report.approval_routes.iter().any(|route| {
            route.pattern == "/api/approvals/exec/apply"
                && route.guarded
                && route.confirmation_required_for_real_mutation
        }));
        assert!(!body.contains("secret-approval-payload"));
    }

    #[test]
    fn optional_configs_report_exposes_metadata_not_contents() {
        let report = native_optional_configs_report();
        let body = serde_json::to_string(&report).expect("serialize optional configs report");

        assert_eq!(report.native_route, true);
        assert_eq!(
            report.compatibility_mode,
            "native_optional_configs_redacted"
        );
        assert_eq!(report.config_content_exposed, false);
        assert_eq!(report.raw_config_value_exposed, false);
        assert!(
            report
                .configs
                .iter()
                .any(|config| { config.label == "agents" && config.content_exposed == false })
        );
        assert!(!body.contains("Be genuinely helpful"));
        assert!(!body.contains("What to call them"));
    }

    #[test]
    fn control_ui_audit_report_keeps_routes_guarded_without_dispatch() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let telegram_plugin = native_telegram::telegram_plugin_status(true, 1500);
        let report = native_control_ui_audit_report(
            NativeControlUiAuditSurface::UiContractAudit,
            &options,
            &telegram_plugin,
        );

        assert_eq!(report.status, "ready");
        assert_eq!(report.native_route, true);
        assert_eq!(report.compatibility_mode, "native_ui_contract_audit");
        assert_eq!(report.route_count, CONTROL_UI_ROUTE_SPECS.len());
        assert_eq!(
            report.get_route_count + report.post_route_count,
            CONTROL_UI_ROUTE_SPECS.len()
        );
        assert_eq!(report.post_route_count, report.guarded_post_route_count);
        assert_eq!(report.action_dispatched, false);
        assert_eq!(report.external_agent_spawned, false);
        assert_eq!(report.external_agent_benchmark_executed, false);
        assert_eq!(report.redaction.raw_action_payload_exposed, false);
        assert_eq!(report.redaction.raw_agent_payload_exposed, false);
        assert_eq!(report.side_effects.gateway_mutation_performed, false);
        assert_eq!(report.side_effects.telegram_read_performed, false);
        assert_eq!(report.side_effects.model_invoked, false);
        assert_eq!(report.side_effects.message_sent, false);
        assert_eq!(report.side_effects.cursor_written, false);
    }

    #[test]
    fn control_ui_shell_routes_return_native_plans_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        for (path, mode, surface, dry_run_only, read_only, plan_target) in [
            (
                "/api/control-ui",
                "native_control_ui_shell_snapshot",
                "control_ui",
                false,
                true,
                None,
            ),
            (
                "/api/ui-contract-audit",
                "native_ui_contract_audit",
                "ui_contract_audit",
                false,
                true,
                None,
            ),
            (
                "/api/gateway-dispatch",
                "native_gateway_dispatch_dry_run",
                "gateway_dispatch",
                true,
                false,
                Some("gateway-dispatch"),
            ),
            (
                "/api/ui-action-plan/gateway-dispatch",
                "native_ui_action_plan_gateway_dispatch",
                "ui_action_plan_gateway_dispatch",
                true,
                false,
                Some("gateway-dispatch"),
            ),
            (
                "/api/external-agent-benchmark",
                "native_external_agent_benchmark_redacted",
                "external_agent_benchmark",
                true,
                false,
                Some("external-agent-benchmark"),
            ),
        ] {
            let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
            assert_eq!(status, "200 OK", "{path}");
            assert_eq!(content_type, "application/json; charset=utf-8");
            let value: serde_json::Value =
                serde_json::from_str(&body).expect("control ui audit route json");

            assert_eq!(value["runtime"], "hepta-codex");
            assert_eq!(value["native_route"], true);
            assert_eq!(value["compatibility_mode"], mode);
            assert_eq!(value["control_surface"], surface);
            assert_eq!(value["side_effect_free"], true);
            assert_eq!(value["dry_run_only"], dry_run_only);
            assert_eq!(value["read_only"], read_only);
            assert_eq!(value["confirmation_required_for_real_mutation"], false);
            assert_eq!(value["action_dispatched"], false);
            assert_eq!(value["external_agent_spawned"], false);
            assert_eq!(value["external_agent_benchmark_executed"], false);
            assert_eq!(value["redaction"]["raw_transcript_exposed"], false);
            assert_eq!(value["redaction"]["transcript_text_exposed"], false);
            assert_eq!(value["redaction"]["raw_token_exposed"], false);
            assert_eq!(value["redaction"]["raw_action_payload_exposed"], false);
            assert_eq!(value["redaction"]["raw_agent_payload_exposed"], false);
            assert_eq!(value["side_effects"]["external_side_effects"], false);
            assert_eq!(value["side_effects"]["gateway_mutation_performed"], false);
            assert_eq!(value["side_effects"]["telegram_read_performed"], false);
            assert_eq!(value["side_effects"]["model_invoked"], false);
            assert_eq!(value["side_effects"]["message_sent"], false);
            assert_eq!(value["side_effects"]["cursor_written"], false);
            assert_eq!(
                value["plan_target"],
                plan_target
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null)
            );
            assert_ne!(
                value["compatibility_mode"],
                "native_control_ui_route_parity_shell"
            );
        }
    }

    #[test]
    fn post_plan_report_redacts_route_parameters_and_never_reads_body() {
        let spec = &native_post_plan_route_specs()[0];
        let report = native_post_plan_report(spec, Some("secret-action-payload"), None);
        let body = serde_json::to_string(&report).expect("serialize post plan report");

        assert_eq!(report.status, "dry_run_ready");
        assert_eq!(report.native_route, true);
        assert_eq!(report.compatibility_mode, "native_action_post_dry_run");
        assert_eq!(report.parameter_present, true);
        assert_eq!(report.parameter_redacted, true);
        assert_eq!(report.parameter_length, Some("secret-action-payload".len()));
        assert_eq!(report.request_body_read, false);
        assert_eq!(report.body_schema_ready, true);
        assert_eq!(report.confirmation_contract_ready, true);
        assert_eq!(report.rollback_contract_ready, true);
        assert_eq!(report.idempotency_evidence_ready, true);
        assert_eq!(report.audit_event_contract_ready, true);
        assert_eq!(report.execution_admission_ready, true);
        assert_eq!(report.body_schema.schema_id, "hepta.post.ui_action.v1");
        assert_eq!(report.body_schema.body_read_during_plan, false);
        assert_eq!(report.body_schema.raw_body_exposed, false);
        assert_eq!(report.body_admission_ready, true);
        assert_eq!(report.body_admission.admission_status, "not_required");
        assert_eq!(report.body_admission.body_received, false);
        assert_eq!(report.body_admission.request_body_read, false);
        assert_eq!(report.body_admission.raw_body_exposed, false);
        assert_eq!(report.body_admission.raw_field_values_exposed, false);
        assert_eq!(
            report
                .body_schema
                .optional_fields
                .contains(&"action_payload"),
            true
        );
        assert_eq!(
            report
                .confirmation_contract
                .current_plan_requires_confirmation,
            false
        );
        assert_eq!(
            report.rollback_contract.current_plan_rollback_strategy,
            "noop_no_state_written"
        );
        assert_eq!(report.rollback_contract.state_written_by_plan, false);
        assert_eq!(report.idempotency_evidence.required, false);
        assert_eq!(report.idempotency_evidence.key_present, false);
        assert_eq!(
            report.idempotency_evidence.current_plan_store_written,
            false
        );
        assert_eq!(report.idempotency_evidence.raw_key_exposed, false);
        assert_eq!(report.audit_event_contract.required, false);
        assert_eq!(
            report.audit_event_contract.schema_id,
            "hepta.post.execution_audit.v1"
        );
        assert_eq!(report.audit_event_contract.event_kind, "ui_action");
        assert_eq!(report.audit_event_contract.ready_for_real_handler, true);
        assert_eq!(
            report.audit_event_contract.current_plan_emits_audit_event,
            false
        );
        assert_eq!(
            report
                .audit_event_contract
                .current_plan_persists_audit_event,
            false
        );
        assert_eq!(report.execution_admission.admission_status, "blocked");
        assert_eq!(
            report
                .execution_admission
                .current_plan_executes_real_handler,
            false
        );
        assert_eq!(
            report.execution_admission.real_handler_currently_enabled,
            false
        );
        assert_eq!(report.execution_admission.real_handler_implemented, false);
        assert_eq!(
            report.execution_admission.allowlisted_for_real_handler,
            false
        );
        assert_eq!(report.execution_admission.enablement_gate_enabled, false);
        assert_eq!(report.execution_admission.requires_dry_run_first, true);
        assert_eq!(report.execution_admission.blocked_reason, "plan_only_route");
        assert_eq!(report.raw_request_body_exposed, false);
        assert_eq!(report.raw_parameter_exposed, false);
        assert_eq!(report.action_dispatched, false);
        assert_eq!(report.gateway_mutation_performed, false);
        assert_eq!(report.message_sent, false);
        assert!(!body.contains("secret-action-payload"));
    }

    #[test]
    fn post_routes_return_native_plans_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        for (path, mode, plan_kind, confirm_required, parameter_present) in [
            (
                "/api/actions/secret-action",
                "native_action_post_dry_run",
                "ui_action",
                false,
                true,
            ),
            (
                "/api/commands/secret-command",
                "native_readonly_command_plan",
                "readonly_command",
                false,
                true,
            ),
            (
                "/api/approvals/exec/apply",
                "native_approvals_exec_apply_dry_run",
                "approval_apply",
                true,
                false,
            ),
            (
                "/api/tasks/plan",
                "native_task_plan_dry_run",
                "task_plan",
                false,
                false,
            ),
            (
                "/api/tasks/publish",
                "native_task_publish_confirm_required",
                "task_publish",
                true,
                false,
            ),
            (
                "/api/chat/register",
                "native_chat_register_dry_run",
                "chat_register",
                false,
                false,
            ),
            (
                "/api/chat/archive",
                "native_chat_archive_dry_run",
                "chat_archive",
                false,
                false,
            ),
            (
                "/api/chat/unarchive",
                "native_chat_unarchive_dry_run",
                "chat_unarchive",
                false,
                false,
            ),
            (
                "/api/chat/delete",
                "native_chat_delete_dry_run",
                "chat_delete",
                false,
                false,
            ),
            (
                "/api/chat/plan",
                "native_chat_plan_dry_run",
                "chat_plan",
                false,
                false,
            ),
            (
                "/api/chat",
                "native_chat_send_confirm_required",
                "chat_send",
                true,
                false,
            ),
        ] {
            let (status, content_type, body) = route_native_gateway_request("POST", path, &options);
            assert_eq!(status, "200 OK", "{path}");
            assert_eq!(content_type, "application/json; charset=utf-8");
            assert!(!body.contains("secret-action"));
            assert!(!body.contains("secret-command"));
            let value: serde_json::Value = serde_json::from_str(&body).expect("post plan json");

            assert_eq!(value["runtime"], "hepta-codex");
            assert_eq!(value["method"], "POST");
            assert_eq!(value["native_route"], true);
            assert_eq!(value["compatibility_mode"], mode);
            assert_eq!(value["plan_kind"], plan_kind);
            assert_eq!(value["side_effect_free"], true);
            assert_eq!(
                value["confirmation_required_for_real_mutation"],
                confirm_required
            );
            assert_eq!(value["parameter_present"], parameter_present);
            assert_eq!(value["parameter_redacted"], parameter_present);
            assert_eq!(value["request_body_read"], false);
            assert_eq!(value["request_body_redacted"], true);
            assert_eq!(value["body_schema_ready"], true);
            assert_eq!(value["body_admission_ready"], true);
            assert_eq!(value["confirmation_contract_ready"], true);
            assert_eq!(value["rollback_contract_ready"], true);
            assert_eq!(value["idempotency_evidence_ready"], true);
            assert_eq!(value["audit_event_contract_ready"], true);
            assert_eq!(value["execution_admission_ready"], true);
            assert_eq!(value["body_schema"]["content_type"], "application/json");
            assert_eq!(value["body_schema"]["body_read_during_plan"], false);
            assert_eq!(value["body_schema"]["raw_body_exposed"], false);
            assert_eq!(value["body_schema"]["raw_field_values_exposed"], false);
            assert_eq!(value["body_admission"]["request_body_read"], false);
            assert_eq!(value["body_admission"]["request_body_redacted"], true);
            assert_eq!(value["body_admission"]["raw_body_exposed"], false);
            assert_eq!(value["body_admission"]["raw_field_values_exposed"], false);
            assert_eq!(
                value["confirmation_contract"]["current_plan_requires_confirmation"],
                false
            );
            assert_eq!(
                value["confirmation_contract"]["real_mutation_requires_confirmation"],
                confirm_required
            );
            assert_eq!(
                value["confirmation_contract"]["operator_approval_required"],
                confirm_required
            );
            assert_eq!(
                value["confirmation_contract"]["raw_confirmation_payload_exposed"],
                false
            );
            assert_eq!(value["rollback_contract"]["current_plan_noop"], true);
            assert_eq!(value["rollback_contract"]["state_written_by_plan"], false);
            assert_eq!(
                value["rollback_contract"]["real_handler_requires_rollback_contract"],
                true
            );
            assert_eq!(
                value["rollback_contract"]["destructive_without_rollback"],
                false
            );
            assert_eq!(value["idempotency_evidence"]["required"], confirm_required);
            assert_eq!(value["idempotency_evidence"]["key_present"], false);
            assert_eq!(value["idempotency_evidence"]["key_redacted"], false);
            assert_eq!(
                value["idempotency_evidence"]["current_plan_lookup_performed"],
                false
            );
            assert_eq!(
                value["idempotency_evidence"]["current_plan_store_written"],
                false
            );
            assert_eq!(value["idempotency_evidence"]["raw_key_exposed"], false);
            assert_eq!(value["audit_event_contract"]["required"], confirm_required);
            assert_eq!(
                value["audit_event_contract"]["schema_id"],
                "hepta.post.execution_audit.v1"
            );
            assert_eq!(value["audit_event_contract"]["event_kind"], plan_kind);
            assert_eq!(
                value["audit_event_contract"]["current_plan_emits_audit_event"],
                false
            );
            assert_eq!(
                value["audit_event_contract"]["current_plan_persists_audit_event"],
                false
            );
            assert_eq!(
                value["audit_event_contract"]["raw_idempotency_key_exposed"],
                false
            );
            assert_eq!(value["execution_admission"]["admission_status"], "blocked");
            assert_eq!(
                value["execution_admission"]["current_plan_executes_real_handler"],
                false
            );
            assert_eq!(
                value["execution_admission"]["real_handler_currently_enabled"],
                false
            );
            assert_eq!(
                value["execution_admission"]["real_handler_implemented"],
                native_post_plan_kind_has_real_handler(plan_kind)
            );
            assert_eq!(
                value["execution_admission"]["allowlisted_for_real_handler"],
                confirm_required
            );
            assert_eq!(
                value["execution_admission"]["enablement_gate_env"],
                "HEPTA_NATIVE_POST_REAL_HANDLERS"
            );
            assert_eq!(
                value["execution_admission"]["enablement_gate_enabled"],
                false
            );
            assert_eq!(
                value["execution_admission"]["operator_approval_env"],
                "HEPTA_NATIVE_POST_REAL_HANDLER_APPROVED"
            );
            assert_eq!(
                value["execution_admission"]["operator_approval_enabled"],
                false
            );
            assert_eq!(
                value["execution_admission"]["request_body_admission_status"],
                value["body_admission"]["admission_status"]
            );
            assert_eq!(
                value["execution_admission"]["requires_body_schema"],
                confirm_required
            );
            assert_eq!(
                value["execution_admission"]["requires_confirmation_contract"],
                confirm_required
            );
            assert_eq!(
                value["execution_admission"]["requires_rollback_contract"],
                confirm_required
            );
            assert_eq!(
                value["execution_admission"]["requires_idempotency_key"],
                confirm_required
            );
            assert_eq!(
                value["execution_admission"]["idempotency_evidence_ready"],
                !confirm_required
            );
            assert_eq!(
                value["execution_admission"]["requires_audit_event"],
                confirm_required
            );
            assert_eq!(
                value["execution_admission"]["audit_event_contract_ready"],
                !confirm_required
            );
            assert_eq!(
                value["execution_admission"]["requires_rate_limit"],
                confirm_required
            );
            assert_eq!(value["execution_admission"]["requires_dry_run_first"], true);
            assert_eq!(
                value["execution_admission"]["external_side_effects_possible"],
                confirm_required
            );
            let expected_blocked_reason = if confirm_required {
                "body_admission_not_ready"
            } else {
                "plan_only_route"
            };
            assert_eq!(
                value["execution_admission"]["blocked_reason"],
                expected_blocked_reason
            );
            assert_eq!(value["real_handler_harness_ready"], true);
            let expected_harness_status = if !confirm_required {
                "plan_only_route"
            } else if native_post_plan_kind_has_real_handler(plan_kind) {
                "blocked"
            } else {
                "not_implemented"
            };
            assert_eq!(
                value["real_handler_harness"]["status"],
                expected_harness_status
            );
            assert_eq!(
                value["real_handler_harness"]["handler_implemented"],
                native_post_plan_kind_has_real_handler(plan_kind)
            );
            assert_eq!(value["real_handler_harness"]["dual_gate_satisfied"], false);
            assert_eq!(
                value["real_handler_harness"]["store_write_attempted"],
                false
            );
            assert_eq!(
                value["real_handler_harness"]["store_write_succeeded"],
                false
            );
            assert_eq!(value["real_handler_harness"]["task_published"], false);
            assert_eq!(
                value["real_handler_harness"]["external_side_effects"],
                false
            );
            assert_eq!(
                value["real_handler_harness"]["raw_idempotency_key_exposed"],
                false
            );
            assert_eq!(value["action_dispatched"], false);
            assert_eq!(value["command_executed"], false);
            assert_eq!(value["approval_applied"], false);
            assert_eq!(value["task_published"], false);
            assert_eq!(value["chat_mutated"], false);
            assert_eq!(value["raw_request_body_exposed"], false);
            assert_eq!(value["raw_parameter_exposed"], false);
            assert_eq!(value["external_side_effects"], false);
            assert_eq!(value["gateway_mutation_performed"], false);
            assert_eq!(value["telegram_read_performed"], false);
            assert_eq!(value["model_invoked"], false);
            assert_eq!(value["message_sent"], false);
            assert_eq!(value["cursor_written"], false);
            assert_ne!(
                value["compatibility_mode"],
                "native_control_ui_route_parity_shell"
            );
        }
    }

    #[test]
    fn post_route_body_admission_reads_and_redacts_confirm_payload() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let body = r#"{"task":"secret task text","confirm":true,"dry_run":true,"idempotency_key":"secret-idem"}"#;

        let (status, content_type, response_body) = route_native_gateway_request_with_body(
            "POST",
            "/api/tasks/publish",
            &options,
            Some(body),
        );

        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");
        assert!(!response_body.contains("secret task text"));
        assert!(!response_body.contains("secret-idem"));
        let value: serde_json::Value =
            serde_json::from_str(&response_body).expect("post body admission json");

        assert_eq!(value["plan_kind"], "task_publish");
        assert_eq!(value["request_body_read"], true);
        assert_eq!(value["request_body_redacted"], true);
        assert_eq!(value["body_schema"]["body_read_during_plan"], true);
        assert_eq!(value["body_admission_ready"], true);
        assert_eq!(
            value["body_admission"]["admission_status"],
            "ready_for_real_handler"
        );
        assert_eq!(value["body_admission"]["body_received"], true);
        assert_eq!(value["body_admission"]["request_body_read"], true);
        assert_eq!(value["body_admission"]["json_parse_attempted"], true);
        assert_eq!(value["body_admission"]["json_parse_ok"], true);
        assert_eq!(value["body_admission"]["json_object_present"], true);
        assert_eq!(value["body_admission"]["required_fields_present"], true);
        assert_eq!(
            value["body_admission"]["missing_required_fields"],
            serde_json::json!([])
        );
        assert_eq!(value["body_admission"]["confirm_field_truthy"], true);
        assert_eq!(value["body_admission"]["dry_run_first_satisfied"], true);
        assert_eq!(value["body_admission"]["idempotency_key_present"], true);
        assert_eq!(
            value["body_admission"]["ready_for_real_handler_input"],
            true
        );
        assert_eq!(value["body_admission"]["raw_body_exposed"], false);
        assert_eq!(value["body_admission"]["raw_field_values_exposed"], false);
        assert_eq!(value["idempotency_evidence"]["required"], true);
        assert_eq!(value["idempotency_evidence"]["key_present"], true);
        assert_eq!(value["idempotency_evidence"]["key_redacted"], true);
        assert_eq!(value["idempotency_evidence"]["key_shape_valid"], true);
        assert_eq!(
            value["idempotency_evidence"]["duplicate_suppression_required"],
            true
        );
        assert_eq!(
            value["idempotency_evidence"]["current_plan_store_written"],
            false
        );
        assert_eq!(value["idempotency_evidence"]["raw_key_exposed"], false);
        assert_eq!(value["audit_event_contract"]["required"], true);
        assert_eq!(
            value["audit_event_contract"]["body_schema_id"],
            "hepta.post.task_publish.v1"
        );
        assert_eq!(
            value["audit_event_contract"]["body_admission_status_recorded"],
            true
        );
        assert_eq!(
            value["audit_event_contract"]["idempotency_evidence_recorded"],
            true
        );
        assert_eq!(
            value["audit_event_contract"]["ready_for_real_handler"],
            true
        );
        assert_eq!(
            value["audit_event_contract"]["current_plan_emits_audit_event"],
            false
        );
        assert_eq!(
            value["audit_event_contract"]["raw_idempotency_key_exposed"],
            false
        );
        assert_eq!(
            value["execution_admission"]["request_body_admission_status"],
            "ready_for_real_handler"
        );
        assert_eq!(
            value["execution_admission"]["request_body_ready_for_real_handler"],
            true
        );
        assert_eq!(
            value["execution_admission"]["idempotency_evidence_ready"],
            true
        );
        assert_eq!(
            value["execution_admission"]["audit_event_contract_ready"],
            true
        );
        assert_eq!(
            value["execution_admission"]["real_handler_implemented"],
            true
        );
        assert_eq!(
            value["execution_admission"]["current_plan_executes_real_handler"],
            false
        );
        assert_eq!(
            value["execution_admission"]["operator_approval_enabled"],
            false
        );
        assert_eq!(
            value["execution_admission"]["blocked_reason"],
            "real_handler_gate_disabled"
        );
        assert_eq!(value["real_handler_harness_ready"], true);
        assert_eq!(value["real_handler_harness"]["status"], "blocked");
        assert_eq!(value["real_handler_harness"]["handler_implemented"], true);
        assert_eq!(
            value["real_handler_harness"]["store_write_attempted"],
            false
        );
        assert_eq!(value["real_handler_harness"]["task_published"], false);
        assert_eq!(
            value["real_handler_harness"]["raw_idempotency_key_exposed"],
            false
        );
        assert_eq!(value["task_published"], false);
        assert_eq!(value["external_side_effects"], false);
        assert_eq!(value["gateway_mutation_performed"], false);
        assert_eq!(value["message_sent"], false);
    }

    #[test]
    fn native_post_execution_readiness_endpoint_is_side_effect_free() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", NATIVE_POST_EXECUTION_READINESS_ENDPOINT, &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value =
            serde_json::from_str(&body).expect("post execution readiness json");

        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["status"], "ready");
        assert_eq!(value["native_route"], true);
        assert_eq!(
            value["compatibility_mode"],
            "native_post_execution_readiness"
        );
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(
            value["post_route_count"],
            serde_json::json!(native_post_plan_route_specs().len())
        );
        assert_eq!(value["real_handler_candidate_count"], 3);
        assert_eq!(
            value["evidence_contract_route_count"],
            value["post_route_count"]
        );
        assert_eq!(value["all_evidence_contracts_ready"], true);
        assert_eq!(value["real_handler_implemented_count"], 3);
        assert_eq!(value["real_handler_ready_count"], 3);
        assert_eq!(value["all_real_handlers_blocked"], true);
        assert_eq!(value["raw_request_body_exposed"], false);
        assert_eq!(value["raw_idempotency_key_exposed"], false);
        assert_eq!(value["raw_audit_payload_exposed"], false);
        assert_eq!(value["action_dispatched"], false);
        assert_eq!(value["task_published"], false);
        assert_eq!(value["chat_mutated"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
        assert!(
            value["routes"]
                .as_array()
                .expect("routes array")
                .iter()
                .any(|route| route["pattern"] == "/api/tasks/publish"
                    && route["allowlisted_for_real_handler"] == true
                    && route["real_handler_implemented"] == true
                    && route["blocked_reason"] == "real_handler_gate_disabled")
        );
    }

    #[test]
    fn native_post_execution_stores_endpoint_is_read_only() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", NATIVE_POST_EXECUTION_STORES_ENDPOINT, &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value = serde_json::from_str(&body).expect("post stores json");

        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["status"], "ready");
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], "native_post_execution_stores");
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["store_root_env"], NATIVE_POST_EXECUTION_STORE_DIR_ENV);
        assert_eq!(value["store_file_count"], 4);
        assert_eq!(
            value["max_store_bytes_env"],
            NATIVE_POST_STORE_MAX_BYTES_ENV
        );
        assert_eq!(
            value["max_store_lines_env"],
            NATIVE_POST_STORE_MAX_LINES_ENV
        );
        assert_eq!(value["total_bytes"], 0);
        assert_eq!(value["store_jsonl_valid"], true);
        assert_eq!(value["store_capacity_ok"], true);
        assert_eq!(value["total_line_count"], 0);
        assert_eq!(value["valid_json_line_count"], 0);
        assert_eq!(value["invalid_json_line_count"], 0);
        assert_eq!(value["persistence_implementation_ready"], true);
        assert_eq!(value["idempotency_store_ready"], true);
        assert_eq!(value["audit_store_ready"], true);
        assert_eq!(value["rollback_store_ready"], true);
        assert_eq!(value["rate_limit_store_ready"], true);
        assert_eq!(value["status_probe_creates_directory"], false);
        assert_eq!(value["status_probe_writes_files"], false);
        assert_eq!(value["current_plan_executes_real_handler"], false);
        assert_eq!(value["raw_request_body_exposed"], false);
        assert_eq!(value["raw_idempotency_key_exposed"], false);
        assert_eq!(value["raw_audit_payload_exposed"], false);
        assert_eq!(value["task_published"], false);
        assert_eq!(value["chat_mutated"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
        assert!(
            value["stores"]
                .as_array()
                .expect("stores array")
                .iter()
                .any(|store| store["filename"] == "idempotency.jsonl"
                    && store["append_only"] == true
                    && store["jsonl_readable"] == true
                    && store["jsonl_valid"] == true
                    && store["line_count"] == 0
                    && store["raw_idempotency_key_exposed"] == false)
        );
    }

    #[test]
    fn native_post_activation_plan_reports_dual_gate_and_rollback_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", NATIVE_POST_ACTIVATION_PLAN_ENDPOINT, &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value = serde_json::from_str(&body).expect("activation plan json");

        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["status"], "ready");
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], "native_post_activation_plan");
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["activation_preflight_ready"], true);
        assert_eq!(value["activation_currently_enabled"], false);
        assert_eq!(
            value["activation_blocked_reason"],
            "real_handler_gate_disabled"
        );
        assert_eq!(value["handler_candidate_count"], 3);
        assert_eq!(value["handler_implemented_count"], 3);
        assert_eq!(value["all_handlers_implemented"], true);
        assert_eq!(
            value["handler_scope_env"],
            NATIVE_POST_REAL_HANDLER_SCOPE_ENV
        );
        assert_eq!(value["handler_scope"], serde_json::Value::Null);
        assert_eq!(value["handler_scope_configured"], false);
        assert_eq!(value["single_handler_scope_ready"], false);
        assert_eq!(value["selected_handler_count"], 0);
        assert_eq!(
            value["selected_handler_kinds"]
                .as_array()
                .expect("selected handler kinds")
                .len(),
            0
        );
        assert_eq!(value["execution_evidence_ready"], true);
        assert_eq!(value["store_contracts_ready"], true);
        assert_eq!(value["store_jsonl_valid"], true);
        assert_eq!(value["store_capacity_ok"], true);
        assert_eq!(value["rollback_ready"], true);
        assert_eq!(value["rollback_anchor_required"], true);
        assert_eq!(value["rollback_store_file"], "rollback.jsonl");
        assert_eq!(value["rollback_schema_id"], "hepta.post.rollback_anchor.v1");
        assert_eq!(value["dry_run_only"], true);
        assert_eq!(value["real_mutation_performed"], false);
        assert_eq!(value["store_write_attempted"], false);
        assert_eq!(value["approval_applied"], false);
        assert_eq!(value["task_published"], false);
        assert_eq!(value["chat_mutated"], false);
        assert_eq!(value["external_side_effects"], false);
        assert_eq!(value["telegram_read_performed"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
        assert_eq!(value["raw_idempotency_key_exposed"], false);
        assert_eq!(value["raw_audit_payload_exposed"], false);
        let gates = value["required_gates"].as_array().expect("gates array");
        assert_eq!(gates.len(), 3);
        assert!(gates.iter().any(|gate| {
            gate["env"] == NATIVE_POST_REAL_HANDLERS_ENV
                && gate["enabled"] == false
                && gate["required_for_activation"] == true
        }));
        assert!(gates.iter().any(|gate| {
            gate["env"] == NATIVE_POST_REAL_HANDLER_APPROVAL_ENV
                && gate["enabled"] == false
                && gate["required_for_activation"] == true
        }));
        assert!(gates.iter().any(|gate| {
            gate["env"] == NATIVE_POST_REAL_HANDLER_SCOPE_ENV
                && gate["enabled"] == false
                && gate["required_for_activation"] == true
        }));
        assert!(
            value["rollback_actions"]
                .as_array()
                .expect("rollback actions")
                .iter()
                .any(|action| action
                    .as_str()
                    .expect("rollback action")
                    .contains("launchctl kickstart"))
        );
    }

    #[test]
    fn native_post_rollout_evidence_route_reports_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT, &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value = serde_json::from_str(&body).expect("rollout evidence json");

        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["status"], "ready");
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], "native_post_rollout_evidence");
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["endpoint"], NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT);
        assert_eq!(value["store_root_env"], NATIVE_POST_EXECUTION_STORE_DIR_ENV);
        assert_eq!(
            value["activation_scope_env"],
            NATIVE_POST_REAL_HANDLER_SCOPE_ENV
        );
        assert_eq!(value["jsonl_readable"], true);
        assert_eq!(value["read_error"], serde_json::Value::Null);
        assert_eq!(value["real_mutation_performed"], false);
        assert_eq!(value["approval_applied"], false);
        assert_eq!(value["task_published"], false);
        assert_eq!(value["chat_mutated"], false);
        assert_eq!(value["telegram_read_performed"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
        assert_eq!(value["raw_request_body_exposed"], false);
        assert_eq!(value["raw_idempotency_key_exposed"], false);
        assert_eq!(value["raw_audit_payload_exposed"], false);
    }

    #[test]
    fn native_post_gray_release_evidence_route_reports_staged_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value =
            serde_json::from_str(&body).expect("gray release evidence json");

        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["status"], "staged");
        assert_eq!(value["native_route"], true);
        assert_eq!(
            value["compatibility_mode"],
            "native_post_gray_release_evidence"
        );
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(
            value["activation_plan_endpoint"],
            NATIVE_POST_ACTIVATION_PLAN_ENDPOINT
        );
        assert_eq!(
            value["rollout_evidence_endpoint"],
            NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT
        );
        assert_eq!(value["handler_scope"], serde_json::Value::Null);
        assert_eq!(value["selected_handler_count"], 0);
        assert_eq!(value["single_handler_scope_ready"], false);
        assert_eq!(value["activation_preflight_ready"], true);
        assert_eq!(value["activation_currently_enabled"], false);
        assert_eq!(value["gray_release_ready"], false);
        assert_eq!(value["gray_release_phase"], "handler_scope_not_single");
        assert_eq!(
            value["selected_handler_evidence"]["dry_run_record_present"],
            false
        );
        assert_eq!(
            value["selected_handler_evidence"]["rollback_anchor_present"],
            false
        );
        assert_eq!(value["store_write_attempted"], false);
        assert_eq!(value["task_published"], false);
        assert_eq!(value["chat_mutated"], false);
        assert_eq!(value["telegram_read_performed"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
        assert_eq!(value["raw_request_body_exposed"], false);
        assert_eq!(value["raw_idempotency_key_exposed"], false);
        assert_eq!(value["raw_audit_payload_exposed"], false);
    }

    #[test]
    fn native_post_execution_store_status_counts_jsonl_health() {
        let temp = tempfile::tempdir().expect("tempdir");
        let store_file = temp.path().join("idempotency.jsonl");
        std::fs::write(
            &store_file,
            concat!(
                r#"{"schema_id":"hepta.post.execution_store_record.v1","plan_kind":"task_publish"}"#,
                "\n",
                "not-json",
                "\n",
            ),
        )
        .expect("write store");

        let report = hepta_gateway::native_post_execution_stores_report(temp.path(), 1024, 10);
        let status = report
            .stores
            .iter()
            .find(|store| store.filename == "idempotency.jsonl")
            .expect("idempotency store status");

        assert_eq!(status.exists, true);
        assert_eq!(status.bytes_within_limit, true);
        assert_eq!(status.jsonl_readable, true);
        assert_eq!(status.jsonl_valid, false);
        assert_eq!(status.line_count, 2);
        assert_eq!(status.line_count_within_limit, true);
        assert_eq!(status.valid_json_line_count, 1);
        assert_eq!(status.invalid_json_line_count, 1);
        assert_eq!(status.raw_idempotency_key_exposed, false);
    }

    #[test]
    fn native_post_execution_store_status_blocks_oversized_jsonl() {
        let temp = tempfile::tempdir().expect("tempdir");
        let store_file = temp.path().join("idempotency.jsonl");
        std::fs::write(
            &store_file,
            concat!(
                r#"{"schema_id":"hepta.post.execution_store_record.v1","plan_kind":"task_publish"}"#,
                "\n",
                r#"{"schema_id":"hepta.post.execution_store_record.v1","plan_kind":"chat_send"}"#,
                "\n",
            ),
        )
        .expect("write store");

        let report = hepta_gateway::native_post_execution_stores_report(temp.path(), 8, 1);
        let status = report
            .stores
            .iter()
            .find(|store| store.filename == "idempotency.jsonl")
            .expect("idempotency store status");

        assert_eq!(status.exists, true);
        assert_eq!(status.jsonl_valid, true);
        assert_eq!(status.bytes_within_limit, false);
        assert_eq!(status.line_count, 2);
        assert_eq!(status.line_count_within_limit, false);
        assert_eq!(status.invalid_json_line_count, 0);
    }

    #[test]
    fn native_post_real_handler_harness_records_redacted_dry_run_under_dual_gate() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let body = r#"{"task":"secret task text","confirm":true,"dry_run":true,"idempotency_key":"secret-idem"}"#;
        let body_schema = native_post_body_schema(spec.plan_kind, true);
        let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
        let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
        let audit_event_contract = native_post_audit_event_contract(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
        );
        let execution_admission = native_post_execution_admission_with_gates(
            spec,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            true,
            true,
        );
        let temp = tempfile::tempdir().expect("tempdir");
        let fingerprint = idempotency_evidence
            .key_fingerprint
            .as_deref()
            .expect("idempotency fingerprint");
        assert!(fingerprint.starts_with("sha256:"));
        assert!(!fingerprint.contains("secret-idem"));

        let harness = native_post_real_handler_harness(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            &execution_admission,
            temp.path(),
        );

        assert_eq!(execution_admission.admission_status, "harness_ready");
        assert_eq!(execution_admission.current_plan_executes_real_handler, true);
        assert_eq!(execution_admission.operator_approval_enabled, true);
        assert_eq!(
            execution_admission.blocked_reason,
            "real_handler_harness_dry_run_only"
        );
        assert_eq!(harness.status, "dry_run_recorded");
        assert_eq!(harness.handler_kind, "task_publish");
        assert_eq!(harness.dry_run_only, true);
        assert_eq!(harness.handler_implemented, true);
        assert_eq!(harness.dual_gate_satisfied, true);
        assert_eq!(harness.capacity_check_performed, true);
        assert_eq!(harness.store_capacity_ok, true);
        assert_eq!(harness.store_write_attempted, true);
        assert_eq!(harness.store_write_succeeded, true);
        assert_eq!(harness.task_published, false);
        assert_eq!(harness.external_side_effects, false);
        assert_eq!(harness.raw_request_body_exposed, false);
        assert_eq!(harness.raw_idempotency_key_exposed, false);
        let report = harness
            .store_write_report
            .as_ref()
            .expect("store write report");
        assert_eq!(report.status, "written");
        assert_eq!(report.written_file_count, 4);
        for file in &report.written_files {
            let content = std::fs::read_to_string(file).expect("read store file");
            assert!(content.contains("hepta.post.execution_store_record.v1"));
            assert!(content.contains("task_publish"));
            assert!(content.contains(fingerprint));
            assert!(content.contains("\"current_plan_executes_real_handler\":true"));
            assert!(!content.contains("secret task text"));
            assert!(!content.contains("secret-idem"));
        }
    }

    #[test]
    fn native_post_execution_store_capacity_blocks_projected_append_over_limits() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let body = r#"{"task":"secret capacity task","confirm":true,"dry_run":true,"idempotency_key":"secret-capacity-idem"}"#;
        let body_schema = native_post_body_schema(spec.plan_kind, true);
        let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
        let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
        let audit_event_contract = native_post_audit_event_contract(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
        );
        let record = native_post_execution_store_record(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            true,
        );
        let temp = tempfile::tempdir().expect("tempdir");
        persist_native_post_execution_store_record(temp.path(), &record).expect("seed stores");

        assert_eq!(
            native_post_execution_store_capacity_allows_append_with_limits(
                temp.path(),
                &record,
                1024 * 1024,
                2,
            )
            .expect("capacity check"),
            true
        );
        assert_eq!(
            native_post_execution_store_capacity_allows_append_with_limits(
                temp.path(),
                &record,
                1024 * 1024,
                1,
            )
            .expect("line capacity check"),
            false
        );
        assert_eq!(
            native_post_execution_store_capacity_allows_append_with_limits(
                temp.path(),
                &record,
                8,
                10,
            )
            .expect("byte capacity check"),
            false
        );
    }

    #[test]
    fn native_post_real_handler_harness_suppresses_duplicate_idempotency_key() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let body = r#"{"task":"secret duplicate task","confirm":true,"dry_run":true,"idempotency_key":"secret-duplicate-idem"}"#;
        let body_schema = native_post_body_schema(spec.plan_kind, true);
        let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
        let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
        let audit_event_contract = native_post_audit_event_contract(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
        );
        let execution_admission = native_post_execution_admission_with_gates(
            spec,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            true,
            true,
        );
        let temp = tempfile::tempdir().expect("tempdir");

        let first = native_post_real_handler_harness(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            &execution_admission,
            temp.path(),
        );
        let second = native_post_real_handler_harness(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            &execution_admission,
            temp.path(),
        );

        assert_eq!(first.status, "dry_run_recorded");
        assert_eq!(first.store_write_succeeded, true);
        assert_eq!(second.status, "duplicate_suppressed");
        assert_eq!(second.duplicate_check_performed, true);
        assert_eq!(second.duplicate_found, true);
        assert_eq!(second.duplicate_suppressed, true);
        assert_eq!(second.store_write_attempted, false);
        assert_eq!(second.store_write_succeeded, false);
        assert!(second.store_write_report.is_none());
        let idempotency_content = std::fs::read_to_string(temp.path().join("idempotency.jsonl"))
            .expect("idempotency store");
        assert_eq!(idempotency_content.lines().count(), 1);
        assert!(
            idempotency_content.contains(
                idempotency_evidence
                    .key_fingerprint
                    .as_deref()
                    .expect("fingerprint")
            )
        );
        assert!(!idempotency_content.contains("secret duplicate task"));
        assert!(!idempotency_content.contains("secret-duplicate-idem"));
    }

    #[test]
    fn native_post_real_handler_harness_rate_limits_recent_bucket() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let first_body = r#"{"task":"secret first task","confirm":true,"dry_run":true,"idempotency_key":"secret-first-idem"}"#;
        let second_body = r#"{"task":"secret second task","confirm":true,"dry_run":true,"idempotency_key":"secret-second-idem"}"#;
        let body_schema = native_post_body_schema(spec.plan_kind, true);
        let first_body_admission = native_post_body_admission(spec, &body_schema, Some(first_body));
        let first_idempotency_evidence =
            native_post_idempotency_evidence(spec, &first_body_admission);
        let first_audit_event_contract = native_post_audit_event_contract(
            spec,
            &body_schema,
            &first_body_admission,
            &first_idempotency_evidence,
        );
        let first_execution_admission = native_post_execution_admission_with_gates(
            spec,
            &first_body_admission,
            &first_idempotency_evidence,
            &first_audit_event_contract,
            true,
            true,
        );
        let second_body_admission =
            native_post_body_admission(spec, &body_schema, Some(second_body));
        let second_idempotency_evidence =
            native_post_idempotency_evidence(spec, &second_body_admission);
        let second_audit_event_contract = native_post_audit_event_contract(
            spec,
            &body_schema,
            &second_body_admission,
            &second_idempotency_evidence,
        );
        let second_execution_admission = native_post_execution_admission_with_gates(
            spec,
            &second_body_admission,
            &second_idempotency_evidence,
            &second_audit_event_contract,
            true,
            true,
        );
        let temp = tempfile::tempdir().expect("tempdir");

        let first = native_post_real_handler_harness(
            spec,
            &body_schema,
            &first_body_admission,
            &first_idempotency_evidence,
            &first_audit_event_contract,
            &first_execution_admission,
            temp.path(),
        );
        let second = native_post_real_handler_harness(
            spec,
            &body_schema,
            &second_body_admission,
            &second_idempotency_evidence,
            &second_audit_event_contract,
            &second_execution_admission,
            temp.path(),
        );

        assert_eq!(first.status, "dry_run_recorded");
        assert_eq!(first.rate_limit_check_performed, true);
        assert_eq!(first.rate_limited, false);
        assert_eq!(second.status, "rate_limited");
        assert_eq!(second.duplicate_check_performed, true);
        assert_eq!(second.duplicate_found, false);
        assert_eq!(second.rate_limit_check_performed, true);
        assert_eq!(second.rate_limited, true);
        assert_eq!(second.rate_limit_suppressed, true);
        assert_eq!(second.store_write_attempted, false);
        assert_eq!(second.store_write_succeeded, false);
        assert!(second.store_write_report.is_none());
        let rate_limit_content = std::fs::read_to_string(temp.path().join("rate-limit.jsonl"))
            .expect("rate-limit store");
        assert_eq!(rate_limit_content.lines().count(), 1);
        assert!(!rate_limit_content.contains("secret first task"));
        assert!(!rate_limit_content.contains("secret second task"));
        assert!(!rate_limit_content.contains("secret-first-idem"));
        assert!(!rate_limit_content.contains("secret-second-idem"));
    }

    #[test]
    fn native_post_real_handler_harness_covers_confirm_required_candidates() {
        let candidates = [
            (
                "approval_apply",
                r#"{"approval_id":"secret approval id","confirm":true,"dry_run":true,"idempotency_key":"secret-approval-idem"}"#,
                "secret approval id",
                "secret-approval-idem",
            ),
            (
                "task_publish",
                r#"{"task":"secret task body","confirm":true,"dry_run":true,"idempotency_key":"secret-task-idem"}"#,
                "secret task body",
                "secret-task-idem",
            ),
            (
                "chat_send",
                r#"{"chat_id":"secret chat id","message":"secret chat message","confirm":true,"dry_run":true,"idempotency_key":"secret-chat-idem"}"#,
                "secret chat message",
                "secret-chat-idem",
            ),
        ];
        let temp = tempfile::tempdir().expect("tempdir");

        for (plan_kind, body, raw_secret, raw_idempotency_key) in candidates {
            let spec = native_post_plan_route_specs()
                .iter()
                .find(|spec| spec.plan_kind == plan_kind)
                .expect("candidate spec");
            let body_schema = native_post_body_schema(spec.plan_kind, true);
            let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
            let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
            let audit_event_contract = native_post_audit_event_contract(
                spec,
                &body_schema,
                &body_admission,
                &idempotency_evidence,
            );
            let execution_admission = native_post_execution_admission_with_gates(
                spec,
                &body_admission,
                &idempotency_evidence,
                &audit_event_contract,
                true,
                true,
            );

            let harness = native_post_real_handler_harness(
                spec,
                &body_schema,
                &body_admission,
                &idempotency_evidence,
                &audit_event_contract,
                &execution_admission,
                temp.path(),
            );

            assert_eq!(body_admission.admission_status, "ready_for_real_handler");
            assert_eq!(native_post_plan_kind_has_real_handler(plan_kind), true);
            assert_eq!(execution_admission.admission_status, "harness_ready");
            assert_eq!(execution_admission.current_plan_executes_real_handler, true);
            assert_eq!(harness.status, "dry_run_recorded");
            assert_eq!(harness.handler_kind, plan_kind);
            assert_eq!(harness.handler_implemented, true);
            assert_eq!(harness.dry_run_only, true);
            assert_eq!(harness.store_write_attempted, true);
            assert_eq!(harness.store_write_succeeded, true);
            assert_eq!(harness.task_published, false);
            assert_eq!(harness.message_sent, false);
            assert_eq!(harness.external_side_effects, false);
            assert_eq!(harness.raw_request_body_exposed, false);
            assert_eq!(harness.raw_idempotency_key_exposed, false);

            let report = harness
                .store_write_report
                .as_ref()
                .expect("store write report");
            assert_eq!(report.written_file_count, 4);
            for file in &report.written_files {
                let content = std::fs::read_to_string(file).expect("read store file");
                assert!(content.contains(plan_kind));
                assert!(!content.contains(raw_secret));
                assert!(!content.contains(raw_idempotency_key));
            }
        }

        let idempotency_content = std::fs::read_to_string(temp.path().join("idempotency.jsonl"))
            .expect("idempotency store");
        assert_eq!(idempotency_content.lines().count(), candidates.len());
    }

    #[test]
    fn native_post_real_handler_harness_requires_operator_approval_gate() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let body = r#"{"task":"secret task text","confirm":true,"dry_run":true,"idempotency_key":"secret-idem"}"#;
        let body_schema = native_post_body_schema(spec.plan_kind, true);
        let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
        let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
        let audit_event_contract = native_post_audit_event_contract(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
        );
        let execution_admission = native_post_execution_admission_with_gates(
            spec,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            true,
            false,
        );
        let temp = tempfile::tempdir().expect("tempdir");

        let harness = native_post_real_handler_harness(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            &execution_admission,
            temp.path(),
        );

        assert_eq!(execution_admission.admission_status, "blocked");
        assert_eq!(
            execution_admission.current_plan_executes_real_handler,
            false
        );
        assert_eq!(execution_admission.enablement_gate_enabled, true);
        assert_eq!(execution_admission.operator_approval_enabled, false);
        assert_eq!(
            execution_admission.blocked_reason,
            "operator_approval_required"
        );
        assert_eq!(harness.status, "blocked");
        assert_eq!(harness.dual_gate_satisfied, false);
        assert_eq!(harness.store_write_attempted, false);
        assert_eq!(harness.store_write_succeeded, false);
        assert!(harness.store_write_report.is_none());
        assert_eq!(temp.path().join("idempotency.jsonl").exists(), false);
    }

    #[test]
    fn native_post_real_handler_harness_requires_matching_handler_scope() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let body = r#"{"task":"secret scoped task","confirm":true,"dry_run":true,"idempotency_key":"secret-scoped-idem"}"#;
        let body_schema = native_post_body_schema(spec.plan_kind, true);
        let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
        let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
        let audit_event_contract = native_post_audit_event_contract(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
        );
        let mismatched_admission = native_post_execution_admission_with_scope(
            spec,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            true,
            true,
            Some("chat_send"),
        );
        let matched_admission = native_post_execution_admission_with_scope(
            spec,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            true,
            true,
            Some("task_publish"),
        );
        let temp = tempfile::tempdir().expect("tempdir");

        let mismatched_harness = native_post_real_handler_harness(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            &mismatched_admission,
            temp.path(),
        );

        assert_eq!(mismatched_admission.admission_status, "blocked");
        assert_eq!(
            mismatched_admission.current_plan_executes_real_handler,
            false
        );
        assert_eq!(mismatched_admission.handler_scope_configured, true);
        assert_eq!(mismatched_admission.handler_scope_required, true);
        assert_eq!(mismatched_admission.handler_scope_matches, false);
        assert_eq!(
            mismatched_admission.blocked_reason,
            "handler_scope_not_selected"
        );
        assert_eq!(mismatched_harness.status, "blocked");
        assert_eq!(mismatched_harness.handler_scope_configured, true);
        assert_eq!(mismatched_harness.handler_scope_matches, false);
        assert_eq!(mismatched_harness.store_write_attempted, false);
        assert_eq!(temp.path().join("idempotency.jsonl").exists(), false);

        let matched_harness = native_post_real_handler_harness(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            &matched_admission,
            temp.path(),
        );

        assert_eq!(matched_admission.admission_status, "harness_ready");
        assert_eq!(matched_admission.handler_scope_matches, true);
        assert_eq!(matched_harness.status, "dry_run_recorded");
        assert_eq!(matched_harness.handler_scope_matches, true);
        assert_eq!(matched_harness.store_write_attempted, true);
        assert_eq!(matched_harness.store_write_succeeded, true);
        let content = std::fs::read_to_string(temp.path().join("idempotency.jsonl"))
            .expect("idempotency store");
        assert!(content.contains("task_publish"));
        assert!(!content.contains("secret scoped task"));
        assert!(!content.contains("secret-scoped-idem"));
    }

    #[test]
    fn native_post_execution_store_writer_persists_redacted_records() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let body = r#"{"task":"secret task text","confirm":true,"dry_run":true,"idempotency_key":"secret-idem"}"#;
        let body_schema = native_post_body_schema(spec.plan_kind, true);
        let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
        let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
        let audit_event_contract = native_post_audit_event_contract(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
        );
        let record = native_post_execution_store_record(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            true,
        );
        let temp = tempfile::tempdir().expect("tempdir");

        let report =
            persist_native_post_execution_store_record(temp.path(), &record).expect("write stores");

        assert_eq!(report.status, "written");
        assert_eq!(report.written_file_count, 4);
        assert_eq!(report.raw_request_body_exposed, false);
        assert_eq!(report.raw_idempotency_key_exposed, false);
        for file in report.written_files {
            let content = std::fs::read_to_string(&file).expect("read store file");
            assert!(content.contains("hepta.post.execution_store_record.v1"));
            assert!(content.contains("task_publish"));
            assert!(content.contains("idempotency_key_redacted"));
            assert!(!content.contains("secret task text"));
            assert!(!content.contains("secret-idem"));
        }
    }

    #[test]
    fn native_post_rollout_evidence_summarizes_redacted_rollback_anchor() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let body = r#"{"task":"secret rollout task","confirm":true,"dry_run":true,"idempotency_key":"secret-rollout-idem"}"#;
        let body_schema = native_post_body_schema(spec.plan_kind, true);
        let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
        let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
        let audit_event_contract = native_post_audit_event_contract(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
        );
        let record = native_post_execution_store_record(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            true,
        );
        let temp = tempfile::tempdir().expect("tempdir");

        let empty = hepta_gateway::native_post_rollout_evidence_report(
            temp.path(),
            DEFAULT_NATIVE_POST_STORE_MAX_BYTES,
            DEFAULT_NATIVE_POST_STORE_MAX_LINES,
            None,
        );
        assert_eq!(empty.status, "ready");
        assert_eq!(empty.record_count, 0);
        assert_eq!(empty.rollback_anchor_present, false);
        assert_eq!(empty.dry_run_record_present, false);
        assert!(empty.latest_record.is_none());

        persist_native_post_execution_store_record(temp.path(), &record).expect("write stores");
        let report = hepta_gateway::native_post_rollout_evidence_report(
            temp.path(),
            DEFAULT_NATIVE_POST_STORE_MAX_BYTES,
            DEFAULT_NATIVE_POST_STORE_MAX_LINES,
            None,
        );

        assert_eq!(report.status, "ready");
        assert_eq!(report.rollout_evidence_ready, true);
        assert_eq!(report.record_count, 1);
        assert_eq!(report.dry_run_record_count, 1);
        assert_eq!(report.rollback_anchor_count, 1);
        assert_eq!(report.rollback_anchor_present, true);
        assert_eq!(report.dry_run_record_present, true);
        assert_eq!(report.invalid_json_line_count, 0);
        assert_eq!(report.plan_kind_counts.len(), 1);
        assert_eq!(report.plan_kind_counts[0].plan_kind, "task_publish");
        assert_eq!(report.plan_kind_counts[0].count, 1);
        assert_eq!(report.raw_request_body_exposed, false);
        assert_eq!(report.raw_idempotency_key_exposed, false);
        assert_eq!(report.task_published, false);
        assert_eq!(report.external_side_effects, false);
        let latest = report.latest_record.expect("latest record");
        assert_eq!(latest.plan_kind.as_deref(), Some("task_publish"));
        assert_eq!(latest.current_plan_executes_real_handler, true);
        assert_eq!(latest.idempotency_key_redacted, true);
        assert_eq!(latest.idempotency_key_fingerprint_present, true);
        assert_eq!(latest.raw_request_body_exposed, false);
        assert_eq!(latest.raw_idempotency_key_exposed, false);
        let rollback_content =
            std::fs::read_to_string(temp.path().join("rollback.jsonl")).expect("rollback store");
        assert!(!rollback_content.contains("secret rollout task"));
        assert!(!rollback_content.contains("secret-rollout-idem"));
    }

    #[test]
    fn native_post_gray_release_evidence_requires_scoped_rollback_anchor() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let body = r#"{"task":"secret gray task","confirm":true,"dry_run":true,"idempotency_key":"secret-gray-idem"}"#;
        let body_schema = native_post_body_schema(spec.plan_kind, true);
        let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
        let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
        let audit_event_contract = native_post_audit_event_contract(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
        );
        let record = native_post_execution_store_record(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            true,
        );
        let temp = tempfile::tempdir().expect("tempdir");
        let before = hepta_gateway::native_post_gray_release_evidence_report(
            temp.path(),
            DEFAULT_NATIVE_POST_STORE_MAX_BYTES,
            DEFAULT_NATIVE_POST_STORE_MAX_LINES,
            Some("task_publish"),
            true,
            true,
        );
        assert_eq!(before.status, "staged");
        assert_eq!(before.gray_release_phase, "awaiting_scoped_dry_run_record");
        assert_eq!(before.gray_release_ready, false);
        assert_eq!(before.selected_handler_evidence_ready, false);

        persist_native_post_execution_store_record(temp.path(), &record).expect("write stores");
        let report = hepta_gateway::native_post_gray_release_evidence_report(
            temp.path(),
            DEFAULT_NATIVE_POST_STORE_MAX_BYTES,
            DEFAULT_NATIVE_POST_STORE_MAX_LINES,
            Some("task_publish"),
            true,
            true,
        );

        assert_eq!(report.status, "ready");
        assert_eq!(report.gray_release_phase, "gray_release_ready");
        assert_eq!(report.activation_currently_enabled, true);
        assert_eq!(report.single_handler_scope_ready, true);
        assert_eq!(
            report.selected_handler_kind.as_deref(),
            Some("task_publish")
        );
        assert_eq!(report.gray_release_evidence_ready, true);
        assert_eq!(report.selected_handler_evidence_ready, true);
        assert_eq!(report.gray_release_ready, true);
        assert_eq!(report.selected_handler_evidence.record_count, 1);
        assert_eq!(report.selected_handler_evidence.dry_run_record_count, 1);
        assert_eq!(report.selected_handler_evidence.rollback_anchor_count, 1);
        assert_eq!(
            report
                .selected_handler_evidence
                .latest_record
                .as_ref()
                .and_then(|record| record.plan_kind.as_deref()),
            Some("task_publish")
        );
        assert_eq!(report.raw_request_body_exposed, false);
        assert_eq!(report.raw_idempotency_key_exposed, false);
        let rollback_content =
            std::fs::read_to_string(temp.path().join("rollback.jsonl")).expect("rollback store");
        assert!(!rollback_content.contains("secret gray task"));
        assert!(!rollback_content.contains("secret-gray-idem"));
    }

    #[test]
    fn operator_console_returns_native_status_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", "/api/operator-console", &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value = serde_json::from_str(&body).expect("operator console json");

        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], "native_operator_console");
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["sessions"]["native_route"], true);
        assert_eq!(
            value["sessions"]["compatibility_mode"],
            "native_sessions_inventory"
        );
        assert_eq!(value["external_side_effects"], false);
        assert_eq!(value["gateway_mutation_performed"], false);
        assert_eq!(value["telegram_read_performed"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
        assert_eq!(value["raw_transcript_exposed"], false);
        assert_ne!(
            value["compatibility_mode"],
            "native_control_ui_route_parity_shell"
        );
    }

    #[test]
    fn operator_security_returns_native_guard_matrix_without_side_effects() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", "/api/operator-security", &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value = serde_json::from_str(&body).expect("operator security json");

        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], "native_operator_security");
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["loopback_bind_required"], true);
        assert_eq!(value["loopback_bound"], true);
        assert_eq!(value["side_effects"]["external_side_effects"], false);
        assert_eq!(value["side_effects"]["gateway_mutation_performed"], false);
        assert_eq!(value["side_effects"]["telegram_read_performed"], false);
        assert_eq!(value["side_effects"]["model_invoked"], false);
        assert_eq!(value["side_effects"]["message_sent"], false);
        assert_eq!(value["side_effects"]["cursor_written"], false);
        assert_eq!(value["redaction"]["raw_transcript_exposed"], false);
        assert_eq!(value["redaction"]["raw_token_exposed"], false);
        assert_eq!(value["redaction"]["raw_idempotency_key_exposed"], false);
        assert_eq!(value["redaction"]["raw_audit_payload_exposed"], false);
        assert_eq!(
            value["post_execution_readiness_endpoint"],
            NATIVE_POST_EXECUTION_READINESS_ENDPOINT
        );
        assert_eq!(
            value["post_execution_stores_endpoint"],
            NATIVE_POST_EXECUTION_STORES_ENDPOINT
        );
        assert_eq!(
            value["post_activation_plan_endpoint"],
            NATIVE_POST_ACTIVATION_PLAN_ENDPOINT
        );
        assert_eq!(
            value["post_gray_release_evidence_endpoint"],
            NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT
        );
        assert_eq!(value["post_execution_readiness"]["status"], "ready");
        assert_eq!(
            value["post_execution_readiness"]["all_real_handlers_blocked"],
            true
        );
        assert_eq!(value["post_execution_stores_ready"], true);
        assert_eq!(value["post_execution_stores"]["status"], "ready");
        assert_eq!(value["post_activation_plan_ready"], true);
        assert_eq!(value["post_activation_plan"]["status"], "ready");
        assert_eq!(
            value["post_activation_plan"]["activation_preflight_ready"],
            true
        );
        assert_eq!(
            value["post_activation_plan"]["activation_currently_enabled"],
            false
        );
        assert_eq!(
            value["post_activation_plan"]["activation_blocked_reason"],
            "real_handler_gate_disabled"
        );
        assert_eq!(value["post_activation_plan"]["rollback_ready"], true);
        assert_eq!(value["post_gray_release_evidence_ready"], true);
        assert_eq!(value["post_gray_release_evidence"]["status"], "staged");
        assert_eq!(
            value["post_gray_release_evidence"]["gray_release_ready"],
            false
        );
        assert_eq!(
            value["post_gray_release_evidence"]["store_write_attempted"],
            false
        );
        assert_eq!(
            value["post_execution_stores"]["status_probe_writes_files"],
            false
        );
        assert_eq!(
            value["post_execution_stores"]["raw_idempotency_key_exposed"],
            false
        );
        assert!(value["telegram_production_readiness_status"].is_object());
        assert_eq!(
            value["telegram_production_readiness_status"]["side_effect_free"],
            true
        );
        assert_eq!(
            value["telegram_production_readiness_status"]["raw_token_exposed"],
            false
        );
        assert_eq!(
            value["telegram_owner_handoff_endpoint"],
            TELEGRAM_OWNER_HANDOFF_ENDPOINT
        );
        assert_eq!(
            value["telegram_owner_handoff_status"]["side_effect_free"],
            true
        );
        assert_eq!(
            value["telegram_owner_handoff_status"]["raw_token_exposed"],
            false
        );
        assert_eq!(value["post_route_count"], value["guarded_post_route_count"]);
        assert!(
            value["dry_run_post_route_count"]
                .as_u64()
                .expect("dry-run count")
                <= value["post_route_count"].as_u64().expect("post count")
        );
        assert_ne!(
            value["compatibility_mode"],
            "native_control_ui_route_parity_shell"
        );
    }

    #[test]
    fn control_ui_route_parity_endpoint_returns_ready_report() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: false,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) =
            route_native_gateway_request("GET", CONTROL_UI_ROUTE_PARITY_ENDPOINT, &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value = serde_json::from_str(&body).expect("parity json");
        assert_eq!(value["runtime"], "hepta-codex");
        assert_eq!(value["ready"], true);
        assert_eq!(value["missing_route_count"], 0);
    }

    #[test]
    fn route_health_returns_ready_json() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: false,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request("GET", "/health", &options);
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");
        assert!(body.contains(r#""status":"ready""#));
    }
}

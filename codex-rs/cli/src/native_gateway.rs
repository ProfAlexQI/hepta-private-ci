use std::collections::BTreeMap;
use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use anyhow::Context;
use anyhow::Result;
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
const ACTIVE_GATEWAY_LABEL: &str = "ai.hepta.gateway";
const ACTIVE_GATEWAY_LEGACY_BINARY: &str = "/Users/qianqi/.local/opt/hepta/bin/hepta";
const HEPTA_CODEX_RELEASE_BINARY: &str = "/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex";
const MAX_NATIVE_SESSION_SUMMARIES: usize = 20;
const MAX_NATIVE_TRANSCRIPT_FILES: usize = 5;
const MAX_NATIVE_TRANSCRIPT_QUERY_FILES: usize = 20;
const MAX_NATIVE_TRANSCRIPT_LINES_PER_FILE: usize = 2_000;
const MAX_NATIVE_TRANSCRIPT_EVENT_PREVIEWS_PER_FILE: usize = 40;

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
        let (status, content_type, body) = route_native_gateway_request(method, path, &options);
        write_http_response(&mut stream, status, content_type, body.as_bytes())?;
    }

    Ok(())
}

fn route_native_gateway_request(
    method: &str,
    path: &str,
    options: &NativeGatewayOptions,
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
            "Telegram cursor state surface",
            "Control UI route parity report",
            "Control UI side-effect-free compatibility endpoints",
            "Gateway live activation side-effect-free plan",
        ],
        next_migration_slice: "continue active Telegram soak and inspect /api/telegram-live-soak before broadening traffic",
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

    let production_soak_ready = telegram_live_soak_status.health_ready
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
    let loopback_bound = is_loopback_bind_addr(&options.bind_addr);
    let ready = control_ui_route_parity.ready
        && gateway_replacement_readiness.ready
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
        telegram_gate_summary: native_telegram::telegram_gateway_gate_summary(),
        telegram_plugin_requested: options.with_telegram_plugin,
        telegram_plugin_status: telegram_plugin.status,
        redaction: NativeOperatorSecurityRedaction {
            raw_transcript_exposed: false,
            raw_token_exposed: false,
            raw_update_payload_exposed: false,
            raw_prompt_text_exposed: false,
            raw_response_text_exposed: false,
        },
        side_effects: NativeOperatorSecuritySideEffects {
            external_side_effects: false,
            gateway_mutation_performed: false,
            telegram_read_performed: false,
            model_invoked: false,
            message_sent: false,
            cursor_written: false,
        },
        next_migration_slice: "keep POST routes dry-run until each action has an explicit confirmation and rollback contract",
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
    telegram_gate_summary: native_telegram::NativeTelegramGatewayGateSummary,
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

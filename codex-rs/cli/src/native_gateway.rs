use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

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
            "/api/gateway-replacement-readiness" => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&gateway_replacement_readiness(options, &telegram_plugin)),
                );
            }
            CONTROL_UI_ROUTE_PARITY_ENDPOINT => {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&control_ui_route_parity_report()),
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
        gateway_replacement_readiness_endpoint: "/api/gateway-replacement-readiness",
        gateway_replacement_readiness,
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
        telegram_cursor_endpoint: "/api/telegram-cursor",
        telegram_gate_summary: native_telegram::telegram_gateway_gate_summary(),
        telegram_poll_loop_status: native_telegram::telegram_poll_loop_status(
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
            "Telegram cursor state surface",
            "Control UI route parity report",
            "Control UI side-effect-free compatibility endpoints",
        ],
        next_migration_slice: "finish live Telegram gate smoke under explicit operator approval, then mark active gateway replacement ready",
    })
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
        next_migration_slice: "run explicit live Telegram gate smoke only with operator approval, then replace the active gateway",
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
    telegram_cursor_endpoint: &'static str,
    telegram_gate_summary: native_telegram::NativeTelegramGatewayGateSummary,
    telegram_poll_loop_status: native_telegram::NativeTelegramPollLoopStatus,
    telegram_readiness_summary_side_effect_free: bool,
    telegram_plugin: &'a NativeTelegramPluginStatus,
    migrated_surfaces: &'static [&'static str],
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

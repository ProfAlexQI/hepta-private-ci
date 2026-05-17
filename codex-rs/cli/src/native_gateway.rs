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
    if method != "GET" {
        return (
            "405 Method Not Allowed",
            "text/plain; charset=utf-8",
            "method not allowed".to_string(),
        );
    }

    let telegram_plugin = native_telegram::telegram_plugin_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    match path {
        "/" | "/index.html" => (
            "200 OK",
            "text/html; charset=utf-8",
            index_html(options, &telegram_plugin),
        ),
        "/health" | "/api/health" => (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&HealthResponse {
                product: "Hepta",
                runtime: "hepta-codex",
                status: "ready",
            }),
        ),
        "/api/native-gateway" | "/api/gateway-runtime" => (
            "200 OK",
            "application/json; charset=utf-8",
            native_gateway_json(options, &telegram_plugin),
        ),
        "/api/telegram-plugin" => (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&telegram_plugin),
        ),
        "/api/telegram-receive-once" => (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&native_telegram::telegram_receive_once_status(
                options.with_telegram_plugin,
                20,
            )),
        ),
        "/api/telegram-model-turn-plan" => (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&native_telegram::telegram_model_turn_plan_status(
                options.with_telegram_plugin,
            )),
        ),
        _ => (
            "404 Not Found",
            "text/plain; charset=utf-8",
            "not found".to_string(),
        ),
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
    json_or_error(&NativeGatewayResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: "ready",
        migration_mode: "codex_fork_native",
        bind_addr: &options.bind_addr,
        launchd_entrypoint_compatible: true,
        active_gateway_replacement_ready: false,
        replacement_blocker: "Telegram Bot API polling/send, Codex model-turn bridge, and full Hepta Control UI route surface are still migrating into hepta-codex",
        telegram_plugin_requested: options.with_telegram_plugin,
        telegram_plugin_status: telegram_plugin.status,
        telegram_plugin_native_supervisor_ready: telegram_plugin.in_process_supervisor_ready,
        telegram_plugin_reply_loop_ready: telegram_plugin.in_process_reply_loop_ready,
        telegram_plugin_poll_ms: options.telegram_plugin_poll_ms,
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
        ],
        next_migration_slice: "wire planned Telegram candidates into a bounded Codex model-turn bridge, then enable gated send",
    })
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
    replacement_blocker: &'static str,
    telegram_plugin_requested: bool,
    telegram_plugin_status: &'static str,
    telegram_plugin_native_supervisor_ready: bool,
    telegram_plugin_reply_loop_ready: bool,
    telegram_plugin_poll_ms: u64,
    telegram_plugin: &'a NativeTelegramPluginStatus,
    migrated_surfaces: &'static [&'static str],
    next_migration_slice: &'static str,
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
        assert!(body.contains(r#""telegram_plugin_native_supervisor_ready":"#));
        assert!(!body.contains("pending_migration"));
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

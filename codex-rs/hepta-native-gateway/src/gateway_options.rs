use std::env;

use anyhow::Context;
use anyhow::Result;

use crate::state_root::validate_state_root_env;

pub(crate) const DEFAULT_BIND_ADDR: &str = "127.0.0.1:7373";
pub(crate) const DEFAULT_TELEGRAM_POLL_MS: u64 = 1500;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeGatewayOptions {
    pub(crate) bind_addr: String,
    pub(crate) with_telegram_plugin: bool,
    pub(crate) telegram_plugin_poll_ms: u64,
}

impl NativeGatewayOptions {
    fn from_env_and_args(args: &[String]) -> Result<Self> {
        validate_state_root_env()?;
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

pub fn parse_serve_ui_args(raw_args: &[String]) -> Result<Option<NativeGatewayOptions>> {
    match raw_args.first().map(String::as_str) {
        Some("--serve-ui") => {
            let options = NativeGatewayOptions::from_env_and_args(&raw_args[1..])?;
            Ok(Some(options))
        }
        _ => Ok(None),
    }
}

pub fn parse_serve_ui_args_from_env() -> Result<Option<NativeGatewayOptions>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    parse_serve_ui_args(&args)
}

fn env_truthy(name: &str) -> bool {
    env::var(name)
        .ok()
        .map(|value| {
            let value = value.trim().to_ascii_lowercase();
            matches!(value.as_str(), "1" | "true" | "yes" | "on")
        })
        .unwrap_or(false)
}

fn env_u64(name: &str) -> Option<u64> {
    env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
}

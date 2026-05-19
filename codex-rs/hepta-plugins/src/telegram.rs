use hepta_core::PluginManifest;

use crate::GatewayPluginBinding;

pub const TELEGRAM_PLUGIN_ID: &str = "telegram";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelegramPluginServiceMode {
    DryRun,
    ReplyLoopDaemon,
}

impl TelegramPluginServiceMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DryRun => "dry_run",
            Self::ReplyLoopDaemon => "reply_loop_daemon",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramPluginDescriptor {
    pub id: &'static str,
    pub version: &'static str,
    pub display_name: &'static str,
    pub surface_id: &'static str,
    pub transport_key: &'static str,
    pub launch_label: &'static str,
    pub legacy_launch_label: &'static str,
    pub service_host_mode: &'static str,
    pub state_namespace: &'static str,
    pub command_selectors: &'static [&'static str],
    pub runtime_event_targets: &'static [&'static str],
    pub lifecycle_stages: &'static [&'static str],
    pub side_effect_policy: &'static str,
    pub description: &'static str,
}

impl TelegramPluginDescriptor {
    pub fn manifest(&self) -> PluginManifest {
        PluginManifest {
            id: self.id.into(),
            version: self.version.into(),
            description: self.description.into(),
        }
    }

    pub fn service_program_arguments(&self, mode: TelegramPluginServiceMode) -> Vec<String> {
        match mode {
            TelegramPluginServiceMode::DryRun => vec![
                "/telegram-plugin".into(),
                "--dry-run".into(),
                "--json".into(),
            ],
            TelegramPluginServiceMode::ReplyLoopDaemon => vec![
                "/telegram-plugin".into(),
                "--serve-reply-loop".into(),
                "--daemon".into(),
                "--confirm-send".into(),
                "--use-message-text".into(),
                "--poll-ms".into(),
                "1500".into(),
                "--json".into(),
            ],
        }
    }

    pub fn lookup_keys_for_command(&self, command_selector: &str) -> Vec<String> {
        vec![
            format!(
                "surface={}|transport={}|command={}",
                self.surface_id,
                self.transport_key,
                command_selector.trim().to_ascii_lowercase()
            ),
            format!(
                "surface={}|transport={}",
                self.surface_id, self.transport_key
            ),
            format!("surface={}", self.surface_id),
        ]
    }
}

pub fn telegram_plugin_descriptor() -> TelegramPluginDescriptor {
    TelegramPluginDescriptor {
        id: TELEGRAM_PLUGIN_ID,
        version: "0.1.0",
        display_name: "Hepta Telegram Channel Plugin",
        surface_id: "hepta",
        transport_key: "telegram",
        launch_label: "ai.hepta.gateway",
        legacy_launch_label: "ai.hepta.telegram",
        service_host_mode: "gateway_in_process_thread",
        state_namespace: ".hepta/telegram",
        command_selectors: &["/telegram-plugin", "/telegram-adapter", "/telegram-runtime"],
        runtime_event_targets: &[
            "inbound-text-event",
            "message-routing-runtime",
            "channel-send-runtime",
            "reply-dispatch-runtime",
            "channel-runtime-context",
        ],
        lifecycle_stages: &[
            "discover", "enable", "doctor", "dispatch", "health", "disable", "rollback",
        ],
        side_effect_policy: "dry-run by default; live read/send/reply-loop require explicit mode, policy, token readiness, confirmation, redacted ledgers, and readback evidence",
        description: "Rust-native Telegram Bot API channel plugin hosted by the Hepta gateway for inbound polling, command routing, reply-loop service entry, and confirmation-gated Bot API egress",
    }
}

pub fn telegram_plugin_manifest() -> PluginManifest {
    telegram_plugin_descriptor().manifest()
}

pub fn telegram_plugin_bindings() -> Vec<GatewayPluginBinding> {
    let descriptor = telegram_plugin_descriptor();
    let mut bindings = vec![
        GatewayPluginBinding::for_surface(
            descriptor.id,
            descriptor.surface_id,
            "Telegram plugin surface fallback for Hepta channel events",
        ),
        GatewayPluginBinding::new(
            descriptor.id,
            descriptor.surface_id,
            descriptor.transport_key,
            "Telegram plugin transport binding for Bot API ingress and egress",
        ),
    ];
    bindings.extend(descriptor.command_selectors.iter().map(|command| {
        GatewayPluginBinding::new(
            descriptor.id,
            descriptor.surface_id,
            descriptor.transport_key,
            "Telegram plugin command entrypoint",
        )
        .with_command_selector(*command)
    }));
    bindings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GatewayPluginBindingCatalog, GatewayPluginBindingTier};

    #[test]
    fn telegram_plugin_manifest_and_bindings_cover_service_entrypoint() {
        let descriptor = telegram_plugin_descriptor();
        let manifest = telegram_plugin_manifest();
        assert_eq!(manifest.id, TELEGRAM_PLUGIN_ID);
        assert_eq!(descriptor.launch_label, "ai.hepta.gateway");
        assert_eq!(descriptor.legacy_launch_label, "ai.hepta.telegram");
        assert_eq!(descriptor.service_host_mode, "gateway_in_process_thread");
        assert!(
            descriptor
                .service_program_arguments(TelegramPluginServiceMode::ReplyLoopDaemon)
                .contains(&"/telegram-plugin".to_string())
        );

        let mut catalog = GatewayPluginBindingCatalog::new();
        for binding in telegram_plugin_bindings() {
            catalog.register(binding);
        }
        let resolution =
            catalog.resolve_lookup_keys(descriptor.lookup_keys_for_command("/telegram-plugin"));

        assert_eq!(resolution.matches.len(), 3);
        assert_eq!(
            resolution.match_tiers(),
            vec![
                Some(GatewayPluginBindingTier::Command),
                Some(GatewayPluginBindingTier::Transport),
                Some(GatewayPluginBindingTier::Surface),
            ]
        );
        assert!(
            resolution
                .plugin_ids()
                .iter()
                .all(|id| *id == TELEGRAM_PLUGIN_ID)
        );
    }
}

use crate::{GatewayEnvelope, GatewayRoutePlan, GatewayTransport};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginHandoffDraft {
    pub surface_id: String,
    pub session_key: String,
    pub transport_key: String,
    pub normalized_text: String,
    pub command_selector: Option<String>,
}

impl GatewayPluginHandoffDraft {
    pub fn from_route(route: &GatewayRoutePlan) -> Self {
        Self {
            surface_id: normalize_surface_id(&route.surface_id),
            session_key: route.session_key.clone(),
            transport_key: transport_key(route.transport).to_string(),
            normalized_text: route.normalized_text.clone(),
            command_selector: command_selector(&route.normalized_text),
        }
    }

    pub fn binding_lookup_keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        if let Some(command_selector) = self.command_selector.as_deref() {
            keys.push(format!(
                "surface={}|transport={}|command={}",
                self.surface_id, self.transport_key, command_selector
            ));
        }
        keys.push(format!(
            "surface={}|transport={}",
            self.surface_id, self.transport_key
        ));
        keys.push(format!("surface={}", self.surface_id));
        keys
    }
}

pub fn plugin_handoff_draft(envelope: &GatewayEnvelope) -> GatewayPluginHandoffDraft {
    GatewayPluginHandoffDraft::from_route(&crate::GatewaySurface.route_plan(envelope))
}

fn normalize_surface_id(surface_id: &str) -> String {
    surface_id.trim().to_ascii_lowercase()
}

fn transport_key(transport: GatewayTransport) -> &'static str {
    match transport {
        GatewayTransport::Cli => "cli",
        GatewayTransport::Webhook => "webhook",
        GatewayTransport::Queue => "queue",
    }
}

fn command_selector(normalized_text: &str) -> Option<String> {
    let token = normalized_text.split_whitespace().next()?.trim();
    if token.is_empty() {
        return None;
    }

    if token.starts_with('/') || token.starts_with('!') {
        return Some(token.to_ascii_lowercase());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{GatewayPluginHandoffDraft, plugin_handoff_draft};
    use crate::{GatewayEnvelope, GatewayRoutePlan, GatewayTransport};

    #[test]
    fn handoff_draft_extracts_command_selector_from_route() {
        let route = GatewayRoutePlan::new(
            "hepta",
            "session-42",
            GatewayTransport::Webhook,
            "/Status --json",
        );

        let draft = GatewayPluginHandoffDraft::from_route(&route);

        assert_eq!(draft.surface_id, "hepta");
        assert_eq!(draft.session_key, "session-42");
        assert_eq!(draft.transport_key, "webhook");
        assert_eq!(draft.command_selector.as_deref(), Some("/status"));
        assert_eq!(
            draft.binding_lookup_keys(),
            vec![
                "surface=hepta|transport=webhook|command=/status",
                "surface=hepta|transport=webhook",
                "surface=hepta",
            ]
        );
    }

    #[test]
    fn handoff_draft_leaves_free_text_without_command_selector() {
        let envelope = GatewayEnvelope::new(
            "telegram",
            "user-9",
            GatewayTransport::Cli,
            "  hello there  ",
        );

        let draft = plugin_handoff_draft(&envelope);

        assert_eq!(draft.transport_key, "cli");
        assert_eq!(draft.command_selector, None);
        assert_eq!(
            draft.binding_lookup_keys(),
            vec!["surface=telegram|transport=cli", "surface=telegram",]
        );
    }

    #[test]
    fn handoff_draft_supports_bang_commands_too() {
        let route =
            GatewayRoutePlan::new("discord", "session-9", GatewayTransport::Cli, "!HELP me");

        let draft = GatewayPluginHandoffDraft::from_route(&route);

        assert_eq!(draft.command_selector.as_deref(), Some("!help"));
    }

    #[test]
    fn handoff_draft_normalizes_surface_id_for_binding_contracts() {
        let route = GatewayRoutePlan::new(
            " Hepta ",
            "session-42",
            GatewayTransport::Webhook,
            "/status",
        );

        let draft = GatewayPluginHandoffDraft::from_route(&route);

        assert_eq!(draft.surface_id, "hepta");
        assert_eq!(
            draft.binding_lookup_keys(),
            vec![
                "surface=hepta|transport=webhook|command=/status",
                "surface=hepta|transport=webhook",
                "surface=hepta",
            ]
        );
    }
}

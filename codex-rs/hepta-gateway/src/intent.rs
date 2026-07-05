use crate::GatewayPluginHandoffDraft;
use crate::GatewayResolvedPluginTier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayHandoffLookupIntentNote {
    pub surface_id: Option<String>,
    pub transport_key: Option<String>,
    pub command_selector: Option<String>,
    pub requested_lookup_keys: Vec<String>,
    pub requested_tier_labels: Vec<Option<String>>,
    pub intent_label: String,
    pub explanation: String,
}

impl GatewayPluginHandoffDraft {
    pub fn lookup_intent_note(&self) -> GatewayHandoffLookupIntentNote {
        let requested_lookup_keys = self.binding_lookup_keys();
        let requested_tier_labels = requested_lookup_keys
            .iter()
            .map(|lookup_key| {
                GatewayResolvedPluginTier::from_lookup_key(lookup_key)
                    .map(|tier| tier.as_str().to_string())
            })
            .collect::<Vec<_>>();
        let surface_id = normalized_field(&self.surface_id);
        let transport_key = normalized_field(&self.transport_key);
        let command_selector = self.command_selector.clone();
        let intent_label =
            lookup_intent_label(command_selector.as_deref(), &requested_lookup_keys).to_string();

        GatewayHandoffLookupIntentNote {
            surface_id,
            transport_key,
            command_selector: command_selector.clone(),
            requested_lookup_keys,
            requested_tier_labels,
            explanation: lookup_intent_explanation(
                intent_label.as_str(),
                command_selector.as_deref(),
            ),
            intent_label,
        }
    }
}

fn normalized_field(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn lookup_intent_label(
    command_selector: Option<&str>,
    requested_lookup_keys: &[String],
) -> &'static str {
    if command_selector.is_some() {
        "command_lookup"
    } else if requested_lookup_keys
        .iter()
        .any(|lookup_key| lookup_key.contains("|transport="))
    {
        "transport_fallback_lookup"
    } else if requested_lookup_keys
        .iter()
        .any(|lookup_key| lookup_key.starts_with("surface="))
    {
        "surface_fallback_lookup"
    } else {
        "empty_lookup"
    }
}

fn lookup_intent_explanation(intent_label: &str, command_selector: Option<&str>) -> String {
    match intent_label {
        "command_lookup" => match command_selector {
            Some(command_selector) => format!(
                "lookup requests command selector {command_selector} before transport and surface fallback"
            ),
            None => {
                "lookup requests command coverage before transport and surface fallback".to_string()
            }
        },
        "transport_fallback_lookup" => {
            "lookup omits a command selector and starts at transport fallback".to_string()
        }
        "surface_fallback_lookup" => "lookup only requests surface fallback coverage".to_string(),
        _ => "lookup request is empty and carries no binding intent".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::GatewayPluginHandoffDraft;
    use crate::GatewayRoutePlan;
    use crate::GatewayTransport;

    #[test]
    fn handoff_draft_reports_command_lookup_intent() {
        let note = GatewayPluginHandoffDraft::from_route(&GatewayRoutePlan::new(
            " Hepta ",
            "session-42",
            GatewayTransport::Webhook,
            " /Status --json ",
        ))
        .lookup_intent_note();

        assert_eq!(note.surface_id.as_deref(), Some("hepta"));
        assert_eq!(note.transport_key.as_deref(), Some("webhook"));
        assert_eq!(note.command_selector.as_deref(), Some("/status"));
        assert_eq!(note.intent_label, "command_lookup");
        assert_eq!(
            note.requested_lookup_keys,
            vec![
                "surface=hepta|transport=webhook|command=/status".to_string(),
                "surface=hepta|transport=webhook".to_string(),
                "surface=hepta".to_string(),
            ]
        );
        assert_eq!(
            note.requested_tier_labels,
            vec![
                Some("command".to_string()),
                Some("transport".to_string()),
                Some("surface".to_string()),
            ]
        );
        assert_eq!(
            note.explanation,
            "lookup requests command selector /status before transport and surface fallback"
        );
    }

    #[test]
    fn handoff_draft_reports_transport_fallback_lookup_intent() {
        let note = GatewayPluginHandoffDraft::from_route(&GatewayRoutePlan::new(
            "telegram",
            "session-9",
            GatewayTransport::Cli,
            " hello there ",
        ))
        .lookup_intent_note();

        assert_eq!(note.surface_id.as_deref(), Some("telegram"));
        assert_eq!(note.transport_key.as_deref(), Some("cli"));
        assert_eq!(note.command_selector, None);
        assert_eq!(note.intent_label, "transport_fallback_lookup");
        assert_eq!(
            note.requested_lookup_keys,
            vec![
                "surface=telegram|transport=cli".to_string(),
                "surface=telegram".to_string(),
            ]
        );
        assert_eq!(
            note.requested_tier_labels,
            vec![Some("transport".to_string()), Some("surface".to_string()),]
        );
        assert_eq!(
            note.explanation,
            "lookup omits a command selector and starts at transport fallback"
        );
    }
}

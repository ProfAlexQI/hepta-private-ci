use crate::GatewayPluginBindingLookupResolution;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginLookupIntentNote {
    pub surface_id: Option<String>,
    pub transport_key: Option<String>,
    pub command_selector: Option<String>,
    pub requested_lookup_keys: Vec<String>,
    pub requested_tier_labels: Vec<Option<String>>,
    pub intent_label: String,
    pub explanation: String,
}

impl GatewayPluginBindingLookupResolution {
    pub fn lookup_intent_note(&self) -> GatewayPluginLookupIntentNote {
        let shape = RequestedLookupShape::from_lookup_keys(&self.requested_lookup_keys);
        let intent_label = shape.intent_label().to_string();

        GatewayPluginLookupIntentNote {
            surface_id: shape.surface_id,
            transport_key: shape.transport_key,
            command_selector: shape.command_selector.clone(),
            requested_lookup_keys: self.requested_lookup_keys.clone(),
            requested_tier_labels: self
                .requested_lookup_tiers()
                .into_iter()
                .map(|tier| tier.map(|tier| tier.as_str().to_string()))
                .collect(),
            explanation: lookup_intent_explanation(
                intent_label.as_str(),
                shape.command_selector.as_deref(),
            ),
            intent_label,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct RequestedLookupShape {
    surface_id: Option<String>,
    transport_key: Option<String>,
    command_selector: Option<String>,
    has_surface: bool,
    has_transport: bool,
    has_command: bool,
}

impl RequestedLookupShape {
    fn from_lookup_keys(lookup_keys: &[String]) -> Self {
        let mut shape = Self::default();

        for lookup_key in lookup_keys {
            let mut saw_surface = false;
            let mut saw_transport = false;
            let mut saw_command = false;

            for segment in lookup_key.split('|').map(str::trim) {
                if let Some(surface_id) = segment.strip_prefix("surface=") {
                    saw_surface = true;
                    if shape.surface_id.is_none() {
                        shape.surface_id = normalized_field(surface_id);
                    }
                } else if let Some(transport_key) = segment.strip_prefix("transport=") {
                    saw_transport = true;
                    if shape.transport_key.is_none() {
                        shape.transport_key = normalized_field(transport_key);
                    }
                } else if let Some(command_selector) = segment.strip_prefix("command=") {
                    saw_command = true;
                    if shape.command_selector.is_none() {
                        shape.command_selector = normalized_field(command_selector);
                    }
                }
            }

            shape.has_surface |= saw_surface;
            shape.has_transport |= saw_transport;
            shape.has_command |= saw_command;
        }

        shape
    }

    fn intent_label(&self) -> &'static str {
        if self.has_command {
            "command_lookup"
        } else if self.has_transport {
            "transport_fallback_lookup"
        } else if self.has_surface {
            "surface_fallback_lookup"
        } else {
            "empty_lookup"
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
    use crate::GatewayPluginBinding;
    use crate::GatewayPluginBindingCatalog;
    use crate::GatewayPluginLookupIntentNote;

    #[test]
    fn lookup_resolution_reports_command_lookup_intent() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(
            GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status")
                .with_command_selector("/status"),
        );

        let note = catalog
            .resolve_lookup_keys([
                "surface=hepta|transport=webhook|command=/status",
                "surface=hepta|transport=webhook",
                "surface=hepta",
            ])
            .lookup_intent_note();

        assert_eq!(
            note,
            GatewayPluginLookupIntentNote {
                surface_id: Some("hepta".to_string()),
                transport_key: Some("webhook".to_string()),
                command_selector: Some("/status".to_string()),
                requested_lookup_keys: vec![
                    "surface=hepta|transport=webhook|command=/status".to_string(),
                    "surface=hepta|transport=webhook".to_string(),
                    "surface=hepta".to_string(),
                ],
                requested_tier_labels: vec![
                    Some("command".to_string()),
                    Some("transport".to_string()),
                    Some("surface".to_string()),
                ],
                intent_label: "command_lookup".to_string(),
                explanation:
                    "lookup requests command selector /status before transport and surface fallback"
                        .to_string(),
            }
        );
    }

    #[test]
    fn lookup_resolution_reports_surface_fallback_when_only_surface_key_is_requested() {
        let note = GatewayPluginBindingCatalog::new()
            .resolve_lookup_keys(["surface=hepta"])
            .lookup_intent_note();

        assert_eq!(note.surface_id.as_deref(), Some("hepta"));
        assert_eq!(note.transport_key, None);
        assert_eq!(note.command_selector, None);
        assert_eq!(note.intent_label, "surface_fallback_lookup");
        assert_eq!(
            note.explanation,
            "lookup only requests surface fallback coverage"
        );
    }

    #[test]
    fn lookup_resolution_reports_empty_lookup_when_no_keys_survive_normalization() {
        let note = GatewayPluginBindingCatalog::new()
            .resolve_lookup_keys(["  ", "\n\t"])
            .lookup_intent_note();

        assert_eq!(note.surface_id, None);
        assert_eq!(note.transport_key, None);
        assert_eq!(note.command_selector, None);
        assert!(note.requested_lookup_keys.is_empty());
        assert!(note.requested_tier_labels.is_empty());
        assert_eq!(note.intent_label, "empty_lookup");
        assert_eq!(
            note.explanation,
            "lookup request is empty and carries no binding intent"
        );
    }
}

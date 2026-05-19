use crate::{GatewayPluginBindingLookupResolution, GatewayPluginBindingTier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginBindingLookupTraceStep {
    pub lookup_key: String,
    pub tier_label: Option<String>,
    pub matched_plugin_ids: Vec<String>,
    pub matched: bool,
    pub note: String,
}

impl GatewayPluginBindingLookupResolution {
    pub fn lookup_trace_steps(&self) -> Vec<GatewayPluginBindingLookupTraceStep> {
        self.requested_lookup_keys
            .iter()
            .map(|lookup_key| {
                let matched_plugin_ids = self
                    .matches
                    .iter()
                    .filter(|binding| binding.lookup_key == lookup_key.as_str())
                    .map(|binding| binding.plugin_id.clone())
                    .collect::<Vec<_>>();

                GatewayPluginBindingLookupTraceStep::from_lookup_key(lookup_key, matched_plugin_ids)
            })
            .collect()
    }
}

impl GatewayPluginBindingLookupTraceStep {
    pub fn from_lookup_key(
        lookup_key: &str,
        matched_plugin_ids: impl IntoIterator<Item = String>,
    ) -> Self {
        let matched_plugin_ids = matched_plugin_ids.into_iter().collect::<Vec<_>>();
        let tier = GatewayPluginBindingTier::from_lookup_key(lookup_key);
        let matched = !matched_plugin_ids.is_empty();

        Self {
            lookup_key: lookup_key.to_string(),
            tier_label: tier.map(|tier| tier.as_str().to_string()),
            note: trace_note(tier, matched, &matched_plugin_ids),
            matched_plugin_ids,
            matched,
        }
    }
}

fn trace_note(
    tier: Option<GatewayPluginBindingTier>,
    matched: bool,
    matched_plugin_ids: &[String],
) -> String {
    let tier_label = match tier {
        Some(GatewayPluginBindingTier::Command) => "command",
        Some(GatewayPluginBindingTier::Transport) => "transport",
        Some(GatewayPluginBindingTier::Surface) => "surface",
        None => "lookup",
    };

    if matched {
        format!(
            "{tier_label} lookup is covered by {}",
            matched_plugin_ids.join(", ")
        )
    } else {
        format!("{tier_label} lookup remains uncovered")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        GatewayPluginBinding, GatewayPluginBindingCatalog, GatewayPluginBindingLookupTraceStep,
    };

    #[test]
    fn lookup_resolution_can_emit_lookup_trace_steps() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(GatewayPluginBinding::new(
            "fallback-plugin",
            "telegram",
            "cli",
            "general cli hooks",
        ));

        let steps = catalog
            .resolve_lookup_keys([
                "surface=telegram|transport=cli|command=/status",
                "surface=telegram|transport=cli",
                "surface=telegram",
            ])
            .lookup_trace_steps();

        assert_eq!(
            steps,
            vec![
                GatewayPluginBindingLookupTraceStep {
                    lookup_key: "surface=telegram|transport=cli|command=/status".to_string(),
                    tier_label: Some("command".to_string()),
                    matched_plugin_ids: Vec::new(),
                    matched: false,
                    note: "command lookup remains uncovered".to_string(),
                },
                GatewayPluginBindingLookupTraceStep {
                    lookup_key: "surface=telegram|transport=cli".to_string(),
                    tier_label: Some("transport".to_string()),
                    matched_plugin_ids: vec!["fallback-plugin".to_string()],
                    matched: true,
                    note: "transport lookup is covered by fallback-plugin".to_string(),
                },
                GatewayPluginBindingLookupTraceStep {
                    lookup_key: "surface=telegram".to_string(),
                    tier_label: Some("surface".to_string()),
                    matched_plugin_ids: Vec::new(),
                    matched: false,
                    note: "surface lookup remains uncovered".to_string(),
                },
            ]
        );
    }

    #[test]
    fn lookup_trace_step_can_summarize_multiple_matches_for_one_lookup_key() {
        let step = GatewayPluginBindingLookupTraceStep::from_lookup_key(
            "surface=hepta|transport=webhook",
            ["alpha-plugin".to_string(), "beta-plugin".to_string()],
        );

        assert_eq!(step.tier_label.as_deref(), Some("transport"));
        assert!(step.matched);
        assert_eq!(
            step.note,
            "transport lookup is covered by alpha-plugin, beta-plugin"
        );
    }
}

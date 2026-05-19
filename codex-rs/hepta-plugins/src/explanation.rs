use crate::{
    GatewayPluginBindingLookupResolution, GatewayPluginBindingMatch,
    GatewayPluginBindingResolution, GatewayPluginBindingTier,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginBindingDiagnosticNote {
    pub plugin_id: String,
    pub lookup_key: String,
    pub tier_label: Option<String>,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginCoverageGapNote {
    pub lookup_key: String,
    pub tier_label: Option<String>,
    pub explanation: String,
}

impl GatewayPluginBindingMatch {
    pub fn diagnostic_note(&self) -> GatewayPluginBindingDiagnosticNote {
        let tier = self.match_tier();

        GatewayPluginBindingDiagnosticNote {
            plugin_id: self.plugin_id.clone(),
            lookup_key: self.lookup_key.clone(),
            tier_label: tier.map(|tier| tier.as_str().to_string()),
            explanation: matched_explanation(tier, &self.lookup_key),
        }
    }
}

impl GatewayPluginBindingResolution {
    pub fn diagnostic_notes(&self) -> Vec<GatewayPluginBindingDiagnosticNote> {
        self.matches
            .iter()
            .map(GatewayPluginBindingMatch::diagnostic_note)
            .collect()
    }
}

impl GatewayPluginBindingLookupResolution {
    pub fn diagnostic_notes(&self) -> Vec<GatewayPluginBindingDiagnosticNote> {
        self.matches
            .iter()
            .map(GatewayPluginBindingMatch::diagnostic_note)
            .collect()
    }

    pub fn coverage_gap_notes(&self) -> Vec<GatewayPluginCoverageGapNote> {
        self.unmatched_lookup_keys()
            .into_iter()
            .map(|lookup_key| {
                let tier = GatewayPluginBindingTier::from_lookup_key(lookup_key);

                GatewayPluginCoverageGapNote {
                    lookup_key: lookup_key.to_string(),
                    tier_label: tier.map(|tier| tier.as_str().to_string()),
                    explanation: gap_explanation(tier, lookup_key),
                }
            })
            .collect()
    }
}

fn matched_explanation(tier: Option<GatewayPluginBindingTier>, lookup_key: &str) -> String {
    match tier {
        Some(GatewayPluginBindingTier::Command) => {
            format!("matched command binding via {lookup_key}")
        }
        Some(GatewayPluginBindingTier::Transport) => {
            format!("matched transport fallback via {lookup_key}")
        }
        Some(GatewayPluginBindingTier::Surface) => {
            format!("matched surface fallback via {lookup_key}")
        }
        None => format!("matched binding via {lookup_key}"),
    }
}

fn gap_explanation(tier: Option<GatewayPluginBindingTier>, lookup_key: &str) -> String {
    match tier {
        Some(GatewayPluginBindingTier::Command) => {
            format!("no command binding matched for {lookup_key}")
        }
        Some(GatewayPluginBindingTier::Transport) => {
            format!("no transport fallback matched for {lookup_key}")
        }
        Some(GatewayPluginBindingTier::Surface) => {
            format!("no surface fallback matched for {lookup_key}")
        }
        None => format!("no binding matched for {lookup_key}"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{GatewayPluginBinding, GatewayPluginBindingCatalog};

    #[test]
    fn route_resolution_exposes_diagnostic_notes_with_stable_tier_labels() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(GatewayPluginBinding::for_surface(
            "surface-plugin",
            "hepta",
            "surface-wide hooks",
        ));
        catalog.register(GatewayPluginBinding::new(
            "fallback-plugin",
            "hepta",
            "webhook",
            "general ingress hooks",
        ));
        catalog.register(
            GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status hooks")
                .with_command_selector("/status"),
        );

        let notes = catalog
            .resolve_route("hepta", "webhook", Some("/status"))
            .diagnostic_notes();

        assert_eq!(
            notes
                .iter()
                .map(|note| {
                    (
                        note.plugin_id.as_str(),
                        note.lookup_key.as_str(),
                        note.tier_label.as_deref(),
                        note.explanation.as_str(),
                    )
                })
                .collect::<Vec<_>>(),
            vec![
                (
                    "status-plugin",
                    "surface=hepta|transport=webhook|command=/status",
                    Some("command"),
                    "matched command binding via surface=hepta|transport=webhook|command=/status",
                ),
                (
                    "fallback-plugin",
                    "surface=hepta|transport=webhook",
                    Some("transport"),
                    "matched transport fallback via surface=hepta|transport=webhook",
                ),
                (
                    "surface-plugin",
                    "surface=hepta",
                    Some("surface"),
                    "matched surface fallback via surface=hepta",
                ),
            ]
        );
    }

    #[test]
    fn lookup_resolution_exposes_coverage_gap_notes() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(GatewayPluginBinding::new(
            "fallback-plugin",
            "telegram",
            "cli",
            "general cli hooks",
        ));

        let notes = catalog
            .resolve_lookup_keys([
                "surface=telegram|transport=cli|command=/status",
                "surface=telegram|transport=cli",
                "surface=telegram",
            ])
            .coverage_gap_notes();

        assert_eq!(
            notes
                .iter()
                .map(|note| {
                    (
                        note.lookup_key.as_str(),
                        note.tier_label.as_deref(),
                        note.explanation.as_str(),
                    )
                })
                .collect::<Vec<_>>(),
            vec![
                (
                    "surface=telegram|transport=cli|command=/status",
                    Some("command"),
                    "no command binding matched for surface=telegram|transport=cli|command=/status",
                ),
                (
                    "surface=telegram",
                    Some("surface"),
                    "no surface fallback matched for surface=telegram",
                ),
            ]
        );
    }
}

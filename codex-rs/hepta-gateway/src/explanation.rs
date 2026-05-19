use crate::{
    GatewayPluginResolutionSnapshot, GatewayResolvedPluginCandidate, GatewayResolvedPluginTier,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayResolvedPluginDiagnosticNote {
    pub plugin_id: String,
    pub lookup_key: String,
    pub tier_label: Option<String>,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayResolutionCoverageGapNote {
    pub lookup_key: String,
    pub tier_label: Option<String>,
    pub explanation: String,
}

impl GatewayResolvedPluginCandidate {
    pub fn diagnostic_note(&self) -> GatewayResolvedPluginDiagnosticNote {
        let tier = self.match_tier();

        GatewayResolvedPluginDiagnosticNote {
            plugin_id: self.plugin_id.clone(),
            lookup_key: self.lookup_key.clone(),
            tier_label: tier.map(|tier| tier.as_str().to_string()),
            explanation: matched_explanation(tier, &self.lookup_key),
        }
    }
}

impl GatewayPluginResolutionSnapshot {
    pub fn diagnostic_notes(&self) -> Vec<GatewayResolvedPluginDiagnosticNote> {
        self.candidates
            .iter()
            .map(GatewayResolvedPluginCandidate::diagnostic_note)
            .collect()
    }

    pub fn coverage_gap_notes(&self) -> Vec<GatewayResolutionCoverageGapNote> {
        self.unmatched_lookup_keys()
            .into_iter()
            .map(|lookup_key| {
                let tier = GatewayResolvedPluginTier::from_lookup_key(lookup_key);

                GatewayResolutionCoverageGapNote {
                    lookup_key: lookup_key.to_string(),
                    tier_label: tier.map(|tier| tier.as_str().to_string()),
                    explanation: gap_explanation(tier, lookup_key),
                }
            })
            .collect()
    }
}

fn matched_explanation(tier: Option<GatewayResolvedPluginTier>, lookup_key: &str) -> String {
    match tier {
        Some(GatewayResolvedPluginTier::Command) => {
            format!("matched command candidate via {lookup_key}")
        }
        Some(GatewayResolvedPluginTier::Transport) => {
            format!("matched transport fallback via {lookup_key}")
        }
        Some(GatewayResolvedPluginTier::Surface) => {
            format!("matched surface fallback via {lookup_key}")
        }
        None => format!("matched candidate via {lookup_key}"),
    }
}

fn gap_explanation(tier: Option<GatewayResolvedPluginTier>, lookup_key: &str) -> String {
    match tier {
        Some(GatewayResolvedPluginTier::Command) => {
            format!("no command candidate recorded for {lookup_key}")
        }
        Some(GatewayResolvedPluginTier::Transport) => {
            format!("no transport fallback recorded for {lookup_key}")
        }
        Some(GatewayResolvedPluginTier::Surface) => {
            format!("no surface fallback recorded for {lookup_key}")
        }
        None => format!("no candidate recorded for {lookup_key}"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        GatewayPluginHandoffDraft, GatewayPluginResolutionSnapshot, GatewayResolvedPluginCandidate,
        GatewayRoutePlan, GatewayTransport,
    };

    #[test]
    fn snapshot_exposes_diagnostic_notes_with_stable_tier_labels() {
        let draft = GatewayPluginHandoffDraft::from_route(&GatewayRoutePlan::new(
            "hepta",
            "session-42",
            GatewayTransport::Webhook,
            "/status --json",
        ));
        let notes = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft)
            .with_candidates([
                GatewayResolvedPluginCandidate::new(
                    "status-plugin",
                    "surface=hepta|transport=webhook|command=/status",
                    2,
                ),
                GatewayResolvedPluginCandidate::new(
                    "fallback-plugin",
                    "surface=hepta|transport=webhook",
                    1,
                ),
                GatewayResolvedPluginCandidate::new("surface-plugin", "surface=hepta", 0),
            ])
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
                    "matched command candidate via surface=hepta|transport=webhook|command=/status",
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
    fn snapshot_exposes_coverage_gap_notes() {
        let draft = GatewayPluginHandoffDraft::from_route(&GatewayRoutePlan::new(
            "telegram",
            "session-9",
            GatewayTransport::Cli,
            "/status",
        ));
        let notes = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft)
            .with_candidates([GatewayResolvedPluginCandidate::new(
                "fallback-plugin",
                "surface=telegram|transport=cli",
                1,
            )])
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
                    "no command candidate recorded for surface=telegram|transport=cli|command=/status",
                ),
                (
                    "surface=telegram",
                    Some("surface"),
                    "no surface fallback recorded for surface=telegram",
                ),
            ]
        );
    }
}

use crate::GatewayPluginHandoffDraft;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayResolvedPluginTier {
    Surface,
    Transport,
    Command,
}

impl GatewayResolvedPluginTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Surface => "surface",
            Self::Transport => "transport",
            Self::Command => "command",
        }
    }

    pub fn from_lookup_key(lookup_key: &str) -> Option<Self> {
        let lookup_key = lookup_key.trim();
        if lookup_key.is_empty() {
            return None;
        }

        if lookup_key.contains("|command=") {
            Some(Self::Command)
        } else if lookup_key.contains("|transport=") {
            Some(Self::Transport)
        } else if lookup_key.starts_with("surface=") {
            Some(Self::Surface)
        } else {
            None
        }
    }

    pub fn from_specificity_score(score: usize) -> Option<Self> {
        match score {
            0 => Some(Self::Surface),
            1 => Some(Self::Transport),
            2 => Some(Self::Command),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayResolvedPluginCandidate {
    pub plugin_id: String,
    pub lookup_key: String,
    pub specificity_score: usize,
}

impl GatewayResolvedPluginCandidate {
    pub fn new(
        plugin_id: impl Into<String>,
        lookup_key: impl Into<String>,
        specificity_score: usize,
    ) -> Self {
        Self {
            plugin_id: plugin_id.into().trim().to_string(),
            lookup_key: lookup_key.into().trim().to_string(),
            specificity_score,
        }
    }

    pub fn match_tier(&self) -> Option<GatewayResolvedPluginTier> {
        GatewayResolvedPluginTier::from_lookup_key(&self.lookup_key)
            .or_else(|| GatewayResolvedPluginTier::from_specificity_score(self.specificity_score))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginResolutionSnapshot {
    pub surface_id: String,
    pub session_key: String,
    pub transport_key: String,
    pub normalized_text: String,
    pub command_selector: Option<String>,
    pub binding_lookup_keys: Vec<String>,
    pub candidates: Vec<GatewayResolvedPluginCandidate>,
}

impl GatewayPluginResolutionSnapshot {
    pub fn from_handoff_draft(draft: &GatewayPluginHandoffDraft) -> Self {
        Self {
            surface_id: draft.surface_id.clone(),
            session_key: draft.session_key.clone(),
            transport_key: draft.transport_key.clone(),
            normalized_text: draft.normalized_text.clone(),
            command_selector: draft.command_selector.clone(),
            binding_lookup_keys: draft.binding_lookup_keys(),
            candidates: Vec::new(),
        }
    }

    pub fn with_candidates(
        mut self,
        candidates: impl IntoIterator<Item = GatewayResolvedPluginCandidate>,
    ) -> Self {
        self.candidates = candidates.into_iter().collect();
        self
    }

    pub fn has_candidates(&self) -> bool {
        !self.candidates.is_empty()
    }

    pub fn plugin_ids(&self) -> Vec<&str> {
        self.candidates
            .iter()
            .map(|candidate| candidate.plugin_id.as_str())
            .collect()
    }

    pub fn matched_lookup_keys(&self) -> Vec<&str> {
        self.candidates
            .iter()
            .map(|candidate| candidate.lookup_key.as_str())
            .collect()
    }

    pub fn unmatched_lookup_keys(&self) -> Vec<&str> {
        self.binding_lookup_keys
            .iter()
            .filter(|lookup_key| {
                !self
                    .candidates
                    .iter()
                    .any(|candidate| candidate.lookup_key == lookup_key.as_str())
            })
            .map(String::as_str)
            .collect()
    }

    pub fn candidate_tiers(&self) -> Vec<Option<GatewayResolvedPluginTier>> {
        self.candidates
            .iter()
            .map(GatewayResolvedPluginCandidate::match_tier)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GatewayPluginResolutionSnapshot, GatewayResolvedPluginCandidate, GatewayResolvedPluginTier,
    };
    use crate::{GatewayRoutePlan, GatewayTransport};

    #[test]
    fn resolution_snapshot_starts_from_handoff_contract() {
        let draft = crate::GatewayPluginHandoffDraft::from_route(&GatewayRoutePlan::new(
            " Hepta ",
            "session-42",
            GatewayTransport::Webhook,
            " /Status --json ",
        ));

        let snapshot = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft);

        assert_eq!(snapshot.surface_id, "hepta");
        assert_eq!(snapshot.session_key, "session-42");
        assert_eq!(snapshot.transport_key, "webhook");
        assert_eq!(snapshot.command_selector.as_deref(), Some("/status"));
        assert_eq!(
            snapshot.binding_lookup_keys,
            vec![
                "surface=hepta|transport=webhook|command=/status",
                "surface=hepta|transport=webhook",
                "surface=hepta",
            ]
        );
        assert!(!snapshot.has_candidates());
    }

    #[test]
    fn resolution_snapshot_preserves_candidate_order() {
        let draft = crate::GatewayPluginHandoffDraft::from_route(&GatewayRoutePlan::new(
            "hepta",
            "session-42",
            GatewayTransport::Webhook,
            "/status",
        ));

        let snapshot =
            GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates([
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
            ]);

        assert!(snapshot.has_candidates());
        assert_eq!(
            snapshot.plugin_ids(),
            vec!["status-plugin", "fallback-plugin"]
        );
        assert_eq!(snapshot.candidates[0].specificity_score, 2);
        assert_eq!(snapshot.candidates[1].specificity_score, 1);
        assert_eq!(
            snapshot.candidate_tiers(),
            vec![
                Some(GatewayResolvedPluginTier::Command),
                Some(GatewayResolvedPluginTier::Transport),
            ]
        );
        assert_eq!(
            snapshot.matched_lookup_keys(),
            vec![
                "surface=hepta|transport=webhook|command=/status",
                "surface=hepta|transport=webhook",
            ]
        );
        assert_eq!(snapshot.unmatched_lookup_keys(), vec!["surface=hepta"]);
    }

    #[test]
    fn resolved_candidate_can_expose_match_tier_without_plugin_runtime_dependency() {
        let candidate = GatewayResolvedPluginCandidate::new("surface-plugin", "surface=hepta", 0);

        assert_eq!(
            candidate.match_tier(),
            Some(GatewayResolvedPluginTier::Surface)
        );
        assert_eq!(GatewayResolvedPluginTier::Command.as_str(), "command");
    }

    #[test]
    fn resolution_snapshot_can_report_full_lookup_coverage() {
        let draft = crate::GatewayPluginHandoffDraft::from_route(&GatewayRoutePlan::new(
            "Hepta",
            "session-42",
            GatewayTransport::Webhook,
            "/status",
        ));

        let snapshot =
            GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates([
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
            ]);

        assert_eq!(
            snapshot.matched_lookup_keys(),
            vec![
                "surface=hepta|transport=webhook|command=/status",
                "surface=hepta|transport=webhook",
                "surface=hepta",
            ]
        );
        assert!(snapshot.unmatched_lookup_keys().is_empty());
    }
}

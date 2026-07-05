use crate::GatewayPluginResolutionSnapshot;
use crate::GatewayResolvedPluginTier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginResolutionCoverageDigest {
    pub requested_lookup_keys: Vec<String>,
    pub requested_tier_labels: Vec<Option<String>>,
    pub matched_lookup_keys: Vec<String>,
    pub matched_tier_labels: Vec<Option<String>>,
    pub unmatched_lookup_keys: Vec<String>,
    pub unmatched_tier_labels: Vec<Option<String>>,
    pub full_coverage: bool,
}

impl GatewayPluginResolutionSnapshot {
    pub fn requested_lookup_tiers(&self) -> Vec<Option<GatewayResolvedPluginTier>> {
        self.binding_lookup_keys
            .iter()
            .map(|lookup_key| GatewayResolvedPluginTier::from_lookup_key(lookup_key))
            .collect()
    }

    pub fn unmatched_lookup_tiers(&self) -> Vec<Option<GatewayResolvedPluginTier>> {
        self.unmatched_lookup_keys()
            .into_iter()
            .map(GatewayResolvedPluginTier::from_lookup_key)
            .collect()
    }

    pub fn has_full_coverage(&self) -> bool {
        self.unmatched_lookup_keys().is_empty()
    }

    pub fn coverage_digest(&self) -> GatewayPluginResolutionCoverageDigest {
        let requested_lookup_keys = self.binding_lookup_keys.clone();
        let matched_lookup_keys = self
            .matched_lookup_keys()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let unmatched_lookup_keys = self
            .unmatched_lookup_keys()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();

        GatewayPluginResolutionCoverageDigest {
            requested_tier_labels: self
                .requested_lookup_tiers()
                .into_iter()
                .map(|tier| tier.map(|tier| tier.as_str().to_string()))
                .collect(),
            matched_tier_labels: self
                .candidate_tiers()
                .into_iter()
                .map(|tier| tier.map(|tier| tier.as_str().to_string()))
                .collect(),
            unmatched_tier_labels: self
                .unmatched_lookup_tiers()
                .into_iter()
                .map(|tier| tier.map(|tier| tier.as_str().to_string()))
                .collect(),
            full_coverage: unmatched_lookup_keys.is_empty(),
            requested_lookup_keys,
            matched_lookup_keys,
            unmatched_lookup_keys,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::GatewayPluginHandoffDraft;
    use crate::GatewayPluginResolutionSnapshot;
    use crate::GatewayResolvedPluginCandidate;
    use crate::GatewayRoutePlan;
    use crate::GatewayTransport;

    #[test]
    fn resolution_snapshot_can_summarize_partial_lookup_coverage() {
        let draft = GatewayPluginHandoffDraft::from_route(&GatewayRoutePlan::new(
            "telegram",
            "session-9",
            GatewayTransport::Cli,
            "/status",
        ));
        let snapshot =
            GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates([
                GatewayResolvedPluginCandidate::new(
                    "fallback-plugin",
                    "surface=telegram|transport=cli",
                    1,
                ),
            ]);

        assert_eq!(
            snapshot.requested_lookup_tiers(),
            vec![
                Some(crate::GatewayResolvedPluginTier::Command),
                Some(crate::GatewayResolvedPluginTier::Transport),
                Some(crate::GatewayResolvedPluginTier::Surface),
            ]
        );
        assert_eq!(
            snapshot.unmatched_lookup_tiers(),
            vec![
                Some(crate::GatewayResolvedPluginTier::Command),
                Some(crate::GatewayResolvedPluginTier::Surface),
            ]
        );
        assert!(!snapshot.has_full_coverage());

        assert_eq!(
            snapshot.coverage_digest(),
            crate::GatewayPluginResolutionCoverageDigest {
                requested_lookup_keys: vec![
                    "surface=telegram|transport=cli|command=/status".to_string(),
                    "surface=telegram|transport=cli".to_string(),
                    "surface=telegram".to_string(),
                ],
                requested_tier_labels: vec![
                    Some("command".to_string()),
                    Some("transport".to_string()),
                    Some("surface".to_string()),
                ],
                matched_lookup_keys: vec!["surface=telegram|transport=cli".to_string()],
                matched_tier_labels: vec![Some("transport".to_string())],
                unmatched_lookup_keys: vec![
                    "surface=telegram|transport=cli|command=/status".to_string(),
                    "surface=telegram".to_string(),
                ],
                unmatched_tier_labels: vec![
                    Some("command".to_string()),
                    Some("surface".to_string()),
                ],
                full_coverage: false,
            }
        );
    }

    #[test]
    fn resolution_snapshot_can_summarize_full_lookup_coverage() {
        let draft = GatewayPluginHandoffDraft::from_route(&GatewayRoutePlan::new(
            "hepta",
            "session-42",
            GatewayTransport::Webhook,
            "/status --json",
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

        assert!(snapshot.has_full_coverage());
        assert!(snapshot.unmatched_lookup_tiers().is_empty());
        assert!(snapshot.coverage_digest().full_coverage);
    }
}

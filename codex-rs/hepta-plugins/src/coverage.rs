use crate::{GatewayPluginBindingLookupResolution, GatewayPluginBindingTier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginBindingCoverageDigest {
    pub requested_lookup_keys: Vec<String>,
    pub requested_tier_labels: Vec<Option<String>>,
    pub matched_lookup_keys: Vec<String>,
    pub matched_tier_labels: Vec<Option<String>>,
    pub unmatched_lookup_keys: Vec<String>,
    pub unmatched_tier_labels: Vec<Option<String>>,
    pub full_coverage: bool,
}

impl GatewayPluginBindingLookupResolution {
    pub fn requested_lookup_tiers(&self) -> Vec<Option<GatewayPluginBindingTier>> {
        self.requested_lookup_keys
            .iter()
            .map(|lookup_key| GatewayPluginBindingTier::from_lookup_key(lookup_key))
            .collect()
    }

    pub fn unmatched_lookup_tiers(&self) -> Vec<Option<GatewayPluginBindingTier>> {
        self.unmatched_lookup_keys()
            .into_iter()
            .map(GatewayPluginBindingTier::from_lookup_key)
            .collect()
    }

    pub fn has_full_coverage(&self) -> bool {
        self.unmatched_lookup_keys().is_empty()
    }

    pub fn coverage_digest(&self) -> GatewayPluginBindingCoverageDigest {
        let requested_lookup_keys = self.requested_lookup_keys.clone();
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

        GatewayPluginBindingCoverageDigest {
            requested_tier_labels: self
                .requested_lookup_tiers()
                .into_iter()
                .map(|tier| tier.map(|tier| tier.as_str().to_string()))
                .collect(),
            matched_tier_labels: self
                .match_tiers()
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
    use crate::{
        GatewayPluginBinding, GatewayPluginBindingCatalog, GatewayPluginBindingCoverageDigest,
        GatewayPluginBindingTier,
    };

    #[test]
    fn lookup_resolution_can_summarize_partial_lookup_coverage() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(GatewayPluginBinding::new(
            "fallback-plugin",
            "telegram",
            "cli",
            "general cli hooks",
        ));

        let resolution = catalog.resolve_lookup_keys([
            "surface=telegram|transport=cli|command=/status",
            "surface=telegram|transport=cli",
            "surface=telegram",
        ]);

        assert_eq!(
            resolution.requested_lookup_tiers(),
            vec![
                Some(GatewayPluginBindingTier::Command),
                Some(GatewayPluginBindingTier::Transport),
                Some(GatewayPluginBindingTier::Surface),
            ]
        );
        assert_eq!(
            resolution.unmatched_lookup_tiers(),
            vec![
                Some(GatewayPluginBindingTier::Command),
                Some(GatewayPluginBindingTier::Surface),
            ]
        );
        assert!(!resolution.has_full_coverage());

        assert_eq!(
            resolution.coverage_digest(),
            GatewayPluginBindingCoverageDigest {
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
    fn lookup_resolution_can_summarize_full_lookup_coverage() {
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

        let resolution = catalog.resolve_lookup_keys([
            "surface=hepta|transport=webhook|command=/status",
            "surface=hepta|transport=webhook",
            "surface=hepta",
        ]);

        assert!(resolution.has_full_coverage());
        assert!(resolution.unmatched_lookup_tiers().is_empty());
        assert!(resolution.coverage_digest().full_coverage);
    }
}

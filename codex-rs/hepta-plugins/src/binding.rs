const SURFACE_WILDCARD_TRANSPORT_KEY: &str = "*";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayPluginBindingTier {
    Surface,
    Transport,
    Command,
}

impl GatewayPluginBindingTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Surface => "surface",
            Self::Transport => "transport",
            Self::Command => "command",
        }
    }

    pub fn specificity_score(&self) -> usize {
        match self {
            Self::Surface => 0,
            Self::Transport => 1,
            Self::Command => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginBindingMatch {
    pub plugin_id: String,
    pub lookup_key: String,
    pub description: String,
    pub specificity_score: usize,
}

impl GatewayPluginBindingMatch {
    pub fn from_binding(binding: &GatewayPluginBinding) -> Self {
        Self {
            plugin_id: binding.plugin_id.clone(),
            lookup_key: binding.lookup_key(),
            description: binding.description.clone(),
            specificity_score: binding.specificity_score(),
        }
    }

    pub fn match_tier(&self) -> Option<GatewayPluginBindingTier> {
        GatewayPluginBindingTier::from_lookup_key(&self.lookup_key)
            .or_else(|| GatewayPluginBindingTier::from_specificity_score(self.specificity_score))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginBindingResolution {
    pub surface_id: String,
    pub transport_key: String,
    pub command_selector: Option<String>,
    pub matches: Vec<GatewayPluginBindingMatch>,
}

impl GatewayPluginBindingResolution {
    pub fn is_empty(&self) -> bool {
        self.matches.is_empty()
    }

    pub fn match_tiers(&self) -> Vec<Option<GatewayPluginBindingTier>> {
        self.matches
            .iter()
            .map(GatewayPluginBindingMatch::match_tier)
            .collect()
    }

    pub fn plugin_ids(&self) -> Vec<&str> {
        self.matches
            .iter()
            .map(|binding| binding.plugin_id.as_str())
            .collect()
    }

    pub fn lookup_keys(&self) -> Vec<&str> {
        self.matches
            .iter()
            .map(|binding| binding.lookup_key.as_str())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginBindingLookupResolution {
    pub requested_lookup_keys: Vec<String>,
    pub matches: Vec<GatewayPluginBindingMatch>,
}

impl GatewayPluginBindingLookupResolution {
    pub fn is_empty(&self) -> bool {
        self.matches.is_empty()
    }

    pub fn match_tiers(&self) -> Vec<Option<GatewayPluginBindingTier>> {
        self.matches
            .iter()
            .map(GatewayPluginBindingMatch::match_tier)
            .collect()
    }

    pub fn plugin_ids(&self) -> Vec<&str> {
        self.matches
            .iter()
            .map(|binding| binding.plugin_id.as_str())
            .collect()
    }

    pub fn matched_lookup_keys(&self) -> Vec<&str> {
        self.matches
            .iter()
            .map(|binding| binding.lookup_key.as_str())
            .collect()
    }

    pub fn unmatched_lookup_keys(&self) -> Vec<&str> {
        self.requested_lookup_keys
            .iter()
            .filter(|lookup_key| {
                !self
                    .matches
                    .iter()
                    .any(|binding| binding.lookup_key == lookup_key.as_str())
            })
            .map(String::as_str)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginBinding {
    pub plugin_id: String,
    pub surface_id: String,
    pub transport_key: String,
    pub command_selector: Option<String>,
    pub description: String,
}

impl GatewayPluginBinding {
    pub fn new(
        plugin_id: impl Into<String>,
        surface_id: impl Into<String>,
        transport_key: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            plugin_id: normalize_required(plugin_id.into()),
            surface_id: normalize_required(surface_id.into()),
            transport_key: normalize_required(transport_key.into()),
            command_selector: None,
            description: description.into().trim().to_string(),
        }
    }

    pub fn for_surface(
        plugin_id: impl Into<String>,
        surface_id: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            plugin_id: normalize_required(plugin_id.into()),
            surface_id: normalize_required(surface_id.into()),
            transport_key: SURFACE_WILDCARD_TRANSPORT_KEY.to_string(),
            command_selector: None,
            description: description.into().trim().to_string(),
        }
    }

    pub fn with_command_selector(mut self, command_selector: impl Into<String>) -> Self {
        if self.is_surface_wildcard() {
            return self;
        }

        let normalized = normalize_optional(command_selector.into());
        if normalized.is_some() {
            self.command_selector = normalized;
        }
        self
    }

    pub fn lookup_key(&self) -> String {
        if self.is_surface_wildcard() {
            return format!("surface={}", self.surface_id);
        }

        match self.command_selector.as_deref() {
            Some(command_selector) => format!(
                "surface={}|transport={}|command={}",
                self.surface_id, self.transport_key, command_selector
            ),
            None => format!(
                "surface={}|transport={}",
                self.surface_id, self.transport_key
            ),
        }
    }

    pub fn matches_route(
        &self,
        surface_id: &str,
        transport_key: &str,
        command_selector: Option<&str>,
    ) -> bool {
        let surface_id = normalize_required(surface_id.to_string());
        if self.surface_id != surface_id {
            return false;
        }

        if self.is_surface_wildcard() {
            return true;
        }

        if self.transport_key != normalize_required(transport_key.to_string()) {
            return false;
        }
        match self.command_selector.as_deref() {
            Some(binding_selector) => {
                command_selector
                    .map(|value| normalize_required(value.to_string()))
                    .as_deref()
                    == Some(binding_selector)
            }
            None => true,
        }
    }

    pub fn specificity_score(&self) -> usize {
        self.match_tier().specificity_score()
    }

    pub fn is_surface_wildcard(&self) -> bool {
        self.transport_key == SURFACE_WILDCARD_TRANSPORT_KEY && self.command_selector.is_none()
    }

    pub fn match_tier(&self) -> GatewayPluginBindingTier {
        if self.command_selector.is_some() {
            GatewayPluginBindingTier::Command
        } else if self.is_surface_wildcard() {
            GatewayPluginBindingTier::Surface
        } else {
            GatewayPluginBindingTier::Transport
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GatewayPluginBindingCatalog {
    bindings: Vec<GatewayPluginBinding>,
}

impl GatewayPluginBindingCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, binding: GatewayPluginBinding) {
        if let Some(existing) = self.bindings.iter_mut().find(|existing| {
            existing.plugin_id == binding.plugin_id
                && existing.surface_id == binding.surface_id
                && existing.transport_key == binding.transport_key
                && existing.command_selector == binding.command_selector
        }) {
            *existing = binding;
            return;
        }

        self.bindings.push(binding);
        self.bindings.sort_by(|left, right| {
            left.plugin_id
                .cmp(&right.plugin_id)
                .then_with(|| left.surface_id.cmp(&right.surface_id))
                .then_with(|| left.transport_key.cmp(&right.transport_key))
                .then_with(|| left.command_selector.cmp(&right.command_selector))
        });
    }

    pub fn bindings(&self) -> &[GatewayPluginBinding] {
        &self.bindings
    }

    pub fn bindings_for_route(
        &self,
        surface_id: &str,
        transport_key: &str,
        command_selector: Option<&str>,
    ) -> Vec<GatewayPluginBinding> {
        let mut matches = self
            .bindings
            .iter()
            .filter(|binding| binding.matches_route(surface_id, transport_key, command_selector))
            .cloned()
            .collect::<Vec<_>>();
        matches.sort_by(|left, right| {
            right
                .specificity_score()
                .cmp(&left.specificity_score())
                .then_with(|| left.plugin_id.cmp(&right.plugin_id))
        });
        matches
    }

    pub fn resolve_route(
        &self,
        surface_id: &str,
        transport_key: &str,
        command_selector: Option<&str>,
    ) -> GatewayPluginBindingResolution {
        let surface_id = normalize_required(surface_id.to_string());
        let transport_key = normalize_required(transport_key.to_string());
        let command_selector = command_selector
            .map(|value| normalize_required(value.to_string()))
            .filter(|value| !value.is_empty());

        let matches = self
            .bindings_for_route(&surface_id, &transport_key, command_selector.as_deref())
            .iter()
            .map(GatewayPluginBindingMatch::from_binding)
            .collect();

        GatewayPluginBindingResolution {
            surface_id,
            transport_key,
            command_selector,
            matches,
        }
    }

    pub fn resolve_lookup_keys<I, S>(&self, lookup_keys: I) -> GatewayPluginBindingLookupResolution
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let requested_lookup_keys = lookup_keys
            .into_iter()
            .filter_map(|lookup_key| normalize_lookup_key(lookup_key.as_ref()))
            .fold(Vec::new(), |mut keys, lookup_key| {
                if !keys.contains(&lookup_key) {
                    keys.push(lookup_key);
                }
                keys
            });

        let mut matches = self
            .bindings
            .iter()
            .filter_map(|binding| {
                let lookup_key = binding.lookup_key();
                let lookup_index = requested_lookup_keys
                    .iter()
                    .position(|requested_lookup_key| requested_lookup_key == &lookup_key)?;
                Some((
                    lookup_index,
                    GatewayPluginBindingMatch::from_binding(binding),
                ))
            })
            .collect::<Vec<_>>();

        matches.sort_by(|(left_index, left_match), (right_index, right_match)| {
            left_index
                .cmp(right_index)
                .then_with(|| left_match.plugin_id.cmp(&right_match.plugin_id))
        });

        GatewayPluginBindingLookupResolution {
            requested_lookup_keys,
            matches: matches.into_iter().map(|(_, binding)| binding).collect(),
        }
    }
}

fn normalize_required(value: String) -> String {
    value.trim().to_ascii_lowercase()
}

fn normalize_optional(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_ascii_lowercase())
    }
}

fn normalize_lookup_key(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_ascii_lowercase())
    }
}

impl GatewayPluginBindingTier {
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

#[cfg(test)]
mod tests {
    use super::GatewayPluginBinding;
    use super::GatewayPluginBindingCatalog;
    use super::GatewayPluginBindingLookupResolution;
    use super::GatewayPluginBindingMatch;
    use super::GatewayPluginBindingTier;

    #[test]
    fn binding_match_captures_lookup_snapshot_fields() {
        let binding =
            GatewayPluginBinding::new("status-plugin", " Hepta ", " WebHook ", "status commands")
                .with_command_selector(" /Status ");

        let snapshot = GatewayPluginBindingMatch::from_binding(&binding);

        assert_eq!(snapshot.plugin_id, "status-plugin");
        assert_eq!(
            snapshot.lookup_key,
            "surface=hepta|transport=webhook|command=/status"
        );
        assert_eq!(snapshot.description, "status commands");
        assert_eq!(snapshot.specificity_score, 2);
        assert_eq!(
            snapshot.match_tier(),
            Some(GatewayPluginBindingTier::Command)
        );
    }

    #[test]
    fn binding_normalizes_surface_transport_and_selector() {
        let binding =
            GatewayPluginBinding::new("status-plugin", " Hepta ", " WebHook ", "status commands")
                .with_command_selector(" /Status ");

        assert_eq!(binding.plugin_id, "status-plugin");
        assert_eq!(binding.surface_id, "hepta");
        assert_eq!(binding.transport_key, "webhook");
        assert_eq!(binding.command_selector.as_deref(), Some("/status"));
        assert_eq!(binding.match_tier(), GatewayPluginBindingTier::Command);
    }

    #[test]
    fn binding_lookup_key_uses_gateway_contract_shape() {
        let specific =
            GatewayPluginBinding::new("status-plugin", " Hepta ", " WebHook ", "status commands")
                .with_command_selector(" /Status ");
        let general = GatewayPluginBinding::new(
            "fallback-plugin",
            " Hepta ",
            " WebHook ",
            "fallback commands",
        );
        let surface =
            GatewayPluginBinding::for_surface("surface-plugin", " Hepta ", "surface-wide fallback");

        assert_eq!(
            specific.lookup_key(),
            "surface=hepta|transport=webhook|command=/status"
        );
        assert_eq!(general.lookup_key(), "surface=hepta|transport=webhook");
        assert_eq!(surface.lookup_key(), "surface=hepta");
        assert_eq!(specific.match_tier(), GatewayPluginBindingTier::Command);
        assert_eq!(general.match_tier(), GatewayPluginBindingTier::Transport);
        assert_eq!(surface.match_tier(), GatewayPluginBindingTier::Surface);
    }

    #[test]
    fn surface_wildcard_binding_ignores_command_specificity_requests() {
        let binding =
            GatewayPluginBinding::for_surface("surface-plugin", "hepta", "surface-wide fallback")
                .with_command_selector("/status");

        assert!(binding.is_surface_wildcard());
        assert_eq!(binding.command_selector, None);
        assert_eq!(binding.lookup_key(), "surface=hepta");
    }

    #[test]
    fn catalog_replaces_duplicate_binding_shape() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(
            GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "first")
                .with_command_selector("/status"),
        );
        catalog.register(
            GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "updated")
                .with_command_selector("/status"),
        );

        assert_eq!(catalog.bindings().len(), 1);
        assert_eq!(catalog.bindings()[0].description, "updated");
    }

    #[test]
    fn route_lookup_prefers_command_specific_bindings_then_general_ones() {
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
            GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status commands")
                .with_command_selector("/status"),
        );
        catalog.register(
            GatewayPluginBinding::new("help-plugin", "hepta", "webhook", "help commands")
                .with_command_selector("/help"),
        );

        let matches = catalog.bindings_for_route("Hepta", "WEBHOOK", Some("/STATUS"));

        assert_eq!(
            matches
                .iter()
                .map(|binding| binding.plugin_id.as_str())
                .collect::<Vec<_>>(),
            vec!["status-plugin", "fallback-plugin", "surface-plugin"]
        );
    }

    #[test]
    fn route_lookup_uses_general_binding_when_no_command_selector_exists() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(GatewayPluginBinding::for_surface(
            "surface-plugin",
            "hepta",
            "surface-wide hooks",
        ));
        catalog.register(GatewayPluginBinding::new(
            "fallback-plugin",
            "hepta",
            "cli",
            "general cli hooks",
        ));
        catalog.register(
            GatewayPluginBinding::new("status-plugin", "hepta", "cli", "status commands")
                .with_command_selector("/status"),
        );

        let matches = catalog.bindings_for_route("hepta", "cli", None);

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].plugin_id, "fallback-plugin");
        assert_eq!(matches[1].plugin_id, "surface-plugin");
    }

    #[test]
    fn route_lookup_can_fall_back_to_surface_only_binding() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(GatewayPluginBinding::for_surface(
            "surface-plugin",
            "hepta",
            "surface-wide hooks",
        ));
        catalog.register(GatewayPluginBinding::new(
            "telegram-plugin",
            "telegram",
            "cli",
            "telegram hooks",
        ));

        let matches = catalog.bindings_for_route("hepta", "queue", Some("/status"));

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].plugin_id, "surface-plugin");
        assert_eq!(matches[0].lookup_key(), "surface=hepta");
    }

    #[test]
    fn resolve_route_returns_ordered_match_snapshot() {
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

        let resolution = catalog.resolve_route(" Hepta ", " WebHook ", Some(" /STATUS "));

        assert_eq!(resolution.surface_id, "hepta");
        assert_eq!(resolution.transport_key, "webhook");
        assert_eq!(resolution.command_selector.as_deref(), Some("/status"));
        assert_eq!(
            resolution.plugin_ids(),
            vec!["status-plugin", "fallback-plugin", "surface-plugin"]
        );
        assert_eq!(
            resolution.match_tiers(),
            vec![
                Some(GatewayPluginBindingTier::Command),
                Some(GatewayPluginBindingTier::Transport),
                Some(GatewayPluginBindingTier::Surface),
            ]
        );
        assert_eq!(
            resolution.lookup_keys(),
            vec![
                "surface=hepta|transport=webhook|command=/status",
                "surface=hepta|transport=webhook",
                "surface=hepta",
            ]
        );
        assert!(!resolution.is_empty());
    }

    #[test]
    fn resolve_route_preserves_empty_snapshot_shape_when_nothing_matches() {
        let catalog = GatewayPluginBindingCatalog::new();

        let resolution = catalog.resolve_route(" Telegram ", " CLI ", None);

        assert_eq!(resolution.surface_id, "telegram");
        assert_eq!(resolution.transport_key, "cli");
        assert_eq!(resolution.command_selector, None);
        assert!(resolution.is_empty());
        assert!(resolution.plugin_ids().is_empty());
        assert!(resolution.lookup_keys().is_empty());
        assert!(resolution.match_tiers().is_empty());
    }

    #[test]
    fn resolve_lookup_keys_preserves_requested_order_and_gap_diagnostics() {
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
            " surface=Hepta|transport=WEBHOOK|command=/STATUS ",
            "surface=hepta|transport=webhook",
            "surface=hepta",
        ]);

        assert_eq!(
            resolution.plugin_ids(),
            vec!["status-plugin", "fallback-plugin", "surface-plugin"]
        );
        assert_eq!(
            resolution.matched_lookup_keys(),
            vec![
                "surface=hepta|transport=webhook|command=/status",
                "surface=hepta|transport=webhook",
                "surface=hepta",
            ]
        );
        assert!(resolution.unmatched_lookup_keys().is_empty());
        assert_eq!(
            resolution.match_tiers(),
            vec![
                Some(GatewayPluginBindingTier::Command),
                Some(GatewayPluginBindingTier::Transport),
                Some(GatewayPluginBindingTier::Surface),
            ]
        );
    }

    #[test]
    fn resolve_lookup_keys_can_report_unmatched_requested_keys() {
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

        assert_eq!(resolution.plugin_ids(), vec!["fallback-plugin"]);
        assert_eq!(
            resolution.matched_lookup_keys(),
            vec!["surface=telegram|transport=cli"]
        );
        assert_eq!(
            resolution.unmatched_lookup_keys(),
            vec![
                "surface=telegram|transport=cli|command=/status",
                "surface=telegram",
            ]
        );
        assert!(!resolution.is_empty());
    }

    #[test]
    fn binding_tier_can_be_derived_from_lookup_key_or_score() {
        assert_eq!(
            GatewayPluginBindingTier::from_lookup_key(
                "surface=hepta|transport=webhook|command=/status"
            ),
            Some(GatewayPluginBindingTier::Command)
        );
        assert_eq!(
            GatewayPluginBindingTier::from_lookup_key("surface=hepta|transport=webhook"),
            Some(GatewayPluginBindingTier::Transport)
        );
        assert_eq!(
            GatewayPluginBindingTier::from_lookup_key("surface=hepta"),
            Some(GatewayPluginBindingTier::Surface)
        );
        assert_eq!(
            GatewayPluginBindingTier::from_specificity_score(2),
            Some(GatewayPluginBindingTier::Command)
        );
        assert_eq!(GatewayPluginBindingTier::Command.as_str(), "command");
    }

    #[test]
    fn lookup_resolution_can_stay_empty_after_normalization() {
        let catalog = GatewayPluginBindingCatalog::new();

        let resolution = catalog.resolve_lookup_keys(["   ", "\n\t"]);

        assert_eq!(
            resolution,
            GatewayPluginBindingLookupResolution {
                requested_lookup_keys: Vec::new(),
                matches: Vec::new(),
            }
        );
        assert!(resolution.is_empty());
    }
}

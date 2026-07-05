use crate::GatewayPluginBindingLookupResolution;
use crate::GatewayPluginBindingTier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginBindingScaffoldPlan {
    pub requested_lookup_keys: Vec<String>,
    pub requested_tier_labels: Vec<Option<String>>,
    pub unmatched_lookup_keys: Vec<String>,
    pub unmatched_tier_labels: Vec<Option<String>>,
    pub binding_scaffold_notes: Vec<GatewayPluginBindingScaffoldNote>,
    pub plugin_scaffold_stubs: Vec<GatewayPluginScaffoldStub>,
    pub scaffolding_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginBindingScaffoldNote {
    pub lookup_key: String,
    pub tier_label: Option<String>,
    pub binding_kind: Option<String>,
    pub surface_id: Option<String>,
    pub transport_key: Option<String>,
    pub command_selector: Option<String>,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginScaffoldStub {
    pub lookup_key: String,
    pub tier_label: Option<String>,
    pub binding_kind: Option<String>,
    pub plugin_id_hint: String,
    pub module_stub_name: String,
    pub description_hint: String,
    pub surface_id: Option<String>,
    pub transport_key: Option<String>,
    pub command_selector: Option<String>,
}

impl GatewayPluginBindingLookupResolution {
    pub fn scaffold_plan(&self) -> GatewayPluginBindingScaffoldPlan {
        let coverage = self.coverage_digest();
        let binding_scaffold_notes = self.binding_scaffold_notes();
        let plugin_scaffold_stubs = binding_scaffold_notes
            .iter()
            .map(GatewayPluginBindingScaffoldNote::plugin_scaffold_stub)
            .collect::<Vec<_>>();

        GatewayPluginBindingScaffoldPlan {
            requested_lookup_keys: coverage.requested_lookup_keys,
            requested_tier_labels: coverage.requested_tier_labels,
            unmatched_lookup_keys: coverage.unmatched_lookup_keys,
            unmatched_tier_labels: coverage.unmatched_tier_labels,
            scaffolding_required: !binding_scaffold_notes.is_empty(),
            binding_scaffold_notes,
            plugin_scaffold_stubs,
        }
    }

    pub fn binding_scaffold_notes(&self) -> Vec<GatewayPluginBindingScaffoldNote> {
        self.unmatched_lookup_keys()
            .into_iter()
            .map(GatewayPluginBindingScaffoldNote::from_lookup_key)
            .collect()
    }

    pub fn plugin_scaffold_stubs(&self) -> Vec<GatewayPluginScaffoldStub> {
        self.binding_scaffold_notes()
            .into_iter()
            .map(|note| note.plugin_scaffold_stub())
            .collect()
    }
}

impl GatewayPluginBindingScaffoldNote {
    pub fn from_lookup_key(lookup_key: &str) -> Self {
        let parsed = ParsedGatewayLookupKey::parse(lookup_key);
        let tier = GatewayPluginBindingTier::from_lookup_key(lookup_key);

        Self {
            lookup_key: lookup_key.to_string(),
            tier_label: tier.map(|tier| tier.as_str().to_string()),
            binding_kind: tier.map(binding_kind_label),
            surface_id: parsed.surface_id.clone(),
            transport_key: parsed.transport_key.clone(),
            command_selector: parsed.command_selector.clone(),
            note: scaffold_note(tier, &parsed, lookup_key),
        }
    }

    pub fn plugin_scaffold_stub(&self) -> GatewayPluginScaffoldStub {
        let tier = self.tier_label.as_deref().and_then(tier_from_label);

        let plugin_id_hint = plugin_id_hint(tier, self);

        GatewayPluginScaffoldStub {
            lookup_key: self.lookup_key.clone(),
            tier_label: self.tier_label.clone(),
            binding_kind: self.binding_kind.clone(),
            module_stub_name: module_stub_name(&plugin_id_hint),
            description_hint: description_hint(tier, self),
            plugin_id_hint,
            surface_id: self.surface_id.clone(),
            transport_key: self.transport_key.clone(),
            command_selector: self.command_selector.clone(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ParsedGatewayLookupKey {
    surface_id: Option<String>,
    transport_key: Option<String>,
    command_selector: Option<String>,
}

impl ParsedGatewayLookupKey {
    fn parse(lookup_key: &str) -> Self {
        let mut parsed = Self::default();

        for segment in lookup_key.split('|').map(str::trim) {
            if let Some(surface_id) = segment.strip_prefix("surface=") {
                parsed.surface_id = normalized_field(surface_id);
            } else if let Some(transport_key) = segment.strip_prefix("transport=") {
                parsed.transport_key = normalized_field(transport_key);
            } else if let Some(command_selector) = segment.strip_prefix("command=") {
                parsed.command_selector = normalized_field(command_selector);
            }
        }

        parsed
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

fn binding_kind_label(tier: GatewayPluginBindingTier) -> String {
    match tier {
        GatewayPluginBindingTier::Command => "command_binding".to_string(),
        GatewayPluginBindingTier::Transport => "transport_binding".to_string(),
        GatewayPluginBindingTier::Surface => "surface_binding".to_string(),
    }
}

fn scaffold_note(
    tier: Option<GatewayPluginBindingTier>,
    parsed: &ParsedGatewayLookupKey,
    lookup_key: &str,
) -> String {
    match tier {
        Some(GatewayPluginBindingTier::Command) => match (
            parsed.surface_id.as_deref(),
            parsed.transport_key.as_deref(),
            parsed.command_selector.as_deref(),
        ) {
            (Some(surface_id), Some(transport_key), Some(command_selector)) => format!(
                "scaffold a command binding for surface={surface_id} transport={transport_key} command={command_selector}"
            ),
            _ => format!("scaffold a command binding for {lookup_key}"),
        },
        Some(GatewayPluginBindingTier::Transport) => {
            match (
                parsed.surface_id.as_deref(),
                parsed.transport_key.as_deref(),
            ) {
                (Some(surface_id), Some(transport_key)) => format!(
                    "scaffold a transport fallback binding for surface={surface_id} transport={transport_key}"
                ),
                _ => format!("scaffold a transport fallback binding for {lookup_key}"),
            }
        }
        Some(GatewayPluginBindingTier::Surface) => match parsed.surface_id.as_deref() {
            Some(surface_id) => {
                format!("scaffold a surface fallback binding for surface={surface_id}")
            }
            None => format!("scaffold a surface fallback binding for {lookup_key}"),
        },
        None => format!("scaffold an explicit binding for {lookup_key}"),
    }
}

fn tier_from_label(label: &str) -> Option<GatewayPluginBindingTier> {
    match label {
        "command" => Some(GatewayPluginBindingTier::Command),
        "transport" => Some(GatewayPluginBindingTier::Transport),
        "surface" => Some(GatewayPluginBindingTier::Surface),
        _ => None,
    }
}

fn plugin_id_hint(
    tier: Option<GatewayPluginBindingTier>,
    note: &GatewayPluginBindingScaffoldNote,
) -> String {
    match tier {
        Some(GatewayPluginBindingTier::Command) => match (
            note.surface_id.as_deref(),
            note.transport_key.as_deref(),
            note.command_selector.as_deref(),
        ) {
            (Some(surface_id), Some(transport_key), Some(command_selector)) => format!(
                "{}-{}-{}-plugin",
                slugify(surface_id),
                slugify(transport_key),
                slugify(command_selector)
            ),
            _ => fallback_plugin_id_hint(&note.lookup_key),
        },
        Some(GatewayPluginBindingTier::Transport) => {
            match (note.surface_id.as_deref(), note.transport_key.as_deref()) {
                (Some(surface_id), Some(transport_key)) => format!(
                    "{}-{}-fallback-plugin",
                    slugify(surface_id),
                    slugify(transport_key)
                ),
                _ => fallback_plugin_id_hint(&note.lookup_key),
            }
        }
        Some(GatewayPluginBindingTier::Surface) => match note.surface_id.as_deref() {
            Some(surface_id) => format!("{}-surface-plugin", slugify(surface_id)),
            None => fallback_plugin_id_hint(&note.lookup_key),
        },
        None => fallback_plugin_id_hint(&note.lookup_key),
    }
}

fn fallback_plugin_id_hint(lookup_key: &str) -> String {
    format!("{}-plugin", slugify(lookup_key))
}

fn module_stub_name(plugin_id_hint: &str) -> String {
    plugin_id_hint.replace('-', "_")
}

fn description_hint(
    tier: Option<GatewayPluginBindingTier>,
    note: &GatewayPluginBindingScaffoldNote,
) -> String {
    match tier {
        Some(GatewayPluginBindingTier::Command) => match (
            note.surface_id.as_deref(),
            note.transport_key.as_deref(),
            note.command_selector.as_deref(),
        ) {
            (Some(surface_id), Some(transport_key), Some(command_selector)) => format!(
                "scaffolded command binding for surface={surface_id} transport={transport_key} command={command_selector}"
            ),
            _ => format!("scaffolded command binding for {}", note.lookup_key),
        },
        Some(GatewayPluginBindingTier::Transport) => {
            match (note.surface_id.as_deref(), note.transport_key.as_deref()) {
                (Some(surface_id), Some(transport_key)) => format!(
                    "scaffolded transport fallback for surface={surface_id} transport={transport_key}"
                ),
                _ => format!("scaffolded transport fallback for {}", note.lookup_key),
            }
        }
        Some(GatewayPluginBindingTier::Surface) => match note.surface_id.as_deref() {
            Some(surface_id) => format!("scaffolded surface fallback for surface={surface_id}"),
            None => format!("scaffolded surface fallback for {}", note.lookup_key),
        },
        None => format!("scaffolded explicit binding for {}", note.lookup_key),
    }
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut previous_was_separator = false;

    for character in value.trim().chars() {
        let lower = character.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            slug.push(lower);
            previous_was_separator = false;
        } else if !previous_was_separator {
            slug.push('-');
            previous_was_separator = true;
        }
    }

    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        "binding".to_string()
    } else {
        slug
    }
}

#[cfg(test)]
mod tests {
    use crate::GatewayPluginBinding;
    use crate::GatewayPluginBindingCatalog;
    use crate::GatewayPluginBindingScaffoldNote;
    use crate::GatewayPluginBindingScaffoldPlan;
    use crate::GatewayPluginScaffoldStub;

    #[test]
    fn lookup_resolution_can_emit_binding_scaffold_notes_for_remaining_gaps() {
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
            .binding_scaffold_notes();

        assert_eq!(
            notes,
            vec![
                GatewayPluginBindingScaffoldNote {
                    lookup_key: "surface=telegram|transport=cli|command=/status".to_string(),
                    tier_label: Some("command".to_string()),
                    binding_kind: Some("command_binding".to_string()),
                    surface_id: Some("telegram".to_string()),
                    transport_key: Some("cli".to_string()),
                    command_selector: Some("/status".to_string()),
                    note: "scaffold a command binding for surface=telegram transport=cli command=/status"
                        .to_string(),
                },
                GatewayPluginBindingScaffoldNote {
                    lookup_key: "surface=telegram".to_string(),
                    tier_label: Some("surface".to_string()),
                    binding_kind: Some("surface_binding".to_string()),
                    surface_id: Some("telegram".to_string()),
                    transport_key: None,
                    command_selector: None,
                    note: "scaffold a surface fallback binding for surface=telegram".to_string(),
                },
            ]
        );
    }

    #[test]
    fn scaffold_note_can_be_derived_from_a_single_lookup_key() {
        let note = GatewayPluginBindingScaffoldNote::from_lookup_key(
            "surface=hepta|transport=webhook|command=/status",
        );

        assert_eq!(note.tier_label.as_deref(), Some("command"));
        assert_eq!(note.binding_kind.as_deref(), Some("command_binding"));
        assert_eq!(note.surface_id.as_deref(), Some("hepta"));
        assert_eq!(note.transport_key.as_deref(), Some("webhook"));
        assert_eq!(note.command_selector.as_deref(), Some("/status"));
        assert_eq!(
            note.note,
            "scaffold a command binding for surface=hepta transport=webhook command=/status"
        );
    }

    #[test]
    fn lookup_resolution_can_emit_plugin_scaffold_stubs_for_remaining_gaps() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(GatewayPluginBinding::new(
            "fallback-plugin",
            "telegram",
            "cli",
            "general cli hooks",
        ));

        let stubs = catalog
            .resolve_lookup_keys([
                "surface=telegram|transport=cli|command=/status",
                "surface=telegram|transport=cli",
                "surface=telegram",
            ])
            .plugin_scaffold_stubs();

        assert_eq!(
            stubs,
            vec![
                GatewayPluginScaffoldStub {
                    lookup_key: "surface=telegram|transport=cli|command=/status".to_string(),
                    tier_label: Some("command".to_string()),
                    binding_kind: Some("command_binding".to_string()),
                    plugin_id_hint: "telegram-cli-status-plugin".to_string(),
                    module_stub_name: "telegram_cli_status_plugin".to_string(),
                    description_hint:
                        "scaffolded command binding for surface=telegram transport=cli command=/status"
                            .to_string(),
                    surface_id: Some("telegram".to_string()),
                    transport_key: Some("cli".to_string()),
                    command_selector: Some("/status".to_string()),
                },
                GatewayPluginScaffoldStub {
                    lookup_key: "surface=telegram".to_string(),
                    tier_label: Some("surface".to_string()),
                    binding_kind: Some("surface_binding".to_string()),
                    plugin_id_hint: "telegram-surface-plugin".to_string(),
                    module_stub_name: "telegram_surface_plugin".to_string(),
                    description_hint: "scaffolded surface fallback for surface=telegram"
                        .to_string(),
                    surface_id: Some("telegram".to_string()),
                    transport_key: None,
                    command_selector: None,
                },
            ]
        );
    }

    #[test]
    fn scaffold_note_can_generate_stable_plugin_stub_hints() {
        let stub = GatewayPluginBindingScaffoldNote::from_lookup_key(
            "surface=hepta|transport=webhook|command=/deploy:prod",
        )
        .plugin_scaffold_stub();

        assert_eq!(stub.plugin_id_hint, "hepta-webhook-deploy-prod-plugin");
        assert_eq!(stub.module_stub_name, "hepta_webhook_deploy_prod_plugin");
        assert_eq!(
            stub.description_hint,
            "scaffolded command binding for surface=hepta transport=webhook command=/deploy:prod"
        );
    }

    #[test]
    fn lookup_resolution_can_bundle_scaffolding_plan_artifacts() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(GatewayPluginBinding::new(
            "fallback-plugin",
            "telegram",
            "cli",
            "general cli hooks",
        ));

        let plan = catalog
            .resolve_lookup_keys([
                "surface=telegram|transport=cli|command=/status",
                "surface=telegram|transport=cli",
                "surface=telegram",
            ])
            .scaffold_plan();

        assert_eq!(
            plan,
            GatewayPluginBindingScaffoldPlan {
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
                unmatched_lookup_keys: vec![
                    "surface=telegram|transport=cli|command=/status".to_string(),
                    "surface=telegram".to_string(),
                ],
                unmatched_tier_labels: vec![
                    Some("command".to_string()),
                    Some("surface".to_string()),
                ],
                binding_scaffold_notes: vec![
                    GatewayPluginBindingScaffoldNote {
                        lookup_key: "surface=telegram|transport=cli|command=/status"
                            .to_string(),
                        tier_label: Some("command".to_string()),
                        binding_kind: Some("command_binding".to_string()),
                        surface_id: Some("telegram".to_string()),
                        transport_key: Some("cli".to_string()),
                        command_selector: Some("/status".to_string()),
                        note: "scaffold a command binding for surface=telegram transport=cli command=/status".to_string(),
                    },
                    GatewayPluginBindingScaffoldNote {
                        lookup_key: "surface=telegram".to_string(),
                        tier_label: Some("surface".to_string()),
                        binding_kind: Some("surface_binding".to_string()),
                        surface_id: Some("telegram".to_string()),
                        transport_key: None,
                        command_selector: None,
                        note: "scaffold a surface fallback binding for surface=telegram"
                            .to_string(),
                    },
                ],
                plugin_scaffold_stubs: vec![
                    GatewayPluginScaffoldStub {
                        lookup_key: "surface=telegram|transport=cli|command=/status"
                            .to_string(),
                        tier_label: Some("command".to_string()),
                        binding_kind: Some("command_binding".to_string()),
                        plugin_id_hint: "telegram-cli-status-plugin".to_string(),
                        module_stub_name: "telegram_cli_status_plugin".to_string(),
                        description_hint:
                            "scaffolded command binding for surface=telegram transport=cli command=/status"
                                .to_string(),
                        surface_id: Some("telegram".to_string()),
                        transport_key: Some("cli".to_string()),
                        command_selector: Some("/status".to_string()),
                    },
                    GatewayPluginScaffoldStub {
                        lookup_key: "surface=telegram".to_string(),
                        tier_label: Some("surface".to_string()),
                        binding_kind: Some("surface_binding".to_string()),
                        plugin_id_hint: "telegram-surface-plugin".to_string(),
                        module_stub_name: "telegram_surface_plugin".to_string(),
                        description_hint:
                            "scaffolded surface fallback for surface=telegram".to_string(),
                        surface_id: Some("telegram".to_string()),
                        transport_key: None,
                        command_selector: None,
                    },
                ],
                scaffolding_required: true,
            }
        );
    }

    #[test]
    fn lookup_resolution_scaffolding_plan_can_stay_empty_when_lookup_is_fully_covered() {
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

        let plan = catalog
            .resolve_lookup_keys([
                "surface=hepta|transport=webhook|command=/status",
                "surface=hepta|transport=webhook",
                "surface=hepta",
            ])
            .scaffold_plan();

        assert!(!plan.scaffolding_required);
        assert!(plan.unmatched_lookup_keys.is_empty());
        assert!(plan.unmatched_tier_labels.is_empty());
        assert!(plan.binding_scaffold_notes.is_empty());
        assert!(plan.plugin_scaffold_stubs.is_empty());
    }
}

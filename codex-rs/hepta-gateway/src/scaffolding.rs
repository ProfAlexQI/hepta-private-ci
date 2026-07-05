use crate::GatewayPluginResolutionSnapshot;
use crate::GatewayResolvedPluginTier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayResolutionScaffoldPlan {
    pub requested_lookup_keys: Vec<String>,
    pub requested_tier_labels: Vec<Option<String>>,
    pub unmatched_lookup_keys: Vec<String>,
    pub unmatched_tier_labels: Vec<Option<String>>,
    pub binding_scaffold_notes: Vec<GatewayResolutionBindingScaffoldNote>,
    pub plugin_scaffold_stubs: Vec<GatewayResolutionPluginScaffoldStub>,
    pub scaffolding_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayResolutionBindingScaffoldNote {
    pub lookup_key: String,
    pub tier_label: Option<String>,
    pub binding_kind: Option<String>,
    pub surface_id: Option<String>,
    pub transport_key: Option<String>,
    pub command_selector: Option<String>,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayResolutionPluginScaffoldStub {
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

impl GatewayPluginResolutionSnapshot {
    pub fn scaffold_plan(&self) -> GatewayResolutionScaffoldPlan {
        let coverage = self.coverage_digest();
        let binding_scaffold_notes = self.binding_scaffold_notes();
        let plugin_scaffold_stubs = binding_scaffold_notes
            .iter()
            .map(GatewayResolutionBindingScaffoldNote::plugin_scaffold_stub)
            .collect::<Vec<_>>();

        GatewayResolutionScaffoldPlan {
            requested_lookup_keys: coverage.requested_lookup_keys,
            requested_tier_labels: coverage.requested_tier_labels,
            unmatched_lookup_keys: coverage.unmatched_lookup_keys,
            unmatched_tier_labels: coverage.unmatched_tier_labels,
            scaffolding_required: !binding_scaffold_notes.is_empty(),
            binding_scaffold_notes,
            plugin_scaffold_stubs,
        }
    }

    pub fn binding_scaffold_notes(&self) -> Vec<GatewayResolutionBindingScaffoldNote> {
        self.unmatched_lookup_keys()
            .into_iter()
            .map(GatewayResolutionBindingScaffoldNote::from_lookup_key)
            .collect()
    }

    pub fn plugin_scaffold_stubs(&self) -> Vec<GatewayResolutionPluginScaffoldStub> {
        self.binding_scaffold_notes()
            .into_iter()
            .map(|note| note.plugin_scaffold_stub())
            .collect()
    }
}

impl GatewayResolutionBindingScaffoldNote {
    pub fn from_lookup_key(lookup_key: &str) -> Self {
        let parsed = ParsedGatewayLookupKey::parse(lookup_key);
        let tier = GatewayResolvedPluginTier::from_lookup_key(lookup_key);

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

    pub fn plugin_scaffold_stub(&self) -> GatewayResolutionPluginScaffoldStub {
        let tier = self.tier_label.as_deref().and_then(tier_from_label);

        let plugin_id_hint = plugin_id_hint(tier, self);

        GatewayResolutionPluginScaffoldStub {
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

fn binding_kind_label(tier: GatewayResolvedPluginTier) -> String {
    match tier {
        GatewayResolvedPluginTier::Command => "command_binding".to_string(),
        GatewayResolvedPluginTier::Transport => "transport_binding".to_string(),
        GatewayResolvedPluginTier::Surface => "surface_binding".to_string(),
    }
}

fn scaffold_note(
    tier: Option<GatewayResolvedPluginTier>,
    parsed: &ParsedGatewayLookupKey,
    lookup_key: &str,
) -> String {
    match tier {
        Some(GatewayResolvedPluginTier::Command) => match (
            parsed.surface_id.as_deref(),
            parsed.transport_key.as_deref(),
            parsed.command_selector.as_deref(),
        ) {
            (Some(surface_id), Some(transport_key), Some(command_selector)) => format!(
                "scaffold a command binding for surface={surface_id} transport={transport_key} command={command_selector}"
            ),
            _ => format!("scaffold a command binding for {lookup_key}"),
        },
        Some(GatewayResolvedPluginTier::Transport) => {
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
        Some(GatewayResolvedPluginTier::Surface) => match parsed.surface_id.as_deref() {
            Some(surface_id) => {
                format!("scaffold a surface fallback binding for surface={surface_id}")
            }
            None => format!("scaffold a surface fallback binding for {lookup_key}"),
        },
        None => format!("scaffold an explicit binding for {lookup_key}"),
    }
}

fn tier_from_label(label: &str) -> Option<GatewayResolvedPluginTier> {
    match label {
        "command" => Some(GatewayResolvedPluginTier::Command),
        "transport" => Some(GatewayResolvedPluginTier::Transport),
        "surface" => Some(GatewayResolvedPluginTier::Surface),
        _ => None,
    }
}

fn plugin_id_hint(
    tier: Option<GatewayResolvedPluginTier>,
    note: &GatewayResolutionBindingScaffoldNote,
) -> String {
    match tier {
        Some(GatewayResolvedPluginTier::Command) => match (
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
        Some(GatewayResolvedPluginTier::Transport) => {
            match (note.surface_id.as_deref(), note.transport_key.as_deref()) {
                (Some(surface_id), Some(transport_key)) => format!(
                    "{}-{}-fallback-plugin",
                    slugify(surface_id),
                    slugify(transport_key)
                ),
                _ => fallback_plugin_id_hint(&note.lookup_key),
            }
        }
        Some(GatewayResolvedPluginTier::Surface) => match note.surface_id.as_deref() {
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
    tier: Option<GatewayResolvedPluginTier>,
    note: &GatewayResolutionBindingScaffoldNote,
) -> String {
    match tier {
        Some(GatewayResolvedPluginTier::Command) => match (
            note.surface_id.as_deref(),
            note.transport_key.as_deref(),
            note.command_selector.as_deref(),
        ) {
            (Some(surface_id), Some(transport_key), Some(command_selector)) => format!(
                "scaffolded command binding for surface={surface_id} transport={transport_key} command={command_selector}"
            ),
            _ => format!("scaffolded command binding for {}", note.lookup_key),
        },
        Some(GatewayResolvedPluginTier::Transport) => {
            match (note.surface_id.as_deref(), note.transport_key.as_deref()) {
                (Some(surface_id), Some(transport_key)) => format!(
                    "scaffolded transport fallback for surface={surface_id} transport={transport_key}"
                ),
                _ => format!("scaffolded transport fallback for {}", note.lookup_key),
            }
        }
        Some(GatewayResolvedPluginTier::Surface) => match note.surface_id.as_deref() {
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
    use crate::GatewayPluginHandoffDraft;
    use crate::GatewayPluginResolutionSnapshot;
    use crate::GatewayResolutionBindingScaffoldNote;
    use crate::GatewayResolutionPluginScaffoldStub;
    use crate::GatewayResolutionScaffoldPlan;
    use crate::GatewayResolvedPluginCandidate;
    use crate::GatewayRoutePlan;
    use crate::GatewayTransport;

    #[test]
    fn resolution_snapshot_can_emit_binding_scaffold_notes_for_remaining_gaps() {
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
            .binding_scaffold_notes();

        assert_eq!(
            notes,
            vec![
                GatewayResolutionBindingScaffoldNote {
                    lookup_key: "surface=telegram|transport=cli|command=/status".to_string(),
                    tier_label: Some("command".to_string()),
                    binding_kind: Some("command_binding".to_string()),
                    surface_id: Some("telegram".to_string()),
                    transport_key: Some("cli".to_string()),
                    command_selector: Some("/status".to_string()),
                    note: "scaffold a command binding for surface=telegram transport=cli command=/status"
                        .to_string(),
                },
                GatewayResolutionBindingScaffoldNote {
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
        let note = GatewayResolutionBindingScaffoldNote::from_lookup_key(
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
    fn resolution_snapshot_can_emit_plugin_scaffold_stubs_for_remaining_gaps() {
        let draft = GatewayPluginHandoffDraft::from_route(&GatewayRoutePlan::new(
            "telegram",
            "session-9",
            GatewayTransport::Cli,
            "/status",
        ));
        let stubs = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft)
            .with_candidates([GatewayResolvedPluginCandidate::new(
                "fallback-plugin",
                "surface=telegram|transport=cli",
                1,
            )])
            .plugin_scaffold_stubs();

        assert_eq!(
            stubs,
            vec![
                GatewayResolutionPluginScaffoldStub {
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
                GatewayResolutionPluginScaffoldStub {
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
        let stub = GatewayResolutionBindingScaffoldNote::from_lookup_key(
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
    fn resolution_snapshot_can_bundle_scaffolding_plan_artifacts() {
        let draft = GatewayPluginHandoffDraft::from_route(&GatewayRoutePlan::new(
            "telegram",
            "session-9",
            GatewayTransport::Cli,
            "/status",
        ));
        let plan = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft)
            .with_candidates([GatewayResolvedPluginCandidate::new(
                "fallback-plugin",
                "surface=telegram|transport=cli",
                1,
            )])
            .scaffold_plan();

        assert_eq!(
            plan,
            GatewayResolutionScaffoldPlan {
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
                    GatewayResolutionBindingScaffoldNote {
                        lookup_key: "surface=telegram|transport=cli|command=/status"
                            .to_string(),
                        tier_label: Some("command".to_string()),
                        binding_kind: Some("command_binding".to_string()),
                        surface_id: Some("telegram".to_string()),
                        transport_key: Some("cli".to_string()),
                        command_selector: Some("/status".to_string()),
                        note: "scaffold a command binding for surface=telegram transport=cli command=/status".to_string(),
                    },
                    GatewayResolutionBindingScaffoldNote {
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
                    GatewayResolutionPluginScaffoldStub {
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
                    GatewayResolutionPluginScaffoldStub {
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
    fn resolution_snapshot_scaffolding_plan_can_stay_empty_when_lookup_is_fully_covered() {
        let draft = GatewayPluginHandoffDraft::from_route(&GatewayRoutePlan::new(
            "hepta",
            "session-42",
            GatewayTransport::Webhook,
            "/status",
        ));
        let plan = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft)
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
            .scaffold_plan();

        assert!(!plan.scaffolding_required);
        assert!(plan.unmatched_lookup_keys.is_empty());
        assert!(plan.unmatched_tier_labels.is_empty());
        assert!(plan.binding_scaffold_notes.is_empty());
        assert!(plan.plugin_scaffold_stubs.is_empty());
    }
}

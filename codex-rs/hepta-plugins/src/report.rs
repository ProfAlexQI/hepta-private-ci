use crate::GatewayPluginBindingDiagnosticNote;
use crate::GatewayPluginBindingLookupResolution;
use crate::GatewayPluginBindingLookupTraceStep;
use crate::GatewayPluginBindingScaffoldNote;
use crate::GatewayPluginCoverageGapNote;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPluginBindingContractReport {
    pub requested_lookup_keys: Vec<String>,
    pub requested_tier_labels: Vec<Option<String>>,
    pub matched_plugin_ids: Vec<String>,
    pub matched_lookup_keys: Vec<String>,
    pub matched_tier_labels: Vec<Option<String>>,
    pub unmatched_lookup_keys: Vec<String>,
    pub unmatched_tier_labels: Vec<Option<String>>,
    pub diagnostic_notes: Vec<GatewayPluginBindingDiagnosticNote>,
    pub coverage_gap_notes: Vec<GatewayPluginCoverageGapNote>,
    pub binding_scaffold_notes: Vec<GatewayPluginBindingScaffoldNote>,
    pub lookup_trace_steps: Vec<GatewayPluginBindingLookupTraceStep>,
    pub full_coverage: bool,
}

impl GatewayPluginBindingLookupResolution {
    pub fn contract_report(&self) -> GatewayPluginBindingContractReport {
        let coverage = self.coverage_digest();

        GatewayPluginBindingContractReport {
            matched_plugin_ids: self.plugin_ids().into_iter().map(str::to_string).collect(),
            diagnostic_notes: self.diagnostic_notes(),
            coverage_gap_notes: self.coverage_gap_notes(),
            binding_scaffold_notes: self.binding_scaffold_notes(),
            lookup_trace_steps: self.lookup_trace_steps(),
            requested_lookup_keys: coverage.requested_lookup_keys,
            requested_tier_labels: coverage.requested_tier_labels,
            matched_lookup_keys: coverage.matched_lookup_keys,
            matched_tier_labels: coverage.matched_tier_labels,
            unmatched_lookup_keys: coverage.unmatched_lookup_keys,
            unmatched_tier_labels: coverage.unmatched_tier_labels,
            full_coverage: coverage.full_coverage,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::GatewayPluginBinding;
    use crate::GatewayPluginBindingCatalog;
    use crate::GatewayPluginBindingContractReport;
    use crate::GatewayPluginBindingDiagnosticNote;
    use crate::GatewayPluginBindingLookupTraceStep;
    use crate::GatewayPluginBindingScaffoldNote;
    use crate::GatewayPluginCoverageGapNote;

    #[test]
    fn contract_report_can_bundle_partial_lookup_coverage_artifacts() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(GatewayPluginBinding::new(
            "fallback-plugin",
            "telegram",
            "cli",
            "general cli hooks",
        ));

        let report = catalog
            .resolve_lookup_keys([
                "surface=telegram|transport=cli|command=/status",
                "surface=telegram|transport=cli",
                "surface=telegram",
            ])
            .contract_report();

        assert_eq!(
            report,
            GatewayPluginBindingContractReport {
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
                matched_plugin_ids: vec!["fallback-plugin".to_string()],
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
                diagnostic_notes: vec![GatewayPluginBindingDiagnosticNote {
                    plugin_id: "fallback-plugin".to_string(),
                    lookup_key: "surface=telegram|transport=cli".to_string(),
                    tier_label: Some("transport".to_string()),
                    explanation:
                        "matched transport fallback via surface=telegram|transport=cli"
                            .to_string(),
                }],
                coverage_gap_notes: vec![
                    GatewayPluginCoverageGapNote {
                        lookup_key: "surface=telegram|transport=cli|command=/status"
                            .to_string(),
                        tier_label: Some("command".to_string()),
                        explanation: "no command binding matched for surface=telegram|transport=cli|command=/status".to_string(),
                    },
                    GatewayPluginCoverageGapNote {
                        lookup_key: "surface=telegram".to_string(),
                        tier_label: Some("surface".to_string()),
                        explanation:
                            "no surface fallback matched for surface=telegram".to_string(),
                    },
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
                lookup_trace_steps: vec![
                    GatewayPluginBindingLookupTraceStep {
                        lookup_key: "surface=telegram|transport=cli|command=/status"
                            .to_string(),
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
                ],
                full_coverage: false,
            }
        );
    }

    #[test]
    fn contract_report_can_bundle_full_lookup_coverage_artifacts() {
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

        let report = catalog
            .resolve_lookup_keys([
                "surface=hepta|transport=webhook|command=/status",
                "surface=hepta|transport=webhook",
                "surface=hepta",
            ])
            .contract_report();

        assert!(report.full_coverage);
        assert!(report.unmatched_lookup_keys.is_empty());
        assert!(report.unmatched_tier_labels.is_empty());
        assert!(report.coverage_gap_notes.is_empty());
        assert!(report.binding_scaffold_notes.is_empty());
        assert_eq!(
            report.lookup_trace_steps,
            vec![
                GatewayPluginBindingLookupTraceStep {
                    lookup_key: "surface=hepta|transport=webhook|command=/status".to_string(),
                    tier_label: Some("command".to_string()),
                    matched_plugin_ids: vec!["status-plugin".to_string()],
                    matched: true,
                    note: "command lookup is covered by status-plugin".to_string(),
                },
                GatewayPluginBindingLookupTraceStep {
                    lookup_key: "surface=hepta|transport=webhook".to_string(),
                    tier_label: Some("transport".to_string()),
                    matched_plugin_ids: vec!["fallback-plugin".to_string()],
                    matched: true,
                    note: "transport lookup is covered by fallback-plugin".to_string(),
                },
                GatewayPluginBindingLookupTraceStep {
                    lookup_key: "surface=hepta".to_string(),
                    tier_label: Some("surface".to_string()),
                    matched_plugin_ids: vec!["surface-plugin".to_string()],
                    matched: true,
                    note: "surface lookup is covered by surface-plugin".to_string(),
                },
            ]
        );
        assert_eq!(
            report.matched_plugin_ids,
            vec!["status-plugin", "fallback-plugin", "surface-plugin"]
        );
    }
}

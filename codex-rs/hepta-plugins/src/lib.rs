mod adapter;
mod binding;
mod catalog;
mod coverage;
mod explanation;
mod integrity;
mod intent;
mod report;
mod scaffolding;
mod telegram;
mod trace;

pub use adapter::EchoGatewayPluginAdapter;
pub use adapter::FailingGatewayPluginAdapter;
pub use adapter::GatewayPluginAdapter;
pub use adapter::GatewayPluginAdapterResult;
pub use adapter::GatewayPluginExecutableHandoff;
pub use adapter::GatewayPluginExecutionAttempt;
pub use adapter::GatewayPluginExecutionPlan;
pub use adapter::GatewayPluginExecutionPolicy;
pub use adapter::GatewayPluginExecutionTelemetry;
pub use adapter::GatewayPluginExecutionTraceStep;
pub use binding::GatewayPluginBinding;
pub use binding::GatewayPluginBindingCatalog;
pub use binding::GatewayPluginBindingLookupResolution;
pub use binding::GatewayPluginBindingMatch;
pub use binding::GatewayPluginBindingResolution;
pub use binding::GatewayPluginBindingTier;
pub use catalog::PluginCatalog;
pub use coverage::GatewayPluginBindingCoverageDigest;
pub use explanation::GatewayPluginBindingDiagnosticNote;
pub use explanation::GatewayPluginCoverageGapNote;
pub use integrity::PLUGIN_MANIFESTS_VALID;
pub use integrity::PLUGIN_OPERATIONAL_READY;
pub use integrity::PLUGIN_REGISTRATIONS_RESOLVED;
pub use integrity::PluginIntegritySnapshot;
pub use integrity::PluginOperationalReadinessReport;
pub use intent::GatewayPluginLookupIntentNote;
pub use report::GatewayPluginBindingContractReport;
pub use scaffolding::GatewayPluginBindingScaffoldNote;
pub use scaffolding::GatewayPluginBindingScaffoldPlan;
pub use scaffolding::GatewayPluginScaffoldStub;
pub use telegram::TELEGRAM_PLUGIN_ID;
pub use telegram::TelegramPluginDescriptor;
pub use telegram::TelegramPluginServiceMode;
pub use telegram::telegram_plugin_bindings;
pub use telegram::telegram_plugin_descriptor;
pub use telegram::telegram_plugin_manifest;
pub use trace::GatewayPluginBindingLookupTraceStep;

use hepta_core::Plugin;
use hepta_core::PluginManifest;

pub struct StaticPlugin {
    manifest: PluginManifest,
}

impl StaticPlugin {
    pub fn new(
        id: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            manifest: PluginManifest {
                id: id.into(),
                version: version.into(),
                description: description.into(),
            },
        }
    }

    pub fn from_manifest(manifest: PluginManifest) -> Self {
        Self { manifest }
    }
}

impl Plugin for StaticPlugin {
    fn manifest(&self) -> &PluginManifest {
        &self.manifest
    }
}

#[cfg(test)]
mod tests {
    use super::GatewayPluginBinding;
    use super::GatewayPluginBindingCatalog;
    use super::PluginCatalog;
    use super::StaticPlugin;
    use hepta_core::Plugin;
    use hepta_core::PluginManifest;

    #[test]
    fn static_plugin_wraps_manifest() {
        let plugin = StaticPlugin::new("lint", "0.1.0", "lint helpers");

        assert_eq!(plugin.manifest().id, "lint");
        assert_eq!(plugin.manifest().version, "0.1.0");
        assert_eq!(plugin.manifest().description, "lint helpers");
    }

    #[test]
    fn static_plugin_can_be_built_from_manifest() {
        let plugin = StaticPlugin::from_manifest(PluginManifest {
            id: "doctor".into(),
            version: "0.2.0".into(),
            description: "doctor hooks".into(),
        });

        assert_eq!(plugin.manifest().id, "doctor");
        assert_eq!(plugin.manifest().version, "0.2.0");
    }

    #[test]
    fn catalog_registers_plugins_in_sorted_order() {
        let mut catalog = PluginCatalog::new();
        let lint = StaticPlugin::new("lint", "0.1.0", "lint helpers");
        let doctor = StaticPlugin::new("doctor", "0.2.0", "doctor hooks");

        catalog.register(&lint);
        catalog.register(&doctor);

        assert_eq!(catalog.ids(), vec!["doctor", "lint"]);
    }

    #[test]
    fn catalog_replaces_existing_manifest_for_same_id() {
        let mut catalog = PluginCatalog::new();
        catalog.register_manifest(PluginManifest {
            id: "lint".into(),
            version: "0.1.0".into(),
            description: "lint helpers".into(),
        });
        catalog.register_manifest(PluginManifest {
            id: "lint".into(),
            version: "0.1.1".into(),
            description: "lint helpers patched".into(),
        });

        assert_eq!(catalog.manifests().len(), 1);
        assert_eq!(catalog.manifest("lint").unwrap().version, "0.1.1");
        assert!(catalog.contains("lint"));
    }

    #[test]
    fn gateway_binding_catalog_is_available_from_crate_surface() {
        let mut catalog = GatewayPluginBindingCatalog::new();
        catalog.register(
            GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status commands")
                .with_command_selector("/status"),
        );

        let matches = catalog.bindings_for_route("hepta", "webhook", Some("/status"));

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].plugin_id, "status-plugin");
    }
}

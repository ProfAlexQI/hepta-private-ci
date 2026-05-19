use hepta_core::{Plugin, PluginManifest};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PluginCatalog {
    manifests: Vec<PluginManifest>,
}

impl PluginCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, plugin: &dyn Plugin) {
        self.register_manifest(plugin.manifest().clone());
    }

    pub fn register_manifest(&mut self, manifest: PluginManifest) {
        if let Some(existing) = self
            .manifests
            .iter_mut()
            .find(|existing| existing.id == manifest.id)
        {
            *existing = manifest;
            return;
        }

        self.manifests.push(manifest);
        self.manifests.sort_by(|left, right| left.id.cmp(&right.id));
    }

    pub fn contains(&self, id: &str) -> bool {
        self.manifests.iter().any(|manifest| manifest.id == id)
    }

    pub fn manifest(&self, id: &str) -> Option<&PluginManifest> {
        self.manifests.iter().find(|manifest| manifest.id == id)
    }

    pub fn manifests(&self) -> &[PluginManifest] {
        &self.manifests
    }

    pub fn ids(&self) -> Vec<&str> {
        self.manifests
            .iter()
            .map(|manifest| manifest.id.as_str())
            .collect()
    }
}

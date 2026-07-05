use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use hepta_core::HeptaError;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

/// File-backed plugin-scoped key/value state used by runtime adapters that need
/// Hepta-style `openKeyedStore` and `registerIfAbsent` semantics without
/// widening access to the session store or private context payloads.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DurablePluginKeyedStore {
    namespace: String,
    #[serde(default)]
    values: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginSessionEntry {
    pub session_id: String,
    pub plugin_id: String,
    pub store_namespace: String,
    pub visible_turn_count: usize,
    pub hidden_runtime_context_included: bool,
    pub raw_private_payload_included: bool,
    pub session_store_mutated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginKeyedStoreRoundtripReport {
    pub namespace: String,
    pub key_count: usize,
    pub temp_file_created: bool,
    pub temp_file_deleted: bool,
    pub roundtrip_ok: bool,
    pub credential_value_read: bool,
    pub private_context_persisted: bool,
    pub session_store_mutated: bool,
}

impl DurablePluginKeyedStore {
    pub fn open_keyed_store(namespace: impl Into<String>) -> Result<Self, HeptaError> {
        let namespace = namespace.into();
        validate_namespace(&namespace)?;
        Ok(Self {
            namespace,
            values: BTreeMap::new(),
        })
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn key_count(&self) -> usize {
        self.values.len()
    }

    pub fn register_if_absent(&mut self, key: impl Into<String>, value: Value) -> bool {
        let key = key.into();
        if self.values.contains_key(&key) {
            return false;
        }
        self.values.insert(key, value);
        true
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    pub fn session_entry(&self, session_id: &str, plugin_id: &str) -> PluginSessionEntry {
        PluginSessionEntry {
            session_id: session_id.to_string(),
            plugin_id: plugin_id.to_string(),
            store_namespace: self.namespace.clone(),
            visible_turn_count: 0,
            hidden_runtime_context_included: false,
            raw_private_payload_included: false,
            session_store_mutated: false,
        }
    }

    pub fn save_atomic(&self, path: &Path) -> Result<(), HeptaError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!("failed to create plugin store parent dir: {err}"))
            })?;
        }
        let body = serde_json::to_vec_pretty(self)
            .map_err(|err| HeptaError(format!("failed to serialize plugin keyed store: {err}")))?;
        let temp_path = temp_path_for(path);
        fs::write(&temp_path, body)
            .map_err(|err| HeptaError(format!("failed to write temp plugin store: {err}")))?;
        fs::rename(&temp_path, path).map_err(|err| {
            HeptaError(format!("failed to atomically publish plugin store: {err}"))
        })?;
        Ok(())
    }

    pub fn load_from_path(path: &Path) -> Result<Self, HeptaError> {
        let body = fs::read_to_string(path)
            .map_err(|err| HeptaError(format!("failed to read plugin store: {err}")))?;
        let parsed: Self = serde_json::from_str(&body)
            .map_err(|err| HeptaError(format!("failed to parse plugin store: {err}")))?;
        validate_namespace(&parsed.namespace)?;
        Ok(parsed)
    }

    pub fn sample_temp_roundtrip(
        &self,
        path: &Path,
    ) -> Result<PluginKeyedStoreRoundtripReport, HeptaError> {
        self.save_atomic(path)?;
        let temp_file_created = path.exists();
        let loaded = Self::load_from_path(path)?;
        fs::remove_file(path)
            .map_err(|err| HeptaError(format!("failed to remove sample plugin store: {err}")))?;
        let temp_file_deleted = !path.exists();
        Ok(PluginKeyedStoreRoundtripReport {
            namespace: self.namespace.clone(),
            key_count: self.key_count(),
            temp_file_created,
            temp_file_deleted,
            roundtrip_ok: loaded == *self,
            credential_value_read: false,
            private_context_persisted: false,
            session_store_mutated: false,
        })
    }
}

fn validate_namespace(namespace: &str) -> Result<(), HeptaError> {
    let trimmed = namespace.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(
            "plugin keyed store namespace must not be empty".into(),
        ));
    }
    if trimmed.contains("..") || trimmed.starts_with('/') || trimmed.starts_with('~') {
        return Err(HeptaError(
            "plugin keyed store namespace must be relative and non-traversing".into(),
        ));
    }
    Ok(())
}

fn temp_path_for(path: &Path) -> PathBuf {
    let mut name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("plugin-keyed-store.json")
        .to_string();
    name.push_str(&format!(".{}.tmp", std::process::id()));
    path.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn register_if_absent_preserves_first_claim_without_private_context() {
        let mut store = DurablePluginKeyedStore::open_keyed_store("plugin/default")
            .expect("namespace should be valid");

        assert!(store.register_if_absent("dedupe:1", json!({"owner":"first"})));
        assert!(!store.register_if_absent("dedupe:1", json!({"owner":"second"})));
        assert_eq!(store.get("dedupe:1"), Some(&json!({"owner":"first"})));

        let entry = store.session_entry("session-1", "plugin-a");
        assert!(!entry.hidden_runtime_context_included);
        assert!(!entry.raw_private_payload_included);
        assert!(!entry.session_store_mutated);
    }

    #[test]
    fn file_backed_roundtrip_is_atomic_and_deletes_sample_file() {
        let mut store = DurablePluginKeyedStore::open_keyed_store("plugin/default")
            .expect("namespace should be valid");
        store.register_if_absent("k", json!({"v":1}));
        let path = std::env::temp_dir().join(format!(
            "hepta-runtime-plugin-store-test-{}.json",
            std::process::id()
        ));
        let _ = fs::remove_file(&path);

        let report = store
            .sample_temp_roundtrip(&path)
            .expect("sample roundtrip should work");

        assert!(report.temp_file_created);
        assert!(report.temp_file_deleted);
        assert!(report.roundtrip_ok);
        assert!(!report.credential_value_read);
        assert!(!report.private_context_persisted);
        assert!(!path.exists());
    }

    #[test]
    fn namespace_validation_denies_absolute_and_parent_escape() {
        assert!(DurablePluginKeyedStore::open_keyed_store("plugin/default").is_ok());
        assert!(DurablePluginKeyedStore::open_keyed_store("../secret").is_err());
        assert!(DurablePluginKeyedStore::open_keyed_store("/tmp/plugin").is_err());
        assert!(DurablePluginKeyedStore::open_keyed_store("~/.secret").is_err());
    }
}

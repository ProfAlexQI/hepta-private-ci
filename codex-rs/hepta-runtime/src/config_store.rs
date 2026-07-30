use std::fs;
use std::path::PathBuf;

use hepta_core::HeptaError;
use serde::Deserialize;
use serde::Serialize;

use crate::current_unix_ms;

pub const DEFAULT_CONFIG_STORE_PATH: &str = ".hepta/config-store-v0.json";
pub const DEFAULT_CONFIG_STORE_ID: &str = "hepta-native-config-store";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigPatchStatus {
    Planned,
    Applied,
    Reverted,
}

impl ConfigPatchStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Applied => "applied",
            Self::Reverted => "reverted",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigStoreFile {
    pub version: u32,
    pub store_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default)]
    pub entries: Vec<ConfigSchemaEntry>,
    #[serde(default)]
    pub patches: Vec<ConfigPatchRecord>,
    #[serde(default)]
    pub restart_refreshes: Vec<ConfigRestartRefreshRecord>,
    #[serde(default)]
    pub mutation_sequence: u64,
    #[serde(default)]
    pub events: Vec<ConfigStoreEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigSchemaEntry {
    pub path: String,
    pub value_json: String,
    pub description: String,
    pub sensitive: bool,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigPatchRecord {
    pub patch_id: String,
    pub path: String,
    pub old_value_json: String,
    pub new_value_json: String,
    pub exact_payload_preview: String,
    pub idempotency_key: String,
    #[serde(default)]
    pub mutation_sequence: u64,
    #[serde(default)]
    pub serialized_mutation_lock: bool,
    #[serde(default)]
    pub restart_refresh_required: bool,
    pub status: ConfigPatchStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readback_evidence_id: Option<String>,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigRestartRefreshRecord {
    pub refresh_id: String,
    pub reason: String,
    pub config_hash: String,
    pub reread_from_disk: bool,
    pub warning_not_fatal: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_env_value_source: Option<String>,
    #[serde(default)]
    pub service_env_value_source_preserved: bool,
    pub service_restart_performed: bool,
    pub created_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigStoreEvent {
    pub event_id: String,
    pub event_type: String,
    pub path: String,
    pub occurred_at_unix_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConfigStoreReport {
    pub store_path: String,
    pub store: ConfigStoreFile,
    pub entry_count: usize,
    pub planned_patch_count: usize,
    pub applied_patch_count: usize,
    pub reverted_patch_count: usize,
    pub restart_refresh_count: usize,
    pub mutation_sequence: u64,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConfigSchemaEntryReport {
    pub store_path: String,
    pub entry: ConfigSchemaEntry,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConfigPatchPlanReport {
    pub store_path: String,
    pub patch: ConfigPatchRecord,
    pub duplicate_idempotency_key: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConfigPatchApplyReport {
    pub store_path: String,
    pub patch_id: String,
    pub path: String,
    pub status: ConfigPatchStatus,
    pub readback_evidence_id: String,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConfigRestartRefreshReport {
    pub store_path: String,
    pub refresh: ConfigRestartRefreshRecord,
    pub persisted: bool,
}

pub struct HeptaConfigStore {
    path: PathBuf,
}

impl HeptaConfigStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        Ok(Self::new(crate::default_state_path(
            DEFAULT_CONFIG_STORE_PATH,
        )?))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(&self, now_unix_ms: Option<u64>) -> Result<ConfigStoreReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let store = self.load_or_default(now)?;
        Ok(ConfigStoreReport {
            store_path: self.path_display(),
            entry_count: store.entries.len(),
            planned_patch_count: count_patch_status(&store, ConfigPatchStatus::Planned),
            applied_patch_count: count_patch_status(&store, ConfigPatchStatus::Applied),
            reverted_patch_count: count_patch_status(&store, ConfigPatchStatus::Reverted),
            restart_refresh_count: store.restart_refreshes.len(),
            mutation_sequence: store.mutation_sequence,
            persisted: self.path.exists(),
            store,
        })
    }

    pub fn upsert_entry(
        &self,
        path: &str,
        value_json: &str,
        description: &str,
        sensitive: bool,
    ) -> Result<ConfigSchemaEntryReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let path = normalize_config_path(path)?;
        let value_json = normalize_json_string(value_json, "value json")?;
        let description = normalize_non_empty(description, "description")?;
        if let Some(entry) = store.entries.iter_mut().find(|entry| entry.path == path) {
            entry.value_json = value_json;
            entry.description = description;
            entry.sensitive = sensitive;
            entry.updated_at_unix_ms = now;
        } else {
            store.entries.push(ConfigSchemaEntry {
                path: path.clone(),
                value_json,
                description,
                sensitive,
                updated_at_unix_ms: now,
            });
        }
        let entry = store
            .entries
            .iter()
            .find(|entry| entry.path == path)
            .cloned()
            .ok_or_else(|| HeptaError("config entry missing after local upsert".into()))?;
        push_event(
            &mut store,
            "config_entry_upserted",
            &path,
            now,
            "Hepta config entry upserted locally",
        );
        self.save(&mut store, now)?;
        Ok(ConfigSchemaEntryReport {
            store_path: self.path_display(),
            entry,
            persisted: true,
        })
    }

    pub fn lookup_entry(&self, path: &str) -> Result<ConfigSchemaEntryReport, HeptaError> {
        let now = current_unix_ms()?;
        let store = self.load_or_default(now)?;
        let path = normalize_config_path(path)?;
        let entry = store
            .entries
            .iter()
            .find(|entry| entry.path == path)
            .cloned()
            .ok_or_else(|| HeptaError(format!("config path not found: {path}")))?;
        Ok(ConfigSchemaEntryReport {
            store_path: self.path_display(),
            entry,
            persisted: self.path.exists(),
        })
    }

    pub fn plan_patch(
        &self,
        path: &str,
        new_value_json: &str,
        exact_payload_preview: &str,
        idempotency_key: &str,
    ) -> Result<ConfigPatchPlanReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let path = normalize_config_path(path)?;
        let new_value_json = normalize_json_string(new_value_json, "new value json")?;
        let exact_payload_preview =
            normalize_non_empty(exact_payload_preview, "exact payload preview")?;
        let idempotency_key = normalize_non_empty(idempotency_key, "idempotency key")?;
        if let Some(existing) = store
            .patches
            .iter()
            .find(|patch| patch.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(ConfigPatchPlanReport {
                store_path: self.path_display(),
                patch: existing,
                duplicate_idempotency_key: true,
                persisted: self.path.exists(),
            });
        }
        let old_value_json = store
            .entries
            .iter()
            .find(|entry| entry.path == path)
            .map(|entry| entry.value_json.clone())
            .ok_or_else(|| HeptaError(format!("config path not found: {path}")))?;
        let mutation_sequence = store.mutation_sequence.saturating_add(1);
        store.mutation_sequence = mutation_sequence;
        let patch_id = format!("cfgpatch-{}-{}", now, store.patches.len() + 1);
        let patch = ConfigPatchRecord {
            patch_id: patch_id.clone(),
            path: path.clone(),
            old_value_json,
            new_value_json,
            exact_payload_preview,
            idempotency_key,
            mutation_sequence,
            serialized_mutation_lock: true,
            restart_refresh_required: true,
            status: ConfigPatchStatus::Planned,
            operator_id: None,
            readback_evidence_id: None,
            created_at_unix_ms: now,
            updated_at_unix_ms: now,
        };
        store.patches.push(patch.clone());
        push_event(
            &mut store,
            "config_patch_planned",
            &path,
            now,
            "config patch planned with exact payload preview and serialized mutation sequence",
        );
        self.save(&mut store, now)?;
        Ok(ConfigPatchPlanReport {
            store_path: self.path_display(),
            patch,
            duplicate_idempotency_key: false,
            persisted: true,
        })
    }

    pub fn apply_patch(
        &self,
        patch_id: &str,
        operator_id: &str,
        readback_evidence_id: &str,
    ) -> Result<ConfigPatchApplyReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let patch_id = normalize_non_empty(patch_id, "patch id")?;
        let operator_id = normalize_non_empty(operator_id, "operator id")?;
        let readback_evidence_id =
            normalize_non_empty(readback_evidence_id, "readback evidence id")?;
        let patch_index = store
            .patches
            .iter()
            .position(|patch| patch.patch_id == patch_id)
            .ok_or_else(|| HeptaError(format!("config patch not found: {patch_id}")))?;
        if store.patches[patch_index].status != ConfigPatchStatus::Planned {
            return Err(HeptaError(format!(
                "config patch {patch_id} is not planned; current status is {}",
                store.patches[patch_index].status.label()
            )));
        }
        let path = store.patches[patch_index].path.clone();
        let new_value_json = store.patches[patch_index].new_value_json.clone();
        let entry = store
            .entries
            .iter_mut()
            .find(|entry| entry.path == path)
            .ok_or_else(|| HeptaError(format!("config path not found: {path}")))?;
        entry.value_json = new_value_json;
        entry.updated_at_unix_ms = now;
        {
            let patch = &mut store.patches[patch_index];
            patch.status = ConfigPatchStatus::Applied;
            patch.operator_id = Some(operator_id);
            patch.readback_evidence_id = Some(readback_evidence_id.clone());
            patch.updated_at_unix_ms = now;
        }
        push_event(
            &mut store,
            "config_patch_applied",
            &path,
            now,
            "config patch applied in Hepta-owned config store under serialized mutation sequence",
        );
        self.save(&mut store, now)?;
        Ok(ConfigPatchApplyReport {
            store_path: self.path_display(),
            patch_id,
            path,
            status: ConfigPatchStatus::Applied,
            readback_evidence_id,
            persisted: true,
        })
    }

    pub fn record_restart_time_refresh(
        &self,
        reason: &str,
        config_hash: &str,
        warning_not_fatal: bool,
        service_env_value_source: Option<&str>,
    ) -> Result<ConfigRestartRefreshReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let reason = normalize_non_empty(reason, "restart refresh reason")?;
        let config_hash = normalize_non_empty(config_hash, "config hash")?;
        let service_env_value_source = service_env_value_source
            .map(|source| normalize_non_empty(source, "service env value source"))
            .transpose()?;
        let refresh_id = format!("cfgrefresh-{}-{}", now, store.restart_refreshes.len() + 1);
        let refresh = ConfigRestartRefreshRecord {
            refresh_id: refresh_id.clone(),
            reason,
            config_hash,
            reread_from_disk: true,
            warning_not_fatal,
            service_env_value_source_preserved: service_env_value_source.is_some(),
            service_env_value_source,
            service_restart_performed: false,
            created_at_unix_ms: now,
        };
        store.restart_refreshes.push(refresh.clone());
        store.restart_refreshes.truncate(1024);
        push_event(
            &mut store,
            "config_restart_refresh_recorded",
            "runtime.config",
            now,
            "restart-time config refresh recorded after rereading Hepta config from disk; service restart not performed",
        );
        self.save(&mut store, now)?;
        Ok(ConfigRestartRefreshReport {
            store_path: self.path_display(),
            refresh,
            persisted: true,
        })
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<ConfigStoreFile, HeptaError> {
        if !self.path.exists() {
            return Ok(ConfigStoreFile {
                version: 1,
                store_id: DEFAULT_CONFIG_STORE_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                entries: Vec::new(),
                patches: Vec::new(),
                restart_refreshes: Vec::new(),
                mutation_sequence: 0,
                events: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read config-store {}: {err}",
                self.path.display()
            ))
        })?;
        let mut store: ConfigStoreFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse config-store {}: {err}",
                self.path.display()
            ))
        })?;
        if store.version != 1 {
            return Err(HeptaError(format!(
                "unsupported config-store version {} in {}",
                store.version,
                self.path.display()
            )));
        }
        store.events.truncate(1024);
        store.restart_refreshes.truncate(1024);
        Ok(store)
    }

    fn save(&self, store: &mut ConfigStoreFile, now_unix_ms: u64) -> Result<(), HeptaError> {
        store.updated_at_unix_ms = now_unix_ms;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create config-store directory {}: {err}",
                    parent.display()
                ))
            })?;
        }
        let text = serde_json::to_string_pretty(store)
            .map_err(|err| HeptaError(format!("failed to serialize config-store: {err}")))?;
        fs::write(&self.path, text).map_err(|err| {
            HeptaError(format!(
                "failed to write config-store {}: {err}",
                self.path.display()
            ))
        })
    }
}

fn count_patch_status(store: &ConfigStoreFile, status: ConfigPatchStatus) -> usize {
    store
        .patches
        .iter()
        .filter(|patch| patch.status == status)
        .count()
}

fn normalize_config_path(value: &str) -> Result<String, HeptaError> {
    let value = normalize_non_empty(value, "config path")?;
    if value.contains("OpenClaw Gateway") {
        return Err(HeptaError(
            "Hepta config paths must not target OpenClaw Gateway runtime".into(),
        ));
    }
    Ok(value)
}

fn normalize_json_string(value: &str, label: &str) -> Result<String, HeptaError> {
    let value = normalize_non_empty(value, label)?;
    serde_json::from_str::<serde_json::Value>(&value)
        .map_err(|err| HeptaError(format!("config store {label} must be valid JSON: {err}")))?;
    Ok(value)
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!(
            "config store {label} must not be empty"
        )));
    }
    Ok(trimmed.to_string())
}

fn push_event(
    store: &mut ConfigStoreFile,
    event_type: &str,
    path: &str,
    now_unix_ms: u64,
    summary: &str,
) {
    store.events.push(ConfigStoreEvent {
        event_id: format!("cfgevt-{}-{}", now_unix_ms, store.events.len() + 1),
        event_type: event_type.into(),
        path: path.into(),
        occurred_at_unix_ms: now_unix_ms,
        summary: summary.into(),
    });
    store.events.truncate(1024);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hepta-config-store-test-{}-{}-{name}.json",
            std::process::id(),
            current_unix_ms().unwrap_or(0)
        ))
    }

    #[test]
    fn config_store_plans_and_applies_patch_with_readback() {
        let path = temp_file("apply");
        let store = HeptaConfigStore::new(&path);
        store
            .upsert_entry(
                "runtime.delivery.max_retries",
                "3",
                "maximum delivery retry attempts",
                false,
            )
            .unwrap();
        let patch = store
            .plan_patch(
                "runtime.delivery.max_retries",
                "5",
                "set runtime.delivery.max_retries = 5",
                "cfg-idem-1",
            )
            .unwrap();
        assert_eq!(patch.patch.status, ConfigPatchStatus::Planned);
        assert_eq!(patch.patch.mutation_sequence, 1);
        assert!(patch.patch.serialized_mutation_lock);
        assert!(patch.patch.restart_refresh_required);
        let duplicate = store
            .plan_patch(
                "runtime.delivery.max_retries",
                "5",
                "set runtime.delivery.max_retries = 5",
                "cfg-idem-1",
            )
            .unwrap();
        assert!(duplicate.duplicate_idempotency_key);
        assert_eq!(
            duplicate.patch.mutation_sequence,
            patch.patch.mutation_sequence
        );
        let applied = store
            .apply_patch(&patch.patch.patch_id, "operator-a", "rb-config-1")
            .unwrap();
        assert_eq!(applied.status, ConfigPatchStatus::Applied);
        let lookup = store.lookup_entry("runtime.delivery.max_retries").unwrap();
        assert_eq!(lookup.entry.value_json, "5");
        let report = store.report(None).unwrap();
        assert_eq!(report.applied_patch_count, 1);
        assert_eq!(report.mutation_sequence, 1);
        assert!(report.store.events.iter().any(|event| {
            event.event_type == "config_patch_applied"
                && event.summary.contains("serialized mutation sequence")
        }));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn config_store_records_restart_refresh_warning_and_service_env_provenance() {
        let path = temp_file("restart-refresh");
        let store = HeptaConfigStore::new(&path);
        let refresh = store
            .record_restart_time_refresh(
                "first in-process restart-loop startup",
                "sha256:config-snapshot",
                true,
                Some("launchd:HEPTA_CONFIG_PATH"),
            )
            .unwrap();
        assert!(refresh.refresh.reread_from_disk);
        assert!(refresh.refresh.warning_not_fatal);
        assert_eq!(
            refresh.refresh.service_env_value_source.as_deref(),
            Some("launchd:HEPTA_CONFIG_PATH")
        );
        assert!(refresh.refresh.service_env_value_source_preserved);
        assert!(!refresh.refresh.service_restart_performed);
        let report = store.report(None).unwrap();
        assert_eq!(report.restart_refresh_count, 1);
        assert!(report.store.events.iter().any(|event| {
            event.event_type == "config_restart_refresh_recorded"
                && event.summary.contains("rereading Hepta config from disk")
        }));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn config_store_rejects_invalid_json_and_gateway_target_paths() {
        let path = temp_file("reject");
        let store = HeptaConfigStore::new(&path);
        assert!(
            store
                .upsert_entry("runtime.bad", "not-json", "bad", false)
                .is_err()
        );
        assert!(
            store
                .upsert_entry("OpenClaw Gateway.config", "true", "forbidden", false)
                .is_err()
        );
        let _ = fs::remove_file(path);
    }
}

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use hepta_core::HeptaError;
use serde::Deserialize;
use serde::Serialize;

use crate::current_unix_ms;

pub const DEFAULT_TOPIC_NEURON_STORE_PATH: &str = ".hepta/topic-neuron-store-v0.json";
pub const DEFAULT_TOPIC_NEURON_STORE_ID: &str = "hepta-native-topic-neuron-store";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopicNeuronStoreFile {
    pub version: u32,
    pub store_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default)]
    pub topics: Vec<TopicNeuronRecord>,
    #[serde(default)]
    pub feedback_events: Vec<TopicNeuronFeedbackEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopicNeuronRecord {
    pub topic_id: String,
    pub label: String,
    pub activation_score_millis: u32,
    pub evidence_count: u32,
    #[serde(default)]
    pub linked_topic_ids: Vec<String>,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopicNeuronFeedbackEvent {
    pub event_id: String,
    pub topic_id: String,
    pub signal: String,
    pub score_delta_millis: i32,
    pub occurred_at_unix_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TopicNeuronStoreReport {
    pub store_path: String,
    pub store: TopicNeuronStoreFile,
    pub topic_count: usize,
    pub feedback_count: usize,
    pub top_topic_id: Option<String>,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TopicNeuronObserveReport {
    pub store_path: String,
    pub topic: TopicNeuronRecord,
    pub created: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TopicNeuronFeedbackReport {
    pub store_path: String,
    pub topic: TopicNeuronRecord,
    pub event: TopicNeuronFeedbackEvent,
    pub persisted: bool,
}

pub struct TopicNeuronStore {
    path: PathBuf,
}

impl TopicNeuronStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        Ok(Self::new(crate::default_state_path(
            DEFAULT_TOPIC_NEURON_STORE_PATH,
        )?))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(&self, now_unix_ms: Option<u64>) -> Result<TopicNeuronStoreReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let store = self.load_or_default(now)?;
        let top_topic_id = store
            .topics
            .iter()
            .max_by_key(|topic| topic.activation_score_millis)
            .map(|topic| topic.topic_id.clone());
        Ok(TopicNeuronStoreReport {
            store_path: self.path_display(),
            topic_count: store.topics.len(),
            feedback_count: store.feedback_events.len(),
            top_topic_id,
            persisted: self.path.exists(),
            store,
        })
    }

    pub fn observe_topic(
        &self,
        topic_id: &str,
        label: &str,
        linked_topic_ids: Vec<String>,
    ) -> Result<TopicNeuronObserveReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let topic_id = normalize_id(topic_id, "topic id")?;
        let label = normalize_non_empty(label, "topic label")?;
        let linked_topic_ids = normalize_links(linked_topic_ids)?;
        let mut created = false;
        if let Some(topic) = store
            .topics
            .iter_mut()
            .find(|topic| topic.topic_id == topic_id)
        {
            topic.label = label;
            topic.evidence_count = topic.evidence_count.saturating_add(1);
            topic.activation_score_millis = topic.activation_score_millis.saturating_add(25);
            topic.linked_topic_ids = merge_links(&topic.linked_topic_ids, linked_topic_ids);
            topic.updated_at_unix_ms = now;
        } else {
            created = true;
            store.topics.push(TopicNeuronRecord {
                topic_id: topic_id.clone(),
                label,
                activation_score_millis: 25,
                evidence_count: 1,
                linked_topic_ids,
                created_at_unix_ms: now,
                updated_at_unix_ms: now,
            });
        }
        let topic = store
            .topics
            .iter()
            .find(|topic| topic.topic_id == topic_id)
            .cloned()
            .ok_or_else(|| HeptaError("observed topic missing after local upsert".into()))?;
        self.save(&mut store, now)?;
        Ok(TopicNeuronObserveReport {
            store_path: self.path_display(),
            topic,
            created,
            persisted: true,
        })
    }

    pub fn apply_feedback(
        &self,
        topic_id: &str,
        signal: &str,
        score_delta_millis: i32,
        summary: &str,
    ) -> Result<TopicNeuronFeedbackReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let topic_id = normalize_id(topic_id, "topic id")?;
        let signal = normalize_non_empty(signal, "signal")?;
        let summary = normalize_non_empty(summary, "summary")?;
        let topic = store
            .topics
            .iter_mut()
            .find(|topic| topic.topic_id == topic_id)
            .ok_or_else(|| HeptaError(format!("topic not found: {topic_id}")))?;
        topic.activation_score_millis =
            apply_delta(topic.activation_score_millis, score_delta_millis);
        topic.updated_at_unix_ms = now;
        let topic = topic.clone();
        let event = TopicNeuronFeedbackEvent {
            event_id: format!("topicevt-{}-{}", now, store.feedback_events.len() + 1),
            topic_id: topic_id.clone(),
            signal,
            score_delta_millis,
            occurred_at_unix_ms: now,
            summary,
        };
        store.feedback_events.push(event.clone());
        store.feedback_events.truncate(1024);
        self.save(&mut store, now)?;
        Ok(TopicNeuronFeedbackReport {
            store_path: self.path_display(),
            topic,
            event,
            persisted: true,
        })
    }

    pub fn routing_weights(&self) -> Result<HashMap<String, u32>, HeptaError> {
        let now = current_unix_ms()?;
        let store = self.load_or_default(now)?;
        Ok(store
            .topics
            .into_iter()
            .map(|topic| (topic.topic_id, topic.activation_score_millis))
            .collect())
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<TopicNeuronStoreFile, HeptaError> {
        if !self.path.exists() {
            return Ok(TopicNeuronStoreFile {
                version: 1,
                store_id: DEFAULT_TOPIC_NEURON_STORE_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                topics: Vec::new(),
                feedback_events: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read topic-neuron store {}: {err}",
                self.path.display()
            ))
        })?;
        let mut store: TopicNeuronStoreFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse topic-neuron store {}: {err}",
                self.path.display()
            ))
        })?;
        if store.version != 1 {
            return Err(HeptaError(format!(
                "unsupported topic-neuron store version {} in {}",
                store.version,
                self.path.display()
            )));
        }
        store.feedback_events.truncate(1024);
        Ok(store)
    }

    fn save(&self, store: &mut TopicNeuronStoreFile, now_unix_ms: u64) -> Result<(), HeptaError> {
        store.updated_at_unix_ms = now_unix_ms;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create topic-neuron store directory {}: {err}",
                    parent.display()
                ))
            })?;
        }
        let text = serde_json::to_string_pretty(store)
            .map_err(|err| HeptaError(format!("failed to serialize topic-neuron store: {err}")))?;
        fs::write(&self.path, text).map_err(|err| {
            HeptaError(format!(
                "failed to write topic-neuron store {}: {err}",
                self.path.display()
            ))
        })
    }
}

fn normalize_links(links: Vec<String>) -> Result<Vec<String>, HeptaError> {
    let mut normalized = Vec::new();
    for link in links {
        let link = normalize_id(&link, "linked topic id")?;
        if !normalized.contains(&link) {
            normalized.push(link);
        }
    }
    Ok(normalized)
}

fn merge_links(existing: &[String], incoming: Vec<String>) -> Vec<String> {
    let mut merged = existing.to_vec();
    for link in incoming {
        if !merged.contains(&link) {
            merged.push(link);
        }
    }
    merged
}

fn apply_delta(score: u32, delta: i32) -> u32 {
    if delta.is_negative() {
        score.saturating_sub(delta.unsigned_abs())
    } else {
        score.saturating_add(delta as u32)
    }
}

fn normalize_id(value: &str, label: &str) -> Result<String, HeptaError> {
    let value = normalize_non_empty(value, label)?;
    if value.contains('\n') || value.contains('\r') || value.contains("..") {
        return Err(HeptaError(format!(
            "topic-neuron {label} must be single-line and scoped"
        )));
    }
    Ok(value)
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!(
            "topic-neuron {label} must not be empty"
        )));
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hepta-topic-neuron-test-{}-{}-{name}.json",
            std::process::id(),
            current_unix_ms().unwrap_or(0)
        ))
    }

    #[test]
    fn topic_neuron_store_observes_feedback_and_weights() {
        let path = temp_file("weights");
        let store = TopicNeuronStore::new(&path);
        let created = store
            .observe_topic("runtime-parity", "Runtime parity", vec!["openclaw".into()])
            .unwrap();
        assert!(created.created);
        let updated = store
            .observe_topic("runtime-parity", "Runtime parity", vec!["hepta".into()])
            .unwrap();
        assert_eq!(updated.topic.evidence_count, 2);
        assert_eq!(updated.topic.linked_topic_ids.len(), 2);
        let feedback = store
            .apply_feedback(
                "runtime-parity",
                "use_as_router_signal",
                75,
                "stable signal",
            )
            .unwrap();
        assert!(feedback.topic.activation_score_millis >= 100);
        let weights = store.routing_weights().unwrap();
        assert!(weights["runtime-parity"] >= 100);
        let report = store.report(None).unwrap();
        assert_eq!(report.topic_count, 1);
        assert_eq!(report.feedback_count, 1);
        assert_eq!(report.top_topic_id.as_deref(), Some("runtime-parity"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn topic_neuron_store_rejects_bad_ids_and_missing_feedback_topics() {
        let path = temp_file("reject");
        let store = TopicNeuronStore::new(&path);
        assert!(store.observe_topic("../bad", "Bad", vec![]).is_err());
        assert!(
            store
                .apply_feedback("missing", "signal", 1, "summary")
                .is_err()
        );
        let _ = fs::remove_file(path);
    }
}

use std::collections::HashSet;

use super::{DoctorCheck, DoctorStatus};

pub(super) const ACTIVE_MODEL_REGISTERED: &str = "active model registered";
pub(super) const ACTIVE_SESSION_EXISTS: &str = "active session exists";
pub(super) const ACTIVE_SESSION_ARCHIVED: &str = "active session archived";
pub(super) const RAW_SESSION_IDS_UNIQUE: &str = "raw session ids unique";
pub(super) const SESSION_MODEL_BINDINGS_UNIQUE: &str = "session model bindings unique";
pub(super) const SESSION_MODEL_BINDINGS_REFERENCE_KNOWN_SESSIONS: &str =
    "session model bindings reference known sessions";
pub(super) const SESSION_MODEL_BINDINGS_REFERENCE_REGISTERED_MODELS: &str =
    "session model bindings reference registered models";
pub(super) const HISTORY_SESSION_REFERENCES: &str = "history session references";
pub(super) const GRANTED_APPROVALS_MAP_TO_KNOWN_TOOLS: &str =
    "granted approvals map to known tools";
pub(super) const PENDING_APPROVALS_MAP_TO_KNOWN_TOOLS: &str =
    "pending approvals map to known tools";
pub(super) const MEMORY_IDS_UNIQUE: &str = "memory ids unique";
pub(super) const TOPIC_SESSIONS_CARRY_TRANSCRIPT_PROVENANCE: &str =
    "topic sessions carry transcript provenance";
pub(super) const ACTIVE_SESSION_INTELLIGENCE_REPLAY_EVAL: &str =
    "active session intelligence replay eval";
pub(super) const ACTIVE_SESSION_NEURON_LIFECYCLE: &str = "active session neuron lifecycle";
pub(super) const LOCAL_CONFIG_IMPORT_READY: &str = "local config import ready";
pub(super) const EXTERNAL_PRODUCTION_READINESS: &str = "external production readiness";
pub(super) const PRODUCTION_PARITY_READY: &str = "production parity ready";
pub(super) const RUNTIME_SNAPSHOT_ROUNDTRIP: &str = "runtime snapshot roundtrip";
pub(super) const ACTIVE_SESSION_EXPORT_ROUNDTRIP: &str = "active session export roundtrip";

pub(super) fn duplicate_values(values: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen = HashSet::new();
    values
        .into_iter()
        .filter_map(|value| {
            if seen.insert(value.clone()) {
                None
            } else {
                Some(value)
            }
        })
        .collect()
}

pub(super) fn status_from_findings(findings: &[String]) -> DoctorStatus {
    if findings.is_empty() {
        DoctorStatus::Ok
    } else {
        DoctorStatus::Fail
    }
}

pub(super) fn joined_values_or_count(values: &[String], ok_count: usize) -> String {
    if values.is_empty() {
        ok_count.to_string()
    } else {
        values.join(", ")
    }
}

pub(super) fn doctor_check(name: &str, status: DoctorStatus, detail: String) -> DoctorCheck {
    DoctorCheck {
        name: name.to_string(),
        status,
        detail,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_values_returns_only_repeated_entries() {
        let duplicates = duplicate_values([
            "session-main".to_string(),
            "session-2".to_string(),
            "session-main".to_string(),
            "session-3".to_string(),
            "session-2".to_string(),
        ]);

        assert_eq!(
            duplicates,
            vec!["session-main".to_string(), "session-2".to_string()]
        );
    }

    #[test]
    fn joined_values_or_count_prefers_counts_for_clean_results() {
        assert_eq!(joined_values_or_count(&[], 3), "3");
        assert_eq!(
            joined_values_or_count(&["tool.exec".into(), "tool.read".into()], 0),
            "tool.exec, tool.read"
        );
    }
}

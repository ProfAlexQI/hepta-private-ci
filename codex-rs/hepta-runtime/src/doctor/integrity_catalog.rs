use hepta_core::{DoctorArea, DoctorCheckOutcome, DoctorOwner};

use super::{DoctorCheck, DoctorStatus, event_log_integrity, integrity};

pub(super) fn adapt_integrity_check(check: DoctorCheck) -> DoctorCheckOutcome {
    let metadata = integrity_check_metadata(&check.name);
    let remediation = if check.status == DoctorStatus::Ok {
        None
    } else {
        metadata.remediation.map(str::to_string)
    };

    DoctorCheckOutcome {
        id: metadata.id.into(),
        area: metadata.area,
        owner: owner(metadata.responsibility),
        status: check.status.into(),
        summary: check.name,
        detail: check.detail,
        remediation,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IntegrityCheckMetadata {
    id: &'static str,
    area: DoctorArea,
    responsibility: &'static str,
    remediation: Option<&'static str>,
}

fn integrity_check_metadata(name: &str) -> IntegrityCheckMetadata {
    match name {
        integrity::ACTIVE_MODEL_REGISTERED => IntegrityCheckMetadata {
            id: "registry.active_model_registered",
            area: DoctorArea::Registry,
            responsibility: "model registry",
            remediation: Some(
                "switch to a registered model or re-register the missing active model",
            ),
        },
        integrity::ACTIVE_SESSION_EXISTS => IntegrityCheckMetadata {
            id: "session_store.active_session_exists",
            area: DoctorArea::SessionStore,
            responsibility: "session store",
            remediation: Some("recreate or switch away from the missing active session"),
        },
        integrity::ACTIVE_SESSION_ARCHIVED => IntegrityCheckMetadata {
            id: "session_store.active_session_live",
            area: DoctorArea::SessionStore,
            responsibility: "session lifecycle",
            remediation: Some(
                "switch to a live session or unarchive the active session before continuing work",
            ),
        },
        integrity::RAW_SESSION_IDS_UNIQUE => IntegrityCheckMetadata {
            id: "session_store.raw_session_ids_unique",
            area: DoctorArea::SessionStore,
            responsibility: "session store",
            remediation: Some(
                "deduplicate persisted session records so each session id appears once",
            ),
        },
        integrity::SESSION_MODEL_BINDINGS_UNIQUE => IntegrityCheckMetadata {
            id: "runtime_snapshot.session_model_bindings_unique",
            area: DoctorArea::RuntimeSnapshot,
            responsibility: "runtime snapshot",
            remediation: Some(
                "deduplicate session model bindings before persisting the runtime snapshot",
            ),
        },
        integrity::SESSION_MODEL_BINDINGS_REFERENCE_KNOWN_SESSIONS => IntegrityCheckMetadata {
            id: "session_store.session_model_bindings_known_sessions",
            area: DoctorArea::SessionStore,
            responsibility: "session model binding state",
            remediation: Some(
                "remove or repair session model bindings that point at missing sessions",
            ),
        },
        integrity::SESSION_MODEL_BINDINGS_REFERENCE_REGISTERED_MODELS => IntegrityCheckMetadata {
            id: "registry.session_model_bindings_registered_models",
            area: DoctorArea::Registry,
            responsibility: "model registry",
            remediation: Some(
                "update session model bindings so every selected model exists in the provider registry",
            ),
        },
        integrity::HISTORY_SESSION_REFERENCES => IntegrityCheckMetadata {
            id: "session_store.history_session_references",
            area: DoctorArea::SessionStore,
            responsibility: "history store",
            remediation: Some("repair or remove history entries that reference unknown sessions"),
        },
        event_log_integrity::SESSION_REFS_KNOWN => IntegrityCheckMetadata {
            id: "events.session_refs_known",
            area: DoctorArea::EventStream,
            responsibility: "event log",
            remediation: Some(
                "repair or purge event records that reference sessions missing from runtime state",
            ),
        },
        event_log_integrity::MONOTONIC_TIMESTAMPS => IntegrityCheckMetadata {
            id: "events.monotonic_timestamps",
            area: DoctorArea::EventStream,
            responsibility: "event log",
            remediation: Some(
                "reorder or regenerate stored event records so emitted timestamps stay monotonic",
            ),
        },
        integrity::GRANTED_APPROVALS_MAP_TO_KNOWN_TOOLS => IntegrityCheckMetadata {
            id: "approval.granted_tools_registered",
            area: DoctorArea::Approval,
            responsibility: "approval state",
            remediation: Some("remove granted approvals for tools that are no longer registered"),
        },
        integrity::PENDING_APPROVALS_MAP_TO_KNOWN_TOOLS => IntegrityCheckMetadata {
            id: "approval.pending_tools_registered",
            area: DoctorArea::Approval,
            responsibility: "approval state",
            remediation: Some("clear or migrate pending approvals that reference unknown tools"),
        },
        integrity::MEMORY_IDS_UNIQUE => IntegrityCheckMetadata {
            id: "runtime_snapshot.memory_ids_unique",
            area: DoctorArea::RuntimeSnapshot,
            responsibility: "memory store",
            remediation: Some("deduplicate memory records so each memory id is unique"),
        },
        integrity::TOPIC_SESSIONS_CARRY_TRANSCRIPT_PROVENANCE => IntegrityCheckMetadata {
            id: "runtime_snapshot.topic_sessions_transcript_provenance",
            area: DoctorArea::RuntimeSnapshot,
            responsibility: "topic-session provenance",
            remediation: Some(
                "re-run routing or backfill linked_transcript_spans so active topic sessions keep transcript provenance",
            ),
        },
        integrity::ACTIVE_SESSION_INTELLIGENCE_REPLAY_EVAL => IntegrityCheckMetadata {
            id: "intelligence.replay_eval",
            area: DoctorArea::Intelligence,
            responsibility: "intelligence quality gate",
            remediation: Some(
                "inspect failed replay cases and harden recall, routing, neuron activation, or skill suggestion contracts",
            ),
        },
        integrity::ACTIVE_SESSION_NEURON_LIFECYCLE => IntegrityCheckMetadata {
            id: "intelligence.neuron_lifecycle",
            area: DoctorArea::Intelligence,
            responsibility: "neuron lifecycle quality gate",
            remediation: Some(
                "inspect stored neurons and refresh compression, provenance, evidence digest, or active-topic coverage",
            ),
        },
        integrity::LOCAL_CONFIG_IMPORT_READY => IntegrityCheckMetadata {
            id: "config.local_config_import_ready",
            area: DoctorArea::Config,
            responsibility: "local config/auth/skills import",
            remediation: Some(
                "run scripts/hepta-local-import.sh and keep the generated .hepta/local-import/private tree untracked",
            ),
        },
        integrity::RUNTIME_SNAPSHOT_ROUNDTRIP => IntegrityCheckMetadata {
            id: "runtime_snapshot.roundtrip",
            area: DoctorArea::RuntimeSnapshot,
            responsibility: "runtime snapshot",
            remediation: Some(
                "update runtime snapshot serde handling so serialize and parse stay stable",
            ),
        },
        integrity::ACTIVE_SESSION_EXPORT_ROUNDTRIP => IntegrityCheckMetadata {
            id: "export_import.active_session_roundtrip",
            area: DoctorArea::ExportImport,
            responsibility: "session export",
            remediation: Some(
                "repair active session export serde handling so exports roundtrip cleanly",
            ),
        },
        _ => IntegrityCheckMetadata {
            id: "runtime_snapshot.integrity_check",
            area: DoctorArea::RuntimeSnapshot,
            responsibility: "doctor integrity",
            remediation: Some(
                "inspect the runtime doctor check and assign stable metadata for this condition",
            ),
        },
    }
}

fn owner(responsibility: &str) -> DoctorOwner {
    DoctorOwner {
        component: "hepta-runtime".into(),
        responsibility: responsibility.into(),
    }
}

#[cfg(test)]
mod tests {
    use hepta_core::{DoctorArea, DoctorStatus as CoreDoctorStatus};

    use super::*;

    #[test]
    fn integrity_catalog_assigns_stable_ids_and_remediation() {
        let outcome = adapt_integrity_check(DoctorCheck {
            name: integrity::RUNTIME_SNAPSHOT_ROUNDTRIP.into(),
            status: DoctorStatus::Warn,
            detail: "serde drift detected".into(),
        });

        assert_eq!(outcome.id, "runtime_snapshot.roundtrip");
        assert_eq!(outcome.area, DoctorArea::RuntimeSnapshot);
        assert_eq!(outcome.status, CoreDoctorStatus::Warn);
        assert_eq!(outcome.owner.component, "hepta-runtime");
        assert_eq!(outcome.owner.responsibility, "runtime snapshot");
        assert!(
            outcome
                .remediation
                .as_deref()
                .expect("warn checks should keep remediation")
                .contains("serialize and parse stay stable")
        );
    }

    #[test]
    fn integrity_catalog_omits_remediation_for_ok_checks() {
        let outcome = adapt_integrity_check(DoctorCheck {
            name: integrity::ACTIVE_SESSION_EXPORT_ROUNDTRIP.into(),
            status: DoctorStatus::Ok,
            detail: "session-main export serializable".into(),
        });

        assert_eq!(outcome.id, "export_import.active_session_roundtrip");
        assert_eq!(outcome.status, CoreDoctorStatus::Ok);
        assert!(outcome.remediation.is_none());
    }

    #[test]
    fn integrity_catalog_assigns_event_stream_metadata() {
        let outcome = adapt_integrity_check(DoctorCheck {
            name: event_log_integrity::MONOTONIC_TIMESTAMPS.into(),
            status: DoctorStatus::Fail,
            detail: "SessionSwitched/session-main@41 after 42".into(),
        });

        assert_eq!(outcome.id, "events.monotonic_timestamps");
        assert_eq!(outcome.area, DoctorArea::EventStream);
        assert_eq!(outcome.status, CoreDoctorStatus::Fail);
        assert_eq!(outcome.owner.component, "hepta-runtime");
        assert_eq!(outcome.owner.responsibility, "event log");
        assert!(
            outcome
                .remediation
                .as_deref()
                .expect("failing event checks should suggest remediation")
                .contains("timestamps stay monotonic")
        );
    }

    #[test]
    fn integrity_catalog_assigns_topic_session_provenance_metadata() {
        let outcome = adapt_integrity_check(DoctorCheck {
            name: integrity::TOPIC_SESSIONS_CARRY_TRANSCRIPT_PROVENANCE.into(),
            status: DoctorStatus::Warn,
            detail: "missing transcript provenance for topic-session-bootstrap:alpha".into(),
        });

        assert_eq!(
            outcome.id,
            "runtime_snapshot.topic_sessions_transcript_provenance"
        );
        assert_eq!(outcome.area, DoctorArea::RuntimeSnapshot);
        assert_eq!(outcome.status, CoreDoctorStatus::Warn);
        assert_eq!(outcome.owner.component, "hepta-runtime");
        assert_eq!(outcome.owner.responsibility, "topic-session provenance");
        assert!(
            outcome
                .remediation
                .as_deref()
                .expect("warn checks should keep remediation")
                .contains("linked_transcript_spans")
        );
    }

    #[test]
    fn integrity_catalog_falls_back_for_unknown_checks() {
        let outcome = adapt_integrity_check(DoctorCheck {
            name: "future doctor condition".into(),
            status: DoctorStatus::Fail,
            detail: "needs metadata".into(),
        });

        assert_eq!(outcome.id, "runtime_snapshot.integrity_check");
        assert_eq!(outcome.area, DoctorArea::RuntimeSnapshot);
        assert_eq!(outcome.status, CoreDoctorStatus::Fail);
        assert!(
            outcome
                .remediation
                .as_deref()
                .expect("unknown failing checks should still suggest follow-up")
                .contains("assign stable metadata")
        );
    }
}

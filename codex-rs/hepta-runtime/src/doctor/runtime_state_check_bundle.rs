use hepta_core::ModelRef;

use super::{DoctorCheck, DoctorStatus, integrity, runtime_state_findings::RuntimeStateFindings};

pub(super) struct RuntimeStateCheckInputs<'a> {
    pub active_model: &'a ModelRef,
    pub active_session_id: &'a str,
    pub raw_session_count: usize,
    pub raw_memory_count: usize,
    pub history_count: usize,
    pub session_model_binding_count: usize,
}

pub(super) fn build_runtime_state_checks(
    findings: &RuntimeStateFindings,
    inputs: RuntimeStateCheckInputs<'_>,
) -> Vec<DoctorCheck> {
    vec![
        integrity::doctor_check(
            integrity::ACTIVE_MODEL_REGISTERED,
            if findings.active_registered {
                DoctorStatus::Ok
            } else {
                DoctorStatus::Fail
            },
            if findings.active_registered {
                format!(
                    "{}/{}",
                    inputs.active_model.provider, inputs.active_model.model
                )
            } else {
                format!(
                    "missing {}/{}",
                    inputs.active_model.provider, inputs.active_model.model
                )
            },
        ),
        integrity::doctor_check(
            integrity::ACTIVE_SESSION_EXISTS,
            if findings.active_session_exists {
                DoctorStatus::Ok
            } else {
                DoctorStatus::Fail
            },
            inputs.active_session_id.to_string(),
        ),
        integrity::doctor_check(
            integrity::ACTIVE_SESSION_ARCHIVED,
            if findings.active_session_archived {
                DoctorStatus::Warn
            } else {
                DoctorStatus::Ok
            },
            if findings.active_session_archived {
                format!("{} is archived", inputs.active_session_id)
            } else {
                format!("{} is live", inputs.active_session_id)
            },
        ),
        integrity::doctor_check(
            integrity::RAW_SESSION_IDS_UNIQUE,
            integrity::status_from_findings(&findings.duplicate_sessions),
            integrity::joined_values_or_count(
                &findings.duplicate_sessions,
                inputs.raw_session_count,
            ),
        ),
        integrity::doctor_check(
            integrity::SESSION_MODEL_BINDINGS_UNIQUE,
            integrity::status_from_findings(&findings.duplicate_session_models),
            integrity::joined_values_or_count(
                &findings.duplicate_session_models,
                inputs.session_model_binding_count,
            ),
        ),
        integrity::doctor_check(
            integrity::SESSION_MODEL_BINDINGS_REFERENCE_KNOWN_SESSIONS,
            integrity::status_from_findings(&findings.orphan_session_models),
            integrity::joined_values_or_count(
                &findings.orphan_session_models,
                inputs.session_model_binding_count,
            ),
        ),
        integrity::doctor_check(
            integrity::SESSION_MODEL_BINDINGS_REFERENCE_REGISTERED_MODELS,
            integrity::status_from_findings(&findings.unknown_session_models),
            integrity::joined_values_or_count(
                &findings.unknown_session_models,
                inputs.session_model_binding_count,
            ),
        ),
        integrity::doctor_check(
            integrity::HISTORY_SESSION_REFERENCES,
            integrity::status_from_findings(&findings.orphan_history),
            integrity::joined_values_or_count(&findings.orphan_history, inputs.history_count),
        ),
        integrity::doctor_check(
            integrity::GRANTED_APPROVALS_MAP_TO_KNOWN_TOOLS,
            integrity::status_from_findings(&findings.unknown_granted),
            integrity::joined_values_or_count(
                &findings.unknown_granted,
                findings.total_granted_approvals,
            ),
        ),
        integrity::doctor_check(
            integrity::PENDING_APPROVALS_MAP_TO_KNOWN_TOOLS,
            integrity::status_from_findings(&findings.unknown_pending),
            integrity::joined_values_or_count(
                &findings.unknown_pending,
                findings.total_pending_approvals,
            ),
        ),
        integrity::doctor_check(
            integrity::MEMORY_IDS_UNIQUE,
            integrity::status_from_findings(&findings.duplicate_memories),
            integrity::joined_values_or_count(
                &findings.duplicate_memories,
                inputs.raw_memory_count,
            ),
        ),
        integrity::doctor_check(
            integrity::TOPIC_SESSIONS_CARRY_TRANSCRIPT_PROVENANCE,
            if findings
                .topic_sessions_missing_transcript_provenance
                .is_empty()
            {
                DoctorStatus::Ok
            } else {
                DoctorStatus::Warn
            },
            if findings
                .topic_sessions_missing_transcript_provenance
                .is_empty()
            {
                format!(
                    "{}/{} active topic sessions",
                    findings.active_topic_sessions_with_transcript_provenance,
                    findings.active_topic_session_count,
                )
            } else {
                format!(
                    "missing transcript provenance for {}",
                    findings
                        .topic_sessions_missing_transcript_provenance
                        .join(", ")
                )
            },
        ),
        integrity::doctor_check(
            integrity::RUNTIME_SNAPSHOT_ROUNDTRIP,
            if findings.snapshot_roundtrip {
                DoctorStatus::Ok
            } else {
                DoctorStatus::Fail
            },
            if findings.snapshot_roundtrip {
                "serialize+parse stable".into()
            } else {
                "snapshot failed serde roundtrip".into()
            },
        ),
        integrity::doctor_check(
            integrity::ACTIVE_SESSION_EXPORT_ROUNDTRIP,
            if findings.active_export_serializable {
                DoctorStatus::Ok
            } else {
                DoctorStatus::Fail
            },
            if findings.active_export_serializable {
                format!("{} export serializable", inputs.active_session_id)
            } else {
                format!("{} export failed roundtrip", inputs.active_session_id)
            },
        ),
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn sample_findings() -> RuntimeStateFindings {
        RuntimeStateFindings {
            known_session_ids: HashSet::new(),
            active_registered: true,
            active_session_exists: true,
            active_session_archived: false,
            duplicate_sessions: vec![],
            orphan_history: vec![],
            unknown_granted: vec![],
            unknown_pending: vec![],
            duplicate_session_models: vec![],
            orphan_session_models: vec![],
            unknown_session_models: vec![],
            duplicate_memories: vec![],
            topic_sessions_missing_transcript_provenance: vec![],
            active_topic_session_count: 2,
            active_topic_sessions_with_transcript_provenance: 2,
            snapshot_roundtrip: true,
            active_export_serializable: true,
            total_granted_approvals: 0,
            total_pending_approvals: 0,
        }
    }

    fn active_model() -> ModelRef {
        ModelRef {
            provider: "demo".into(),
            model: "demo-chat".into(),
        }
    }

    fn check<'a>(checks: &'a [DoctorCheck], name: &str) -> &'a DoctorCheck {
        checks
            .iter()
            .find(|check| check.name == name)
            .expect("expected named runtime-state check")
    }

    #[test]
    fn build_runtime_state_checks_uses_count_summaries_for_clean_findings() {
        let checks = build_runtime_state_checks(
            &sample_findings(),
            RuntimeStateCheckInputs {
                active_model: &active_model(),
                active_session_id: "session-main",
                raw_session_count: 3,
                raw_memory_count: 5,
                history_count: 7,
                session_model_binding_count: 2,
            },
        );

        assert_eq!(
            check(&checks, integrity::ACTIVE_MODEL_REGISTERED).detail,
            "demo/demo-chat"
        );
        assert_eq!(
            check(&checks, integrity::RAW_SESSION_IDS_UNIQUE).detail,
            "3"
        );
        assert_eq!(
            check(
                &checks,
                integrity::TOPIC_SESSIONS_CARRY_TRANSCRIPT_PROVENANCE
            )
            .detail,
            "2/2 active topic sessions"
        );
        assert_eq!(
            check(&checks, integrity::ACTIVE_SESSION_EXPORT_ROUNDTRIP).detail,
            "session-main export serializable"
        );
    }

    #[test]
    fn build_runtime_state_checks_surfaces_problem_details_from_findings() {
        let mut findings = sample_findings();
        findings.active_registered = false;
        findings.active_session_exists = false;
        findings.active_session_archived = true;
        findings.duplicate_sessions = vec!["session-main".into()];
        findings.unknown_pending = vec!["tool.exec".into()];
        findings.topic_sessions_missing_transcript_provenance =
            vec!["topic-1".into(), "topic-2".into()];
        findings.active_topic_session_count = 3;
        findings.active_topic_sessions_with_transcript_provenance = 1;
        findings.snapshot_roundtrip = false;
        findings.active_export_serializable = false;
        findings.total_pending_approvals = 1;

        let checks = build_runtime_state_checks(
            &findings,
            RuntimeStateCheckInputs {
                active_model: &active_model(),
                active_session_id: "session-main",
                raw_session_count: 3,
                raw_memory_count: 5,
                history_count: 7,
                session_model_binding_count: 2,
            },
        );

        let active_model = check(&checks, integrity::ACTIVE_MODEL_REGISTERED);
        assert_eq!(active_model.status, DoctorStatus::Fail);
        assert_eq!(active_model.detail, "missing demo/demo-chat");

        let topic_provenance = check(
            &checks,
            integrity::TOPIC_SESSIONS_CARRY_TRANSCRIPT_PROVENANCE,
        );
        assert_eq!(topic_provenance.status, DoctorStatus::Warn);
        assert_eq!(
            topic_provenance.detail,
            "missing transcript provenance for topic-1, topic-2"
        );

        let export_roundtrip = check(&checks, integrity::ACTIVE_SESSION_EXPORT_ROUNDTRIP);
        assert_eq!(export_roundtrip.status, DoctorStatus::Fail);
        assert_eq!(
            export_roundtrip.detail,
            "session-main export failed roundtrip"
        );
    }
}

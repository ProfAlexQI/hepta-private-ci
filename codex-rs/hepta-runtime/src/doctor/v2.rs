use hepta_core::DoctorCheckOutcome;
use hepta_core::DoctorReportContext;
use hepta_core::DoctorReportV2;
use hepta_core::DoctorRuntimeMetrics;
use hepta_core::ModelRef;

pub(super) fn assemble_report(
    active_model: ModelRef,
    active_session_id: String,
    checks: Vec<DoctorCheckOutcome>,
    observed_at_unix_ms: u64,
    runtime_metrics: DoctorRuntimeMetrics,
) -> DoctorReportV2 {
    DoctorReportV2::from_checks(
        report_context(&active_model, &active_session_id, observed_at_unix_ms),
        checks,
    )
    .with_runtime(runtime_metrics)
}

fn report_context(
    active_model: &ModelRef,
    active_session_id: &str,
    observed_at_unix_ms: u64,
) -> DoctorReportContext {
    DoctorReportContext {
        observed_at_unix_ms,
        scope: "runtime".into(),
        active_model: Some(active_model.clone()),
        active_session_id: Some(active_session_id.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use hepta_core::DoctorArea;
    use hepta_core::DoctorCheckOutcome;
    use hepta_core::DoctorOwner;
    use hepta_core::DoctorRuntimeMetrics;
    use hepta_core::DoctorStatus as CoreDoctorStatus;
    use hepta_core::ModelRef;

    use super::*;

    #[test]
    fn assembles_core_v2_report_from_native_doctor_outcomes() {
        let adapted = assemble_report(
            ModelRef {
                provider: "demo".into(),
                model: "demo-chat".into(),
            },
            "session-main".into(),
            vec![
                DoctorCheckOutcome {
                    id: "provider_probe.mock-ollama.local-chat".into(),
                    area: DoctorArea::ProviderProbe,
                    owner: DoctorOwner {
                        component: "hepta-runtime".into(),
                        responsibility: "provider probing".into(),
                    },
                    status: CoreDoctorStatus::Fail,
                    summary: "provider probe mock-ollama via mock-ollama/local-chat".into(),
                    detail: "connection refused".into(),
                    remediation: Some(
                        "check connectivity and credentials for mock-ollama/local-chat on provider mock-ollama"
                            .into(),
                    ),
                },
                DoctorCheckOutcome {
                    id: "runtime_snapshot.roundtrip".into(),
                    area: DoctorArea::RuntimeSnapshot,
                    owner: DoctorOwner {
                        component: "hepta-runtime".into(),
                        responsibility: "runtime snapshot".into(),
                    },
                    status: CoreDoctorStatus::Warn,
                    summary: "runtime snapshot roundtrip".into(),
                    detail: "serde drift detected".into(),
                    remediation: Some(
                        "update runtime snapshot serde handling so serialize and parse stay stable"
                            .into(),
                    ),
                },
                DoctorCheckOutcome {
                    id: "export_import.active_session_roundtrip".into(),
                    area: DoctorArea::ExportImport,
                    owner: DoctorOwner {
                        component: "hepta-runtime".into(),
                        responsibility: "session export".into(),
                    },
                    status: CoreDoctorStatus::Ok,
                    summary: "active session export roundtrip".into(),
                    detail: "session-main export serializable".into(),
                    remediation: None,
                },
            ],
            123,
            DoctorRuntimeMetrics {
                registered_providers: 2,
                registered_tools: 4,
                sessions: 3,
                raw_session_records: 3,
                memories: 5,
                history_entries: 7,
                active_session_pending_approvals: 1,
                approval_scoped_sessions: 2,
                topic_sessions: 3,
                topic_graph_edges: 1,
                active_topic_sessions: 2,
                active_topic_sessions_with_transcript_provenance: 1,
                active_topic_sessions_missing_transcript_provenance: 1,
                active_session_recall_transcript_evidence_spans: 2,
                active_session_recall_omitted_items: 3,
                active_session_intuition_transcript_evidence_spans: 2,
                active_session_intuition_foreground_topic_sessions: 1,
            },
        );

        assert_eq!(adapted.context.scope, "runtime");
        assert_eq!(adapted.context.observed_at_unix_ms, 123);
        assert_eq!(adapted.overall_status, CoreDoctorStatus::Fail);
        assert_eq!(adapted.counts.ok, 1);
        assert_eq!(adapted.counts.warn, 1);
        assert_eq!(adapted.counts.fail, 1);
        let runtime = adapted.runtime.expect("runtime metrics should be present");
        assert_eq!(runtime.sessions, 3);
        assert_eq!(runtime.topic_sessions, 3);
        assert_eq!(runtime.topic_graph_edges, 1);
        assert_eq!(runtime.active_topic_sessions, 2);
        assert_eq!(runtime.active_topic_sessions_with_transcript_provenance, 1);
        assert_eq!(
            runtime.active_topic_sessions_missing_transcript_provenance,
            1
        );
        assert_eq!(runtime.active_session_recall_transcript_evidence_spans, 2);
        assert_eq!(runtime.active_session_recall_omitted_items, 3);
        assert_eq!(
            runtime.active_session_intuition_transcript_evidence_spans,
            2
        );
        assert_eq!(
            runtime.active_session_intuition_foreground_topic_sessions,
            1
        );

        let provider = adapted
            .checks
            .iter()
            .find(|check| check.id == "provider_probe.mock-ollama.local-chat")
            .expect("provider probe should be adapted into a stable v2 check");
        assert_eq!(provider.area, DoctorArea::ProviderProbe);
        assert_eq!(provider.status, CoreDoctorStatus::Fail);
        assert!(
            provider
                .remediation
                .as_ref()
                .expect("failing provider checks should suggest remediation")
                .contains("mock-ollama")
        );

        let snapshot = adapted
            .checks
            .iter()
            .find(|check| check.id == "runtime_snapshot.roundtrip")
            .expect("snapshot roundtrip check should keep a stable id");
        assert_eq!(snapshot.area, DoctorArea::RuntimeSnapshot);
        assert_eq!(snapshot.status, CoreDoctorStatus::Warn);
    }
}

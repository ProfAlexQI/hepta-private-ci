use hepta_core::DoctorCheckOutcome;
use hepta_core::ModelRef;

use super::DoctorCheck;
use super::DoctorProviderProbe;
use super::DoctorStatus;
use super::check_outcomes;
use super::report_outputs;
use super::runtime_metrics::DoctorRuntimeStats;

pub(super) struct DoctorReportBundle {
    active_model: ModelRef,
    active_session_id: String,
    stats: DoctorRuntimeStats,
    overall_status: DoctorStatus,
    provider_probes: Vec<DoctorProviderProbe>,
    integrity_checks: Vec<DoctorCheck>,
    v2_checks: Vec<DoctorCheckOutcome>,
}

pub(super) fn assemble_report_bundle(
    active_model: ModelRef,
    active_session_id: String,
    stats: DoctorRuntimeStats,
    provider_probes: Vec<DoctorProviderProbe>,
    integrity_checks: Vec<DoctorCheck>,
) -> DoctorReportBundle {
    let outcomes = check_outcomes::assemble_doctor_outcomes(&provider_probes, &integrity_checks);

    DoctorReportBundle {
        active_model,
        active_session_id,
        stats,
        overall_status: outcomes.overall_status,
        provider_probes,
        integrity_checks,
        v2_checks: outcomes.v2_checks,
    }
}

impl DoctorReportBundle {
    pub(super) fn into_report(self) -> super::DoctorReport {
        report_outputs::build_legacy_report(
            self.overall_status,
            self.active_model,
            self.active_session_id,
            self.stats,
            self.provider_probes,
            self.integrity_checks,
        )
    }

    pub(super) fn into_v2_report(self, observed_at_unix_ms: u64) -> hepta_core::DoctorReportV2 {
        report_outputs::build_v2_report(
            self.active_model,
            self.active_session_id,
            self.stats,
            self.v2_checks,
            observed_at_unix_ms,
        )
    }
}

#[cfg(test)]
mod tests {
    use hepta_core::DoctorArea;
    use hepta_core::DoctorStatus as CoreDoctorStatus;

    use super::*;

    fn stats() -> DoctorRuntimeStats {
        DoctorRuntimeStats {
            registered_providers: 2,
            registered_tools: 4,
            sessions: 3,
            raw_session_records: 3,
            memories: 5,
            history_entries: 7,
            active_session_pending_approvals: 1,
            approval_scoped_sessions: 2,
            total_topic_sessions: 3,
            total_topic_graph_edges: 1,
            active_topic_sessions: 2,
            active_topic_sessions_with_transcript_provenance: 1,
            active_topic_sessions_missing_transcript_provenance: 1,
            active_session_recall_transcript_evidence_spans: 2,
            active_session_recall_omitted_items: 3,
            active_session_intuition_transcript_evidence_spans: 2,
            active_session_intuition_foreground_topic_sessions: 1,
        }
    }

    fn active_model() -> ModelRef {
        ModelRef {
            provider: "demo".into(),
            model: "demo-chat".into(),
        }
    }

    #[test]
    fn bundle_keeps_legacy_report_fields_stable() {
        let report = assemble_report_bundle(
            active_model(),
            "session-main".into(),
            stats(),
            vec![DoctorProviderProbe {
                provider_name: "demo".into(),
                model: Some(active_model()),
                status: DoctorStatus::Ok,
                detail: "pong".into(),
            }],
            vec![DoctorCheck {
                name: "runtime snapshot roundtrip".into(),
                status: DoctorStatus::Ok,
                detail: "serialize+parse stable".into(),
            }],
        )
        .into_report();

        assert_eq!(report.overall_status, DoctorStatus::Ok);
        assert_eq!(report.active_session_id, "session-main");
        assert_eq!(report.registered_providers, 2);
        assert_eq!(report.registered_tools, 4);
        assert_eq!(report.sessions, 3);
        assert_eq!(report.raw_session_records, 3);
        assert_eq!(report.memories, 5);
        assert_eq!(report.history_entries, 7);
        assert_eq!(report.active_session_pending_approvals, 1);
        assert_eq!(report.approval_scoped_sessions, 2);
        assert_eq!(report.total_topic_sessions, 3);
        assert_eq!(report.total_topic_graph_edges, 1);
        assert_eq!(report.active_topic_sessions, 2);
        assert_eq!(report.active_topic_sessions_with_transcript_provenance, 1);
        assert_eq!(
            report.active_topic_sessions_missing_transcript_provenance,
            1
        );
        assert_eq!(report.active_session_recall_transcript_evidence_spans, 2);
        assert_eq!(report.active_session_recall_omitted_items, 3);
        assert_eq!(report.active_session_intuition_transcript_evidence_spans, 2);
        assert_eq!(report.active_session_intuition_foreground_topic_sessions, 1);
        assert_eq!(report.provider_probes.len(), 1);
        assert_eq!(report.integrity_checks.len(), 1);
    }

    #[test]
    fn bundle_reuses_canonical_outcomes_for_v2_report() {
        let report = assemble_report_bundle(
            active_model(),
            "session-main".into(),
            stats(),
            vec![DoctorProviderProbe {
                provider_name: "demo".into(),
                model: Some(active_model()),
                status: DoctorStatus::Fail,
                detail: "connection refused".into(),
            }],
            vec![DoctorCheck {
                name: "runtime snapshot roundtrip".into(),
                status: DoctorStatus::Warn,
                detail: "serde drift detected".into(),
            }],
        )
        .into_v2_report(123);

        assert_eq!(report.context.scope, "runtime");
        assert_eq!(report.context.observed_at_unix_ms, 123);
        assert_eq!(report.overall_status, CoreDoctorStatus::Fail);
        assert_eq!(report.checks.len(), 2);
        let runtime = report.runtime.expect("runtime metrics should be present");
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

        let provider = report
            .checks
            .iter()
            .find(|check| check.id == "provider_probe.demo.demo-chat")
            .expect("provider probe should keep a stable v2 id");
        assert_eq!(provider.area, DoctorArea::ProviderProbe);
        assert_eq!(provider.status, CoreDoctorStatus::Fail);

        let snapshot = report
            .checks
            .iter()
            .find(|check| check.id == "runtime_snapshot.roundtrip")
            .expect("integrity checks should keep stable metadata in v2");
        assert_eq!(snapshot.area, DoctorArea::RuntimeSnapshot);
        assert_eq!(snapshot.status, CoreDoctorStatus::Warn);
    }

    #[test]
    fn bundle_preserves_warn_status_for_legacy_report() {
        let report = assemble_report_bundle(
            active_model(),
            "session-main".into(),
            stats(),
            vec![DoctorProviderProbe {
                provider_name: "demo".into(),
                model: Some(active_model()),
                status: DoctorStatus::Ok,
                detail: "pong".into(),
            }],
            vec![DoctorCheck {
                name: "runtime snapshot roundtrip".into(),
                status: DoctorStatus::Warn,
                detail: "serde drift detected".into(),
            }],
        )
        .into_report();

        assert_eq!(report.overall_status, DoctorStatus::Warn);
    }
}

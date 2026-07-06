use hepta_core::DoctorReportV2;
use hepta_core::HeptaError;

use crate::RuntimeKernel;

mod check_outcomes;
mod checks;
mod collector;
mod contracts;
mod event_log_integrity;
mod integrity;
mod integrity_catalog;
mod intelligence_eval_gate;
mod probes;
mod provider_probe;
mod report_bundle;
mod report_input_collectors;
mod report_inputs;
mod report_outputs;
mod runtime_metrics;
mod runtime_state_check_bundle;
mod runtime_state_findings;
mod runtime_state_integrity;
mod state_checks;
mod status_rollup;
mod v2;

pub use contracts::DoctorCheck;
pub use contracts::DoctorProviderProbe;
pub use contracts::DoctorReport;
pub use contracts::DoctorStatus;

impl RuntimeKernel {
    pub async fn doctor_report(&self) -> Result<DoctorReport, HeptaError> {
        collector::collect_doctor_report(self).await
    }

    pub async fn doctor_report_v2(&self) -> Result<DoctorReportV2, HeptaError> {
        collector::collect_doctor_report_v2(self).await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use hepta_core::DoctorArea;
    use hepta_core::DoctorStatus as CoreDoctorStatus;

    use super::*;

    #[tokio::test]
    async fn doctor_report_v2_exposes_runtime_context_and_stable_ids() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello doctor v2")
            .await
            .expect("plain turn should succeed");
        runtime
            .route_topics("session-main", Some("hello doctor v2"), 4, 4, 4, 1)
            .expect("topic route should succeed");

        let report = runtime
            .doctor_report_v2()
            .await
            .expect("doctor v2 report should succeed");

        assert_eq!(report.context.scope, "runtime");
        assert_eq!(
            report.context.active_session_id.as_deref(),
            Some("session-main")
        );
        let runtime_metrics = report
            .runtime
            .as_ref()
            .expect("runtime metrics should be present");
        assert_eq!(runtime_metrics.sessions, 1);
        assert_eq!(runtime_metrics.topic_sessions, 1);
        assert_eq!(runtime_metrics.topic_graph_edges, 0);
        assert_eq!(runtime_metrics.active_topic_sessions, 1);
        assert_eq!(
            runtime_metrics.active_topic_sessions_with_transcript_provenance,
            1
        );
        assert_eq!(
            runtime_metrics.active_topic_sessions_missing_transcript_provenance,
            0
        );
        assert!(runtime_metrics.active_session_recall_transcript_evidence_spans > 0);
        assert!(runtime_metrics.active_session_intuition_transcript_evidence_spans > 0);
        assert_eq!(
            runtime_metrics.active_session_intuition_foreground_topic_sessions,
            1
        );
        assert_eq!(report.overall_status, CoreDoctorStatus::Ok);
        assert!(report.checks.iter().any(|check| {
            check.id == "provider_probe.demo.demo-chat"
                && check.area == DoctorArea::ProviderProbe
                && check.status == CoreDoctorStatus::Ok
        }));
        assert!(report.checks.iter().any(|check| {
            check.id == "runtime_snapshot.roundtrip"
                && check.area == DoctorArea::RuntimeSnapshot
                && check.status == CoreDoctorStatus::Ok
        }));
        assert!(report.checks.iter().any(|check| {
            check.id == "events.session_refs_known"
                && check.area == DoctorArea::EventStream
                && check.status == CoreDoctorStatus::Ok
        }));
        assert!(report.checks.iter().any(|check| {
            check.id == "events.monotonic_timestamps"
                && check.area == DoctorArea::EventStream
                && check.status == CoreDoctorStatus::Ok
        }));
        assert!(report.checks.iter().any(|check| {
            check.id == "runtime_snapshot.topic_sessions_transcript_provenance"
                && check.area == DoctorArea::RuntimeSnapshot
                && check.status == CoreDoctorStatus::Ok
        }));
    }

    #[tokio::test]
    async fn doctor_report_v2_native_assembly_matches_legacy_totals() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello doctor v2 alignment")
            .await
            .expect("plain turn should succeed");
        runtime
            .route_topics(
                "session-main",
                Some("hello doctor v2 alignment"),
                4,
                4,
                4,
                1,
            )
            .expect("topic route should succeed");

        let legacy = runtime
            .doctor_report()
            .await
            .expect("legacy doctor report should succeed");
        let v2 = runtime
            .doctor_report_v2()
            .await
            .expect("doctor v2 report should succeed");

        assert_eq!(v2.overall_status, legacy.overall_status.into());
        assert_eq!(
            v2.checks.len(),
            legacy.provider_probes.len() + legacy.integrity_checks.len()
        );
        assert_eq!(
            v2.counts.ok + v2.counts.warn + v2.counts.fail,
            v2.checks.len()
        );
        let runtime_metrics = v2.runtime.expect("runtime metrics should be present");
        assert_eq!(runtime_metrics.sessions, legacy.sessions);
        assert_eq!(
            runtime_metrics.raw_session_records,
            legacy.raw_session_records
        );
        assert_eq!(runtime_metrics.memories, legacy.memories);
        assert_eq!(runtime_metrics.history_entries, legacy.history_entries);
        assert_eq!(
            runtime_metrics.active_session_pending_approvals,
            legacy.active_session_pending_approvals
        );
        assert_eq!(
            runtime_metrics.approval_scoped_sessions,
            legacy.approval_scoped_sessions
        );
        assert_eq!(runtime_metrics.topic_sessions, legacy.total_topic_sessions);
        assert_eq!(
            runtime_metrics.topic_graph_edges,
            legacy.total_topic_graph_edges
        );
        assert_eq!(
            runtime_metrics.active_topic_sessions,
            legacy.active_topic_sessions
        );
        assert_eq!(
            runtime_metrics.active_topic_sessions_with_transcript_provenance,
            legacy.active_topic_sessions_with_transcript_provenance
        );
        assert_eq!(
            runtime_metrics.active_topic_sessions_missing_transcript_provenance,
            legacy.active_topic_sessions_missing_transcript_provenance
        );
        assert_eq!(
            runtime_metrics.active_session_recall_transcript_evidence_spans,
            legacy.active_session_recall_transcript_evidence_spans
        );
        assert_eq!(
            runtime_metrics.active_session_recall_omitted_items,
            legacy.active_session_recall_omitted_items
        );
        assert_eq!(
            runtime_metrics.active_session_intuition_transcript_evidence_spans,
            legacy.active_session_intuition_transcript_evidence_spans
        );
        assert_eq!(
            runtime_metrics.active_session_intuition_foreground_topic_sessions,
            legacy.active_session_intuition_foreground_topic_sessions
        );
    }

    #[tokio::test]
    async fn doctor_runtime_metrics_match_lightweight_provenance_overview() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello doctor provenance alignment")
            .await
            .expect("plain turn should succeed");
        runtime
            .route_topics(
                "session-main",
                Some("hello doctor provenance alignment"),
                4,
                4,
                4,
                1,
            )
            .expect("topic route should succeed");

        let overview = runtime
            .provenance_overview("session-main")
            .expect("provenance overview should succeed");
        let report = runtime
            .doctor_report_v2()
            .await
            .expect("doctor v2 report should succeed");
        let runtime_metrics = report.runtime.expect("runtime metrics should be present");

        assert_eq!(
            runtime_metrics.active_topic_sessions,
            overview.active_topic_sessions
        );
        assert_eq!(
            runtime_metrics.active_topic_sessions_with_transcript_provenance,
            overview.active_topic_sessions_with_transcript_provenance
        );
        assert_eq!(
            runtime_metrics.active_topic_sessions_missing_transcript_provenance,
            overview.active_topic_sessions_missing_transcript_provenance
        );
        assert_eq!(
            runtime_metrics.active_session_recall_transcript_evidence_spans,
            overview.recall_transcript_evidence_spans
        );
        assert_eq!(
            runtime_metrics.active_session_recall_omitted_items,
            overview.recall_omitted_items
        );
        assert_eq!(
            runtime_metrics.active_session_intuition_transcript_evidence_spans,
            overview.intuition_transcript_evidence_spans
        );
        assert_eq!(
            runtime_metrics.active_session_intuition_foreground_topic_sessions,
            overview.intuition_foreground_topic_sessions
        );
    }

    #[tokio::test]
    async fn doctor_report_keeps_the_extracted_integrity_catalog_stable() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello extracted integrity catalog")
            .await
            .expect("plain turn should succeed");

        let report = runtime
            .doctor_report()
            .await
            .expect("legacy doctor report should succeed");

        let names = report
            .integrity_checks
            .iter()
            .map(|check| check.name.as_str())
            .collect::<Vec<_>>();
        let unique_names = names.iter().copied().collect::<HashSet<_>>();

        assert_eq!(names.len(), unique_names.len());
        assert!(names.contains(&integrity::ACTIVE_MODEL_REGISTERED));
        assert!(names.contains(&integrity::ACTIVE_SESSION_EXISTS));
        assert!(names.contains(&integrity::ACTIVE_SESSION_ARCHIVED));
        assert!(names.contains(&integrity::RAW_SESSION_IDS_UNIQUE));
        assert!(names.contains(&integrity::SESSION_MODEL_BINDINGS_UNIQUE));
        assert!(names.contains(&integrity::SESSION_MODEL_BINDINGS_REFERENCE_KNOWN_SESSIONS));
        assert!(names.contains(&integrity::SESSION_MODEL_BINDINGS_REFERENCE_REGISTERED_MODELS));
        assert!(names.contains(&integrity::HISTORY_SESSION_REFERENCES));
        assert!(names.contains(&event_log_integrity::SESSION_REFS_KNOWN));
        assert!(names.contains(&event_log_integrity::MONOTONIC_TIMESTAMPS));
        assert!(names.contains(&integrity::GRANTED_APPROVALS_MAP_TO_KNOWN_TOOLS));
        assert!(names.contains(&integrity::PENDING_APPROVALS_MAP_TO_KNOWN_TOOLS));
        assert!(names.contains(&integrity::MEMORY_IDS_UNIQUE));
        assert!(names.contains(&integrity::TOPIC_SESSIONS_CARRY_TRANSCRIPT_PROVENANCE));
        assert!(names.contains(&integrity::RUNTIME_SNAPSHOT_ROUNDTRIP));
        assert!(names.contains(&integrity::ACTIVE_SESSION_EXPORT_ROUNDTRIP));
    }
}

use serde::Deserialize;
use serde::Serialize;

use crate::model::ModelRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DoctorStatus {
    Ok,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DoctorArea {
    ProviderProbe,
    RuntimeSnapshot,
    Intelligence,
    SessionStore,
    ExportImport,
    EventStream,
    Registry,
    Approval,
    WriteSafety,
    Config,
    Gateway,
    Plugin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorOwner {
    pub component: String,
    pub responsibility: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorCheckOutcome {
    pub id: String,
    pub area: DoctorArea,
    pub owner: DoctorOwner,
    pub status: DoctorStatus,
    pub summary: String,
    pub detail: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorSummaryCounts {
    pub ok: usize,
    pub warn: usize,
    pub fail: usize,
}

impl DoctorSummaryCounts {
    pub fn from_checks(checks: &[DoctorCheckOutcome]) -> Self {
        let mut counts = Self::default();
        for check in checks {
            match check.status {
                DoctorStatus::Ok => counts.ok += 1,
                DoctorStatus::Warn => counts.warn += 1,
                DoctorStatus::Fail => counts.fail += 1,
            }
        }
        counts
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorReportContext {
    pub observed_at_unix_ms: u64,
    pub scope: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_model: Option<ModelRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_session_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorRuntimeMetrics {
    pub registered_providers: usize,
    pub registered_tools: usize,
    pub sessions: usize,
    pub raw_session_records: usize,
    pub memories: usize,
    pub history_entries: usize,
    pub active_session_pending_approvals: usize,
    pub approval_scoped_sessions: usize,
    pub topic_sessions: usize,
    pub topic_graph_edges: usize,
    pub active_topic_sessions: usize,
    pub active_topic_sessions_with_transcript_provenance: usize,
    pub active_topic_sessions_missing_transcript_provenance: usize,
    pub active_session_recall_transcript_evidence_spans: usize,
    pub active_session_recall_omitted_items: usize,
    pub active_session_intuition_transcript_evidence_spans: usize,
    pub active_session_intuition_foreground_topic_sessions: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorReportV2 {
    pub overall_status: DoctorStatus,
    pub context: DoctorReportContext,
    pub counts: DoctorSummaryCounts,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime: Option<DoctorRuntimeMetrics>,
    pub checks: Vec<DoctorCheckOutcome>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DoctorDryRunCheckKind {
    DoctorSnapshot,
    OnboardingPlan,
    UpdatePlan,
    ConfigRepairPreview,
    PluginRepairPreview,
    ProxyValidate,
    SystemEnvironment,
    GitEnvironment,
    TerminalEnvironment,
    TerminalTitle,
    StartupWarnings,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorDryRunCheck {
    pub id: String,
    pub kind: DoctorDryRunCheckKind,
    pub passed: bool,
    pub dry_run: bool,
    pub config_written: bool,
    pub package_manager_invoked: bool,
    pub plugin_installed: bool,
    pub listener_started: bool,
    pub credential_value_read: bool,
    pub external_network_read: bool,
}

impl DoctorDryRunCheck {
    pub fn new(id: impl Into<String>, kind: DoctorDryRunCheckKind) -> Self {
        Self {
            id: id.into(),
            kind,
            passed: true,
            dry_run: true,
            config_written: false,
            package_manager_invoked: false,
            plugin_installed: false,
            listener_started: false,
            credential_value_read: false,
            external_network_read: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorDryRunReport {
    pub check_count: usize,
    pub checks_passed: usize,
    pub doctor_snapshot_check: bool,
    pub onboarding_plan_check: bool,
    pub update_plan_check: bool,
    pub config_repair_preview_check: bool,
    pub plugin_repair_preview_check: bool,
    pub proxy_validate_check: bool,
    pub system_environment_check: bool,
    pub git_environment_check: bool,
    pub terminal_environment_check: bool,
    pub terminal_title_check: bool,
    pub startup_warning_check: bool,
    pub config_written: bool,
    pub package_manager_invoked: bool,
    pub plugin_installed: bool,
    pub listener_started: bool,
    pub credential_value_read: bool,
    pub external_network_read: bool,
    pub raw_environment_value_exposed: bool,
    pub dry_run_checks_ready: bool,
    pub checks: Vec<DoctorDryRunCheck>,
}

impl DoctorDryRunReport {
    pub fn native_default() -> Self {
        Self::from_checks(vec![
            DoctorDryRunCheck::new(
                "doctor-snapshot-local-only",
                DoctorDryRunCheckKind::DoctorSnapshot,
            ),
            DoctorDryRunCheck::new(
                "onboarding-plan-no-write",
                DoctorDryRunCheckKind::OnboardingPlan,
            ),
            DoctorDryRunCheck::new(
                "update-plan-no-package-manager",
                DoctorDryRunCheckKind::UpdatePlan,
            ),
            DoctorDryRunCheck::new(
                "config-repair-preview-no-write",
                DoctorDryRunCheckKind::ConfigRepairPreview,
            ),
            DoctorDryRunCheck::new(
                "plugin-repair-preview-no-install",
                DoctorDryRunCheckKind::PluginRepairPreview,
            ),
            DoctorDryRunCheck::new(
                "proxy-validate-no-listener",
                DoctorDryRunCheckKind::ProxyValidate,
            ),
            DoctorDryRunCheck::new(
                "system-environment-redacted-local-only",
                DoctorDryRunCheckKind::SystemEnvironment,
            ),
            DoctorDryRunCheck::new(
                "git-environment-redacted-local-only",
                DoctorDryRunCheckKind::GitEnvironment,
            ),
            DoctorDryRunCheck::new(
                "terminal-environment-redacted-local-only",
                DoctorDryRunCheckKind::TerminalEnvironment,
            ),
            DoctorDryRunCheck::new(
                "terminal-title-redacted-local-only",
                DoctorDryRunCheckKind::TerminalTitle,
            ),
            DoctorDryRunCheck::new(
                "startup-warning-count-redacted-local-only",
                DoctorDryRunCheckKind::StartupWarnings,
            ),
        ])
    }

    pub fn from_checks(checks: Vec<DoctorDryRunCheck>) -> Self {
        let check_count = checks.len();
        let checks_passed = checks
            .iter()
            .filter(|check| check.passed && check.dry_run)
            .count();
        let has_kind = |kind: DoctorDryRunCheckKind| {
            checks
                .iter()
                .any(|check| check.passed && check.dry_run && check.kind == kind)
        };
        let config_written = checks.iter().any(|check| check.config_written);
        let package_manager_invoked = checks.iter().any(|check| check.package_manager_invoked);
        let plugin_installed = checks.iter().any(|check| check.plugin_installed);
        let listener_started = checks.iter().any(|check| check.listener_started);
        let credential_value_read = checks.iter().any(|check| check.credential_value_read);
        let external_network_read = checks.iter().any(|check| check.external_network_read);
        let doctor_snapshot_check = has_kind(DoctorDryRunCheckKind::DoctorSnapshot);
        let onboarding_plan_check = has_kind(DoctorDryRunCheckKind::OnboardingPlan);
        let update_plan_check = has_kind(DoctorDryRunCheckKind::UpdatePlan);
        let config_repair_preview_check = has_kind(DoctorDryRunCheckKind::ConfigRepairPreview);
        let plugin_repair_preview_check = has_kind(DoctorDryRunCheckKind::PluginRepairPreview);
        let proxy_validate_check = has_kind(DoctorDryRunCheckKind::ProxyValidate);
        let system_environment_check = has_kind(DoctorDryRunCheckKind::SystemEnvironment);
        let git_environment_check = has_kind(DoctorDryRunCheckKind::GitEnvironment);
        let terminal_environment_check = has_kind(DoctorDryRunCheckKind::TerminalEnvironment);
        let terminal_title_check = has_kind(DoctorDryRunCheckKind::TerminalTitle);
        let startup_warning_check = has_kind(DoctorDryRunCheckKind::StartupWarnings);
        let raw_environment_value_exposed = false;
        let dry_run_checks_ready = check_count > 0
            && check_count == checks_passed
            && doctor_snapshot_check
            && onboarding_plan_check
            && update_plan_check
            && config_repair_preview_check
            && plugin_repair_preview_check
            && proxy_validate_check
            && system_environment_check
            && git_environment_check
            && terminal_environment_check
            && terminal_title_check
            && startup_warning_check
            && !config_written
            && !package_manager_invoked
            && !plugin_installed
            && !listener_started
            && !credential_value_read
            && !external_network_read
            && !raw_environment_value_exposed;

        Self {
            check_count,
            checks_passed,
            doctor_snapshot_check,
            onboarding_plan_check,
            update_plan_check,
            config_repair_preview_check,
            plugin_repair_preview_check,
            proxy_validate_check,
            system_environment_check,
            git_environment_check,
            terminal_environment_check,
            terminal_title_check,
            startup_warning_check,
            config_written,
            package_manager_invoked,
            plugin_installed,
            listener_started,
            credential_value_read,
            external_network_read,
            raw_environment_value_exposed,
            dry_run_checks_ready,
            checks,
        }
    }
}

impl DoctorReportV2 {
    pub fn from_checks(context: DoctorReportContext, checks: Vec<DoctorCheckOutcome>) -> Self {
        let counts = DoctorSummaryCounts::from_checks(&checks);
        let overall_status = if counts.fail > 0 {
            DoctorStatus::Fail
        } else if counts.warn > 0 {
            DoctorStatus::Warn
        } else {
            DoctorStatus::Ok
        };

        Self {
            overall_status,
            context,
            counts,
            runtime: None,
            checks,
        }
    }

    pub fn with_runtime(mut self, runtime: DoctorRuntimeMetrics) -> Self {
        self.runtime = Some(runtime);
        self
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn owner() -> DoctorOwner {
        DoctorOwner {
            component: "hepta-runtime".into(),
            responsibility: "doctor aggregation".into(),
        }
    }

    fn context() -> DoctorReportContext {
        DoctorReportContext {
            observed_at_unix_ms: 123,
            scope: "runtime".into(),
            active_model: Some(ModelRef {
                provider: "demo".into(),
                model: "demo-model".into(),
            }),
            active_session_id: Some("session-1".into()),
        }
    }

    #[test]
    fn report_rolls_up_warning_and_failure_counts() {
        let checks = vec![
            DoctorCheckOutcome {
                id: "snapshot.roundtrip".into(),
                area: DoctorArea::RuntimeSnapshot,
                owner: owner(),
                status: DoctorStatus::Ok,
                summary: "runtime snapshot roundtrip".into(),
                detail: "serialize+parse stable".into(),
                remediation: None,
            },
            DoctorCheckOutcome {
                id: "events.monotonic".into(),
                area: DoctorArea::EventStream,
                owner: owner(),
                status: DoctorStatus::Warn,
                summary: "event timestamps monotonic".into(),
                detail: "one duplicate timestamp observed".into(),
                remediation: Some("sort events before emitting report".into()),
            },
            DoctorCheckOutcome {
                id: "registry.tools_present".into(),
                area: DoctorArea::Registry,
                owner: owner(),
                status: DoctorStatus::Fail,
                summary: "tool registry populated".into(),
                detail: "no tools registered".into(),
                remediation: Some("register builtin tools during runtime boot".into()),
            },
        ];

        let report = DoctorReportV2::from_checks(context(), checks);

        assert_eq!(report.overall_status, DoctorStatus::Fail);
        assert_eq!(report.counts.ok, 1);
        assert_eq!(report.counts.warn, 1);
        assert_eq!(report.counts.fail, 1);
    }

    #[test]
    fn report_roundtrips_through_json() {
        let report = DoctorReportV2::from_checks(
            context(),
            vec![DoctorCheckOutcome {
                id: "snapshot.roundtrip".into(),
                area: DoctorArea::RuntimeSnapshot,
                owner: owner(),
                status: DoctorStatus::Ok,
                summary: "runtime snapshot roundtrip".into(),
                detail: "serialize+parse stable".into(),
                remediation: None,
            }],
        )
        .with_runtime(DoctorRuntimeMetrics {
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
        });

        let json = serde_json::to_string(&report).expect("doctor report should serialize");
        let parsed: DoctorReportV2 =
            serde_json::from_str(&json).expect("doctor report should deserialize");

        assert_eq!(parsed, report);
    }

    #[test]
    fn report_defaults_runtime_metrics_to_none() {
        let report = DoctorReportV2::from_checks(context(), Vec::new());

        assert!(report.runtime.is_none());
    }

    #[test]
    fn report_serializes_with_stable_runtime_shape() {
        let report = DoctorReportV2::from_checks(
            context(),
            vec![DoctorCheckOutcome {
                id: "snapshot.roundtrip".into(),
                area: DoctorArea::RuntimeSnapshot,
                owner: owner(),
                status: DoctorStatus::Ok,
                summary: "runtime snapshot roundtrip".into(),
                detail: "serialize+parse stable".into(),
                remediation: None,
            }],
        )
        .with_runtime(DoctorRuntimeMetrics {
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
        });

        let value = serde_json::to_value(&report).expect("doctor report should serialize");

        assert_eq!(
            value,
            json!({
                "overall_status": "ok",
                "context": {
                    "observed_at_unix_ms": 123,
                    "scope": "runtime",
                    "active_model": {
                        "provider": "demo",
                        "model": "demo-model"
                    },
                    "active_session_id": "session-1"
                },
                "counts": {
                    "ok": 1,
                    "warn": 0,
                    "fail": 0
                },
                "runtime": {
                    "registered_providers": 2,
                    "registered_tools": 4,
                    "sessions": 3,
                    "raw_session_records": 3,
                    "memories": 5,
                    "history_entries": 7,
                    "active_session_pending_approvals": 1,
                    "approval_scoped_sessions": 2,
                    "topic_sessions": 3,
                    "topic_graph_edges": 1,
                    "active_topic_sessions": 2,
                    "active_topic_sessions_with_transcript_provenance": 1,
                    "active_topic_sessions_missing_transcript_provenance": 1,
                    "active_session_recall_transcript_evidence_spans": 2,
                    "active_session_recall_omitted_items": 3,
                    "active_session_intuition_transcript_evidence_spans": 2,
                    "active_session_intuition_foreground_topic_sessions": 1
                },
                "checks": [
                    {
                        "id": "snapshot.roundtrip",
                        "area": "runtime_snapshot",
                        "owner": {
                            "component": "hepta-runtime",
                            "responsibility": "doctor aggregation"
                        },
                        "status": "ok",
                        "summary": "runtime snapshot roundtrip",
                        "detail": "serialize+parse stable"
                    }
                ]
            })
        );
    }

    #[test]
    fn report_omits_runtime_field_when_absent() {
        let report = DoctorReportV2::from_checks(context(), Vec::new());
        let value = serde_json::to_value(&report).expect("doctor report should serialize");

        assert!(value.get("runtime").is_none());
    }

    #[test]
    fn doctor_onboarding_update_dry_run_checks_are_real_and_side_effect_free() {
        let report = DoctorDryRunReport::native_default();

        assert_eq!(report.check_count, 11);
        assert_eq!(report.checks_passed, report.check_count);
        assert!(report.doctor_snapshot_check);
        assert!(report.onboarding_plan_check);
        assert!(report.update_plan_check);
        assert!(report.config_repair_preview_check);
        assert!(report.plugin_repair_preview_check);
        assert!(report.proxy_validate_check);
        assert!(report.system_environment_check);
        assert!(report.git_environment_check);
        assert!(report.terminal_environment_check);
        assert!(report.terminal_title_check);
        assert!(report.startup_warning_check);
        assert!(!report.config_written);
        assert!(!report.package_manager_invoked);
        assert!(!report.plugin_installed);
        assert!(!report.listener_started);
        assert!(!report.credential_value_read);
        assert!(!report.external_network_read);
        assert!(!report.raw_environment_value_exposed);
        assert!(report.dry_run_checks_ready);
    }
}

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalProductionDomain {
    Provider,
    Channel,
    Device,
    Plugin,
    Worker,
    Scheduler,
    Release,
    Observability,
    Security,
    Recovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalProductionStatus {
    Verified,
    Pending,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalProductionEvidence {
    pub verified: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_at_unix_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalProductionEvidenceManifest {
    pub schema_version: u32,
    #[serde(default)]
    pub evidence: BTreeMap<String, ExternalProductionEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExternalProductionRequirement {
    pub id: &'static str,
    pub title: &'static str,
    pub domain: ExternalProductionDomain,
    pub destructive_or_external_side_effect: bool,
    pub requires_operator_approval: bool,
    pub local_control_plane_ready: bool,
    pub status: ExternalProductionStatus,
    pub gate: &'static str,
    pub remediation: &'static str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExternalProductionReadinessReport {
    pub product: &'static str,
    pub manifest_path: String,
    pub manifest_present: bool,
    pub requirement_count: usize,
    pub local_control_plane_ready_count: usize,
    pub verified_count: usize,
    pub pending_count: usize,
    pub blocked_count: usize,
    pub local_control_plane_coverage_percent: u8,
    pub external_verification_percent: u8,
    pub external_production_ready: bool,
    pub external_side_effects_require_operator_approval: bool,
    pub requirements: Vec<ExternalProductionRequirement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_error: Option<String>,
}

impl ExternalProductionReadinessReport {
    pub fn next_blockers(&self) -> Vec<&ExternalProductionRequirement> {
        self.requirements
            .iter()
            .filter(|requirement| requirement.status != ExternalProductionStatus::Verified)
            .collect()
    }
}

pub fn external_production_readiness_report(
    manifest_path: impl AsRef<Path>,
) -> ExternalProductionReadinessReport {
    let manifest_path = manifest_path.as_ref().to_path_buf();
    let (manifest_present, evidence, manifest_error) = load_manifest(&manifest_path);
    let requirements = external_production_requirement_registry()
        .into_iter()
        .map(|mut requirement| {
            if let Some(item) = evidence.get(requirement.id) {
                if item.verified {
                    requirement.status = ExternalProductionStatus::Verified;
                    requirement.evidence_ref = item.evidence_ref.clone();
                } else {
                    requirement.status = ExternalProductionStatus::Pending;
                    requirement.evidence_ref = item.evidence_ref.clone();
                }
            }
            requirement
        })
        .collect::<Vec<_>>();
    let requirement_count = requirements.len();
    let local_control_plane_ready_count = requirements
        .iter()
        .filter(|requirement| requirement.local_control_plane_ready)
        .count();
    let verified_count = requirements
        .iter()
        .filter(|requirement| requirement.status == ExternalProductionStatus::Verified)
        .count();
    let pending_count = requirements
        .iter()
        .filter(|requirement| requirement.status == ExternalProductionStatus::Pending)
        .count();
    let blocked_count = requirements
        .iter()
        .filter(|requirement| requirement.status == ExternalProductionStatus::Blocked)
        .count();
    ExternalProductionReadinessReport {
        product: "Hepta",
        manifest_path: manifest_path.to_string_lossy().to_string(),
        manifest_present,
        requirement_count,
        local_control_plane_ready_count,
        verified_count,
        pending_count,
        blocked_count,
        local_control_plane_coverage_percent: percent(
            local_control_plane_ready_count,
            requirement_count,
        ),
        external_verification_percent: percent(verified_count, requirement_count),
        external_production_ready: requirement_count > 0 && verified_count == requirement_count,
        external_side_effects_require_operator_approval: true,
        requirements,
        manifest_error,
    }
}

pub fn external_production_requirement_registry() -> Vec<ExternalProductionRequirement> {
    use ExternalProductionDomain::*;
    vec![
        requirement(
            "credentialed-provider-smoke",
            "Credentialed model/provider smoke with redacted telemetry",
            Provider,
            true,
            "HEPTA_EXTERNAL_PROVIDER_SMOKE=1",
            "run a credentialed provider smoke after explicit operator approval",
        ),
        requirement(
            "channel-delivery-smoke",
            "Real channel send/edit/delete delivery smoke",
            Channel,
            true,
            "HEPTA_EXTERNAL_CHANNEL_SMOKE=1",
            "approve a bounded real-channel delivery smoke target",
        ),
        requirement(
            "device-node-smoke",
            "Real paired device/node invoke and health smoke",
            Device,
            true,
            "HEPTA_EXTERNAL_DEVICE_SMOKE=1",
            "approve a bounded paired-device smoke with no private payload disclosure",
        ),
        requirement(
            "plugin-adapter-smoke",
            "Credentialed plugin adapter handoff smoke",
            Plugin,
            true,
            "HEPTA_EXTERNAL_PLUGIN_SMOKE=1",
            "run plugin handoff against a configured non-destructive adapter",
        ),
        requirement(
            "remote-worker-harness",
            "Remote/ACP worker harness execution",
            Worker,
            true,
            "HEPTA_EXTERNAL_WORKER_HARNESS=1",
            "bind worker lanes to an approved external execution harness",
        ),
        requirement(
            "unattended-scheduler-daemon",
            "Long-running unattended scheduler daemon",
            Scheduler,
            true,
            "HEPTA_EXTERNAL_SCHEDULER_DAEMON=1",
            "run a bounded unattended scheduler soak with wake/recovery evidence",
        ),
        requirement(
            "long-production-soak",
            "Long external production soak",
            Observability,
            true,
            "HEPTA_EXTERNAL_LONG_SOAK=1",
            "complete a long soak with incident-free metrics",
        ),
        requirement(
            "release-channel-governance",
            "Release channel, versioning, and rollback governance",
            Release,
            true,
            "HEPTA_EXTERNAL_RELEASE_CHANNEL=1",
            "choose release channel and validate rollback path",
        ),
        requirement(
            "public-network-ingress",
            "Public or tailnet ingress exposure validation",
            Security,
            true,
            "HEPTA_EXTERNAL_NETWORK_INGRESS=1",
            "validate ingress, TLS, auth, rate limits, and firewall posture",
        ),
        requirement(
            "secret-rotation-drill",
            "Secret rotation and revocation drill",
            Security,
            true,
            "HEPTA_EXTERNAL_SECRET_ROTATION=1",
            "rotate/revoke a test credential and verify redaction",
        ),
        requirement(
            "incident-observability",
            "Production logs, metrics, alerts, and incident traceability",
            Observability,
            false,
            "HEPTA_EXTERNAL_OBSERVABILITY=1",
            "wire metrics/alerts and capture a dry-run incident report",
        ),
        requirement(
            "concurrency-resource-governance",
            "Concurrent worker resource limits and fairness",
            Worker,
            false,
            "HEPTA_EXTERNAL_RESOURCE_GOVERNANCE=1",
            "stress concurrent workers under configured limits",
        ),
        requirement(
            "failure-recovery-drill",
            "Crash/restart recovery and task replay drill",
            Recovery,
            false,
            "HEPTA_EXTERNAL_FAILURE_RECOVERY=1",
            "kill/restart during active work and verify replay/join state",
        ),
        requirement(
            "backup-restore-drill",
            "Backup/restore and disaster recovery drill",
            Recovery,
            false,
            "HEPTA_EXTERNAL_BACKUP_RESTORE=1",
            "restore runtime state from backup and verify integrity gates",
        ),
        requirement(
            "external-policy-audit",
            "External side-effect policy audit",
            Security,
            false,
            "HEPTA_EXTERNAL_POLICY_AUDIT=1",
            "audit policy decisions for every external tool path",
        ),
        requirement(
            "multi-user-permission-smoke",
            "Multi-user/session permission and isolation smoke",
            Security,
            true,
            "HEPTA_EXTERNAL_MULTI_USER=1",
            "validate session/user isolation with approved test accounts",
        ),
        requirement(
            "production-docs-runbook",
            "Production runbook and operator handoff",
            Release,
            false,
            "HEPTA_EXTERNAL_RUNBOOK=1",
            "complete operator runbook with exact gates and rollback steps",
        ),
    ]
}

fn requirement(
    id: &'static str,
    title: &'static str,
    domain: ExternalProductionDomain,
    destructive_or_external_side_effect: bool,
    gate: &'static str,
    remediation: &'static str,
) -> ExternalProductionRequirement {
    ExternalProductionRequirement {
        id,
        title,
        domain,
        destructive_or_external_side_effect,
        requires_operator_approval: destructive_or_external_side_effect,
        local_control_plane_ready: true,
        status: ExternalProductionStatus::Blocked,
        gate,
        remediation,
        evidence_ref: None,
    }
}

fn load_manifest(
    manifest_path: &PathBuf,
) -> (
    bool,
    BTreeMap<String, ExternalProductionEvidence>,
    Option<String>,
) {
    if !manifest_path.exists() {
        return (false, BTreeMap::new(), None);
    }
    match fs::read_to_string(manifest_path)
        .map_err(|err| err.to_string())
        .and_then(|text| {
            serde_json::from_str::<ExternalProductionEvidenceManifest>(&text)
                .map_err(|err| err.to_string())
        }) {
        Ok(manifest) => (true, manifest.evidence, None),
        Err(error) => (true, BTreeMap::new(), Some(error)),
    }
}

fn percent(numerator: usize, denominator: usize) -> u8 {
    if denominator == 0 {
        return 0;
    }
    ((numerator * 100) / denominator) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_production_registry_tracks_seventeen_boundaries() {
        let registry = external_production_requirement_registry();
        assert_eq!(registry.len(), 17);
        assert!(registry.iter().all(|item| item.local_control_plane_ready));
        assert!(
            registry
                .iter()
                .any(|item| item.id == "remote-worker-harness")
        );
        assert!(
            registry
                .iter()
                .any(|item| item.id == "long-production-soak")
        );
    }

    #[test]
    fn missing_manifest_keeps_external_verification_blocked() {
        let report =
            external_production_readiness_report("/tmp/hepta-missing-external-manifest.json");
        assert_eq!(report.requirement_count, 17);
        assert_eq!(report.local_control_plane_coverage_percent, 100);
        assert_eq!(report.external_verification_percent, 0);
        assert_eq!(report.blocked_count, 17);
        assert!(!report.external_production_ready);
        assert!(report.next_blockers().len() == 17);
    }
}

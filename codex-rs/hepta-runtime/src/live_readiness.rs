use hepta_core::HeptaError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeReadinessStage {
    ContractOnly,
    Planner,
    LocalAdapter,
    GatedLiveAdapter,
    LiveReady,
}

impl RuntimeReadinessStage {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ContractOnly => "M0-contract-only",
            Self::Planner => "M1-planner",
            Self::LocalAdapter => "M2-local-adapter",
            Self::GatedLiveAdapter => "M3-gated-live-adapter",
            Self::LiveReady => "M4-product-ready",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuntimeReadinessGateReport {
    pub capability_id: String,
    pub sample_run_ready: bool,
    pub local_adapter_ready: bool,
    pub live_mutation_enabled: bool,
    pub readback_evidence_present: bool,
    pub exact_payload_preview_present: bool,
    pub operator_confirmed: bool,
    pub stage: RuntimeReadinessStage,
    pub product_ready: bool,
    pub warning: Option<String>,
}

pub fn evaluate_runtime_readiness(
    capability_id: &str,
    sample_run_ready: bool,
    local_adapter_ready: bool,
    live_mutation_enabled: bool,
    readback_evidence_present: bool,
    exact_payload_preview_present: bool,
    operator_confirmed: bool,
) -> Result<RuntimeReadinessGateReport, HeptaError> {
    let capability_id = normalize_capability_id(capability_id)?;
    let product_ready = local_adapter_ready
        && live_mutation_enabled
        && readback_evidence_present
        && exact_payload_preview_present
        && operator_confirmed;
    let stage = if product_ready {
        RuntimeReadinessStage::LiveReady
    } else if local_adapter_ready
        && readback_evidence_present
        && exact_payload_preview_present
        && operator_confirmed
    {
        RuntimeReadinessStage::GatedLiveAdapter
    } else if local_adapter_ready {
        RuntimeReadinessStage::LocalAdapter
    } else if sample_run_ready {
        RuntimeReadinessStage::Planner
    } else {
        RuntimeReadinessStage::ContractOnly
    };
    let warning = if sample_run_ready && !product_ready {
        Some("sample_run_ready is contract/local evidence only; do not label as live-ready".into())
    } else {
        None
    };
    Ok(RuntimeReadinessGateReport {
        capability_id,
        sample_run_ready,
        local_adapter_ready,
        live_mutation_enabled,
        readback_evidence_present,
        exact_payload_preview_present,
        operator_confirmed,
        stage,
        product_ready,
        warning,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeProductGateStatus {
    pub contract: bool,
    pub policy: bool,
    pub dry_run: bool,
    pub live_adapter: bool,
    pub readback: bool,
    pub event: bool,
    pub cli: bool,
    pub ui: bool,
    pub tests: bool,
    pub docs: bool,
}

impl RuntimeProductGateStatus {
    pub const fn all_ready(self) -> bool {
        self.contract
            && self.policy
            && self.dry_run
            && self.live_adapter
            && self.readback
            && self.event
            && self.cli
            && self.ui
            && self.tests
            && self.docs
    }

    fn missing_gate_names(self) -> Vec<String> {
        let mut missing = Vec::new();
        if !self.contract {
            missing.push("contract".to_string());
        }
        if !self.policy {
            missing.push("policy".to_string());
        }
        if !self.dry_run {
            missing.push("dry_run".to_string());
        }
        if !self.live_adapter {
            missing.push("live_adapter".to_string());
        }
        if !self.readback {
            missing.push("readback".to_string());
        }
        if !self.event {
            missing.push("event".to_string());
        }
        if !self.cli {
            missing.push("cli".to_string());
        }
        if !self.ui {
            missing.push("ui".to_string());
        }
        if !self.tests {
            missing.push("tests".to_string());
        }
        if !self.docs {
            missing.push("docs".to_string());
        }
        missing
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuntimeProductReadinessReport {
    pub capability_id: String,
    pub gates: RuntimeProductGateStatus,
    pub missing_gates: Vec<String>,
    pub product_ready: bool,
    pub maturity_label: String,
    pub completion_rule: String,
    pub warning: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveAdapterActivationKind {
    Provider,
    Channel,
    Node,
    Process,
}

impl LiveAdapterActivationKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Provider => "provider",
            Self::Channel => "channel",
            Self::Node => "node",
            Self::Process => "process",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveAdapterActivationInput<'a> {
    pub adapter_id: &'a str,
    pub kind: LiveAdapterActivationKind,
    pub exact_payload_preview: &'a str,
    pub exact_payload_hash: &'a str,
    pub policy_allowed: bool,
    pub operator_confirmed: bool,
    pub idempotency_key: &'a str,
    pub readback_evidence_id: &'a str,
    pub duplicate_replay: bool,
    pub dry_run: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LiveAdapterActivationDisciplineReport {
    pub adapter_id: String,
    pub adapter_kind: &'static str,
    pub exact_payload_preview_present: bool,
    pub exact_payload_hash_present: bool,
    pub policy_allowed: bool,
    pub operator_confirmed: bool,
    pub idempotency_key_present: bool,
    pub readback_evidence_present: bool,
    pub duplicate_replay_suppressed: bool,
    pub dry_run: bool,
    pub discipline_ready: bool,
    pub activation_permitted: bool,
    pub blocked_reasons: Vec<String>,
    pub live_side_effect_performed_by_gate: bool,
    pub provider_invoked_by_gate: bool,
    pub channel_delivery_performed_by_gate: bool,
    pub node_invoked_by_gate: bool,
    pub process_spawned_by_gate: bool,
}

pub fn evaluate_live_adapter_activation(
    input: LiveAdapterActivationInput<'_>,
) -> Result<LiveAdapterActivationDisciplineReport, HeptaError> {
    let adapter_id = normalize_capability_id(input.adapter_id)?;
    let exact_payload_preview_present = !input.exact_payload_preview.trim().is_empty();
    let exact_payload_hash_present = normalize_optional_single_line(
        input.exact_payload_hash,
        "live adapter exact payload hash",
    )?
    .is_some();
    let idempotency_key_present =
        normalize_optional_single_line(input.idempotency_key, "live adapter idempotency key")?
            .is_some();
    let readback_evidence_present = normalize_optional_single_line(
        input.readback_evidence_id,
        "live adapter readback evidence id",
    )?
    .is_some();

    let mut blocked_reasons = Vec::new();
    if !exact_payload_preview_present {
        blocked_reasons.push("missing_exact_payload_preview".to_string());
    }
    if !exact_payload_hash_present {
        blocked_reasons.push("missing_exact_payload_hash".to_string());
    }
    if !input.policy_allowed {
        blocked_reasons.push("policy_not_allowed".to_string());
    }
    if !input.operator_confirmed {
        blocked_reasons.push("operator_not_confirmed".to_string());
    }
    if !idempotency_key_present {
        blocked_reasons.push("missing_idempotency_key".to_string());
    }
    if !readback_evidence_present {
        blocked_reasons.push("missing_readback_evidence".to_string());
    }
    if input.duplicate_replay {
        blocked_reasons.push("duplicate_replay_suppressed".to_string());
    }
    if input.dry_run {
        blocked_reasons.push("dry_run_only".to_string());
    }

    let discipline_ready = exact_payload_preview_present
        && exact_payload_hash_present
        && input.policy_allowed
        && input.operator_confirmed
        && idempotency_key_present
        && readback_evidence_present
        && !input.duplicate_replay;
    let activation_permitted = discipline_ready && !input.dry_run;

    Ok(LiveAdapterActivationDisciplineReport {
        adapter_id,
        adapter_kind: input.kind.label(),
        exact_payload_preview_present,
        exact_payload_hash_present,
        policy_allowed: input.policy_allowed,
        operator_confirmed: input.operator_confirmed,
        idempotency_key_present,
        readback_evidence_present,
        duplicate_replay_suppressed: input.duplicate_replay,
        dry_run: input.dry_run,
        discipline_ready,
        activation_permitted,
        blocked_reasons,
        live_side_effect_performed_by_gate: false,
        provider_invoked_by_gate: false,
        channel_delivery_performed_by_gate: false,
        node_invoked_by_gate: false,
        process_spawned_by_gate: false,
    })
}

pub fn live_adapter_activation_discipline_sample()
-> Result<Vec<LiveAdapterActivationDisciplineReport>, HeptaError> {
    [
        LiveAdapterActivationKind::Provider,
        LiveAdapterActivationKind::Channel,
        LiveAdapterActivationKind::Node,
        LiveAdapterActivationKind::Process,
    ]
    .into_iter()
    .map(|kind| {
        let adapter_id = format!("{}-activation-discipline", kind.label());
        let payload = format!("{{\"kind\":\"{}\",\"target\":\"redacted\"}}", kind.label());
        evaluate_live_adapter_activation(LiveAdapterActivationInput {
            adapter_id: &adapter_id,
            kind,
            exact_payload_preview: &payload,
            exact_payload_hash: "sha256:redacted-exact-payload",
            policy_allowed: true,
            operator_confirmed: true,
            idempotency_key: "idempotency:redacted-live-adapter",
            readback_evidence_id: "readback:redacted-live-adapter",
            duplicate_replay: false,
            dry_run: true,
        })
    })
    .collect()
}

pub fn evaluate_runtime_product_readiness(
    capability_id: &str,
    gates: RuntimeProductGateStatus,
) -> Result<RuntimeProductReadinessReport, HeptaError> {
    let capability_id = normalize_capability_id(capability_id)?;
    let product_ready = gates.all_ready();
    let missing_gates = gates.missing_gate_names();
    let maturity_label = if product_ready {
        RuntimeReadinessStage::LiveReady.label()
    } else {
        RuntimeReadinessStage::GatedLiveAdapter.label()
    };
    let warning = if product_ready {
        None
    } else {
        Some("M3 gated handoff is not product-ready until all M4 gates are green".to_string())
    };
    Ok(RuntimeProductReadinessReport {
        capability_id,
        gates,
        missing_gates,
        product_ready,
        maturity_label: maturity_label.to_string(),
        completion_rule:
            "contract + policy + dry_run + live_adapter + readback + event + cli + ui + tests + docs"
                .to_string(),
        warning,
    })
}

fn normalize_capability_id(value: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(
            "runtime readiness capability id must not be empty".into(),
        ));
    }
    if trimmed.contains('\n') || trimmed.contains('\r') || trimmed.contains("..") {
        return Err(HeptaError(
            "runtime readiness capability id must be single-line and scoped".into(),
        ));
    }
    Ok(trimmed.to_string())
}

fn normalize_optional_single_line(value: &str, label: &str) -> Result<Option<String>, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    if trimmed.contains('\n') || trimmed.contains('\r') || trimmed.contains("..") {
        return Err(HeptaError(format!(
            "{label} must be single-line and scoped"
        )));
    }
    Ok(Some(trimmed.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_ready_never_implies_product_ready_without_live_gates() {
        let report = evaluate_runtime_readiness(
            "runtime-capability-matrix",
            true,
            true,
            false,
            true,
            true,
            true,
        )
        .unwrap();
        assert_eq!(report.stage, RuntimeReadinessStage::GatedLiveAdapter);
        assert!(!report.product_ready);
        assert!(
            report
                .warning
                .unwrap()
                .contains("do not label as live-ready")
        );
    }

    #[test]
    fn live_ready_requires_all_mutation_gates() {
        let report = evaluate_runtime_readiness(
            "delivery-queue-message-routing",
            true,
            true,
            true,
            true,
            true,
            true,
        )
        .unwrap();
        assert_eq!(report.stage, RuntimeReadinessStage::LiveReady);
        assert!(report.product_ready);
        assert!(report.warning.is_none());
    }

    #[test]
    fn product_readiness_reports_missing_m4_gates() {
        let report = evaluate_runtime_product_readiness(
            "sessions-transcripts-status",
            RuntimeProductGateStatus {
                contract: true,
                policy: true,
                dry_run: true,
                live_adapter: false,
                readback: true,
                event: true,
                cli: true,
                ui: false,
                tests: true,
                docs: true,
            },
        )
        .unwrap();
        assert!(!report.product_ready);
        assert_eq!(report.maturity_label, "M3-gated-live-adapter");
        assert_eq!(report.missing_gates, vec!["live_adapter", "ui"]);
        assert!(
            report
                .warning
                .unwrap()
                .contains("M3 gated handoff is not product-ready")
        );
    }

    #[test]
    fn product_readiness_requires_all_ten_completion_gates() {
        let report = evaluate_runtime_product_readiness(
            "delivery-queue-message-routing",
            RuntimeProductGateStatus {
                contract: true,
                policy: true,
                dry_run: true,
                live_adapter: true,
                readback: true,
                event: true,
                cli: true,
                ui: true,
                tests: true,
                docs: true,
            },
        )
        .unwrap();
        assert!(report.product_ready);
        assert_eq!(report.maturity_label, "M4-product-ready");
        assert!(report.missing_gates.is_empty());
        assert!(report.warning.is_none());
    }

    #[test]
    fn readiness_rejects_unscoped_ids() {
        assert!(evaluate_runtime_readiness("../bad", true, true, true, true, true, true).is_err());
    }

    #[test]
    fn live_adapter_activation_sample_is_disciplined_but_dry_run_only() {
        let reports = live_adapter_activation_discipline_sample().unwrap();
        assert_eq!(reports.len(), 4);
        assert!(reports.iter().all(|report| report.discipline_ready));
        assert!(reports.iter().all(|report| !report.activation_permitted));
        assert!(reports.iter().all(|report| report.dry_run));
        assert!(
            reports
                .iter()
                .all(|report| report.blocked_reasons == ["dry_run_only"])
        );
        assert!(
            reports
                .iter()
                .all(|report| !report.live_side_effect_performed_by_gate)
        );
        assert!(
            reports
                .iter()
                .all(|report| !report.provider_invoked_by_gate)
        );
        assert!(
            reports
                .iter()
                .all(|report| !report.channel_delivery_performed_by_gate)
        );
        assert!(reports.iter().all(|report| !report.node_invoked_by_gate));
        assert!(reports.iter().all(|report| !report.process_spawned_by_gate));
    }

    #[test]
    fn live_adapter_activation_blocks_without_exact_payload_confirmation_and_readback() {
        let report = evaluate_live_adapter_activation(LiveAdapterActivationInput {
            adapter_id: "provider-openai",
            kind: LiveAdapterActivationKind::Provider,
            exact_payload_preview: "",
            exact_payload_hash: "",
            policy_allowed: false,
            operator_confirmed: false,
            idempotency_key: "",
            readback_evidence_id: "",
            duplicate_replay: false,
            dry_run: false,
        })
        .unwrap();

        assert!(!report.discipline_ready);
        assert!(!report.activation_permitted);
        assert_eq!(
            report.blocked_reasons,
            [
                "missing_exact_payload_preview",
                "missing_exact_payload_hash",
                "policy_not_allowed",
                "operator_not_confirmed",
                "missing_idempotency_key",
                "missing_readback_evidence",
            ]
        );
    }

    #[test]
    fn live_adapter_activation_suppresses_duplicate_replays() {
        let report = evaluate_live_adapter_activation(LiveAdapterActivationInput {
            adapter_id: "channel-telegram",
            kind: LiveAdapterActivationKind::Channel,
            exact_payload_preview: "{\"text\":\"redacted\"}",
            exact_payload_hash: "sha256:redacted",
            policy_allowed: true,
            operator_confirmed: true,
            idempotency_key: "idempotency:redacted",
            readback_evidence_id: "readback:redacted",
            duplicate_replay: true,
            dry_run: false,
        })
        .unwrap();

        assert!(report.duplicate_replay_suppressed);
        assert!(!report.discipline_ready);
        assert!(!report.activation_permitted);
        assert_eq!(report.blocked_reasons, ["duplicate_replay_suppressed"]);
    }
}

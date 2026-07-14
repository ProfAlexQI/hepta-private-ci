use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use sha2::Digest;
use sha2::Sha256;

use crate::gate_spec::GateSpec;
use crate::gate_spec::ReceiptStateMachine;

const SHELL_GATE_PAIR_SPECS_JSON: &str =
    include_str!("../../../scripts/hepta-gate-pair-specs-v1.json");

#[allow(dead_code)]
pub(crate) const CORE_ACTIVATION_CHAIN_GATE_SPECS: [GateSpec; 3] = [
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate.sh",
        source_command: "bash scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate.sh",
        capability: "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-replay-idempotency-denial",
        side_effect_boundary: "local read-only report-only result-receipt replay/idempotency denial persistence gate; captures and validates the no-persistence predecessor with the long-soak environment while never recording or persisting replay state, mutating runtime, invoking providers/models, writing Memory/KG, sending channels, or publishing release state",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh",
        source_command: "bash scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh",
        capability: "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial",
        side_effect_boundary: "local read-only report-only result-receipt ordering/monotonicity denial persistence gate; captures and validates the replay/idempotency predecessor with the long-soak environment while never recording cursors or monotonicity state, mutating runtime, invoking providers/models, writing Memory/KG, sending channels, or publishing release state",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-gate.sh",
        source_command: "bash scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-gate.sh",
        capability: "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial",
        side_effect_boundary: "local read-only report-only result-receipt cancellation/supersession denial persistence gate; captures and validates the ordering/monotonicity predecessor with the long-soak environment while never recording cancellation, replacement, tombstone, or supersession state, mutating runtime, invoking providers/models, writing Memory/KG, sending channels, or publishing release state",
    },
];

#[allow(dead_code)]
pub(crate) const PROVIDER_ROUTER_ACTIVATION_CHAIN_GATE_SPECS: [GateSpec; 3] = [
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial",
        side_effect_boundary: "local read-only report-only runtime provider-router activation-command result-receipt replay/idempotency denial gate; never mutates runtime, invokes providers/models, writes Memory/KG, reads credentials or secrets, sends channels, publishes releases, installs, or restarts",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial",
        side_effect_boundary: "local read-only report-only runtime provider-router activation-command result-receipt ordering/monotonicity denial gate; never mutates runtime, invokes providers/models, writes Memory/KG, reads credentials or secrets, sends channels, publishes releases, installs, or restarts",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial",
        side_effect_boundary: "local read-only report-only runtime provider-router activation-command result-receipt cancellation/supersession denial gate; never mutates runtime, invokes providers/models, writes Memory/KG, reads credentials or secrets, sends channels, publishes releases, installs, or restarts",
    },
];

#[allow(dead_code)]
pub(crate) const OPERATOR_CANARY_ACTIVATION_CHAIN_GATE_SPECS: [GateSpec; 3] = [
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial",
        side_effect_boundary: "local read-only report-only operator-canary controlled-request activation-command result-receipt replay/idempotency denial gate; preserves HEPTA_RELEASE_BIN while never mutating runtime, invoking providers/models, writing Memory/KG, reading credentials or secrets, sending channels, publishing releases, installing, or restarting",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial",
        side_effect_boundary: "local read-only report-only operator-canary controlled-request activation-command result-receipt ordering/monotonicity denial gate; preserves HEPTA_RELEASE_BIN while never mutating runtime, invoking providers/models, writing Memory/KG, reading credentials or secrets, sending channels, publishing releases, installing, or restarting",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial",
        side_effect_boundary: "local read-only report-only operator-canary controlled-request activation-command result-receipt cancellation/supersession denial gate; preserves HEPTA_RELEASE_BIN while never mutating runtime, invoking providers/models, writing Memory/KG, reading credentials or secrets, sending channels, publishing releases, installing, or restarting",
    },
];

#[allow(dead_code)]
pub(crate) const RELEASE_PUBLICATION_RESULT_RECEIPT_GATE_SPECS: [GateSpec; 3] = [
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-replay-idempotency-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-replay-idempotency-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-replay-idempotency-denial",
        side_effect_boundary: "local read-only report-only release-publication result-receipt replay/idempotency denial gate; never mutates runtime, invokes providers/models, writes Memory/KG, reads credentials or secrets, sends channels, publishes releases, installs, or restarts",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-ordering-monotonicity-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-ordering-monotonicity-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-ordering-monotonicity-denial",
        side_effect_boundary: "local read-only report-only release-publication result-receipt ordering/monotonicity denial gate; never mutates runtime, invokes providers/models, writes Memory/KG, reads credentials or secrets, sends channels, publishes releases, installs, or restarts",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial",
        side_effect_boundary: "local read-only report-only release-publication result-receipt cancellation/supersession denial gate; never mutates runtime, invokes providers/models, writes Memory/KG, reads credentials or secrets, sends channels, publishes releases, installs, or restarts",
    },
];

#[allow(dead_code)]
pub(crate) const ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_GATE_SPECS: [GateSpec; 3] = [
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-replay-idempotency-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-replay-idempotency-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-replay-idempotency-denial",
        side_effect_boundary: "local read-only report-only artifact-download/install-affordance result-receipt replay/idempotency denial gate; never mutates runtime, invokes providers/models, writes Memory/KG, reads credentials or secrets, sends channels, publishes releases, installs, or restarts",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial",
        side_effect_boundary: "local read-only report-only artifact-download/install-affordance result-receipt ordering/monotonicity denial gate; never mutates runtime, invokes providers/models, writes Memory/KG, reads credentials or secrets, sends channels, publishes releases, installs, or restarts",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-cancellation-supersession-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-cancellation-supersession-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-cancellation-supersession-denial",
        side_effect_boundary: "local read-only report-only artifact-download/install-affordance result-receipt cancellation/supersession denial gate; never mutates runtime, invokes providers/models, writes Memory/KG, reads credentials or secrets, sends channels, publishes releases, installs, or restarts",
    },
];

#[allow(dead_code)]
pub(crate) const MEMORY_LIVE_MUTATION_RESULT_RECEIPT_GATE_SPECS: [GateSpec; 3] = [
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-gate.sh",
        capability: "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial",
        side_effect_boundary: "local read-only report-only Memory live-mutation result-receipt replay/idempotency denial gate; captures its prior no-state-write gate while never recording or storing replay state, mutating runtime, invoking providers/models, writing Memory/KG, reading secrets, sending channels, publishing, installing, or restarting",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh",
        capability: "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-ordering-monotonicity-denial",
        side_effect_boundary: "local read-only report-only Memory live-mutation result-receipt ordering/monotonicity denial gate; captures its replay/idempotency predecessor while never recording cursor or monotonicity state, mutating runtime, invoking providers/models, writing Memory/KG, reading secrets, sending channels, publishing, installing, or restarting",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-cancellation-supersession-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-cancellation-supersession-denial-gate.sh",
        capability: "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-cancellation-supersession-denial",
        side_effect_boundary: "local read-only report-only Memory live-mutation result-receipt cancellation/supersession denial gate; captures its ordering/monotonicity predecessor while never recording cancellation, replacement, tombstone, or supersession state, mutating runtime, invoking providers/models, writing Memory/KG, reading secrets, sending channels, publishing, installing, or restarting",
    },
];

#[allow(dead_code)]
pub(crate) const TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_GATE_SPECS: [GateSpec; 3] = [
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-denial",
        side_effect_boundary: "local read-only report-only terminal-public-claim delivery-receipt replay/idempotency denial gate; captures its non-persistence predecessor while never replaying, deduplicating, storing idempotency state, mutating runtime, invoking providers/models, writing Memory/KG, reading secrets, sending channels, publishing, signing, installing, or restarting",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-denial",
        side_effect_boundary: "local read-only report-only terminal-public-claim delivery-receipt ordering/monotonicity denial gate; captures its replay/idempotency predecessor while never recording sequence or monotonicity state, mutating runtime, invoking providers/models, writing Memory/KG, reading secrets, sending channels, publishing, signing, installing, or restarting",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-denial",
        side_effect_boundary: "local read-only report-only terminal-public-claim delivery-receipt cancellation/supersession denial gate; captures its ordering/monotonicity predecessor while never recording cancellation, replacement, tombstone, or supersession state, mutating runtime, invoking providers/models, writing Memory/KG, reading secrets, sending channels, publishing, signing, installing, or restarting",
    },
];

#[allow(dead_code)]
pub(crate) const PACKET_ACCEPTANCE_RECEIPT_GATE_SPECS: [GateSpec; 3] = [
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-replay-idempotency-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-replay-idempotency-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-replay-idempotency-denial",
        side_effect_boundary: "local read-only report-only packet-acceptance receipt replay/idempotency denial gate; captures its non-state predecessor while never replaying, deduplicating, or storing idempotency state, mutating runtime, invoking providers/models, writing Memory/KG, reading secrets, sending channels, publishing, signing, installing, or restarting",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-ordering-monotonicity-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-ordering-monotonicity-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-ordering-monotonicity-denial",
        side_effect_boundary: "local read-only report-only packet-acceptance receipt ordering/monotonicity denial gate; captures its replay/idempotency predecessor while never recording sequence or monotonicity state, mutating runtime, invoking providers/models, writing Memory/KG, reading secrets, sending channels, publishing, signing, installing, or restarting",
    },
    GateSpec {
        method: "LOCAL",
        pattern: "scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial-gate.sh",
        source_command: "bash scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial-gate.sh",
        capability: "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial",
        side_effect_boundary: "local read-only report-only packet-acceptance receipt cancellation/supersession denial gate; captures its ordering/monotonicity predecessor while never recording cancellation, replacement, tombstone, or supersession state, mutating runtime, invoking providers/models, writing Memory/KG, reading secrets, sending channels, publishing, signing, installing, or restarting",
    },
];

#[derive(Debug, Default)]
struct ShellScriptAvailability {
    gate: Option<PathBuf>,
    report: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ShellPairManifest {
    schema_version: String,
    receipt_state_machine: Vec<String>,
    pairs: Vec<ShellPairMigrationSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ShellPairMigrationSpec {
    id: String,
    template: String,
    capability: String,
    receipt_state: String,
    side_effect_boundary: String,
    source_report: String,
    report_path: String,
    attachment_surface: String,
    readback_surface: String,
    acknowledgement_prefix: String,
    summary_prefix: String,
    observability_prefix: Option<String>,
    final_ack_denial_side_effect_key: Option<String>,
    final_ack_attachment_surface: Option<String>,
    readback_mode: Option<String>,
    blocker_count: u64,
    attachment_blocker_count: Option<u64>,
    source_gate: Option<String>,
    architecture_note: Option<String>,
    architecture_title: Option<String>,
    terminal_status_gate: Option<String>,
    terminal_status_doc: Option<String>,
    public_status_gate: Option<String>,
    public_status_doc: Option<String>,
    source_file_prefix: Option<String>,
    missing_source_gate_message: Option<String>,
    missing_architecture_note_message: Option<String>,
    missing_terminal_status_gate_message: Option<String>,
    missing_terminal_status_doc_message: Option<String>,
    missing_public_status_gate_message: Option<String>,
    missing_public_status_doc_message: Option<String>,
    next_migration_step: String,
    missing_source_message: String,
    missing_report_message: String,
    pass_message: String,
    gate_implementation: Option<String>,
    gate_source_sha256: Option<String>,
    gate_implementation_sha256: Option<String>,
    report_implementation: Option<String>,
    report_source_sha256: Option<String>,
    report_implementation_sha256: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GateScriptKind {
    Gate,
    Report,
}

pub(crate) fn execute_gate(id: &str) -> Result<String> {
    execute_compatibility_script(id, GateScriptKind::Gate)
}

pub(crate) fn execute_report(id: &str) -> Result<String> {
    execute_compatibility_script(id, GateScriptKind::Report)
}

pub(crate) fn shell_gate_catalog_json() -> Result<String> {
    let repo_root = execution_repo_root()?;
    shell_gate_catalog_json_for_root(&repo_root)
}

pub(crate) fn shell_gate_snapshot_json() -> Result<String> {
    let repo_root = execution_repo_root()?;
    shell_gate_snapshot_json_for_root(&repo_root)
}

pub(crate) fn migrated_pair_spec_json(id: &str) -> Result<Option<String>> {
    validate_id(id)?;
    let specs = migrated_pair_specs()?;
    Ok(specs.get(id).map(|spec| {
        json_or_error(&serde_json::json!({
            "product": "Hepta",
            "runtime": "hepta",
            "status": "ready",
            "runner": "hepta gate",
            "mode": "declarative_shell_pair_migration",
            "id": spec.id,
            "template": spec.template,
            "capability": spec.capability,
            "receipt_state": spec.receipt_state,
            "side_effect_boundary": spec.side_effect_boundary,
            "source_report": spec.source_report,
            "report_path": spec.report_path,
            "blocker_count": spec.blocker_count,
            "captured_shell_compatibility": spec.template == "captured_shell_compat_v1",
            "gate_source_sha256": spec.gate_source_sha256,
            "gate_implementation_sha256": spec.gate_implementation_sha256,
            "report_source_sha256": spec.report_source_sha256,
            "report_implementation_sha256": spec.report_implementation_sha256,
            "report_execution_performed": false,
            "side_effect_free": true,
        }))
    }))
}

pub(crate) fn migrated_pair_registry_summary() -> Result<(&'static str, usize)> {
    let manifest: ShellPairManifest = serde_json::from_str(SHELL_GATE_PAIR_SPECS_JSON)
        .context("failed to parse Hepta migrated gate pair specs")?;
    let schema_version = match manifest.schema_version.as_str() {
        "hepta_gate_pair_specs_v1" => "hepta_gate_pair_specs_v1",
        unsupported => anyhow::bail!("unsupported Hepta migrated gate pair schema: {unsupported}"),
    };
    let pair_count = migrated_pair_specs()?.len();
    Ok((schema_version, pair_count))
}

fn execute_compatibility_script(id: &str, kind: GateScriptKind) -> Result<String> {
    let repo_root = execution_repo_root()?;
    let script = resolve_compatibility_script(&repo_root, id, kind)?;
    let output = Command::new("/bin/bash")
        .arg(&script)
        .current_dir(&repo_root)
        .env("HEPTA_REPO_ROOT", &repo_root)
        .output()
        .with_context(|| format!("failed to execute {}", script.display()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        anyhow::bail!(
            "Hepta {} failed for {id} with status {}\n{}{}",
            kind.label(),
            output.status,
            stdout,
            stderr
        );
    }

    Ok(format!("{stdout}{stderr}"))
}

fn execution_repo_root() -> Result<PathBuf> {
    let repo_root = env::var_os("HEPTA_REPO_ROOT")
        .context("HEPTA_REPO_ROOT is required for source gate/report execution")?;
    fs::canonicalize(PathBuf::from(repo_root)).context("failed to canonicalize HEPTA_REPO_ROOT")
}

fn migrated_pair_specs() -> Result<BTreeMap<String, ShellPairMigrationSpec>> {
    let manifest: ShellPairManifest = serde_json::from_str(SHELL_GATE_PAIR_SPECS_JSON)
        .context("failed to parse Hepta migrated gate pair specs")?;
    if manifest.schema_version != "hepta_gate_pair_specs_v1" {
        anyhow::bail!(
            "unsupported Hepta migrated gate pair schema: {}",
            manifest.schema_version
        );
    }

    let ordered_states = ReceiptStateMachine::ORDERED_STATES
        .iter()
        .map(|state| state.as_str())
        .collect::<Vec<_>>();
    if manifest.receipt_state_machine != ordered_states {
        anyhow::bail!("Hepta migrated gate pair receipt state machine is stale");
    }

    let mut specs = BTreeMap::new();
    for spec in manifest.pairs {
        validate_id(&spec.id)?;
        if !matches!(
            spec.template.as_str(),
            "captured_shell_compat_v1"
                | "signing_final_ack_readback"
                | "signing_final_ack_final_index"
                | "signing_terminal_status_attachment"
                | "signing_terminal_status_final_index"
                | "signing_terminal_status_readback"
                | "signing_public_status_attachment"
                | "signing_public_status_readback"
                | "signing_public_status_final_index"
                | "signing_summary_readback"
        ) {
            anyhow::bail!(
                "Hepta migrated gate pair {} has invalid template: {}",
                spec.id,
                spec.template
            );
        }
        let expected_report_path = format!("scripts/{}-report.sh", spec.id);
        if spec.report_path != expected_report_path {
            anyhow::bail!(
                "Hepta migrated gate pair {} has unexpected report path: {}",
                spec.id,
                spec.report_path
            );
        }
        if !spec.source_report.starts_with("scripts/")
            || (spec.template != "captured_shell_compat_v1"
                && !spec.source_report.ends_with("-report.sh"))
        {
            anyhow::bail!(
                "Hepta migrated gate pair {} has invalid source report: {}",
                spec.id,
                spec.source_report
            );
        }
        if !ReceiptStateMachine::contains_label(&spec.receipt_state) {
            anyhow::bail!(
                "Hepta migrated gate pair {} has invalid receipt state: {}",
                spec.id,
                spec.receipt_state
            );
        }
        let classified_state = ReceiptStateMachine::classify_fields(
            &spec.capability,
            &spec.source_report,
            &spec.side_effect_boundary,
        )
        .map(super::gate_spec::ReceiptState::as_str);
        if classified_state != Some(spec.receipt_state.as_str()) {
            anyhow::bail!(
                "Hepta migrated gate pair {} receipt state does not match ReceiptStateMachine",
                spec.id
            );
        }
        let required_fields = [
            spec.template.as_str(),
            spec.capability.as_str(),
            spec.side_effect_boundary.as_str(),
            spec.attachment_surface.as_str(),
            spec.readback_surface.as_str(),
            spec.acknowledgement_prefix.as_str(),
            spec.summary_prefix.as_str(),
            spec.next_migration_step.as_str(),
            spec.missing_source_message.as_str(),
            spec.missing_report_message.as_str(),
            spec.pass_message.as_str(),
        ];
        if required_fields.iter().any(|field| field.trim().is_empty()) || spec.blocker_count == 0 {
            anyhow::bail!(
                "Hepta migrated gate pair {} has empty required fields",
                spec.id
            );
        }
        if spec.template == "signing_summary_readback"
            && spec
                .observability_prefix
                .as_deref()
                .is_none_or(|prefix| prefix.trim().is_empty())
        {
            anyhow::bail!(
                "Hepta migrated summary pair {} has no observability prefix",
                spec.id
            );
        }
        if spec.template == "captured_shell_compat_v1" {
            let required_compatibility_fields = [
                spec.gate_implementation.as_deref(),
                spec.gate_source_sha256.as_deref(),
                spec.gate_implementation_sha256.as_deref(),
                spec.report_implementation.as_deref(),
                spec.report_source_sha256.as_deref(),
                spec.report_implementation_sha256.as_deref(),
            ];
            if required_compatibility_fields
                .iter()
                .any(|field| field.is_none_or(|value| value.trim().is_empty()))
                || !spec.gate_implementation.as_deref().is_some_and(|path| {
                    path.starts_with("scripts/lib/hepta-gate-pair-compat-v1/")
                        && path.ends_with(".gate")
                })
                || !spec.report_implementation.as_deref().is_some_and(|path| {
                    path.starts_with("scripts/lib/hepta-gate-pair-compat-v1/")
                        && path.ends_with(".report")
                })
                || !spec.gate_source_sha256.as_deref().is_some_and(is_sha256)
                || !spec
                    .gate_implementation_sha256
                    .as_deref()
                    .is_some_and(is_sha256)
                || !spec.report_source_sha256.as_deref().is_some_and(is_sha256)
                || !spec
                    .report_implementation_sha256
                    .as_deref()
                    .is_some_and(is_sha256)
            {
                anyhow::bail!(
                    "Hepta captured-shell pair {} has invalid compatibility fields",
                    spec.id
                );
            }
            if spec.source_report != *spec.report_implementation.as_ref().unwrap() {
                anyhow::bail!(
                    "Hepta captured-shell pair {} source report is not its compatibility payload",
                    spec.id
                );
            }
        }
        if spec.template == "signing_final_ack_final_index"
            && spec
                .final_ack_denial_side_effect_key
                .as_deref()
                .is_none_or(|key| key.trim().is_empty())
        {
            anyhow::bail!(
                "Hepta migrated final-index pair {} has no final-ack denial side-effect key",
                spec.id
            );
        }
        if spec.template == "signing_terminal_status_attachment" {
            let required_template_fields = [
                spec.source_gate.as_deref(),
                spec.architecture_note.as_deref(),
                spec.architecture_title.as_deref(),
                spec.terminal_status_gate.as_deref(),
                spec.terminal_status_doc.as_deref(),
                spec.missing_source_gate_message.as_deref(),
                spec.missing_architecture_note_message.as_deref(),
                spec.missing_terminal_status_gate_message.as_deref(),
                spec.missing_terminal_status_doc_message.as_deref(),
            ];
            if spec.attachment_blocker_count.is_none_or(|count| count == 0)
                || required_template_fields
                    .iter()
                    .any(|field| field.is_none_or(|value| value.trim().is_empty()))
            {
                anyhow::bail!(
                    "Hepta migrated terminal-status pair {} has empty template fields",
                    spec.id
                );
            }
        }
        if spec.template == "signing_terminal_status_readback" {
            let required_template_fields = [
                spec.readback_mode.as_deref(),
                spec.final_ack_attachment_surface.as_deref(),
                spec.source_gate.as_deref(),
                spec.architecture_note.as_deref(),
                spec.architecture_title.as_deref(),
                spec.missing_source_gate_message.as_deref(),
                spec.missing_architecture_note_message.as_deref(),
            ];
            if required_template_fields
                .iter()
                .any(|field| field.is_none_or(|value| value.trim().is_empty()))
            {
                anyhow::bail!(
                    "Hepta migrated terminal-status readback pair {} has empty template fields",
                    spec.id
                );
            }
        }
        if spec.template == "signing_terminal_status_final_index" {
            let required_template_fields = [
                spec.final_ack_attachment_surface.as_deref(),
                spec.source_gate.as_deref(),
                spec.architecture_note.as_deref(),
                spec.architecture_title.as_deref(),
                spec.missing_source_gate_message.as_deref(),
                spec.missing_architecture_note_message.as_deref(),
            ];
            if required_template_fields
                .iter()
                .any(|field| field.is_none_or(|value| value.trim().is_empty()))
            {
                anyhow::bail!(
                    "Hepta migrated terminal-status final-index pair {} has empty template fields",
                    spec.id
                );
            }
        }
        if spec.template == "signing_public_status_attachment" {
            let required_template_fields = [
                spec.source_gate.as_deref(),
                spec.architecture_note.as_deref(),
                spec.architecture_title.as_deref(),
                spec.public_status_gate.as_deref(),
                spec.public_status_doc.as_deref(),
                spec.source_file_prefix.as_deref(),
                spec.missing_source_gate_message.as_deref(),
                spec.missing_architecture_note_message.as_deref(),
                spec.missing_public_status_gate_message.as_deref(),
                spec.missing_public_status_doc_message.as_deref(),
            ];
            if spec.attachment_blocker_count.is_none_or(|count| count == 0)
                || required_template_fields
                    .iter()
                    .any(|field| field.is_none_or(|value| value.trim().is_empty()))
            {
                anyhow::bail!(
                    "Hepta migrated public-status pair {} has empty template fields",
                    spec.id
                );
            }
        }
        if spec.template == "signing_public_status_readback" {
            let required_template_fields = [
                spec.readback_mode.as_deref(),
                spec.source_file_prefix.as_deref(),
                spec.source_gate.as_deref(),
                spec.architecture_note.as_deref(),
                spec.architecture_title.as_deref(),
                spec.missing_source_gate_message.as_deref(),
                spec.missing_architecture_note_message.as_deref(),
            ];
            if required_template_fields
                .iter()
                .any(|field| field.is_none_or(|value| value.trim().is_empty()))
            {
                anyhow::bail!(
                    "Hepta migrated public-status readback pair {} has empty template fields",
                    spec.id
                );
            }
        }
        if spec.template == "signing_public_status_final_index" {
            let required_template_fields = [
                spec.source_file_prefix.as_deref(),
                spec.source_gate.as_deref(),
                spec.architecture_note.as_deref(),
                spec.architecture_title.as_deref(),
                spec.missing_source_gate_message.as_deref(),
                spec.missing_architecture_note_message.as_deref(),
            ];
            if required_template_fields
                .iter()
                .any(|field| field.is_none_or(|value| value.trim().is_empty()))
            {
                anyhow::bail!(
                    "Hepta migrated public-status final-index pair {} has empty template fields",
                    spec.id
                );
            }
        }
        let id = spec.id.clone();
        if specs.insert(id.clone(), spec).is_some() {
            anyhow::bail!("duplicate Hepta migrated gate pair id: {id}");
        }
    }
    Ok(specs)
}

fn validate_migrated_pairs(
    repo_root: &Path,
    catalog: &BTreeMap<String, ShellScriptAvailability>,
    migrated: &BTreeMap<String, ShellPairMigrationSpec>,
) -> Result<()> {
    let scripts_root = fs::canonicalize(repo_root.join("scripts"))
        .context("HEPTA_REPO_ROOT does not contain a scripts directory")?;
    for (id, spec) in migrated {
        let availability = catalog
            .get(id)
            .with_context(|| format!("migrated Hepta gate pair is absent from catalog: {id}"))?;
        let expected_gate = fs::canonicalize(scripts_root.join(format!("{id}-gate.sh")))
            .with_context(|| format!("missing migrated Hepta gate wrapper: {id}"))?;
        let expected_report = fs::canonicalize(repo_root.join(&spec.report_path))
            .with_context(|| format!("missing migrated Hepta report wrapper: {id}"))?;
        let source_report = fs::canonicalize(repo_root.join(&spec.source_report))
            .with_context(|| format!("missing migrated Hepta source report: {id}"))?;
        if availability.gate.as_ref() != Some(&expected_gate)
            || availability.report.as_ref() != Some(&expected_report)
            || !source_report.starts_with(&scripts_root)
        {
            anyhow::bail!("migrated Hepta gate pair path mismatch: {id}");
        }
        validate_thin_wrapper(&expected_gate, "gate", id)?;
        validate_thin_wrapper(&expected_report, "report", id)?;
        if spec.template == "captured_shell_compat_v1" {
            for (kind, relative_path, expected_sha) in [
                (
                    "gate",
                    spec.gate_implementation.as_deref().unwrap(),
                    spec.gate_implementation_sha256.as_deref().unwrap(),
                ),
                (
                    "report",
                    spec.report_implementation.as_deref().unwrap(),
                    spec.report_implementation_sha256.as_deref().unwrap(),
                ),
            ] {
                let implementation = fs::canonicalize(repo_root.join(relative_path))
                    .with_context(|| format!("missing captured Hepta {kind} payload: {id}"))?;
                if !implementation.starts_with(&scripts_root) {
                    anyhow::bail!("captured Hepta {kind} payload escapes scripts root: {id}");
                }
                let bytes = fs::read(&implementation).with_context(|| {
                    format!("failed to read captured Hepta {kind} payload: {id}")
                })?;
                let mut hasher = Sha256::new();
                hasher.update(bytes);
                if hex_digest(hasher.finalize()) != expected_sha {
                    anyhow::bail!("captured Hepta {kind} payload SHA-256 mismatch: {id}");
                }
            }
        }
    }
    Ok(())
}

fn validate_thin_wrapper(path: &Path, kind: &str, id: &str) -> Result<()> {
    let expected = format!(
        "#!/usr/bin/env bash\nset -euo pipefail\n\nROOT=\"$(cd \"$(dirname \"${{BASH_SOURCE[0]}}\")/..\" && pwd -P)\"\nexec \"$ROOT/scripts/hepta-gate-pair-runner\" {kind} \"{id}\"\n"
    );
    let actual = fs::read_to_string(path)
        .with_context(|| format!("failed to read thin Hepta {kind} wrapper: {id}"))?;
    if actual != expected {
        anyhow::bail!("Hepta {kind} wrapper is not canonical and thin: {id}");
    }
    Ok(())
}

fn resolve_compatibility_script(
    repo_root: &Path,
    id: &str,
    kind: GateScriptKind,
) -> Result<PathBuf> {
    validate_id(id)?;

    let scripts_root = fs::canonicalize(repo_root.join("scripts"))
        .context("HEPTA_REPO_ROOT does not contain a scripts directory")?;
    let mut resolved = Vec::new();
    for candidate_name in kind.candidate_names(id) {
        let candidate = scripts_root.join(candidate_name);
        if !candidate.is_file() {
            continue;
        }
        let canonical = fs::canonicalize(&candidate)
            .with_context(|| format!("failed to canonicalize {}", candidate.display()))?;
        if !canonical.starts_with(&scripts_root) {
            anyhow::bail!(
                "Hepta gate script escapes scripts root: {}",
                canonical.display()
            );
        }
        resolved.push(canonical);
    }

    match resolved.as_slice() {
        [script] => Ok(script.clone()),
        [] => anyhow::bail!(
            "no {} compatibility script found for Hepta gate id: {id}",
            kind.label()
        ),
        scripts => anyhow::bail!(
            "ambiguous {} compatibility scripts for Hepta gate id {id}: {}",
            kind.label(),
            scripts
                .iter()
                .map(|script| script.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn shell_gate_catalog_json_for_root(repo_root: &Path) -> Result<String> {
    let catalog = shell_gate_catalog_for_root(repo_root)?;
    let migrated = migrated_pair_specs()?;
    validate_migrated_pairs(repo_root, &catalog, &migrated)?;

    let gate_count = catalog
        .values()
        .filter(|availability| availability.gate.is_some())
        .count();
    let report_count = catalog
        .values()
        .filter(|availability| availability.report.is_some())
        .count();
    let exact_pair_count = catalog
        .values()
        .filter(|availability| availability.gate.is_some() && availability.report.is_some())
        .count();
    let entries = catalog
        .into_iter()
        .map(|(id, availability)| {
            let gate = availability.gate.is_some();
            let report = availability.report.is_some();
            let migration = migrated.get(&id);
            serde_json::json!({
                "id": id,
                "gate": gate,
                "report": report,
                "exact_pair": gate && report,
                "thin_wrapper_migrated": migration.is_some(),
                "receipt_state": migration.map(|spec| spec.receipt_state.as_str()),
            })
        })
        .collect::<Vec<_>>();

    Ok(json_or_error(&serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": "ready",
        "runner": "hepta gate",
        "mode": "legacy_shell_compatibility_catalog",
        "gate_count": gate_count,
        "report_count": report_count,
        "exact_pair_count": exact_pair_count,
        "thin_wrapper_pair_count": migrated.len(),
        "legacy_pair_count": exact_pair_count.saturating_sub(migrated.len()),
        "entry_count": entries.len(),
        "execution_requires_explicit_flag": true,
        "repo_root_required": true,
        "entries": entries,
    })))
}

fn shell_gate_snapshot_json_for_root(repo_root: &Path) -> Result<String> {
    let catalog = shell_gate_catalog_for_root(repo_root)?;
    let migrated = migrated_pair_specs()?;
    validate_migrated_pairs(repo_root, &catalog, &migrated)?;
    let gate_count = catalog
        .values()
        .filter(|availability| availability.gate.is_some())
        .count();
    let report_count = catalog
        .values()
        .filter(|availability| availability.report.is_some())
        .count();
    let exact_pair_count = catalog
        .values()
        .filter(|availability| availability.gate.is_some() && availability.report.is_some())
        .count();
    let mut catalog_hasher = Sha256::new();
    let mut pair_id_hasher = Sha256::new();
    let mut entries = Vec::with_capacity(catalog.len());

    for (id, availability) in catalog {
        let migration = migrated.get(&id);
        let gate = script_snapshot(repo_root, availability.gate.as_deref())?;
        let report = script_snapshot(repo_root, availability.report.as_deref())?;
        let exact_pair = gate.is_some() && report.is_some();
        let gate_path = gate
            .as_ref()
            .map(|snapshot| snapshot.relative_path.as_str())
            .unwrap_or("");
        let gate_sha256 = gate
            .as_ref()
            .map(|snapshot| snapshot.sha256.as_str())
            .unwrap_or("");
        let report_path = report
            .as_ref()
            .map(|snapshot| snapshot.relative_path.as_str())
            .unwrap_or("");
        let report_sha256 = report
            .as_ref()
            .map(|snapshot| snapshot.sha256.as_str())
            .unwrap_or("");

        catalog_hasher.update(id.as_bytes());
        catalog_hasher.update(b"\t");
        catalog_hasher.update(gate_path.as_bytes());
        catalog_hasher.update(b"\t");
        catalog_hasher.update(gate_sha256.as_bytes());
        catalog_hasher.update(b"\t");
        catalog_hasher.update(report_path.as_bytes());
        catalog_hasher.update(b"\t");
        catalog_hasher.update(report_sha256.as_bytes());
        catalog_hasher.update(b"\n");
        if exact_pair {
            pair_id_hasher.update(id.as_bytes());
            pair_id_hasher.update(b"\n");
        }

        entries.push(serde_json::json!({
            "id": id,
            "gate_path": gate.as_ref().map(|snapshot| snapshot.relative_path.as_str()),
            "gate_sha256": gate.as_ref().map(|snapshot| snapshot.sha256.as_str()),
            "report_path": report.as_ref().map(|snapshot| snapshot.relative_path.as_str()),
            "report_sha256": report.as_ref().map(|snapshot| snapshot.sha256.as_str()),
            "exact_pair": exact_pair,
            "thin_wrapper_migrated": migration.is_some(),
            "receipt_state": migration.map(|spec| spec.receipt_state.as_str()),
        }));
    }

    Ok(json_or_error(&serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": "ready",
        "runner": "hepta gate",
        "mode": "legacy_shell_compatibility_parity_snapshot",
        "schema_version": "hepta_shell_gate_parity_snapshot_v1",
        "gate_count": gate_count,
        "report_count": report_count,
        "exact_pair_count": exact_pair_count,
        "thin_wrapper_pair_count": migrated.len(),
        "legacy_pair_count": exact_pair_count.saturating_sub(migrated.len()),
        "entry_count": entries.len(),
        "catalog_sha256": hex_digest(catalog_hasher.finalize()),
        "exact_pair_id_sha256": hex_digest(pair_id_hasher.finalize()),
        "script_execution_performed": false,
        "side_effect_free": true,
        "entries": entries,
    })))
}

fn shell_gate_catalog_for_root(
    repo_root: &Path,
) -> Result<BTreeMap<String, ShellScriptAvailability>> {
    let scripts_root = fs::canonicalize(repo_root.join("scripts"))
        .context("HEPTA_REPO_ROOT does not contain a scripts directory")?;
    let mut catalog = BTreeMap::<String, ShellScriptAvailability>::new();
    for entry in fs::read_dir(&scripts_root).context("failed to read Hepta scripts directory")? {
        let entry = entry.context("failed to read Hepta scripts entry")?;
        if !entry
            .file_type()
            .context("failed to read Hepta scripts entry type")?
            .is_file()
        {
            continue;
        }
        let filename = entry.file_name().to_string_lossy().into_owned();
        let (id, kind) = if let Some(id) = filename.strip_suffix("-gate.sh") {
            (id, GateScriptKind::Gate)
        } else if let Some(id) = filename.strip_suffix("-report.sh") {
            (id, GateScriptKind::Report)
        } else {
            continue;
        };
        if validate_id(id).is_err() {
            continue;
        }
        let path = fs::canonicalize(entry.path())
            .with_context(|| format!("failed to canonicalize {filename}"))?;
        if !path.starts_with(&scripts_root) {
            anyhow::bail!(
                "Hepta shell catalog entry escapes scripts root: {}",
                path.display()
            );
        }
        let availability = catalog.entry(id.to_string()).or_default();
        let slot = match kind {
            GateScriptKind::Gate => &mut availability.gate,
            GateScriptKind::Report => &mut availability.report,
        };
        if let Some(existing) = slot {
            anyhow::bail!(
                "duplicate Hepta {} catalog entry for {id}: {} and {}",
                kind.label(),
                existing.display(),
                path.display()
            );
        }
        *slot = Some(path);
    }
    Ok(catalog)
}

#[derive(Debug)]
struct ScriptSnapshot {
    relative_path: String,
    sha256: String,
}

fn script_snapshot(repo_root: &Path, path: Option<&Path>) -> Result<Option<ScriptSnapshot>> {
    let Some(path) = path else {
        return Ok(None);
    };
    let relative_path = path
        .strip_prefix(repo_root)
        .with_context(|| format!("script is outside Hepta repo root: {}", path.display()))?
        .to_string_lossy()
        .into_owned();
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(Some(ScriptSnapshot {
        relative_path,
        sha256: hex_digest(hasher.finalize()),
    }))
}

fn hex_digest(bytes: impl AsRef<[u8]>) -> String {
    let bytes = bytes.as_ref();
    let mut digest = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(&mut digest, "{byte:02x}");
    }
    digest
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn validate_id(id: &str) -> Result<()> {
    if id.is_empty()
        || !id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        anyhow::bail!("invalid Hepta gate id: {id}");
    }
    Ok(())
}

fn json_or_error(value: &serde_json::Value) -> String {
    serde_json::to_string(value)
        .unwrap_or_else(|err| format!(r#"{{"error":"gate runner serialization failed: {err}"}}"#))
}

impl GateScriptKind {
    fn label(self) -> &'static str {
        match self {
            Self::Gate => "gate",
            Self::Report => "report",
        }
    }

    fn candidate_names(self, id: &str) -> Vec<String> {
        match self {
            Self::Gate => vec![
                format!("{id}-gate.sh"),
                format!("{id}-route-gate.sh"),
                format!("{id}-lane-gate.sh"),
            ],
            Self::Report => vec![format!("{id}-report.sh")],
        }
    }
}

#[cfg(test)]
#[path = "gate_runner_tests.rs"]
mod tests;

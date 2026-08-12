use serde::Deserialize;
use serde::Serialize;

use crate::model::AuthorityBoundary;
use crate::model::FrozenProductBinding;
use crate::model::OperatorBinding;
use crate::model::OracleBinding;
use crate::model::SignatureBinding;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateBindingV2 {
    pub bundle_receipt_gate: String,
    pub bundle_relative_path: String,
    pub bundle_sha256: String,
    pub bundle_size_bytes: u64,
    pub head: String,
    pub parent_1: String,
    pub parent_2: String,
    pub tree: String,
    pub upstream_cutoff: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiptManifestBinding {
    pub manifest_entry_count: usize,
    pub manifest_sha256: String,
    pub receipt_root: String,
    pub status_artifact_relative_path: String,
    pub status_artifact_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlatformGateBinding {
    pub candidate_executed: bool,
    pub candidate_failure: bool,
    pub executed_steps: u64,
    pub excluded_from_pass: bool,
    pub gate: String,
    pub pass: bool,
    pub receipt: ReceiptManifestBinding,
    pub required: bool,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrerequisiteReceiptBinding {
    pub id: String,
    pub pass: bool,
    pub receipt: ReceiptManifestBinding,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlatformPolicy {
    pub blocked_external_satisfies_required_gate: bool,
    pub native_windows_substitutes_for_github: bool,
    pub require_all_required_gates_pass: bool,
    pub required_gates: Vec<String>,
    pub zero_step_execution_satisfies_pass: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationDecision {
    pub blockers: Vec<String>,
    pub complete_gate_count: usize,
    pub pass_gate_count: usize,
    pub verdict: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AggregateQualificationPacket {
    pub automatic_transition: bool,
    pub authority: AuthorityBoundary,
    pub candidate: CandidateBindingV2,
    pub decision: QualificationDecision,
    pub legacy_frozen_product: FrozenProductBinding,
    pub legacy_oracle: OracleBinding,
    pub platform_policy: PlatformPolicy,
    pub platform_receipts: Vec<PlatformGateBinding>,
    pub prerequisite_receipts: Vec<PrerequisiteReceiptBinding>,
    pub schema: String,
    pub schema_version: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AggregateManifestBinding {
    pub manifest_entry_count: usize,
    pub manifest_root_kind: String,
    pub manifest_sha256: String,
    pub packet_relative_path: String,
    pub packet_sha256: String,
    pub receipt_root: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AcceptanceChallengeV2 {
    pub aggregate_manifest: AggregateManifestBinding,
    pub automatic_transition: bool,
    pub authority: AuthorityBoundary,
    pub candidate: CandidateBindingV2,
    pub decision: String,
    pub declaration: String,
    pub expires_at_unix_seconds: u64,
    pub issued_at_unix_seconds: u64,
    pub namespace: String,
    pub nonce: String,
    pub not_before_unix_seconds: u64,
    pub operator: OperatorBinding,
    pub qualification_packet: AggregateQualificationPacket,
    pub schema: String,
    pub schema_version: u32,
    pub scope: String,
    pub signature_algorithm: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AcceptanceReceiptV2 {
    pub accepted_at_unix_seconds: u64,
    pub authority: AuthorityBoundary,
    pub challenge: AcceptanceChallengeV2,
    pub challenge_sha256: String,
    pub schema: String,
    pub schema_version: u32,
    pub signature: SignatureBinding,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NonceClaimV2 {
    pub accepted_at_unix_seconds: u64,
    pub challenge_sha256: String,
    pub detached_signature_sha256: String,
    pub nonce: String,
    pub schema: String,
    pub schema_version: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct QualificationAssessment {
    pub aggregate_manifest_sha256: String,
    pub blockers: Vec<String>,
    pub candidate_head: String,
    pub candidate_tree: String,
    pub github_excluded_from_pass: bool,
    pub ready_for_challenge: bool,
    pub schema: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PreparedChallengeV2 {
    pub challenge_path: String,
    pub challenge_sha256: String,
    pub expires_at_unix_seconds: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SealedAcceptanceV2 {
    pub acceptance_receipt_path: String,
    pub acceptance_receipt_sha256: String,
    pub challenge_sha256: String,
}

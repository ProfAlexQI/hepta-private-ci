use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::model::AuthorityBoundary;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateBundleBindingV3 {
    pub prerequisite_id: String,
    pub relative_path: String,
    pub sha256: String,
    pub size_bytes: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateBindingV3 {
    pub bundle: CandidateBundleBindingV3,
    pub head: String,
    pub integration_merge: String,
    pub parents: Vec<String>,
    pub tree: String,
    pub upstream_cutoff: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestRootKindV3 {
    Sha256ManifestFullInventoryV1,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestLayerBindingV3 {
    pub manifest_entry_count: usize,
    pub manifest_relative_path: String,
    pub manifest_root_kind: ManifestRootKindV3,
    pub manifest_sha256: String,
    pub root_relative_path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceArtifactFormatV3 {
    CanonicalJsonV1,
    JsonV1,
    KeyValueLinesV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModeManifestFormatV3 {
    PosixModePathTsvV1,
    PosixModeSizePathTsvV1,
    TypedLiteralBackslashTModePathV1,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModeManifestBindingV3 {
    pub format: ModeManifestFormatV3,
    pub relative_path: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactAssertionV3 {
    pub expected: Value,
    pub selector: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceArtifactBindingV3 {
    pub assertions: Vec<ArtifactAssertionV3>,
    pub format: EvidenceArtifactFormatV3,
    pub id: String,
    pub relative_path: String,
    pub sha256: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticClaimV3 {
    CandidateExecuted,
    CandidateFailure,
    CandidateHead,
    CandidateParent,
    CandidateTree,
    ExecutedSteps,
    HarnessFailure,
    Pass,
    ProductionChanged,
    Qualification,
    RefsChanged,
    Schema,
    Status,
    UpstreamCutoff,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticClaimBindingV3 {
    pub artifact_id: String,
    pub claim: SemanticClaimV3,
    pub selector: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiptEvidenceBindingV3 {
    pub artifacts: Vec<EvidenceArtifactBindingV3>,
    pub expected_schema: String,
    pub manifest_layers: Vec<ManifestLayerBindingV3>,
    pub mode_manifest: Option<ModeManifestBindingV3>,
    pub receipt_root: String,
    pub semantic_claims: Vec<SemanticClaimBindingV3>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlatformPolicyV3 {
    pub blocked_external_satisfies_required_gate: bool,
    pub native_windows_substitutes_for_github: bool,
    pub require_all_required_gates_pass: bool,
    pub required_gates: Vec<String>,
    pub zero_step_execution_satisfies_pass: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlatformGateInputV3 {
    pub gate: String,
    pub receipt: ReceiptEvidenceBindingV3,
    pub required: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrerequisiteInputV3 {
    pub id: String,
    pub receipt: ReceiptEvidenceBindingV3,
    pub required: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AggregateBuildSpecV3 {
    pub automatic_transition: bool,
    pub authority: AuthorityBoundary,
    pub candidate: CandidateBindingV3,
    pub platform_gates: Vec<PlatformGateInputV3>,
    pub platform_policy: PlatformPolicyV3,
    pub prerequisite_receipts: Vec<PrerequisiteInputV3>,
    pub schema: String,
    pub schema_version: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObservedGateV3 {
    pub candidate_executed: bool,
    pub candidate_failure: bool,
    pub executed_steps: u64,
    pub harness_failure: bool,
    pub pass: bool,
    pub production_changed: bool,
    pub qualification: bool,
    pub refs_changed: bool,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlatformGateBindingV3 {
    pub gate: String,
    pub observed: ObservedGateV3,
    pub receipt: ReceiptEvidenceBindingV3,
    pub required: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObservedPrerequisiteV3 {
    pub pass: bool,
    pub production_changed: bool,
    pub refs_changed: bool,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrerequisiteReceiptBindingV3 {
    pub id: String,
    pub observed: ObservedPrerequisiteV3,
    pub receipt: ReceiptEvidenceBindingV3,
    pub required: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationDecisionV3 {
    pub blockers: Vec<String>,
    pub complete_gate_count: usize,
    pub pass_gate_count: usize,
    pub prerequisite_pass_count: usize,
    pub verdict: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AggregateQualificationPacketV3 {
    pub automatic_transition: bool,
    pub authority: AuthorityBoundary,
    pub candidate: CandidateBindingV3,
    pub decision: QualificationDecisionV3,
    pub platform_policy: PlatformPolicyV3,
    pub platform_receipts: Vec<PlatformGateBindingV3>,
    pub prerequisite_receipts: Vec<PrerequisiteReceiptBindingV3>,
    pub schema: String,
    pub schema_version: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AggregateBuildRecordV3 {
    pub automatic_transition: bool,
    pub authority: AuthorityBoundary,
    pub build_spec_sha256: String,
    pub candidate_head: String,
    pub candidate_tree: String,
    pub evidence_reverified: bool,
    pub qualification_packet_sha256: String,
    pub schema: String,
    pub schema_version: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct QualificationAssessmentV3 {
    pub aggregate_manifest_sha256: String,
    pub blockers: Vec<String>,
    pub candidate_head: String,
    pub candidate_tree: String,
    pub complete_gate_count: usize,
    pub pass_gate_count: usize,
    pub prerequisite_pass_count: usize,
    pub ready_for_challenge: bool,
    pub schema: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AggregateBuildPlanV3 {
    pub blockers: Vec<String>,
    pub build_spec_sha256: String,
    pub candidate_head: String,
    pub candidate_tree: String,
    pub execute_required: bool,
    pub output_root: String,
    pub qualification_packet_sha256: String,
    pub ready_for_challenge: bool,
    pub schema: String,
    pub would_create_files: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SealedAggregateV3 {
    pub aggregate_manifest_entry_count: usize,
    pub aggregate_manifest_sha256: String,
    pub aggregate_root: String,
    pub assessment: QualificationAssessmentV3,
    pub build_spec_sha256: String,
    pub qualification_packet_sha256: String,
    pub schema: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedAggregateV3 {
    pub aggregate_manifest_entry_count: usize,
    pub aggregate_manifest_sha256: String,
    pub aggregate_root: String,
    pub assessment: QualificationAssessmentV3,
    pub build_spec_sha256: String,
    pub evidence_reverified: bool,
    pub qualification_packet_sha256: String,
    pub schema: String,
}

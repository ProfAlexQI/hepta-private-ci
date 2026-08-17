use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedAuthorityV1 {
    pub automatic_transition: bool,
    pub cutover: bool,
    pub default_ref_change: bool,
    pub deletion: bool,
    pub enforce: bool,
    pub full_matrix_claim: bool,
    pub ga_claim: bool,
    pub local_ref_change: bool,
    pub operator_acceptance: bool,
    pub outbound: bool,
    pub production: bool,
    pub promotion: bool,
    pub qualification_authority: bool,
    pub recutover: bool,
    pub remote_ref_change: bool,
    pub retirement: bool,
    pub rollback: bool,
}

impl ClosedAuthorityV1 {
    pub(super) const fn exact() -> Self {
        Self {
            automatic_transition: false,
            cutover: false,
            default_ref_change: false,
            deletion: false,
            enforce: false,
            full_matrix_claim: false,
            ga_claim: false,
            local_ref_change: false,
            operator_acceptance: false,
            outbound: false,
            production: false,
            promotion: false,
            qualification_authority: false,
            recutover: false,
            remote_ref_change: false,
            retirement: false,
            rollback: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryBindingV1 {
    pub head: String,
    pub preservation_ref: String,
    pub tree: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UiRouteStrategyV1 {
    TwentyTwoReadOnlyGetProjectionsDeferredToFirstVnextDevelopmentCycle,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UiCandidateBindingV1 {
    pub backend_contains_ui_tree: bool,
    pub catalog_route_count: u32,
    pub freeze_decision_sha256: String,
    pub freeze_manifest_sha256: String,
    pub integration_deferred: bool,
    pub inventory_schema: String,
    pub inventory_serialization: String,
    pub read_only_get_projection_count: u32,
    pub read_only_get_projection_inventory_bytes: u64,
    pub read_only_get_projection_inventory_sha256: String,
    pub repository: RepositoryBindingV1,
    pub route_strategy: UiRouteStrategyV1,
    pub snapshot_route_count: u32,
    pub source_blob_oid: String,
    pub source_path: String,
    pub source_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProductCandidateBindingV1 {
    pub backend: RepositoryBindingV1,
    pub ui: UiCandidateBindingV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolingIntegrationBindingV1 {
    pub commit: String,
    pub first_parent: String,
    pub second_parent: String,
    pub tree: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiptArtifactPinV1 {
    pub relative_path: String,
    pub sha256: String,
    pub size_bytes: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionReceiptBindingV1 {
    pub decision: ReceiptArtifactPinV1,
    pub manifest_entry_count: usize,
    pub manifest_relative_path: String,
    pub manifest_sha256: String,
    pub manifest_size_bytes: u64,
    pub receipt_root_name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiptLayerBindingV1 {
    pub manifest_entry_count: usize,
    pub manifest_relative_path: String,
    pub manifest_sha256: String,
    pub mode_manifest_relative_path: String,
    pub mode_manifest_sha256: String,
    pub root_relative_path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlatformReceiptBindingV1 {
    pub gate: GateIdV1,
    pub inner: Option<ReceiptLayerBindingV1>,
    pub outer: ReceiptLayerBindingV1,
    pub profile: PlatformProfileV1,
    pub receipt_root_name: String,
    pub terminal: ReceiptArtifactPinV1,
    pub terminal_layer: ReceiptLayerIdV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptLayerIdV1 {
    Inner,
    Outer,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum GateIdV1 {
    #[serde(rename = "github-actions")]
    GithubActions,
    #[serde(rename = "linux-x86_64")]
    LinuxX8664,
    #[serde(rename = "macos-aarch64")]
    MacosAarch64,
    #[serde(rename = "nix-x86_64-linux")]
    NixX8664Linux,
    #[serde(rename = "windows-x86_64-native")]
    WindowsX8664Native,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformProfileV1 {
    GithubHostedDeferredMnlSuccessorV1,
    LinuxSuccessorIdentityPendingV1,
    MacFrozenRev7MnlSuccessorV1,
    NixFrozenRev7MnlSuccessorV1,
    WindowsNativeDeferredMnlSuccessorV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredReasonV1 {
    OutsideMacNixLinuxScopedCutover,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredOwnerV1 {
    GithubHostedQualificationLane,
    WindowsQualificationLane,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredResumeConditionV1 {
    ImmutableHostedQualificationRefAndExactHostedPassReceiptFrozen,
    NativeWindowsSuccessorProfileAndExactPassReceiptFrozen,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredGateMilestoneV1 {
    FullMatrixSuccessorBeforeAnyGaClaim,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, tag = "disposition", rename_all = "snake_case")]
pub enum GateContractV1 {
    Deferred {
        gate: GateIdV1,
        milestone: DeferredGateMilestoneV1,
        owner: DeferredOwnerV1,
        profile: PlatformProfileV1,
        reason: DeferredReasonV1,
        resume_condition: DeferredResumeConditionV1,
    },
    Required {
        gate: GateIdV1,
        profile: PlatformProfileV1,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DevelopmentWorkItemV1 {
    UiTwentyTwoReadOnlyGetProjections,
    UpstreamPostCutoffDrift,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DevelopmentMilestoneV1 {
    FirstVnextDevelopmentCycle,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DevelopmentDeferralV1 {
    pub candidate_changed: bool,
    pub cutover_parity_claimed: bool,
    pub item: DevelopmentWorkItemV1,
    pub target_milestone: DevelopmentMilestoneV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SuccessorContractV1 {
    pub authority: ClosedAuthorityV1,
    pub development_deferrals: Vec<DevelopmentDeferralV1>,
    pub development_freeze_receipt: DecisionReceiptBindingV1,
    pub gates: Vec<GateContractV1>,
    pub present_platform_receipts: Vec<PlatformReceiptBindingV1>,
    pub product_candidate: ProductCandidateBindingV1,
    pub profile_set: String,
    pub receipts_parent: String,
    pub schema: String,
    pub schema_version: u32,
    pub strategy_receipt: DecisionReceiptBindingV1,
    pub tooling_integration_base: ToolingIntegrationBindingV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GateVerificationStateV1 {
    ContentIdentityVerified,
    DeferredDebt,
    ProfileIdentityUnpinned,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedReceiptIdentityV1 {
    pub inner_manifest_sha256: Option<String>,
    pub outer_manifest_sha256: String,
    pub receipt_root: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GateVerificationV1 {
    pub gate: GateIdV1,
    pub profile: PlatformProfileV1,
    pub receipt: Option<VerifiedReceiptIdentityV1>,
    pub state: GateVerificationStateV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedDecisionReceiptV1 {
    pub decision_sha256: String,
    pub manifest_sha256: String,
    pub receipt_root: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PhaseAVerdictV1 {
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FullMatrixVerdictV1 {
    NotClaimed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SuccessorVerificationV1 {
    pub authority: ClosedAuthorityV1,
    pub blockers: Vec<String>,
    pub contract_sha256: String,
    pub development_freeze_receipt: VerifiedDecisionReceiptV1,
    pub full_matrix_verdict: FullMatrixVerdictV1,
    pub gates: Vec<GateVerificationV1>,
    pub phase_a_verdict: PhaseAVerdictV1,
    pub present_wrapper_content_reverified: bool,
    pub product_candidate: ProductCandidateBindingV1,
    pub ready_for_successor_builder: bool,
    pub required_gate_count: usize,
    pub required_pass_count: usize,
    pub schema: String,
    pub strategy_receipt: VerifiedDecisionReceiptV1,
    pub tooling_integration_base: ToolingIntegrationBindingV1,
}

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedAuthorityV1 {
    pub automatic_transition: bool,
    pub enforce: bool,
    pub full_matrix_claim: bool,
    pub ga_claim: bool,
    pub outbound: bool,
    pub production: bool,
    pub promotion: bool,
    pub retirement: bool,
}

impl ClosedAuthorityV1 {
    pub(super) const fn exact() -> Self {
        Self {
            automatic_transition: false,
            enforce: false,
            full_matrix_claim: false,
            ga_claim: false,
            outbound: false,
            production: false,
            promotion: false,
            retirement: false,
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
pub struct CandidateBindingV1 {
    pub backend: RepositoryBindingV1,
    pub ui: UiCandidateBindingV1,
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

impl GateIdV1 {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::GithubActions => "github-actions",
            Self::LinuxX8664 => "linux-x86_64",
            Self::MacosAarch64 => "macos-aarch64",
            Self::NixX8664Linux => "nix-x86_64-linux",
            Self::WindowsX8664Native => "windows-x86_64-native",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformProfileV1 {
    GithubHostedDeferredMnlV1,
    LinuxExactMnlV1,
    MacExactMnlV1,
    NixExactMnlV1,
    WindowsNativeDeferredMnlV1,
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
pub struct ScopeContractV1 {
    pub acceptance_artifact_prefix: String,
    pub aggregate_artifact_prefix: String,
    pub authority: ClosedAuthorityV1,
    pub candidate: CandidateBindingV1,
    pub development_deferrals: Vec<DevelopmentDeferralV1>,
    pub gates: Vec<GateContractV1>,
    pub profile_set: String,
    pub qualification_namespace: String,
    pub schema: String,
    pub schema_version: u32,
    pub sshsig_namespace: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiptManifestPinV1 {
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    deny_unknown_fields,
    tag = "status",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum RequiredGateObservationV1 {
    Fail { receipt: ReceiptManifestPinV1 },
    Missing,
    Pass { receipt: ReceiptManifestPinV1 },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, tag = "disposition", rename_all = "snake_case")]
pub enum GateEvidenceV1 {
    Deferred {
        gate: GateIdV1,
        profile: PlatformProfileV1,
    },
    Required {
        gate: GateIdV1,
        observation: RequiredGateObservationV1,
        profile: PlatformProfileV1,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScopedQualificationInputV1 {
    pub contract: ScopeContractV1,
    pub evidence: Vec<GateEvidenceV1>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScopeVerdictV1 {
    Blocked,
    Pass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FullMatrixVerdictV1 {
    NotClaimed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ScopedQualificationAssessmentV1 {
    pub blockers: Vec<String>,
    pub candidate: CandidateBindingV1,
    pub deferred_gate_count: usize,
    pub full_matrix_verdict: FullMatrixVerdictV1,
    pub listed_gate_count: usize,
    pub ready_for_scoped_challenge: bool,
    pub required_gate_count: usize,
    pub required_pass_count: usize,
    pub schema: String,
    pub scope_verdict: ScopeVerdictV1,
}

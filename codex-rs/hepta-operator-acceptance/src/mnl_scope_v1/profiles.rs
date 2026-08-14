use super::CandidateBindingV1;
use super::ClosedAuthorityV1;
use super::DeferredGateMilestoneV1;
use super::DeferredOwnerV1;
use super::DeferredReasonV1;
use super::DeferredResumeConditionV1;
use super::DevelopmentDeferralV1;
use super::DevelopmentMilestoneV1;
use super::DevelopmentWorkItemV1;
use super::GateContractV1;
use super::GateIdV1;
use super::PlatformProfileV1;
use super::RepositoryBindingV1;
use super::ScopeContractV1;
use super::UiCandidateBindingV1;
use super::UiRouteStrategyV1;
use crate::AcceptanceError;

pub const SCHEMA: &str = "hepta_vnext_mnl_scope_contract_v1";
pub const ASSESSMENT_SCHEMA: &str = "hepta_vnext_mnl_scope_assessment_v1";
pub const QUALIFICATION_NAMESPACE: &str = "hepta-vnext-mnl-scope-qualification-v1";
pub const SSHSIG_NAMESPACE: &str = "hepta-vnext-mnl-scope-operator-acceptance-v1";
pub const PROFILE_SET: &str = "hepta_vnext_52ec4b3868fc_mnl_scope_evidence_profiles_v1_revision_1";
pub const AGGREGATE_ARTIFACT_PREFIX: &str =
    "vnext-main-52ec4b3868fc-mnl-scope-aggregate-qualification-v1";
pub const ACCEPTANCE_ARTIFACT_PREFIX: &str =
    "vnext-main-52ec4b3868fc-mnl-scope-operator-acceptance-v1";
pub const BACKEND_CANDIDATE_HEAD: &str = "52ec4b3868fc5272e19ed516d00e11e44c549ea4";
pub const BACKEND_CANDIDATE_TREE: &str = "247e9e7cfcb41dbfcc8c5b3b531b1e1407c0bd5d";
pub const UI_CANDIDATE_HEAD: &str = "64612c01de811f647d7f113d3104e2c9d8e17656";
pub const UI_CANDIDATE_TREE: &str = "7cae3967f9ab878bc67be8083beb9308482725f5";

pub(super) const LISTED_GATE_COUNT: usize = 5;
pub(super) const REQUIRED_GATE_COUNT: usize = 3;
pub(super) const DEFERRED_GATE_COUNT: usize = 2;

pub fn exact_contract() -> ScopeContractV1 {
    ScopeContractV1 {
        acceptance_artifact_prefix: ACCEPTANCE_ARTIFACT_PREFIX.to_string(),
        aggregate_artifact_prefix: AGGREGATE_ARTIFACT_PREFIX.to_string(),
        authority: ClosedAuthorityV1::exact(),
        candidate: exact_candidate(),
        development_deferrals: [
            DevelopmentWorkItemV1::UpstreamPostCutoffDrift,
            DevelopmentWorkItemV1::UiTwentyTwoReadOnlyGetProjections,
        ]
        .into_iter()
        .map(|item| DevelopmentDeferralV1 {
            candidate_changed: false,
            cutover_parity_claimed: false,
            item,
            target_milestone: DevelopmentMilestoneV1::FirstVnextDevelopmentCycle,
        })
        .collect(),
        gates: exact_gate_contracts(),
        profile_set: PROFILE_SET.to_string(),
        qualification_namespace: QUALIFICATION_NAMESPACE.to_string(),
        schema: SCHEMA.to_string(),
        schema_version: 1,
        sshsig_namespace: SSHSIG_NAMESPACE.to_string(),
    }
}

pub fn validate_contract(contract: &ScopeContractV1) -> Result<(), AcceptanceError> {
    if contract != &exact_contract() {
        return Err(AcceptanceError::Invalid(
            "scope contract differs from the compiled Mac/Nix/Linux boundary".to_string(),
        ));
    }
    Ok(())
}

pub(super) fn exact_candidate() -> CandidateBindingV1 {
    CandidateBindingV1 {
        backend: RepositoryBindingV1 {
            head: BACKEND_CANDIDATE_HEAD.to_string(),
            preservation_ref: "refs/heads/archive/vnext-candidate-52ec4b3868-20260813".to_string(),
            tree: BACKEND_CANDIDATE_TREE.to_string(),
        },
        ui: UiCandidateBindingV1 {
            backend_contains_ui_tree: false,
            catalog_route_count: 21,
            freeze_decision_sha256:
                "9316980821e019f91f3da7380ebf9c473f3c6cfb3a2b01b14a0756de21910a79"
                    .to_string(),
            freeze_manifest_sha256:
                "4d4ec050cd73ef55a52fcb85d15b7a3bfd8e10ec63f1e9d314f2e1150770fc12"
                    .to_string(),
            integration_deferred: true,
            inventory_schema: "hepta_control_ui_read_only_get_inventory_v1".to_string(),
            inventory_serialization: "utf8_sorted_method_tab_path_lf_v1".to_string(),
            read_only_get_projection_count: 22,
            read_only_get_projection_inventory_bytes: 533,
            read_only_get_projection_inventory_sha256:
                "3b57324c845f33f8f0f89d5c69ea716fb3dd948b42a959dabcb1f9c412fdd762"
                    .to_string(),
            repository: RepositoryBindingV1 {
                head: UI_CANDIDATE_HEAD.to_string(),
                preservation_ref: "refs/heads/ui/vnext-main".to_string(),
                tree: UI_CANDIDATE_TREE.to_string(),
            },
            route_strategy:
                UiRouteStrategyV1::TwentyTwoReadOnlyGetProjectionsDeferredToFirstVnextDevelopmentCycle,
            snapshot_route_count: 1,
            source_blob_oid: "44e19b3fb9f84da67d94b0d4151a0eca1b9a1862".to_string(),
            source_path: "apps/hepta-control-ui/control-ui.js".to_string(),
            source_sha256:
                "8e4fdf8264545f3e0f1dd823c617594e5e6994463ed1723f8b3fd65fb04962b5"
                    .to_string(),
        },
    }
}

pub(super) fn exact_gate_contracts() -> Vec<GateContractV1> {
    vec![
        required(GateIdV1::MacosAarch64, PlatformProfileV1::MacExactMnlV1),
        required(
            GateIdV1::LinuxX8664,
            PlatformProfileV1::LinuxExactMnlV1,
        ),
        required(
            GateIdV1::NixX8664Linux,
            PlatformProfileV1::NixExactMnlV1,
        ),
        GateContractV1::Deferred {
            gate: GateIdV1::WindowsX8664Native,
            milestone: DeferredGateMilestoneV1::FullMatrixSuccessorBeforeAnyGaClaim,
            owner: DeferredOwnerV1::WindowsQualificationLane,
            profile: PlatformProfileV1::WindowsNativeDeferredMnlV1,
            reason: DeferredReasonV1::OutsideMacNixLinuxScopedCutover,
            resume_condition:
                DeferredResumeConditionV1::NativeWindowsSuccessorProfileAndExactPassReceiptFrozen,
        },
        GateContractV1::Deferred {
            gate: GateIdV1::GithubActions,
            milestone: DeferredGateMilestoneV1::FullMatrixSuccessorBeforeAnyGaClaim,
            owner: DeferredOwnerV1::GithubHostedQualificationLane,
            profile: PlatformProfileV1::GithubHostedDeferredMnlV1,
            reason: DeferredReasonV1::OutsideMacNixLinuxScopedCutover,
            resume_condition: DeferredResumeConditionV1::ImmutableHostedQualificationRefAndExactHostedPassReceiptFrozen,
        },
    ]
}

fn required(gate: GateIdV1, profile: PlatformProfileV1) -> GateContractV1 {
    GateContractV1::Required { gate, profile }
}

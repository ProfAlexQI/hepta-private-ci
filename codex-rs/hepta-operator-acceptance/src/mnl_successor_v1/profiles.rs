use super::ClosedAuthorityV1;
use super::DecisionReceiptBindingV1;
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
use super::PlatformReceiptBindingV1;
use super::ProductCandidateBindingV1;
use super::ReceiptArtifactPinV1;
use super::ReceiptLayerBindingV1;
use super::ReceiptLayerIdV1;
use super::RepositoryBindingV1;
use super::SuccessorContractV1;
use super::ToolingIntegrationBindingV1;
use super::UiCandidateBindingV1;
use super::UiRouteStrategyV1;
use crate::AcceptanceError;

pub const SCHEMA: &str = "hepta_vnext_mnl_successor_phase_a_contract_v1";
pub const VERIFICATION_SCHEMA: &str = "hepta_vnext_mnl_successor_phase_a_verification_v1";
pub const PROFILE_SET: &str =
    "hepta_vnext_52ec4b3868fc_64612c01de81_898628204ff6_mnl_successor_v1_revision_1";
pub const RECEIPTS_PARENT: &str = "/Volumes/T5/hepta-vnext/artifacts/receipts";

pub const BACKEND_CANDIDATE_HEAD: &str = "52ec4b3868fc5272e19ed516d00e11e44c549ea4";
pub const BACKEND_CANDIDATE_TREE: &str = "247e9e7cfcb41dbfcc8c5b3b531b1e1407c0bd5d";
pub const UI_CANDIDATE_HEAD: &str = "64612c01de811f647d7f113d3104e2c9d8e17656";
pub const UI_CANDIDATE_TREE: &str = "7cae3967f9ab878bc67be8083beb9308482725f5";
pub const TOOLING_INTEGRATION_BASE: &str = "898628204ff60131b8b015555a3f3a5b2ff80987";
pub const TOOLING_INTEGRATION_TREE: &str = "4977641b9bf4e91e1f548c73bc7622fc4e2874ee";
pub const TOOLING_FIRST_PARENT: &str = "9590e76b421000f3af7d5ef1b2c8fa7eaf282305";
pub const TOOLING_SECOND_PARENT: &str = "5352601eeda40aca9fc9a40bb834b8328b498c83";

pub const STRATEGY_RECEIPT_ROOT_NAME: &str =
    "vnext-52ec4b3868-upstream-ui-strategy-20260814T075430Z";
pub const STRATEGY_RECEIPT_MANIFEST_SHA256: &str =
    "80517be5420e31516a0331a5fbb1f97dc1e5228a3b9cbc23089aa75c4460b926";
pub const STRATEGY_DECISION_SHA256: &str =
    "1e4ffbfe6c6603b3a24e9ff4d25187982dd590f7448cb2932cc356ca118e0231";

pub const DEVELOPMENT_FREEZE_RECEIPT_ROOT_NAME: &str =
    "vnext-main-52ec4b3868-development-tree-freeze-decision-20260813T111625Z";
pub const DEVELOPMENT_FREEZE_MANIFEST_SHA256: &str =
    "4d4ec050cd73ef55a52fcb85d15b7a3bfd8e10ec63f1e9d314f2e1150770fc12";
pub const DEVELOPMENT_FREEZE_DECISION_SHA256: &str =
    "9316980821e019f91f3da7380ebf9c473f3c6cfb3a2b01b14a0756de21910a79";

pub const MAC_RECEIPT_ROOT_NAME: &str =
    "vnext-main-52ec4b3868-mac-exact-reemitted-rev7-prepared-20260813T182053Z";
pub const MAC_OUTER_MANIFEST_SHA256: &str =
    "1bc706ba581e9b1498ff65890ca429fb9fd328ebe0f4b103657d9d02a2fef10b";
pub const MAC_OUTER_MODE_SHA256: &str =
    "c6b12f4e161fd4a350fc51c619a8a845bef513c7a1785b2a65c82ad7199e697a";
pub const MAC_TERMINAL_SHA256: &str =
    "d7b8ab9f9c63f9730546c75e53a9ab78c8f8e78aa4f2498fe280c2485ddfacf5";

pub const NIX_RECEIPT_ROOT_NAME: &str =
    "vnext-main-52ec4b3868-nix-exact-reemitted-rev7-prepared-20260813T185012Z";
pub const NIX_OUTER_MANIFEST_SHA256: &str =
    "f81c84fe01076307c80816914d696cf2a2b234b90847c6294b0e283d2ba55ab2";
pub const NIX_OUTER_MODE_SHA256: &str =
    "c2ed5d64444054d8ec52fe04f511bdd58c86ab6481cfcf79c71dc70a5bbb9012";
pub const NIX_INNER_MANIFEST_SHA256: &str =
    "24e6bf9b8b5bd0134b01ea044582570d45a2085cf019fab43dc3c139b1a45a27";
pub const NIX_INNER_MODE_SHA256: &str =
    "5f6be3e09d9373ba794ea6456d071e8f00716c1a4107ae3e7bbc1f4e37f3d7cd";
pub const NIX_TERMINAL_SHA256: &str =
    "fde51ffb1695a8201dfe2e3162514511e8e591c22361c5675da8a8a1131da8df";

pub(super) const REQUIRED_GATE_COUNT: usize = 3;

pub fn exact_contract() -> SuccessorContractV1 {
    SuccessorContractV1 {
        authority: ClosedAuthorityV1::exact(),
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
        development_freeze_receipt: exact_development_freeze_receipt(),
        gates: exact_gate_contracts(),
        present_platform_receipts: vec![exact_mac_receipt(), exact_nix_receipt()],
        product_candidate: exact_product_candidate(),
        profile_set: PROFILE_SET.to_string(),
        receipts_parent: RECEIPTS_PARENT.to_string(),
        schema: SCHEMA.to_string(),
        schema_version: 1,
        strategy_receipt: exact_strategy_receipt(),
        tooling_integration_base: ToolingIntegrationBindingV1 {
            commit: TOOLING_INTEGRATION_BASE.to_string(),
            first_parent: TOOLING_FIRST_PARENT.to_string(),
            second_parent: TOOLING_SECOND_PARENT.to_string(),
            tree: TOOLING_INTEGRATION_TREE.to_string(),
        },
    }
}

pub fn validate_contract(contract: &SuccessorContractV1) -> Result<(), AcceptanceError> {
    if contract != &exact_contract() {
        return Err(invalid(
            "successor contract differs from the compiled dual-product, tooling, or gate boundary",
        ));
    }
    Ok(())
}

fn exact_product_candidate() -> ProductCandidateBindingV1 {
    ProductCandidateBindingV1 {
        backend: RepositoryBindingV1 {
            head: BACKEND_CANDIDATE_HEAD.to_string(),
            preservation_ref: "refs/heads/archive/vnext-candidate-52ec4b3868-20260813"
                .to_string(),
            tree: BACKEND_CANDIDATE_TREE.to_string(),
        },
        ui: UiCandidateBindingV1 {
            backend_contains_ui_tree: false,
            catalog_route_count: 21,
            freeze_decision_sha256: DEVELOPMENT_FREEZE_DECISION_SHA256.to_string(),
            freeze_manifest_sha256: DEVELOPMENT_FREEZE_MANIFEST_SHA256.to_string(),
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

fn exact_strategy_receipt() -> DecisionReceiptBindingV1 {
    DecisionReceiptBindingV1 {
        decision: ReceiptArtifactPinV1 {
            relative_path: "DECISION_RECEIPT.v1.txt".to_string(),
            sha256: STRATEGY_DECISION_SHA256.to_string(),
            size_bytes: 1_666,
        },
        manifest_entry_count: 1,
        manifest_relative_path: "SHA256SUMS".to_string(),
        manifest_sha256: STRATEGY_RECEIPT_MANIFEST_SHA256.to_string(),
        manifest_size_bytes: 90,
        receipt_root_name: STRATEGY_RECEIPT_ROOT_NAME.to_string(),
    }
}

fn exact_development_freeze_receipt() -> DecisionReceiptBindingV1 {
    DecisionReceiptBindingV1 {
        decision: ReceiptArtifactPinV1 {
            relative_path: "decision.txt".to_string(),
            sha256: DEVELOPMENT_FREEZE_DECISION_SHA256.to_string(),
            size_bytes: 1_314,
        },
        manifest_entry_count: 3,
        manifest_relative_path: "SHA256SUMS".to_string(),
        manifest_sha256: DEVELOPMENT_FREEZE_MANIFEST_SHA256.to_string(),
        manifest_size_bytes: 231,
        receipt_root_name: DEVELOPMENT_FREEZE_RECEIPT_ROOT_NAME.to_string(),
    }
}

fn exact_mac_receipt() -> PlatformReceiptBindingV1 {
    PlatformReceiptBindingV1 {
        gate: GateIdV1::MacosAarch64,
        inner: None,
        outer: ReceiptLayerBindingV1 {
            manifest_entry_count: 236,
            manifest_relative_path: "SHA256SUMS".to_string(),
            manifest_sha256: MAC_OUTER_MANIFEST_SHA256.to_string(),
            mode_manifest_relative_path: "MODES.tsv".to_string(),
            mode_manifest_sha256: MAC_OUTER_MODE_SHA256.to_string(),
            root_relative_path: ".".to_string(),
        },
        profile: PlatformProfileV1::MacFrozenRev7MnlSuccessorV1,
        receipt_root_name: MAC_RECEIPT_ROOT_NAME.to_string(),
        terminal: ReceiptArtifactPinV1 {
            relative_path: "qualification-status.txt".to_string(),
            sha256: MAC_TERMINAL_SHA256.to_string(),
            size_bytes: 3_649,
        },
        terminal_layer: ReceiptLayerIdV1::Outer,
    }
}

fn exact_nix_receipt() -> PlatformReceiptBindingV1 {
    PlatformReceiptBindingV1 {
        gate: GateIdV1::NixX8664Linux,
        inner: Some(ReceiptLayerBindingV1 {
            manifest_entry_count: 103,
            manifest_relative_path: "SHA256SUMS".to_string(),
            manifest_sha256: NIX_INNER_MANIFEST_SHA256.to_string(),
            mode_manifest_relative_path: "MODES.tsv".to_string(),
            mode_manifest_sha256: NIX_INNER_MODE_SHA256.to_string(),
            root_relative_path: "receipt".to_string(),
        }),
        outer: ReceiptLayerBindingV1 {
            manifest_entry_count: 239,
            manifest_relative_path: "OUTER-SHA256SUMS".to_string(),
            manifest_sha256: NIX_OUTER_MANIFEST_SHA256.to_string(),
            mode_manifest_relative_path: "OUTER-MODES.tsv".to_string(),
            mode_manifest_sha256: NIX_OUTER_MODE_SHA256.to_string(),
            root_relative_path: ".".to_string(),
        },
        profile: PlatformProfileV1::NixFrozenRev7MnlSuccessorV1,
        receipt_root_name: NIX_RECEIPT_ROOT_NAME.to_string(),
        terminal: ReceiptArtifactPinV1 {
            relative_path: "result.txt".to_string(),
            sha256: NIX_TERMINAL_SHA256.to_string(),
            size_bytes: 1_412,
        },
        terminal_layer: ReceiptLayerIdV1::Inner,
    }
}

fn exact_gate_contracts() -> Vec<GateContractV1> {
    vec![
        required(
            GateIdV1::MacosAarch64,
            PlatformProfileV1::MacFrozenRev7MnlSuccessorV1,
        ),
        required(
            GateIdV1::LinuxX8664,
            PlatformProfileV1::LinuxSuccessorIdentityPendingV1,
        ),
        required(
            GateIdV1::NixX8664Linux,
            PlatformProfileV1::NixFrozenRev7MnlSuccessorV1,
        ),
        GateContractV1::Deferred {
            gate: GateIdV1::WindowsX8664Native,
            milestone: DeferredGateMilestoneV1::FullMatrixSuccessorBeforeAnyGaClaim,
            owner: DeferredOwnerV1::WindowsQualificationLane,
            profile: PlatformProfileV1::WindowsNativeDeferredMnlSuccessorV1,
            reason: DeferredReasonV1::OutsideMacNixLinuxScopedCutover,
            resume_condition:
                DeferredResumeConditionV1::NativeWindowsSuccessorProfileAndExactPassReceiptFrozen,
        },
        GateContractV1::Deferred {
            gate: GateIdV1::GithubActions,
            milestone: DeferredGateMilestoneV1::FullMatrixSuccessorBeforeAnyGaClaim,
            owner: DeferredOwnerV1::GithubHostedQualificationLane,
            profile: PlatformProfileV1::GithubHostedDeferredMnlSuccessorV1,
            reason: DeferredReasonV1::OutsideMacNixLinuxScopedCutover,
            resume_condition: DeferredResumeConditionV1::ImmutableHostedQualificationRefAndExactHostedPassReceiptFrozen,
        },
    ]
}

fn required(gate: GateIdV1, profile: PlatformProfileV1) -> GateContractV1 {
    GateContractV1::Required { gate, profile }
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}

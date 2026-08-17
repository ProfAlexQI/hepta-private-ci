#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
#[cfg(unix)]
use std::path::PathBuf;

use pretty_assertions::assert_eq;
#[cfg(unix)]
use tempfile::TempDir;

use super::*;
#[cfg(unix)]
use crate::durable::sha256;

#[test]
fn exact_contract_binds_dual_products_tooling_and_all_closed_authority() {
    let contract = exact_contract();
    assert_eq!(
        contract.product_candidate.backend.head,
        BACKEND_CANDIDATE_HEAD
    );
    assert_eq!(
        contract.product_candidate.backend.tree,
        BACKEND_CANDIDATE_TREE
    );
    assert_eq!(
        contract.product_candidate.ui.repository.head,
        UI_CANDIDATE_HEAD
    );
    assert_eq!(
        contract.product_candidate.ui.repository.tree,
        UI_CANDIDATE_TREE
    );
    assert_eq!(
        contract
            .product_candidate
            .ui
            .read_only_get_projection_inventory_sha256,
        "3b57324c845f33f8f0f89d5c69ea716fb3dd948b42a959dabcb1f9c412fdd762"
    );
    assert_eq!(
        contract.tooling_integration_base.commit,
        TOOLING_INTEGRATION_BASE
    );
    assert_eq!(
        contract.tooling_integration_base.tree,
        TOOLING_INTEGRATION_TREE
    );
    let authority = serde_json::to_value(&contract.authority).expect("authority JSON");
    assert!(
        authority
            .as_object()
            .expect("authority object")
            .values()
            .all(|value| value == &serde_json::Value::Bool(false))
    );
}

#[test]
fn gate_dispositions_and_present_identity_topology_are_canonical() {
    let contract = exact_contract();
    assert_eq!(contract.gates.len(), 5);
    assert!(matches!(
        contract.gates.as_slice(),
        [
            GateContractV1::Required {
                gate: GateIdV1::MacosAarch64,
                ..
            },
            GateContractV1::Required {
                gate: GateIdV1::LinuxX8664,
                ..
            },
            GateContractV1::Required {
                gate: GateIdV1::NixX8664Linux,
                ..
            },
            GateContractV1::Deferred {
                gate: GateIdV1::WindowsX8664Native,
                ..
            },
            GateContractV1::Deferred {
                gate: GateIdV1::GithubActions,
                ..
            }
        ]
    ));
    assert_eq!(contract.present_platform_receipts.len(), 2);
    assert_eq!(
        contract.present_platform_receipts[0].outer.manifest_sha256,
        MAC_OUTER_MANIFEST_SHA256
    );
    assert!(contract.present_platform_receipts[0].inner.is_none());
    assert_eq!(
        contract.present_platform_receipts[1].outer.manifest_sha256,
        NIX_OUTER_MANIFEST_SHA256
    );
    assert_eq!(
        contract.present_platform_receipts[1]
            .inner
            .as_ref()
            .expect("Nix inner identity")
            .manifest_sha256,
        NIX_INNER_MANIFEST_SHA256
    );
}

#[test]
fn exact_contract_rejects_digest_shaped_identity_substitution_and_debt_relaxation() {
    let mut digest_substitution = exact_contract();
    digest_substitution.present_platform_receipts[0]
        .outer
        .manifest_sha256 = "a".repeat(64);
    assert!(validate_contract(&digest_substitution).is_err());

    let mut invented_linux = exact_contract();
    let mut fake = invented_linux.present_platform_receipts[0].clone();
    fake.gate = GateIdV1::LinuxX8664;
    fake.profile = PlatformProfileV1::LinuxSuccessorIdentityPendingV1;
    fake.outer.manifest_sha256 = "b".repeat(64);
    invented_linux.present_platform_receipts.push(fake);
    assert!(validate_contract(&invented_linux).is_err());

    let mut relaxed_windows = exact_contract();
    relaxed_windows.gates[3] = GateContractV1::Required {
        gate: GateIdV1::WindowsX8664Native,
        profile: PlatformProfileV1::WindowsNativeDeferredMnlSuccessorV1,
    };
    assert!(validate_contract(&relaxed_windows).is_err());
}

#[test]
fn unknown_contract_fields_fail_closed() {
    let mut value = serde_json::to_value(exact_contract()).expect("contract JSON");
    value["ready_for_successor_builder"] = serde_json::json!(true);
    assert!(serde_json::from_value::<SuccessorContractV1>(value).is_err());
}

#[test]
fn old_mnl_scope_digest_declarations_never_become_consumable_passes() {
    use crate::mnl_scope_v1 as old;

    let pass = |sha256: &str| old::RequiredGateObservationV1::Pass {
        receipt: old::ReceiptManifestPinV1 {
            sha256: sha256.to_string(),
        },
    };
    let input = old::ScopedQualificationInputV1 {
        contract: old::exact_contract(),
        evidence: vec![
            old::GateEvidenceV1::Required {
                gate: old::GateIdV1::MacosAarch64,
                observation: pass(MAC_OUTER_MANIFEST_SHA256),
                profile: old::PlatformProfileV1::MacExactMnlV1,
            },
            old::GateEvidenceV1::Required {
                gate: old::GateIdV1::LinuxX8664,
                observation: pass(&"1".repeat(64)),
                profile: old::PlatformProfileV1::LinuxExactMnlV1,
            },
            old::GateEvidenceV1::Required {
                gate: old::GateIdV1::NixX8664Linux,
                observation: pass(NIX_OUTER_MANIFEST_SHA256),
                profile: old::PlatformProfileV1::NixExactMnlV1,
            },
            old::GateEvidenceV1::Deferred {
                gate: old::GateIdV1::WindowsX8664Native,
                profile: old::PlatformProfileV1::WindowsNativeDeferredMnlV1,
            },
            old::GateEvidenceV1::Deferred {
                gate: old::GateIdV1::GithubActions,
                profile: old::PlatformProfileV1::GithubHostedDeferredMnlV1,
            },
        ],
    };
    let old_assessment = old::assess(&input).expect("old digest declarations");
    assert!(!old_assessment.ready_for_scoped_challenge);
    assert_eq!(old_assessment.required_pass_count, 0);
    assert_eq!(old_assessment.blockers.len(), 3);
    assert!(
        old_assessment
            .blockers
            .iter()
            .all(|blocker| blocker.ends_with(":CONTENT_NOT_REVERIFIED"))
    );
}

#[test]
fn public_verifier_rejects_a_broader_receipt_ancestor() {
    let error = verify_current_receipts(
        &exact_contract(),
        Path::new("/Volumes/T5/hepta-vnext/artifacts"),
    )
    .expect_err("broader receipt ancestor must fail");
    assert!(
        error
            .to_string()
            .contains("exact canonical receipts parent")
    );
}

#[cfg(target_os = "macos")]
#[test]
fn exact_phase_a_reverifies_present_content_but_stays_blocked_on_linux() {
    let verified = verify_current_receipts(&exact_contract(), Path::new(RECEIPTS_PARENT))
        .expect("exact current Phase A verification");
    assert!(verified.present_wrapper_content_reverified);
    assert_eq!(verified.phase_a_verdict, PhaseAVerdictV1::Blocked);
    assert!(!verified.ready_for_successor_builder);
    assert_eq!(verified.required_pass_count, 0);
    assert_eq!(
        verified.blockers,
        vec!["gate:linux-x86_64:PROFILE_IDENTITY_UNPINNED".to_string()]
    );
    assert_eq!(
        verified
            .gates
            .iter()
            .map(|gate| gate.state)
            .collect::<Vec<_>>(),
        vec![
            GateVerificationStateV1::ContentIdentityVerified,
            GateVerificationStateV1::ProfileIdentityUnpinned,
            GateVerificationStateV1::ContentIdentityVerified,
            GateVerificationStateV1::DeferredDebt,
            GateVerificationStateV1::DeferredDebt,
        ]
    );
    assert_eq!(
        verified.strategy_receipt.decision_sha256,
        STRATEGY_DECISION_SHA256
    );
    assert_eq!(
        verified.development_freeze_receipt.decision_sha256,
        DEVELOPMENT_FREEZE_DECISION_SHA256
    );
    assert_eq!(verified.authority, exact_contract().authority);
}

#[cfg(unix)]
#[test]
fn strategy_verifier_rejects_coherent_reseal_and_unmanifested_files() {
    let contract = exact_contract();
    let valid = StrategyFixture::new(strategy_decision_fixture());
    super::verifier::verify_strategy_receipt_for_test(&valid.binding, &valid.parent, &contract)
        .expect("valid strategy fixture");

    let mut substituted = StrategyFixture::new(strategy_decision_fixture());
    substituted
        .reseal(strategy_decision_fixture().replace(BACKEND_CANDIDATE_HEAD, &"f".repeat(40)));
    let error = super::verifier::verify_strategy_receipt_for_test(
        &substituted.binding,
        &substituted.parent,
        &contract,
    )
    .expect_err("coherent candidate substitution must fail");
    assert!(error.to_string().contains("backend_candidate.commit"));

    let with_extra = StrategyFixture::new(strategy_decision_fixture());
    write_private(
        &with_extra.root.join("UNSEALED.txt"),
        "shadow\n".to_string(),
    );
    let error = super::verifier::verify_strategy_receipt_for_test(
        &with_extra.binding,
        &with_extra.parent,
        &contract,
    )
    .expect_err("unmanifested strategy file must fail");
    assert!(error.to_string().contains("inventory differs"));
}

#[cfg(unix)]
struct StrategyFixture {
    _temp: TempDir,
    binding: DecisionReceiptBindingV1,
    parent: PathBuf,
    root: PathBuf,
}

#[cfg(unix)]
impl StrategyFixture {
    fn new(decision: String) -> Self {
        let temp = TempDir::new().expect("strategy tempdir");
        let parent = temp.path().canonicalize().expect("canonical tempdir");
        let root = parent.join("strategy-receipt");
        fs::create_dir(&root).expect("strategy root");
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).expect("strategy root mode");
        let mut fixture = Self {
            _temp: temp,
            binding: DecisionReceiptBindingV1 {
                decision: ReceiptArtifactPinV1 {
                    relative_path: "DECISION_RECEIPT.v1.txt".to_string(),
                    sha256: String::new(),
                    size_bytes: 0,
                },
                manifest_entry_count: 1,
                manifest_relative_path: "SHA256SUMS".to_string(),
                manifest_sha256: String::new(),
                manifest_size_bytes: 0,
                receipt_root_name: "strategy-receipt".to_string(),
            },
            parent,
            root,
        };
        fixture.reseal(decision);
        fixture
    }

    fn reseal(&mut self, decision: String) {
        self.binding.decision.sha256 = sha256(decision.as_bytes());
        self.binding.decision.size_bytes = decision.len() as u64;
        write_private(&self.root.join("DECISION_RECEIPT.v1.txt"), decision);
        let manifest = format!(
            "{}  DECISION_RECEIPT.v1.txt\n",
            self.binding.decision.sha256
        );
        write_private(&self.root.join("SHA256SUMS"), manifest.clone());
        self.binding.manifest_sha256 = sha256(manifest.as_bytes());
        self.binding.manifest_size_bytes = manifest.len() as u64;
    }
}

#[cfg(unix)]
fn write_private(path: &Path, value: String) {
    fs::write(path, value).expect("write fixture");
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).expect("fixture mode");
}

#[cfg(unix)]
fn strategy_decision_fixture() -> String {
    format!(
        "schema=hepta_vnext_upstream_ui_strategy_v1\n\
         \n\
         [backend_candidate]\n\
         commit={BACKEND_CANDIDATE_HEAD}\n\
         tree={BACKEND_CANDIDATE_TREE}\n\
         \n\
         [upstream]\n\
         frozen_cutoff=74004b5397b24662a87a5264a6ae80664168c7f3\n\
         live_main_observed=86b1123ff6b5d089a146be4e603a324cf454223a\n\
         frozen_cutoff_to_live_main_ahead=92\n\
         decision=freeze_74004_for_52ec_qualification\n\
         deferred_action=intake_live_upstream_delta_in_first_post_cutover_vnext_development_cycle\n\
         forbidden_claim=upstream_plus_32\n\
         \n\
         [ui_candidate]\n\
         commit={UI_CANDIDATE_HEAD}\n\
         qualification_route_links=22\n\
         route_directory_coverage=26_of_26\n\
         backend_ui_merge_base=none\n\
         \n\
         [ui_upstream]\n\
         decision=bounded_patch_ledger_only\n\
         whole_tree_overwrite_allowed=false\n\
         \n\
         [integration_policy]\n\
         decision=dual_exact_head_binding\n\
         backend_head={BACKEND_CANDIDATE_HEAD}\n\
         ui_head={UI_CANDIDATE_HEAD}\n\
         unrelated_history_merge_allowed=false\n\
         aggregate_must_bind_both_heads=true\n\
         windows_and_github_ci_deferred=true\n\
         promotion_authority=false\n\
         qualification_pass_authority=false\n\
         production_authority=false\n"
    )
}

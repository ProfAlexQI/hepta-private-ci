use pretty_assertions::assert_eq;

use super::*;

const MAC_RECEIPT_MANIFEST: &str =
    "1bc706ba581e9b1498ff65890ca429fb9fd328ebe0f4b103657d9d02a2fef10b";
const NIX_RECEIPT_MANIFEST: &str =
    "f81c84fe01076307c80816914d696cf2a2b234b90847c6294b0e283d2ba55ab2";
const LINUX_RECEIPT_MANIFEST: &str =
    "1111111111111111111111111111111111111111111111111111111111111111";

#[test]
fn current_exact_state_is_blocked_on_linux_without_claiming_the_full_matrix() {
    assert_eq!(
        assess(&input_with_linux(RequiredGateObservationV1::Missing))
            .expect("current scoped assessment"),
        ScopedQualificationAssessmentV1 {
            blockers: vec![
                "gate:macos-aarch64:CONTENT_NOT_REVERIFIED".to_string(),
                "gate:linux-x86_64:MISSING".to_string(),
                "gate:nix-x86_64-linux:CONTENT_NOT_REVERIFIED".to_string(),
            ],
            candidate: exact_contract().candidate,
            deferred_gate_count: 2,
            full_matrix_verdict: FullMatrixVerdictV1::NotClaimed,
            listed_gate_count: 5,
            ready_for_scoped_challenge: false,
            required_gate_count: 3,
            required_pass_count: 0,
            schema: ASSESSMENT_SCHEMA.to_string(),
            scope_verdict: ScopeVerdictV1::Blocked,
        }
    );
}

#[test]
fn digest_shaped_pass_declarations_never_create_scoped_readiness() {
    assert_eq!(
        assess(&input_with_linux(pass(LINUX_RECEIPT_MANIFEST)))
            .expect("digest declarations are assessed fail-closed"),
        ScopedQualificationAssessmentV1 {
            blockers: vec![
                "gate:macos-aarch64:CONTENT_NOT_REVERIFIED".to_string(),
                "gate:linux-x86_64:CONTENT_NOT_REVERIFIED".to_string(),
                "gate:nix-x86_64-linux:CONTENT_NOT_REVERIFIED".to_string(),
            ],
            candidate: exact_contract().candidate,
            deferred_gate_count: 2,
            full_matrix_verdict: FullMatrixVerdictV1::NotClaimed,
            listed_gate_count: 5,
            ready_for_scoped_challenge: false,
            required_gate_count: 3,
            required_pass_count: 0,
            schema: ASSESSMENT_SCHEMA.to_string(),
            scope_verdict: ScopeVerdictV1::Blocked,
        }
    );
}

#[test]
fn failed_required_gate_is_pinned_and_blocks_the_scope() {
    let assessment =
        assess(&input_with_linux(fail(LINUX_RECEIPT_MANIFEST))).expect("pinned failure assessment");
    assert_eq!(
        assessment,
        ScopedQualificationAssessmentV1 {
            blockers: vec![
                "gate:macos-aarch64:CONTENT_NOT_REVERIFIED".to_string(),
                "gate:linux-x86_64:FAIL".to_string(),
                "gate:nix-x86_64-linux:CONTENT_NOT_REVERIFIED".to_string(),
            ],
            candidate: exact_contract().candidate,
            deferred_gate_count: 2,
            full_matrix_verdict: FullMatrixVerdictV1::NotClaimed,
            listed_gate_count: 5,
            ready_for_scoped_challenge: false,
            required_gate_count: 3,
            required_pass_count: 0,
            schema: ASSESSMENT_SCHEMA.to_string(),
            scope_verdict: ScopeVerdictV1::Blocked,
        }
    );

    let malformed = input_with_linux(pass("not-a-digest"));
    assert!(assess(&malformed).is_err());
}

#[test]
fn tagged_dispositions_reject_receipts_on_deferred_gates() {
    let mut value = serde_json::to_value(input_with_linux(RequiredGateObservationV1::Missing))
        .expect("scope input JSON");
    value["evidence"][3]["receipt"] = serde_json::json!({"sha256": "2".repeat(64)});
    assert!(serde_json::from_value::<ScopedQualificationInputV1>(value).is_err());
}

#[test]
fn tagged_required_pass_cannot_omit_its_receipt() {
    let mut value = serde_json::to_value(input_with_linux(pass(LINUX_RECEIPT_MANIFEST)))
        .expect("scope input JSON");
    value["evidence"][1]["observation"] = serde_json::json!({"status": "PASS"});
    assert!(serde_json::from_value::<ScopedQualificationInputV1>(value).is_err());
}

#[test]
fn evidence_order_profile_and_disposition_are_compiled() {
    let mut wrong_order = input_with_linux(RequiredGateObservationV1::Missing);
    wrong_order.evidence.swap(0, 1);
    assert!(assess(&wrong_order).is_err());

    let mut wrong_profile = input_with_linux(RequiredGateObservationV1::Missing);
    wrong_profile.evidence[0] = GateEvidenceV1::Required {
        gate: GateIdV1::MacosAarch64,
        observation: pass(MAC_RECEIPT_MANIFEST),
        profile: PlatformProfileV1::NixExactMnlV1,
    };
    assert!(assess(&wrong_profile).is_err());

    let mut promoted_windows = input_with_linux(RequiredGateObservationV1::Missing);
    promoted_windows.evidence[3] = GateEvidenceV1::Required {
        gate: GateIdV1::WindowsX8664Native,
        observation: RequiredGateObservationV1::Missing,
        profile: PlatformProfileV1::WindowsNativeDeferredMnlV1,
    };
    assert!(assess(&promoted_windows).is_err());
}

#[test]
fn old_protocol_identifiers_and_deferred_policy_edits_are_rejected() {
    let mut old_schema = exact_contract();
    old_schema.schema = "hepta_vnext_aggregate_build_spec_v3".to_string();
    assert!(validate_contract(&old_schema).is_err());

    let mut old_namespace = exact_contract();
    old_namespace.qualification_namespace = "hepta-operator-acceptance-v2".to_string();
    assert!(validate_contract(&old_namespace).is_err());

    let mut wrong_prefix = exact_contract();
    wrong_prefix.aggregate_artifact_prefix = "vnext-main-52ec4b3868-v3".to_string();
    assert!(validate_contract(&wrong_prefix).is_err());

    let mut relaxed_policy = exact_contract();
    relaxed_policy.gates[3] = GateContractV1::Deferred {
        gate: GateIdV1::WindowsX8664Native,
        milestone: DeferredGateMilestoneV1::FullMatrixSuccessorBeforeAnyGaClaim,
        owner: DeferredOwnerV1::GithubHostedQualificationLane,
        profile: PlatformProfileV1::WindowsNativeDeferredMnlV1,
        reason: DeferredReasonV1::OutsideMacNixLinuxScopedCutover,
        resume_condition:
            DeferredResumeConditionV1::NativeWindowsSuccessorProfileAndExactPassReceiptFrozen,
    };
    assert!(validate_contract(&relaxed_policy).is_err());

    let mut parity_claimed = exact_contract();
    parity_claimed.development_deferrals[1].cutover_parity_claimed = true;
    assert!(validate_contract(&parity_claimed).is_err());

    let mut changed_ui_inventory = exact_contract();
    changed_ui_inventory
        .candidate
        .ui
        .read_only_get_projection_count = 21;
    assert!(validate_contract(&changed_ui_inventory).is_err());
}

#[test]
fn unknown_contract_fields_fail_closed() {
    let mut value = serde_json::to_value(exact_contract()).expect("contract JSON");
    value["full_matrix_verdict"] = serde_json::json!("PASS");
    assert!(serde_json::from_value::<ScopeContractV1>(value).is_err());
}

fn pass(receipt_manifest: &str) -> RequiredGateObservationV1 {
    RequiredGateObservationV1::Pass {
        receipt: receipt(receipt_manifest),
    }
}

fn fail(receipt_manifest: &str) -> RequiredGateObservationV1 {
    RequiredGateObservationV1::Fail {
        receipt: receipt(receipt_manifest),
    }
}

fn receipt(sha256: &str) -> ReceiptManifestPinV1 {
    ReceiptManifestPinV1 {
        sha256: sha256.to_string(),
    }
}

fn input_with_linux(linux_observation: RequiredGateObservationV1) -> ScopedQualificationInputV1 {
    ScopedQualificationInputV1 {
        contract: exact_contract(),
        evidence: vec![
            GateEvidenceV1::Required {
                gate: GateIdV1::MacosAarch64,
                observation: pass(MAC_RECEIPT_MANIFEST),
                profile: PlatformProfileV1::MacExactMnlV1,
            },
            GateEvidenceV1::Required {
                gate: GateIdV1::LinuxX8664,
                observation: linux_observation,
                profile: PlatformProfileV1::LinuxExactMnlV1,
            },
            GateEvidenceV1::Required {
                gate: GateIdV1::NixX8664Linux,
                observation: pass(NIX_RECEIPT_MANIFEST),
                profile: PlatformProfileV1::NixExactMnlV1,
            },
            GateEvidenceV1::Deferred {
                gate: GateIdV1::WindowsX8664Native,
                profile: PlatformProfileV1::WindowsNativeDeferredMnlV1,
            },
            GateEvidenceV1::Deferred {
                gate: GateIdV1::GithubActions,
                profile: PlatformProfileV1::GithubHostedDeferredMnlV1,
            },
        ],
    }
}

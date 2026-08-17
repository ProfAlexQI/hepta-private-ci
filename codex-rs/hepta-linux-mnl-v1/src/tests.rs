use pretty_assertions::assert_eq;
use serde_json::Value;

use crate::AuthorityBoundaryV1;
use crate::LinuxMnlError;
use crate::PRODUCTION_TARGET_ALIAS_V1;
use crate::PublishedProfileDocumentsV1;
use crate::canonical_json;
use crate::compiled_phase1_status;
use crate::compiled_profile_status;
use crate::decode_canonical_json;
use crate::exact_composite_identity;
use crate::phase1::test_support as phase1;
use crate::phase1::validate_phase1_result;
use crate::production_phase1_plan;
use crate::profiles::test_support as profiles;
use crate::validate_composite_identity;
use crate::validate_published_profiles;

#[test]
fn binds_backend_ui_and_tooling_baseline_as_three_distinct_roles() {
    let identity = exact_composite_identity();
    validate_composite_identity(&identity).expect("exact identity");
    assert_eq!(
        identity.backend_product.head,
        "52ec4b3868fc5272e19ed516d00e11e44c549ea4"
    );
    assert_eq!(
        identity.ui_product.head,
        "64612c01de811f647d7f113d3104e2c9d8e17656"
    );
    assert_eq!(
        identity.tooling_baseline.head,
        "898628204ff60131b8b015555a3f3a5b2ff80987"
    );
    assert_ne!(
        identity.backend_product.role,
        identity.tooling_baseline.role
    );
}

#[test]
fn rejects_wrong_composite_identity_even_with_well_formed_git_oids() {
    let mut identity = exact_composite_identity();
    identity.backend_product.head = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
    assert!(validate_composite_identity(&identity).is_err());
}

#[test]
fn production_profile_tooling_and_plan_getters_are_hard_blocked() {
    let documents = profiles::documents();
    let profile_status = compiled_profile_status();
    assert!(profile_status.blocked);
    assert!(!profile_status.production_plan_available);
    assert_eq!(profile_status.missing_pins.len(), 14);

    let phase1_status = compiled_phase1_status();
    assert!(phase1_status.blocked);
    assert!(!phase1_status.production_plan_available);
    assert_eq!(phase1_status.missing_requirements.len(), 19);
    assert!(
        phase1_status
            .missing_requirements
            .iter()
            .any(|item| item == "tooling:successor_final_tooling_head")
    );
    assert!(
        phase1_status
            .missing_requirements
            .iter()
            .any(|item| item == "collector:internal_fresh_challenge_nonce_generator")
    );
    assert!(matches!(
        validate_published_profiles(&documents),
        Err(LinuxMnlError::Blocked(_))
    ));
    assert!(matches!(
        production_phase1_plan(),
        Err(LinuxMnlError::Blocked(_))
    ));
}

#[test]
fn test_only_closed_pin_set_validates_all_purpose_separated_profiles() {
    let documents = profiles::documents();
    let verified = profiles::verify(&documents, profiles::pins(&documents)).expect("profiles");
    assert_eq!(verified.target_host_alias(), PRODUCTION_TARGET_ALIAS_V1);
}

#[test]
fn rejects_duplicate_trust_purpose_and_reused_root() {
    let duplicated_purpose = mutated_documents(|value| {
        value["trust_profiles"][1]["purpose"] = Value::String("install_v2".to_string());
    });
    assert!(profiles::verify(&duplicated_purpose, profiles::rehash(&duplicated_purpose)).is_err());

    let reused_root = mutated_documents(|value| {
        let root = value["trust_profiles"][0]["trust_root_id"].clone();
        value["trust_profiles"][1]["trust_root_id"] = root;
    });
    assert!(profiles::verify(&reused_root, profiles::rehash(&reused_root)).is_err());
}

#[test]
fn rejects_missing_install_epoch_genesis_or_current_tip_documents() {
    let mut missing_install_epoch = serde_json::to_value(profiles::documents()).expect("JSON");
    missing_install_epoch
        .as_object_mut()
        .expect("documents")
        .remove("install_epoch_completion");
    assert!(serde_json::from_value::<PublishedProfileDocumentsV1>(missing_install_epoch).is_err());

    let mut missing_genesis = serde_json::to_value(profiles::documents()).expect("JSON");
    missing_genesis["external_watermark_provider"]
        .as_object_mut()
        .expect("provider")
        .remove("genesis_tip_sha256");
    assert!(serde_json::from_value::<PublishedProfileDocumentsV1>(missing_genesis).is_err());

    let mut missing_current_tip = serde_json::to_value(profiles::documents()).expect("JSON");
    missing_current_tip
        .as_object_mut()
        .expect("documents")
        .remove("external_watermark_current_tip");
    assert!(serde_json::from_value::<PublishedProfileDocumentsV1>(missing_current_tip).is_err());
}

#[test]
fn rejects_install_epoch_wrong_trust_or_provider_binding() {
    let wrong_trust = mutated_documents(|value| {
        value["install_epoch_completion"]["install_epoch_trust_profile_sha256"] =
            Value::String(profiles::digest('a'));
    });
    assert!(profiles::verify(&wrong_trust, profiles::rehash(&wrong_trust)).is_err());

    let wrong_provider_binding = mutated_documents(|value| {
        value["external_watermark_provider"]["genesis_epoch_binding_sha256"] =
            Value::String(profiles::digest('b'));
    });
    assert!(
        profiles::verify(
            &wrong_provider_binding,
            profiles::rehash(&wrong_provider_binding)
        )
        .is_err()
    );
}

#[test]
fn rejects_empty_genesis_or_current_tip_under_fresh_matching_digest_pins() {
    let empty_genesis = mutated_documents(|value| {
        value["external_watermark_provider"]["genesis_epoch_binding_sha256"] =
            Value::String(String::new());
    });
    assert!(profiles::verify(&empty_genesis, profiles::rehash(&empty_genesis)).is_err());

    let empty_tip = mutated_documents(|value| {
        value["external_watermark_current_tip"]["observed_tip_sha256"] =
            Value::String(String::new());
    });
    assert!(profiles::verify(&empty_tip, profiles::rehash(&empty_tip)).is_err());
}

#[test]
fn rejects_genesis_revision_with_non_genesis_tip() {
    let wrong_tip = mutated_documents(|value| {
        value["external_watermark_current_tip"]["observed_tip_sha256"] =
            Value::String(profiles::digest('a'));
    });
    assert!(profiles::verify(&wrong_tip, profiles::rehash(&wrong_tip)).is_err());
}

#[test]
fn rejects_wrong_state_root_mode_machine_binding_and_machine_id_path() {
    let wrong_mode = mutated_documents(|value| {
        value["state_root"]["mode"] = Value::from(0o755);
    });
    assert!(profiles::verify(&wrong_mode, profiles::rehash(&wrong_mode)).is_err());

    let wrong_machine = mutated_documents(|value| {
        value["state_root"]["machine_id_sha256"] = Value::String(profiles::digest('a'));
    });
    assert!(profiles::verify(&wrong_machine, profiles::rehash(&wrong_machine)).is_err());

    let wrong_path = mutated_documents(|value| {
        value["target"]["machine_id_path"] =
            Value::String("/tmp/caller-selected-machine-id".to_string());
    });
    assert!(profiles::verify(&wrong_path, profiles::rehash(&wrong_path)).is_err());
}

#[test]
fn only_positive_desktop_target_identity_is_accepted() {
    for alias in [
        "x230-ts",
        "x230-ts.example",
        "desktop-ts-shadow",
        "10.0.0.2",
    ] {
        let substituted = mutated_documents(|value| {
            value["target"]["host_alias"] = Value::String(alias.to_string());
        });
        assert!(
            profiles::verify(&substituted, profiles::rehash(&substituted)).is_err(),
            "target alias {alias} escaped"
        );
    }

    let fixture_role = mutated_documents(|value| {
        value["target"]["role"] = Value::String("qualification_fixture_only".to_string());
        value["target"]["fixture_substitution_allowed"] = Value::Bool(true);
    });
    assert!(profiles::verify(&fixture_role, profiles::rehash(&fixture_role)).is_err());
}

#[test]
fn phase1_structural_completion_is_bound_and_still_no_authority() {
    let plan = phase1::plan();
    assert_eq!(plan.target_host_alias(), PRODUCTION_TARGET_ALIAS_V1);
    assert_ne!(
        plan.successor_final_tooling().head,
        plan.composite_identity().tooling_baseline.head
    );
    assert_eq!(plan.challenge_nonce_sha256().len(), 64);
    assert!(
        plan.receipt_topology()
            .iter()
            .skip(1)
            .all(|node| !node.phase1_may_mint())
    );
    let result = phase1::result(&plan);
    validate_phase1_result(&plan, &result).expect("structurally complete read-only observation");
    assert!(phase1::is_structurally_complete(&result));
    assert_eq!(phase1::authority(&result), &AuthorityBoundaryV1::closed());
    assert!(!phase1::has_target_receipt(&result));
}

#[test]
fn deterministic_observation_receipt_rejects_manifest_tampering() {
    let plan = phase1::plan();
    let mut result = phase1::result(&plan);
    phase1::tamper_receipt_manifest(&mut result);
    assert!(validate_phase1_result(&plan, &result).is_err());
}

#[test]
fn observation_result_cannot_replay_under_a_different_internal_challenge() {
    let first_plan = phase1::plan_with_challenge("first-private-test-challenge");
    let second_plan = phase1::plan_with_challenge("second-private-test-challenge");
    assert_ne!(
        first_plan.challenge_nonce_sha256(),
        second_plan.challenge_nonce_sha256()
    );
    let first_result = phase1::result(&first_plan);
    assert!(validate_phase1_result(&second_plan, &first_result).is_err());
}

#[test]
fn rejects_digest_only_target_pass_claim() {
    let plan = phase1::plan();
    let mut result = phase1::result(&plan);
    phase1::claims_target_pass(&mut result);
    assert!(validate_phase1_result(&plan, &result).is_err());
}

#[test]
fn rejects_every_individual_authority_bit_even_when_all_observations_match() {
    macro_rules! assert_rejected {
        ($field:ident) => {{
            let plan = phase1::plan();
            let mut result = phase1::result(&plan);
            phase1::authority_mut(&mut result).$field = true;
            assert!(
                validate_phase1_result(&plan, &result).is_err(),
                "authority bit {} escaped",
                stringify!($field)
            );
        }};
    }

    assert_rejected!(automatic_transition);
    assert_rejected!(canary);
    assert_rejected!(credential_use);
    assert_rejected!(cutover);
    assert_rejected!(default_ref_change);
    assert_rejected!(deletion);
    assert_rejected!(enforce);
    assert_rejected!(filesystem_mutation);
    assert_rejected!(full_matrix_claim);
    assert_rejected!(ga_claim);
    assert_rejected!(global_authority);
    assert_rejected!(install_activation);
    assert_rejected!(linux_gate_pass);
    assert_rejected!(local_ref_change);
    assert_rejected!(operator_acceptance);
    assert_rejected!(outbound);
    assert_rejected!(process_execution);
    assert_rejected!(production);
    assert_rejected!(promotion);
    assert_rejected!(qualification_authority);
    assert_rejected!(receipt_signing);
    assert_rejected!(recutover);
    assert_rejected!(remote_ref_change);
    assert_rejected!(retirement);
    assert_rejected!(rollback);
    assert_rejected!(service_control);
    assert_rejected!(snapshot);
    assert_rejected!(state_root_mutation);
    assert_rejected!(target_qualification_pass);
    assert_rejected!(watermark_state_mutation);
    assert_rejected!(writer_control);
}

#[test]
fn rejects_x230_collector_environment_even_with_exact_evidence() {
    let plan = phase1::plan();
    let mut result = phase1::result(&plan);
    phase1::set_fixture_environment(&mut result);
    assert!(validate_phase1_result(&plan, &result).is_err());
}

#[test]
fn blocked_observation_cannot_be_relabelled_structurally_complete() {
    let plan = phase1::plan();
    let mut result = phase1::result(&plan);
    phase1::set_first_observation_missing(&mut result);
    assert!(validate_phase1_result(&plan, &result).is_err());
    phase1::set_blocked_verdict_and_rebind(&plan, &mut result);
    validate_phase1_result(&plan, &result).expect("blocked no-authority result");
}

#[test]
fn mismatched_observation_is_bound_but_never_structurally_complete() {
    let plan = phase1::plan();
    let mut result = phase1::result(&plan);
    phase1::set_first_observation_mismatch(&mut result);
    assert!(validate_phase1_result(&plan, &result).is_err());
    phase1::set_blocked_verdict_and_rebind(&plan, &mut result);
    validate_phase1_result(&plan, &result).expect("mismatch remains blocked no-authority");
}

#[test]
fn strict_canonical_decoder_rejects_whitespace_and_unknown_fields() {
    let documents = profiles::documents();
    let canonical = canonical_json(&documents).expect("canonical");
    let decoded: PublishedProfileDocumentsV1 =
        decode_canonical_json(&canonical).expect("decode canonical");
    assert_eq!(decoded, documents);

    let mut padded = canonical;
    padded.push(b'\n');
    assert!(decode_canonical_json::<PublishedProfileDocumentsV1>(&padded).is_err());

    let mut unknown = serde_json::to_value(documents).expect("JSON");
    unknown["caller_selected_root"] = Value::String("forbidden".to_string());
    assert!(serde_json::from_value::<PublishedProfileDocumentsV1>(unknown).is_err());
}

fn mutated_documents(mutate: impl FnOnce(&mut Value)) -> PublishedProfileDocumentsV1 {
    let mut value = serde_json::to_value(profiles::documents()).expect("JSON");
    mutate(&mut value);
    serde_json::from_value(value).expect("typed mutated document")
}

use hepta_contracts::CapabilityId;
use hepta_contracts::CapabilityManifestRef;
use hepta_contracts::ContentHash;
use hepta_contracts::PreferenceEvidenceId;
use hepta_contracts::PreferenceEvidenceRef;
use hepta_contracts::PreferenceId;
use hepta_contracts::PreferenceState;
use hepta_contracts::PrincipalId;
use hepta_contracts::ReceiptId;
use hepta_contracts::ReceiptRef;
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;

use super::*;

#[test]
fn explicit_preference_genesis_and_reduction_have_golden_payloads_and_hashes()
-> Result<(), PreferenceReductionError> {
    let genesis = fixture_genesis();
    let evidence = evidence(
        "golden",
        genesis.subject().clone(),
        genesis.preference().clone(),
        genesis.target_binding_hash().clone(),
        ExplicitPreferenceSignal::Accepted,
    );
    let reduction =
        reduce_explicit_preference(genesis.state(), genesis.canonical_payload(), &evidence)?;

    assert_eq!(genesis.revision(), Revision::new(0));
    assert_eq!(genesis.accepted_count(), 0);
    assert_eq!(genesis.rejected_count(), 0);
    assert_eq!(
        genesis.target_binding_hash().as_str(),
        "sha256:05b7b83b516dadccadd8bf87907b98bd4b8a0feaec74061917419a8aed87ccc0"
    );
    assert_eq!(
        genesis.state().content_hash().as_str(),
        "sha256:900ab8bf25036556da2a00ce396dba58d2a94b5e26d5e94e94050af1efefb42c"
    );
    assert_eq!(
        genesis.canonical_payload(),
        "reducer=hepta.intelligence.explicit-preference.reducer.v1|schema=hepta.intelligence.explicit-preference.accumulator.v1|target=capability|subject=757365723a616c696365|preference=707265666572656e63653a746f6f6c2d63686f696365|capability_id=746f6f6c3a6563686f|capability_revision=7|capability_manifest_hash=7368613235363a6d616e69666573742d6563686f|catalog_revision=11|catalog_hash=7368613235363a636174616c6f672d6d61696e|target_binding_hash=7368613235363a30356237623833623531366461646363616464386266383739303762393862643462386130666561656337343036313931373431396138616564383763636330|revision=0|accepted=0|rejected=0"
    );
    assert_eq!(
        reduction.next_state().content_hash().as_str(),
        "sha256:3699296c24f0638b3e7bdbbcd8b1a791b84feb3af34501b529f5f3c269fecc10"
    );
    assert_eq!(
        reduction.canonical_payload(),
        "reducer=hepta.intelligence.explicit-preference.reducer.v1|schema=hepta.intelligence.explicit-preference.accumulator.v1|target=capability|subject=757365723a616c696365|preference=707265666572656e63653a746f6f6c2d63686f696365|capability_id=746f6f6c3a6563686f|capability_revision=7|capability_manifest_hash=7368613235363a6d616e69666573742d6563686f|catalog_revision=11|catalog_hash=7368613235363a636174616c6f672d6d61696e|target_binding_hash=7368613235363a30356237623833623531366461646363616464386266383739303762393862643462386130666561656337343036313931373431396138616564383763636330|revision=1|accepted=1|rejected=0"
    );
    Ok(())
}

#[test]
fn explicit_preference_accepted_and_rejected_increment_only_their_counters()
-> Result<(), PreferenceReductionError> {
    let genesis = fixture_genesis();
    let accepted_evidence =
        matching_evidence("accepted", &genesis, ExplicitPreferenceSignal::Accepted);
    let accepted = reduce_explicit_preference(
        genesis.state(),
        genesis.canonical_payload(),
        &accepted_evidence,
    )?;
    assert_eq!(accepted.next().revision(), Revision::new(1));
    assert_eq!(accepted.next().accepted_count(), 1);
    assert_eq!(accepted.next().rejected_count(), 0);
    assert_eq!(accepted.signal(), ExplicitPreferenceSignal::Accepted);

    let rejected_evidence = matching_evidence(
        "rejected",
        accepted.next(),
        ExplicitPreferenceSignal::Rejected,
    );
    let rejected = reduce_explicit_preference(
        accepted.next_state(),
        accepted.canonical_payload(),
        &rejected_evidence,
    )?;
    assert_eq!(rejected.next().revision(), Revision::new(2));
    assert_eq!(rejected.next().accepted_count(), 1);
    assert_eq!(rejected.next().rejected_count(), 1);
    assert_eq!(rejected.signal(), ExplicitPreferenceSignal::Rejected);
    Ok(())
}

#[test]
fn explicit_preference_rejects_payload_and_state_hash_tampering() {
    let genesis = fixture_genesis();
    let evidence = matching_evidence("tamper", &genesis, ExplicitPreferenceSignal::Accepted);
    let tampered_payload = genesis
        .canonical_payload()
        .replace("|accepted=0|", "|accepted=1|");
    assert!(matches!(
        reduce_explicit_preference(genesis.state(), &tampered_payload, &evidence),
        Err(PreferenceReductionError::PreviousHashMismatch { .. })
    ));

    let tampered_state = PreferenceState::new(
        genesis.revision(),
        ContentHash::new("sha256:caller-supplied-drift"),
    );
    assert!(matches!(
        reduce_explicit_preference(&tampered_state, genesis.canonical_payload(), &evidence),
        Err(PreferenceReductionError::PreviousHashMismatch { .. })
    ));
}

#[test]
fn explicit_preference_rejects_cross_subject_preference_and_target_evidence() {
    let genesis = fixture_genesis();
    let wrong_subject = evidence(
        "wrong-subject",
        PrincipalId::new("user:bob"),
        genesis.preference().clone(),
        genesis.target_binding_hash().clone(),
        ExplicitPreferenceSignal::Accepted,
    );
    assert_eq!(
        reduce_explicit_preference(genesis.state(), genesis.canonical_payload(), &wrong_subject,),
        Err(PreferenceReductionError::SubjectBindingMismatch)
    );

    let wrong_preference = evidence(
        "wrong-preference",
        genesis.subject().clone(),
        PreferenceId::new("preference:other"),
        genesis.target_binding_hash().clone(),
        ExplicitPreferenceSignal::Accepted,
    );
    assert_eq!(
        reduce_explicit_preference(
            genesis.state(),
            genesis.canonical_payload(),
            &wrong_preference,
        ),
        Err(PreferenceReductionError::PreferenceBindingMismatch)
    );

    let wrong_target = evidence(
        "wrong-target",
        genesis.subject().clone(),
        genesis.preference().clone(),
        target("other").binding_hash(),
        ExplicitPreferenceSignal::Accepted,
    );
    assert_eq!(
        reduce_explicit_preference(genesis.state(), genesis.canonical_payload(), &wrong_target),
        Err(PreferenceReductionError::TargetBindingMismatch)
    );
}

#[test]
fn explicit_preference_rejects_revision_and_target_payload_drift() {
    let genesis = fixture_genesis();
    let evidence = matching_evidence("drift", &genesis, ExplicitPreferenceSignal::Rejected);
    let wrong_revision =
        PreferenceState::new(Revision::new(1), genesis.state().content_hash().clone());
    assert!(matches!(
        reduce_explicit_preference(&wrong_revision, genesis.canonical_payload(), &evidence),
        Err(PreferenceReductionError::PreviousRevisionMismatch { .. })
    ));

    let target_marker = hex(genesis.target_binding_hash().as_str());
    let drifted_payload = genesis.canonical_payload().replace(
        &format!("target_binding_hash={target_marker}"),
        "target_binding_hash=7368613235363a6472696674",
    );
    let drifted_state =
        PreferenceState::new(genesis.revision(), canonical::state_hash(&drifted_payload));
    assert!(matches!(
        reduce_explicit_preference(&drifted_state, &drifted_payload, &evidence),
        Err(PreferenceReductionError::PayloadTargetBindingMismatch { .. })
    ));
}

#[test]
fn explicit_preference_checks_revision_and_both_counter_overflows() {
    let genesis = fixture_genesis();
    let revision_max = accumulator_with(
        &genesis,
        Revision::new(u64::MAX),
        genesis.accepted_count(),
        genesis.rejected_count(),
    );
    assert_eq!(
        reduce_explicit_preference(
            revision_max.state(),
            revision_max.canonical_payload(),
            &matching_evidence(
                "revision-overflow",
                &revision_max,
                ExplicitPreferenceSignal::Accepted,
            ),
        ),
        Err(PreferenceReductionError::RevisionOverflow)
    );

    let accepted_max = accumulator_with(&genesis, Revision::new(7), u64::MAX, 0);
    assert_eq!(
        reduce_explicit_preference(
            accepted_max.state(),
            accepted_max.canonical_payload(),
            &matching_evidence(
                "accepted-overflow",
                &accepted_max,
                ExplicitPreferenceSignal::Accepted,
            ),
        ),
        Err(PreferenceReductionError::CounterOverflow(
            ExplicitPreferenceSignal::Accepted
        ))
    );

    let rejected_max = accumulator_with(&genesis, Revision::new(7), 0, u64::MAX);
    assert_eq!(
        reduce_explicit_preference(
            rejected_max.state(),
            rejected_max.canonical_payload(),
            &matching_evidence(
                "rejected-overflow",
                &rejected_max,
                ExplicitPreferenceSignal::Rejected,
            ),
        ),
        Err(PreferenceReductionError::CounterOverflow(
            ExplicitPreferenceSignal::Rejected
        ))
    );
}

#[test]
fn explicit_preference_genesis_and_reduction_are_deterministic() {
    let first = fixture_genesis();
    let second = fixture_genesis();
    assert_eq!(first, second);

    let evidence = matching_evidence("deterministic", &first, ExplicitPreferenceSignal::Rejected);
    let first_reduction =
        reduce_explicit_preference(first.state(), first.canonical_payload(), &evidence);
    let second_reduction =
        reduce_explicit_preference(second.state(), second.canonical_payload(), &evidence);
    assert_eq!(first_reduction, second_reduction);
}

#[test]
fn explicit_preference_rejects_noncanonical_and_version_drifted_payloads() {
    let genesis = fixture_genesis();
    let evidence = matching_evidence("canonical", &genesis, ExplicitPreferenceSignal::Accepted);
    let noncanonical = genesis
        .canonical_payload()
        .replace("|revision=0|", "|revision=00|");
    assert_eq!(
        reduce_explicit_preference(genesis.state(), &noncanonical, &evidence),
        Err(PreferenceReductionError::NonCanonicalPreviousPayload)
    );
    let version_drift =
        genesis
            .canonical_payload()
            .replacen(EXPLICIT_PREFERENCE_REDUCER_VERSION, "reducer.v2", 1);
    assert_eq!(
        reduce_explicit_preference(genesis.state(), &version_drift, &evidence),
        Err(PreferenceReductionError::UnsupportedVersion)
    );
}

fn fixture_genesis() -> PreferenceAccumulator {
    explicit_preference_genesis(
        PrincipalId::new("user:alice"),
        PreferenceId::new("preference:tool-choice"),
        target("echo"),
    )
}

fn target(label: &str) -> ExplicitPreferenceTarget {
    ExplicitPreferenceTarget::Capability(CapabilityManifestRef::new(
        CapabilityId::new(format!("tool:{label}")),
        Revision::new(7),
        ContentHash::new(format!("sha256:manifest-{label}")),
        RevisionStamp::new(Revision::new(11), ContentHash::new("sha256:catalog-main")),
    ))
}

fn accumulator_with(
    base: &PreferenceAccumulator,
    revision: Revision,
    accepted_count: u64,
    rejected_count: u64,
) -> PreferenceAccumulator {
    build_accumulator(AccumulatorData {
        subject: base.subject().clone(),
        preference: base.preference().clone(),
        target: base.target().clone(),
        revision,
        accepted_count,
        rejected_count,
    })
}

fn matching_evidence(
    id: &str,
    accumulator: &PreferenceAccumulator,
    signal: ExplicitPreferenceSignal,
) -> PreferenceEvidenceRef {
    evidence(
        id,
        accumulator.subject().clone(),
        accumulator.preference().clone(),
        accumulator.target_binding_hash().clone(),
        signal,
    )
}

fn evidence(
    id: &str,
    subject: PrincipalId,
    preference: PreferenceId,
    target_binding_hash: ContentHash,
    signal: ExplicitPreferenceSignal,
) -> PreferenceEvidenceRef {
    PreferenceEvidenceRef::new(
        PreferenceEvidenceId::new(format!("evidence:{id}")),
        ContentHash::new(format!("sha256:evidence-{id}")),
        signal,
        ReceiptRef::new(
            ReceiptId::new(format!("receipt:{id}")),
            ContentHash::new(format!("sha256:receipt-{id}")),
        ),
        ContentHash::new(format!("sha256:session-{id}")),
        subject,
        preference,
        target_binding_hash,
    )
}

fn hex(value: &str) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.bytes() {
        encoded.push(DIGITS[(byte >> 4) as usize] as char);
        encoded.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    encoded
}

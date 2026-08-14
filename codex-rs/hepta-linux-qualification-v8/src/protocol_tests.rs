use pretty_assertions::assert_eq;

use super::*;
use crate::AUTHORITY_SCHEMA_V8;
use crate::AuthorityChallengeV8;
use crate::AuthorityReplayGuardV8;
use crate::AuthorityScopeV8;
use crate::AuthoritySignerBindingV8;
use crate::BootStampV8;
use crate::BreakGlassCapabilityV8;
use crate::CANDIDATE_HEAD;
use crate::CANDIDATE_TREE;
use crate::DriverPeerBindingV8;
use crate::JournalAssessmentV8;
use crate::JournalEffectV8;
use crate::JournalEventV8;
use crate::JournalRecordV8;
use crate::OneShotRunCapabilityV8;
use crate::RecoveryStateBindingV8;
use crate::SignedAuthorityV8;
use crate::TargetHostBindingV8;
use crate::VerifiedAuthorityV8;
use crate::canonical_authority_statement_for_test_v8;
use crate::test_only_trust_binding_v8;
use crate::validate_journal_v8;
use crate::verify_signed_authority_with_observation_for_test_v8;

#[test]
fn release_reads_durable_facts_instead_of_accepting_boolean_claims() {
    let attempt_identity = attempt();
    let mut state = AdmissionStateV8::new(attempt_identity.clone()).expect("state");
    state
        .apply(AdmissionEventV8::Arm {
            authority: verified_authority(run_scope(attempt_identity.clone()), 'a'),
        })
        .expect("arm");
    state
        .apply(AdmissionEventV8::Claim {
            capability_sha256: digest('1'),
        })
        .expect("claim");
    state
        .apply(AdmissionEventV8::RunnerStopped {
            evidence_sha256: digest('2'),
        })
        .expect("stop");
    state
        .apply(AdmissionEventV8::CandidatePublished {
            result: candidate(attempt_identity.clone(), CandidateOutcomeV8::Pass),
        })
        .expect("publish");
    let candidate = state.candidate_result.as_ref().expect("candidate").clone();
    let copy_ack = verified_copy_ack(&candidate);
    state
        .apply(AdmissionEventV8::CopyAcknowledged {
            verified: copy_ack.clone(),
        })
        .expect("copy ack");
    state
        .apply(AdmissionEventV8::RunnerRestored {
            evidence_sha256: digest('4'),
        })
        .expect("restore");
    state
        .apply(AdmissionEventV8::PostSnapshot {
            snapshot_sha256: digest('5'),
        })
        .expect("post snapshot");
    state
        .apply(AdmissionEventV8::AuthorizeRelease {
            journal: pre_release_journal(&candidate),
        })
        .expect("authorize release");
    assert_eq!(state.phase(), AdmissionPhaseV8::ReleaseAuthorized);
    assert!(state.barrier_armed());
    let release_authorization = state
        .release_authorization()
        .expect("internal release authorization")
        .clone();
    let release_manifest = release_authorization.release_manifest_sha256().to_string();
    let completed_journal = completed_release_journal(&candidate, &release_manifest);
    state
        .apply(AdmissionEventV8::CompleteRelease {
            journal: completed_journal.clone(),
        })
        .expect("complete release");
    assert_eq!(state.phase(), AdmissionPhaseV8::Released);
    assert!(!state.barrier_armed());
    assert!(state.qualification_pass());
    let receipt = state.final_receipt().expect("final receipt");
    let verified = verify_final_receipt_v8(
        &receipt,
        &attempt_identity,
        &candidate,
        &copy_ack,
        &completed_journal,
        &release_authorization,
    )
    .expect("full-chain final receipt verification");
    assert!(verified.qualification_pass());
    assert_eq!(verified.receipt_sha256(), receipt.sha256().unwrap());
}

#[test]
fn copy_ack_verifies_real_signature_bytes_binding_and_replay() {
    let candidate = candidate(attempt(), CandidateOutcomeV8::Pass);
    let ack = copy_ack(&candidate);
    let statement_sha256 = sha256(&ack.canonical_statement_for_test().expect("statement"));
    let observation = copy_ack_observation(&ack, statement_sha256);
    let mut replay = CopyAckReplayGuardV8::default();
    assert!(ack.canonical_statement().is_err());
    assert!(replay.verify_and_consume(&ack, &candidate, 1_500).is_err());
    let verified = replay
        .verify_and_consume_with_observation_for_test(&ack, &candidate, &observation, 1_500)
        .expect("copy acknowledgement");
    assert_eq!(
        verified.trust_policy_binding(),
        &test_only_trust_binding_v8(SshsigTrustPurposeV8::MacCopyAck)
    );
    assert!(
        replay
            .verify_and_consume_with_observation_for_test(&ack, &candidate, &observation, 1_500,)
            .is_err()
    );

    let forged = CryptographicSignatureObservation::for_test_only(
        digest('f'),
        sha256(&ack.canonical_statement_for_test().unwrap()),
        SshsigTrustPurposeV8::OneShotRunAuthority,
    );
    assert!(
        CopyAckReplayGuardV8::default()
            .verify_and_consume_with_observation_for_test(&ack, &candidate, &forged, 1_500)
            .is_err()
    );
}

#[test]
fn recovery_and_break_glass_never_release_or_pass() {
    let attempt_identity = attempt();
    for event in [
        AdmissionEventV8::RecoverAfterCrash {
            recovery_evidence_sha256: digest('8'),
        },
        AdmissionEventV8::BreakGlassRestore {
            authority: verified_authority(break_glass_scope(attempt_identity.clone()), 'b'),
            current_state: recovery_state(&attempt_identity),
            restore_evidence_sha256: digest('9'),
        },
    ] {
        let mut state = AdmissionStateV8::new(attempt_identity.clone()).expect("state");
        state
            .apply(AdmissionEventV8::Arm {
                authority: verified_authority(run_scope(attempt_identity.clone()), 'a'),
            })
            .expect("arm");
        state.apply(event).expect("recovery");
        assert_eq!(state.phase(), AdmissionPhaseV8::Abandoned);
        assert!(state.barrier_armed());
        assert!(state.permanent_quarantine());
        assert!(state.qualification_abandoned());
        assert!(!state.qualification_pass());
    }
}

#[test]
fn failed_candidate_can_restore_and_release_but_cannot_qualify() {
    let attempt = attempt();
    let mut state = AdmissionStateV8::new(attempt.clone()).expect("state");
    let candidate = candidate(attempt.clone(), CandidateOutcomeV8::Fail);
    let copy_ack = verified_copy_ack(&candidate);
    for event in [
        AdmissionEventV8::Arm {
            authority: verified_authority(run_scope(attempt), 'a'),
        },
        AdmissionEventV8::Claim {
            capability_sha256: digest('1'),
        },
        AdmissionEventV8::RunnerStopped {
            evidence_sha256: digest('2'),
        },
        AdmissionEventV8::CandidatePublished {
            result: candidate.clone(),
        },
        AdmissionEventV8::CopyAcknowledged { verified: copy_ack },
        AdmissionEventV8::RunnerRestored {
            evidence_sha256: digest('4'),
        },
        AdmissionEventV8::PostSnapshot {
            snapshot_sha256: digest('5'),
        },
    ] {
        state.apply(event).expect("ordered transition");
    }
    state
        .apply(AdmissionEventV8::AuthorizeRelease {
            journal: pre_release_journal(&candidate),
        })
        .expect("authorize failed-candidate release");
    let release_manifest = state
        .release_authorization()
        .expect("release authorization")
        .release_manifest_sha256()
        .to_string();
    state
        .apply(AdmissionEventV8::CompleteRelease {
            journal: completed_release_journal(&candidate, &release_manifest),
        })
        .expect("complete failed-candidate release");
    let receipt = state.final_receipt().expect("failure receipt");
    assert_eq!(receipt.outcome, CandidateOutcomeV8::Fail);
    assert!(!receipt.qualification_pass);
}

#[test]
fn candidate_result_rejects_nonempty_or_delegated_containment() {
    let mut result = candidate(attempt(), CandidateOutcomeV8::Pass);
    result.containment.observed_process_count = 1;
    assert!(result.validate().is_err());
    result.containment.observed_process_count = 0;
    result.containment.delegated_controller_count = 1;
    assert!(result.validate().is_err());
}

#[test]
fn candidate_and_copy_ack_digests_bind_every_mutable_security_field() {
    let baseline = candidate(attempt(), CandidateOutcomeV8::Pass);
    let baseline_digest = baseline.sha256().unwrap();
    let mut variants = Vec::new();

    let mut changed = baseline.clone();
    changed.containment.cgroup_path = "/hepta-vnext/linux-v8/attempt-b".to_string();
    variants.push(changed);
    let mut changed = baseline.clone();
    changed.containment.root_observation_sha256 = digest('a');
    variants.push(changed);
    let mut changed = baseline.clone();
    changed.manifest_sha256 = digest('b');
    variants.push(changed);
    let mut changed = baseline.clone();
    changed.source.inode += 1;
    variants.push(changed);
    let mut changed = baseline.clone();
    changed.source.size_bytes += 1;
    variants.push(changed);

    assert!(
        variants
            .iter()
            .all(|variant| { variant.sha256().expect("valid changed bundle") != baseline_digest })
    );

    let ack = copy_ack(&baseline);
    let baseline_statement = ack.canonical_statement_for_test().unwrap();
    let mut changed_ack = ack.clone();
    changed_ack.copied_publication.size_bytes += 1;
    assert_ne!(
        changed_ack.canonical_statement_for_test().unwrap(),
        baseline_statement
    );

    let mut overlong = ack;
    overlong.valid_before_unix_seconds += 1;
    assert!(overlong.canonical_statement_for_test().is_err());
}

#[test]
fn release_rejects_a_valid_journal_bound_to_different_durable_facts() {
    let attempt = attempt();
    let candidate = candidate(attempt.clone(), CandidateOutcomeV8::Pass);
    let mut state = AdmissionStateV8::new(attempt.clone()).unwrap();
    for event in [
        AdmissionEventV8::Arm {
            authority: verified_authority(run_scope(attempt), 'a'),
        },
        AdmissionEventV8::Claim {
            capability_sha256: digest('1'),
        },
        AdmissionEventV8::RunnerStopped {
            evidence_sha256: digest('f'),
        },
        AdmissionEventV8::CandidatePublished {
            result: candidate.clone(),
        },
        AdmissionEventV8::CopyAcknowledged {
            verified: verified_copy_ack(&candidate),
        },
        AdmissionEventV8::RunnerRestored {
            evidence_sha256: digest('4'),
        },
        AdmissionEventV8::PostSnapshot {
            snapshot_sha256: digest('5'),
        },
    ] {
        state.apply(event).unwrap();
    }
    assert!(
        state
            .apply(AdmissionEventV8::AuthorizeRelease {
                journal: pre_release_journal(&candidate),
            })
            .is_err()
    );
}

#[test]
fn externally_preclaimed_release_cannot_authorize_or_complete_the_barrier() {
    let candidate = candidate(attempt(), CandidateOutcomeV8::Pass);
    let mut state = post_snapshot_state(&candidate, digest('2'));

    assert!(
        state
            .apply(AdmissionEventV8::AuthorizeRelease {
                journal: completed_release_journal(&candidate, &digest('f')),
            })
            .is_err()
    );
    assert_eq!(state.phase(), AdmissionPhaseV8::PostSnapshot);
    assert!(state.barrier_armed());

    state
        .apply(AdmissionEventV8::AuthorizeRelease {
            journal: pre_release_journal(&candidate),
        })
        .expect("internal authorization after exact pre-release prefix");
    assert_eq!(state.phase(), AdmissionPhaseV8::ReleaseAuthorized);
    assert!(state.final_receipt().is_err());
    assert!(state.barrier_armed());

    let release_manifest = state
        .release_authorization()
        .expect("release authorization")
        .release_manifest_sha256()
        .to_string();
    assert!(
        state
            .apply(AdmissionEventV8::CompleteRelease {
                journal: completed_release_journal(&candidate, &digest('e')),
            })
            .is_err()
    );
    assert!(state.barrier_armed());
    state
        .apply(AdmissionEventV8::CompleteRelease {
            journal: completed_release_journal(&candidate, &release_manifest),
        })
        .expect("exact authorized release completion");
    assert!(!state.barrier_armed());
}

#[test]
fn final_receipt_rejects_redundant_verdict_tampering() {
    let mut receipt = FinalReceiptBindingV8 {
        attempt_identity_sha256: digest('1'),
        candidate_result_bundle_sha256: digest('2'),
        copy_ack_statement_sha256: digest('3'),
        journal_tip_sha256: digest('4'),
        outcome: CandidateOutcomeV8::Pass,
        post_snapshot_sha256: digest('5'),
        qualification_pass: false,
        restore_evidence_sha256: digest('6'),
        release_authorization_sha256: digest('7'),
        barrier_release_observation_sha256: digest('8'),
    };
    assert!(receipt.validate_shape().is_err());
    receipt.qualification_pass = true;
    assert!(receipt.sha256().is_ok());
}

#[test]
fn shape_valid_final_receipt_cannot_forge_the_typed_chain() {
    let (attempt, candidate, copy_ack, journal, authorization, receipt) =
        completed_release_context(CandidateOutcomeV8::Pass);
    assert!(
        verify_final_receipt_v8(
            &receipt,
            &attempt,
            &candidate,
            &copy_ack,
            &journal,
            &authorization,
        )
        .is_ok()
    );

    let mut variants = Vec::new();
    for field in 0..8 {
        let mut changed = receipt.clone();
        match field {
            0 => changed.attempt_identity_sha256 = digest('a'),
            1 => changed.candidate_result_bundle_sha256 = digest('b'),
            2 => changed.copy_ack_statement_sha256 = digest('c'),
            3 => changed.journal_tip_sha256 = digest('d'),
            4 => changed.post_snapshot_sha256 = digest('e'),
            5 => changed.restore_evidence_sha256 = digest('f'),
            6 => changed.release_authorization_sha256 = digest('7'),
            7 => changed.barrier_release_observation_sha256 = digest('8'),
            _ => unreachable!(),
        }
        assert!(changed.validate_shape().is_ok());
        variants.push(changed);
    }
    for forged in variants {
        assert!(
            verify_final_receipt_v8(
                &forged,
                &attempt,
                &candidate,
                &copy_ack,
                &journal,
                &authorization,
            )
            .is_err()
        );
    }

    assert!(
        verify_final_receipt_v8(
            &receipt,
            &attempt,
            &candidate,
            &copy_ack,
            &pre_release_journal(&candidate),
            &authorization,
        )
        .is_err()
    );

    let mut wrong_authorization = authorization;
    wrong_authorization.release_manifest_sha256 = digest('9');
    assert!(
        verify_final_receipt_v8(
            &receipt,
            &attempt,
            &candidate,
            &copy_ack,
            &journal,
            &wrong_authorization,
        )
        .is_err()
    );
}

#[test]
fn full_chain_verifier_accepts_exact_fail_without_upgrading_it_to_pass() {
    let (attempt, candidate, copy_ack, journal, authorization, receipt) =
        completed_release_context(CandidateOutcomeV8::Fail);
    let verified = verify_final_receipt_v8(
        &receipt,
        &attempt,
        &candidate,
        &copy_ack,
        &journal,
        &authorization,
    )
    .expect("exact failure receipt remains admissible as failure evidence");
    assert!(!verified.qualification_pass());
    assert!(!receipt.qualification_pass);
}

fn completed_release_context(
    outcome: CandidateOutcomeV8,
) -> (
    AttemptIdentityV8,
    CandidateResultBundleV8,
    VerifiedCopyAckV8,
    JournalAssessmentV8,
    BarrierReleaseAuthorizationV8,
    FinalReceiptBindingV8,
) {
    let attempt = attempt();
    let candidate = candidate(attempt.clone(), outcome);
    let copy_ack = verified_copy_ack(&candidate);
    let mut state = AdmissionStateV8::new(attempt.clone()).expect("state");
    for event in [
        AdmissionEventV8::Arm {
            authority: verified_authority(run_scope(attempt.clone()), 'a'),
        },
        AdmissionEventV8::Claim {
            capability_sha256: digest('1'),
        },
        AdmissionEventV8::RunnerStopped {
            evidence_sha256: digest('2'),
        },
        AdmissionEventV8::CandidatePublished {
            result: candidate.clone(),
        },
        AdmissionEventV8::CopyAcknowledged {
            verified: copy_ack.clone(),
        },
        AdmissionEventV8::RunnerRestored {
            evidence_sha256: digest('4'),
        },
        AdmissionEventV8::PostSnapshot {
            snapshot_sha256: digest('5'),
        },
    ] {
        state.apply(event).expect("ordered release context");
    }
    state
        .apply(AdmissionEventV8::AuthorizeRelease {
            journal: pre_release_journal(&candidate),
        })
        .expect("authorize release context");
    let authorization = state
        .release_authorization()
        .expect("release authorization")
        .clone();
    let journal = completed_release_journal(&candidate, authorization.release_manifest_sha256());
    state
        .apply(AdmissionEventV8::CompleteRelease {
            journal: journal.clone(),
        })
        .expect("complete release context");
    let receipt = state.final_receipt().expect("final receipt context");
    (
        attempt,
        candidate,
        copy_ack,
        journal,
        authorization,
        receipt,
    )
}

fn attempt() -> AttemptIdentityV8 {
    AttemptIdentityV8 {
        attempt_nonce: digest('a'),
        barrier_generation: 8,
        candidate_head: CANDIDATE_HEAD.to_string(),
        candidate_tree: CANDIDATE_TREE.to_string(),
        driver_manifest_sha256: digest('b'),
        profile_manifest_sha256: digest('c'),
        parameter_manifest_sha256: digest('d'),
        machine_id_sha256: digest('e'),
        runner_snapshot_sha256: digest('1'),
        restore_plan_sha256: digest('2'),
    }
}

fn candidate(attempt: AttemptIdentityV8, outcome: CandidateOutcomeV8) -> CandidateResultBundleV8 {
    let attempt_identity_sha256 = attempt.sha256().expect("attempt digest");
    CandidateResultBundleV8 {
        attempt,
        containment: CandidateContainmentEvidenceV8 {
            attempt_identity_sha256,
            cgroup_path: "/hepta-vnext/linux-v8/attempt-a".to_string(),
            delegated_controller_count: 0,
            observed_process_count: 0,
            owner_gid: 0,
            owner_uid: 0,
            populated_value: 0,
            root_observation_sha256: digest('3'),
        },
        manifest_sha256: digest('4'),
        outcome,
        publication_method: NoReplacePublicationMethodV8::RenameAt2NoReplaceFileAndDirectoryFsync,
        source: PublishedFileIdentityV8 {
            device: 1,
            inode: 2,
            mode: 0o600,
            nlink: 1,
            sha256: digest('5'),
            size_bytes: 4096,
        },
    }
}

fn post_snapshot_state(
    candidate: &CandidateResultBundleV8,
    runner_stop_evidence_sha256: String,
) -> AdmissionStateV8 {
    let mut state = AdmissionStateV8::new(candidate.attempt.clone()).expect("state");
    for event in [
        AdmissionEventV8::Arm {
            authority: verified_authority(run_scope(candidate.attempt.clone()), 'a'),
        },
        AdmissionEventV8::Claim {
            capability_sha256: digest('1'),
        },
        AdmissionEventV8::RunnerStopped {
            evidence_sha256: runner_stop_evidence_sha256,
        },
        AdmissionEventV8::CandidatePublished {
            result: candidate.clone(),
        },
        AdmissionEventV8::CopyAcknowledged {
            verified: verified_copy_ack(candidate),
        },
        AdmissionEventV8::RunnerRestored {
            evidence_sha256: digest('4'),
        },
        AdmissionEventV8::PostSnapshot {
            snapshot_sha256: digest('5'),
        },
    ] {
        state.apply(event).expect("ordered pre-release transition");
    }
    state
}

fn copy_ack(candidate: &CandidateResultBundleV8) -> MacCopyAckV8 {
    let trust_policy = test_only_trust_binding_v8(SshsigTrustPurposeV8::MacCopyAck);
    MacCopyAckV8 {
        allowed_signers_sha256: trust_policy.allowed_signers_sha256().to_string(),
        attempt_identity_sha256: candidate.attempt.sha256().expect("attempt digest"),
        candidate_result_bundle_sha256: candidate.sha256().expect("candidate digest"),
        challenge_nonce: digest('7'),
        copied_manifest_sha256: digest('8'),
        copied_publication: PublishedFileIdentityV8 {
            device: 9,
            inode: 10,
            mode: 0o600,
            nlink: 1,
            sha256: digest('9'),
            size_bytes: 8192,
        },
        issued_unix_seconds: 1_000,
        linux_source_device: candidate.source.device,
        linux_source_inode: candidate.source.inode,
        principal: trust_policy.principal().to_string(),
        signature_bytes: b"real detached sshsig bytes fixture".to_vec(),
        signer_fingerprint: trust_policy.key_fingerprint().to_string(),
        valid_before_unix_seconds: 1_900,
    }
}

fn copy_ack_observation(
    ack: &MacCopyAckV8,
    statement_sha256: String,
) -> CryptographicSignatureObservation {
    CryptographicSignatureObservation::for_test_only(
        sha256(&ack.signature_bytes),
        statement_sha256,
        SshsigTrustPurposeV8::MacCopyAck,
    )
}

fn verified_copy_ack(candidate: &CandidateResultBundleV8) -> VerifiedCopyAckV8 {
    let ack = copy_ack(candidate);
    let observation = copy_ack_observation(
        &ack,
        sha256(
            &ack.canonical_statement_for_test()
                .expect("copy-ack statement"),
        ),
    );
    CopyAckReplayGuardV8::default()
        .verify_and_consume_with_observation_for_test(&ack, candidate, &observation, 1_500)
        .expect("verified copy acknowledgement")
}

fn run_scope(attempt: AttemptIdentityV8) -> AuthorityScopeV8 {
    AuthorityScopeV8::OneShotRun {
        target_host: TargetHostBindingV8 {
            machine_id_sha256: attempt.machine_id_sha256.clone(),
        },
        attempt,
        capability: OneShotRunCapabilityV8::Runner22And23SharedProcessGroupSigstopThenSigcontOnly,
        driver_peer: DriverPeerBindingV8 {
            executable_sha256: digest('b'),
            gid: 1000,
            uid: 1000,
        },
    }
}

fn break_glass_scope(attempt: AttemptIdentityV8) -> AuthorityScopeV8 {
    AuthorityScopeV8::BreakGlass {
        restore_plan_sha256: attempt.restore_plan_sha256.clone(),
        recovery_state: recovery_state(&attempt),
        target_host: TargetHostBindingV8 {
            machine_id_sha256: attempt.machine_id_sha256.clone(),
        },
        attempt,
        capability: BreakGlassCapabilityV8::ExactRestorePlanThenAbandonKeepQuarantineAndBarrier,
    }
}

fn recovery_state(attempt: &AttemptIdentityV8) -> RecoveryStateBindingV8 {
    RecoveryStateBindingV8 {
        boot_epoch: 1,
        boot_id: TEST_BOOT_ID.to_string(),
        journal_tip_sha256: digest('c'),
        restore_state_sha256: digest('d'),
        runner_snapshot_sha256: attempt.runner_snapshot_sha256.clone(),
    }
}

fn verified_authority(scope: AuthorityScopeV8, nonce: char) -> VerifiedAuthorityV8 {
    let purpose = match &scope {
        AuthorityScopeV8::Install { .. } => SshsigTrustPurposeV8::InstallAuthority,
        AuthorityScopeV8::OneShotRun { .. } => SshsigTrustPurposeV8::OneShotRunAuthority,
        AuthorityScopeV8::BreakGlass { .. } => SshsigTrustPurposeV8::BreakGlassAuthority,
    };
    let trust_policy = test_only_trust_binding_v8(purpose);
    let namespace = trust_policy.namespace();
    let challenge = AuthorityChallengeV8 {
        authority_nonce: digest(nonce),
        expires_at_unix_seconds: 1_900,
        issued_at_unix_seconds: 1_000,
        namespace: namespace.to_string(),
        schema: AUTHORITY_SCHEMA_V8.to_string(),
        scope,
        signer: AuthoritySignerBindingV8 {
            allowed_signers_sha256: trust_policy.allowed_signers_sha256().to_string(),
            key_fingerprint: trust_policy.key_fingerprint().to_string(),
            principal: trust_policy.principal().to_string(),
            signature_algorithm: trust_policy.signature_algorithm(),
        },
    };
    let statement =
        canonical_authority_statement_for_test_v8(&challenge).expect("authority statement");
    let signature_bytes = format!("test authority signature {nonce}").into_bytes();
    let signed = SignedAuthorityV8 {
        canonical_statement_sha256: sha256(&statement),
        challenge,
        detached_signature_sha256: sha256(&signature_bytes),
        detached_signature_bytes: signature_bytes,
    };
    let observation = CryptographicSignatureObservation::for_test_only(
        signed.detached_signature_sha256.clone(),
        signed.canonical_statement_sha256.clone(),
        purpose,
    );
    verify_signed_authority_with_observation_for_test_v8(
        &signed,
        &observation,
        1_500,
        &mut AuthorityReplayGuardV8::default(),
    )
    .expect("verified authority")
}

const TEST_BOOT_ID: &str = "11111111-1111-1111-1111-111111111111";

fn pre_release_journal(candidate: &CandidateResultBundleV8) -> JournalAssessmentV8 {
    validate_journal_v8(&pre_release_records(candidate)).expect("pre-release journal")
}

fn completed_release_journal(
    candidate: &CandidateResultBundleV8,
    release_manifest_sha256: &str,
) -> JournalAssessmentV8 {
    let mut records = pre_release_records(candidate);
    append_effect_with_manifest(
        &mut records,
        &candidate.attempt,
        JournalEffectV8::BarrierRelease,
        release_manifest_sha256.to_string(),
        digest('6'),
    );
    validate_journal_v8(&records).expect("completed release journal")
}

fn pre_release_records(candidate: &CandidateResultBundleV8) -> Vec<JournalRecordV8> {
    let attempt = &candidate.attempt;
    let mut records = vec![
        JournalRecordV8::new(
            attempt.clone(),
            1,
            BootStampV8 {
                boot_epoch: 1,
                boot_id: TEST_BOOT_ID.to_string(),
                boot_seq: 1,
                monotonic_ns: 100,
            },
            None,
            JournalEventV8::AttemptOpened {
                authority_manifest_sha256: digest('a'),
            },
        )
        .expect("opened journal"),
    ];
    append_effect(
        &mut records,
        attempt,
        JournalEffectV8::RunnerStop,
        digest('2'),
    );
    append_effect(
        &mut records,
        attempt,
        JournalEffectV8::CandidateExecution,
        digest('3'),
    );
    append_journal(
        &mut records,
        attempt,
        JournalEventV8::CandidateCompleted {
            candidate_result_sha256: candidate.sha256().expect("candidate digest"),
        },
    );
    append_effect(
        &mut records,
        attempt,
        JournalEffectV8::CandidateRelay,
        verified_copy_ack(candidate).statement_sha256().to_string(),
    );
    append_effect(
        &mut records,
        attempt,
        JournalEffectV8::RunnerRestore,
        digest('4'),
    );
    append_effect(
        &mut records,
        attempt,
        JournalEffectV8::PostRestoreSnapshot,
        digest('5'),
    );
    records
}

fn append_effect(
    records: &mut Vec<JournalRecordV8>,
    attempt: &AttemptIdentityV8,
    effect: JournalEffectV8,
    observation_sha256: String,
) {
    append_effect_with_manifest(records, attempt, effect, digest('e'), observation_sha256);
}

fn append_effect_with_manifest(
    records: &mut Vec<JournalRecordV8>,
    attempt: &AttemptIdentityV8,
    effect: JournalEffectV8,
    effect_manifest_sha256: String,
    observation_sha256: String,
) {
    append_journal(
        records,
        attempt,
        JournalEventV8::EffectIntent {
            effect,
            effect_manifest_sha256,
        },
    );
    let intent_record_sha256 = records.last().expect("intent").record_sha256.clone();
    append_journal(
        records,
        attempt,
        JournalEventV8::EffectObserved {
            effect,
            intent_record_sha256,
            observation_sha256,
        },
    );
}

fn append_journal(
    records: &mut Vec<JournalRecordV8>,
    attempt: &AttemptIdentityV8,
    event: JournalEventV8,
) {
    let previous = records.last().expect("journal predecessor");
    let global_seq = previous.global_seq + 1;
    records.push(
        JournalRecordV8::new(
            attempt.clone(),
            global_seq,
            BootStampV8 {
                boot_epoch: 1,
                boot_id: TEST_BOOT_ID.to_string(),
                boot_seq: global_seq,
                monotonic_ns: global_seq * 100,
            },
            Some(previous.record_sha256.clone()),
            event,
        )
        .expect("journal record"),
    );
}

fn digest(byte: char) -> String {
    byte.to_string().repeat(64)
}

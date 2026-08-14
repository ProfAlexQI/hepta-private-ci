use pretty_assertions::assert_eq;

use super::*;
use crate::AUTHORITY_SCHEMA_V8;
use crate::AuthorityChallengeV8;
use crate::AuthorityReplayGuardV8;
use crate::AuthorityScopeV8;
use crate::AuthoritySignatureAlgorithmV8;
use crate::AuthoritySignerBindingV8;
use crate::BREAK_GLASS_NAMESPACE_V8;
use crate::BootStampV8;
use crate::BreakGlassCapabilityV8;
use crate::CANDIDATE_HEAD;
use crate::CANDIDATE_TREE;
use crate::DriverPeerBindingV8;
use crate::INSTALL_NAMESPACE_V8;
use crate::JournalAssessmentV8;
use crate::JournalEffectV8;
use crate::JournalEventV8;
use crate::JournalRecordV8;
use crate::ONE_SHOT_RUN_NAMESPACE_V8;
use crate::OneShotRunCapabilityV8;
use crate::RecoveryStateBindingV8;
use crate::SignedAuthorityV8;
use crate::TargetHostBindingV8;
use crate::VerifiedAuthorityV8;
use crate::canonical_authority_statement_v8;
use crate::validate_journal_v8;
use crate::verify_signed_authority_v8;

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
            result: candidate(attempt_identity, CandidateOutcomeV8::Pass),
        })
        .expect("publish");
    let candidate = state.candidate_result.as_ref().expect("candidate").clone();
    state
        .apply(AdmissionEventV8::CopyAcknowledged {
            verified: verified_copy_ack(&candidate),
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
        .apply(AdmissionEventV8::Release {
            journal: release_journal(&candidate),
        })
        .expect("release");
    assert_eq!(state.phase(), AdmissionPhaseV8::Released);
    assert!(!state.barrier_armed());
    assert!(state.qualification_pass());
    assert!(state.final_receipt().is_ok());
}

#[test]
fn copy_ack_verifies_real_signature_bytes_binding_and_replay() {
    let candidate = candidate(attempt(), CandidateOutcomeV8::Pass);
    let ack = copy_ack(&candidate);
    let statement_sha256 = sha256(&ack.canonical_statement().expect("statement"));
    let observation = copy_ack_observation(&ack, statement_sha256);
    let mut replay = CopyAckReplayGuardV8::default();
    replay
        .verify_and_consume(&ack, &candidate, &observation, 1_500)
        .expect("copy acknowledgement");
    assert!(
        replay
            .verify_and_consume(&ack, &candidate, &observation, 1_500)
            .is_err()
    );

    let forged = CryptographicSignatureObservation::for_test_only(
        digest('f'),
        sha256(&ack.canonical_statement().unwrap()),
        ack.allowed_signers_sha256.clone(),
        ack.signer_fingerprint.clone(),
        COPY_ACK_NAMESPACE_V8.to_string(),
        ack.principal.clone(),
    );
    assert!(
        CopyAckReplayGuardV8::default()
            .verify_and_consume(&ack, &candidate, &forged, 1_500)
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
    let journal = release_journal(&candidate);
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
        AdmissionEventV8::CandidatePublished { result: candidate },
        AdmissionEventV8::CopyAcknowledged { verified: copy_ack },
        AdmissionEventV8::RunnerRestored {
            evidence_sha256: digest('4'),
        },
        AdmissionEventV8::PostSnapshot {
            snapshot_sha256: digest('5'),
        },
        AdmissionEventV8::Release { journal },
    ] {
        state.apply(event).expect("ordered transition");
    }
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
    let baseline_statement = ack.canonical_statement().unwrap();
    let mut changed_ack = ack.clone();
    changed_ack.copied_publication.size_bytes += 1;
    assert_ne!(
        changed_ack.canonical_statement().unwrap(),
        baseline_statement
    );

    let mut overlong = ack;
    overlong.valid_before_unix_seconds += 1;
    assert!(overlong.canonical_statement().is_err());
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
            .apply(AdmissionEventV8::Release {
                journal: release_journal(&candidate),
            })
            .is_err()
    );
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
    };
    assert!(receipt.validate().is_err());
    receipt.qualification_pass = true;
    assert!(receipt.sha256().is_ok());
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

fn copy_ack(candidate: &CandidateResultBundleV8) -> MacCopyAckV8 {
    MacCopyAckV8 {
        allowed_signers_sha256: digest('6'),
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
        principal: "hepta-linux-v8-operator".to_string(),
        signature_bytes: b"real detached sshsig bytes fixture".to_vec(),
        signer_fingerprint: format!("SHA256:{}", "A".repeat(43)),
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
        ack.allowed_signers_sha256.clone(),
        ack.signer_fingerprint.clone(),
        COPY_ACK_NAMESPACE_V8.to_string(),
        ack.principal.clone(),
    )
}

fn verified_copy_ack(candidate: &CandidateResultBundleV8) -> VerifiedCopyAckV8 {
    let ack = copy_ack(candidate);
    let observation = copy_ack_observation(
        &ack,
        sha256(&ack.canonical_statement().expect("copy-ack statement")),
    );
    CopyAckReplayGuardV8::default()
        .verify_and_consume(&ack, candidate, &observation, 1_500)
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
    let namespace = match &scope {
        AuthorityScopeV8::Install { .. } => INSTALL_NAMESPACE_V8,
        AuthorityScopeV8::OneShotRun { .. } => ONE_SHOT_RUN_NAMESPACE_V8,
        AuthorityScopeV8::BreakGlass { .. } => BREAK_GLASS_NAMESPACE_V8,
    };
    let challenge = AuthorityChallengeV8 {
        authority_nonce: digest(nonce),
        expires_at_unix_seconds: 1_900,
        issued_at_unix_seconds: 1_000,
        namespace: namespace.to_string(),
        schema: AUTHORITY_SCHEMA_V8.to_string(),
        scope,
        signer: AuthoritySignerBindingV8 {
            allowed_signers_sha256: digest('f'),
            key_fingerprint: format!("SHA256:{}", "A".repeat(43)),
            principal: "linux-v8-operator@example".to_string(),
            signature_algorithm: AuthoritySignatureAlgorithmV8::OpenSshSshsigEd25519,
        },
    };
    let statement = canonical_authority_statement_v8(&challenge).expect("authority statement");
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
        signed.challenge.signer.allowed_signers_sha256.clone(),
        signed.challenge.signer.key_fingerprint.clone(),
        signed.challenge.namespace.clone(),
        signed.challenge.signer.principal.clone(),
    );
    verify_signed_authority_v8(
        &signed,
        &observation,
        1_500,
        &mut AuthorityReplayGuardV8::default(),
    )
    .expect("verified authority")
}

const TEST_BOOT_ID: &str = "11111111-1111-1111-1111-111111111111";

fn release_journal(candidate: &CandidateResultBundleV8) -> JournalAssessmentV8 {
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
        JournalEffectV8::BarrierRelease,
        digest('5'),
    );
    validate_journal_v8(&records).expect("release journal")
}

fn append_effect(
    records: &mut Vec<JournalRecordV8>,
    attempt: &AttemptIdentityV8,
    effect: JournalEffectV8,
    observation_sha256: String,
) {
    append_journal(
        records,
        attempt,
        JournalEventV8::EffectIntent {
            effect,
            effect_manifest_sha256: digest('e'),
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

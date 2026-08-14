use super::*;
use crate::CANDIDATE_HEAD;
use crate::CANDIDATE_TREE;
use pretty_assertions::assert_eq;

const BOOT_ONE: &str = "11111111-1111-1111-1111-111111111111";
const BOOT_TWO: &str = "22222222-2222-2222-2222-222222222222";

fn digest(byte: char) -> String {
    std::iter::repeat_n(byte, 64).collect()
}

fn attempt() -> AttemptIdentityV8 {
    AttemptIdentityV8 {
        attempt_nonce: digest('1'),
        barrier_generation: 9,
        candidate_head: CANDIDATE_HEAD.to_string(),
        candidate_tree: CANDIDATE_TREE.to_string(),
        driver_manifest_sha256: digest('2'),
        profile_manifest_sha256: digest('3'),
        parameter_manifest_sha256: digest('4'),
        machine_id_sha256: digest('5'),
        runner_snapshot_sha256: digest('6'),
        restore_plan_sha256: digest('7'),
    }
}

fn stamp(boot_epoch: u64, boot_id: &str, boot_seq: u64, monotonic_ns: u64) -> BootStampV8 {
    BootStampV8 {
        boot_epoch,
        boot_id: boot_id.to_string(),
        boot_seq,
        monotonic_ns,
    }
}

fn opened() -> JournalRecordV8 {
    JournalRecordV8::new(
        attempt(),
        1,
        stamp(1, BOOT_ONE, 1, 500),
        None,
        JournalEventV8::AttemptOpened {
            authority_manifest_sha256: digest('8'),
        },
    )
    .expect("valid first record")
}

fn append(records: &mut Vec<JournalRecordV8>, boot: BootStampV8, event: JournalEventV8) {
    let previous = records.last().expect("existing predecessor");
    records.push(
        JournalRecordV8::new(
            attempt(),
            previous.global_seq + 1,
            boot,
            Some(previous.record_sha256.clone()),
            event,
        )
        .expect("valid record content"),
    );
}

fn append_effect(
    records: &mut Vec<JournalRecordV8>,
    boot_seq: &mut u64,
    monotonic_ns: &mut u64,
    effect: JournalEffectV8,
    digest_byte: char,
) {
    append(
        records,
        stamp(1, BOOT_ONE, *boot_seq, *monotonic_ns),
        JournalEventV8::EffectIntent {
            effect,
            effect_manifest_sha256: digest(digest_byte),
        },
    );
    let intent_record_sha256 = records.last().expect("intent").record_sha256.clone();
    *boot_seq += 1;
    *monotonic_ns += 100;
    append(
        records,
        stamp(1, BOOT_ONE, *boot_seq, *monotonic_ns),
        JournalEventV8::EffectObserved {
            effect,
            intent_record_sha256,
            observation_sha256: digest(digest_byte),
        },
    );
    *boot_seq += 1;
    *monotonic_ns += 100;
}

fn same_boot_chain() -> Vec<JournalRecordV8> {
    let mut records = vec![opened()];
    let mut boot_seq = 2;
    let mut monotonic_ns = 600;
    append_effect(
        &mut records,
        &mut boot_seq,
        &mut monotonic_ns,
        JournalEffectV8::RunnerStop,
        '8',
    );
    append_effect(
        &mut records,
        &mut boot_seq,
        &mut monotonic_ns,
        JournalEffectV8::CandidateExecution,
        '9',
    );
    append(
        &mut records,
        stamp(1, BOOT_ONE, boot_seq, monotonic_ns),
        JournalEventV8::CandidateCompleted {
            candidate_result_sha256: digest('a'),
        },
    );
    boot_seq += 1;
    monotonic_ns += 100;
    for (effect, digest_byte) in [
        (JournalEffectV8::CandidateRelay, 'b'),
        (JournalEffectV8::RunnerRestore, 'c'),
        (JournalEffectV8::PostRestoreSnapshot, 'd'),
        (JournalEffectV8::BarrierRelease, 'd'),
    ] {
        append_effect(
            &mut records,
            &mut boot_seq,
            &mut monotonic_ns,
            effect,
            digest_byte,
        );
    }
    records
}

fn interrupted_chain() -> Vec<JournalRecordV8> {
    let mut records = vec![opened()];
    append(
        &mut records,
        stamp(1, BOOT_ONE, 2, 600),
        JournalEventV8::EffectIntent {
            effect: JournalEffectV8::RunnerStop,
            effect_manifest_sha256: digest('9'),
        },
    );
    records
}

#[test]
fn validates_contiguous_same_boot_chain() {
    let records = same_boot_chain();
    let assessment = validate_journal_v8(&records).expect("valid journal");

    assert_eq!(assessment.attempt_sha256(), attempt().sha256().unwrap());
    assert_eq!(
        assessment.tip_sha256(),
        records.last().expect("tip").record_sha256
    );
    assert_eq!(assessment.record_count(), 14);
    assert_eq!(assessment.boot_count(), 1);
    assert!(!assessment.reboot_observed());
    assert!(!assessment.qualification_abandoned());
    assert!(assessment.release_complete());
    assert!(!assessment.ready_for_release_authorization());
    assert!(assessment.qualification_may_pass());
}

#[test]
fn pre_release_prefix_authorizes_but_does_not_claim_release() {
    let mut records = same_boot_chain();
    records.truncate(records.len() - 2);
    let assessment = validate_journal_v8(&records).expect("valid pre-release journal");

    assert!(assessment.ready_for_release_authorization());
    assert!(!assessment.release_complete());
    assert!(!assessment.qualification_may_pass());
    assert_eq!(
        assessment.pre_release_tip_sha256(),
        Some(
            records
                .last()
                .expect("pre-release tip")
                .record_sha256
                .as_str()
        )
    );
    assert!(assessment.barrier_release_manifest_sha256().is_none());
    assert!(assessment.barrier_release_observation_sha256().is_none());
}

#[test]
fn rejects_attempt_splice_gap_reorder_and_broken_hash_link() {
    let records = same_boot_chain();

    let mut spliced = records.clone();
    spliced[1].attempt.attempt_nonce = digest('b');
    spliced[1].record_sha256 = spliced[1].computed_sha256().expect("valid spliced record");
    assert!(validate_journal_v8(&spliced).is_err());

    let mut gap = records.clone();
    gap[1].global_seq = 3;
    gap[1].record_sha256 = gap[1].computed_sha256().expect("valid gap record");
    assert!(validate_journal_v8(&gap).is_err());

    let mut reordered = records.clone();
    reordered.swap(1, 2);
    assert!(validate_journal_v8(&reordered).is_err());

    let mut broken = records;
    broken[2].previous_record_sha256 = Some(digest('c'));
    broken[2].record_sha256 = broken[2].computed_sha256().expect("valid broken record");
    assert!(validate_journal_v8(&broken).is_err());
}

#[test]
fn rejects_record_tamper_even_at_the_tip() {
    let mut records = same_boot_chain();
    records.last_mut().expect("tip").boot.monotonic_ns += 1;

    assert!(validate_journal_v8(&records).is_err());
}

#[test]
fn rejects_same_boot_sequence_or_monotonic_regression() {
    let records = same_boot_chain();

    let mut duplicate_seq = records.clone();
    duplicate_seq[2].boot.boot_seq = 2;
    duplicate_seq[2].record_sha256 = duplicate_seq[2]
        .computed_sha256()
        .expect("valid duplicate-seq record");
    assert!(validate_journal_v8(&duplicate_seq).is_err());

    let mut regressed = records;
    regressed[2].boot.monotonic_ns = 599;
    regressed[2].record_sha256 = regressed[2]
        .computed_sha256()
        .expect("valid regressed record");
    assert!(validate_journal_v8(&regressed).is_err());
}

#[test]
fn valid_boot_recovery_does_not_compare_cross_boot_monotonic_time() {
    let mut records = interrupted_chain();
    let previous = records.last().expect("prior tip");
    let event = JournalEventV8::BootRecovery {
        previous_boot_id: previous.boot.boot_id.clone(),
        previous_journal_tip_sha256: previous.record_sha256.clone(),
        recovery_observation_sha256: digest('d'),
    };
    append(&mut records, stamp(2, BOOT_TWO, 1, 1), event);

    let assessment = validate_journal_v8(&records).expect("valid recovery journal");
    assert_eq!(assessment.record_count(), 3);
    assert_eq!(assessment.boot_count(), 2);
    assert!(assessment.reboot_observed());
    assert!(!assessment.release_complete());
    assert!(!assessment.qualification_may_pass());
}

#[test]
fn abandonment_permanently_disallows_pass_without_reboot() {
    let mut records = interrupted_chain();
    append(
        &mut records,
        stamp(1, BOOT_ONE, 3, 700),
        JournalEventV8::QualificationAbandoned {
            abandonment_evidence_sha256: digest('d'),
        },
    );

    let assessment = validate_journal_v8(&records).expect("valid abandoned journal");
    assert!(assessment.qualification_abandoned());
    assert!(!assessment.qualification_may_pass());
}

#[test]
fn rejects_non_recovery_or_wrong_binding_at_new_boot() {
    let records = interrupted_chain();

    let mut non_recovery = records.clone();
    append(
        &mut non_recovery,
        stamp(2, BOOT_TWO, 1, 1),
        JournalEventV8::QualificationAbandoned {
            abandonment_evidence_sha256: digest('d'),
        },
    );
    assert!(validate_journal_v8(&non_recovery).is_err());

    let mut wrong_binding = records;
    let event = JournalEventV8::BootRecovery {
        previous_boot_id: BOOT_TWO.to_string(),
        previous_journal_tip_sha256: digest('e'),
        recovery_observation_sha256: digest('d'),
    };
    append(&mut wrong_binding, stamp(2, BOOT_TWO, 1, 1), event);
    assert!(validate_journal_v8(&wrong_binding).is_err());
}

#[test]
fn rejects_boot_identity_or_epoch_drift_without_exact_boundary() {
    let mut changed_id = same_boot_chain();
    changed_id[2].boot.boot_id = BOOT_TWO.to_string();
    changed_id[2].record_sha256 = changed_id[2]
        .computed_sha256()
        .expect("valid changed-id record");
    assert!(validate_journal_v8(&changed_id).is_err());

    let mut skipped_epoch = interrupted_chain();
    let previous = skipped_epoch.last().expect("prior tip");
    let event = JournalEventV8::BootRecovery {
        previous_boot_id: previous.boot.boot_id.clone(),
        previous_journal_tip_sha256: previous.record_sha256.clone(),
        recovery_observation_sha256: digest('d'),
    };
    append(&mut skipped_epoch, stamp(3, BOOT_TWO, 1, 1), event);
    assert!(validate_journal_v8(&skipped_epoch).is_err());
}

#[test]
fn rejects_invalid_start_and_unknown_record_fields() {
    let mut records = same_boot_chain();
    records.remove(0);
    assert!(validate_journal_v8(&records).is_err());

    let mut value = serde_json::to_value(opened()).expect("serialize record");
    value.as_object_mut().expect("record object").insert(
        "caller_asserted_complete".to_string(),
        serde_json::json!(true),
    );
    assert!(serde_json::from_value::<JournalRecordV8>(value).is_err());
}

#[test]
fn incomplete_or_unpaired_effects_never_form_release_authority() {
    let interrupted = interrupted_chain();
    let assessment = validate_journal_v8(&interrupted).expect("valid interrupted journal");
    assert!(!assessment.release_complete());
    assert!(!assessment.qualification_may_pass());

    let mut observed_without_intent = vec![opened()];
    let opened_sha256 = observed_without_intent[0].record_sha256.clone();
    append(
        &mut observed_without_intent,
        stamp(1, BOOT_ONE, 2, 600),
        JournalEventV8::EffectObserved {
            effect: JournalEffectV8::RunnerStop,
            intent_record_sha256: opened_sha256,
            observation_sha256: digest('a'),
        },
    );
    assert!(validate_journal_v8(&observed_without_intent).is_err());

    let mut wrong_intent = same_boot_chain();
    wrong_intent.truncate(3);
    if let JournalEventV8::EffectObserved {
        intent_record_sha256,
        ..
    } = &mut wrong_intent[2].event
    {
        *intent_record_sha256 = digest('f');
    }
    wrong_intent[2].record_sha256 = wrong_intent[2].computed_sha256().unwrap();
    assert!(validate_journal_v8(&wrong_intent).is_err());
}

#[test]
fn terminal_release_and_abandonment_reject_any_later_record() {
    let mut released = same_boot_chain();
    append(
        &mut released,
        stamp(1, BOOT_ONE, 15, 1_900),
        JournalEventV8::QualificationAbandoned {
            abandonment_evidence_sha256: digest('e'),
        },
    );
    assert!(validate_journal_v8(&released).is_err());

    let mut abandoned = interrupted_chain();
    append(
        &mut abandoned,
        stamp(1, BOOT_ONE, 3, 700),
        JournalEventV8::QualificationAbandoned {
            abandonment_evidence_sha256: digest('e'),
        },
    );
    append(
        &mut abandoned,
        stamp(1, BOOT_ONE, 4, 800),
        JournalEventV8::QualificationAbandoned {
            abandonment_evidence_sha256: digest('f'),
        },
    );
    assert!(validate_journal_v8(&abandoned).is_err());
}

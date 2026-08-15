use super::*;
use pretty_assertions::assert_eq;

fn digest(byte: char) -> String {
    byte.to_string().repeat(64)
}

fn prepared() -> DisposableLifecycleEventV2 {
    DisposableLifecycleEventV2::OperationPrepared {
        baseline_inventory_sha256: digest('a'),
        backing_identity_sha256: digest('b'),
        boot_session_uuid: "12345678-1234-1234-1234-123456789abc".to_string(),
        collector_policy_sha256: digest('e'),
        mountpoint_underlying_sha256: digest('c'),
    }
}

fn absence(
    operation_nonce: &str,
    boot_session_uuid: &str,
    before: u64,
) -> FreshAbsenceObservationV2 {
    FreshAbsenceObservationV2 {
        artifact_evidence_sha256: digest('d'),
        baseline_inventory_sha256: digest('a'),
        backing_identity_sha256: digest('b'),
        boot_session_uuid: boot_session_uuid.to_string(),
        collector_policy_sha256: digest('e'),
        collector_receipt_sha256: digest('f'),
        iomedia_evidence_sha256: digest('1'),
        monotonic_after_nanoseconds: before + 1,
        monotonic_before_nanoseconds: before,
        mount_evidence_sha256: digest('2'),
        mountpoint_underlying_sha256: digest('c'),
        no_matching_iomedia: true,
        no_nested_mounts: true,
        operation_nonce: operation_nonce.to_string(),
        operation_artifacts_absent: true,
        post_inventory_sha256: digest('a'),
        reconciliation_snapshot_sha256: None,
        restart_epoch_nonce: None,
    }
}

fn full_flow(operation_nonce: &str) -> Vec<DisposableLifecycleEventV2> {
    let observation = absence(operation_nonce, "12345678-1234-1234-1234-123456789abc", 10);
    let absence_digest = fresh_absence_sha256(&observation).expect("absence digest");
    vec![
        prepared(),
        DisposableLifecycleEventV2::CreateIssuedOrUncertain { effect_id: 1 },
        DisposableLifecycleEventV2::CreateObserved {
            effect_id: 1,
            image_identity_sha256: digest('d'),
        },
        DisposableLifecycleEventV2::AttachIssuedOrUncertain { effect_id: 2 },
        DisposableLifecycleEventV2::AttachObserved {
            effect_id: 2,
            topology_sha256: digest('e'),
        },
        DisposableLifecycleEventV2::MountIssuedOrUncertain { effect_id: 3 },
        DisposableLifecycleEventV2::MountObserved {
            effect_id: 3,
            mount_observation_sha256: digest('f'),
        },
        DisposableLifecycleEventV2::UnmountIssuedOrUncertain {
            effect_id: 4,
            purpose: EffectPurposeV2::ForwardFlow,
        },
        DisposableLifecycleEventV2::UnmountCallbackObserved {
            effect_id: 4,
            outcome: CallbackOutcomeV2::Succeeded,
        },
        DisposableLifecycleEventV2::UnmountObserved {
            effect_id: 4,
            mount_absence_sha256: digest('1'),
        },
        DisposableLifecycleEventV2::EjectIssuedOrUncertain {
            effect_id: 5,
            purpose: EffectPurposeV2::ForwardFlow,
        },
        DisposableLifecycleEventV2::EjectCallbackObserved {
            effect_id: 5,
            outcome: CallbackOutcomeV2::Succeeded,
        },
        DisposableLifecycleEventV2::EjectObserved {
            effect_id: 5,
            iomedia_absence_sha256: digest('2'),
        },
        DisposableLifecycleEventV2::FreshAbsenceObserved { observation },
        DisposableLifecycleEventV2::TerminalAbsenceProved {
            disposition: TerminalDispositionV2::Completed,
            fresh_absence_sha256: absence_digest,
        },
    ]
}

fn append(
    journal: &mut DisposableLifecycleJournalV2,
    records: &mut Vec<Vec<u8>>,
    event: DisposableLifecycleEventV2,
) -> Result<String, LifecycleErrorV2> {
    journal.append_with(event, |_, bytes| {
        records.push(bytes.to_vec());
        Ok(())
    })
}

fn restart(
    boot_session_uuid: &str,
    monotonic_nanoseconds: u64,
    restart_epoch_nonce: &str,
) -> DisposableLifecycleEventV2 {
    DisposableLifecycleEventV2::RestartReconciliationStarted {
        boot_session_uuid: boot_session_uuid.to_string(),
        collector_policy_sha256: digest('e'),
        monotonic_nanoseconds,
        restart_epoch_nonce: restart_epoch_nonce.to_string(),
    }
}

fn snapshot(
    operation_nonce: &str,
    boot_session_uuid: &str,
    restart_epoch_nonce: &str,
    before: u64,
    receipt_byte: char,
    match_result: ReconciliationMatchV2,
) -> ReconciliationSnapshotV2 {
    ReconciliationSnapshotV2 {
        backing_identity_sha256: digest('b'),
        boot_session_uuid: boot_session_uuid.to_string(),
        collector_policy_sha256: digest('e'),
        collector_receipt_sha256: digest(receipt_byte),
        iomedia_evidence_sha256: digest('6'),
        match_result,
        monotonic_after_nanoseconds: before + 1,
        monotonic_before_nanoseconds: before,
        mount_evidence_sha256: digest('7'),
        mountpoint_underlying_sha256: digest('c'),
        operation_nonce: operation_nonce.to_string(),
        restart_epoch_nonce: restart_epoch_nonce.to_string(),
    }
}

#[test]
fn restart_epochs_are_boot_scoped_and_snapshots_are_epoch_exact() {
    let operation_nonce = digest('3');
    let boot_a = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
    let boot_b = "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb";
    let epoch_a = digest('4');
    let epoch_b = digest('5');
    let mut initial = DisposableLifecycleJournalV2::new(&operation_nonce).expect("journal");
    let mut records = Vec::new();
    append(&mut initial, &mut records, prepared()).expect("prepare");

    let mut first =
        DisposableLifecycleJournalV2::resume_for_reconciliation(&records).expect("first restart");
    append(&mut first, &mut records, restart(boot_a, 100, &epoch_a)).expect("epoch A");
    let snapshot_a = snapshot(
        &operation_nonce,
        boot_a,
        &epoch_a,
        101,
        '8',
        ReconciliationMatchV2::Zero,
    );
    append(
        &mut first,
        &mut records,
        DisposableLifecycleEventV2::ReconciliationSnapshotObserved {
            snapshot: snapshot_a.clone(),
        },
    )
    .expect("snapshot A");
    assert!(
        append(
            &mut first,
            &mut records,
            DisposableLifecycleEventV2::ReconciliationSnapshotObserved {
                snapshot: snapshot(
                    &operation_nonce,
                    boot_a,
                    &epoch_a,
                    103,
                    '9',
                    ReconciliationMatchV2::Zero,
                ),
            },
        )
        .is_err(),
        "a restart epoch admits exactly one snapshot"
    );

    let mut second =
        DisposableLifecycleJournalV2::resume_for_reconciliation(&records).expect("second restart");
    assert!(append(&mut second, &mut records, restart(boot_a, 99, &epoch_b)).is_err());
    append(&mut second, &mut records, restart(boot_b, 1, &epoch_b))
        .expect("a new boot has a new monotonic domain");
    assert!(
        append(
            &mut second,
            &mut records,
            DisposableLifecycleEventV2::ReconciliationSnapshotObserved {
                snapshot: snapshot_a,
            },
        )
        .is_err(),
        "an old boot/epoch snapshot cannot be spliced"
    );
    append(
        &mut second,
        &mut records,
        DisposableLifecycleEventV2::ReconciliationSnapshotObserved {
            snapshot: snapshot(
                &operation_nonce,
                boot_b,
                &epoch_b,
                2,
                'a',
                ReconciliationMatchV2::Zero,
            ),
        },
    )
    .expect("fresh snapshot in the new boot");
}

#[test]
fn fresh_absence_binds_the_current_epoch_and_exact_snapshot() {
    let operation_nonce = digest('3');
    let boot = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
    let epoch = digest('4');
    let mut initial = DisposableLifecycleJournalV2::new(&operation_nonce).expect("journal");
    let mut records = Vec::new();
    append(&mut initial, &mut records, prepared()).expect("prepare");
    let mut resumed =
        DisposableLifecycleJournalV2::resume_for_reconciliation(&records).expect("restart");
    append(&mut resumed, &mut records, restart(boot, 100, &epoch)).expect("epoch");
    let observed = snapshot(
        &operation_nonce,
        boot,
        &epoch,
        101,
        '5',
        ReconciliationMatchV2::Zero,
    );
    let observed_sha = reconciliation_snapshot_sha256(&observed).expect("snapshot digest");
    append(
        &mut resumed,
        &mut records,
        DisposableLifecycleEventV2::ReconciliationSnapshotObserved { snapshot: observed },
    )
    .expect("snapshot");
    let mut stale = absence(&operation_nonce, boot, 103);
    stale.restart_epoch_nonce = Some(epoch.clone());
    stale.reconciliation_snapshot_sha256 = Some(digest('9'));
    assert!(
        append(
            &mut resumed,
            &mut records,
            DisposableLifecycleEventV2::FreshAbsenceObserved { observation: stale },
        )
        .is_err()
    );
    let mut exact = absence(&operation_nonce, boot, 103);
    exact.restart_epoch_nonce = Some(epoch);
    exact.reconciliation_snapshot_sha256 = Some(observed_sha);
    append(
        &mut resumed,
        &mut records,
        DisposableLifecycleEventV2::FreshAbsenceObserved { observation: exact },
    )
    .expect("exact epoch absence");
}

#[test]
fn restart_unique_cleanup_requires_one_success_callback_and_ambiguous_blocks() {
    let operation_nonce = digest('3');
    let boot = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
    let epoch_a = digest('4');
    let epoch_b = digest('5');
    let mut initial = DisposableLifecycleJournalV2::new(&operation_nonce).expect("journal");
    let mut records = Vec::new();
    append(&mut initial, &mut records, prepared()).expect("prepare");
    let mut first =
        DisposableLifecycleJournalV2::resume_for_reconciliation(&records).expect("restart");
    append(&mut first, &mut records, restart(boot, 100, &epoch_a)).expect("epoch");
    append(
        &mut first,
        &mut records,
        DisposableLifecycleEventV2::ReconciliationSnapshotObserved {
            snapshot: snapshot(
                &operation_nonce,
                boot,
                &epoch_a,
                101,
                '6',
                ReconciliationMatchV2::Unique { mounted: true },
            ),
        },
    )
    .expect("mounted snapshot");
    append(
        &mut first,
        &mut records,
        DisposableLifecycleEventV2::UnmountIssuedOrUncertain {
            effect_id: 1,
            purpose: EffectPurposeV2::Reconciliation,
        },
    )
    .expect("unmount intent");
    append(
        &mut first,
        &mut records,
        DisposableLifecycleEventV2::UnmountCallbackObserved {
            effect_id: 1,
            outcome: CallbackOutcomeV2::ChannelLost,
        },
    )
    .expect("lost callback is durable uncertainty");
    assert!(
        append(
            &mut first,
            &mut records,
            DisposableLifecycleEventV2::UnmountCallbackObserved {
                effect_id: 1,
                outcome: CallbackOutcomeV2::Succeeded,
            },
        )
        .is_err(),
        "a callback cannot be replaced or duplicated"
    );
    assert!(
        append(
            &mut first,
            &mut records,
            DisposableLifecycleEventV2::UnmountObserved {
                effect_id: 1,
                mount_absence_sha256: digest('7'),
            },
        )
        .is_err(),
        "lost callback is not success"
    );

    let mut second =
        DisposableLifecycleJournalV2::resume_for_reconciliation(&records).expect("fresh process");
    append(&mut second, &mut records, restart(boot, 200, &epoch_b)).expect("new epoch");
    append(
        &mut second,
        &mut records,
        DisposableLifecycleEventV2::ReconciliationSnapshotObserved {
            snapshot: snapshot(
                &operation_nonce,
                boot,
                &epoch_b,
                201,
                '8',
                ReconciliationMatchV2::Ambiguous {
                    matching_objects: 2,
                },
            ),
        },
    )
    .expect("ambiguous snapshot");
    assert!(
        append(
            &mut second,
            &mut records,
            DisposableLifecycleEventV2::EjectIssuedOrUncertain {
                effect_id: 2,
                purpose: EffectPurposeV2::Reconciliation,
            },
        )
        .is_err(),
        "ambiguous identity grants no cleanup target"
    );
    let mut impossible_absence = absence(&operation_nonce, boot, 203);
    impossible_absence.restart_epoch_nonce = Some(epoch_b);
    impossible_absence.reconciliation_snapshot_sha256 = Some(digest('9'));
    assert!(
        append(
            &mut second,
            &mut records,
            DisposableLifecycleEventV2::FreshAbsenceObserved {
                observation: impossible_absence,
            },
        )
        .is_err()
    );
}

#[test]
fn restart_cleanup_cannot_use_a_pre_snapshot_stale_target() {
    let operation_nonce = digest('9');
    let boot = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
    let epoch = digest('a');
    let events = full_flow(&operation_nonce);
    for (cutpoint, cleanup) in [
        (
            6,
            DisposableLifecycleEventV2::UnmountIssuedOrUncertain {
                effect_id: 4,
                purpose: EffectPurposeV2::Reconciliation,
            },
        ),
        (
            9,
            DisposableLifecycleEventV2::EjectIssuedOrUncertain {
                effect_id: 5,
                purpose: EffectPurposeV2::Reconciliation,
            },
        ),
    ] {
        let mut initial = DisposableLifecycleJournalV2::new(&operation_nonce).expect("journal");
        let mut records = Vec::new();
        for event in events.iter().take(cutpoint + 1) {
            append(&mut initial, &mut records, event.clone()).expect("observed prefix");
        }
        let mut resumed =
            DisposableLifecycleJournalV2::resume_for_reconciliation(&records).expect("restart");
        append(&mut resumed, &mut records, restart(boot, 100, &epoch)).expect("epoch");
        assert!(
            append(&mut resumed, &mut records, cleanup).is_err(),
            "reconciliation cleanup used the stale pre-snapshot target at cutpoint {cutpoint}"
        );
        assert!(
            inspect_lifecycle_v2(&records)
                .expect("inspection")
                .blocks_new_operations
        );
    }
}

#[test]
fn every_issued_cutpoint_restarts_abort_only_and_requires_fresh_absence() {
    let operation_nonce = digest('3');
    let events = full_flow(&operation_nonce);
    let cutpoints = events
        .iter()
        .enumerate()
        .filter_map(|(index, event)| {
            matches!(
                event,
                DisposableLifecycleEventV2::CreateIssuedOrUncertain { .. }
                    | DisposableLifecycleEventV2::AttachIssuedOrUncertain { .. }
                    | DisposableLifecycleEventV2::MountIssuedOrUncertain { .. }
                    | DisposableLifecycleEventV2::UnmountIssuedOrUncertain { .. }
                    | DisposableLifecycleEventV2::EjectIssuedOrUncertain { .. }
            )
            .then_some(index)
        })
        .collect::<Vec<_>>();
    assert_eq!(cutpoints.len(), 5);
    for cutpoint in cutpoints {
        let mut journal = DisposableLifecycleJournalV2::new(&operation_nonce).expect("journal");
        let mut records = Vec::new();
        for event in events.iter().take(cutpoint + 1) {
            append(&mut journal, &mut records, event.clone()).expect("append prefix");
        }
        let inspection = inspect_lifecycle_v2(&records).expect("inspect cutpoint");
        assert!(inspection.blocks_new_operations);
        assert!(!inspection.authority.any());
        assert!(!inspection.restart_forward_flow_authority);

        let mut resumed = DisposableLifecycleJournalV2::resume_for_reconciliation(&records)
            .expect("resume for reconciliation");
        let forward_event = events[cutpoint + 1].clone();
        assert!(resumed.append_with(forward_event, |_, _| Ok(())).is_err());
        resumed
            .append_with(
                DisposableLifecycleEventV2::RestartReconciliationStarted {
                    boot_session_uuid: "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".to_string(),
                    collector_policy_sha256: digest('e'),
                    monotonic_nanoseconds: 100,
                    restart_epoch_nonce: digest('4'),
                },
                |_, _| Ok(()),
            )
            .expect("restart epoch");
        let snapshot = ReconciliationSnapshotV2 {
            backing_identity_sha256: digest('b'),
            boot_session_uuid: "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".to_string(),
            collector_policy_sha256: digest('e'),
            collector_receipt_sha256: digest('5'),
            iomedia_evidence_sha256: digest('6'),
            match_result: ReconciliationMatchV2::Zero,
            monotonic_after_nanoseconds: 102,
            monotonic_before_nanoseconds: 101,
            mount_evidence_sha256: digest('7'),
            mountpoint_underlying_sha256: digest('c'),
            operation_nonce: operation_nonce.clone(),
            restart_epoch_nonce: digest('4'),
        };
        let snapshot_digest = reconciliation_snapshot_sha256(&snapshot).expect("snapshot digest");
        resumed
            .append_with(
                DisposableLifecycleEventV2::ReconciliationSnapshotObserved { snapshot },
                |_, _| Ok(()),
            )
            .expect("zero-match snapshot");
        let mut observation = absence(
            &operation_nonce,
            "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
            103,
        );
        observation.restart_epoch_nonce = Some(digest('4'));
        observation.reconciliation_snapshot_sha256 = Some(snapshot_digest);
        let absence_digest = fresh_absence_sha256(&observation).expect("absence digest");
        resumed
            .append_with(
                DisposableLifecycleEventV2::FreshAbsenceObserved { observation },
                |_, _| Ok(()),
            )
            .expect("fresh absence");
        resumed
            .append_with(
                DisposableLifecycleEventV2::TerminalAbsenceProved {
                    disposition: TerminalDispositionV2::Aborted,
                    fresh_absence_sha256: absence_digest,
                },
                |_, _| Ok(()),
            )
            .expect("abort terminal");
        assert_eq!(
            resumed.disposition(),
            LifecycleDispositionV2::TerminalAborted
        );
    }
}

#[test]
fn pending_or_incomplete_forward_flow_cannot_cross_an_absence_terminal() {
    let operation_nonce = digest('b');
    let events = full_flow(&operation_nonce);
    for cutpoint in events.iter().enumerate().filter_map(|(index, event)| {
        matches!(
            event,
            DisposableLifecycleEventV2::CreateIssuedOrUncertain { .. }
                | DisposableLifecycleEventV2::AttachIssuedOrUncertain { .. }
                | DisposableLifecycleEventV2::MountIssuedOrUncertain { .. }
                | DisposableLifecycleEventV2::UnmountIssuedOrUncertain { .. }
                | DisposableLifecycleEventV2::EjectIssuedOrUncertain { .. }
        )
        .then_some(index)
    }) {
        let mut journal = DisposableLifecycleJournalV2::new(&operation_nonce).expect("journal");
        let mut records = Vec::new();
        for event in events.iter().take(cutpoint + 1) {
            append(&mut journal, &mut records, event.clone()).expect("append prefix");
        }
        assert!(
            append(
                &mut journal,
                &mut records,
                DisposableLifecycleEventV2::FreshAbsenceObserved {
                    observation: absence(
                        &operation_nonce,
                        "12345678-1234-1234-1234-123456789abc",
                        100,
                    ),
                },
            )
            .is_err(),
            "issued-or-uncertain effect at cutpoint {cutpoint} crossed fresh absence"
        );
        let inspection = inspect_lifecycle_v2(&records).expect("blocked prefix inspection");
        assert!(inspection.blocks_new_operations);
        assert!(!inspection.authority.any());
    }

    let mut prepared_only =
        DisposableLifecycleJournalV2::new(&operation_nonce).expect("prepared journal");
    let mut prepared_records = Vec::new();
    append(&mut prepared_only, &mut prepared_records, prepared()).expect("prepared");
    assert!(
        append(
            &mut prepared_only,
            &mut prepared_records,
            DisposableLifecycleEventV2::FreshAbsenceObserved {
                observation: absence(
                    &operation_nonce,
                    "12345678-1234-1234-1234-123456789abc",
                    200,
                ),
            },
        )
        .is_err(),
        "a fresh forward process cannot skip the observed effect lifecycle"
    );
}

#[test]
fn forward_flow_cannot_relabel_completion_as_restart_abort() {
    let operation_nonce = digest('c');
    let mut journal = DisposableLifecycleJournalV2::new(&operation_nonce).expect("journal");
    let mut records = Vec::new();
    let events = full_flow(&operation_nonce);
    for event in events.iter().take(events.len() - 1) {
        append(&mut journal, &mut records, event.clone()).expect("forward flow prefix");
    }
    let absence_digest = match &events[events.len() - 2] {
        DisposableLifecycleEventV2::FreshAbsenceObserved { observation } => {
            fresh_absence_sha256(observation).expect("absence digest")
        }
        _ => panic!("full flow must end with absence then terminal"),
    };
    assert!(
        append(
            &mut journal,
            &mut records,
            DisposableLifecycleEventV2::TerminalAbsenceProved {
                disposition: TerminalDispositionV2::Aborted,
                fresh_absence_sha256: absence_digest,
            },
        )
        .is_err(),
        "forward completion cannot be relabeled as a restart abort"
    );
    let inspection = inspect_lifecycle_v2(&records).expect("unterminated forward inspection");
    assert!(inspection.blocks_new_operations);
    assert!(!inspection.authority.any());
}

#[test]
fn callbacks_manual_intervention_and_quarantine_are_never_terminal() {
    let operation_nonce = digest('4');
    let mut journal = DisposableLifecycleJournalV2::new(&operation_nonce).expect("journal");
    let mut records = Vec::new();
    for event in full_flow(&operation_nonce).into_iter().take(9) {
        append(&mut journal, &mut records, event).expect("append through callback");
    }
    assert_eq!(
        inspect_lifecycle_v2(&records)
            .expect("callback inspection")
            .disposition,
        LifecycleDispositionV2::Outstanding
    );
    append(
        &mut journal,
        &mut records,
        DisposableLifecycleEventV2::ManualIntervention {
            reason_sha256: digest('5'),
        },
    )
    .expect("manual intervention");
    append(
        &mut journal,
        &mut records,
        DisposableLifecycleEventV2::Quarantined {
            reason_sha256: digest('6'),
        },
    )
    .expect("quarantine");
    let inspection = inspect_lifecycle_v2(&records).expect("quarantine inspection");
    assert_eq!(inspection.disposition, LifecycleDispositionV2::Quarantined);
    assert!(inspection.blocks_new_operations);
    assert!(!inspection.authority.any());
}

#[test]
fn append_failure_poison_requires_descriptor_replay_and_never_retries_forward() {
    let mut journal = DisposableLifecycleJournalV2::new(&digest('7')).expect("journal");
    let mut records = Vec::new();
    append(&mut journal, &mut records, prepared()).expect("prepared");
    let failure = journal.append_with(
        DisposableLifecycleEventV2::CreateIssuedOrUncertain { effect_id: 1 },
        |_, _| Err(std::io::Error::other("injected append failure")),
    );
    assert!(matches!(failure, Err(LifecycleErrorV2::Persistence(_))));
    assert!(journal.persistence_uncertain());
    assert_eq!(journal.last_effect_id(), 0);
    assert_eq!(journal.disposition(), LifecycleDispositionV2::Outstanding);
    assert!(
        append(
            &mut journal,
            &mut records,
            DisposableLifecycleEventV2::CreateIssuedOrUncertain { effect_id: 1 },
        )
        .is_err(),
        "issued-or-uncertain persistence must poison the in-memory journal"
    );
    let mut resumed =
        DisposableLifecycleJournalV2::resume_for_reconciliation(&records).expect("replay");
    assert!(
        append(
            &mut resumed,
            &mut records,
            DisposableLifecycleEventV2::CreateIssuedOrUncertain { effect_id: 1 },
        )
        .is_err(),
        "descriptor replay may reconcile or abort but never retry forward"
    );
    let inspection = inspect_lifecycle_v2(&records).expect("inspection");
    assert_eq!(inspection.records, 1);
    assert!(inspection.blocks_new_operations);
    assert!(!inspection.authority.any());
}

#[test]
fn persistence_panic_is_caught_and_poisons_the_journal() {
    let mut journal = DisposableLifecycleJournalV2::new(&digest('6')).expect("journal");
    let failure = journal.append_with(prepared(), |_, _| {
        panic!("injected persistence panic");
    });
    assert!(matches!(failure, Err(LifecycleErrorV2::Persistence(_))));
    assert!(journal.persistence_uncertain());
    assert_eq!(journal.last_effect_id(), 0);
    assert_eq!(journal.disposition(), LifecycleDispositionV2::Outstanding);
}

#[test]
fn sequence_overflow_is_rejected_before_persistence_is_called() {
    let mut journal = DisposableLifecycleJournalV2::new(&digest('5')).expect("journal");
    journal.sequence = u32::MAX;
    let persist_called = std::cell::Cell::new(false);
    let failure = journal.append_with(prepared(), |_, _| {
        persist_called.set(true);
        Ok(())
    });
    assert!(matches!(failure, Err(LifecycleErrorV2::Invalid(_))));
    assert!(!persist_called.get());
    assert!(!journal.persistence_uncertain());
}

#[test]
fn persisted_absence_without_terminal_is_superseded_by_a_fresh_restart_epoch() {
    let operation_nonce = digest('e');
    let boot = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
    let epoch = digest('f');
    let events = full_flow(&operation_nonce);
    let mut initial = DisposableLifecycleJournalV2::new(&operation_nonce).expect("journal");
    let mut records = Vec::new();
    for event in events.iter().take(events.len() - 1) {
        append(&mut initial, &mut records, event.clone()).expect("through fresh absence");
    }
    assert!(
        inspect_lifecycle_v2(&records)
            .expect("inspection")
            .blocks_new_operations
    );

    let mut resumed =
        DisposableLifecycleJournalV2::resume_for_reconciliation(&records).expect("restart");
    append(&mut resumed, &mut records, restart(boot, 100, &epoch)).expect("new epoch");
    let observed = snapshot(
        &operation_nonce,
        boot,
        &epoch,
        101,
        '1',
        ReconciliationMatchV2::Zero,
    );
    let observed_sha = reconciliation_snapshot_sha256(&observed).expect("snapshot digest");
    append(
        &mut resumed,
        &mut records,
        DisposableLifecycleEventV2::ReconciliationSnapshotObserved { snapshot: observed },
    )
    .expect("fresh zero snapshot");
    let mut observation = absence(&operation_nonce, boot, 103);
    observation.restart_epoch_nonce = Some(epoch);
    observation.reconciliation_snapshot_sha256 = Some(observed_sha);
    let absence_sha = fresh_absence_sha256(&observation).expect("absence digest");
    append(
        &mut resumed,
        &mut records,
        DisposableLifecycleEventV2::FreshAbsenceObserved { observation },
    )
    .expect("fresh restart absence");
    append(
        &mut resumed,
        &mut records,
        DisposableLifecycleEventV2::TerminalAbsenceProved {
            disposition: TerminalDispositionV2::Aborted,
            fresh_absence_sha256: absence_sha,
        },
    )
    .expect("restart abort terminal");
    assert_eq!(
        resumed.disposition(),
        LifecycleDispositionV2::TerminalAborted
    );
}

#[test]
fn effect_ids_are_monotonic_unique_and_terminal_is_absence_only() {
    let mut journal = DisposableLifecycleJournalV2::new(&digest('8')).expect("journal");
    let mut records = Vec::new();
    append(&mut journal, &mut records, prepared()).expect("prepared");
    assert!(
        append(
            &mut journal,
            &mut records,
            DisposableLifecycleEventV2::CreateIssuedOrUncertain { effect_id: 2 },
        )
        .is_err()
    );
    assert!(
        append(
            &mut journal,
            &mut records,
            DisposableLifecycleEventV2::TerminalAbsenceProved {
                disposition: TerminalDispositionV2::Aborted,
                fresh_absence_sha256: digest('9'),
            },
        )
        .is_err()
    );
    let bad_absence = FreshAbsenceObservationV2 {
        no_matching_iomedia: false,
        ..absence(&digest('8'), "12345678-1234-1234-1234-123456789abc", 10)
    };
    assert!(
        append(
            &mut journal,
            &mut records,
            DisposableLifecycleEventV2::FreshAbsenceObserved {
                observation: bad_absence,
            },
        )
        .is_err()
    );
}

#[test]
fn prepared_boot_uuid_must_be_canonical_lowercase_and_non_nil() {
    for invalid_uuid in [
        "00000000-0000-0000-0000-000000000000",
        "AAAAAAAA-AAAA-4AAA-8AAA-AAAAAAAAAAAA",
    ] {
        let mut event = prepared();
        let DisposableLifecycleEventV2::OperationPrepared {
            boot_session_uuid, ..
        } = &mut event
        else {
            unreachable!("prepared helper must return OperationPrepared");
        };
        *boot_session_uuid = invalid_uuid.to_string();
        let mut journal = DisposableLifecycleJournalV2::new(&digest('a')).expect("journal");
        let mut records = Vec::new();
        assert!(append(&mut journal, &mut records, event).is_err());
        assert!(records.is_empty());
    }
}

#[test]
fn forward_absence_must_match_the_prepared_collector_policy() {
    let operation_nonce = digest('b');
    let mut journal = DisposableLifecycleJournalV2::new(&operation_nonce).expect("journal");
    let mut records = Vec::new();
    for event in full_flow(&operation_nonce).into_iter().take(13) {
        append(&mut journal, &mut records, event).expect("append through observed eject");
    }
    let mut observation = absence(&operation_nonce, "12345678-1234-1234-1234-123456789abc", 10);
    observation.collector_policy_sha256 = digest('9');
    assert!(
        append(
            &mut journal,
            &mut records,
            DisposableLifecycleEventV2::FreshAbsenceObserved { observation },
        )
        .is_err()
    );
    assert_eq!(records.len(), 13);
}

fn historical_record(disposition: &str, kind: &str) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "authority_granted": false,
        "boot_session_uuid": "12345678-1234-1234-1234-123456789abc",
        "challenge_sha256": digest('a'),
        "disposition": disposition,
        "epoch_receipt_sha256": digest('b'),
        "event": { "kind": kind, "value": {} },
        "operation_nonce": digest('c'),
        "previous_record_sha256": null,
        "schema": HISTORICAL_RECORD_SCHEMA_V1,
        "sequence": 1
    }))
    .expect("historical JSON")
}

#[test]
fn historical_v1_completed_label_cannot_bypass_frozen_replay() {
    assert!(
        dispatch_lifecycle_records(&[historical_record("reconciled", "terminal_reconciled")])
            .is_err()
    );
}

#[test]
fn full_flow_terminal_still_grants_zero_authority() {
    let operation_nonce = digest('d');
    let mut journal = DisposableLifecycleJournalV2::new(&operation_nonce).expect("journal");
    let mut records = Vec::new();
    for event in full_flow(&operation_nonce) {
        append(&mut journal, &mut records, event).expect("full flow append");
    }
    let inspection = inspect_lifecycle_v2(&records).expect("full inspection");
    assert_eq!(
        inspection.disposition,
        LifecycleDispositionV2::TerminalCompleted
    );
    assert!(!inspection.blocks_new_operations);
    assert!(!inspection.authority.any());
    assert!(!inspection.restart_forward_flow_authority);
}

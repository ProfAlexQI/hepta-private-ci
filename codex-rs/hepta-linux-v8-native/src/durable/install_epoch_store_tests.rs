use super::*;

fn digest(character: char) -> String {
    character.to_string().repeat(64)
}

fn structural_record(
    event_sequence: u64,
    previous_event_sha256: String,
    kind: InstallEpochStoreEventKindV1,
    phase_predecessor_revision: u64,
    phase_predecessor_state_sha256: String,
    payload: &[u8],
) -> InstallEpochStoreRecordV1 {
    InstallEpochStoreRecordV1::new(
        digest('1'),
        digest('2'),
        1,
        kind,
        event_sequence,
        digest('3'),
        digest('4'),
        digest('5'),
        digest('6'),
        payload.to_vec(),
        digest('7'),
        phase_predecessor_revision,
        phase_predecessor_state_sha256,
        phase_predecessor_revision + 1,
        digest(char::from_digit((phase_predecessor_revision as u32 % 6) + 8, 16).unwrap()),
        digest('a'),
        previous_event_sha256,
        digest('b'),
        digest('c'),
        digest('d'),
    )
    .unwrap()
}

#[test]
fn canonical_record_round_trips_and_tamper_is_rejected() {
    let record = structural_record(
        1,
        ZERO_SHA256.to_string(),
        InstallEpochStoreEventKindV1::Intent,
        0,
        ZERO_SHA256.to_string(),
        b"intent",
    );
    let encoded = record.canonical_bytes().unwrap();
    assert_eq!(
        InstallEpochStoreRecordV1::decode_exact(&encoded).unwrap(),
        record
    );
    assert_eq!(record.record_sha256().unwrap().len(), 64);

    let mut tampered = encoded;
    *tampered.last_mut().unwrap() ^= 1;
    let decoded = InstallEpochStoreRecordV1::decode_exact(&tampered).unwrap();
    assert_ne!(decoded, record);

    let mut trailing = record.canonical_bytes().unwrap();
    trailing.push(0);
    assert!(InstallEpochStoreRecordV1::decode_exact(&trailing).is_err());
}

#[test]
fn record_rejects_gaps_noncanonical_genesis_and_empty_payload() {
    assert!(
        InstallEpochStoreRecordV1::new(
            digest('1'),
            digest('2'),
            1,
            InstallEpochStoreEventKindV1::Intent,
            1,
            digest('3'),
            digest('4'),
            digest('5'),
            digest('6'),
            Vec::new(),
            digest('7'),
            0,
            ZERO_SHA256.to_string(),
            2,
            digest('8'),
            digest('a'),
            ZERO_SHA256.to_string(),
            digest('b'),
            digest('c'),
            digest('d'),
        )
        .is_err()
    );
    assert!(
        InstallEpochStoreRecordV1::new(
            digest('1'),
            digest('2'),
            1,
            InstallEpochStoreEventKindV1::Intent,
            1,
            digest('3'),
            digest('4'),
            digest('5'),
            digest('6'),
            b"x".to_vec(),
            digest('7'),
            0,
            digest('9'),
            1,
            digest('8'),
            digest('a'),
            ZERO_SHA256.to_string(),
            digest('b'),
            digest('c'),
            digest('d'),
        )
        .is_err()
    );
}

#[test]
fn prospective_store_budget_fails_before_publication_or_overflow() {
    assert_eq!(
        checked_store_total_bytes_v1(MAX_STORE_BYTES_V1 - 1, 1).unwrap(),
        MAX_STORE_BYTES_V1
    );
    assert!(checked_store_total_bytes_v1(MAX_STORE_BYTES_V1, 1).is_err());
    assert!(checked_store_total_bytes_v1(u64::MAX, 1).is_err());
}

#[test]
fn cross_job_transition_stays_closed_until_the_verified_bridge_exists() {
    let previous = InstallEpochStoreRecordV1::new(
        digest('1'),
        digest('2'),
        1,
        InstallEpochStoreEventKindV1::Final,
        1,
        digest('3'),
        digest('4'),
        digest('5'),
        digest('6'),
        b"final".to_vec(),
        digest('7'),
        8,
        digest('8'),
        9,
        digest('9'),
        digest('a'),
        ZERO_SHA256.to_string(),
        digest('b'),
        digest('c'),
        digest('d'),
    )
    .unwrap();
    let next = InstallEpochStoreRecordV1::new(
        digest('e'),
        digest('f'),
        2,
        InstallEpochStoreEventKindV1::Intent,
        2,
        digest('e'),
        digest('4'),
        digest('5'),
        digest('6'),
        b"next intent".to_vec(),
        digest('7'),
        0,
        ZERO_SHA256.to_string(),
        1,
        digest('1'),
        digest('2'),
        previous.record_sha256().unwrap(),
        digest('b'),
        digest('c'),
        digest('d'),
    )
    .unwrap();
    assert!(validate_record_transition_v1(&previous, &next).is_err());
}

#[test]
fn operation_binding_cannot_change_within_one_job() {
    let previous = structural_record(
        1,
        ZERO_SHA256.to_string(),
        InstallEpochStoreEventKindV1::Intent,
        0,
        ZERO_SHA256.to_string(),
        b"intent",
    );
    let mut next = structural_record(
        2,
        previous.record_sha256().unwrap(),
        InstallEpochStoreEventKindV1::CasOutbox,
        1,
        previous.phase_successor_state_sha256.to_string(),
        b"outbox",
    );
    next.operation_binding_sha256 = digest('e');
    assert!(validate_record_transition_v1(&previous, &next).is_err());
}

#[test]
fn phase_aligned_roster_allows_retry_outboxes_but_not_reserved_edges() {
    assert!(
        InstallEpochStoreEventKindV1::CurrentTipOutbox
            .permits_successor(InstallEpochStoreEventKindV1::CurrentTipOutbox)
    );
    assert!(
        InstallEpochStoreEventKindV1::CurrentTipOutbox
            .permits_successor(InstallEpochStoreEventKindV1::Final)
    );
    for reserved in [
        InstallEpochStoreEventKindV1::CasFence,
        InstallEpochStoreEventKindV1::CurrentTipFence,
        InstallEpochStoreEventKindV1::CurrentTipReceipt,
        InstallEpochStoreEventKindV1::QueryClosed,
    ] {
        assert!(!InstallEpochStoreEventKindV1::Intent.permits_successor(reserved));
        assert!(!reserved.permits_successor(InstallEpochStoreEventKindV1::Quarantine));
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    use super::*;
    use crate::ATTEMPTS_DIRECTORY_V8;
    use crate::INSTALL_EPOCH_DIRECTORY_V8;
    use crate::JOURNAL_DIRECTORY_V8;
    use crate::NONCE_CLAIMS_DIRECTORY_V8;
    use crate::PROCESS_FD_LIFETIME_TEST_MUTEX;
    use crate::QUARANTINE_DIRECTORY_V8;
    use crate::durable::trusted_state_root::open_test_trusted_state_root_v8;

    fn serialize_process_fd_lifetime() -> std::sync::MutexGuard<'static, ()> {
        PROCESS_FD_LIFETIME_TEST_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    struct TestRoot {
        path: std::path::PathBuf,
    }

    impl TestRoot {
        fn new(label: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "hepta-v8-install-store-{label}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            fs::create_dir(&path).unwrap();
            fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
            for name in [
                ATTEMPTS_DIRECTORY_V8,
                INSTALL_EPOCH_DIRECTORY_V8,
                JOURNAL_DIRECTORY_V8,
                NONCE_CLAIMS_DIRECTORY_V8,
                QUARANTINE_DIRECTORY_V8,
            ] {
                fs::create_dir(path.join(name)).unwrap();
                fs::set_permissions(path.join(name), fs::Permissions::from_mode(0o700)).unwrap();
            }
            Self { path }
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn record_for_root(
        root: &TrustedStateRootV8,
        event_sequence: u64,
        previous_event_sha256: String,
        kind: InstallEpochStoreEventKindV1,
        phase_predecessor_revision: u64,
        phase_predecessor_state_sha256: String,
        payload: &[u8],
    ) -> InstallEpochStoreRecordV1 {
        InstallEpochStoreRecordV1::new(
            digest('1'),
            digest('2'),
            1,
            kind,
            event_sequence,
            digest('3'),
            root.layout_manifest_sha256().to_string(),
            root.machine_id_sha256().to_string(),
            digest('6'),
            payload.to_vec(),
            digest('7'),
            phase_predecessor_revision,
            phase_predecessor_state_sha256,
            phase_predecessor_revision + 1,
            digest(char::from_digit((phase_predecessor_revision as u32 % 6) + 8, 16).unwrap()),
            digest('a'),
            previous_event_sha256,
            digest('b'),
            root.profile_sha256().to_string(),
            digest('d'),
        )
        .unwrap()
    }

    #[test]
    fn fresh_chain_exact_replay_and_conflict_are_distinct() {
        let state = TestRoot::new("chain");
        let mut root = open_test_trusted_state_root_v8(&state.path).unwrap();
        let empty = scan_install_epoch_store_v1(&mut root).unwrap();
        assert_eq!(empty.event_count(), 0);
        assert_eq!(empty.tip_sha256(), ZERO_SHA256);
        assert!(matches!(
            empty.recovery_view(),
            InstallEpochStoreRecoveryViewV1::Empty
        ));

        let intent = record_for_root(
            &root,
            1,
            ZERO_SHA256.to_string(),
            InstallEpochStoreEventKindV1::Intent,
            0,
            ZERO_SHA256.to_string(),
            b"intent",
        );
        let first = append_install_epoch_store_event_v1(&mut root, &intent, &digest('e')).unwrap();
        assert!(matches!(first, InstallEpochStoreAppendOutcomeV1::Fresh(_)));
        let replay = append_install_epoch_store_event_v1(&mut root, &intent, &digest('f')).unwrap();
        assert!(matches!(
            replay,
            InstallEpochStoreAppendOutcomeV1::ExactExisting(_)
        ));

        let outbox = record_for_root(
            &root,
            2,
            intent.record_sha256().unwrap(),
            InstallEpochStoreEventKindV1::CasOutbox,
            1,
            intent.phase_successor_state_sha256.to_string(),
            b"cas-outbox",
        );
        let second = append_install_epoch_store_event_v1(&mut root, &outbox, &digest('1')).unwrap();
        assert!(matches!(second, InstallEpochStoreAppendOutcomeV1::Fresh(_)));

        let conflicting = record_for_root(
            &root,
            2,
            intent.record_sha256().unwrap(),
            InstallEpochStoreEventKindV1::CasOutbox,
            1,
            intent.phase_successor_state_sha256.to_string(),
            b"different",
        );
        assert!(matches!(
            append_install_epoch_store_event_v1(&mut root, &conflicting, &digest('2')),
            Err(InstallEpochStoreErrorV1::ConflictHold(_))
        ));
    }

    #[test]
    fn reopen_recovers_one_byte_exact_redacted_terminal_tip() {
        let _process_guard = serialize_process_fd_lifetime();
        let state = TestRoot::new("recovery-view");
        let mut root = open_test_trusted_state_root_v8(&state.path).unwrap();
        let intent = record_for_root(
            &root,
            1,
            ZERO_SHA256.to_string(),
            InstallEpochStoreEventKindV1::Intent,
            0,
            ZERO_SHA256.to_string(),
            b"intent",
        );
        append_install_epoch_store_event_v1(&mut root, &intent, &digest('e')).unwrap();
        let payload = b"\0\xffsigned-request-secret\x80";
        let outbox = record_for_root(
            &root,
            2,
            intent.record_sha256().unwrap(),
            InstallEpochStoreEventKindV1::CasOutbox,
            1,
            intent.phase_successor_state_sha256.to_string(),
            payload,
        );
        append_install_epoch_store_event_v1(&mut root, &outbox, &digest('f')).unwrap();
        let expected_bytes = outbox.canonical_bytes().unwrap();
        let expected_sha256 = outbox.record_sha256().unwrap();
        assert!(!format!("{outbox:?}").contains("signed-request-secret"));

        drop(root);
        let mut reopened = open_test_trusted_state_root_v8(&state.path).unwrap();
        let scan = scan_install_epoch_store_v1(&mut reopened).unwrap();
        assert!(!format!("{scan:?}").contains("signed-request-secret"));
        let tip = match scan.recovery_view() {
            InstallEpochStoreRecoveryViewV1::Clean { tip } => tip,
            other => panic!("expected clean recovery tip, got {other:?}"),
        };
        assert_eq!(
            tip.completion_profile_sha256(),
            outbox.completion_profile_sha256
        );
        assert_eq!(tip.completion_slot_sha256(), outbox.completion_slot_sha256);
        assert_eq!(tip.epoch_sequence(), outbox.epoch_sequence);
        assert_eq!(tip.event_kind(), outbox.event_kind);
        assert_eq!(tip.event_sequence(), outbox.event_sequence);
        assert_eq!(tip.job_id_sha256(), outbox.job_id_sha256);
        assert_eq!(tip.layout_manifest_sha256(), outbox.layout_manifest_sha256);
        assert_eq!(tip.machine_id_sha256(), outbox.machine_id_sha256);
        assert_eq!(
            tip.operation_binding_sha256(),
            outbox.operation_binding_sha256
        );
        assert_eq!(tip.payload(), payload);
        assert_eq!(tip.phase_head_sha256(), outbox.phase_head_sha256);
        assert_eq!(
            tip.phase_predecessor_revision(),
            outbox.phase_predecessor_revision
        );
        assert_eq!(
            tip.phase_predecessor_state_sha256(),
            outbox.phase_predecessor_state_sha256
        );
        assert_eq!(
            tip.phase_successor_revision(),
            outbox.phase_successor_revision
        );
        assert_eq!(
            tip.phase_successor_state_sha256(),
            outbox.phase_successor_state_sha256
        );
        assert_eq!(
            tip.prepared_epoch_binding_sha256(),
            outbox.prepared_epoch_binding_sha256
        );
        assert_eq!(tip.previous_event_sha256(), outbox.previous_event_sha256);
        assert_eq!(
            tip.provider_profile_sha256(),
            outbox.provider_profile_sha256
        );
        assert_eq!(
            tip.state_root_profile_sha256(),
            outbox.state_root_profile_sha256
        );
        assert_eq!(tip.stream_id_sha256(), outbox.stream_id_sha256);
        assert_eq!(tip.canonical_bytes().unwrap(), expected_bytes);
        assert_eq!(tip.record_sha256(), expected_sha256);
        assert!(!format!("{tip:?}").contains("signed-request-secret"));

        drop(scan);
        drop(reopened);
        fs::write(
            state
                .path
                .join(INSTALL_EPOCH_DIRECTORY_V8)
                .join(".00000000000000000003.event.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.incoming"),
            b"partial",
        )
        .unwrap();
        let mut interrupted_root = open_test_trusted_state_root_v8(&state.path).unwrap();
        let interrupted = scan_install_epoch_store_v1(&mut interrupted_root).unwrap();
        match interrupted.recovery_view() {
            InstallEpochStoreRecoveryViewV1::Interrupted {
                committed_tip: Some(tip),
            } => assert_eq!(tip.record_sha256(), expected_sha256),
            other => panic!("expected interrupted committed prefix, got {other:?}"),
        }
    }

    #[test]
    fn incoming_unknown_and_illegal_transition_fail_closed() {
        let _process_guard = serialize_process_fd_lifetime();
        let state = TestRoot::new("negative");
        let mut root = open_test_trusted_state_root_v8(&state.path).unwrap();
        fs::write(
            state
                .path
                .join(INSTALL_EPOCH_DIRECTORY_V8)
                .join(".00000000000000000001.event.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.incoming"),
            b"partial",
        )
        .unwrap();
        let scan = scan_install_epoch_store_v1(&mut root).unwrap();
        assert!(scan.incoming_residue_detected());
        assert!(matches!(
            scan.recovery_view(),
            InstallEpochStoreRecoveryViewV1::Interrupted {
                committed_tip: None
            }
        ));

        drop(root);
        fs::remove_file(
            state
                .path
                .join(INSTALL_EPOCH_DIRECTORY_V8)
                .join(".00000000000000000001.event.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.incoming"),
        )
        .unwrap();
        fs::write(
            state.path.join(INSTALL_EPOCH_DIRECTORY_V8).join("unknown"),
            b"x",
        )
        .unwrap();
        let mut reopened = open_test_trusted_state_root_v8(&state.path).unwrap();
        assert!(matches!(
            scan_install_epoch_store_v1(&mut reopened),
            Err(InstallEpochStoreErrorV1::RecoveryRequired(_))
        ));
    }

    #[test]
    fn canonical_root_replacement_before_return_forces_recovery() {
        let state = TestRoot::new("root-swap");
        let moved = state.path.with_extension("anchored-old");
        let mut root = open_test_trusted_state_root_v8(&state.path).unwrap();
        let intent = record_for_root(
            &root,
            1,
            ZERO_SHA256.to_string(),
            InstallEpochStoreEventKindV1::Intent,
            0,
            ZERO_SHA256.to_string(),
            b"intent",
        );
        let mut replaced = false;
        let result =
            append_install_epoch_store_event_observed_v1(&mut root, &intent, &digest('e'), |_| {
                if replaced {
                    return;
                }
                replaced = true;
                fs::rename(&state.path, &moved).unwrap();
                fs::create_dir(&state.path).unwrap();
                fs::set_permissions(&state.path, fs::Permissions::from_mode(0o700)).unwrap();
                for name in [
                    ATTEMPTS_DIRECTORY_V8,
                    INSTALL_EPOCH_DIRECTORY_V8,
                    JOURNAL_DIRECTORY_V8,
                    NONCE_CLAIMS_DIRECTORY_V8,
                    QUARANTINE_DIRECTORY_V8,
                ] {
                    fs::create_dir(state.path.join(name)).unwrap();
                    fs::set_permissions(state.path.join(name), fs::Permissions::from_mode(0o700))
                        .unwrap();
                }
            });
        assert!(matches!(
            result,
            Err(InstallEpochStoreErrorV1::RecoveryRequired(_))
        ));
        assert!(
            moved
                .join(INSTALL_EPOCH_DIRECTORY_V8)
                .join("00000000000000000001.event")
                .is_file()
        );
        drop(root);
        fs::remove_dir_all(moved).unwrap();
    }
}

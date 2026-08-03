#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    use std::sync::Arc;
    use std::sync::Barrier;
    use std::thread;

    fn private_root() -> Result<tempfile::TempDir, PluginMutationJournalError> {
        let root = tempfile::tempdir()
            .map_err(|error| PluginMutationJournalError::new(format!("create tempdir: {error}")))?;
        #[cfg(unix)]
        std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o700))
            .map_err(|error| PluginMutationJournalError::new(format!("secure tempdir: {error}")))?;
        Ok(root)
    }

    #[cfg(unix)]
    fn production_fixture(
        mode: u32,
    ) -> Result<(tempfile::TempDir, PathBuf, PluginMutationJournal), PluginMutationJournalError>
    {
        let root = tempfile::tempdir()
            .map_err(|error| PluginMutationJournalError::new(format!("create tempdir: {error}")))?;
        let codex_home = root.path().join("codex-home");
        std::fs::create_dir(&codex_home).map_err(|error| {
            PluginMutationJournalError::new(format!("create CODEX_HOME: {error}"))
        })?;
        std::fs::set_permissions(&codex_home, std::fs::Permissions::from_mode(mode)).map_err(
            |error| PluginMutationJournalError::new(format!("set CODEX_HOME mode: {error}")),
        )?;
        let journal = PluginMutationJournal::for_codex_home_with_lookup(&codex_home, |_| None)?;
        Ok((root, codex_home, journal))
    }

    #[cfg(unix)]
    fn publish_legacy_bytes(path: &Path, bytes: &[u8]) -> Result<(), PluginMutationJournalError> {
        legacy_private_store(path, MAX_JOURNAL_BYTES, "hepta-plugin-mutation-legacy-test")?
            .publish(bytes)
            .map_err(|error| store_error("publish legacy plugin mutation test bytes", error))
    }

    #[cfg(unix)]
    fn publish_legacy_key(path: &Path, key: &[u8; 32]) -> Result<(), PluginMutationJournalError> {
        legacy_private_store(
            &path.with_extension("key"),
            MAX_KEY_BYTES,
            "hepta-plugin-mutation-legacy-key-test",
        )?
        .publish(hex_encode(key).as_bytes())
        .map_err(|error| store_error("publish legacy plugin mutation test key", error))
    }

    #[cfg(unix)]
    fn legacy_v1_state(
        generation: u64,
        record_index: u64,
    ) -> Result<LegacyPluginMutationState, PluginMutationJournalError> {
        let mut state = LegacyPluginMutationState {
            version: LEGACY_JOURNAL_VERSION,
            generation,
            records: vec![terminal_record(
                record_index,
                PluginMutationStatus::Succeeded,
            )],
            state_hash: String::new(),
        };
        state.refresh_hash()?;
        Ok(state)
    }

    fn envelope_index(index: u64) -> PluginMutationEnvelope {
        PluginMutationEnvelope {
            request_binding: format!("{index:064x}"),
            operation: "plugin_share_save".to_string(),
            target_binding: "plugin-path".to_string(),
            payload_digest: format!("sha256:{}", "a".repeat(64)),
            idempotency_binding: format!("{:064x}", index.saturating_add(10_000)),
            effect_plan_hash: format!("sha256:{}", "c".repeat(64)),
        }
    }

    fn terminal_record(index: u64, status: PluginMutationStatus) -> PluginMutationRecord {
        let (response, error) = match status {
            PluginMutationStatus::Succeeded => (Some(serde_json::json!({"index": index})), None),
            PluginMutationStatus::Failed => (None, Some(format!("failure-{index}"))),
            _ => (None, None),
        };
        PluginMutationRecord {
            envelope: envelope_index(index),
            status,
            provider_ack_hash: status
                .is_terminal()
                .then(|| format!("sha256:{}", "d".repeat(64))),
            terminal_receipt_hash: status
                .is_terminal()
                .then(|| format!("sha256:{}", "e".repeat(64))),
            response,
            error,
        }
    }

    #[test]
    fn replays_terminal_success_and_blocks_in_doubt() -> Result<(), PluginMutationJournalError> {
        let root = private_root()?;
        let journal = PluginMutationJournal::new(root.path().join("journal.json"));
        let envelope = envelope_index(1);
        assert_eq!(
            journal.begin(envelope.clone())?,
            PluginMutationBegin::Planned
        );
        assert_eq!(
            journal.begin(envelope.clone())?,
            PluginMutationBegin::InDoubt
        );
        journal.mark_committing(&envelope.request_binding)?;
        journal.succeed(
            &envelope.request_binding,
            format!("sha256:{}", "d".repeat(64)),
            format!("sha256:{}", "e".repeat(64)),
            serde_json::json!({"ok": true}),
        )?;
        assert_eq!(
            journal.begin(envelope)?,
            PluginMutationBegin::ReplayedSuccess(serde_json::json!({"ok": true}))
        );
        Ok(())
    }

    #[test]
    fn rejects_request_binding_reuse_with_different_payload()
    -> Result<(), PluginMutationJournalError> {
        let root = private_root()?;
        let journal = PluginMutationJournal::new(root.path().join("journal.json"));
        let first = envelope_index(2);
        journal.begin(first.clone())?;
        let mut conflicting = first;
        conflicting.payload_digest = format!("sha256:{}", "f".repeat(64));
        assert!(journal.begin(conflicting).is_err());
        Ok(())
    }

    #[test]
    fn checkpoint_preserves_success_and_failure_replay_after_record_limit()
    -> Result<(), PluginMutationJournalError> {
        let root = private_root()?;
        let path = root.path().join("journal.json");
        let journal = PluginMutationJournal::new(&path);
        journal.begin(envelope_index(1))?;
        journal.with_locked_state(|state| {
            state.records = (1..=MAX_RECORDS as u64)
                .map(|index| {
                    terminal_record(
                        index,
                        if index == 2 {
                            PluginMutationStatus::Failed
                        } else {
                            PluginMutationStatus::Succeeded
                        },
                    )
                })
                .collect();
            state.generation = state.generation.saturating_add(1);
            Ok(())
        })?;
        assert_eq!(
            journal.begin(envelope_index(MAX_RECORDS as u64 + 1))?,
            PluginMutationBegin::Planned
        );
        assert_eq!(
            journal.begin(envelope_index(1))?,
            PluginMutationBegin::ReplayedSuccess(serde_json::json!({"index": 1}))
        );
        assert_eq!(
            journal.begin(envelope_index(2))?,
            PluginMutationBegin::ReplayedFailure("failure-2".to_string())
        );
        let state: PluginMutationState =
            serde_json::from_slice(&read_private_bytes(&path, "plugin mutation journal")?)
                .map_err(|error| {
                    PluginMutationJournalError::new(format!("decode state: {error}"))
                })?;
        assert_eq!(
            state.checkpoint.compacted_records,
            (MAX_RECORDS - RETAIN_TERMINAL_RECORDS) as u64
        );
        Ok(())
    }

    #[test]
    fn rejects_authenticated_journal_rollback() -> Result<(), PluginMutationJournalError> {
        let root = private_root()?;
        let path = root.path().join("journal.json");
        let journal = PluginMutationJournal::new(&path);
        journal.begin(envelope_index(1))?;
        let old_state = read_private_bytes(&path, "plugin mutation journal")?;
        journal.begin(envelope_index(2))?;
        publish_private_bytes(&path, &old_state, "plugin mutation journal")?;
        let error = journal
            .begin(envelope_index(3))
            .expect_err("rollback must fail closed");
        assert!(error.to_string().contains("rollback detected"));
        Ok(())
    }

    #[test]
    fn migrates_verified_v1_state_and_creates_anchor() -> Result<(), PluginMutationJournalError> {
        let root = private_root()?;
        let path = root.path().join("journal.json");
        let mut legacy = LegacyPluginMutationState {
            version: LEGACY_JOURNAL_VERSION,
            generation: 7,
            records: vec![terminal_record(7, PluginMutationStatus::Succeeded)],
            state_hash: String::new(),
        };
        legacy.refresh_hash()?;
        publish_private_bytes(
            &path,
            &serde_json::to_vec(&legacy).map_err(|error| {
                PluginMutationJournalError::new(format!("encode legacy state: {error}"))
            })?,
            "plugin mutation journal",
        )?;
        let journal = PluginMutationJournal::new(&path);
        assert_eq!(
            journal.begin(envelope_index(7))?,
            PluginMutationBegin::ReplayedSuccess(serde_json::json!({"index": 7}))
        );
        let state: PluginMutationState =
            serde_json::from_slice(&read_private_bytes(&path, "plugin mutation journal")?)
                .map_err(|error| {
                    PluginMutationJournalError::new(format!("decode v2 state: {error}"))
                })?;
        assert_eq!(state.version, JOURNAL_VERSION);
        assert!(path.with_extension("key").is_file());
        assert!(path.with_extension("anchor").is_file());
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn production_journal_places_anchor_outside_codex_home()
    -> Result<(), PluginMutationJournalError> {
        let root = tempfile::tempdir()
            .map_err(|error| PluginMutationJournalError::new(format!("create tempdir: {error}")))?;
        let codex_home = root.path().join("codex-home");
        std::fs::create_dir(&codex_home).map_err(|error| {
            PluginMutationJournalError::new(format!("create CODEX_HOME: {error}"))
        })?;
        std::fs::set_permissions(&codex_home, std::fs::Permissions::from_mode(0o755)).map_err(
            |error| PluginMutationJournalError::new(format!("secure CODEX_HOME: {error}")),
        )?;
        let journal = PluginMutationJournal::for_codex_home_with_lookup(&codex_home, |_| None)?;
        assert!(!journal.anchor_path.starts_with(&codex_home));
        let second_codex_home = root.path().join("second-codex-home");
        std::fs::create_dir(&second_codex_home).map_err(|error| {
            PluginMutationJournalError::new(format!("create second CODEX_HOME: {error}"))
        })?;
        let second =
            PluginMutationJournal::for_codex_home_with_lookup(&second_codex_home, |_| None)?;
        assert_ne!(journal.anchor_path, second.anchor_path);
        journal.begin(envelope_index(1))?;
        assert!(journal.path.is_file());
        assert!(journal.anchor_path.is_file());
        assert_eq!(
            std::fs::metadata(&codex_home)
                .map_err(|error| PluginMutationJournalError::new(error.to_string()))?
                .permissions()
                .mode()
                & 0o7777,
            0o755
        );
        assert_eq!(
            std::fs::metadata(journal.path.parent().expect("active journal parent"))
                .map_err(|error| PluginMutationJournalError::new(error.to_string()))?
                .permissions()
                .mode()
                & 0o7777,
            0o700
        );
        for path in [
            journal.path.clone(),
            journal.path.with_extension("key"),
            journal.path.with_extension("lock"),
            journal
                .migration
                .as_ref()
                .expect("production migration")
                .marker_path
                .clone(),
        ] {
            assert_eq!(
                std::fs::metadata(path)
                    .map_err(|error| PluginMutationJournalError::new(error.to_string()))?
                    .permissions()
                    .mode()
                    & 0o7777,
                0o600
            );
        }
        let legacy_path = &journal
            .migration
            .as_ref()
            .expect("production migration")
            .legacy_path;
        let tombstone: RetiredLegacyPluginMutationJournal = serde_json::from_slice(
            &legacy_private_store(
                legacy_path,
                MAX_JOURNAL_BYTES,
                "hepta-plugin-mutation-legacy-test",
            )?
            .read()
            .map_err(|error| store_error("read legacy tombstone", error))?
            .expect("legacy tombstone"),
        )
        .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
        assert_eq!(tombstone.version, RETIRED_LEGACY_JOURNAL_VERSION);
        assert!(
            PluginMutationJournal::new(legacy_path)
                .begin(envelope_index(2))
                .is_err(),
            "a pre-migration binary must fail closed on the retired root journal"
        );
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn production_layout_migrates_verified_v1_without_chmodding_codex_home()
    -> Result<(), PluginMutationJournalError> {
        let (_root, codex_home, journal) = production_fixture(0o755)?;
        let layout = journal.migration.as_ref().expect("production migration");
        let legacy = legacy_v1_state(7, 7)?;
        publish_legacy_bytes(
            &layout.legacy_path,
            &serde_json::to_vec(&legacy)
                .map_err(|error| PluginMutationJournalError::new(error.to_string()))?,
        )?;

        assert_eq!(
            journal.begin(envelope_index(7))?,
            PluginMutationBegin::ReplayedSuccess(serde_json::json!({"index": 7}))
        );
        let state: PluginMutationState = serde_json::from_slice(&read_private_bytes(
            &journal.path,
            "plugin mutation journal",
        )?)
        .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
        assert_eq!(state.generation, 7);
        assert_eq!(state.records.len(), 1);
        let marker: PluginMutationMigrationMarker = serde_json::from_slice(&read_private_bytes(
            &layout.marker_path,
            "plugin mutation migration marker",
        )?)
        .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
        assert_eq!(marker.phase, MigrationPhase::Committed);
        assert_eq!(marker.claim.source, MigrationSource::LegacyV1);
        assert!(marker.claim.matches_state(&state));
        assert_eq!(
            std::fs::metadata(codex_home)
                .map_err(|error| PluginMutationJournalError::new(error.to_string()))?
                .permissions()
                .mode()
                & 0o7777,
            0o755
        );
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn production_layout_migrates_authenticated_v2_with_the_external_anchor()
    -> Result<(), PluginMutationJournalError> {
        let (_root, _codex_home, journal) = production_fixture(0o755)?;
        let layout = journal.migration.as_ref().expect("production migration");
        let key = [0x42; 32];
        let mut state = PluginMutationState::empty(&key)?;
        state.generation = 9;
        state
            .records
            .push(terminal_record(9, PluginMutationStatus::Succeeded));
        state.refresh_integrity(&key)?;
        publish_legacy_bytes(
            &layout.legacy_path,
            &serde_json::to_vec(&state)
                .map_err(|error| PluginMutationJournalError::new(error.to_string()))?,
        )?;
        publish_legacy_key(&layout.legacy_path, &key)?;
        publish_anchor(
            &private_store(
                &journal.anchor_path,
                MAX_ANCHOR_BYTES,
                "hepta-plugin-mutation-anchor-test",
            )?,
            &state,
            &key,
        )?;

        assert_eq!(
            journal.begin(envelope_index(9))?,
            PluginMutationBegin::ReplayedSuccess(serde_json::json!({"index": 9}))
        );
        let active_key = decode_key(&read_private_bytes(
            &journal.path.with_extension("key"),
            "plugin mutation journal key",
        )?)?;
        assert_eq!(active_key, key);
        let marker: PluginMutationMigrationMarker = serde_json::from_slice(&read_private_bytes(
            &layout.marker_path,
            "plugin mutation migration marker",
        )?)
        .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
        assert_eq!(marker.claim.source, MigrationSource::LegacyV2);
        assert_eq!(marker.phase, MigrationPhase::Committed);
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn production_layout_recovers_every_durable_migration_crash_boundary()
    -> Result<(), PluginMutationJournalError> {
        for (index, fault) in [
            MigrationFaultPoint::KeyPublication,
            MigrationFaultPoint::PreparingMarker,
            MigrationFaultPoint::StateCopy,
            MigrationFaultPoint::AnchorReconciliation,
            MigrationFaultPoint::CommitMarker,
            MigrationFaultPoint::LegacyRetirement,
        ]
        .into_iter()
        .enumerate()
        {
            let (_root, codex_home, journal) = production_fixture(0o755)?;
            let envelope = envelope_index(index as u64 + 1);
            let error = journal
                .clone()
                .with_migration_fault(fault)
                .begin(envelope.clone())
                .expect_err("injected migration crash must interrupt the first operation");
            assert!(error.to_string().contains("injected"));

            let recovered =
                PluginMutationJournal::for_codex_home_with_lookup(&codex_home, |_| None)?;
            assert_eq!(
                recovered.begin(envelope.clone())?,
                PluginMutationBegin::Planned
            );
            assert_eq!(recovered.begin(envelope)?, PluginMutationBegin::InDoubt);
            let layout = recovered.migration.as_ref().expect("production migration");
            let marker: PluginMutationMigrationMarker = serde_json::from_slice(
                &read_private_bytes(&layout.marker_path, "plugin mutation migration marker")?,
            )
            .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
            assert_eq!(marker.phase, MigrationPhase::Committed);
            let retired: VersionHeader = serde_json::from_slice(
                &legacy_private_store(
                    &layout.legacy_path,
                    MAX_JOURNAL_BYTES,
                    "hepta-plugin-mutation-legacy-test",
                )?
                .read()
                .map_err(|error| store_error("read recovered tombstone", error))?
                .expect("recovered tombstone"),
            )
            .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
            assert_eq!(retired.version, RETIRED_LEGACY_JOURNAL_VERSION);
        }
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn migration_rejects_corrupt_evidence_and_same_generation_source_forks()
    -> Result<(), PluginMutationJournalError> {
        let (_root, _codex_home, corrupt) = production_fixture(0o755)?;
        let corrupt_layout = corrupt.migration.as_ref().expect("production migration");
        publish_legacy_bytes(&corrupt_layout.legacy_path, b"{not-json")?;
        assert!(corrupt.begin(envelope_index(1)).is_err());
        assert!(!corrupt.path.exists());
        assert!(!corrupt.path.with_extension("key").exists());

        let (_root, codex_home, forked) = production_fixture(0o755)?;
        let layout = forked.migration.as_ref().expect("production migration");
        let original = legacy_v1_state(7, 7)?;
        publish_legacy_bytes(
            &layout.legacy_path,
            &serde_json::to_vec(&original)
                .map_err(|error| PluginMutationJournalError::new(error.to_string()))?,
        )?;
        let error = forked
            .clone()
            .with_migration_fault(MigrationFaultPoint::PreparingMarker)
            .begin(envelope_index(10))
            .expect_err("preparing marker fault");
        assert!(error.to_string().contains("injected"));
        let substituted = legacy_v1_state(7, 8)?;
        publish_legacy_bytes(
            &layout.legacy_path,
            &serde_json::to_vec(&substituted)
                .map_err(|error| PluginMutationJournalError::new(error.to_string()))?,
        )?;
        let resumed = PluginMutationJournal::for_codex_home_with_lookup(&codex_home, |_| None)?;
        let error = resumed
            .begin(envelope_index(10))
            .expect_err("same-generation legacy fork must fail closed");
        assert!(error.to_string().contains("diverged"));
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn migration_marker_and_tombstone_tampering_fail_closed()
    -> Result<(), PluginMutationJournalError> {
        let (_root, _codex_home, marker_tampered) = production_fixture(0o755)?;
        marker_tampered.begin(envelope_index(1))?;
        let marker_path = &marker_tampered
            .migration
            .as_ref()
            .expect("production migration")
            .marker_path;
        let mut marker: PluginMutationMigrationMarker = serde_json::from_slice(
            &read_private_bytes(marker_path, "plugin mutation migration marker")?,
        )
        .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
        marker.phase = MigrationPhase::Preparing;
        publish_private_bytes(
            marker_path,
            &serde_json::to_vec(&marker)
                .map_err(|error| PluginMutationJournalError::new(error.to_string()))?,
            "plugin mutation migration marker",
        )?;
        let error = marker_tampered
            .begin(envelope_index(2))
            .expect_err("marker HMAC must bind the migration phase");
        assert!(error.to_string().contains("integrity check failed"));

        let (_root, _codex_home, tombstone_tampered) = production_fixture(0o755)?;
        tombstone_tampered.begin(envelope_index(3))?;
        let legacy_path = &tombstone_tampered
            .migration
            .as_ref()
            .expect("production migration")
            .legacy_path;
        let mut tombstone: RetiredLegacyPluginMutationJournal = serde_json::from_slice(
            &legacy_private_store(
                legacy_path,
                MAX_JOURNAL_BYTES,
                "hepta-plugin-mutation-legacy-test",
            )?
            .read()
            .map_err(|error| store_error("read legacy tombstone", error))?
            .expect("legacy tombstone"),
        )
        .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
        tombstone.claim.generation = tombstone.claim.generation.saturating_add(1);
        publish_legacy_bytes(
            legacy_path,
            &serde_json::to_vec(&tombstone)
                .map_err(|error| PluginMutationJournalError::new(error.to_string()))?,
        )?;
        let error = tombstone_tampered
            .begin(envelope_index(4))
            .expect_err("tombstone HMAC must bind the migration claim");
        assert!(error.to_string().contains("integrity check failed"));
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn production_layout_rejects_writable_parents_and_symlink_redirection()
    -> Result<(), PluginMutationJournalError> {
        let (_root, codex_home, writable) = production_fixture(0o775)?;
        let error = writable
            .begin(envelope_index(1))
            .expect_err("group-writable CODEX_HOME must fail closed");
        assert!(error.to_string().contains("not writable by group or other"));
        assert!(!codex_home.join(".hepta-authority").exists());

        let (_root, codex_home, linked_secure_root) = production_fixture(0o755)?;
        let outside = codex_home.parent().expect("fixture parent").join("outside");
        std::fs::create_dir(&outside)
            .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
        std::fs::set_permissions(&outside, std::fs::Permissions::from_mode(0o700))
            .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
        symlink(&outside, codex_home.join(".hepta-authority"))
            .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
        assert!(linked_secure_root.begin(envelope_index(2)).is_err());
        assert!(!outside.join("plugin-mutation").exists());

        let (_root, codex_home, linked_legacy) = production_fixture(0o755)?;
        let victim = codex_home.join("victim.json");
        std::fs::write(&victim, b"victim")
            .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
        std::fs::set_permissions(&victim, std::fs::Permissions::from_mode(0o600))
            .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
        let legacy_path = &linked_legacy
            .migration
            .as_ref()
            .expect("production migration")
            .legacy_path;
        symlink(&victim, legacy_path)
            .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
        assert!(linked_legacy.begin(envelope_index(3)).is_err());
        assert_eq!(
            std::fs::read(&victim)
                .map_err(|error| PluginMutationJournalError::new(error.to_string()))?,
            b"victim"
        );
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn production_dual_lock_serializes_first_use_and_reentrant_calls()
    -> Result<(), PluginMutationJournalError> {
        let (_root, _codex_home, journal) = production_fixture(0o755)?;
        let journal = Arc::new(journal);
        let barrier = Arc::new(Barrier::new(8));
        let mut workers = Vec::new();
        for index in 1..=8 {
            let journal = Arc::clone(&journal);
            let barrier = Arc::clone(&barrier);
            workers.push(thread::spawn(move || {
                barrier.wait();
                journal.begin(envelope_index(index))
            }));
        }
        for worker in workers {
            assert_eq!(
                worker
                    .join()
                    .map_err(|_| PluginMutationJournalError::new("worker panicked"))??,
                PluginMutationBegin::Planned
            );
        }
        assert_eq!(
            journal.begin(envelope_index(1))?,
            PluginMutationBegin::InDoubt
        );
        let state: PluginMutationState = serde_json::from_slice(&read_private_bytes(
            &journal.path,
            "plugin mutation journal",
        )?)
        .map_err(|error| PluginMutationJournalError::new(error.to_string()))?;
        assert_eq!(state.records.len(), 8);
        assert_eq!(state.generation, 8);
        Ok(())
    }

    #[test]
    fn external_anchor_rejects_whole_codex_home_rollback() -> Result<(), PluginMutationJournalError>
    {
        let root = tempfile::tempdir()
            .map_err(|error| PluginMutationJournalError::new(format!("create tempdir: {error}")))?;
        let codex_home = root.path().join("codex-home");
        std::fs::create_dir(&codex_home).map_err(|error| {
            PluginMutationJournalError::new(format!("create CODEX_HOME: {error}"))
        })?;
        #[cfg(unix)]
        std::fs::set_permissions(&codex_home, std::fs::Permissions::from_mode(0o700)).map_err(
            |error| PluginMutationJournalError::new(format!("secure CODEX_HOME: {error}")),
        )?;
        let journal = PluginMutationJournal::for_codex_home_with_lookup(&codex_home, |_| None)?;
        journal.begin(envelope_index(1))?;
        let old_state = read_private_bytes(&journal.path, "plugin mutation journal")?;
        journal.begin(envelope_index(2))?;
        publish_private_bytes(&journal.path, &old_state, "plugin mutation journal")?;
        let error = journal
            .begin(envelope_index(3))
            .expect_err("external anchor must reject CODEX_HOME rollback");
        assert!(error.to_string().contains("rollback detected"));
        Ok(())
    }

    #[test]
    fn terminal_payloads_are_bounded() -> Result<(), PluginMutationJournalError> {
        let root = private_root()?;
        let journal = PluginMutationJournal::new(root.path().join("journal.json"));
        let envelope = envelope_index(1);
        journal.begin(envelope.clone())?;
        journal.mark_committing(&envelope.request_binding)?;
        let response = serde_json::json!({"payload": "x".repeat(MAX_TERMINAL_RESPONSE_BYTES)});
        let error = journal
            .succeed(
                &envelope.request_binding,
                format!("sha256:{}", "d".repeat(64)),
                format!("sha256:{}", "e".repeat(64)),
                response,
            )
            .expect_err("oversized terminal response must fail closed");
        assert!(error.to_string().contains("terminal response exceeds"));
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn test_constructor_rejects_non_private_parent() -> Result<(), PluginMutationJournalError> {
        let root = private_root()?;
        let public = root.path().join("public");
        std::fs::create_dir(&public).map_err(|error| {
            PluginMutationJournalError::new(format!("create public parent: {error}"))
        })?;
        std::fs::set_permissions(&public, std::fs::Permissions::from_mode(0o755)).map_err(
            |error| PluginMutationJournalError::new(format!("set public parent mode: {error}")),
        )?;
        let journal = PluginMutationJournal::new(public.join("journal.json"));
        let error = journal
            .begin(envelope_index(1))
            .expect_err("non-private journal parent must fail closed");
        assert!(error.to_string().contains("owned mode-0700 directory"));
        Ok(())
    }
}

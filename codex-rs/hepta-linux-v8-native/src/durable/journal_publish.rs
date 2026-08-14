use std::path::Path;

use crate::DirectoryAnchorV8;
use crate::NativeErrorV8;
use crate::StateRootLockV8;
use crate::invalid;

use super::DurableJournalRecordV8;
use super::FreshActiveAttemptPublicationV8;
use super::PublishedRecordV8;
use super::VerifiedDurableJournalScanV8;
use super::attempt_relative_path_v8;
use super::journal_record_name_v8;
use super::scan_journal_directory_v8;

/// A newly appended record plus a fresh anchored replay of the entire chain.
#[derive(Debug)]
pub struct PublishedDurableJournalRecordV8 {
    publication: PublishedRecordV8,
    verified_chain: VerifiedDurableJournalScanV8,
}

impl PublishedDurableJournalRecordV8 {
    pub fn publication(&self) -> &PublishedRecordV8 {
        &self.publication
    }

    pub fn verified_chain(&self) -> &VerifiedDurableJournalScanV8 {
        &self.verified_chain
    }
}

pub fn append_journal_record_durably_v8(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &mut StateRootLockV8,
    active_attempt: &FreshActiveAttemptPublicationV8,
    record: &DurableJournalRecordV8,
    publication_nonce: &str,
) -> Result<PublishedDurableJournalRecordV8, NativeErrorV8> {
    append_journal_record_durably_observed_v8(
        state_root,
        state_root_lock,
        active_attempt,
        record,
        publication_nonce,
        |_| {},
    )
}

pub(super) fn append_journal_record_durably_observed_v8<F>(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &mut StateRootLockV8,
    active_attempt: &FreshActiveAttemptPublicationV8,
    record: &DurableJournalRecordV8,
    publication_nonce: &str,
    observe: F,
) -> Result<PublishedDurableJournalRecordV8, NativeErrorV8>
where
    F: FnMut(super::DurablePublicationCheckpointV8),
{
    if !state_root_lock
        .state_root_identity()
        .matches_stable_directory(state_root.identity())
    {
        return Err(invalid(
            "durable journal lock belongs to a different state root",
        ));
    }
    if !active_attempt.matches_lock(state_root_lock)
        || !active_attempt.matches_state_root(state_root.identity())
        || active_attempt.attempt_identity_sha256() != record.attempt_identity_sha256()
        || active_attempt.boot_id() != record.boot_id()
    {
        return Err(invalid(
            "durable journal append does not match the sole fresh active attempt",
        ));
    }
    state_root_lock.revalidate_for_root(state_root)?;
    let attempt_directory = attempt_relative_path_v8(record.attempt_identity_sha256())?;
    let journal_relative = format!("{attempt_directory}/journal");
    let journal_directory = state_root.open_directory_beneath(Path::new(&journal_relative))?;

    if record.global_sequence() == 1 {
        if !journal_directory
            .list_leaf_names_bounded(super::MAX_DURABLE_JOURNAL_LEAVES_V8)?
            .is_empty()
        {
            return Err(invalid(
                "first durable journal append requires an empty journal directory",
            ));
        }
    } else {
        let prior = scan_journal_directory_v8(
            &journal_directory,
            record.attempt_identity_sha256(),
            state_root.identity(),
        )?;
        if prior.incoming_residue_detected() {
            return Err(invalid(
                "durable journal has an interrupted incoming publication",
            ));
        }
        if prior.record_count()
            >= u64::try_from(super::MAX_DURABLE_JOURNAL_RECORDS_V8)
                .map_err(|_| invalid("durable journal record limit does not fit u64"))?
        {
            return Err(invalid(
                "durable journal reached its frozen record inventory limit",
            ));
        }
        if prior.record_count().checked_add(1) != Some(record.global_sequence())
            || prior.tip_sha256() != record.previous_record_sha256()
        {
            return Err(invalid(
                "durable journal append does not extend the exact committed tip",
            ));
        }
    }

    let final_name = journal_record_name_v8(record.global_sequence())?;
    state_root_lock.revalidate_for_root(state_root)?;
    let publication = super::publish_record_noreplace_observed_v8(
        &journal_directory,
        &final_name,
        publication_nonce,
        &record.canonical_bytes()?,
        observe,
    )?;
    state_root_lock.revalidate_for_root(state_root)?;
    let verified_chain = scan_journal_directory_v8(
        &journal_directory,
        record.attempt_identity_sha256(),
        state_root.identity(),
    )?;
    if verified_chain.incoming_residue_detected()
        || verified_chain.record_count() != record.global_sequence()
        || verified_chain.tip_sha256() != record.record_sha256()?
    {
        return Err(invalid(
            "durable journal post-publication replay does not match the appended record",
        ));
    }
    Ok(PublishedDurableJournalRecordV8 {
        publication,
        verified_chain,
    })
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use std::ffi::OsStr;
    use std::fs;

    use super::*;
    use crate::ATTEMPTS_DIRECTORY_V8;
    use crate::ActiveAttemptPublicationOutcomeV8;
    use crate::ActiveAttemptRequestV8;
    use crate::acquire_state_root_lock_v8;
    use crate::publish_active_attempt_durably_v8;

    const ZERO_SHA256: &str = "0000000000000000000000000000000000000000000000000000000000000000";

    fn digest(character: char) -> String {
        character.to_string().repeat(64)
    }

    fn temporary_state_root(attempt: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "hepta-linux-v8-journal-publish-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&root).unwrap();
        fs::create_dir_all(
            root.join(ATTEMPTS_DIRECTORY_V8)
                .join(attempt)
                .join("journal"),
        )
        .unwrap();
        root
    }

    fn record(attempt: &str, sequence: u64, previous: String) -> DurableJournalRecordV8 {
        DurableJournalRecordV8::new(
            attempt.to_string(),
            1,
            "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            sequence,
            previous,
            format!("payload-{sequence}").into_bytes(),
        )
        .unwrap()
    }

    #[test]
    fn locked_append_replays_exact_chain_and_rejects_duplicate() {
        const CHILD_ENV: &str = "HEPTA_LINUX_V8_JOURNAL_REACQUIRE_CHILD";
        if std::env::var_os(CHILD_ENV).is_none() {
            // A concurrently spawned test child briefly inherits every live
            // descriptor before exec closes O_CLOEXEC handles. Isolate the
            // drop-then-reacquire assertion so that transient fork inheritance
            // cannot retain this test's flock after the owning token drops.
            let status = std::process::Command::new(
                std::env::current_exe().expect("current test executable"),
            )
            .arg("--exact")
            .arg("durable::journal_publish::tests::locked_append_replays_exact_chain_and_rejects_duplicate")
            .arg("--nocapture")
            .arg("--test-threads=1")
            .env(CHILD_ENV, "1")
            .status()
            .expect("launch isolated journal reacquire child");
            assert!(status.success(), "journal reacquire child failed");
            return;
        }

        let attempt = digest('1');
        let root = temporary_state_root(&attempt);
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let mut lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
        let active = match publish_active_attempt_durably_v8(
            &anchor,
            &mut lock,
            &ActiveAttemptRequestV8::new(
                attempt.clone(),
                7,
                "01234567-89ab-cdef-0123-456789abcdef".to_string(),
                digest('8'),
                digest('9'),
            )
            .unwrap(),
            &digest('7'),
        )
        .unwrap()
        {
            ActiveAttemptPublicationOutcomeV8::Fresh(active) => active,
            ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(_) => panic!(),
        };
        let first = record(&attempt, 1, ZERO_SHA256.to_string());
        let first_published =
            append_journal_record_durably_v8(&anchor, &mut lock, &active, &first, &digest('2'))
                .unwrap();
        assert_eq!(first_published.verified_chain().record_count(), 1);

        let second = record(&attempt, 2, first.record_sha256().unwrap());
        let second_published =
            append_journal_record_durably_v8(&anchor, &mut lock, &active, &second, &digest('3'))
                .unwrap();
        assert_eq!(second_published.verified_chain().record_count(), 2);
        assert!(
            append_journal_record_durably_v8(&anchor, &mut lock, &active, &second, &digest('4'))
                .is_err()
        );
        let wrong_attempt = record(&digest('5'), 1, ZERO_SHA256.to_string());
        assert!(
            append_journal_record_durably_v8(
                &anchor,
                &mut lock,
                &active,
                &wrong_attempt,
                &digest('6')
            )
            .is_err()
        );
        drop(lock);
        let mut reacquired = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
        let third = record(&attempt, 3, second.record_sha256().unwrap());
        assert!(
            append_journal_record_durably_v8(
                &anchor,
                &mut reacquired,
                &active,
                &third,
                &digest('7')
            )
            .is_err()
        );
        drop(reacquired);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn lock_from_another_root_is_rejected() {
        let attempt = digest('1');
        let root = temporary_state_root(&attempt);
        let other_root = temporary_state_root(&attempt);
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let other_anchor = DirectoryAnchorV8::open(&other_root).unwrap();
        let mut other_lock =
            acquire_state_root_lock_v8(&other_anchor, OsStr::new("state.lock")).unwrap();
        let active = match publish_active_attempt_durably_v8(
            &other_anchor,
            &mut other_lock,
            &ActiveAttemptRequestV8::new(
                attempt.clone(),
                7,
                "01234567-89ab-cdef-0123-456789abcdef".to_string(),
                digest('8'),
                digest('9'),
            )
            .unwrap(),
            &digest('7'),
        )
        .unwrap()
        {
            ActiveAttemptPublicationOutcomeV8::Fresh(active) => active,
            ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(_) => panic!(),
        };
        let first = record(&attempt, 1, ZERO_SHA256.to_string());
        assert!(
            append_journal_record_durably_v8(
                &anchor,
                &mut other_lock,
                &active,
                &first,
                &digest('2')
            )
            .is_err()
        );
        drop(other_lock);
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(other_root).unwrap();
    }
}

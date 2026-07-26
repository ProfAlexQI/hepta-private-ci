use super::*;
use hepta_contracts::ContentHash;
use std::os::unix::fs::PermissionsExt;
use std::sync::mpsc;
use std::time::Duration;

fn candidate(revision: u64, content_hash: &str) -> PreferenceAttachmentCandidate {
    PreferenceAttachmentCandidate {
        session_binding_hash: "sha256:session".into(),
        subject: "operator".into(),
        preference: "preference".into(),
        stamp: RevisionStamp::new(Revision::new(revision), ContentHash::new(content_hash)),
    }
}

#[test]
fn attachment_rejects_tampering_and_non_private_mode() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let database = root.path().join("preferences.sqlite3");
    let store = PreferenceAttachmentStore::for_database(&database, [7; 32])?;
    let candidate = candidate(4, "sha256:preference");
    store.persist(&candidate)?;
    assert!(store.read_verified()?.is_some());

    let mut bytes = fs::read(&store.path)?;
    let index = bytes
        .iter()
        .position(|byte| *byte == b'4')
        .context("revision")?;
    bytes[index] = b'5';
    fs::write(&store.path, bytes)?;
    assert!(store.read_verified().is_err());
    assert!(store.persist(&candidate).is_err());

    fs::remove_file(&store.path)?;
    store.persist(&candidate)?;
    fs::set_permissions(&store.path, fs::Permissions::from_mode(0o644))?;
    assert!(store.read_verified().is_err());
    Ok(())
}

#[test]
fn attachment_persistence_rejects_revision_rollback_and_divergence() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let database = root.path().join("preferences.sqlite3");
    let store = PreferenceAttachmentStore::for_database(&database, [8; 32])?;

    let revision_two = candidate(2, "sha256:revision-two");
    store.persist(&revision_two)?;
    store.persist(&revision_two)?;
    assert!(store.persist(&candidate(1, "sha256:revision-one")).is_err());
    assert!(
        store
            .persist(&candidate(2, "sha256:revision-two-diverged"))
            .is_err()
    );

    store.persist(&candidate(3, "sha256:revision-three"))?;
    let current = store.read_verified()?.context("current attachment")?;
    assert_eq!(current.revision, 3);
    assert_eq!(current.content_hash, "sha256:revision-three");
    Ok(())
}

#[test]
fn concurrent_stores_serialize_verified_read_and_publish_with_sidecar_flock() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let database = root.path().join("preferences.sqlite3");
    let low_store = PreferenceAttachmentStore::for_database(&database, [9; 32])?;
    let high_store = PreferenceAttachmentStore::for_database(&database, [9; 32])?;
    let verification_store = PreferenceAttachmentStore::for_database(&database, [9; 32])?;
    let (low_read_tx, low_read_rx) = mpsc::sync_channel(0);
    let (release_low_tx, release_low_rx) = mpsc::sync_channel(0);
    let low = std::thread::spawn(move || {
        low_store.persist_with_after_verified_read(&candidate(1, "sha256:revision-one"), || {
            low_read_tx.send(()).expect("signal low verified read");
            release_low_rx.recv().expect("release low publish");
        })
    });
    low_read_rx.recv().context("low verified read")?;

    let (high_started_tx, high_started_rx) = mpsc::sync_channel(0);
    let (high_done_tx, high_done_rx) = mpsc::sync_channel(1);
    let high = std::thread::spawn(move || {
        high_started_tx.send(()).expect("signal high start");
        let result = high_store
            .persist(&candidate(2, "sha256:revision-two"))
            .map_err(|error| format!("{error:#}"));
        high_done_tx.send(result).expect("signal high completion");
    });
    high_started_rx.recv().context("high start")?;
    assert!(matches!(
        high_done_rx.recv_timeout(Duration::from_millis(200)),
        Err(mpsc::RecvTimeoutError::Timeout)
    ));

    release_low_tx.send(()).context("release low publish")?;
    low.join()
        .map_err(|_| anyhow::anyhow!("low attachment thread panicked"))??;
    high_done_rx
        .recv_timeout(Duration::from_secs(5))
        .context("high attachment completion")?
        .map_err(anyhow::Error::msg)?;
    high.join()
        .map_err(|_| anyhow::anyhow!("high attachment thread panicked"))?;

    let current = verification_store
        .read_verified()?
        .context("concurrent attachment")?;
    assert_eq!(current.revision, 2);
    assert_eq!(current.content_hash, "sha256:revision-two");
    assert!(
        verification_store
            .persist(&candidate(1, "sha256:revision-one"))
            .is_err()
    );
    assert_eq!(
        verification_store
            .read_verified()?
            .context("attachment after rejected rollback")?
            .revision,
        2
    );
    Ok(())
}

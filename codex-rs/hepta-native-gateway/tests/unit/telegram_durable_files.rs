use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::symlink;

use super::*;

#[test]
fn cursor_round_trip_is_private_atomic_and_parseable() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let cursor = root.path().join("telegram/cursor.json");
    write_cursor_next_update_offset(&cursor, 41)?;
    let metadata = fs::metadata(&cursor)?;
    assert_eq!(metadata.permissions().mode() & 0o777, 0o600);
    let status = cursor_status(true, &cursor, ".hepta/telegram/cursor.json");
    assert!(status.cursor_parse_ok);
    assert_eq!(status.next_update_offset, Some(41));
    Ok(())
}

#[test]
fn cursor_rejects_final_and_intermediate_symlinks_without_touching_victim() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let private = root.path().join("private");
    fs::create_dir(&private)?;
    fs::set_permissions(&private, fs::Permissions::from_mode(0o700))?;
    let victim = private.join("victim");
    fs::write(&victim, b"unchanged")?;
    fs::set_permissions(&victim, fs::Permissions::from_mode(0o600))?;

    let final_link = private.join("cursor.json");
    symlink(&victim, &final_link)?;
    assert!(write_cursor_next_update_offset(&final_link, 7).is_err());
    assert_eq!(fs::read(&victim)?, b"unchanged");

    let outside = root.path().join("outside");
    fs::create_dir(&outside)?;
    fs::set_permissions(&outside, fs::Permissions::from_mode(0o700))?;
    let intermediate = root.path().join("linked");
    symlink(&outside, &intermediate)?;
    assert!(write_cursor_next_update_offset(&intermediate.join("cursor.json"), 8).is_err());
    assert!(!outside.join("cursor.json").exists());
    Ok(())
}

#[test]
fn delivery_ledger_is_private_fsynced_and_rejects_symlink_redirection() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let ledger = root.path().join("telegram/delivery.jsonl");
    append_delivery_lifecycle_record(&ledger, &serde_json::json!({"stage":"enqueued"}))?;
    append_delivery_lifecycle_record(&ledger, &serde_json::json!({"stage":"acked"}))?;
    let metadata = fs::metadata(&ledger)?;
    assert_eq!(metadata.permissions().mode() & 0o777, 0o600);
    let raw = fs::read_to_string(&ledger)?;
    assert_eq!(raw.lines().count(), 2);

    let victim = root.path().join("victim");
    fs::write(&victim, b"unchanged")?;
    fs::set_permissions(&victim, fs::Permissions::from_mode(0o600))?;
    let redirected = root.path().join("telegram/redirected.jsonl");
    symlink(&victim, &redirected)?;
    assert!(
        append_delivery_lifecycle_record(&redirected, &serde_json::json!({"stage":"forbidden"}))
            .is_err()
    );
    assert_eq!(fs::read(&victim)?, b"unchanged");
    Ok(())
}

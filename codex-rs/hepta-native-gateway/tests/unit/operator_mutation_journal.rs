use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::symlink;

use super::*;

#[test]
fn journal_consumes_mutation_and_commit_exactly_once() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let journal = OperatorMutationJournal::for_test_path(root.path().join("journal.json"));
    let key = [7_u8; 32];
    let mutation = "1".repeat(64);
    let plan = "2".repeat(64);
    let plan_request = "3".repeat(64);
    let session = "4".repeat(64);
    let candidate = format!("sha256:{}", "5".repeat(64));
    let commit_request = "6".repeat(64);
    let receipt = RuntimeExecutionReceipt {
        attempt_id: "attempt:operator-note".to_string(),
        durable_intent_recorded: true,
        effect_plan_recorded: true,
        effect_plan_hash: Some(format!("sha256:{}", "7".repeat(64))),
        provider_effect_ack_hash: Some(format!("sha256:{}", "8".repeat(64))),
        terminal_receipt_id: "receipt:operator-note".to_string(),
        terminal_receipt_hash: format!("sha256:{}", "9".repeat(64)),
        terminal_outcome_hash: format!("sha256:{}", "a".repeat(64)),
        terminal_evidence_hash: format!("sha256:{}", "b".repeat(64)),
        terminal_status: "succeeded".to_string(),
    };

    journal.reserve_plan(&key, &mutation, &plan, &plan_request, &session)?;
    assert!(
        journal
            .reserve_plan(&key, &mutation, &"8".repeat(64), &plan_request, &session)
            .is_err()
    );
    journal.publish_candidate(&key, &plan, &candidate)?;
    journal.begin_commit(
        &key,
        &mutation,
        &plan,
        &plan_request,
        &session,
        &candidate,
        &commit_request,
    )?;
    assert!(
        journal
            .begin_commit(
                &key,
                &mutation,
                &plan,
                &plan_request,
                &session,
                &candidate,
                &commit_request,
            )
            .is_err()
    );
    journal.record_runtime_linkage(&key, &plan, &receipt)?;
    let pending = journal.inspect(&key, &plan)?;
    assert_eq!(pending.phase, "in_doubt");
    assert_eq!(pending.attempt_id.as_deref(), Some("attempt:operator-note"));
    journal.finalize_linked_success(&key, &plan, &receipt)?;
    assert_eq!(journal.inspect(&key, &plan)?.phase, "succeeded");
    assert!(journal.mark_in_doubt(&key, &plan).is_err());
    Ok(())
}

#[test]
fn monotonic_state_exposes_authenticated_revision_and_deletion_as_genesis() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let path = root.path().join("journal.json");
    let journal = OperatorMutationJournal::for_test_path(path.clone());
    let key = [13_u8; 32];
    let genesis = journal.monotonic_state(&key)?;
    assert_eq!(genesis.journal_revision, 0);

    journal.reserve_plan(
        &key,
        &"1".repeat(64),
        &"2".repeat(64),
        &"3".repeat(64),
        &"4".repeat(64),
    )?;
    let advanced = journal.monotonic_state(&key)?;
    assert_eq!(advanced.journal_revision, 1);
    assert_ne!(advanced.state_hash, genesis.state_hash);

    fs::remove_file(path)?;
    assert_eq!(journal.monotonic_state(&key)?, genesis);
    Ok(())
}

#[test]
fn journal_rejects_tampering_and_symlink_redirection() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let path = root.path().join("journal.json");
    let journal = OperatorMutationJournal::for_test_path(path.clone());
    let key = [9_u8; 32];
    journal.reserve_plan(
        &key,
        &"1".repeat(64),
        &"2".repeat(64),
        &"3".repeat(64),
        &"4".repeat(64),
    )?;
    let mut bytes = fs::read(&path)?;
    let index = bytes
        .iter()
        .position(|byte| *byte == b'2')
        .context("journal contains plan hash")?;
    bytes[index] = b'a';
    fs::write(&path, bytes)?;
    assert!(
        journal
            .reserve_plan(
                &key,
                &"5".repeat(64),
                &"6".repeat(64),
                &"7".repeat(64),
                &"8".repeat(64),
            )
            .is_err()
    );

    fs::remove_file(&path)?;
    let victim = root.path().join("victim");
    fs::write(&victim, b"unchanged")?;
    fs::set_permissions(&victim, fs::Permissions::from_mode(0o600))?;
    symlink(&victim, &path)?;
    assert!(
        journal
            .reserve_plan(
                &key,
                &"5".repeat(64),
                &"6".repeat(64),
                &"7".repeat(64),
                &"8".repeat(64),
            )
            .is_err()
    );
    assert_eq!(fs::read(&victim)?, b"unchanged");
    Ok(())
}

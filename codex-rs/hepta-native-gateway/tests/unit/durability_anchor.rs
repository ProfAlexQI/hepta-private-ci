use super::*;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::time::Duration;

impl ExternalMonotonicAnchorConfig {
    fn for_test(root: &Path) -> Result<Self> {
        let key_file = root.join("anchor.key");
        fs::write(
            &key_file,
            b"606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f",
        )?;
        fs::set_permissions(&key_file, fs::Permissions::from_mode(0o600))?;
        Ok(Self {
            path: root.join("monotonic.anchor"),
            key_file,
        })
    }
}

impl ExternalMonotonicAnchor {
    fn ensure_healthy(&self) -> Result<()> {
        let operation = self
            .operation
            .lock()
            .map_err(|_| anyhow::anyhow!("external monotonic anchor operation mutex poisoned"))?;
        ensure_operation_healthy(&operation)
    }

    fn verify_and_advance_with_after_health_check(
        &self,
        outcome: &DurableMonotonicState,
        preference: &DurableMonotonicState,
        telegram: Option<&TelegramAuthorityMonotonicState>,
        operator: Option<&OperatorMutationMonotonicState>,
        after_health_check: impl FnOnce(),
    ) -> Result<()> {
        self.run_serialized_operation(|| {
            after_health_check();
            self.verify_and_advance_inner(outcome, preference, telegram, operator)
        })
    }
}

fn state(generation: u64, hash: &str) -> DurableMonotonicState {
    serde_json::from_value(serde_json::json!({
        "generation": generation,
        "state_hash": hash,
    }))
    .expect("state")
}

fn telegram_state(sequence: u64, hash: &str) -> TelegramAuthorityMonotonicState {
    TelegramAuthorityMonotonicState {
        schema: "hepta.telegram.operator-authority.monotonic-state.v1",
        authority_owner: "TelegramPipelineAuthority",
        journal_sequence: sequence,
        latest_event_hash: hash.to_owned(),
        latest_event_mac: Some("a".repeat(64)),
    }
}

fn operator_state(revision: u64, hash: &str) -> OperatorMutationMonotonicState {
    OperatorMutationMonotonicState {
        schema: "hepta.native.operator-mutation-journal.v1",
        journal_revision: revision,
        state_hash: hash.to_owned(),
    }
}

#[test]
fn anchor_advances_and_fails_closed_on_rollback_or_tamper() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let config = ExternalMonotonicAnchorConfig::for_test(root.path())?;
    let anchor = ExternalMonotonicAnchor::open(config.clone())?;
    anchor.verify_and_advance(&state(2, "sha256:o2"), &state(3, "sha256:p3"), None, None)?;
    anchor.verify_and_advance(&state(4, "sha256:o4"), &state(3, "sha256:p3"), None, None)?;
    assert_eq!(fs::read_to_string(&config.path)?.lines().count(), 2);

    let reopened = ExternalMonotonicAnchor::open(config.clone())?;
    assert!(
        reopened
            .verify_and_advance(&state(1, "sha256:o1"), &state(3, "sha256:p3"), None, None,)
            .is_err()
    );
    assert!(reopened.ensure_healthy().is_err());

    let mut bytes = fs::read(&config.path)?;
    let index = bytes
        .iter()
        .position(|byte| *byte == b'o')
        .context("anchored hash byte")?;
    bytes[index] = b'x';
    fs::write(&config.path, bytes)?;
    let tampered = ExternalMonotonicAnchor::open(config)?;
    assert!(
        tampered
            .verify_and_advance(&state(4, "sha256:o4"), &state(3, "sha256:p3"), None, None,)
            .is_err()
    );
    Ok(())
}

#[test]
fn anchor_binds_telegram_journal_after_enablement_and_rejects_rollback() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let config = ExternalMonotonicAnchorConfig::for_test(root.path())?;
    let anchor = ExternalMonotonicAnchor::open(config.clone())?;
    let outcome = state(2, "sha256:o2");
    let preference = state(3, "sha256:p3");
    anchor.verify_and_advance(&outcome, &preference, None, None)?;
    anchor.verify_and_advance(
        &outcome,
        &preference,
        Some(&telegram_state(4, "sha256:t4")),
        None,
    )?;
    anchor.verify_and_advance(
        &outcome,
        &preference,
        Some(&telegram_state(6, "sha256:t6")),
        None,
    )?;

    let reopened = ExternalMonotonicAnchor::open(config)?;
    assert!(
        reopened
            .verify_and_advance(
                &outcome,
                &preference,
                Some(&telegram_state(5, "sha256:t5")),
                None,
            )
            .is_err()
    );
    assert!(reopened.ensure_healthy().is_err());
    Ok(())
}

#[test]
fn anchor_binds_operator_journal_and_rejects_deletion_or_same_revision_divergence() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let config = ExternalMonotonicAnchorConfig::for_test(root.path())?;
    let outcome = state(2, "sha256:o2");
    let preference = state(3, "sha256:p3");
    let anchor = ExternalMonotonicAnchor::open(config.clone())?;
    anchor.verify_and_advance(
        &outcome,
        &preference,
        None,
        Some(&operator_state(4, "sha256:j4")),
    )?;
    anchor.verify_and_advance(
        &outcome,
        &preference,
        None,
        Some(&operator_state(6, "sha256:j6")),
    )?;

    let deleted = ExternalMonotonicAnchor::open(config.clone())?;
    let deletion = deleted
        .verify_and_advance(
            &outcome,
            &preference,
            None,
            Some(&operator_state(0, "sha256:genesis")),
        )
        .expect_err("deleted journal must project an anchored rollback");
    assert!(format!("{deletion:#}").contains("rolled back"));

    let divergent = ExternalMonotonicAnchor::open(config)?;
    let divergence = divergent
        .verify_and_advance(
            &outcome,
            &preference,
            None,
            Some(&operator_state(6, "sha256:different")),
        )
        .expect_err("same-revision journal divergence must fail closed");
    assert!(format!("{divergence:#}").contains("diverged"));
    Ok(())
}

#[test]
fn concurrent_fault_publication_prevents_a_prechecked_operation_from_succeeding() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let config = ExternalMonotonicAnchorConfig::for_test(root.path())?;
    let anchor = Arc::new(ExternalMonotonicAnchor::open(config.clone())?);
    anchor.verify_and_advance(&state(2, "sha256:o2"), &state(3, "sha256:p3"), None, None)?;

    let (fault_checked_tx, fault_checked_rx) = mpsc::sync_channel(0);
    let (release_fault_tx, release_fault_rx) = mpsc::sync_channel(0);
    let faulting_anchor = Arc::clone(&anchor);
    let faulting = std::thread::spawn(move || {
        faulting_anchor.verify_and_advance_with_after_health_check(
            &state(1, "sha256:o1"),
            &state(3, "sha256:p3"),
            None,
            None,
            || {
                fault_checked_tx
                    .send(())
                    .expect("signal faulting health check");
                release_fault_rx.recv().expect("release faulting operation");
            },
        )
    });
    fault_checked_rx.recv().context("faulting health check")?;

    let (healthy_started_tx, healthy_started_rx) = mpsc::sync_channel(0);
    let (healthy_done_tx, healthy_done_rx) = mpsc::sync_channel(1);
    let healthy_anchor = Arc::clone(&anchor);
    let healthy = std::thread::spawn(move || {
        healthy_started_tx
            .send(())
            .expect("signal healthy operation start");
        let result = healthy_anchor
            .verify_and_advance(&state(4, "sha256:o4"), &state(3, "sha256:p3"), None, None)
            .map_err(|error| format!("{error:#}"));
        healthy_done_tx
            .send(result)
            .expect("signal healthy operation completion");
    });
    healthy_started_rx
        .recv()
        .context("healthy operation start")?;
    assert!(matches!(
        healthy_done_rx.recv_timeout(Duration::from_millis(200)),
        Err(mpsc::RecvTimeoutError::Timeout)
    ));

    release_fault_tx
        .send(())
        .context("release faulting operation")?;
    assert!(
        faulting
            .join()
            .map_err(|_| anyhow::anyhow!("faulting anchor thread panicked"))?
            .is_err()
    );
    let healthy_error = healthy_done_rx
        .recv_timeout(Duration::from_secs(5))
        .context("healthy operation completion")?
        .expect_err("operation queued behind a published fault must fail");
    assert!(healthy_error.contains("external monotonic anchor is faulted"));
    healthy
        .join()
        .map_err(|_| anyhow::anyhow!("healthy anchor thread panicked"))?;

    assert_eq!(fs::read_to_string(&config.path)?.lines().count(), 1);
    assert!(anchor.ensure_healthy().is_err());
    Ok(())
}

#[test]
fn effect_lease_excludes_fault_checks_until_terminal_state_is_anchored() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let config = ExternalMonotonicAnchorConfig::for_test(root.path())?;
    let anchor = Arc::new(ExternalMonotonicAnchor::open(config.clone())?);
    let initial_outcome = state(2, "sha256:o2");
    let preference = state(3, "sha256:p3");
    anchor.verify_and_advance(&initial_outcome, &preference, None, None)?;
    let lease = anchor.begin_effect_lease(&initial_outcome, &preference, None, None)?;

    let (started_tx, started_rx) = mpsc::sync_channel(0);
    let (finished_tx, finished_rx) = mpsc::sync_channel(1);
    let queued_anchor = Arc::clone(&anchor);
    let queued_outcome = initial_outcome.clone();
    let queued_preference = preference.clone();
    let queued = std::thread::spawn(move || {
        started_tx.send(()).expect("queued anchor start");
        let result = queued_anchor
            .verify_and_advance(&queued_outcome, &queued_preference, None, None)
            .map_err(|error| format!("{error:#}"));
        finished_tx.send(result).expect("queued anchor finish");
    });
    started_rx.recv().context("queued anchor start")?;
    assert!(matches!(
        finished_rx.recv_timeout(Duration::from_millis(200)),
        Err(mpsc::RecvTimeoutError::Timeout)
    ));

    lease.finalize(&state(4, "sha256:o4"), &preference, None, None)?;
    let queued_error = finished_rx
        .recv_timeout(Duration::from_secs(5))
        .context("queued anchor result")?
        .expect_err("stale check must run only after terminal anchor publication");
    assert!(queued_error.contains("rolled back"));
    queued
        .join()
        .map_err(|_| anyhow::anyhow!("queued anchor thread panicked"))?;
    assert_eq!(fs::read_to_string(&config.path)?.lines().count(), 3);
    Ok(())
}

#[test]
fn dropped_effect_lease_faults_anchor_before_waiters_resume() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let config = ExternalMonotonicAnchorConfig::for_test(root.path())?;
    let anchor = Arc::new(ExternalMonotonicAnchor::open(config)?);
    let outcome = state(2, "sha256:o2");
    let preference = state(3, "sha256:p3");
    anchor.verify_and_advance(&outcome, &preference, None, None)?;
    let lease = anchor.begin_effect_lease(&outcome, &preference, None, None)?;
    drop(lease);
    let error = anchor
        .verify_and_advance(&outcome, &preference, None, None)
        .expect_err("abandoned effect lease must permanently fail closed");
    assert!(format!("{error:#}").contains("dropped before finalization"));
    Ok(())
}

#[test]
fn external_effect_lease_subprocess_probe() -> Result<()> {
    let Some(mode) = env::var_os("HEPTA_TEST_EXTERNAL_EFFECT_LEASE_MODE") else {
        return Ok(());
    };
    let config = ExternalMonotonicAnchorConfig {
        path: PathBuf::from(
            env::var_os("HEPTA_TEST_EXTERNAL_EFFECT_LEASE_ANCHOR")
                .context("subprocess anchor path")?,
        ),
        key_file: PathBuf::from(
            env::var_os("HEPTA_TEST_EXTERNAL_EFFECT_LEASE_KEY").context("subprocess key path")?,
        ),
    };
    let anchor = Arc::new(ExternalMonotonicAnchor::open(config)?);
    let outcome = state(2, "sha256:o2");
    let preference = state(3, "sha256:p3");
    match mode.to_string_lossy().as_ref() {
        "blocked" => {
            let error = match anchor.begin_effect_lease(&outcome, &preference, None, None) {
                Ok(_) => anyhow::bail!("second process entered a held effect lease"),
                Err(error) => error,
            };
            assert!(format!("{error:#}").contains("active in another process"));
            Ok(())
        }
        "crash" => {
            let _lease = anchor.begin_effect_lease(&outcome, &preference, None, None)?;
            std::process::exit(0);
        }
        other => anyhow::bail!("unknown external effect lease subprocess mode: {other}"),
    }
}

fn run_effect_lease_subprocess(config: &ExternalMonotonicAnchorConfig, mode: &str) -> Result<()> {
    let output = Command::new(env::current_exe()?)
        .arg("external_effect_lease_subprocess_probe")
        .arg("--nocapture")
        .env("HEPTA_TEST_EXTERNAL_EFFECT_LEASE_MODE", mode)
        .env("HEPTA_TEST_EXTERNAL_EFFECT_LEASE_ANCHOR", &config.path)
        .env("HEPTA_TEST_EXTERNAL_EFFECT_LEASE_KEY", &config.key_file)
        .output()
        .context("run external effect lease subprocess")?;
    if !output.status.success() {
        anyhow::bail!(
            "external effect lease subprocess failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn effect_lease_is_exclusive_across_processes() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let config = ExternalMonotonicAnchorConfig::for_test(root.path())?;
    let outcome = state(2, "sha256:o2");
    let preference = state(3, "sha256:p3");
    let anchor = Arc::new(ExternalMonotonicAnchor::open(config.clone())?);
    anchor.verify_and_advance(&outcome, &preference, None, None)?;
    let lease = anchor.begin_effect_lease(&outcome, &preference, None, None)?;

    run_effect_lease_subprocess(&config, "blocked")?;
    lease.finalize(&outcome, &preference, None, None)?;
    ExternalMonotonicAnchor::open(config)?.verify_and_advance(&outcome, &preference, None, None)
}

#[test]
fn retryable_effect_lock_contention_does_not_fault_the_anchor() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let config = ExternalMonotonicAnchorConfig::for_test(root.path())?;
    let outcome = state(2, "sha256:o2");
    let preference = state(3, "sha256:p3");
    let anchor = ExternalMonotonicAnchor::open(config.clone())?;
    anchor.verify_and_advance(&outcome, &preference, None, None)?;

    let held_lock = open_effect_lock_file(&effect_lock_path(&config.path)?)?;
    try_lock_effect(&held_lock)?;
    let error = anchor
        .verify_and_advance(&outcome, &preference, None, None)
        .expect_err("held cross-process lock must make this operation retryable");
    assert!(
        error
            .downcast_ref::<ExternalMonotonicAnchorBusy>()
            .is_some()
    );
    anchor.ensure_healthy()?;
    drop(held_lock);

    anchor.verify_and_advance(&outcome, &preference, None, None)
}

#[test]
fn state_is_sampled_only_after_cross_process_exclusion_without_false_rollback() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let config = ExternalMonotonicAnchorConfig::for_test(root.path())?;
    let initial_outcome = state(2, "sha256:o2");
    let preference = state(3, "sha256:p3");
    ExternalMonotonicAnchor::open(config.clone())?.verify_and_advance(
        &initial_outcome,
        &preference,
        None,
        None,
    )?;

    let first = Arc::new(ExternalMonotonicAnchor::open(config.clone())?);
    let second = Arc::new(ExternalMonotonicAnchor::open(config.clone())?);
    let (provider_entered_tx, provider_entered_rx) = mpsc::sync_channel(0);
    let (release_provider_tx, release_provider_rx) = mpsc::sync_channel(0);
    let first_outcome = initial_outcome.clone();
    let first_preference = preference.clone();
    let first_thread = std::thread::spawn(move || {
        first.verify_and_advance_with(|| {
            provider_entered_tx
                .send(())
                .expect("signal state provider entered");
            release_provider_rx.recv().expect("release state provider");
            Ok(DurableAnchorStateSnapshot {
                outcome: first_outcome,
                preference: first_preference,
                telegram: None,
                operator: None,
            })
        })
    });
    provider_entered_rx
        .recv()
        .context("state provider did not enter")?;

    let competing_provider_called = AtomicBool::new(false);
    let error = second
        .verify_and_advance_with(|| {
            competing_provider_called.store(true, Ordering::SeqCst);
            Ok(DurableAnchorStateSnapshot {
                outcome: state(4, "sha256:o4"),
                preference: preference.clone(),
                telegram: None,
                operator: None,
            })
        })
        .expect_err("competing process must not sample state before exclusion");
    assert!(
        error
            .downcast_ref::<ExternalMonotonicAnchorBusy>()
            .is_some()
    );
    assert!(!competing_provider_called.load(Ordering::SeqCst));
    second.ensure_healthy()?;

    release_provider_tx
        .send(())
        .context("release first state provider")?;
    first_thread
        .join()
        .map_err(|_| anyhow::anyhow!("first anchor thread panicked"))??;

    let advanced = state(4, "sha256:o4");
    second.verify_and_advance_with(|| {
        Ok(DurableAnchorStateSnapshot {
            outcome: advanced.clone(),
            preference: preference.clone(),
            telegram: None,
            operator: None,
        })
    })?;
    ExternalMonotonicAnchor::open(config)?.verify_and_advance(&advanced, &preference, None, None)
}

#[test]
fn crashed_effect_lease_remains_authenticated_and_in_doubt_after_restart() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let config = ExternalMonotonicAnchorConfig::for_test(root.path())?;
    let outcome = state(2, "sha256:o2");
    let preference = state(3, "sha256:p3");
    ExternalMonotonicAnchor::open(config.clone())?.verify_and_advance(
        &outcome,
        &preference,
        None,
        None,
    )?;

    run_effect_lease_subprocess(&config, "crash")?;
    let restarted = Arc::new(ExternalMonotonicAnchor::open(config.clone())?);
    let error = match restarted.begin_effect_lease(&outcome, &preference, None, None) {
        Ok(_) => anyhow::bail!("crashed effect lease re-entered after OS lock release"),
        Err(error) => error,
    };
    assert!(format!("{error:#}").contains("in-doubt effect lease"));
    assert_eq!(fs::read_to_string(&config.path)?.lines().count(), 2);

    let mut bytes = fs::read(&config.path)?;
    let marker = b"\"effect_lease_id\":\"";
    let marker_start = bytes
        .windows(marker.len())
        .position(|window| window == marker)
        .context("authenticated pending effect lease marker")?
        + marker.len();
    bytes[marker_start] = if bytes[marker_start] == b'a' {
        b'b'
    } else {
        b'a'
    };
    fs::write(&config.path, bytes)?;
    let tampered = ExternalMonotonicAnchor::open(config)?;
    let error = tampered
        .verify_and_advance(&outcome, &preference, None, None)
        .expect_err("tampered pending effect lease marker must fail closed");
    assert!(format!("{error:#}").contains("MAC is invalid"));
    Ok(())
}

#[test]
fn effect_lease_refuses_before_terminal_record_capacity_is_exhausted() -> Result<()> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let config = ExternalMonotonicAnchorConfig::for_test(root.path())?;
    let outcome = state(2, "sha256:o2");
    let preference = state(3, "sha256:p3");
    let key = std::array::from_fn(|index| 0x60 + index as u8);
    let mut previous_entry_hash = GENESIS_PREVIOUS_HASH.to_owned();
    let mut bytes = Vec::new();
    for sequence in 1..MAX_ANCHOR_RECORDS as u64 {
        let mut entry = AnchorEntry {
            schema: ANCHOR_SCHEMA.into(),
            sequence,
            previous_entry_hash,
            outcome_generation: outcome.generation(),
            outcome_state_hash: outcome.state_hash().to_owned(),
            preference_generation: preference.generation(),
            preference_state_hash: preference.state_hash().to_owned(),
            telegram_generation: None,
            telegram_state_hash: None,
            operator_generation: None,
            operator_state_hash: None,
            effect_lease_state: None,
            effect_lease_id: None,
            effect_lease_source_hash: None,
            mac: String::new(),
        };
        entry.mac = entry_mac(&entry, &key)?;
        previous_entry_hash = entry_hash(&entry);
        serde_json::to_writer(&mut bytes, &entry)?;
        bytes.push(b'\n');
    }
    fs::write(&config.path, bytes)?;
    fs::set_permissions(&config.path, fs::Permissions::from_mode(0o600))?;

    let anchor = Arc::new(ExternalMonotonicAnchor::open(config.clone())?);
    let error = match anchor.begin_effect_lease(&outcome, &preference, None, None) {
        Ok(_) => anyhow::bail!("effect entered without terminal record capacity"),
        Err(error) => error,
    };
    assert!(format!("{error:#}").contains("lacks two reserved records"));
    assert_eq!(
        fs::read_to_string(config.path)?.lines().count(),
        MAX_ANCHOR_RECORDS - 1
    );
    Ok(())
}

#[test]
fn anchor_configuration_requires_a_complete_absolute_pair() {
    let missing = ExternalMonotonicAnchorConfig::from_lookup(|name| {
        (name == MONOTONIC_ANCHOR_FILE_ENV).then(|| OsString::from("/tmp/anchor"))
    });
    assert!(missing.is_err());
    let relative = ExternalMonotonicAnchorConfig::from_lookup(|name| match name {
        MONOTONIC_ANCHOR_FILE_ENV => Some(OsString::from("anchor")),
        MONOTONIC_ANCHOR_KEY_FILE_ENV => Some(OsString::from("/tmp/key")),
        _ => None,
    });
    assert!(relative.is_err());
}

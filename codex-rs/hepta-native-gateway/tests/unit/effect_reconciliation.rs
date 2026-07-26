use super::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn authority() -> Result<EffectReconciliationAuthority> {
    let root = tempfile::tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let key_file = root.path().join("reconciliation.key");
    fs::write(
        &key_file,
        b"707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f",
    )?;
    fs::set_permissions(&key_file, fs::Permissions::from_mode(0o600))?;
    EffectReconciliationAuthority::open(EffectReconciliationConfig { key_file })
}

#[test]
fn reconciliation_proof_binds_route_session_attempt_plan_and_decision() -> Result<()> {
    let authority = authority()?;
    let session_hash = format!("sha256:{}", "2".repeat(64));
    let effect_hash = format!("sha256:{}", "3".repeat(64));
    let request_hash = "4".repeat(64);
    let inspect = authority.proof(
        "POST",
        EFFECT_RECONCILIATION_INSPECT_ENDPOINT,
        &session_hash,
        "attempt:one",
        &effect_hash,
        &request_hash,
        None,
    )?;
    let resolve = authority.proof(
        "POST",
        EFFECT_RECONCILIATION_RESOLVE_ENDPOINT,
        &session_hash,
        "attempt:one",
        &effect_hash,
        &request_hash,
        Some(ReconciliationDecision::RetryTerminalReceiptOnly),
    )?;
    assert_ne!(inspect, resolve);
    assert_ne!(
        inspect,
        authority.proof(
            "POST",
            EFFECT_RECONCILIATION_INSPECT_ENDPOINT,
            &session_hash,
            "attempt:two",
            &effect_hash,
            &request_hash,
            None,
        )?
    );
    assert_ne!(
        inspect,
        authority.proof(
            "POST",
            EFFECT_RECONCILIATION_INSPECT_ENDPOINT,
            &session_hash,
            "attempt:one",
            &effect_hash,
            &"5".repeat(64),
            None,
        )?,
        "reconciliation authority must bind the current transport request"
    );
    Ok(())
}

#[test]
fn reconciliation_configuration_is_optional_but_never_partial_or_relative() {
    assert!(
        EffectReconciliationConfig::from_lookup(|_| None)
            .expect("disabled config")
            .is_none()
    );
    assert!(
        EffectReconciliationConfig::from_lookup(|_| Some(OsString::from("relative.key"))).is_err()
    );
}

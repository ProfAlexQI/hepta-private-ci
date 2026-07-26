use super::*;

#[test]
fn live_preference_composition_fails_closed_when_required_env_is_missing() {
    let error = NativePreferenceIngressConfig::from_lookup(|_| None)
        .expect_err("live ingress must not silently disable itself");
    assert!(
        error
            .to_string()
            .contains("HEPTA_PREFERENCE_DATABASE is required for --serve-ui")
    );
}

#[test]
fn live_preference_composition_rejects_relative_paths_and_unknown_mode() {
    let relative = NativePreferenceIngressConfig::from_lookup(|name| match name {
        PREFERENCE_DATABASE_ENV => Some(OsString::from("relative.sqlite3")),
        PREFERENCE_INTEGRITY_KEY_FILE_ENV => Some(OsString::from("/tmp/integrity.key")),
        PREFERENCE_AUTH_KEY_FILE_ENV => Some(OsString::from("/tmp/auth.key")),
        _ => None,
    })
    .expect_err("relative database must fail");
    assert!(relative.to_string().contains("must be an absolute path"));

    let mode = NativePreferenceIngressConfig::from_lookup(|name| match name {
        PREFERENCE_DATABASE_ENV => Some(OsString::from("/tmp/preference.sqlite3")),
        PREFERENCE_INTEGRITY_KEY_FILE_ENV => Some(OsString::from("/tmp/integrity.key")),
        PREFERENCE_AUTH_KEY_FILE_ENV => Some(OsString::from("/tmp/auth.key")),
        PREFERENCE_STORE_MODE_ENV => Some(OsString::from("unsafe-default")),
        _ => None,
    })
    .expect_err("unknown mode must fail");
    assert!(mode.to_string().contains(PREFERENCE_STORE_MODE_ENV));
}

#[cfg(unix)]
#[test]
fn authenticated_preference_attachment_hydrates_only_the_exact_reopened_state() -> Result<()> {
    let root = tempfile::tempdir()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o700))?;
    }
    let bootstrap = NativePreferenceIngressConfig::bootstrap_for_test(root.path())?;
    let database = bootstrap.database.clone();
    let integrity_key_file = bootstrap.integrity_key_file.clone();
    let authentication_key_file = bootstrap.authentication_key_file.clone();
    let ingress = NativePreferenceIngress::open(NativePreferenceIngress::prepare(bootstrap)?)?;
    let session_binding_hash = format!("sha256:{}", "1".repeat(64));
    let request = serde_json::json!({
        "transition_id": "transition:restart-hydration",
        "evidence_id": "evidence:restart-hydration",
        "signal": "accepted",
        "receipt": {
            "id": "receipt:restart-hydration",
            "hash": "sha256:receipt-restart-hydration"
        },
        "session_binding_hash": session_binding_hash,
        "subject": "subject:restart-hydration",
        "preference": "preference:restart-hydration",
        "target": {
            "kind": "capability",
            "capability_id": "tool:restart-hydration",
            "capability_revision": 1,
            "manifest_hash": "sha256:manifest-restart-hydration",
            "catalog_revision": 1,
            "catalog_hash": "sha256:catalog-restart-hydration"
        }
    });
    let key = std::array::from_fn(|index| 0x40 + u8::try_from(index).expect("key index"));
    let envelope = authenticated_challenge_envelope_for_test(&request, key)?;
    let challenge = ingress
        .route_http(
            "POST",
            PREFERENCE_CHALLENGE_ENDPOINT,
            Some(&serde_json::to_string(&envelope)?),
            &"2".repeat(64),
            &session_binding_hash,
        )
        .context("challenge response")?;
    assert_eq!(challenge.status, "200 OK");
    let challenge: serde_json::Value = serde_json::from_str(&challenge.body)?;
    let challenge_hash = challenge["commit"]["challenge_hash"]
        .as_str()
        .context("challenge hash")?;
    let proof = hepta_intelligence::sign_preference_ingress_challenge(
        &PreferenceIngressAuthenticationKey::from_bytes(key),
        &ContentHash::new(challenge_hash),
    )?
    .to_hex();
    let commit = serde_json::json!({
        "commit": challenge["commit"].clone(),
        "proof": proof,
    });
    let committed = ingress
        .route_http(
            "POST",
            PREFERENCE_COMMIT_ENDPOINT,
            Some(&serde_json::to_string(&commit)?),
            &"3".repeat(64),
            &session_binding_hash,
        )
        .context("commit response")?;
    assert_eq!(committed.status, "200 OK");
    drop(ingress);

    let reopened = NativePreferenceIngress::open(NativePreferenceIngress::prepare(
        NativePreferenceIngressConfig {
            database,
            integrity_key_file,
            authentication_key_file,
            mode: PreferenceStoreMode::OpenExisting,
        },
    )?)?;
    let hydrated = reopened
        .hydrate_runtime_context(&session_binding_hash)?
        .context("hydrated preference context")?;
    assert_eq!(hydrated.revision().get(), 1);
    assert!(
        reopened
            .hydrate_runtime_context(&format!("sha256:{}", "9".repeat(64)))
            .is_err()
    );
    Ok(())
}

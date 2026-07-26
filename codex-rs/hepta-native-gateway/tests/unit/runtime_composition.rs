use std::fs;
use std::os::unix::fs::PermissionsExt;

use tempfile::tempdir;

use super::*;

fn write_key(path: &Path, encoded: &[u8]) {
    fs::write(path, encoded).expect("write key");
    fs::set_permissions(path, fs::Permissions::from_mode(PRIVATE_FILE_MODE))
        .expect("set key permissions");
}

#[test]
fn authorization_request_binding_excludes_only_proof_and_canonicalizes_object_order() {
    let first = proof_excluded_authorization_body(
        crate::operator_mutation::OPERATOR_MUTATION_PLAN_ENDPOINT,
        Some(r#"{"note":"n","proof":"aaa","mutation_id":"m"}"#),
    )
    .expect("first canonical body");
    let second = proof_excluded_authorization_body(
        crate::operator_mutation::OPERATOR_MUTATION_PLAN_ENDPOINT,
        Some(r#"{"proof":"bbb","mutation_id":"m","note":"n"}"#),
    )
    .expect("second canonical body");
    let changed = proof_excluded_authorization_body(
        crate::operator_mutation::OPERATOR_MUTATION_PLAN_ENDPOINT,
        Some(r#"{"proof":"aaa","mutation_id":"m","note":"changed"}"#),
    )
    .expect("changed canonical body");
    assert_eq!(first, second);
    assert_ne!(first, changed);
    assert!(
        proof_excluded_authorization_body(
            crate::operator_mutation::OPERATOR_MUTATION_PLAN_ENDPOINT,
            Some(r#"{"mutation_id":"m","note":"n"}"#),
        )
        .is_err()
    );
}

#[test]
fn live_mutation_modes_require_a_configured_anchor_while_default_off_remains_usable() {
    assert!(require_live_mutation_anchor(false, false, false).is_ok());
    assert!(require_live_mutation_anchor(true, false, true).is_ok());
    assert!(require_live_mutation_anchor(false, true, true).is_ok());
    assert!(require_live_mutation_anchor(true, true, true).is_ok());

    let operator = require_live_mutation_anchor(true, false, false)
        .expect_err("operator mutation without anchor must fail closed");
    assert!(format!("{operator:#}").contains("operator mutation"));
    let telegram = require_live_mutation_anchor(false, true, false)
        .expect_err("Telegram pipeline without anchor must fail closed");
    assert!(format!("{telegram:#}").contains("Telegram pipeline"));
    let both = require_live_mutation_anchor(true, true, false)
        .expect_err("combined live mutation modes without anchor must fail closed");
    assert!(format!("{both:#}").contains("operator mutation and Telegram pipeline"));
}

#[test]
fn durable_mutation_surfaces_without_anchor_fail_before_state_change() -> Result<()> {
    let root = tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let mut runtime = NativeGatewayRuntime::bootstrap_for_test(root.path())?;
    runtime.effect_reconciliation = Some(EffectReconciliationAuthority::open(
        EffectReconciliationConfig::for_test(root.path())?,
    )?);
    let before = runtime.durable_anchor_states()?;

    let read_canary_error = runtime
        .execute_runtime_kernel_canary(&"0".repeat(64))
        .expect_err("durable read canary without anchor must fail closed");
    assert!(format!("{read_canary_error:#}").contains("external monotonic anchor is required"));

    let mutation_error = runtime
        .execute_runtime_mutation_canary(&"a".repeat(64), &"b".repeat(64))
        .expect_err("mutation canary without anchor must fail closed");
    assert!(format!("{mutation_error:#}").contains("external monotonic anchor is required"));

    let preference = runtime
        .route_preference_ingress(
            "POST",
            crate::preference_ingress::PREFERENCE_COMMIT_ENDPOINT,
            Some("{}"),
            &"c".repeat(64),
        )
        .expect("preference commit route");
    assert_eq!(preference.status, "503 Service Unavailable");
    assert!(
        preference
            .body
            .contains("trusted_preference_ingress.monotonic_anchor_failed")
    );

    let reconciliation = runtime
        .route_effect_reconciliation(
            "POST",
            crate::effect_reconciliation::EFFECT_RECONCILIATION_RESOLVE_ENDPOINT,
            Some("{}"),
            &"d".repeat(64),
        )
        .expect("effect reconciliation route");
    assert_eq!(reconciliation.status, "503 Service Unavailable");
    assert!(!reconciliation.outcome_state_changed);
    assert!(
        reconciliation
            .body
            .contains("operator_effect_reconciliation.monotonic_anchor_failed")
    );

    let after = runtime.durable_anchor_states()?;
    assert_eq!(before.outcome, after.outcome);
    assert_eq!(before.preference, after.preference);
    assert_eq!(before.telegram, after.telegram);
    assert_eq!(before.operator, after.operator);
    Ok(())
}

#[test]
fn invalid_preference_hmac_does_not_consume_anchor_or_change_domain_state() -> Result<()> {
    let root = tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let runtime = NativeGatewayRuntime::bootstrap_with_anchor_for_test(root.path())?;
    let (_, session_binding_hash) = runtime.preference_session_binding()?;
    let request = serde_json::json!({
        "transition_id": "transition:invalid-anchor-proof",
        "evidence_id": "evidence:invalid-anchor-proof",
        "signal": "accepted",
        "receipt": {
            "id": "receipt:invalid-anchor-proof",
            "hash": "sha256:receipt-invalid-anchor-proof"
        },
        "session_binding_hash": session_binding_hash,
        "subject": "subject:invalid-anchor-proof",
        "preference": "preference:invalid-anchor-proof",
        "target": {
            "kind": "capability",
            "capability_id": "tool:invalid-anchor-proof",
            "capability_revision": 1,
            "manifest_hash": "sha256:manifest-invalid-anchor-proof",
            "catalog_revision": 1,
            "catalog_hash": "sha256:catalog-invalid-anchor-proof"
        }
    });
    let key = std::array::from_fn(|index| 0x40 + u8::try_from(index).expect("key index"));
    let envelope =
        crate::preference_ingress::authenticated_challenge_envelope_for_test(&request, key)?;
    let challenge = runtime
        .route_preference_ingress(
            "POST",
            crate::preference_ingress::PREFERENCE_CHALLENGE_ENDPOINT,
            Some(&serde_json::to_string(&envelope)?),
            &"2".repeat(64),
        )
        .context("challenge response")?;
    assert_eq!(challenge.status, "200 OK");
    let challenge: serde_json::Value = serde_json::from_str(&challenge.body)?;
    let commit = serde_json::json!({
        "commit": challenge["commit"].clone(),
        "proof": "0".repeat(64),
    });
    let anchor_path = root.path().join("monotonic.anchor");
    let anchor_before = fs::read(&anchor_path)?;
    let states_before = runtime.durable_anchor_states()?;

    let denied = runtime
        .route_preference_ingress(
            "POST",
            crate::preference_ingress::PREFERENCE_COMMIT_ENDPOINT,
            Some(&serde_json::to_string(&commit)?),
            &"3".repeat(64),
        )
        .context("preference denial response")?;
    assert_eq!(denied.status, "403 Forbidden");
    assert!(denied.body.contains("authentication_denied"));
    assert_eq!(fs::read(&anchor_path)?, anchor_before);
    let states_after = runtime.durable_anchor_states()?;
    assert_eq!(states_before.outcome, states_after.outcome);
    assert_eq!(states_before.preference, states_after.preference);
    assert_eq!(states_before.telegram, states_after.telegram);
    assert_eq!(states_before.operator, states_after.operator);
    Ok(())
}

#[test]
fn effect_reconciliation_uses_the_operator_runtime_session_binding() -> Result<()> {
    let root = tempdir()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))?;
    let mut runtime = NativeGatewayRuntime::bootstrap_for_test(root.path())?;
    runtime.effect_reconciliation = Some(EffectReconciliationAuthority::open(
        EffectReconciliationConfig::for_test(root.path())?,
    )?);
    let operator_session = runtime.operator_runtime_session_binding()?;
    let (_, preference_session) = runtime.preference_session_binding()?;
    assert_ne!(operator_session, preference_session);
    let attempt_id = "missing-attempt";
    let effect_plan_hash = "e".repeat(64);
    let request_binding_hash = "f".repeat(64);
    let authority = runtime
        .effect_reconciliation
        .as_ref()
        .context("effect reconciliation authority")?;
    let proof = authority.inspect_proof_for_test(
        &operator_session,
        attempt_id,
        &effect_plan_hash,
        &request_binding_hash,
    )?;
    let body = serde_json::json!({
        "attempt_id": attempt_id,
        "effect_plan_hash": effect_plan_hash,
        "session_binding_hash": operator_session,
        "proof": proof,
    })
    .to_string();
    let response = runtime
        .route_effect_reconciliation(
            "POST",
            crate::effect_reconciliation::EFFECT_RECONCILIATION_INSPECT_ENDPOINT,
            Some(&body),
            &request_binding_hash,
        )
        .context("effect reconciliation response")?;
    assert_eq!(response.status, "404 Not Found");
    assert!(
        response
            .body
            .contains("operator_effect_reconciliation.pending_attempt_not_found")
    );
    Ok(())
}

#[test]
fn keyed_runtime_bootstraps_then_opens_existing_database() {
    let root = tempdir().expect("tempdir");
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))
        .expect("set root permissions");
    let key_path = root.path().join("runtime.key");
    let database_path = root.path().join("outcomes.sqlite3");
    let state_path = root.path().join("runtime-state.json");
    write_key(
        &key_path,
        b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f\n",
    );

    let preference_root = root.path().join("preference");
    fs::create_dir(&preference_root).expect("preference root");
    fs::set_permissions(&preference_root, fs::Permissions::from_mode(0o700))
        .expect("set preference root permissions");
    let bootstrap = NativeGatewayRuntime::open(
        RuntimeCompositionConfig {
            outcome_database: database_path.clone(),
            state_database: state_path.clone(),
            integrity_key_file: key_path.clone(),
            outcome_mode: RuntimeOutcomeMode::BootstrapNew,
        },
        NativePreferenceIngressConfig::bootstrap_for_test(&preference_root)
            .expect("preference config"),
    )
    .expect("bootstrap keyed runtime");
    bootstrap.validate_readiness().expect("bootstrap readiness");
    let read = bootstrap
        .preflight_request("GET", "/api/health", None)
        .expect("read-only preflight");
    assert_eq!(
        read.disposition,
        RuntimeRequestDisposition::ReadOnlyDispatch
    );
    assert!(!read.mutation_authorized);
    assert!(!read.durable_intent_recorded);
    assert!(!read.provider_effect_ack_recorded);
    assert!(!read.terminal_receipt_recorded);
    let plan = bootstrap
        .preflight_request("POST", "/api/tasks/publish", Some(r#"{"dry_run":true}"#))
        .expect("plan-only preflight");
    assert_eq!(
        plan.disposition,
        RuntimeRequestDisposition::PlanOnlyQuarantine
    );
    assert_ne!(read.request_binding_hash, plan.request_binding_hash);
    assert!(!plan.mutation_authorized);
    let configured = plan.native_post_gate_inputs(true, true);
    assert!(!configured.real_handler_enabled);
    assert!(!configured.operator_approval_enabled);
    assert!(
        bootstrap
            .preflight_request("DELETE", "/api/tasks/1", None)
            .expect_err("mutation method must fail closed")
            .to_string()
            .contains("unsupported HTTP method")
    );
    let telegram = bootstrap
        .preflight_telegram_drain(Some(42))
        .expect("telegram preflight");
    assert!(!telegram.request_binding_hash.is_empty());
    assert_eq!(
        telegram
            .require_live_pipeline_authority()
            .expect_err("telegram live pipeline must remain quarantined")
            .to_string(),
        "telegram_runtime_admission.exact_authority_unavailable"
    );
    let challenge = bootstrap
        .operator_authority_challenge()
        .expect("operator authority challenge");
    assert_eq!(
        challenge.schema,
        "hepta.native.operator-authority-challenge.v1"
    );
    assert_eq!(challenge.session_binding_hash.len(), 64);
    assert!(
        challenge
            .session_binding_hash
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    );
    assert!(
        challenge
            .preference_session_binding_hash
            .strip_prefix("sha256:")
            .is_some_and(|digest| {
                digest.len() == 64
                    && digest
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
            })
    );
    assert_eq!(
        challenge.effect_reconciliation_resolve_endpoint,
        crate::effect_reconciliation::EFFECT_RECONCILIATION_RESOLVE_ENDPOINT
    );
    assert_eq!(challenge.telegram_cursor, None);
    assert!(!challenge.telegram_pipeline_enabled);
    assert_eq!(challenge.telegram_session_binding_hash, None);
    assert_eq!(challenge.telegram_execution_identity_hash, None);
    assert!(!challenge.secret_material_returned);
    assert!(!challenge.external_effect_performed);
    drop(bootstrap);

    let opened = NativeGatewayRuntime::open(
        RuntimeCompositionConfig {
            outcome_database: database_path,
            state_database: state_path,
            integrity_key_file: key_path,
            outcome_mode: RuntimeOutcomeMode::OpenExisting,
        },
        NativePreferenceIngressConfig {
            database: preference_root.join("preferences.sqlite3"),
            integrity_key_file: preference_root.join("preference-integrity.key"),
            authentication_key_file: preference_root.join("preference-authentication.key"),
            mode: crate::preference_ingress::PreferenceStoreMode::OpenExisting,
        },
    )
    .expect("open keyed runtime");
    assert_eq!(opened.outcome_mode(), OPEN_EXISTING_MODE);
    opened.validate_readiness().expect("open readiness");
}

#[test]
fn keyed_runtime_rejects_non_private_key_file() {
    let root = tempdir().expect("tempdir");
    let key_path = root.path().join("runtime.key");
    write_key(
        &key_path,
        b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
    );
    fs::set_permissions(&key_path, fs::Permissions::from_mode(0o644))
        .expect("relax key permissions");

    let error = read_integrity_key(&key_path).expect_err("unsafe key must fail");
    assert!(
        error
            .to_string()
            .contains("integrity key must have mode 0o600")
    );
}

#[test]
fn keyed_runtime_rejects_noncanonical_key_encoding() {
    let root = tempdir().expect("tempdir");
    let key_path = root.path().join("runtime.key");
    write_key(
        &key_path,
        b"000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F",
    );
    let error = read_integrity_key(&key_path).expect_err("uppercase key must fail");
    assert!(error.to_string().contains("canonical lowercase hex"));
}

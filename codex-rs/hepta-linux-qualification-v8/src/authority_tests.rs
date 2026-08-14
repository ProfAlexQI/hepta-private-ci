use pretty_assertions::assert_eq;

use super::*;
use crate::CANDIDATE_HEAD;
use crate::CANDIDATE_TREE;
use crate::test_only_trust_binding_v8;

#[test]
fn exact_install_run_and_break_glass_authorities_verify() {
    let cases = [
        (install_scope(), 'a', AuthorityScopeKindV8::Install),
        (run_scope(), 'b', AuthorityScopeKindV8::OneShotRun),
        (break_glass_scope(), 'c', AuthorityScopeKindV8::BreakGlass),
        (install_v2_scope(), 'd', AuthorityScopeKindV8::InstallV2),
        (run_v2_scope(), 'e', AuthorityScopeKindV8::OneShotRunV2),
    ];

    for (scope, nonce, expected_kind) in cases {
        let signed = signed_authority(scope, nonce);
        let observation = observation(&signed);
        let mut replay = AuthorityReplayGuardV8::default();
        let verified = verify_signed_authority_with_observation_for_test_v8(
            &signed,
            &observation,
            1_100,
            &mut replay,
        )
        .expect("exact authority verifies");
        assert_eq!(verified.authority_nonce(), digest(nonce));
        assert_eq!(verified.consumed_at_unix_seconds(), 1_100);
        assert_eq!(
            verified.detached_signature_sha256(),
            signed.detached_signature_sha256
        );
        assert_eq!(verified.namespace(), signed.challenge.namespace);
        assert_eq!(verified.scope_kind(), expected_kind);
        assert_eq!(
            verified.trust_policy_binding(),
            &test_only_trust_binding_v8(authority_trust_purpose_v8(expected_kind))
        );
        assert_eq!(
            verified.statement_sha256(),
            signed.canonical_statement_sha256
        );
    }
}

#[test]
fn validity_window_is_half_open_bounded_and_nonce_is_single_use() {
    let signed = signed_authority(run_scope(), 'a');
    let observation = observation(&signed);
    assert!(
        verify_signed_authority_with_observation_for_test_v8(
            &signed,
            &observation,
            signed.challenge.issued_at_unix_seconds - 1,
            &mut AuthorityReplayGuardV8::default(),
        )
        .is_err()
    );
    assert!(
        verify_signed_authority_with_observation_for_test_v8(
            &signed,
            &observation,
            signed.challenge.expires_at_unix_seconds,
            &mut AuthorityReplayGuardV8::default(),
        )
        .is_err()
    );

    let mut replay = AuthorityReplayGuardV8::default();
    verify_signed_authority_with_observation_for_test_v8(
        &signed,
        &observation,
        signed.challenge.issued_at_unix_seconds,
        &mut replay,
    )
    .expect("inclusive issued-at boundary");
    assert!(
        verify_signed_authority_with_observation_for_test_v8(
            &signed,
            &observation,
            signed.challenge.issued_at_unix_seconds,
            &mut replay,
        )
        .is_err()
    );

    let mut overlong = challenge(run_scope(), 'd');
    overlong.expires_at_unix_seconds =
        overlong.issued_at_unix_seconds + MAX_AUTHORITY_LIFETIME_SECONDS_V8 + 1;
    assert!(canonical_authority_statement_for_test_v8(&overlong).is_err());
}

#[test]
fn cryptographic_observation_must_bind_namespace_principal_statement_and_signature() {
    let signed = signed_authority(run_scope(), 'a');
    let exact = observation(&signed);
    let wrong = [
        CryptographicSignatureObservation::for_test_only(
            signed.detached_signature_sha256.clone(),
            signed.canonical_statement_sha256.clone(),
            SshsigTrustPurposeV8::InstallAuthority,
        ),
        CryptographicSignatureObservation {
            signed_statement_sha256: digest('e'),
            ..exact.clone()
        },
        CryptographicSignatureObservation {
            signature_sha256: digest('f'),
            ..exact.clone()
        },
    ];
    let mut replay = AuthorityReplayGuardV8::default();
    for observation in wrong {
        assert!(
            verify_signed_authority_with_observation_for_test_v8(
                &signed,
                &observation,
                1_100,
                &mut replay,
            )
            .is_err()
        );
    }
    assert!(!replay.is_consumed(&signed.challenge.authority_nonce));
    verify_signed_authority_with_observation_for_test_v8(&signed, &exact, 1_100, &mut replay)
        .expect("failed observations do not consume the nonce");
}

#[test]
fn actual_signature_bytes_are_nonempty_and_digest_bound() {
    let signed = signed_authority(run_scope(), 'a');
    let observation = observation(&signed);

    let mut empty = signed.clone();
    empty.detached_signature_bytes.clear();
    empty.detached_signature_sha256 = sha256(&empty.detached_signature_bytes);
    assert!(
        verify_signed_authority_with_observation_for_test_v8(
            &empty,
            &observation,
            1_100,
            &mut AuthorityReplayGuardV8::default(),
        )
        .is_err()
    );

    let mut tampered = signed;
    tampered.detached_signature_bytes.push(0);
    assert!(
        verify_signed_authority_with_observation_for_test_v8(
            &tampered,
            &observation,
            1_100,
            &mut AuthorityReplayGuardV8::default(),
        )
        .is_err()
    );
}

#[test]
fn run_and_break_glass_are_bound_to_exact_attempt_host_and_restore_plan() {
    let mut wrong_run_host = challenge(run_scope(), 'a');
    if let AuthorityScopeV8::OneShotRun { target_host, .. } = &mut wrong_run_host.scope {
        target_host.machine_id_sha256 = digest('1');
    }
    assert!(canonical_authority_statement_for_test_v8(&wrong_run_host).is_err());

    let mut wrong_restore = challenge(break_glass_scope(), 'b');
    if let AuthorityScopeV8::BreakGlass {
        restore_plan_sha256,
        ..
    } = &mut wrong_restore.scope
    {
        *restore_plan_sha256 = digest('2');
    }
    assert!(canonical_authority_statement_for_test_v8(&wrong_restore).is_err());

    let mut wrong_candidate = challenge(run_scope(), 'c');
    if let AuthorityScopeV8::OneShotRun { attempt, .. } = &mut wrong_candidate.scope {
        attempt.candidate_head = "0".repeat(40);
    }
    assert!(canonical_authority_statement_for_test_v8(&wrong_candidate).is_err());

    let mut root_driver = challenge(run_scope(), 'd');
    if let AuthorityScopeV8::OneShotRun { driver_peer, .. } = &mut root_driver.scope {
        driver_peer.uid = 0;
    }
    assert!(canonical_authority_statement_for_test_v8(&root_driver).is_err());

    let mut stale_break_glass = challenge(break_glass_scope(), 'e');
    if let AuthorityScopeV8::BreakGlass { recovery_state, .. } = &mut stale_break_glass.scope {
        recovery_state.journal_tip_sha256 = digest('9');
    }
    let baseline = canonical_authority_statement_for_test_v8(&challenge(break_glass_scope(), 'e'))
        .expect("baseline break-glass statement");
    assert_ne!(
        canonical_authority_statement_for_test_v8(&stale_break_glass)
            .expect("changed current state"),
        baseline
    );
}

#[test]
fn install_authority_accepts_only_the_exact_root_inventory() {
    let mut wrong_path = challenge(install_scope(), 'a');
    if let AuthorityScopeV8::Install { inventory, .. } = &mut wrong_path.scope {
        inventory.admissiond_binary.path = "/tmp/admissiond".to_string();
    }
    assert!(canonical_authority_statement_for_test_v8(&wrong_path).is_err());

    let mut wrong_owner = challenge(install_scope(), 'b');
    if let AuthorityScopeV8::Install { inventory, .. } = &mut wrong_owner.scope {
        inventory.recovery_unit.uid = 1000;
    }
    assert!(canonical_authority_statement_for_test_v8(&wrong_owner).is_err());

    let mut wrong_mode = challenge(install_scope(), 'c');
    if let AuthorityScopeV8::Install { inventory, .. } = &mut wrong_mode.scope {
        inventory.state_root.mode = 0o755;
    }
    assert!(canonical_authority_statement_for_test_v8(&wrong_mode).is_err());

    let mut value = serde_json::to_value(challenge(install_scope(), 'd')).unwrap();
    value["scope"]["activation"] = serde_json::json!("enable_and_start");
    assert!(serde_json::from_value::<AuthorityChallengeV8>(value).is_err());
}

#[test]
fn install_v2_authority_binds_the_complete_fixed_inventory_and_target_boot() {
    let signed = signed_authority(install_v2_scope(), 'a');
    let verified = verify_signed_authority_with_observation_for_test_v8(
        &signed,
        &observation(&signed),
        1_100,
        &mut AuthorityReplayGuardV8::default(),
    )
    .expect("exact install-v2 authority");
    let (install_plan_sha256, state_disposition, inventory, target_host) = verified
        .authorized_install_v2()
        .expect("install-v2 scope remains opaque and exact");
    assert_eq!(install_plan_sha256, digest('7'));
    assert_eq!(state_disposition, &InstallStateDispositionV2::FreshEmpty);
    assert_eq!(inventory, &install_v2_inventory());
    assert_eq!(target_host, &install_v2_host());

    let mut wrong_plan = challenge(install_v2_scope(), 'f');
    if let AuthorityScopeV8::InstallV2 {
        install_plan_sha256,
        ..
    } = &mut wrong_plan.scope
    {
        *install_plan_sha256 = "not-a-plan-digest".to_string();
    }
    assert!(canonical_authority_statement_for_test_v8(&wrong_plan).is_err());

    let mut wrong_ctl = challenge(install_v2_scope(), 'b');
    if let AuthorityScopeV8::InstallV2 { inventory, .. } = &mut wrong_ctl.scope {
        inventory.ctl_binary.path = "/tmp/hepta-linux-v8ctl".to_string();
    }
    assert!(canonical_authority_statement_for_test_v8(&wrong_ctl).is_err());

    let mut wrong_lock = challenge(install_v2_scope(), 'c');
    if let AuthorityScopeV8::InstallV2 { inventory, .. } = &mut wrong_lock.scope {
        inventory.state_lock.path = "/var/lib/hepta-linux-v8/runtime.lock".to_string();
    }
    assert!(canonical_authority_statement_for_test_v8(&wrong_lock).is_err());

    let mut wrong_directory = challenge(install_v2_scope(), 'd');
    if let AuthorityScopeV8::InstallV2 { inventory, .. } = &mut wrong_directory.scope {
        inventory.nonce_claims_directory.path =
            "/var/lib/hepta-linux-v8/nonce-claims-other".to_string();
    }
    assert!(canonical_authority_statement_for_test_v8(&wrong_directory).is_err());

    let mut wrong_boot = challenge(install_v2_scope(), 'e');
    if let AuthorityScopeV8::InstallV2 { target_host, .. } = &mut wrong_boot.scope {
        target_host.boot_id = "01234567-89AB-cdef-8123-456789abcdef".to_string();
    }
    assert!(canonical_authority_statement_for_test_v8(&wrong_boot).is_err());
}

#[test]
fn one_shot_v2_authority_binds_exact_service_child_containment() {
    let scope = run_v2_scope();
    let (attempt, containment, driver_peer, target_host) = match &scope {
        AuthorityScopeV8::OneShotRunV2 {
            attempt,
            containment,
            driver_peer,
            target_host,
            ..
        } => (
            attempt.clone(),
            containment.clone(),
            driver_peer.clone(),
            target_host.clone(),
        ),
        _ => unreachable!("fixture is one-shot-v2"),
    };
    let signed = signed_authority(scope, 'a');
    let verified = verify_signed_authority_with_observation_for_test_v8(
        &signed,
        &observation(&signed),
        1_100,
        &mut AuthorityReplayGuardV8::default(),
    )
    .expect("exact one-shot-v2 authority");
    let observation = VerifiedOneShotEnvironmentV2::for_test_only(
        attempt.clone(),
        containment.clone(),
        driver_peer.clone(),
        target_host.clone(),
    );
    assert!(verified.authorizes_one_shot_v2(&observation));
    let mut wrong_driver_peer = driver_peer;
    wrong_driver_peer.uid += 1;
    let wrong_peer_observation = VerifiedOneShotEnvironmentV2::for_test_only(
        attempt.clone(),
        containment.clone(),
        wrong_driver_peer,
        target_host.clone(),
    );
    assert!(!verified.authorizes_one_shot_v2(&wrong_peer_observation));

    let mut wrong_boot_host = target_host;
    wrong_boot_host.boot_id = "11111111-1111-1111-1111-111111111111".to_string();
    let wrong_boot_observation = VerifiedOneShotEnvironmentV2::for_test_only(
        attempt,
        containment,
        DriverPeerBindingV8 {
            executable_sha256: digest('2'),
            gid: 1000,
            uid: 1000,
        },
        wrong_boot_host,
    );
    assert!(!verified.authorizes_one_shot_v2(&wrong_boot_observation));

    let mut wrong_parent = challenge(run_v2_scope(), 'b');
    if let AuthorityScopeV8::OneShotRunV2 { containment, .. } = &mut wrong_parent.scope {
        containment.service_parent_absolute_path = "/hepta-vnext/linux-v8".to_string();
        containment.child_absolute_path = format!(
            "{}/{}",
            containment.service_parent_absolute_path, containment.child_relative_name
        );
    }
    assert!(canonical_authority_statement_for_test_v8(&wrong_parent).is_err());

    let mut delegated_child = challenge(run_v2_scope(), 'c');
    if let AuthorityScopeV8::OneShotRunV2 { containment, .. } = &mut delegated_child.scope {
        containment.child_delegated = true;
    }
    assert!(canonical_authority_statement_for_test_v8(&delegated_child).is_err());

    let mut wrong_leaf = challenge(run_v2_scope(), 'd');
    if let AuthorityScopeV8::OneShotRunV2 { containment, .. } = &mut wrong_leaf.scope {
        containment.child_relative_name = "hepta-v8-forged".to_string();
        containment.child_absolute_path = format!(
            "{}/{}",
            containment.service_parent_absolute_path, containment.child_relative_name
        );
    }
    assert!(canonical_authority_statement_for_test_v8(&wrong_leaf).is_err());
}

#[test]
fn canonical_statement_binds_attempt_inventory_time_and_signer() {
    let baseline = challenge(run_scope(), 'a');
    let baseline_bytes =
        canonical_authority_statement_for_test_v8(&baseline).expect("baseline statement");

    let mut attempt_changed = baseline.clone();
    if let AuthorityScopeV8::OneShotRun { attempt, .. } = &mut attempt_changed.scope {
        attempt.runner_snapshot_sha256 = digest('1');
    }
    let mut time_changed = baseline.clone();
    time_changed.expires_at_unix_seconds -= 1;
    let mut signer_changed = baseline;
    signer_changed.signer.key_fingerprint = format!("SHA256:{}", "B".repeat(43));

    for changed in [attempt_changed, time_changed] {
        assert_ne!(
            canonical_authority_statement_for_test_v8(&changed).expect("valid changed statement"),
            baseline_bytes
        );
    }
    assert!(canonical_authority_statement_for_test_v8(&signer_changed).is_err());

    let install = canonical_authority_statement_for_test_v8(&challenge(install_scope(), 'd'))
        .expect("install statement");
    let mut changed_install = challenge(install_scope(), 'd');
    if let AuthorityScopeV8::Install { inventory, .. } = &mut changed_install.scope {
        inventory.admissiond_binary.content_sha256 = digest('9');
    }
    assert_ne!(
        canonical_authority_statement_for_test_v8(&changed_install)
            .expect("changed install statement"),
        install
    );
}

#[test]
fn unpublished_production_trust_policy_fails_closed_without_consuming_nonce() {
    for (scope, nonce) in [
        (run_scope(), 'a'),
        (install_v2_scope(), 'b'),
        (run_v2_scope(), 'c'),
    ] {
        let signed = signed_authority(scope, nonce);
        let mut replay = AuthorityReplayGuardV8::default();
        assert!(verify_signed_authority_v8(&signed, 1_100, &mut replay).is_err());
        assert!(!replay.is_consumed(&signed.challenge.authority_nonce));
    }
}

#[test]
fn forbidden_operations_and_signature_verified_flags_are_not_encodable() {
    let signed = signed_authority(run_scope(), 'a');
    let mut value = serde_json::to_value(&signed).expect("serialize authority");
    let object = value.as_object_mut().expect("authority object");
    object.insert(
        "signature_verified".to_string(),
        serde_json::Value::Bool(true),
    );
    assert!(serde_json::from_value::<SignedAuthorityV8>(value).is_err());

    let run_challenge = challenge(run_scope(), 'b');
    let mut value = serde_json::to_value(&run_challenge).expect("serialize challenge");
    let scope = value
        .get_mut("scope")
        .and_then(serde_json::Value::as_object_mut)
        .expect("scope object");
    scope.insert("allow_sigkill".to_string(), serde_json::Value::Bool(true));
    scope.insert(
        "allow_ref_mutation".to_string(),
        serde_json::Value::Bool(true),
    );
    assert!(serde_json::from_value::<AuthorityChallengeV8>(value).is_err());

    let statement =
        canonical_authority_statement_for_test_v8(&run_challenge).expect("run statement");
    assert!(contains(&statement, b"SIGSTOP,SIGCONT"));
    assert!(contains(
        &statement,
        b"SIGKILL,unregister,delete,reconfigure,ref_mutation,production_mutation"
    ));

    let break_glass =
        canonical_authority_statement_for_test_v8(&challenge(break_glass_scope(), 'c'))
            .expect("break-glass statement");
    assert!(contains(&break_glass, b"abandoned"));
    assert!(contains(&break_glass, b"permanent"));
    assert!(contains(&break_glass, b"forbidden"));
}

#[test]
fn persisted_replay_claims_are_validated_and_apply_across_scopes() {
    assert!(AuthorityReplayGuardV8::from_consumed_nonces(["A".repeat(64)]).is_err());

    let nonce = digest('a');
    let mut replay =
        AuthorityReplayGuardV8::from_consumed_nonces([nonce]).expect("valid persisted claim");
    let signed = signed_authority(install_scope(), 'a');
    assert!(
        verify_signed_authority_with_observation_for_test_v8(
            &signed,
            &observation(&signed),
            1_100,
            &mut replay,
        )
        .is_err()
    );
}

fn signed_authority(scope: AuthorityScopeV8, nonce: char) -> SignedAuthorityV8 {
    let challenge = challenge(scope, nonce);
    let statement =
        canonical_authority_statement_for_test_v8(&challenge).expect("canonical challenge");
    let detached_signature_bytes =
        format!("test detached signature for nonce {nonce}").into_bytes();
    SignedAuthorityV8 {
        canonical_statement_sha256: sha256(&statement),
        challenge,
        detached_signature_sha256: sha256(&detached_signature_bytes),
        detached_signature_bytes,
    }
}

fn observation(signed: &SignedAuthorityV8) -> CryptographicSignatureObservation {
    CryptographicSignatureObservation::for_test_only(
        signed.detached_signature_sha256.clone(),
        signed.canonical_statement_sha256.clone(),
        authority_trust_purpose_v8(signed.challenge.scope_kind()),
    )
}

fn challenge(scope: AuthorityScopeV8, nonce: char) -> AuthorityChallengeV8 {
    let namespace = scope.namespace().to_string();
    let kind = scope.kind();
    let trust_policy = test_only_trust_binding_v8(authority_trust_purpose_v8(kind));
    let schema = match kind {
        AuthorityScopeKindV8::InstallV2 => INSTALL_AUTHORITY_SCHEMA_V2,
        AuthorityScopeKindV8::OneShotRunV2 => ONE_SHOT_RUN_AUTHORITY_SCHEMA_V2,
        AuthorityScopeKindV8::Install
        | AuthorityScopeKindV8::OneShotRun
        | AuthorityScopeKindV8::BreakGlass => AUTHORITY_SCHEMA_V8,
    };
    AuthorityChallengeV8 {
        authority_nonce: digest(nonce),
        expires_at_unix_seconds: 1_900,
        issued_at_unix_seconds: 1_000,
        namespace,
        schema: schema.to_string(),
        scope,
        signer: AuthoritySignerBindingV8 {
            allowed_signers_sha256: trust_policy.allowed_signers_sha256().to_string(),
            key_fingerprint: trust_policy.key_fingerprint().to_string(),
            principal: trust_policy.principal().to_string(),
            signature_algorithm: trust_policy.signature_algorithm(),
        },
    }
}

fn install_scope() -> AuthorityScopeV8 {
    AuthorityScopeV8::Install {
        activation: InstallActivationV8::InstallFilesOnlyNoDaemonReloadEnableOrStart,
        inventory: ExactRootInstallInventoryV8 {
            admissiond_binary: root_file(ADMISSIOND_INSTALL_PATH_V8, '1', 0o555),
            admissiond_unit: root_file(ADMISSIOND_UNIT_PATH_V8, '2', 0o444),
            recovery_binary: root_file(RECOVERY_INSTALL_PATH_V8, '3', 0o555),
            recovery_unit: root_file(RECOVERY_UNIT_PATH_V8, '4', 0o444),
            state_root: RootStateIdentityV8 {
                gid: 0,
                layout_manifest_sha256: digest('5'),
                mode: 0o700,
                path: STATE_ROOT_PATH_V8.to_string(),
                uid: 0,
            },
        },
        target_host: host(),
    }
}

fn install_v2_scope() -> AuthorityScopeV8 {
    AuthorityScopeV8::InstallV2 {
        activation: InstallActivationV8::InstallFilesOnlyNoDaemonReloadEnableOrStart,
        install_plan_sha256: digest('7'),
        inventory: Box::new(install_v2_inventory()),
        state_disposition: InstallStateDispositionV2::FreshEmpty,
        target_host: install_v2_host(),
    }
}

fn install_v2_inventory() -> ExactRootInstallInventoryV2 {
    ExactRootInstallInventoryV2 {
        ctl_binary: root_file(CTL_INSTALL_PATH_V2, '1', 0o555),
        admissiond_binary: root_file(ADMISSIOND_INSTALL_PATH_V2, '2', 0o555),
        recovery_binary: root_file(RECOVERY_INSTALL_PATH_V2, '3', 0o555),
        admissiond_unit: root_file(ADMISSIOND_UNIT_PATH_V8, '4', 0o444),
        recovery_unit: root_file(RECOVERY_UNIT_PATH_V8, '5', 0o444),
        binary_directory: root_directory(INSTALL_BINARY_DIRECTORY_PATH_V2, 0o755),
        state_root: RootStateIdentityV8 {
            gid: 0,
            layout_manifest_sha256: digest('6'),
            mode: 0o700,
            path: STATE_ROOT_PATH_V8.to_string(),
            uid: 0,
        },
        attempts_directory: root_directory(ATTEMPTS_DIRECTORY_PATH_V2, 0o700),
        install_epoch_directory: root_directory(INSTALL_EPOCH_DIRECTORY_PATH_V2, 0o700),
        journal_directory: root_directory(JOURNAL_DIRECTORY_PATH_V2, 0o700),
        nonce_claims_directory: root_directory(NONCE_CLAIMS_DIRECTORY_PATH_V2, 0o700),
        quarantine_directory: root_directory(QUARANTINE_DIRECTORY_PATH_V2, 0o700),
        state_lock: RootFileInstallIdentityV8 {
            content_sha256: sha256(b""),
            gid: 0,
            mode: 0o600,
            path: STATE_LOCK_PATH_V2.to_string(),
            size_bytes: 0,
            uid: 0,
        },
    }
}

fn install_v2_host() -> InstallTargetHostBindingV2 {
    InstallTargetHostBindingV2 {
        boot_id: "01234567-89ab-cdef-8123-456789abcdef".to_string(),
        machine_id_sha256: digest('e'),
    }
}

fn run_scope() -> AuthorityScopeV8 {
    AuthorityScopeV8::OneShotRun {
        attempt: attempt(),
        capability: OneShotRunCapabilityV8::Runner22And23SharedProcessGroupSigstopThenSigcontOnly,
        driver_peer: DriverPeerBindingV8 {
            executable_sha256: digest('2'),
            gid: 1000,
            uid: 1000,
        },
        target_host: host(),
    }
}

fn run_v2_scope() -> AuthorityScopeV8 {
    let attempt = attempt();
    let attempt_identity_sha256 = attempt.sha256().expect("attempt digest");
    let child_relative_name = format!("hepta-v8-{attempt_identity_sha256}");
    AuthorityScopeV8::OneShotRunV2 {
        target_host: run_v2_host(),
        attempt,
        capability: OneShotRunCapabilityV8::Runner22And23SharedProcessGroupSigstopThenSigcontOnly,
        containment: crate::CandidateContainmentProfileV2 {
            schema: crate::CANDIDATE_CONTAINMENT_PROFILE_SCHEMA_V2.to_string(),
            attempt_identity_sha256,
            service_parent_absolute_path: crate::ADMISSIOND_SERVICE_CGROUP_PARENT_V2.to_string(),
            child_absolute_path: format!(
                "{}/{}",
                crate::ADMISSIOND_SERVICE_CGROUP_PARENT_V2,
                child_relative_name
            ),
            child_relative_name,
            child_delegated: false,
        },
        driver_peer: DriverPeerBindingV8 {
            executable_sha256: digest('2'),
            gid: 1000,
            uid: 1000,
        },
    }
}

fn run_v2_host() -> RunTargetHostBindingV2 {
    RunTargetHostBindingV2 {
        boot_id: "01234567-89ab-cdef-8123-456789abcdef".to_string(),
        cgroup_namespace_inode: 103,
        machine_id_sha256: digest('e'),
        mount_namespace_inode: 102,
        pid_namespace_inode: 101,
        systemd_manager_pid: 1,
        systemd_manager_start_time_ticks: 99,
        systemd_unit_fragment_sha256: digest('9'),
        systemd_unit_name: ADMISSIOND_UNIT_NAME_V2.to_string(),
    }
}

fn break_glass_scope() -> AuthorityScopeV8 {
    let attempt = attempt();
    AuthorityScopeV8::BreakGlass {
        restore_plan_sha256: attempt.restore_plan_sha256.clone(),
        recovery_state: RecoveryStateBindingV8 {
            boot_epoch: 1,
            boot_id: "11111111-1111-1111-1111-111111111111".to_string(),
            journal_tip_sha256: digest('2'),
            restore_state_sha256: digest('3'),
            runner_snapshot_sha256: attempt.runner_snapshot_sha256.clone(),
        },
        attempt,
        capability: BreakGlassCapabilityV8::ExactRestorePlanThenAbandonKeepQuarantineAndBarrier,
        target_host: host(),
    }
}

fn attempt() -> AttemptIdentityV8 {
    AttemptIdentityV8 {
        attempt_nonce: digest('a'),
        barrier_generation: 7,
        candidate_head: CANDIDATE_HEAD.to_string(),
        candidate_tree: CANDIDATE_TREE.to_string(),
        driver_manifest_sha256: digest('b'),
        machine_id_sha256: digest('e'),
        parameter_manifest_sha256: digest('d'),
        profile_manifest_sha256: digest('c'),
        restore_plan_sha256: digest('1'),
        runner_snapshot_sha256: digest('f'),
    }
}

fn host() -> TargetHostBindingV8 {
    TargetHostBindingV8 {
        machine_id_sha256: digest('e'),
    }
}

fn root_file(path: &str, digest_character: char, mode: u32) -> RootFileInstallIdentityV8 {
    RootFileInstallIdentityV8 {
        content_sha256: digest(digest_character),
        gid: 0,
        mode,
        path: path.to_string(),
        size_bytes: 100,
        uid: 0,
    }
}

fn root_directory(path: &str, mode: u32) -> RootDirectoryInstallIdentityV2 {
    RootDirectoryInstallIdentityV2 {
        gid: 0,
        mode,
        path: path.to_string(),
        uid: 0,
    }
}

fn digest(character: char) -> String {
    character.to_string().repeat(64)
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

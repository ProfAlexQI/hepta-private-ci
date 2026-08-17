use crate::*;

static_assertions::assert_not_impl_any!(
    SupervisorTrustContextV1: Clone,
    Copy,
    Default,
    serde::Serialize,
    serde::de::DeserializeOwned
);

fn digest(byte: char) -> String {
    byte.to_string().repeat(64)
}

fn statement(signature: char, role: &str) -> SignedStatementShapeV1 {
    SignedStatementShapeV1 {
        algorithm: "ed25519-detached-sha256".to_string(),
        object_sha256: String::new(),
        profile_id: format!("untrusted-{role}-shape-profile-v1"),
        signature_sha256: digest(signature),
        signer_key_id: format!("untrusted-{role}-shape-key-v1"),
        statement_sha256: String::new(),
    }
}

fn fixture() -> CandidateEvidenceV1 {
    let contract = exact_phase_a_contract();
    let boot = digest('b');
    let binary = digest('7');
    let mut evidence = CandidateEvidenceV1 {
        authority: ClosedAuthorityV1::exact(),
        build: BuildEvidenceV1 {
            binary_mode: "0555".to_string(),
            binary_sha256: binary.clone(),
            closure_sha256: digest('6'),
            derivation_path: "/nix/store/0123456789abcdfghijklmnpqrsvwxyz-hepta-vnext.drv"
                .to_string(),
            derivation_path_sha256: String::new(),
            derivation_sha256: digest('3'),
            nar_sha256: digest('5'),
            output_store_path: "/nix/store/abcdfghijklmnpqrsvwxyz0123456789-hepta-vnext"
                .to_string(),
            output_store_path_sha256: String::new(),
            source_archive_sha256: digest('1'),
        },
        checks: CheckEvidenceV1 {
            all_passed: true,
            failed_count: 0,
            ignored_count: 0,
            subject_binary_sha256: String::new(),
            subject_derivation_path_sha256: String::new(),
            subject_derivation_sha256: String::new(),
            subject_output_store_path_sha256: String::new(),
            suite_inventory_sha256: digest('8'),
            test_count: 360,
        },
        contract: contract.clone(),
        disposition: CandidateDispositionV1::ShapeOnlyNoFreshPass,
        events: Vec::new(),
        exclusive: ExclusiveCensusV1 {
            active_candidate_containers_before: 0,
            active_named_volumes_before: 0,
            active_runs_before: 0,
            exclusive_lock_acquired: true,
            lock_name: "hepta-nix-mnl-successor-v1-exclusive".to_string(),
            lock_nonce: digest('a'),
            named_volume_unique: true,
        },
        host: HostEvidenceV1 {
            architecture: "x86_64".to_string(),
            boot_id_sha256: boot.clone(),
            data_volume_free_bytes: MINIMUM_DATA_VOLUME_FREE_BYTES,
            data_volume_root: "/data".to_string(),
            docker_config_sha256: digest('c'),
            docker_platform: "linux/amd64".to_string(),
            host_identity_sha256: digest('d'),
            kernel_system: "Linux".to_string(),
            nix_system: "x86_64-linux".to_string(),
        },
        image: contract.image.clone(),
        input: InputEvidenceV1 {
            candidate_can_read_evidence: false,
            driver_binary_sha256: digest('2'),
            driver_source_sha256: digest('3'),
            product: contract.product.clone(),
            source_archive_sha256: digest('1'),
            tooling: contract.tooling,
            verifier_binary_sha256: digest('4'),
            verifier_source_sha256: digest('5'),
        },
        isolation: IsolationEvidenceV1 {
            cap_drop_all: true,
            candidate_evidence_access: false,
            cores: 1,
            cpuset_single_cpu: true,
            devices_exposed: false,
            driver_read_only: true,
            flake_registry_empty: true,
            host_ipc: false,
            host_pid: false,
            isolation_mode: NixIsolationModeV1::NixSandboxEnabled,
            max_jobs: 1,
            memory_limit_bytes: 8 * 1024 * 1024 * 1024,
            nano_cpus: 1_000_000_000,
            network_none: true,
            nix_store_volume_read_write: true,
            no_new_privileges: true,
            pids_limit: 256,
            presealed_offline_closure_sha256: None,
            privileged: false,
            rootfs_read_only: true,
            source_read_only: true,
            substituters_empty: true,
            tmpfs_nodev_noexec_nosuid: true,
        },
        run: RunIdentityV1 {
            boot_id_sha256: boot,
            run_identity_sha256: String::new(),
            run_nonce: digest('a'),
        },
        runtime_smoke: RuntimeSmokeV1 {
            binary_sha256: binary,
            completed: true,
            exit_code: 0,
            network_attempted: false,
            started: true,
            state_mutated: false,
            stderr_sha256: digest('0'),
            stdout_sha256: digest('9'),
        },
        schema: EVIDENCE_SCHEMA.to_string(),
        schema_version: 1,
        supervisor: SupervisorEvidenceV1 {
            authority: ClosedAuthorityV1::exact(),
            candidate_output_closed_before_commit: true,
            candidate_writes_manifests: false,
            commitment: TwoStageCommitmentV1 {
                close_event_sha256: String::new(),
                copy_ack_material: CopyAckMaterialV1 {
                    byte_count: 65_536,
                    byte_identical: true,
                    bundle_format: "canonical-single-bundle-bytes-v1".to_string(),
                    copy_session_nonce: digest('f'),
                    destination_failure_domain: "independent-supervisor-copy-ack-failure-domain-v1"
                        .to_string(),
                    destination_identity_sha256: digest('e'),
                    destination_read_back: true,
                    destination_sha256: String::new(),
                    freeze_manifest_statement_sha256: String::new(),
                    pre_ack_sealed_bundle_sha256: String::new(),
                    receipt_set_sha256: String::new(),
                    run_identity_sha256: String::new(),
                    seal_event_sha256: String::new(),
                    source_sha256: String::new(),
                    supervisor_seal_statement_sha256: String::new(),
                },
                copy_ack_material_sha256: String::new(),
                copy_ack_failure_domain: "independent-supervisor-copy-ack-failure-domain-v1"
                    .to_string(),
                disposition:
                    CommitmentDispositionV1::UnattestedShapeOnlyNoAuthenticatedSupervisorSignature,
                excludes_own_hash_and_git_self_reference: true,
                final_event_anchor_sha256: String::new(),
                freeze_manifest: statement('1', "freeze"),
                full_evidence_core_sha256: String::new(),
                immutable_after_close: true,
                independent_copy_ack: statement('2', "copy-ack"),
                out_of_tree: true,
                preflight_freshness: PreflightFreshnessBindingV1 {
                    challenge_nonce: digest('e'),
                    expires_generation: 8,
                    generation_epoch_id: "test-monotonic-generation-epoch-v1".to_string(),
                    issued_generation: 7,
                    one_shot: true,
                    session_nonce: digest('a'),
                    verification_generation: 7,
                },
                pre_run_authorization_envelope_sha256: digest('7'),
                pre_run_authorization_payload_sha256: digest('6'),
                pre_ack_sealed_bundle_sha256: String::new(),
                precommit_payload_sha256: String::new(),
                receipt_set_sha256: String::new(),
                seal_event_sha256: String::new(),
                supervisor_seal: statement('3', "seal"),
                terminal_manifest: statement('4', "terminal"),
            },
            driver_source_sha256: digest('3'),
            receipt_set_excludes_git_self_reference: true,
            receipt_set_pin_method: ReceiptPinMethodV1::ExternalOutOfTreeManifestSha256,
            verifier_source_sha256: digest('5'),
        },
    };
    populate_derived_fields(&mut evidence).expect("derived evidence");
    evidence
}

fn production_fixture() -> (CandidateEvidenceV1, SupervisorTrustContextV1) {
    let mut evidence = fixture();
    evidence.contract.tooling.successor_final_tooling = Some(RepositoryIdentityV1 {
        head: "c".repeat(40),
        tree: "d".repeat(40),
    });
    evidence
        .contract
        .external_trust_status
        .external_supervisor_profile_available = true;
    evidence
        .contract
        .external_trust_status
        .external_supervisor_trust_root_available = true;
    evidence
        .contract
        .external_trust_status
        .immutable_stage_one_manifest_available = true;
    evidence
        .contract
        .external_trust_status
        .immutable_stage_two_manifest_available = true;
    evidence.input.tooling = evidence.contract.tooling.clone();
    evidence.disposition =
        CandidateDispositionV1::AwaitingExternalSupervisorVerificationNoAuthority;
    evidence.supervisor.commitment.disposition =
        CommitmentDispositionV1::AwaitingExternalSupervisorSignatureVerificationNoAuthority;
    let context = test_trust_context_for(&mut evidence).expect("test-only opaque trust context");
    (evidence, context)
}

fn rejected(evidence: &CandidateEvidenceV1) {
    assert!(validate_candidate_shape(evidence).is_err());
}

#[test]
fn contract_binds_products_ui_receipts_baseline_phase_a_and_absent_final() {
    let contract = exact_phase_a_contract();
    assert_eq!(contract.product.backend.head, BACKEND_HEAD);
    assert_eq!(contract.product.backend.tree, BACKEND_TREE);
    assert_eq!(contract.product.ui.head, UI_HEAD);
    assert_eq!(contract.product.ui.tree, UI_TREE);
    assert_eq!(contract.product.ui.source_blob_oid, UI_SOURCE_BLOB_OID);
    assert_eq!(contract.product.ui.source_path, UI_SOURCE_PATH);
    assert_eq!(contract.product.ui.source_sha256, UI_SOURCE_SHA256);
    assert_eq!(contract.product.ui.projection_count, 22);
    assert_eq!(
        contract.product.ui.inventory_sha256,
        UI_ROUTE_INVENTORY_SHA256
    );
    assert_eq!(
        contract.development_freeze.manifest_sha256,
        FREEZE_MANIFEST_SHA256
    );
    assert_eq!(contract.strategy.manifest_sha256, STRATEGY_MANIFEST_SHA256);
    assert_eq!(
        contract.tooling.tooling_baseline.head,
        TOOLING_BASELINE_HEAD
    );
    assert_eq!(
        contract.tooling.successor_phase_a_ancestry.head,
        SUCCESSOR_PHASE_A_HEAD
    );
    assert!(contract.tooling.successor_final_tooling.is_none());
    assert_eq!(
        contract.historical_rev7.disposition,
        HistoricalDispositionV1::HistoricalContentIdentity
    );
    assert!(!contract.historical_rev7.fresh_pass);
    assert!(
        !contract
            .external_trust_status
            .candidate_source_contains_self_pins
    );
    assert!(contract.external_trust_status.out_of_tree_freeze_required);
    assert!(
        !contract
            .external_trust_status
            .external_supervisor_profile_available
    );
    assert!(
        !contract
            .external_trust_status
            .external_supervisor_trust_root_available
    );
    let json = serde_json::to_string(&contract).expect("contract JSON");
    for forbidden_self_pin in [
        "host_identity_sha256",
        "driver_source_sha256",
        "driver_binary_sha256",
        "verifier_source_sha256",
        "verifier_binary_sha256",
        "external_receipt_set_sha256",
    ] {
        assert!(!json.contains(forbidden_self_pin));
    }
    assert!(contract.authority.is_fully_closed());
}

#[test]
fn production_status_plan_verify_and_live_execution_are_blocked_before_run() {
    let status = production_status();
    assert_eq!(status.disposition, ProductionDispositionV1::Blocked);
    assert!(!status.ready_to_plan);
    assert_eq!(status.blockers.len(), 24);
    assert!(
        status
            .blockers
            .contains(&"successor_final_tooling_unfrozen".to_string())
    );
    assert!(
        status
            .blockers
            .contains(&"external_supervisor_profile_missing".to_string())
    );
    assert!(
        status
            .blockers
            .contains(&"external_supervisor_trust_root_missing".to_string())
    );
    assert!(
        status
            .blockers
            .contains(&"immutable_stage_one_manifest_missing".to_string())
    );
    assert!(
        status
            .blockers
            .contains(&"immutable_stage_two_manifest_missing".to_string())
    );
    assert!(
        status
            .blockers
            .contains(&"host_identity_pin_missing_from_external_profile".to_string())
    );
    assert!(
        status
            .blockers
            .contains(&"docker_config_pin_missing_from_external_profile".to_string())
    );
    assert!(
        status
            .blockers
            .contains(&"independent_copy_ack_signature_missing".to_string())
    );
    assert!(
        status
            .blockers
            .contains(&"production_role_separated_signature_policy_missing".to_string())
    );
    assert!(status.blockers.contains(
        &"production_durable_replay_policy_and_crash_reboot_qualification_missing".to_string()
    ));
    assert!(
        status
            .blockers
            .contains(&"production_wall_clock_immediate_spawn_state_machine_missing".to_string())
    );
    for blocker in [
        "typed_final_artifact_freeze_semantics_not_joined_to_platform_plan",
        "final_tooling_ancestry_proof_not_joined_to_platform_plan",
        "successor_receipt_run_identity_algorithm_migration_missing",
        "live_read_only_host_collector_and_closed_runner_missing",
        "qualified_workspace_flake_check_output_missing",
        "qualified_nix_sandbox_container_feasibility_missing",
        "auditable_network_attempt_observer_missing",
        "exact_state_mutation_pre_post_inventory_diff_missing",
        "independent_bundle_copy_readback_process_identity_missing",
    ] {
        assert!(status.blockers.contains(&blocker.to_string()));
    }
    assert!(matches!(
        plan_production_run(),
        Err(NixMnlError::Blocked(_))
    ));
    assert!(matches!(
        execute_live_nix_run_v1(),
        Err(NixMnlError::Blocked(_))
    ));
    assert!(matches!(
        verify_production_evidence(b"not even evidence"),
        Err(NixMnlError::Blocked(_))
    ));
}

#[test]
fn successor_plan_run_identity_requires_explicit_receipt_algorithm_migration() {
    let run_nonce = digest('d');
    let boot_id = digest('b');
    let successor =
        codex_hepta_mnl_trust_v1::derive_run_identity_sha256(&run_nonce, &boot_id).unwrap();
    let frozen_v1 =
        crate::verify::legacy_receipt_run_identity_sha256(&run_nonce, &boot_id).unwrap();

    assert_ne!(successor, frozen_v1);
    assert!(
        production_status()
            .blockers
            .contains(&"successor_receipt_run_identity_algorithm_migration_missing".to_string())
    );
}

#[test]
fn exact_x86_64_shape_and_canonical_bytes_are_non_authorizing_only() {
    let evidence = fixture();
    let validated = validate_candidate_shape(&evidence).expect("shape");
    assert_eq!(validated.evidence_sha256().len(), 64);
    let bytes = serde_json::to_vec(&evidence).expect("canonical bytes");
    assert_eq!(
        validate_canonical_shape_only(&bytes)
            .expect("canonical shape")
            .evidence_sha256(),
        validated.evidence_sha256()
    );
    assert!(matches!(
        verify_production_evidence(&bytes),
        Err(NixMnlError::Blocked(_))
    ));
}

#[test]
fn sandbox_or_presealed_offline_closure_is_required_with_network_none() {
    let mut offline = fixture();
    offline.isolation.isolation_mode = NixIsolationModeV1::PresealedOfflineClosure;
    offline.isolation.presealed_offline_closure_sha256 = Some(digest('e'));
    populate_derived_fields(&mut offline).expect("offline derived");
    validate_candidate_shape(&offline).expect("presealed offline shape");

    let mut missing = offline.clone();
    missing.isolation.presealed_offline_closure_sha256 = None;
    rejected(&missing);
    let mut offline_substituter = offline.clone();
    offline_substituter.isolation.substituters_empty = false;
    rejected(&offline_substituter);
    let mut offline_registry = offline;
    offline_registry.isolation.flake_registry_empty = false;
    rejected(&offline_registry);
    let mut sandbox_with_closure = fixture();
    sandbox_with_closure
        .isolation
        .presealed_offline_closure_sha256 = Some(digest('e'));
    rejected(&sandbox_with_closure);
    let mut network = fixture();
    network.isolation.network_none = false;
    rejected(&network);
}

#[test]
fn identity_and_digest_transplants_are_rejected() {
    let mut host = fixture();
    host.host.architecture = "aarch64".to_string();
    rejected(&host);
    let mut product = fixture();
    product.input.product.backend.head = "0".repeat(40);
    rejected(&product);
    let mut final_tooling = fixture();
    final_tooling.contract.tooling.successor_final_tooling = Some(RepositoryIdentityV1 {
        head: "a".repeat(40),
        tree: "b".repeat(40),
    });
    rejected(&final_tooling);
    let mut image = fixture();
    image.image.docker_image_sha256 = digest('f');
    rejected(&image);
    let mut run = fixture();
    run.run.run_nonce = digest('e');
    rejected(&run);
    let mut source = fixture();
    source.build.source_archive_sha256 = digest('e');
    rejected(&source);
    let mut driver = fixture();
    driver.supervisor.driver_source_sha256 = digest('e');
    rejected(&driver);
}

#[test]
fn caller_cannot_claim_external_profile_trust_root_or_freeze_manifests() {
    let mut value = fixture();
    value
        .contract
        .external_trust_status
        .external_supervisor_profile_available = true;
    rejected(&value);
    let mut value = fixture();
    value
        .contract
        .external_trust_status
        .external_supervisor_trust_root_available = true;
    rejected(&value);
    let mut value = fixture();
    value
        .contract
        .external_trust_status
        .immutable_stage_one_manifest_available = true;
    rejected(&value);
    let mut value = fixture();
    value
        .contract
        .external_trust_status
        .immutable_stage_two_manifest_available = true;
    rejected(&value);
}

#[test]
fn historical_rev7_cannot_be_promoted_to_fresh_pass() {
    let mut value = fixture();
    value.contract.historical_rev7.fresh_pass = true;
    rejected(&value);
    let mut value = fixture();
    value.contract.historical_rev7.original_terminal_claim = "FRESH_PASS".to_string();
    rejected(&value);
    let mut value = fixture();
    value.contract.historical_rev7.terminal_sha256 = digest('e');
    rejected(&value);
}

#[test]
fn isolation_violations_are_rejected() {
    let mutations: &[fn(&mut CandidateEvidenceV1)] = &[
        |v| v.isolation.rootfs_read_only = false,
        |v| v.isolation.source_read_only = false,
        |v| v.isolation.driver_read_only = false,
        |v| v.isolation.candidate_evidence_access = true,
        |v| v.input.candidate_can_read_evidence = true,
        |v| v.isolation.cap_drop_all = false,
        |v| v.isolation.no_new_privileges = false,
        |v| v.isolation.privileged = true,
        |v| v.isolation.devices_exposed = true,
        |v| v.isolation.host_pid = true,
        |v| v.isolation.host_ipc = true,
        |v| v.isolation.substituters_empty = false,
        |v| v.isolation.flake_registry_empty = false,
        |v| v.isolation.tmpfs_nodev_noexec_nosuid = false,
    ];
    for mutate in mutations {
        let mut value = fixture();
        mutate(&mut value);
        rejected(&value);
    }
}

#[test]
fn resource_violations_are_rejected() {
    let mutations: &[fn(&mut CandidateEvidenceV1)] = &[
        |v| v.host.data_volume_free_bytes = MINIMUM_DATA_VOLUME_FREE_BYTES - 1,
        |v| v.isolation.nano_cpus = 2_000_000_000,
        |v| v.isolation.cpuset_single_cpu = false,
        |v| v.isolation.max_jobs = 2,
        |v| v.isolation.cores = 2,
        |v| v.isolation.memory_limit_bytes = 0,
        |v| v.isolation.pids_limit = 0,
    ];
    for mutate in mutations {
        let mut value = fixture();
        mutate(&mut value);
        rejected(&value);
    }
}

#[test]
fn drv_store_nar_closure_check_and_smoke_violations_are_rejected() {
    let mutations: &[fn(&mut CandidateEvidenceV1)] = &[
        |v| v.build.derivation_path = "/tmp/result.drv".to_string(),
        |v| v.build.output_store_path = "/tmp/result".to_string(),
        |v| v.build.nar_sha256 = "not-a-digest".to_string(),
        |v| v.build.binary_mode = "0755".to_string(),
        |v| v.checks.all_passed = false,
        |v| v.checks.test_count = 0,
        |v| v.checks.failed_count = 1,
        |v| v.checks.ignored_count = 1,
        |v| v.checks.subject_binary_sha256 = digest('1'),
        |v| v.checks.subject_derivation_path_sha256 = digest('1'),
        |v| v.checks.subject_derivation_sha256 = digest('2'),
        |v| v.checks.subject_output_store_path_sha256 = digest('3'),
        |v| v.runtime_smoke.binary_sha256 = digest('e'),
        |v| v.runtime_smoke.started = false,
        |v| v.runtime_smoke.completed = false,
        |v| v.runtime_smoke.exit_code = 1,
        |v| v.runtime_smoke.network_attempted = true,
        |v| v.runtime_smoke.state_mutated = true,
    ];
    for mutate in mutations {
        let mut value = fixture();
        mutate(&mut value);
        rejected(&value);
    }
}

#[test]
fn nix_store_paths_require_exact_canonical_hash_name_and_drv_roles() {
    let invalid_derivations = [
        "/tmp/result.drv".to_string(),
        "/nix/store/0123456789abcdfghijklmnpqrsvwxyz-result/child.drv".to_string(),
        "/nix/store/./0123456789abcdfghijklmnpqrsvwxyz-result.drv".to_string(),
        "/nix/store/../0123456789abcdfghijklmnpqrsvwxyz-result.drv".to_string(),
        format!("/nix/store/{}-result.drv", "0".repeat(31)),
        format!("/nix/store/{}-result.drv", "e".repeat(32)),
        "/nix/store/0123456789abcdfghijklmnpqrsvwxyz-".to_string(),
        "/nix/store/0123456789abcdfghijklmnpqrsvwxyz-.hidden.drv".to_string(),
        "/nix/store/0123456789abcdfghijklmnpqrsvwxyz-result name.drv".to_string(),
        "/nix/store/0123456789abcdfghijklmnpqrsvwxyz-result".to_string(),
        "/nix/store/0123456789abcdfghijklmnpqrsvwxyz-result.drv.drv".to_string(),
    ];
    for derivation_path in invalid_derivations {
        let mut value = fixture();
        value.build.derivation_path = derivation_path;
        populate_derived_fields(&mut value).expect("invalid path still has a model digest");
        rejected(&value);
    }

    let invalid_outputs = [
        "/tmp/result".to_string(),
        "/nix/store/abcdfghijklmnpqrsvwxyz0123456789-result/child".to_string(),
        "/nix/store/abcdfghijklmnpqrsvwxyz0123456789-result.drv".to_string(),
        "/nix/store/abcdfghijklmnpqrsvwxyz0123456789-result name".to_string(),
    ];
    for output_store_path in invalid_outputs {
        let mut value = fixture();
        value.build.output_store_path = output_store_path;
        populate_derived_fields(&mut value).expect("invalid path still has a model digest");
        rejected(&value);
    }

    for allowed_leading_byte in ['+', '-', '_', '?', '='] {
        let mut value = fixture();
        value.build.output_store_path =
            format!("/nix/store/abcdfghijklmnpqrsvwxyz0123456789-{allowed_leading_byte}result");
        populate_derived_fields(&mut value).expect("canonical leading-byte path digest");
        validate_candidate_shape(&value).expect("Nix permits this non-dot store-name prefix");
    }

    let mut drv_substring = fixture();
    drv_substring.build.derivation_path =
        "/nix/store/0123456789abcdfghijklmnpqrsvwxyz-result.drv-copy.drv".to_string();
    populate_derived_fields(&mut drv_substring).expect("canonical derivation path digest");
    validate_candidate_shape(&drv_substring)
        .expect("only a repeated final .drv suffix is noncanonical for a derivation role");

    let mut derivation_path_digest = fixture();
    derivation_path_digest.build.derivation_path_sha256 = digest('f');
    rejected(&derivation_path_digest);

    let mut checked_path_digest = fixture();
    checked_path_digest.checks.subject_derivation_path_sha256 = digest('f');
    rejected(&checked_path_digest);
}

#[test]
fn output_seal_copy_ack_and_self_reference_violations_are_rejected() {
    let mutations: &[fn(&mut CandidateEvidenceV1)] = &[
        |v| v.supervisor.candidate_output_closed_before_commit = false,
        |v| v.supervisor.candidate_writes_manifests = true,
        |v| v.supervisor.receipt_set_excludes_git_self_reference = false,
        |v| v.supervisor.receipt_set_pin_method = ReceiptPinMethodV1::GitTrackedSelfReferential,
        |v| v.supervisor.commitment.out_of_tree = false,
        |v| v.supervisor.commitment.immutable_after_close = false,
        |v| v.supervisor.commitment.copy_ack_failure_domain = "same-domain".to_string(),
        |v| {
            v.supervisor.commitment.independent_copy_ack.signer_key_id = v
                .supervisor
                .commitment
                .supervisor_seal
                .signer_key_id
                .clone()
        },
        |v| {
            v.supervisor
                .commitment
                .excludes_own_hash_and_git_self_reference = false
        },
        |v| v.supervisor.commitment.precommit_payload_sha256 = digest('a'),
        |v| v.supervisor.commitment.freeze_manifest.object_sha256 = digest('f'),
        |v| v.supervisor.commitment.freeze_manifest.signature_sha256 = digest('e'),
        |v| v.supervisor.commitment.freeze_manifest.statement_sha256 = digest('b'),
        |v| v.supervisor.commitment.full_evidence_core_sha256 = digest('c'),
        |v| v.supervisor.commitment.close_event_sha256 = digest('d'),
        |v| v.supervisor.commitment.receipt_set_sha256 = digest('e'),
        |v| v.supervisor.commitment.supervisor_seal.statement_sha256 = digest('f'),
        |v| v.supervisor.commitment.seal_event_sha256 = digest('1'),
        |v| v.supervisor.commitment.pre_ack_sealed_bundle_sha256 = digest('2'),
        |v| v.supervisor.commitment.copy_ack_material_sha256 = digest('3'),
        |v| v.supervisor.commitment.copy_ack_material.source_sha256 = digest('4'),
        |v| v.supervisor.commitment.copy_ack_material.destination_sha256 = digest('5'),
        |v| v.supervisor.commitment.copy_ack_material.byte_count = 0,
        |v| v.supervisor.commitment.copy_ack_material.byte_identical = false,
        |v| {
            v.supervisor
                .commitment
                .copy_ack_material
                .destination_read_back = false
        },
        |v| v.supervisor.commitment.copy_ack_material.bundle_format = "directory-tree".to_string(),
        |v| {
            v.supervisor
                .commitment
                .copy_ack_material
                .destination_failure_domain = "same-domain".to_string()
        },
        |v| {
            v.supervisor
                .commitment
                .copy_ack_material
                .destination_identity_sha256 = digest('6')
        },
        |v| v.supervisor.commitment.copy_ack_material.copy_session_nonce = "bad".to_string(),
        |v| {
            v.supervisor
                .commitment
                .copy_ack_material
                .freeze_manifest_statement_sha256 = digest('7')
        },
        |v| {
            v.supervisor
                .commitment
                .copy_ack_material
                .run_identity_sha256 = digest('8')
        },
        |v| v.supervisor.commitment.copy_ack_material.receipt_set_sha256 = digest('9'),
        |v| {
            v.supervisor
                .commitment
                .copy_ack_material
                .supervisor_seal_statement_sha256 = digest('0')
        },
        |v| v.supervisor.commitment.copy_ack_material.seal_event_sha256 = digest('a'),
        |v| {
            v.supervisor
                .commitment
                .copy_ack_material
                .pre_ack_sealed_bundle_sha256 = digest('b')
        },
        |v| {
            v.supervisor
                .commitment
                .independent_copy_ack
                .statement_sha256 = digest('3')
        },
        |v| v.supervisor.commitment.independent_copy_ack.object_sha256 = digest('6'),
        |v| v.supervisor.commitment.final_event_anchor_sha256 = digest('4'),
        |v| v.supervisor.commitment.terminal_manifest.object_sha256 = digest('7'),
        |v| v.supervisor.commitment.terminal_manifest.signature_sha256 = digest('8'),
        |v| v.supervisor.commitment.terminal_manifest.statement_sha256 = digest('5'),
    ];
    for mutate in mutations {
        let mut value = fixture();
        mutate(&mut value);
        rejected(&value);
    }
}

#[test]
fn copy_ack_byte_count_is_exact_not_nonzero_or_transplantable() {
    let exact = fixture();
    let exact_byte_count = exact.supervisor.commitment.copy_ack_material.byte_count;
    assert!(exact_byte_count > 1);

    for wrong_byte_count in [
        0,
        exact_byte_count - 1,
        exact_byte_count + 1,
        exact.build.source_archive_sha256.len() as u64,
    ] {
        assert_ne!(wrong_byte_count, exact_byte_count);
        let mut value = exact.clone();
        value.supervisor.commitment.copy_ack_material.byte_count = wrong_byte_count;
        rebind_test_copy_ack_downstream_preserving_byte_count(&mut value)
            .expect("self-consistent wrong copy-byte-count DAG");
        rejected(&value);
    }
}

#[test]
fn replay_and_event_chain_violations_are_rejected() {
    let mut missing = fixture();
    missing.events.pop();
    rejected(&missing);
    let mut replay = fixture();
    replay.events.push(replay.events[0].clone());
    rejected(&replay);
    let mut index = fixture();
    index.events[3].event_index = 2;
    rejected(&index);
    let mut predecessor = fixture();
    predecessor.events[4].predecessor_event_sha256 = Some(digest('f'));
    rejected(&predecessor);
    let mut nonce = fixture();
    nonce.events[5].run_nonce = digest('f');
    rejected(&nonce);
    let mut event = fixture();
    event.events[6].event_sha256 = digest('f');
    rejected(&event);
}

#[test]
fn private_full_verify_chain_is_satisfiable_but_cannot_mint_verified_token() {
    let (exact, context) = production_fixture();
    validate_trust_context(&context).expect("opaque trust context shape");
    validate_trusted_equalities(&exact, &context).expect("all trusted equalities");
    let bytes = serde_json::to_vec(&exact).expect("canonical production evidence");
    let error = verify_with_trusted_context(&bytes, context)
        .expect_err("real signatures and durable replay storage remain blocked");
    assert!(matches!(&error, NixMnlError::Blocked(_)));
    assert!(
        error
            .to_string()
            .contains("detached-signature verification")
    );
    assert!(error.to_string().contains("atomic one-shot replay"));
    assert!(matches!(
        verify_production_evidence(&bytes),
        Err(NixMnlError::Blocked(_))
    ));
}

#[test]
fn rederived_runtime_evidence_is_rejected_by_frozen_r_and_t_result_envelope() {
    let (mut changed, context) = production_fixture();
    changed.host.data_volume_free_bytes += 1;
    populate_derived_fields(&mut changed).expect("re-derived dynamic evidence DAG");
    let bytes = serde_json::to_vec(&changed).expect("canonical changed evidence");
    let error = verify_with_trusted_context(&bytes, context)
        .expect_err("old post-run result envelope cannot authorize changed R/T");
    assert!(matches!(&error, NixMnlError::Invalid(_)));
    assert!(error.to_string().contains("byte-equal"));
}

#[test]
fn rederived_docker_config_and_post_run_result_cannot_escape_pre_run_profile_pin() {
    let (mut changed, mut context) = production_fixture();
    let frozen_host_identity = changed.host.host_identity_sha256.clone();
    changed.host.docker_config_sha256 = digest('1');
    assert_eq!(changed.host.host_identity_sha256, frozen_host_identity);
    populate_derived_fields(&mut changed).expect("re-derived Docker-config evidence DAG");
    rebind_test_post_run_result_to_evidence(&mut context, &changed)
        .expect("re-derived post-run result envelope");
    validate_trust_context(&context).expect("internally consistent old pre-run profile");
    assert!(validate_trusted_equalities(&changed, &context).is_err());

    let bytes = serde_json::to_vec(&changed).expect("canonical changed evidence");
    let error = verify_with_trusted_context(&bytes, context)
        .expect_err("frozen Docker config must reject a fully re-derived post-run result");
    assert!(matches!(&error, NixMnlError::Invalid(_)));
    assert!(error.to_string().contains("byte-equal"));
}

#[test]
fn trusted_equalities_reject_every_static_and_post_run_transplant() {
    let (exact, context) = production_fixture();

    let mutations: &[fn(&mut CandidateEvidenceV1)] = &[
        |v| v.host.host_identity_sha256 = digest('1'),
        |v| v.input.driver_source_sha256 = digest('2'),
        |v| v.input.driver_binary_sha256 = digest('3'),
        |v| v.input.verifier_source_sha256 = digest('4'),
        |v| v.input.verifier_binary_sha256 = digest('5'),
        |v| v.input.source_archive_sha256 = digest('6'),
        |v| v.supervisor.commitment.pre_run_authorization_payload_sha256 = digest('7'),
        |v| {
            v.supervisor
                .commitment
                .pre_run_authorization_envelope_sha256 = digest('8')
        },
        |v| v.supervisor.commitment.receipt_set_sha256 = digest('6'),
        |v| v.supervisor.commitment.freeze_manifest.statement_sha256 = digest('7'),
        |v| v.supervisor.commitment.supervisor_seal.statement_sha256 = digest('8'),
        |v| {
            v.supervisor
                .commitment
                .independent_copy_ack
                .statement_sha256 = digest('9')
        },
        |v| {
            v.supervisor
                .commitment
                .independent_copy_ack
                .signature_sha256 = digest('0')
        },
        |v| v.supervisor.commitment.terminal_manifest.statement_sha256 = digest('a'),
        |v| v.supervisor.commitment.terminal_manifest.signature_sha256 = digest('b'),
        |v| {
            v.supervisor
                .commitment
                .copy_ack_material
                .destination_identity_sha256 = digest('c')
        },
        |v| {
            v.input.tooling.successor_final_tooling = Some(RepositoryIdentityV1 {
                head: "e".repeat(40),
                tree: "f".repeat(40),
            })
        },
    ];
    for mutate in mutations {
        let mut changed = exact.clone();
        mutate(&mut changed);
        assert!(validate_trusted_equalities(&changed, &context).is_err());
    }
}

#[test]
fn profile_envelopes_root_and_freshness_transplants_are_rejected() {
    let (_, mut wrong_root) = production_fixture();
    wrong_root.trust_root.authorized_pre_run_payload_sha256 = digest('f');
    assert!(validate_trust_context(&wrong_root).is_err());

    let (_, mut wrong_envelope) = production_fixture();
    wrong_envelope
        .pre_run_authorization
        .authorization
        .signature_sha256 = digest('0');
    assert!(validate_trust_context(&wrong_envelope).is_err());

    let (_, mut wrong_result) = production_fixture();
    wrong_result.post_run_result.payload.receipt_set_sha256 = digest('1');
    assert!(validate_trust_context(&wrong_result).is_err());

    let (_, mut replayed) = production_fixture();
    replayed.freshness_lease.consumed_in_this_verification = true;
    assert!(validate_trust_context(&replayed).is_err());

    let (_, mut expired) = production_fixture();
    expired.freshness_lease.verification_generation = expired.freshness_lease.expires_generation;
    assert!(validate_trust_context(&expired).is_err());

    // Even a fully rehashed/re-authorized profile cannot widen the frozen
    // Phase-A contract beyond final tooling plus the four trust-stage bits.
    let (_, mut widened) = production_fixture();
    widened
        .pre_run_authorization
        .payload
        .production_contract
        .product
        .backend
        .head = "0".repeat(40);
    rebind_test_context_hashes(&mut widened).expect("fully rehashed widened context");
    assert!(validate_trust_context(&widened).is_err());
}

#[test]
fn copy_ack_key_cannot_reuse_profile_or_post_run_authorization_key() {
    let (_, mut profile_reuse) = production_fixture();
    let copy_key = profile_reuse.trust_root.copy_ack_signer_key_id.clone();
    profile_reuse
        .pre_run_authorization
        .authorization
        .signer_key_id = copy_key.clone();
    profile_reuse.trust_root.profile_authorization_signer_key_id = copy_key.clone();
    rebind_test_context_hashes(&mut profile_reuse)
        .expect("self-consistent profile-authorization key reuse");
    assert!(validate_trust_context(&profile_reuse).is_err());

    let (_, mut result_reuse) = production_fixture();
    result_reuse
        .pre_run_authorization
        .payload
        .result_envelope_authorization
        .signer_key_id = copy_key.clone();
    result_reuse.post_run_result.authorization.signer_key_id = copy_key;
    rebind_test_context_hashes(&mut result_reuse)
        .expect("self-consistent post-run authorization key reuse");
    assert!(validate_trust_context(&result_reuse).is_err());
}

#[test]
fn pre_run_p_and_dynamic_c_have_distinct_stage_sensitivity() {
    let base = fixture();
    let base_p = base.supervisor.commitment.precommit_payload_sha256.clone();
    let base_m = base
        .supervisor
        .commitment
        .freeze_manifest
        .statement_sha256
        .clone();
    let base_c = base.supervisor.commitment.full_evidence_core_sha256.clone();
    let base_r = base.supervisor.commitment.receipt_set_sha256.clone();
    let base_b = base
        .supervisor
        .commitment
        .pre_ack_sealed_bundle_sha256
        .clone();
    let base_t = base
        .supervisor
        .commitment
        .terminal_manifest
        .statement_sha256
        .clone();

    let mut runtime_change = base.clone();
    runtime_change.host.data_volume_free_bytes += 1;
    populate_derived_fields(&mut runtime_change).expect("dynamic re-derive");
    assert_eq!(
        runtime_change
            .supervisor
            .commitment
            .precommit_payload_sha256,
        base_p
    );
    assert_eq!(
        runtime_change
            .supervisor
            .commitment
            .freeze_manifest
            .statement_sha256,
        base_m
    );
    assert_ne!(
        runtime_change
            .supervisor
            .commitment
            .full_evidence_core_sha256,
        base_c
    );
    assert_ne!(
        runtime_change.supervisor.commitment.receipt_set_sha256,
        base_r
    );
    assert_ne!(
        runtime_change
            .supervisor
            .commitment
            .pre_ack_sealed_bundle_sha256,
        base_b
    );
    assert_ne!(
        runtime_change
            .supervisor
            .commitment
            .terminal_manifest
            .statement_sha256,
        base_t
    );

    let mut static_change = base;
    static_change.input.source_archive_sha256 = digest('2');
    static_change.build.source_archive_sha256 = digest('2');
    populate_derived_fields(&mut static_change).expect("static re-derive");
    assert_ne!(
        static_change.supervisor.commitment.precommit_payload_sha256,
        base_p
    );
}

#[test]
fn m_pre_run_envelope_and_lease_changes_flow_through_e0_and_downstream() {
    let base = fixture();
    let base_p = base.supervisor.commitment.precommit_payload_sha256.clone();
    let base_e0 = base.events[0].event_sha256.clone();
    let base_c = base.supervisor.commitment.full_evidence_core_sha256.clone();
    let base_t = base
        .supervisor
        .commitment
        .terminal_manifest
        .statement_sha256
        .clone();

    let mut changed_m = base.clone();
    changed_m
        .supervisor
        .commitment
        .freeze_manifest
        .signature_sha256 = digest('9');
    populate_derived_fields(&mut changed_m).expect("M re-derive");
    assert_eq!(
        changed_m.supervisor.commitment.precommit_payload_sha256,
        base_p
    );
    assert_ne!(changed_m.events[0].event_sha256, base_e0);
    assert_ne!(
        changed_m.supervisor.commitment.full_evidence_core_sha256,
        base_c
    );
    assert_ne!(
        changed_m
            .supervisor
            .commitment
            .terminal_manifest
            .statement_sha256,
        base_t
    );

    let mut changed_envelope = base.clone();
    changed_envelope
        .supervisor
        .commitment
        .pre_run_authorization_envelope_sha256 = digest('8');
    populate_derived_fields(&mut changed_envelope).expect("envelope re-derive");
    assert_ne!(
        changed_envelope
            .supervisor
            .commitment
            .precommit_payload_sha256,
        base_p
    );
    assert_ne!(changed_envelope.events[0].event_sha256, base_e0);

    let mut changed_session = base;
    changed_session.run.run_nonce = digest('b');
    changed_session.exclusive.lock_nonce = digest('b');
    changed_session
        .supervisor
        .commitment
        .preflight_freshness
        .session_nonce = digest('b');
    populate_derived_fields(&mut changed_session).expect("lease session re-derive");
    assert_eq!(
        changed_session
            .supervisor
            .commitment
            .precommit_payload_sha256,
        base_p
    );
    assert_ne!(changed_session.events[0].event_sha256, base_e0);
    assert_ne!(
        changed_session
            .supervisor
            .commitment
            .full_evidence_core_sha256,
        base_c
    );
}

#[test]
fn every_authority_bit_is_false_and_individually_rejected() {
    assert!(fixture().authority.is_fully_closed());
    for index in 0..24 {
        let mut value = fixture();
        let authority = &mut value.authority;
        match index {
            0 => authority.automatic_transition = true,
            1 => authority.candidate_execution = true,
            2 => authority.container_launch = true,
            3 => authority.cutover = true,
            4 => authority.default_ref_change = true,
            5 => authority.deletion = true,
            6 => authority.enforce = true,
            7 => authority.evidence_acceptance = true,
            8 => authority.full_matrix_claim = true,
            9 => authority.ga_claim = true,
            10 => authority.install = true,
            11 => authority.local_ref_change = true,
            12 => authority.mutation = true,
            13 => authority.operator_acceptance = true,
            14 => authority.outbound = true,
            15 => authority.production = true,
            16 => authority.promotion = true,
            17 => authority.qualification_authority = true,
            18 => authority.recutover = true,
            19 => authority.refs = true,
            20 => authority.remote_ref_change = true,
            21 => authority.retirement = true,
            22 => authority.rollback = true,
            23 => authority.writer_control = true,
            _ => unreachable!(),
        }
        rejected(&value);
    }
    let mut supervisor = fixture();
    supervisor.supervisor.authority.qualification_authority = true;
    rejected(&supervisor);
}

#[test]
fn unknown_fields_and_noncanonical_bytes_are_rejected() {
    let evidence = fixture();
    let mut value = serde_json::to_value(&evidence).expect("value");
    value
        .as_object_mut()
        .expect("object")
        .insert("success".to_string(), serde_json::Value::Bool(true));
    assert!(validate_canonical_shape_only(&serde_json::to_vec(&value).expect("bytes")).is_err());
    let mut bytes = serde_json::to_vec(&evidence).expect("bytes");
    bytes.push(b'\n');
    assert!(validate_canonical_shape_only(&bytes).is_err());
}

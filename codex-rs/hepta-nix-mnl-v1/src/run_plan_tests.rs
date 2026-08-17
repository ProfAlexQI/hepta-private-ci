use serde::Serialize;
use sha2::Digest;
use static_assertions::assert_not_impl_any;

use crate::*;

assert_not_impl_any!(InspectedNixClosedRunPlanV1: Clone, Copy, Serialize, serde::de::DeserializeOwned);
assert_not_impl_any!(JoinedNixClosedRunPlanPreparedClaimInspectionV1: Clone, Copy, Serialize, serde::de::DeserializeOwned);

const TEST_SANDBOX_PLAN_BYTE_COUNT: usize = 57_524;
const TEST_SANDBOX_PLAN_SHA256: &str =
    "bd42d959c45d3c50a9b5de77918a03d83af66259655487bca967e5cc62bb4686";
const TEST_PRESEALED_PLAN_BYTE_COUNT: usize = 45_033;
const TEST_PRESEALED_PLAN_SHA256: &str =
    "6b09e63a9dfeb8b574cdfa5a0930fc25a79574a949ace0e6e461ce1442931960";

#[test]
fn exact_sandbox_plan_is_canonical_closed_and_non_authorizing() {
    let plan = sandbox_plan();
    let bytes = serde_json::to_vec(&plan).expect("canonical closed plan");
    let inspected = inspect_canonical_nix_closed_run_plan(&bytes).expect("closed plan inspection");

    assert_eq!(inspected.canonical_bytes(), bytes);
    assert_eq!(inspected.closed_run_plan_sha256(), sha256(&bytes));
    assert_eq!(inspected.profile_id(), "test-nix-pre-run-profile-v1");
    assert_eq!(
        inspected.disposition(),
        ClosedRunPlanDispositionV1::FreshSandboxBuildInspectionOnlyNoLaunchAuthority
    );
    assert!(!inspected.authorizes_live());
    assert!(!inspected.durable_claim_observed());
    assert!(!inspected.host_observed());
    assert!(!inspected.execution_ready());
    assert!(!inspected.flake_check_output_observed());
    assert!(!inspected.source_materialized());
    assert!(!inspected.artifacts_staged());
    assert!(!inspected.plan_digest_joined_to_prepared_claim());
    assert!(!inspected.builder_and_verifier_isolation_observed());
    assert!(!inspected.immutable_evidence_collected());
    assert!(!inspected.store_provisioned());
    assert!(!inspected.seccomp_applied());
    assert!(!inspected.typed_final_freeze_bound());
    assert!(!inspected.launch_grant_available());
    assert!(!inspected.launch_performed());
    assert!(!inspected.wall_clock_verified());

    assert!(plan.authority.is_fully_closed());
    assert_eq!(plan.schema, NIX_CLOSED_RUN_PLAN_SCHEMA);
    assert_eq!(plan.schema_version, 2);
    assert_eq!(
        plan.successor_receipt_identity_contract,
        NixSuccessorReceiptIdentityContractV2 {
            boot_id_sha256: plan.binding.boot_id_sha256.clone(),
            legacy_candidate_evidence_v1_accepted: false,
            receipt_schema: NIX_SUCCESSOR_RECEIPT_SCHEMA.to_string(),
            receipt_schema_version: NIX_SUCCESSOR_RECEIPT_SCHEMA_VERSION,
            run_identity_algorithm: NIX_SUCCESSOR_RUN_IDENTITY_ALGORITHM.to_string(),
            run_identity_schema: NIX_SUCCESSOR_RUN_IDENTITY_SCHEMA.to_string(),
            run_identity_sha256: plan.binding.run_identity_sha256.clone(),
            run_nonce_sha256: plan.binding.run_nonce_sha256.clone(),
        }
    );
    let builder = plan.builder_container.as_ref().expect("sandbox builder");
    let verifier = &plan.verifier_container;
    assert_eq!(builder.role, ClosedContainerRoleV1::Builder);
    assert_eq!(verifier.role, ClosedContainerRoleV1::ReadOnlyVerifierSmoke);
    assert!(!builder.named_volume_mounts[0].read_only);
    assert!(verifier.named_volume_mounts[0].read_only);
    assert!(!builder.named_volume_mounts[0].create_if_missing);
    assert!(builder.named_volume_mounts[0].must_exist_exact_identity);
    assert!(builder.named_volume_mounts[0].docker_inspect_exact_identity_before_execute);
    assert_eq!(verifier.user, "65532:65532");
    assert!(
        verifier
            .environment
            .iter()
            .any(|value| value == "USER=hepta")
    );
    assert!(
        verifier
            .environment
            .iter()
            .any(|value| value == "LOGNAME=hepta")
    );
    assert!(
        verifier
            .environment
            .iter()
            .any(|value| value == "HOME=/tmp/home")
    );
    assert!(
        verifier
            .environment
            .iter()
            .all(|value| !value.contains("/root/.nix-profile"))
    );
    for container in [builder, verifier] {
        assert_eq!(container.image, PINNED_IMAGE);
        assert_eq!(
            container.image_config_id_sha256,
            plan.binding.docker_platform_config_image_id_sha256
        );
        assert_eq!(container.image_manifest_sha256, PINNED_IMAGE_SHA256);
        assert!(container.image_manifest_and_config_read_back_exact);
        assert!(container.docker_create_response_id_retained);
        assert!(container.docker_inspect_before_execute);
        assert!(container.labels_read_back_exact);
        assert_eq!(container.image_pull_policy, ClosedImagePullPolicyV1::Never);
        assert_eq!(
            container.command_executable,
            "/driver/codex-hepta-nix-mnl-driver-v1"
        );
        assert_eq!(
            container.command_arguments,
            [
                "--closed-plan",
                "/run/hepta/closed-plan.json",
                "--closed-plan-sha256-file",
                "/run/hepta/closed-plan.sha256",
                "--container-role",
                if container.role == ClosedContainerRoleV1::Builder {
                    "builder"
                } else {
                    "verifier"
                },
            ]
        );
        assert!(!container.inherit_environment);
        assert!(container.network_disabled);
        assert_eq!(container.network_mode, "none");
        assert_eq!(container.cap_drop, ["ALL"]);
        assert!(container.cap_add.is_empty());
        assert!(container.read_only_rootfs);
        assert!(container.no_new_privileges);
        assert!(!container.privileged);
        assert!(!container.host_pid);
        assert!(!container.host_ipc);
        assert!(container.devices.is_empty());
        assert!(container.ports.is_empty());
        assert!(container.dns.is_empty());
        assert!(container.extra_hosts.is_empty());
        assert!(container.candidate_can_read_closed_plan);
        assert!(!container.candidate_can_read_evidence);
        assert!(!container.candidate_can_read_profile);
        assert!(!container.candidate_can_read_replay_store);
        assert!(!container.candidate_can_read_copy_store);
        assert_eq!(container.bind_mounts.len(), 4);
        assert!(container.bind_mounts.iter().all(|mount| mount.read_only));
        assert!(container.bind_mounts.iter().all(|mount| {
            mount.host_uid == 0
                && mount.host_gid == 0
                && mount.retained_host_inode_through_container_start
                && mount.immediate_pre_start_revalidate_canonical_path_inode_mode_uid_gid
                && mount.docker_inspect_read_back_exact
        }));
        assert_eq!(container.named_volume_mounts.len(), 1);
        assert!(container.named_volume_mounts[0].no_copy);
        assert!(
            container.named_volume_mounts[0]
                .immediate_pre_start_revalidate_name_driver_options_labels_mountpoint_identity
        );
        assert_eq!(container.named_volume_mounts[0].driver, "local");
        assert!(container.named_volume_mounts[0].driver_options.is_empty());
        assert!(
            container.named_volume_mounts[0]
                .labels
                .iter()
                .any(|label| matches!(
                    label,
                    ClosedDockerLabelV1::ExternalVerifiedPlanDigest { key }
                        if key == "hepta.mnl.plan-sha256"
                ))
        );
        assert_eq!(
            container.resources.memory_limit_bytes,
            8 * 1024 * 1024 * 1024
        );
        assert_eq!(container.resources.nano_cpus, 1_000_000_000);
        assert_eq!(container.resources.pids_limit, 256);
        assert_eq!(container.security_options.len(), 2);
        assert_eq!(
            container.tmpfs_mounts[0].uid,
            if container.role == ClosedContainerRoleV1::Builder {
                0
            } else {
                65_532
            }
        );
        assert_eq!(container.tmpfs_mounts[0].gid, container.tmpfs_mounts[0].uid);
        assert!(container.labels.iter().any(|label| matches!(
            label,
            ClosedDockerLabelV1::Literal { key, value }
                if key == "hepta.mnl.role"
                    && value == if container.role == ClosedContainerRoleV1::Builder { "builder" } else { "verifier" }
        )));
    }
    assert!(plan.source_materialization.destination_must_not_exist);
    assert!(plan.source_materialization.read_back_tree_manifest);
    assert!(plan.source_materialization.read_only_after_materialization);
    assert!(plan.plan_delivery.mount_read_only);
    assert!(plan.plan_delivery.atomic_no_replace_publish_after_census);
    assert!(
        plan.plan_delivery
            .candidate_recomputes_external_digest_before_first_stage
    );
    assert!(
        plan.plan_delivery
            .retain_published_inode_through_container_start
    );
    assert!(
        plan.plan_delivery
            .immediate_pre_start_revalidate_plan_and_sidecar_canonical_path_inode_mode_uid_gid
    );
    assert!(
        plan.plan_delivery
            .sidecar_bytes_derived_only_from_joined_prepared_claim
    );
    assert!(plan.plan_delivery.sidecar_equals_joined_plan_digest);
    assert!(
        plan.plan_delivery
            .sidecar_candidate_compares_before_any_container_stage
    );
    assert!(plan.plan_delivery.sidecar_reopen_and_read_back_exact_bytes);
    assert!(plan.plan_delivery.plan_must_not_contain_own_digest);
    assert!(
        plan.plan_delivery
            .supervisor_reopens_and_verifies_exact_bytes
    );
    assert!(plan.seccomp_input.docker_request_uses_exact_verified_bytes);
    assert!(plan.artifact_staging.atomic_no_replace_publish);
    assert!(
        plan.artifact_staging
            .external_sources_openat2_no_symlink_no_magiclink_no_xdev
    );
    assert!(
        plan.artifact_staging
            .retain_external_source_inodes_until_publish
    );
    assert!(plan.artifact_staging.reopen_and_read_back_exact_bytes);
    assert!(plan.artifact_staging.retain_published_inodes_until_terminal);
    assert_eq!(plan.artifact_staging.directories.len(), 6);
    let expected_directories = [
        (
            ClosedRunDirectoryRoleV1::RunRoot,
            plan.run_root.clone(),
            ClosedRunStageKindV1::RunRootEstablishedAndActiveMarkerPublished,
            "0700",
        ),
        (
            ClosedRunDirectoryRoleV1::InputArtifacts,
            format!("{}/input", plan.run_root),
            ClosedRunStageKindV1::ArtifactsStaged,
            "0555",
        ),
        (
            ClosedRunDirectoryRoleV1::ClosedPlanControl,
            format!("{}/control", plan.run_root),
            ClosedRunStageKindV1::ClosedPlanPublishedAndReadBack,
            "0555",
        ),
        (
            ClosedRunDirectoryRoleV1::SupervisorTools,
            format!("{}/supervisor", plan.run_root),
            ClosedRunStageKindV1::ArtifactsStaged,
            "0555",
        ),
        (
            ClosedRunDirectoryRoleV1::DriverTools,
            format!("{}/driver", plan.run_root),
            ClosedRunStageKindV1::ArtifactsStaged,
            "0555",
        ),
        (
            ClosedRunDirectoryRoleV1::MaterializedSource,
            format!("{}/source", plan.run_root),
            ClosedRunStageKindV1::SourceMaterialized,
            "0555",
        ),
    ];
    for (directory, (role, host_path, stage, final_mode)) in plan
        .artifact_staging
        .directories
        .iter()
        .zip(expected_directories)
    {
        assert_eq!(directory.role, role);
        assert_eq!(directory.host_path, host_path);
        assert_eq!(directory.create_at_stage, stage);
        assert_eq!(directory.create_uid, 0);
        assert_eq!(directory.create_gid, 0);
        assert_eq!(directory.create_mode, "0700");
        assert_eq!(directory.final_mode, final_mode);
        assert!(directory.must_not_exist_before_create);
        assert!(directory.openat2_no_symlink_no_magiclink_no_xdev);
        assert!(directory.final_mode_applied_by_retained_fd_before_first_use);
        assert!(directory.inode_preserved_across_final_mode_application);
        assert!(directory.fsync_parent_after_create);
        assert!(directory.fsync_directory_after_final_mode);
        assert!(directory.reopen_and_verify_uid_gid_mode_inode_before_first_use);
        assert!(directory.retain_inode_until_terminal);
    }
    assert_eq!(plan.artifact_staging.artifacts.len(), 7);
    assert!(plan.artifact_staging.artifacts.iter().all(|artifact| {
        artifact
            .external_freeze_path
            .starts_with("/data/hepta-nix-mnl-v1/frozen-artifacts/")
            && artifact.host_path.starts_with(&plan.run_root)
            && artifact.external_source_uid == 0
            && artifact.external_source_gid == 0
            && artifact.staged_destination_uid == 0
            && artifact.staged_destination_gid == 0
            && artifact.staged_destination_mode_equals_pin
    }));
    assert!(
        plan.freshness_prerequisite
            .exact_inspected_plan_digest_joined_to_prepared_claim
    );
    assert!(
        plan.freshness_prerequisite
            .indivisible_clock_publication_clock_sequence
    );
    assert!(
        plan.freshness_prerequisite
            .claim_publication_and_post_clock_deferred_until_launch_ready
    );
    assert!(
        plan.freshness_prerequisite
            .immediate_launch_is_same_internal_state_machine_step
    );
    assert!(
        plan.freshness_prerequisite
            .post_publication_failure_is_uncertain_and_nonce_stays_consumed
    );
    assert!(
        plan.freshness_prerequisite
            .subsequent_verifier_start_requires_retained_nonserializable_run_state
    );
    assert_eq!(plan.global_exclusion.active_candidate_containers_before, 0);
    assert_eq!(plan.global_exclusion.active_named_volumes_before, 0);
    assert!(plan.global_exclusion.lock_retained_until_terminal);
    assert_eq!(
        plan.global_exclusion.lock_parent_host_path,
        "/data/hepta-nix-mnl-v1/locks"
    );
    assert_eq!(plan.global_exclusion.lock_parent_mode, "0700");
    assert!(
        plan.global_exclusion
            .lock_parent_openat2_no_symlink_no_magiclink_no_xdev
    );
    assert!(
        plan.global_exclusion
            .lock_path_opened_from_retained_parent_fd
    );
    assert!(
        plan.global_exclusion
            .lock_regular_file_mode_owner_inode_verified_before_flock
    );
    assert!(
        plan.global_exclusion
            .lock_existing_file_never_truncated_or_replaced
    );
    assert!(
        plan.global_exclusion
            .active_run_marker_bytes_derived_only_from_joined_prepared_claim
    );
    assert_eq!(plan.global_exclusion.active_run_marker_mode, "0600");
    assert_eq!(plan.global_exclusion.active_run_marker_uid, 0);
    assert_eq!(plan.global_exclusion.active_run_marker_gid, 0);
    assert!(plan.global_exclusion.active_run_marker_file_fsync);
    assert!(plan.global_exclusion.active_run_marker_directory_fsync);
    assert!(
        plan.global_exclusion
            .active_run_marker_reopen_and_read_back_exact_bytes
    );
    assert!(
        plan.global_exclusion
            .active_run_marker_retained_inode_until_terminal
    );
    assert!(plan.nix_store_provisioning.no_image_copy_up);
    assert!(
        plan.nix_store_provisioning
            .pre_builder_inventory_retained_before_freshness_barrier
    );
    assert!(
        plan.nix_store_provisioning
            .fresh_product_check_and_target_derivation_outputs_must_be_absent_from_pre_builder_inventory
    );
    assert_eq!(
        plan.nix_store_provisioning.seed_bundle_format,
        "zstd_single_frame_nar_stream_set_v1"
    );
    assert!(
        plan.nix_store_provisioning
            .reopen_same_volume_read_only_before_verifier
    );
    assert!(!plan.evidence_collection.candidate_self_report_authoritative);
    assert!(
        plan.evidence_collection
            .network_access_prevented_by_enforcement
    );
    assert!(
        !plan
            .evidence_collection
            .network_attempted_receipt_field_available
    );
    assert!(
        plan.evidence_collection
            .lossless_network_attempt_observer_required_before_receipt
    );
    assert!(
        !plan
            .evidence_collection
            .state_mutated_receipt_field_available
    );
    assert!(
        plan.evidence_collection
            .exact_state_surface_pre_post_inventory_required_before_receipt
    );
    assert!(
        plan.evidence_collection
            .product_and_check_output_paths_are_distinct
    );
    assert!(plan.evidence_collection.fresh_check_output_required);
    assert!(
        plan.evidence_collection
            .fresh_realization_proven_after_immediate_launch
    );
    assert!(
        plan.dynamic_output_handoff
            .fresh_outputs_and_target_derivation_outputs_checked_absent_from_pre_builder_inventory
    );
    assert!(
        plan.dynamic_output_handoff
            .fresh_builder_transcript_exit_status_and_no_truncation_bound_before_accept
    );
    assert!(
        !plan
            .dynamic_output_handoff
            .presealed_signed_product_and_check_bindings_verified_before_verifier_create
    );
    assert!(
        plan.dynamic_output_handoff
            .fresh_retained_state_also_binds_builder_container_id
    );
    assert_eq!(
        plan.dynamic_output_handoff.handoff_source,
        ClosedOutputHandoffSourceV1::FreshBuilderAndCheckTranscripts
    );
    assert!(
        plan.dynamic_output_handoff
            .qualified_tokens_resolved_only_from_retained_supervisor_state
    );
    assert!(
        plan.dynamic_output_handoff
            .docker_exec_argv_constructed_by_supervisor_without_shell
    );
    assert_eq!(
        plan.evidence_collection.product_output_path_source,
        ClosedOutputPathSourceV1::FreshBuildStdoutSingleCanonicalStorePath
    );
    assert_eq!(
        plan.evidence_collection.check_output_path_source,
        ClosedOutputPathSourceV1::RealChecksStdoutSingleCanonicalStorePath
    );
    assert!(
        !plan
            .evidence_collection
            .presealed_check_provenance_is_historical_and_not_fresh
    );
    assert!(
        !plan
            .evidence_collection
            .presealed_signed_closure_digest_must_equal_recomputed_union_inventory
    );
    assert!(
        plan.evidence_collection
            .binary_retained_fd_pre_and_post_smoke_read_back
    );
    assert!(!plan.failure_policy.automatic_cleanup);
    assert!(!plan.failure_policy.automatic_retry);
    assert!(!plan.failure_policy.fallback_isolation_mode);
    assert!(plan.failure_policy.post_launch_unknown_is_uncertain);
    assert!(plan.failure_policy.preserve_container_and_volume_on_failure);
    assert_eq!(plan.verifier_docker_exec_policy.user, "65532:65532");
    assert!(!plan.verifier_docker_exec_policy.privileged);
    assert!(!plan.verifier_docker_exec_policy.start_detach);
    assert!(plan.verifier_docker_exec_policy.start_tty_matches_create);
    assert!(!plan.verifier_docker_exec_policy.tty);
    assert!(!plan.verifier_docker_exec_policy.attach_stdin);
    assert!(plan.verifier_docker_exec_policy.attach_stdout);
    assert!(plan.verifier_docker_exec_policy.attach_stderr);
    assert!(
        plan.verifier_docker_exec_policy
            .environment_overrides
            .is_empty()
    );
    assert!(!plan.verifier_docker_exec_policy.inherit_caller_environment);
    assert_eq!(
        plan.tool_execution.driver_container_entrypoint_path,
        "/driver/codex-hepta-nix-mnl-driver-v1"
    );
    assert!(
        plan.tool_execution
            .runner_bootstrap_external_freeze_path
            .starts_with("/data/hepta-nix-mnl-v1/frozen-artifacts/")
    );
    assert!(
        plan.tool_execution
            .bootstrap_runner_inode_retained_before_preflight
    );
    assert_eq!(
        plan.tool_execution.bootstrap_runner_pin_authority_source,
        ClosedBootstrapPinAuthoritySourceV1::FutureTypedVerifiedFinalArtifactFreezeInspectionOnly
    );
    assert!(
        plan.tool_execution
            .bootstrap_runner_pin_never_from_closed_plan_or_caller
    );
    assert!(
        plan.tool_execution
            .bootstrap_runner_exec_forbidden_when_typed_final_freeze_unavailable
    );
    assert_eq!(
        plan.tool_execution.fresh_driver_builder_stages,
        [
            ClosedRunStageKindV1::FreshBuild,
            ClosedRunStageKindV1::RealChecks,
        ]
    );
    assert!(
        plan.tool_execution
            .driver_verifies_mounted_plan_and_sidecar_before_any_container_stage
    );
    assert!(
        !plan
            .tool_execution
            .collector_helper_stages
            .contains(&ClosedRunStageKindV1::Preflight)
    );
    assert!(
        !plan
            .tool_execution
            .verifier_helper_stages
            .contains(&ClosedRunStageKindV1::SignedPlanJoinedToPreparedClaim)
    );
    assert_eq!(
        plan.tool_execution.collector_helper_stages,
        [
            ClosedRunStageKindV1::BuilderIsolationVerified,
            ClosedRunStageKindV1::VerifierIsolationVerified,
            ClosedRunStageKindV1::PreSmokeReadBack,
            ClosedRunStageKindV1::PostSmokeReadBack,
        ]
    );
    assert_eq!(
        plan.tool_execution.verifier_helper_stages,
        [
            ClosedRunStageKindV1::BuilderOutputsRetainedAndQualified,
            ClosedRunStageKindV1::ImmutableEvidenceCollected,
            ClosedRunStageKindV1::EvidenceClosed,
        ]
    );

    let stage_kinds: Vec<_> = plan.stages.iter().map(|stage| stage.kind).collect();
    assert_eq!(
        stage_kinds,
        [
            ClosedRunStageKindV1::Preflight,
            ClosedRunStageKindV1::SignedPlanJoinedToPreparedClaim,
            ClosedRunStageKindV1::ExclusiveLock,
            ClosedRunStageKindV1::CensusClear,
            ClosedRunStageKindV1::RunRootEstablishedAndActiveMarkerPublished,
            ClosedRunStageKindV1::ArtifactsStaged,
            ClosedRunStageKindV1::ClosedPlanPublishedAndReadBack,
            ClosedRunStageKindV1::SourceMaterialized,
            ClosedRunStageKindV1::NixStoreProvisioned,
            ClosedRunStageKindV1::BuilderCreatedAndInspected,
            ClosedRunStageKindV1::BuilderIsolationVerified,
            ClosedRunStageKindV1::PreRunClaimPublishedClockRecheckedAndImmediateLaunch,
            ClosedRunStageKindV1::FreshBuild,
            ClosedRunStageKindV1::RealChecks,
            ClosedRunStageKindV1::BuilderOutputsRetainedAndQualified,
            ClosedRunStageKindV1::BuilderStoppedNoDescendants,
            ClosedRunStageKindV1::StoreReopenedReadOnly,
            ClosedRunStageKindV1::VerifierCreatedAndInspected,
            ClosedRunStageKindV1::VerifierIsolationVerified,
            ClosedRunStageKindV1::VerifierStartedWithinRetainedRunState,
            ClosedRunStageKindV1::ReadOnlyArtifactVerification,
            ClosedRunStageKindV1::ImmutableEvidenceCollected,
            ClosedRunStageKindV1::PreSmokeReadBack,
            ClosedRunStageKindV1::RuntimeSmoke,
            ClosedRunStageKindV1::PostSmokeReadBack,
            ClosedRunStageKindV1::VerifierStoppedNoDescendants,
            ClosedRunStageKindV1::EvidenceClosed,
        ]
    );
    assert!(
        plan.stages
            .iter()
            .flat_map(|stage| &stage.commands)
            .all(|command| {
                !command.inherit_environment
                    && command.stdin_closed
                    && command.truncation_is_failure
                    && match command.stdout_handling {
                        ClosedStdoutHandlingV1::BoundedCapture { maximum_bytes }
                        | ClosedStdoutHandlingV1::StreamingSha256ToSupervisor {
                            maximum_bytes,
                            ..
                        } => maximum_bytes > 0,
                    }
                    && command.stderr_limit_bytes > 0
            })
    );
    let streaming: Vec<_> = plan
        .dynamic_evidence_traversal
        .closure_member_commands
        .iter()
        .filter(|command| {
            matches!(
                command.stdout_handling,
                ClosedStdoutHandlingV1::StreamingSha256ToSupervisor { .. }
            )
        })
        .collect();
    assert_eq!(streaming.len(), 1);
    assert!(matches!(
        streaming[0].stdout_handling,
        ClosedStdoutHandlingV1::StreamingSha256ToSupervisor {
            maximum_bytes: 68_719_476_736,
            require_eof: true,
        }
    ));
    assert!(
        plan.dynamic_evidence_traversal
            .closure_member_paths_sorted_unique_bytewise
    );
    assert!(
        plan.dynamic_evidence_traversal
            .closure_requisites_must_include_product_output
    );
    assert!(
        plan.dynamic_evidence_traversal
            .closure_requisites_must_include_check_output
    );
    assert!(
        plan.dynamic_evidence_traversal
            .closure_inventory_is_sorted_unique_union_of_product_and_check_requisites
    );
    assert_eq!(
        plan.dynamic_evidence_traversal.executor,
        ClosedRunExecutorV1::SupervisorDockerExecIntoReadOnlyVerifier
    );
    assert_eq!(
        plan.dynamic_evidence_traversal.stage_kind,
        ClosedRunStageKindV1::ImmutableEvidenceCollected
    );
    assert!(
        plan.dynamic_evidence_traversal
            .inputs_from_preceding_artifact_verification_retained_output
    );
    assert_eq!(
        plan.stages
            .iter()
            .find(|stage| stage.kind == ClosedRunStageKindV1::ImmutableEvidenceCollected)
            .expect("dynamic evidence stage")
            .executor,
        ClosedRunExecutorV1::SupervisorDockerExecIntoReadOnlyVerifier
    );
    assert_eq!(
        plan.dynamic_evidence_traversal.maximum_closure_members,
        100_000
    );
    assert_eq!(
        plan.dynamic_evidence_traversal
            .product_derivation_commands
            .len(),
        1
    );
    assert_eq!(
        plan.dynamic_evidence_traversal
            .check_derivation_commands
            .len(),
        1
    );
    assert!(matches!(
        plan.dynamic_evidence_traversal.product_derivation_commands[0].stdout_handling,
        ClosedStdoutHandlingV1::BoundedCapture {
            maximum_bytes: 16_777_216,
        }
    ));
    assert!(
        plan.stages
            .iter()
            .flat_map(|stage| &stage.commands)
            .all(|command| match &command.executable {
                ClosedExecutableV1::FixedAbsolute { path } => path.starts_with('/'),
                ClosedExecutableV1::QualifiedProductOutputRelative { relative_path } => {
                    relative_path == "bin/codex"
                }
            })
    );
}

#[test]
fn successor_receipt_v2_identity_is_shared_exact_and_never_falls_back_to_v1() {
    let plan = sandbox_plan();
    let contract = &plan.successor_receipt_identity_contract;
    let shared = codex_hepta_mnl_trust_v1::derive_run_identity_sha256(
        &contract.run_nonce_sha256,
        &contract.boot_id_sha256,
    )
    .expect("shared successor run identity");
    let legacy = crate::verify::legacy_receipt_run_identity_sha256(
        &contract.run_nonce_sha256,
        &contract.boot_id_sha256,
    )
    .expect("frozen V1 run identity");

    assert_eq!(contract.run_identity_sha256, shared);
    assert_ne!(contract.run_identity_sha256, legacy);
    assert_eq!(contract.receipt_schema_version, 2);
    assert_eq!(
        contract.run_identity_algorithm,
        NIX_SUCCESSOR_RUN_IDENTITY_ALGORITHM
    );
    assert!(!contract.legacy_candidate_evidence_v1_accepted);
}

#[test]
fn sandbox_and_presealed_plan_byte_goldens_are_stable() {
    let sandbox_bytes = serde_json::to_vec(&sandbox_plan()).expect("sandbox plan bytes");
    assert_eq!(sandbox_bytes.len(), TEST_SANDBOX_PLAN_BYTE_COUNT);
    assert_eq!(sha256(&sandbox_bytes), TEST_SANDBOX_PLAN_SHA256);

    let mut presealed_binding = binding();
    presealed_binding.isolation_mode = NixIsolationModeV1::PresealedOfflineClosure;
    presealed_binding.presealed_offline_closure_sha256 = Some(digest('a'));
    presealed_binding.presealed_output_store_path =
        Some("/nix/store/abcdfghijklmnpqrsvwxyz0123456789-codex-rs-0.0.0".to_string());
    presealed_binding.presealed_check_output_store_path =
        Some("/nix/store/bbcdfghijklmnpqrsvwxyz0123456789-codex-rs-checks-0.0.0".to_string());
    let presealed = derive_nix_closed_run_plan(presealed_binding).expect("presealed plan");
    let presealed_bytes = serde_json::to_vec(&presealed).expect("presealed plan bytes");
    assert_eq!(presealed_bytes.len(), TEST_PRESEALED_PLAN_BYTE_COUNT);
    assert_eq!(sha256(&presealed_bytes), TEST_PRESEALED_PLAN_SHA256);
}

#[test]
fn presealed_plan_is_read_only_not_a_fresh_build_and_has_no_fallback() {
    let mut binding = binding();
    binding.isolation_mode = NixIsolationModeV1::PresealedOfflineClosure;
    binding.presealed_offline_closure_sha256 = Some(digest('e'));
    binding.presealed_output_store_path =
        Some("/nix/store/abcdfghijklmnpqrsvwxyz0123456789-codex-rs-0.0.0".to_string());
    binding.presealed_check_output_store_path =
        Some("/nix/store/bbcdfghijklmnpqrsvwxyz0123456789-codex-rs-checks-0.0.0".to_string());
    let plan = derive_nix_closed_run_plan(binding).expect("presealed closed plan");
    let inspected = inspect_canonical_nix_closed_run_plan(
        &serde_json::to_vec(&plan).expect("canonical presealed plan"),
    )
    .expect("presealed inspection");

    assert_eq!(
        inspected.disposition(),
        ClosedRunPlanDispositionV1::PresealedOfflineArtifactInspectionOnlyNotFreshBuild
    );
    assert!(plan.builder_container.is_none());
    assert!(plan.verifier_container.named_volume_mounts[0].read_only);
    assert!(!plan.failure_policy.fallback_isolation_mode);
    assert!(!plan.evidence_collection.fresh_check_output_required);
    assert!(
        !plan
            .nix_store_provisioning
            .pre_builder_inventory_retained_before_freshness_barrier
    );
    assert_eq!(
        plan.dynamic_output_handoff.handoff_source,
        ClosedOutputHandoffSourceV1::SignedPresealedProductAndCheckBindings
    );
    assert!(
        !plan
            .dynamic_output_handoff
            .fresh_builder_transcript_exit_status_and_no_truncation_bound_before_accept
    );
    assert!(
        plan.dynamic_output_handoff
            .presealed_signed_product_and_check_bindings_verified_before_verifier_create
    );
    assert!(
        !plan
            .dynamic_output_handoff
            .fresh_retained_state_also_binds_builder_container_id
    );
    assert!(
        !plan
            .freshness_prerequisite
            .subsequent_verifier_start_requires_retained_nonserializable_run_state
    );
    assert!(plan.tool_execution.fresh_driver_builder_stages.is_empty());
    assert_eq!(
        plan.tool_execution.collector_helper_stages,
        [
            ClosedRunStageKindV1::VerifierIsolationVerified,
            ClosedRunStageKindV1::PreSmokeReadBack,
            ClosedRunStageKindV1::PostSmokeReadBack,
        ]
    );
    assert_eq!(
        plan.tool_execution.verifier_helper_stages,
        [
            ClosedRunStageKindV1::PresealedOutputBindingsRetainedAndQualified,
            ClosedRunStageKindV1::ImmutableEvidenceCollected,
            ClosedRunStageKindV1::EvidenceClosed,
        ]
    );
    assert!(
        plan.nix_store_provisioning
            .reopen_same_volume_read_only_before_verifier
    );
    assert!(
        !plan
            .evidence_collection
            .fresh_realization_proven_after_immediate_launch
    );
    assert!(
        plan.evidence_collection
            .presealed_check_provenance_is_historical_and_not_fresh
    );
    assert!(
        plan.evidence_collection
            .presealed_product_and_check_paths_must_be_closure_members
    );
    assert!(
        plan.evidence_collection
            .presealed_signed_closure_digest_must_equal_recomputed_union_inventory
    );
    assert_eq!(
        plan.evidence_collection.product_output_path_source,
        ClosedOutputPathSourceV1::SignedPresealedBindingExactCanonicalStorePath
    );
    assert_eq!(
        plan.evidence_collection.check_output_path_source,
        ClosedOutputPathSourceV1::SignedPresealedCheckOutputBindingExactCanonicalStorePath
    );
    assert!(
        plan.stages
            .iter()
            .any(|stage| { stage.kind == ClosedRunStageKindV1::PresealedArtifactVerification })
    );
    assert!(!plan.stages.iter().any(|stage| {
        matches!(
            stage.kind,
            ClosedRunStageKindV1::FreshBuild | ClosedRunStageKindV1::RealChecks
        )
    }));
    assert!(
        plan.stages
            .iter()
            .find(|stage| stage.kind == ClosedRunStageKindV1::PresealedArtifactVerification)
            .expect("presealed verification stage")
            .commands
            .iter()
            .flat_map(|command| &command.arguments)
            .any(|argument| matches!(argument, ClosedArgumentV1::QualifiedCheckOutputStorePath))
    );
    assert_eq!(
        plan.stages
            .iter()
            .map(|stage| stage.kind)
            .collect::<Vec<_>>(),
        [
            ClosedRunStageKindV1::Preflight,
            ClosedRunStageKindV1::SignedPlanJoinedToPreparedClaim,
            ClosedRunStageKindV1::ExclusiveLock,
            ClosedRunStageKindV1::CensusClear,
            ClosedRunStageKindV1::RunRootEstablishedAndActiveMarkerPublished,
            ClosedRunStageKindV1::ArtifactsStaged,
            ClosedRunStageKindV1::ClosedPlanPublishedAndReadBack,
            ClosedRunStageKindV1::SourceMaterialized,
            ClosedRunStageKindV1::NixStoreProvisioned,
            ClosedRunStageKindV1::StoreReopenedReadOnly,
            ClosedRunStageKindV1::PresealedOutputBindingsRetainedAndQualified,
            ClosedRunStageKindV1::VerifierCreatedAndInspected,
            ClosedRunStageKindV1::VerifierIsolationVerified,
            ClosedRunStageKindV1::PreRunClaimPublishedClockRecheckedAndImmediateLaunch,
            ClosedRunStageKindV1::PresealedArtifactVerification,
            ClosedRunStageKindV1::ImmutableEvidenceCollected,
            ClosedRunStageKindV1::PreSmokeReadBack,
            ClosedRunStageKindV1::RuntimeSmoke,
            ClosedRunStageKindV1::PostSmokeReadBack,
            ClosedRunStageKindV1::VerifierStoppedNoDescendants,
            ClosedRunStageKindV1::EvidenceClosed,
        ]
    );
    assert!(!inspected.authorizes_live());
}

#[test]
fn plan_bytes_exclude_self_profile_claim_publication_and_downstream_digests() {
    let bytes = serde_json::to_vec(&sandbox_plan()).expect("canonical plan");
    let text = std::str::from_utf8(&bytes).expect("plan UTF-8");
    for forbidden in [
        "platform_closed_run_plan_sha256",
        "pre_run_profile_manifest_sha256",
        "pre_run_profile_payload_sha256",
        "pre_run_profile_signature_sha256",
        "pre_run_profile_signed_frame_sha256",
        "pre_run_full_binding_sha256",
        "replay_slot_sha256",
        "durable_publication",
        "receipt_set_sha256",
        "seal_event_sha256",
        "copy_ack",
        "terminal_manifest",
    ] {
        assert!(
            !text.contains(forbidden),
            "forbidden self/downstream binding: {forbidden}"
        );
    }
}

#[test]
fn every_exact_execution_and_directory_stage_exists_in_its_selected_mode() {
    let sandbox = sandbox_plan();
    let mut presealed_binding = binding();
    presealed_binding.isolation_mode = NixIsolationModeV1::PresealedOfflineClosure;
    presealed_binding.presealed_offline_closure_sha256 = Some(digest('a'));
    presealed_binding.presealed_output_store_path =
        Some("/nix/store/abcdfghijklmnpqrsvwxyz0123456789-hepta-product-v1".to_string());
    presealed_binding.presealed_check_output_store_path =
        Some("/nix/store/bbcdfghijklmnpqrsvwxyz0123456789-hepta-check-v1".to_string());
    let presealed =
        derive_nix_closed_run_plan(presealed_binding).expect("presealed closed plan stages");

    for plan in [&sandbox, &presealed] {
        let present: Vec<_> = plan.stages.iter().map(|stage| stage.kind).collect();
        for referenced in plan
            .tool_execution
            .collector_helper_stages
            .iter()
            .chain(&plan.tool_execution.verifier_helper_stages)
            .chain(&plan.tool_execution.fresh_driver_builder_stages)
            .chain(
                plan.artifact_staging
                    .directories
                    .iter()
                    .map(|directory| &directory.create_at_stage),
            )
            .chain([&plan.dynamic_evidence_traversal.stage_kind])
        {
            assert!(
                present.contains(referenced),
                "exact execution map references absent stage {referenced:?}"
            );
        }
    }
}

#[test]
fn canonical_bounds_unknown_trailing_pretty_and_duplicate_fields_are_rejected() {
    let plan = sandbox_plan();
    let canonical = serde_json::to_vec(&plan).expect("canonical plan");
    let pretty = serde_json::to_vec_pretty(&plan).expect("pretty plan");
    assert!(inspect_canonical_nix_closed_run_plan(&pretty).is_err());
    assert!(inspect_canonical_nix_closed_run_plan(&[]).is_err());

    let mut trailing = canonical.clone();
    trailing.push(b'\n');
    assert!(inspect_canonical_nix_closed_run_plan(&trailing).is_err());

    let mut unknown = serde_json::to_value(&plan).expect("plan value");
    unknown["launch"] = serde_json::json!(true);
    assert!(
        inspect_canonical_nix_closed_run_plan(
            &serde_json::to_vec(&unknown).expect("unknown field JSON")
        )
        .is_err()
    );

    let mut nested_unknown = serde_json::to_value(&plan).expect("plan value");
    nested_unknown["binding"]["caller_authority"] = serde_json::json!(true);
    assert!(
        inspect_canonical_nix_closed_run_plan(
            &serde_json::to_vec(&nested_unknown).expect("nested unknown field JSON")
        )
        .is_err()
    );

    let mut nested_missing = serde_json::to_value(&plan).expect("plan value");
    nested_missing["binding"]
        .as_object_mut()
        .expect("binding object")
        .remove("docker_platform_config_image_id_sha256");
    assert!(
        inspect_canonical_nix_closed_run_plan(
            &serde_json::to_vec(&nested_missing).expect("nested missing field JSON")
        )
        .is_err()
    );

    let duplicate = format!(
        "{{\"schema\":\"duplicate\",{}",
        std::str::from_utf8(&canonical)
            .expect("canonical UTF-8")
            .trim_start_matches('{')
    );
    assert!(inspect_canonical_nix_closed_run_plan(duplicate.as_bytes()).is_err());
    assert!(
        inspect_canonical_nix_closed_run_plan(&vec![b'x'; MAX_NIX_CLOSED_RUN_PLAN_BYTES + 1])
            .is_err()
    );
}

#[test]
fn every_redundantly_derived_binding_rejects_a_stale_plan() {
    let plan = sandbox_plan();
    macro_rules! reject {
        ($field:ident, $value:expr) => {{
            let mut changed = plan.clone();
            changed.binding.$field = $value;
            assert!(
                inspect_canonical_nix_closed_run_plan(
                    &serde_json::to_vec(&changed).expect("changed canonical plan")
                )
                .is_err(),
                "transplanted {} must fail",
                stringify!($field),
            );
        }};
    }

    reject!(boot_id_sha256, digest('1'));
    reject!(collector_binary, artifact('a', "0555"));
    reject!(cpuset_cpu, 25);
    reject!(docker_api_version, "1.48".to_string());
    reject!(docker_config_sha256, digest('3'));
    reject!(docker_platform_config_image_id_sha256, digest('4'));
    reject!(driver_binary, artifact('a', "0555"));
    reject!(host_identity_sha256, digest('5'));
    reject!(nix_store_seed_bundle, artifact('a', "0444"));
    reject!(nix_store_seed_inventory_sha256, digest('a'));
    reject!(profile_id, "other-pre-run-v1".to_string());
    reject!(run_nonce_sha256, digest('6'));
    reject!(runner_binary, artifact('a', "0555"));
    reject!(seccomp_profile, artifact('a', "0444"));
    reject!(source_archive, artifact('a', "0444"));
    reject!(source_tree_manifest_sha256, digest('a'));
    reject!(verifier_binary, artifact('a', "0555"));
}

#[test]
fn pre_sign_only_binding_variations_rederive_distinct_non_authorizing_plans() {
    let original = sandbox_plan();
    let original_bytes = serde_json::to_vec(&original).expect("original plan bytes");
    let original_inspection =
        inspect_canonical_nix_closed_run_plan(&original_bytes).expect("original inspection");
    let mut variations = Vec::new();
    let mut changed = binding();
    changed.challenge_nonce_sha256 = digest('1');
    variations.push(changed);
    let mut changed = binding();
    changed.final_artifact_freeze_payload_sha256 = digest('2');
    variations.push(changed);
    let mut changed = binding();
    changed.final_artifact_freeze_profile_id = "other-final-freeze-v1".to_string();
    variations.push(changed);
    let mut changed = binding();
    changed.final_tooling = RepositoryIdentityV1 {
        head: "c".repeat(40),
        tree: "d".repeat(40),
    };
    variations.push(changed);

    for variation in variations {
        let plan = derive_nix_closed_run_plan(variation).expect("changed closed plan");
        let bytes = serde_json::to_vec(&plan).expect("changed plan bytes");
        let inspection = inspect_canonical_nix_closed_run_plan(&bytes).expect("changed inspection");
        assert_ne!(
            original_inspection.closed_run_plan_sha256(),
            inspection.closed_run_plan_sha256()
        );
        assert!(!inspection.authorizes_live());
        assert!(!inspection.launch_grant_available());
    }
}

#[test]
fn static_container_command_mount_environment_resource_and_policy_drift_are_rejected() {
    let plan = sandbox_plan();
    let mut mutations = Vec::new();
    let mut changed = plan.clone();
    changed.builder_container.as_mut().expect("builder").image = "nixos/nix:latest".to_string();
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .command_executable = "/bin/sh".to_string();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .command_arguments = vec!["-c".to_string(), "true".to_string()];
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .bind_mounts[0]
        .read_only = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .bind_mounts
        .push(ClosedBindMountV1 {
            container_path: "/evidence".to_string(),
            docker_inspect_read_back_exact: true,
            host_gid: 0,
            host_mode: "0555".to_string(),
            host_path: "/data/evidence".to_string(),
            host_uid: 0,
            immediate_pre_start_revalidate_canonical_path_inode_mode_uid_gid: true,
            read_only: true,
            retained_host_inode_through_container_start: true,
        });
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .environment
        .push("HTTP_PROXY=http://caller".to_string());
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .network_disabled = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .cap_add
        .push("SYS_ADMIN".to_string());
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .host_pid = true;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .resources
        .pids_limit += 1;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .auto_remove = true;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.failure_policy.automatic_retry = true;
    mutations.push(changed);
    let mut changed = plan;
    changed.stages.swap(4, 5);
    mutations.push(changed);

    for changed in mutations {
        assert!(
            inspect_canonical_nix_closed_run_plan(
                &serde_json::to_vec(&changed).expect("mutated plan JSON")
            )
            .is_err()
        );
    }
}

#[test]
fn authority_isolation_materialization_and_evidence_drift_are_rejected() {
    let plan = sandbox_plan();
    let mut mutations = Vec::new();

    let mut changed = plan.clone();
    changed.authority.container_launch = true;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .successor_receipt_identity_contract
        .legacy_candidate_evidence_v1_accepted = true;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .successor_receipt_identity_contract
        .run_identity_sha256 = digest('0');
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.successor_receipt_identity_contract.receipt_schema =
        "hepta_nix_exact_mnl_successor_candidate_evidence_v1".to_string();
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .candidate_can_read_replay_store = true;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .cap_drop
        .clear();
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .no_new_privileges = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .read_only_rootfs = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.verifier_container.privileged = true;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.verifier_container.named_volume_mounts[0].read_only = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.source_materialization.read_back_tree_manifest = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .plan_delivery
        .supervisor_reopens_and_verifies_exact_bytes = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .plan_delivery
        .retain_published_inode_through_container_start = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.plan_delivery.sidecar_equals_joined_plan_digest = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.artifact_staging.atomic_no_replace_publish = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.artifact_staging.directories[4].final_mode = "0700".to_string();
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.artifact_staging.artifacts[0].external_source_uid = 501;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .freshness_prerequisite
        .exact_inspected_plan_digest_joined_to_prepared_claim = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .seccomp_input
        .docker_request_uses_exact_verified_bytes = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.global_exclusion.active_candidate_containers_before = 1;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.verifier_docker_exec_policy.user = "0:0".to_string();
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.verifier_docker_exec_policy.privileged = true;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.verifier_docker_exec_policy.start_detach = true;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.tool_execution.driver_container_entrypoint_path =
        changed.tool_execution.driver_staged_host_path.clone();
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .labels
        .clear();
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .builder_container
        .as_mut()
        .expect("builder")
        .image_config_id_sha256 = PINNED_IMAGE_SHA256.to_string();
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.nix_store_provisioning.no_image_copy_up = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.host_preflight.minimum_data_volume_free_bytes -= 1;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .evidence_collection
        .candidate_self_report_authoritative = true;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.evidence_algorithms.nar_stream = "caller_hashes_a_path_string".to_string();
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .dynamic_evidence_traversal
        .closure_member_paths_sorted_unique_bytewise = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .dynamic_evidence_traversal
        .closure_member_commands
        .pop();
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.failure_policy.automatic_cleanup = true;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed.failure_policy.post_launch_unknown_is_uncertain = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    changed
        .failure_policy
        .preserve_container_and_volume_on_failure = false;
    mutations.push(changed);
    let mut changed = plan.clone();
    let command = changed
        .stages
        .iter_mut()
        .flat_map(|stage| &mut stage.commands)
        .next()
        .expect("planned command");
    command.executable = ClosedExecutableV1::FixedAbsolute {
        path: "/bin/sh".to_string(),
    };
    mutations.push(changed);
    let mut changed = plan.clone();
    let command = changed
        .stages
        .iter_mut()
        .flat_map(|stage| &mut stage.commands)
        .next()
        .expect("planned command");
    command.inherit_environment = true;
    mutations.push(changed);
    let mut changed = plan;
    let command = changed
        .stages
        .iter_mut()
        .flat_map(|stage| &mut stage.commands)
        .next()
        .expect("planned command");
    command.truncation_is_failure = false;
    mutations.push(changed);

    for changed in mutations {
        assert!(
            inspect_canonical_nix_closed_run_plan(
                &serde_json::to_vec(&changed).expect("mutated closed plan")
            )
            .is_err()
        );
    }
}

#[test]
fn isolation_artifact_identity_and_shared_run_identity_must_be_exact() {
    let mut mixed = binding();
    mixed.presealed_offline_closure_sha256 = Some(digest('e'));
    assert!(derive_nix_closed_run_plan(mixed).is_err());

    let mut missing = binding();
    missing.isolation_mode = NixIsolationModeV1::PresealedOfflineClosure;
    assert!(derive_nix_closed_run_plan(missing).is_err());

    let mut wrong_mode = binding();
    wrong_mode.driver_binary.mode = "0755".to_string();
    assert!(derive_nix_closed_run_plan(wrong_mode).is_err());

    let mut wrong_run = binding();
    wrong_run.run_identity_sha256 = digest('f');
    assert!(derive_nix_closed_run_plan(wrong_run).is_err());

    let mut malformed_api = binding();
    malformed_api.docker_api_version = "latest".to_string();
    assert!(derive_nix_closed_run_plan(malformed_api).is_err());

    let mut zero_bytes = binding();
    zero_bytes.source_archive.byte_count = 0;
    assert!(derive_nix_closed_run_plan(zero_bytes).is_err());

    let mut oversized = binding();
    oversized.source_archive.byte_count = 4 * 1024 * 1024 * 1024 + 1;
    assert!(derive_nix_closed_run_plan(oversized).is_err());

    let mut zero_digest = binding();
    zero_digest.source_archive.sha256 = "0".repeat(64);
    assert!(derive_nix_closed_run_plan(zero_digest).is_err());

    let mut uppercase_digest = binding();
    uppercase_digest.docker_platform_config_image_id_sha256 = "A".repeat(64);
    assert!(derive_nix_closed_run_plan(uppercase_digest).is_err());

    let mut short_digest = binding();
    short_digest.docker_platform_config_image_id_sha256 = "a".repeat(63);
    assert!(derive_nix_closed_run_plan(short_digest).is_err());

    let mut conflated_image_digests = binding();
    conflated_image_digests.docker_platform_config_image_id_sha256 =
        PINNED_IMAGE_SHA256.to_string();
    assert!(derive_nix_closed_run_plan(conflated_image_digests).is_err());

    let mut uppercase_oid = binding();
    uppercase_oid.final_tooling.head = "A".repeat(40);
    assert!(derive_nix_closed_run_plan(uppercase_oid).is_err());

    let mut invalid_profile = binding();
    invalid_profile.profile_id = "Caller Profile".to_string();
    assert!(derive_nix_closed_run_plan(invalid_profile).is_err());

    let mut cpu_out_of_range = binding();
    cpu_out_of_range.cpuset_cpu = 4096;
    assert!(derive_nix_closed_run_plan(cpu_out_of_range).is_err());

    let mut wrong_seccomp_mode = binding();
    wrong_seccomp_mode.seccomp_profile.mode = "0555".to_string();
    assert!(derive_nix_closed_run_plan(wrong_seccomp_mode).is_err());

    let mut oversized_seed = binding();
    oversized_seed.nix_store_seed_bundle.byte_count = 64 * 1024 * 1024 * 1024 + 1;
    assert!(derive_nix_closed_run_plan(oversized_seed).is_err());

    let mut incomplete_presealed = binding();
    incomplete_presealed.isolation_mode = NixIsolationModeV1::PresealedOfflineClosure;
    incomplete_presealed.presealed_offline_closure_sha256 = Some(digest('a'));
    assert!(derive_nix_closed_run_plan(incomplete_presealed).is_err());

    let mut missing_presealed_check = binding();
    missing_presealed_check.isolation_mode = NixIsolationModeV1::PresealedOfflineClosure;
    missing_presealed_check.presealed_offline_closure_sha256 = Some(digest('a'));
    missing_presealed_check.presealed_output_store_path =
        Some("/nix/store/abcdfghijklmnpqrsvwxyz0123456789-codex-rs-0.0.0".to_string());
    assert!(derive_nix_closed_run_plan(missing_presealed_check).is_err());

    let mut same_presealed_outputs = binding();
    same_presealed_outputs.isolation_mode = NixIsolationModeV1::PresealedOfflineClosure;
    same_presealed_outputs.presealed_offline_closure_sha256 = Some(digest('a'));
    let same_path = "/nix/store/abcdfghijklmnpqrsvwxyz0123456789-codex-rs-0.0.0".to_string();
    same_presealed_outputs.presealed_output_store_path = Some(same_path.clone());
    same_presealed_outputs.presealed_check_output_store_path = Some(same_path);
    assert!(derive_nix_closed_run_plan(same_presealed_outputs).is_err());

    let mut malformed_presealed_check = binding();
    malformed_presealed_check.isolation_mode = NixIsolationModeV1::PresealedOfflineClosure;
    malformed_presealed_check.presealed_offline_closure_sha256 = Some(digest('a'));
    malformed_presealed_check.presealed_output_store_path =
        Some("/nix/store/abcdfghijklmnpqrsvwxyz0123456789-codex-rs-0.0.0".to_string());
    malformed_presealed_check.presealed_check_output_store_path =
        Some("/nix/store/eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee-checks".to_string());
    assert!(derive_nix_closed_run_plan(malformed_presealed_check).is_err());

    let mut leading_dot_store_name = binding();
    leading_dot_store_name.isolation_mode = NixIsolationModeV1::PresealedOfflineClosure;
    leading_dot_store_name.presealed_offline_closure_sha256 = Some(digest('a'));
    leading_dot_store_name.presealed_output_store_path =
        Some("/nix/store/abcdfghijklmnpqrsvwxyz0123456789-.hidden".to_string());
    leading_dot_store_name.presealed_check_output_store_path =
        Some("/nix/store/bbcdfghijklmnpqrsvwxyz0123456789-codex-rs-checks-0.0.0".to_string());
    assert!(derive_nix_closed_run_plan(leading_dot_store_name).is_err());

    let mut bad_store_hash = binding();
    bad_store_hash.isolation_mode = NixIsolationModeV1::PresealedOfflineClosure;
    bad_store_hash.presealed_offline_closure_sha256 = Some(digest('a'));
    bad_store_hash.presealed_output_store_path =
        Some("/nix/store/eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee-codex-rs-0.0.0".to_string());
    bad_store_hash.presealed_check_output_store_path =
        Some("/nix/store/bbcdfghijklmnpqrsvwxyz0123456789-codex-rs-checks-0.0.0".to_string());
    assert!(derive_nix_closed_run_plan(bad_store_hash).is_err());
}

#[test]
fn a_changed_pre_sign_binding_changes_plan_hash_without_changing_authority() {
    let first = sandbox_plan();
    let mut second_binding = binding();
    second_binding.cpuset_cpu = 25;
    let second = derive_nix_closed_run_plan(second_binding).expect("second closed plan");
    let first_inspection = inspect_canonical_nix_closed_run_plan(
        &serde_json::to_vec(&first).expect("first plan bytes"),
    )
    .expect("first inspection");
    let second_inspection = inspect_canonical_nix_closed_run_plan(
        &serde_json::to_vec(&second).expect("second plan bytes"),
    )
    .expect("second inspection");
    assert_ne!(
        first_inspection.closed_run_plan_sha256(),
        second_inspection.closed_run_plan_sha256()
    );
    assert!(!first_inspection.authorizes_live());
    assert!(!second_inspection.authorizes_live());
}

#[test]
fn nix_join_wrapper_projects_every_exact_plan_lineage_axis() {
    let binding = binding();
    let plan = derive_nix_closed_run_plan(binding.clone()).expect("closed plan");
    let canonical = serde_json::to_vec(&plan).expect("canonical closed plan");
    let inspected =
        inspect_canonical_nix_closed_run_plan(&canonical).expect("inspected closed plan");
    let expected = expected_prepared_claim_lineage_for_inspected_plan(&inspected);

    assert_eq!(
        expected.platform_scope,
        codex_hepta_mnl_trust_v1::ReplayPlatformScopeV1::Nix
    );
    assert_eq!(expected.platform_closed_run_plan_sha256, sha256(&canonical));
    assert_eq!(expected.profile_id, binding.profile_id);
    assert_eq!(expected.run_identity_sha256, binding.run_identity_sha256);
    assert_eq!(expected.run_nonce_sha256, binding.run_nonce_sha256);
    assert_eq!(expected.boot_id_sha256, binding.boot_id_sha256);
    assert_eq!(expected.host_identity_sha256, binding.host_identity_sha256);
    assert_eq!(
        expected.challenge_nonce_sha256,
        binding.challenge_nonce_sha256
    );
    assert_eq!(
        expected.final_artifact_freeze_payload_sha256,
        binding.final_artifact_freeze_payload_sha256
    );
    assert_eq!(
        expected.final_artifact_freeze_profile_id,
        binding.final_artifact_freeze_profile_id
    );
}

fn sandbox_plan() -> NixClosedRunPlanWireV1 {
    derive_nix_closed_run_plan(binding()).expect("sandbox closed plan")
}

pub(crate) fn binding() -> NixClosedRunPlanBindingV1 {
    let boot = digest('b');
    let run_nonce = digest('d');
    NixClosedRunPlanBindingV1 {
        boot_id_sha256: boot.clone(),
        challenge_nonce_sha256: digest('c'),
        collector_binary: artifact('2', "0555"),
        cpuset_cpu: 24,
        docker_api_version: "1.47".to_string(),
        docker_config_sha256: digest('a'),
        docker_platform_config_image_id_sha256: digest('1'),
        driver_binary: artifact('3', "0555"),
        final_artifact_freeze_payload_sha256: digest('4'),
        final_artifact_freeze_profile_id: "test-final-artifact-freeze-v1".to_string(),
        final_tooling: RepositoryIdentityV1 {
            head: "a".repeat(40),
            tree: "b".repeat(40),
        },
        host_identity_sha256: digest('f'),
        isolation_mode: NixIsolationModeV1::NixSandboxEnabled,
        nix_store_seed_bundle: artifact('8', "0444"),
        nix_store_seed_inventory_sha256: digest('9'),
        presealed_offline_closure_sha256: None,
        presealed_check_output_store_path: None,
        presealed_output_store_path: None,
        profile_id: "test-nix-pre-run-profile-v1".to_string(),
        run_identity_sha256: shared_run_identity(&run_nonce, &boot),
        run_nonce_sha256: run_nonce,
        runner_binary: artifact('5', "0555"),
        seccomp_profile: artifact('6', "0444"),
        source_archive: artifact('1', "0444"),
        source_tree_manifest_sha256: digest('e'),
        verifier_binary: artifact('7', "0555"),
    }
}

fn artifact(character: char, mode: &str) -> ClosedArtifactPinV1 {
    ClosedArtifactPinV1 {
        byte_count: 4096,
        mode: mode.to_string(),
        sha256: digest(character),
    }
}

fn shared_run_identity(run_nonce: &str, boot: &str) -> String {
    codex_hepta_mnl_trust_v1::derive_run_identity_sha256(run_nonce, boot)
        .expect("shared run identity")
}

fn digest(character: char) -> String {
    character.to_string().repeat(64)
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

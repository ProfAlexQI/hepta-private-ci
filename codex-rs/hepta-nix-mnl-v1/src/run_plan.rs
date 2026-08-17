use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;

use codex_hepta_mnl_trust_v1::ExpectedPreparedPreRunReplayClaimLineageV1;
use codex_hepta_mnl_trust_v1::MatchedPreparedPreRunReplayClaimInspectionV1;
use codex_hepta_mnl_trust_v1::ReplayPlatformScopeV1;
use codex_hepta_mnl_trust_v1::derive_run_identity_sha256;
use codex_hepta_mnl_trust_v1::inspect_prepared_pre_run_replay_claim_lineage;

use crate::ClosedAuthorityV1;
use crate::MINIMUM_DATA_VOLUME_FREE_BYTES;
use crate::NIX_VERSION;
use crate::NixIsolationModeV1;
use crate::NixMnlError;
use crate::PINNED_IMAGE;
use crate::PINNED_IMAGE_SHA256;
use crate::RepositoryIdentityV1;
use crate::invalid;

pub const NIX_CLOSED_RUN_PLAN_SCHEMA: &str = "hepta_nix_mnl_closed_run_plan_shape_v3";
pub const NIX_SUCCESSOR_RECEIPT_SCHEMA: &str =
    "hepta_nix_exact_mnl_successor_candidate_evidence_v2";
pub const NIX_SUCCESSOR_RECEIPT_SCHEMA_VERSION: u32 = 2;
pub const NIX_SUCCESSOR_RUN_IDENTITY_SCHEMA: &str = "hepta_mnl_successor_run_identity_v1";
pub const NIX_SUCCESSOR_RUN_IDENTITY_ALGORITHM: &str = "hepta.mnl.run-identity.v1";
pub const NIX_WORKSPACE_CHECK_CONTRACT_SCHEMA: &str = "hepta_nix_mnl_workspace_check_contract_v1";
pub const NIX_WORKSPACE_EXPECTED_INVENTORY_SCHEMA: &str =
    "hepta_nix_mnl_expected_check_inventory_v1";
pub const NIX_WORKSPACE_CHECK_CONTRACT_NAMED_MATERIAL: &str = "workspace_check_contract";
pub const MAX_NIX_CLOSED_RUN_PLAN_BYTES: usize = 72 * 1024;

const DATA_ROOT: &str = "/data";
const DOCKER_SOCKET: &str = "/var/run/docker.sock";
const DRIVER_CONTAINER_PATH: &str = "/driver/codex-hepta-nix-mnl-driver-v1";
const CLOSED_PLAN_CONTAINER_PATH: &str = "/run/hepta/closed-plan.json";
const CLOSED_PLAN_DIGEST_CONTAINER_PATH: &str = "/run/hepta/closed-plan.sha256";
const NIX_BIN: &str = "/nix/var/nix/profiles/default/bin/nix";
const NIX_STORE_BIN: &str = "/nix/var/nix/profiles/default/bin/nix-store";
const SOURCE_CONTAINER_ROOT: &str = "/workspace";
const NIX_STORE_CONTAINER_ROOT: &str = "/nix";
const MEMORY_LIMIT_BYTES: u64 = 8 * 1024 * 1024 * 1024;
const NANO_CPUS: u64 = 1_000_000_000;
const PIDS_LIMIT: u32 = 256;
const TMPFS_SIZE_BYTES: u64 = 1024 * 1024 * 1024;
const MAX_ARTIFACT_BYTES: u64 = 4 * 1024 * 1024 * 1024;
const MAX_STORE_SEED_BYTES: u64 = 64 * 1024 * 1024 * 1024;
const MAX_NAR_STREAM_BYTES: u64 = 64 * 1024 * 1024 * 1024;
const MAX_DERIVATION_BYTES: u64 = 16 * 1024 * 1024;
const MAX_CLOSURE_MEMBERS: u32 = 100_000;
const MAX_TOTAL_CLOSURE_NAR_BYTES: u64 = 64 * 1024 * 1024 * 1024;
const MAX_CPU_INDEX: u32 = 4095;
const MAX_STDOUT_BYTES: u64 = 64 * 1024 * 1024;
const MAX_STDERR_BYTES: u64 = 64 * 1024 * 1024;
const MAX_WORKSPACE_CHECK_INVENTORY_BYTES: u64 = 16 * 1024 * 1024;
const GLOBAL_LOCK_NAME: &str = "hepta-nix-mnl-successor-v1-exclusive";
const GLOBAL_LOCK_PATH: &str = "/data/hepta-nix-mnl-v1/locks/global.lock";
const CHECK_FLAKE_ATTRIBUTE: &str = "/workspace#checks.x86_64-linux.workspace";
const CARGO_NEXTEST_VERSION: &str = "0.9.124";
const WORKSPACE_CHECK_TEST_IDENTITY_ALGORITHM: &str = "sha256_domain_hepta.mnl.check-suite.inventory.v1_pid_norm_path_uri_source=cwd_cwd=common_abs_root/member(package)_format={package}_0.0.0_(workspace-member:{relative})_u64n_sortuniq_suites(lp_pkg,lp_pid,lp_bin,lp_kind,lp_name,u64_tests)_u64n_sortuniq_tests(lp_pkg,lp_pid,lp_bin,lp_kind,lp_name,lp_test,u8_ignored0)_v1";
pub(crate) const WORKSPACE_CHECK_PACKAGE_MEMBER_PATHS: [(&str, &str); 11] = [
    ("codex-hepta-contracts", "hepta-contracts"),
    ("codex-hepta-evidence", "hepta-evidence"),
    ("codex-hepta-governance", "ext/hepta-governance"),
    ("codex-hepta-memory", "hepta-memory"),
    ("codex-hepta-memory-extension", "ext/hepta-memory"),
    ("codex-hepta-mnl-replay-v1", "hepta-mnl-replay-v1"),
    ("codex-hepta-mnl-trust-v1", "hepta-mnl-trust-v1"),
    ("codex-hepta-native-gateway", "hepta-native-gateway"),
    ("codex-hepta-nix-mnl-v1", "hepta-nix-mnl-v1"),
    ("codex-hepta-paths", "hepta-paths"),
    ("codex-hepta-runtime", "hepta-runtime"),
];
const WORKSPACE_CHECK_PACKAGES: [&str; 11] = [
    WORKSPACE_CHECK_PACKAGE_MEMBER_PATHS[0].0,
    WORKSPACE_CHECK_PACKAGE_MEMBER_PATHS[1].0,
    WORKSPACE_CHECK_PACKAGE_MEMBER_PATHS[2].0,
    WORKSPACE_CHECK_PACKAGE_MEMBER_PATHS[3].0,
    WORKSPACE_CHECK_PACKAGE_MEMBER_PATHS[4].0,
    WORKSPACE_CHECK_PACKAGE_MEMBER_PATHS[5].0,
    WORKSPACE_CHECK_PACKAGE_MEMBER_PATHS[6].0,
    WORKSPACE_CHECK_PACKAGE_MEMBER_PATHS[7].0,
    WORKSPACE_CHECK_PACKAGE_MEMBER_PATHS[8].0,
    WORKSPACE_CHECK_PACKAGE_MEMBER_PATHS[9].0,
    WORKSPACE_CHECK_PACKAGE_MEMBER_PATHS[10].0,
];
const EXACT_EXPECTED_WORKSPACE_CHECK_SUITES: u32 = 11;
const MAX_EXPECTED_WORKSPACE_CHECK_TESTS: u32 = 65_536;
const MAX_EXPECTED_WORKSPACE_CHECK_INVENTORY_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClosedRunPlanDispositionV1 {
    FreshSandboxBuildInspectionOnlyNoLaunchAuthority,
    PresealedOfflineArtifactInspectionOnlyNotFreshBuild,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosedImagePullPolicyV1 {
    Never,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosedRestartPolicyV1 {
    Never,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosedRunExecutorV1 {
    SupervisorInternalStateMachine,
    BuilderCandidateDriver,
    SupervisorDockerExecIntoReadOnlyVerifier,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosedRunStageKindV1 {
    Preflight,
    SignedPlanJoinedToPreparedClaim,
    ExclusiveLock,
    CensusClear,
    RunRootEstablishedAndActiveMarkerPublished,
    ArtifactsStaged,
    ClosedPlanPublishedAndReadBack,
    SourceMaterialized,
    NixStoreProvisioned,
    BuilderCreatedAndInspected,
    BuilderIsolationVerified,
    PreRunClaimPublishedClockRecheckedAndImmediateLaunch,
    FreshBuild,
    RealChecks,
    BuilderOutputsRetainedAndQualified,
    PresealedOutputBindingsRetainedAndQualified,
    PresealedArtifactVerification,
    ReadOnlyArtifactVerification,
    BuilderStoppedNoDescendants,
    StoreReopenedReadOnly,
    VerifierCreatedAndInspected,
    VerifierIsolationVerified,
    VerifierStartedWithinRetainedRunState,
    ImmutableEvidenceCollected,
    PreSmokeReadBack,
    RuntimeSmoke,
    PostSmokeReadBack,
    VerifierStoppedNoDescendants,
    EvidenceClosed,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosedArtifactRoleV1 {
    SourceArchive,
    SeccompProfile,
    NixStoreSeedBundle,
    CollectorBinary,
    DriverBinary,
    RunnerBinary,
    VerifierBinary,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedArtifactPinV1 {
    pub byte_count: u64,
    pub mode: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixClosedRunPlanBindingV1 {
    pub boot_id_sha256: String,
    pub challenge_nonce_sha256: String,
    pub collector_binary: ClosedArtifactPinV1,
    pub cpuset_cpu: u32,
    pub docker_api_version: String,
    pub docker_config_sha256: String,
    pub docker_platform_config_image_id_sha256: String,
    pub driver_binary: ClosedArtifactPinV1,
    pub final_artifact_freeze_payload_sha256: String,
    pub final_artifact_freeze_profile_id: String,
    pub final_tooling: RepositoryIdentityV1,
    pub host_identity_sha256: String,
    pub isolation_mode: NixIsolationModeV1,
    pub nix_store_seed_bundle: ClosedArtifactPinV1,
    pub nix_store_seed_inventory_sha256: String,
    pub nextest_config_sha256: String,
    pub presealed_offline_closure_sha256: Option<String>,
    pub presealed_check_output_store_path: Option<String>,
    pub presealed_output_store_path: Option<String>,
    pub profile_id: String,
    pub run_identity_sha256: String,
    pub run_nonce_sha256: String,
    pub runner_binary: ClosedArtifactPinV1,
    pub seccomp_profile: ClosedArtifactPinV1,
    pub source_archive: ClosedArtifactPinV1,
    pub source_tree_manifest_sha256: String,
    pub verifier_binary: ClosedArtifactPinV1,
    pub workspace_check_contract_sha256: String,
    pub workspace_check_expected_inventory_sha256: String,
    pub workspace_check_expected_nonempty_suite_count: u32,
    pub workspace_check_expected_suite_count: u32,
    pub workspace_check_expected_test_count: u32,
    pub workspace_check_toolchain_manifest_sha256: String,
}

/// Exact successor check outcome required by the closed plan.
///
/// The three equality fields refer to complete test-identity sets, not merely
/// equal cardinalities. The inventory domain binds sorted, unique suite tuples
/// `(package, normalized_package_id, binary_id, target_kind, target_name,
/// test_count)` followed by test tuples `(package, normalized_package_id,
/// binary_id, target_kind, target_name, test_name, ignored=false)`. A raw
/// nextest package ID is accepted only when it is a Cargo path URI whose
/// source equals the suite working directory. Every suite working directory
/// must be the exact frozen package-to-member relative path beneath one common
/// absolute workspace root, then the ID is normalized to the frozen
/// `{package} 0.0.0 (workspace-member:{relative})` representation. This is a
/// structural declaration binding, not observed source provenance.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedWorkspaceCheckOutcomeContractV1 {
    pub discovered_equals_executed_test_identity_set: bool,
    pub executed_equals_passed_test_identity_set: bool,
    pub expected_equals_discovered_test_identity_set: bool,
    pub required_failed_count: u32,
    pub required_filtered_out_count: u32,
    pub required_ignored_count: u32,
    pub required_measured_count: u32,
    pub required_retried_count: u32,
    pub required_skipped_count: u32,
    pub required_timed_out_count: u32,
}

/// Exact raw-material verification the trusted supervisor must perform.
///
/// The deterministic candidate check output retains only its canonical
/// discovered inventory and candidate summary. It does not retain or
/// authorize raw Cargo metadata or nextest streams. A future trusted supervisor
/// must capture all of those raw inputs independently outside the candidate
/// store output and bind their capture paths in terminal evidence before
/// reparsing them. A candidate-reported copy of the recipe does not prove the
/// invocation; the future supervisor must bind the exact check derivation and
/// wrapper.
/// The candidate summary contains no subject-product, store-path, or
/// derivation identity. A future trusted supervisor must bind the separately
/// retained product. This source-workspace check does not execute it, so a
/// later exact-product smoke gate remains required.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedWorkspaceCheckSupervisorReparseV1 {
    pub candidate_raw_material_retained_in_check_output: bool,
    pub candidate_reported_recipe_authoritative: bool,
    pub candidate_summary_authoritative: bool,
    pub candidate_summary_contains_subject_product_identity: bool,
    pub candidate_summary_must_equal_recomputed_values: bool,
    pub candidate_summary_relative_path: String,
    pub discovered_inventory_relative_path: String,
    pub product_and_check_output_and_derivation_paths_must_be_distinct: bool,
    pub raw_capture_independent_of_candidate: bool,
    pub raw_inputs_require_eof_and_no_truncation: bool,
    pub reject_duplicate_unknown_or_unfinished_events: bool,
    pub semantic_inventory_digest_alone_authoritative: bool,
    pub source_workspace_check_only: bool,
    pub subject_product_binding_must_equal_retained_product: bool,
    pub subject_product_executed_by_workspace_check: bool,
    pub supervisor_recomputes_inventory_counts_and_outcomes: bool,
    pub supervisor_reparses_raw_cargo_metadata: bool,
    pub supervisor_reparses_raw_list_and_events: bool,
    pub trusted_supervisor_binds_exact_check_derivation_and_wrapper: bool,
    pub trusted_supervisor_capture_paths_bound_by_future_terminal_evidence: bool,
    pub trusted_supervisor_raw_capture_out_of_store: bool,
}

/// Exact nextest/Cargo selection axes used by the frozen workspace check.
///
/// Every axis is explicit so ambient workspace configuration, default
/// filters, feature selection, or caller CLI arguments cannot widen or narrow
/// the expected test identity set. The exact Cargo metadata projection requires
/// a single lib target per package with `test=true` and `doctest=false`.
/// Cargo 1.95 does not report target `bench` or required-features axes here, so
/// no claim is made about them. Every nextest list suite must be `kind=lib`,
/// join that exact target roster, and reject any additional target or suite.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedWorkspaceCheckSelectionV1 {
    pub all: bool,
    pub all_features: bool,
    pub benchmark_mode: String,
    pub build_target: String,
    pub cargo_target_selection_mode: String,
    pub doctests: String,
    pub exclude: Vec<String>,
    pub features: Vec<String>,
    pub filter_expression: String,
    pub ignored_test_policy: String,
    pub nextest_list_suites_must_join_cargo_metadata_projection: bool,
    pub no_default_features: bool,
    pub package_selection_mode: String,
    pub packages: Vec<String>,
    pub partition: String,
    pub target_selector_argv: Vec<String>,
    pub test_name_filters: Vec<String>,
    pub workspace: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedWorkspaceCheckEnvironmentV1 {
    pub name: String,
    pub value: String,
}

/// Exact logical Cargo/nextest recipe.
///
/// These are frozen intended argv and environment values, not independently
/// observed execution. The argv values use source-relative logical paths only.
/// `wrapper_explicit_environment_overrides` lists only values exported by the
/// check wrapper; inherited Nix/stdenv variables are instead bound by the
/// exact derivation and toolchain identities and are not claimed to be fully
/// enumerated here.
/// They deliberately exclude Nix store paths, the contract's own digest, and
/// every post-run result digest. Metadata projection parsing and comparison
/// precede nextest list; canonical inventory comparison follows list and only
/// an exact match permits nextest run. After the run, candidate verification
/// must re-read that discovered inventory and require byte equality with both
/// the frozen inventory and its own recomputation before installation.
/// Nextest list itself launches test
/// binaries for enumeration, which is recorded without treating
/// candidate-reported invocation as authority.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedWorkspaceCheckRecipeV1 {
    pub archive: String,
    pub binaries_metadata: String,
    pub build_jobs: u32,
    pub build_profile: String,
    pub caller_manifest_allowed: bool,
    pub candidate_verify_revalidates_discovered_inventory_after_run: bool,
    pub candidate_verify_revalidates_tool_versions_after_run: bool,
    pub cargo_metadata_argv: Vec<String>,
    pub cargo_metadata_projection_preflight_before_nextest_list: bool,
    pub execution_order: Vec<String>,
    pub expected_inventory_compare_argv: Vec<String>,
    pub expected_inventory_compared_before_nextest_run: bool,
    pub list_and_run_share_target_dir: bool,
    pub list_argv: Vec<String>,
    pub manifest_path: String,
    pub nextest_config_preflight_before_nextest_list: bool,
    pub nextest_list_launches_test_binaries_for_enumeration: bool,
    pub nextest_reuse_build_option: String,
    pub no_run: bool,
    pub no_tests_behavior: String,
    pub release: bool,
    pub run_argv: Vec<String>,
    pub run_environment: Vec<ClosedWorkspaceCheckEnvironmentV1>,
    pub target_dir_remap: String,
    pub tool_versions_preflight_before_cargo_metadata_and_nextest_list: bool,
    pub workspace_remap: String,
    pub workspace_root: String,
    pub wrapper_explicit_environment_overrides: Vec<ClosedWorkspaceCheckEnvironmentV1>,
}

/// Canonical, non-authorizing contract for the selected Nix workspace check.
///
/// Only this pre-run policy and the expected canonical inventory are frozen.
/// The expected inventory also freezes Cargo metadata's exact eleven-package
/// target projection: member path, package/version, empty feature map, one lib
/// target, `test=true`, and `doctest=false`. Cargo 1.95 exposes no target
/// `bench` or required-features field in this projection; those axes are not
/// inferred. The nextest list must contain only the exact joined `kind=lib`
/// suites and reject every additional target or suite. Discovered inventories,
/// event streams, summaries, and result digests are post-run evidence and
/// deliberately do not appear here.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixWorkspaceCheckContractV1 {
    pub cargo_nextest_version: String,
    pub cargo_version_requirement: String,
    pub expected_inventory_digest_algorithm: String,
    pub expected_inventory_file_encoding: String,
    pub expected_inventory_is_exact_canonical_json: bool,
    pub expected_inventory_maximum_file_bytes_including_trailing_lf: u64,
    pub expected_inventory_schema: String,
    pub expected_inventory_sha256: String,
    pub expected_inventory_sha256_covers_exact_file_bytes_including_trailing_lf: bool,
    pub expected_inventory_source_relative_path: String,
    pub expected_nonempty_suite_count: u32,
    pub expected_suite_count: u32,
    pub expected_test_count: u32,
    pub flake_attribute: String,
    pub flaky_result: String,
    pub ignore_default_filter: bool,
    pub leak_timeout_period: String,
    pub leak_timeout_result: String,
    pub list_message_format: String,
    pub locked: bool,
    pub nextest_config_exact_bytes_required: bool,
    pub nextest_config_relative_path: String,
    pub nextest_config_sha256: String,
    pub nix_system: String,
    pub no_fail_fast: bool,
    pub offline: bool,
    pub outcome: ClosedWorkspaceCheckOutcomeContractV1,
    pub package_count: u32,
    pub package_id_normalized_format: String,
    pub packages: Vec<String>,
    pub profile: String,
    pub raw_nextest_package_id_cargo_path_uri_required: bool,
    pub raw_nextest_package_id_source_must_equal_suite_cwd: bool,
    pub recipe: ClosedWorkspaceCheckRecipeV1,
    pub retries: u32,
    pub run_message_format: String,
    pub run_message_format_version: String,
    pub runner_name: String,
    pub rustc_version_requirement: String,
    pub schema: String,
    pub schema_version: u32,
    pub selection: ClosedWorkspaceCheckSelectionV1,
    pub slow_timeout_grace_period: String,
    pub slow_timeout_on_timeout: String,
    pub slow_timeout_period: String,
    pub slow_timeout_terminate_after: u32,
    pub suite_scope: String,
    pub supervisor_reparse: ClosedWorkspaceCheckSupervisorReparseV1,
    pub target_triple: String,
    pub test_identity_inventory_algorithm: String,
    pub test_threads: u32,
    pub toolchain_manifest_sha256: String,
    pub user_config_file: String,
}

/// Canonical identity contract for the successor receipt revision.
///
/// Frozen CandidateEvidence V1 continues to use its legacy canonical-JSON
/// identity. This contract is embedded in every successor closed plan so a
/// future V2 receipt builder cannot silently fall back to or reinterpret V1.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixSuccessorReceiptIdentityContractV2 {
    pub boot_id_sha256: String,
    pub legacy_candidate_evidence_v1_accepted: bool,
    pub receipt_schema: String,
    pub receipt_schema_version: u32,
    pub run_identity_algorithm: String,
    pub run_identity_schema: String,
    pub run_identity_sha256: String,
    pub run_nonce_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedArtifactPathV1 {
    pub external_freeze_path: String,
    pub external_source_gid: u32,
    pub external_source_uid: u32,
    pub pin: ClosedArtifactPinV1,
    pub role: ClosedArtifactRoleV1,
    pub host_path: String,
    pub staged_destination_gid: u32,
    pub staged_destination_mode_equals_pin: bool,
    pub staged_destination_uid: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosedRunDirectoryRoleV1 {
    RunRoot,
    InputArtifacts,
    ClosedPlanControl,
    SupervisorTools,
    DriverTools,
    MaterializedSource,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedRunDirectoryPlanV1 {
    pub create_at_stage: ClosedRunStageKindV1,
    pub create_gid: u32,
    pub create_mode: String,
    pub create_uid: u32,
    pub final_mode: String,
    pub final_mode_applied_by_retained_fd_before_first_use: bool,
    pub fsync_directory_after_final_mode: bool,
    pub fsync_parent_after_create: bool,
    pub host_path: String,
    pub inode_preserved_across_final_mode_application: bool,
    pub must_not_exist_before_create: bool,
    pub openat2_no_symlink_no_magiclink_no_xdev: bool,
    pub reopen_and_verify_uid_gid_mode_inode_before_first_use: bool,
    pub retain_inode_until_terminal: bool,
    pub role: ClosedRunDirectoryRoleV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedBindMountV1 {
    pub container_path: String,
    pub docker_inspect_read_back_exact: bool,
    pub host_gid: u32,
    pub host_mode: String,
    pub host_path: String,
    pub host_uid: u32,
    pub immediate_pre_start_revalidate_canonical_path_inode_mode_uid_gid: bool,
    pub read_only: bool,
    pub retained_host_inode_through_container_start: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedNamedVolumeMountV1 {
    pub name: String,
    pub container_path: String,
    pub create_if_missing: bool,
    pub driver: String,
    pub driver_options: Vec<String>,
    pub docker_inspect_exact_identity_before_execute: bool,
    pub immediate_pre_start_revalidate_name_driver_options_labels_mountpoint_identity: bool,
    pub labels: Vec<ClosedDockerLabelV1>,
    pub no_copy: bool,
    pub read_only: bool,
    pub must_exist_exact_identity: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosedContainerRoleV1 {
    Builder,
    ReadOnlyVerifierSmoke,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ClosedSecurityOptionV1 {
    NoNewPrivileges,
    SeccompFromVerifiedArtifactBytes { role: ClosedArtifactRoleV1 },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ClosedDockerLabelV1 {
    Literal { key: String, value: String },
    ExternalVerifiedPlanDigest { key: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedTmpfsMountV1 {
    pub container_path: String,
    pub gid: u32,
    pub mode: String,
    pub nodev: bool,
    pub noexec: bool,
    pub nosuid: bool,
    pub size_bytes: u64,
    pub uid: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedResourcePlanV1 {
    pub cpuset_cpus: String,
    pub effective_cores: u32,
    pub effective_max_jobs: u32,
    pub memory_limit_bytes: u64,
    pub nano_cpus: u64,
    pub pids_limit: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedContainerSpecV1 {
    pub api_version: String,
    pub artifact_paths: Vec<ClosedArtifactPathV1>,
    pub attach_stderr: bool,
    pub attach_stdout: bool,
    pub auto_remove: bool,
    pub bind_mounts: Vec<ClosedBindMountV1>,
    pub cap_add: Vec<String>,
    pub cap_drop: Vec<String>,
    pub candidate_can_read_closed_plan: bool,
    pub candidate_can_read_copy_store: bool,
    pub candidate_can_read_evidence: bool,
    pub candidate_can_read_profile: bool,
    pub candidate_can_read_replay_store: bool,
    pub command_arguments: Vec<String>,
    pub command_executable: String,
    pub container_name: String,
    pub devices: Vec<String>,
    pub dns: Vec<String>,
    pub docker_socket: String,
    pub docker_create_response_id_retained: bool,
    pub docker_inspect_before_execute: bool,
    pub environment: Vec<String>,
    pub extra_hosts: Vec<String>,
    pub host_ipc: bool,
    pub host_pid: bool,
    pub hostname: String,
    pub image: String,
    pub image_config_id_sha256: String,
    pub image_manifest_sha256: String,
    pub image_manifest_and_config_read_back_exact: bool,
    pub image_pull_policy: ClosedImagePullPolicyV1,
    pub inherit_environment: bool,
    pub labels: Vec<ClosedDockerLabelV1>,
    pub labels_read_back_exact: bool,
    pub named_volume_mounts: Vec<ClosedNamedVolumeMountV1>,
    pub network_disabled: bool,
    pub network_mode: String,
    pub no_new_privileges: bool,
    pub open_stdin: bool,
    pub platform: String,
    pub ports: Vec<String>,
    pub privileged: bool,
    pub read_only_rootfs: bool,
    pub role: ClosedContainerRoleV1,
    pub resources: ClosedResourcePlanV1,
    pub restart_policy: ClosedRestartPolicyV1,
    pub retry_count: u32,
    pub seccomp_profile_sha256: String,
    pub security_options: Vec<ClosedSecurityOptionV1>,
    pub tmpfs_mounts: Vec<ClosedTmpfsMountV1>,
    pub tty: bool,
    pub user: String,
    pub working_directory: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ClosedExecutableV1 {
    FixedAbsolute { path: String },
    QualifiedProductOutputRelative { relative_path: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ClosedArgumentV1 {
    Literal { value: String },
    QualifiedProductOutputStorePath,
    QualifiedCheckOutputStorePath,
    QualifiedCheckOutputRelative { relative_path: String },
    QualifiedProductDerivationStorePath,
    QualifiedCheckDerivationStorePath,
    QualifiedClosureMemberStorePath,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ClosedStdoutHandlingV1 {
    BoundedCapture {
        maximum_bytes: u64,
    },
    StreamingSha256ToSupervisor {
        maximum_bytes: u64,
        require_eof: bool,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedCommandSpecV1 {
    pub arguments: Vec<ClosedArgumentV1>,
    pub executable: ClosedExecutableV1,
    pub inherit_environment: bool,
    pub stderr_limit_bytes: u64,
    pub stdin_closed: bool,
    pub stdout_handling: ClosedStdoutHandlingV1,
    pub timeout_seconds: u32,
    pub truncation_is_failure: bool,
    pub working_directory: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedRunStageV1 {
    pub commands: Vec<ClosedCommandSpecV1>,
    pub executor: ClosedRunExecutorV1,
    pub kind: ClosedRunStageKindV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedEvidenceAlgorithmPlanV1 {
    pub artifact_bytes: String,
    pub closure_inventory: String,
    pub derivation_bytes: String,
    pub nar_stream: String,
    pub path_bytes: String,
    pub source_archive_bytes: String,
    pub stderr_bytes: String,
    pub stdout_bytes: String,
    pub suite_inventory: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedDynamicEvidenceTraversalPlanV1 {
    pub closure_member_commands: Vec<ClosedCommandSpecV1>,
    pub closure_member_paths_sorted_unique_bytewise: bool,
    pub closure_inventory_is_sorted_unique_union_of_product_and_check_requisites: bool,
    pub closure_requisites_must_include_check_output: bool,
    pub closure_requisites_must_include_product_output: bool,
    pub product_derivation_commands: Vec<ClosedCommandSpecV1>,
    pub check_derivation_commands: Vec<ClosedCommandSpecV1>,
    pub product_and_check_derivation_paths_must_be_distinct_single_canonical_drv_store_paths: bool,
    pub executor: ClosedRunExecutorV1,
    pub inputs_from_preceding_artifact_verification_retained_output: bool,
    pub maximum_closure_members: u32,
    pub maximum_total_closure_nar_bytes: u64,
    pub no_shell_or_candidate_selected_commands: bool,
    pub stage_kind: ClosedRunStageKindV1,
    pub supervisor_drives_each_iteration: bool,
    pub traversal_must_complete_before_pre_smoke_read_back: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosedOutputHandoffSourceV1 {
    FreshBuilderAndCheckTranscripts,
    SignedPresealedProductAndCheckBindings,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedDynamicOutputHandoffPlanV1 {
    pub docker_exec_argv_constructed_by_supervisor_without_shell: bool,
    pub dynamic_derivation_and_closure_queries_retained_after_exit_zero: bool,
    pub fresh_builder_transcript_exit_status_and_no_truncation_bound_before_accept: bool,
    pub handoff_source: ClosedOutputHandoffSourceV1,
    pub fresh_outputs_and_target_derivation_outputs_checked_absent_from_pre_builder_inventory: bool,
    pub no_cli_env_file_stdin_or_candidate_selected_dynamic_path: bool,
    pub no_serializable_or_cloneable_handoff_token: bool,
    pub presealed_signed_product_and_check_bindings_verified_before_verifier_create: bool,
    pub product_and_check_paths_must_be_distinct_canonical_store_paths: bool,
    pub qualified_tokens_resolved_only_from_retained_supervisor_state: bool,
    pub retained_state_binds_plan_run_profile_and_verifier_container_id: bool,
    pub fresh_retained_state_also_binds_builder_container_id: bool,
    pub verifier_docker_exec_inspected_for_exit_stdout_stderr_and_truncation: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedVerifierDockerExecPolicyV1 {
    pub attach_stderr: bool,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    pub docker_exec_create_id_retained: bool,
    pub docker_exec_inspect_binds_same_verifier_container_id: bool,
    pub docker_exec_inspect_started_running_exit_code_read_back: bool,
    pub environment_overrides: Vec<String>,
    pub inherit_caller_environment: bool,
    pub privileged: bool,
    pub start_detach: bool,
    pub start_tty_matches_create: bool,
    pub tty: bool,
    pub user: String,
    pub working_directory: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosedBootstrapPinAuthoritySourceV1 {
    FutureTypedVerifiedFinalArtifactFreezeInspectionOnly,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedToolExecutionPlanV1 {
    pub bootstrap_runner_exec_forbidden_when_typed_final_freeze_unavailable: bool,
    pub collector_helper_host_path: String,
    pub collector_helper_role: ClosedArtifactRoleV1,
    pub collector_helper_stages: Vec<ClosedRunStageKindV1>,
    pub driver_container_entrypoint_path: String,
    pub driver_container_role: ClosedArtifactRoleV1,
    pub fresh_driver_builder_stages: Vec<ClosedRunStageKindV1>,
    pub driver_is_only_candidate_container_entrypoint: bool,
    pub driver_staged_host_path: String,
    pub driver_verifies_mounted_plan_and_sidecar_before_any_container_stage: bool,
    pub bootstrap_runner_inode_retained_before_preflight: bool,
    pub bootstrap_runner_pin_authority_source: ClosedBootstrapPinAuthoritySourceV1,
    pub bootstrap_runner_pin_never_from_closed_plan_or_caller: bool,
    pub bootstrap_runner_verified_by_trusted_launcher_before_exec: bool,
    pub host_tool_inodes_retained_and_reverified_before_each_invocation: bool,
    pub no_unpinned_or_path_searched_tool: bool,
    pub runner_bootstrap_external_freeze_path: String,
    pub runner_role: ClosedArtifactRoleV1,
    pub runner_drives_all_supervisor_and_docker_api_stages: bool,
    pub runner_staged_readback_must_equal_bootstrap_bytes: bool,
    pub runner_staged_readback_path: String,
    pub verifier_helper_host_path: String,
    pub verifier_helper_role: ClosedArtifactRoleV1,
    pub verifier_helper_stages: Vec<ClosedRunStageKindV1>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosedOutputPathSourceV1 {
    FreshBuildStdoutSingleCanonicalStorePath,
    RealChecksStdoutSingleCanonicalStorePath,
    SignedPresealedBindingExactCanonicalStorePath,
    SignedPresealedCheckOutputBindingExactCanonicalStorePath,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedSourceMaterializationPlanV1 {
    pub archive_host_path: String,
    pub archive_role: ClosedArtifactRoleV1,
    pub destination_host_path: String,
    pub destination_mode: String,
    pub destination_must_not_exist: bool,
    pub expected_tree_manifest_sha256: String,
    pub extraction_policy: String,
    pub materializer_role: ClosedArtifactRoleV1,
    pub read_back_tree_manifest: bool,
    pub read_only_after_materialization: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedPlanDeliveryV1 {
    pub atomic_no_replace_publish_after_census: bool,
    pub candidate_can_read_plan: bool,
    pub candidate_recomputes_external_digest_before_first_stage: bool,
    pub container_path: String,
    pub external_digest_source: String,
    pub external_digest_container_path: String,
    pub external_digest_host_path: String,
    pub external_digest_sidecar_format: String,
    pub host_path: String,
    pub host_file_gid: u32,
    pub host_file_mode: String,
    pub host_file_uid: u32,
    pub immediate_pre_start_revalidate_plan_and_sidecar_canonical_path_inode_mode_uid_gid: bool,
    pub mount_read_only: bool,
    pub plan_must_not_contain_own_digest: bool,
    pub publish_directory_fsync: bool,
    pub publish_file_fsync: bool,
    pub retain_published_inode_through_container_start: bool,
    pub retain_sidecar_inode_through_container_start: bool,
    pub sidecar_atomic_no_replace_publish: bool,
    pub sidecar_bytes_derived_only_from_joined_prepared_claim: bool,
    pub sidecar_candidate_compares_before_any_container_stage: bool,
    pub sidecar_directory_fsync: bool,
    pub sidecar_equals_joined_plan_digest: bool,
    pub sidecar_file_fsync: bool,
    pub sidecar_gid: u32,
    pub sidecar_mode: String,
    pub sidecar_reject_existing_or_residue: bool,
    pub sidecar_reopen_and_read_back_exact_bytes: bool,
    pub sidecar_uid: u32,
    pub supervisor_reopens_and_verifies_exact_bytes: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedArtifactStagingPlanV1 {
    pub artifacts: Vec<ClosedArtifactPathV1>,
    pub atomic_no_replace_publish: bool,
    pub destination_run_root: String,
    pub directories: Vec<ClosedRunDirectoryPlanV1>,
    pub directory_fsync_after_each_publish: bool,
    pub external_freeze_parent_gid: u32,
    pub external_freeze_parent_mode: String,
    pub external_freeze_parent_uid: u32,
    pub external_freeze_root: String,
    pub external_sources_exact_pin_mode_uid_gid: bool,
    pub external_sources_openat2_no_symlink_no_magiclink_no_xdev: bool,
    pub file_fsync_before_publish: bool,
    pub reject_existing_or_residual_destination: bool,
    pub reopen_and_read_back_exact_bytes: bool,
    pub retain_external_source_inodes_until_publish: bool,
    pub retain_published_inodes_until_terminal: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedFreshnessPrerequisitePlanV1 {
    pub claim_publication_and_post_clock_deferred_until_launch_ready: bool,
    pub exact_inspected_plan_digest_joined_to_prepared_claim: bool,
    pub external_digest_never_accepted_from_caller: bool,
    pub immediate_launch_is_same_internal_state_machine_step: bool,
    pub indivisible_clock_publication_clock_sequence: bool,
    pub no_delayed_or_serializable_launch_grant: bool,
    pub post_publication_failure_is_uncertain_and_nonce_stays_consumed: bool,
    pub prepared_claim_scope: String,
    pub sequence: String,
    pub signed_plan_join_occurs_before_setup_without_consuming_nonce: bool,
    pub subsequent_verifier_start_requires_retained_nonserializable_run_state: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedSeccompInputV1 {
    pub artifact_host_path: String,
    pub artifact_role: ClosedArtifactRoleV1,
    pub canonical_json_required: bool,
    pub docker_request_uses_exact_verified_bytes: bool,
    pub pin: ClosedArtifactPinV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedGlobalExclusionPlanV1 {
    pub active_candidate_containers_before: u32,
    pub active_named_volumes_before: u32,
    pub active_run_markers_before: u32,
    pub active_run_marker_bytes_derived_only_from_joined_prepared_claim: bool,
    pub active_run_marker_bytes_format: String,
    pub active_run_marker_directory_fsync: bool,
    pub active_run_marker_file_fsync: bool,
    pub active_run_marker_gid: u32,
    pub active_run_marker_host_path: String,
    pub active_run_marker_mode: String,
    pub active_run_marker_publish_no_replace_after_census: bool,
    pub active_run_marker_reject_existing_or_residue: bool,
    pub active_run_marker_reopen_and_read_back_exact_bytes: bool,
    pub active_run_marker_retained_inode_until_terminal: bool,
    pub active_run_marker_uid: u32,
    pub container_label_selector: String,
    pub lock_acquisition: String,
    pub lock_acquisition_run_nonce_sha256: String,
    pub lock_creation_file_and_parent_fsync_if_absent: bool,
    pub lock_existing_file_never_truncated_or_replaced: bool,
    pub lock_host_path: String,
    pub lock_mode: String,
    pub lock_name: String,
    pub lock_parent_gid: u32,
    pub lock_parent_host_path: String,
    pub lock_parent_mode: String,
    pub lock_parent_openat2_no_symlink_no_magiclink_no_xdev: bool,
    pub lock_parent_uid: u32,
    pub lock_path_opened_from_retained_parent_fd: bool,
    pub lock_regular_file_mode_owner_inode_verified_before_flock: bool,
    pub lock_retained_until_terminal: bool,
    pub lock_owner_gid: u32,
    pub lock_owner_uid: u32,
    pub volume_label_selector: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedHostPreflightPlanV1 {
    pub architecture: String,
    pub boot_id_sha256: String,
    pub cpuset_cpu: u32,
    pub data_root: String,
    pub docker_api_version: String,
    pub docker_config_sha256: String,
    pub docker_platform: String,
    pub docker_socket: String,
    pub host_identity_sha256: String,
    pub image: String,
    pub image_config_id_sha256: String,
    pub image_manifest_sha256: String,
    pub kernel_system: String,
    pub minimum_data_volume_free_bytes: u64,
    pub nix_system: String,
    pub nix_version: String,
    pub selected_cpu_must_be_online: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedNixStoreProvisioningPlanV1 {
    pub create_run_unique_volume_no_replace: bool,
    pub initial_seed_bundle_host_path: String,
    pub initial_seed_bundle_role: ClosedArtifactRoleV1,
    pub initial_seed_inventory_sha256: String,
    pub seed_bundle_format: String,
    pub seed_extraction_policy: String,
    pub seed_materializer_role: ClosedArtifactRoleV1,
    pub no_image_copy_up: bool,
    pub pre_builder_inventory_retained_before_freshness_barrier: bool,
    pub fresh_product_check_and_target_derivation_outputs_must_be_absent_from_pre_builder_inventory:
        bool,
    pub seed_inventory_read_back_before_any_candidate_container: bool,
    pub reopen_same_volume_read_only_before_verifier: bool,
    pub volume_name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedEvidenceCollectionPlanV1 {
    pub binary_retained_fd_pre_and_post_smoke_read_back: bool,
    pub candidate_self_report_authoritative: bool,
    pub check_output_path_source: ClosedOutputPathSourceV1,
    pub check_manifest_relative_path: String,
    pub check_manifest_schema: String,
    pub closure_members_each_require_path_nar_size_and_nar_sha256: bool,
    pub command_transcripts_bind_argv_exit_stdout_stderr_and_truncation: bool,
    pub derivation_exact_regular_file_bytes: bool,
    pub exact_state_surface_pre_post_inventory_required_before_receipt: bool,
    pub lossless_network_attempt_observer_required_before_receipt: bool,
    pub network_access_prevented_by_enforcement: bool,
    pub network_attempted_receipt_field_available: bool,
    pub nix_store_dump_complete_stream_required: bool,
    pub product_and_check_output_paths_are_distinct: bool,
    pub product_output_path_source: ClosedOutputPathSourceV1,
    pub fresh_check_output_required: bool,
    pub fresh_realization_proven_after_immediate_launch: bool,
    pub presealed_check_provenance_is_historical_and_not_fresh: bool,
    pub presealed_product_and_check_paths_must_be_closure_members: bool,
    pub presealed_signed_closure_digest_must_equal_recomputed_union_inventory: bool,
    pub single_product_output_path_required: bool,
    pub runtime_smoke_executable_resolves_only_from_product_output: bool,
    pub state_mutated_receipt_field_available: bool,
    pub supervisor_is_evidence_writer: bool,
    pub test_manifest_requires_all_pass_no_ignored_no_filtered: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedFailurePolicyV1 {
    pub automatic_cleanup: bool,
    pub automatic_retry: bool,
    pub fallback_isolation_mode: bool,
    pub post_launch_unknown_is_uncertain: bool,
    pub preserve_container_and_volume_on_failure: bool,
    pub truncation_is_failure: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixClosedRunPlanWireV1 {
    pub authority: ClosedAuthorityV1,
    pub artifact_staging: ClosedArtifactStagingPlanV1,
    pub binding: NixClosedRunPlanBindingV1,
    pub builder_container: Option<ClosedContainerSpecV1>,
    pub disposition: ClosedRunPlanDispositionV1,
    pub dynamic_evidence_traversal: ClosedDynamicEvidenceTraversalPlanV1,
    pub dynamic_output_handoff: ClosedDynamicOutputHandoffPlanV1,
    pub evidence_collection: ClosedEvidenceCollectionPlanV1,
    pub evidence_algorithms: ClosedEvidenceAlgorithmPlanV1,
    pub failure_policy: ClosedFailurePolicyV1,
    pub freshness_prerequisite: ClosedFreshnessPrerequisitePlanV1,
    pub global_exclusion: ClosedGlobalExclusionPlanV1,
    pub host_preflight: ClosedHostPreflightPlanV1,
    pub nix_store_provisioning: ClosedNixStoreProvisioningPlanV1,
    pub plan_delivery: ClosedPlanDeliveryV1,
    pub run_root: String,
    pub schema: String,
    pub schema_version: u32,
    pub seccomp_input: ClosedSeccompInputV1,
    pub source_materialization: ClosedSourceMaterializationPlanV1,
    pub stages: Vec<ClosedRunStageV1>,
    pub successor_receipt_identity_contract: NixSuccessorReceiptIdentityContractV2,
    pub tool_execution: ClosedToolExecutionPlanV1,
    pub verifier_container: ClosedContainerSpecV1,
    pub verifier_docker_exec_policy: ClosedVerifierDockerExecPolicyV1,
    pub workspace_check_contract: NixWorkspaceCheckContractV1,
}

#[derive(Debug)]
pub struct InspectedNixClosedRunPlanV1 {
    binding: NixClosedRunPlanBindingV1,
    boot_id_sha256: String,
    canonical_bytes: Vec<u8>,
    challenge_nonce_sha256: String,
    closed_run_plan_sha256: String,
    disposition: ClosedRunPlanDispositionV1,
    final_artifact_freeze_payload_sha256: String,
    final_artifact_freeze_profile_id: String,
    host_identity_sha256: String,
    profile_id: String,
    run_identity_sha256: String,
    run_nonce_sha256: String,
}

#[derive(Debug)]
pub struct JoinedNixClosedRunPlanPreparedClaimInspectionV1 {
    plan: InspectedNixClosedRunPlanV1,
    matched_claim: MatchedPreparedPreRunReplayClaimInspectionV1,
}

impl InspectedNixClosedRunPlanV1 {
    pub(crate) fn binding(&self) -> &NixClosedRunPlanBindingV1 {
        &self.binding
    }

    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }

    pub fn closed_run_plan_sha256(&self) -> &str {
        &self.closed_run_plan_sha256
    }

    pub fn disposition(&self) -> ClosedRunPlanDispositionV1 {
        self.disposition
    }

    pub fn profile_id(&self) -> &str {
        &self.profile_id
    }

    pub fn run_identity_sha256(&self) -> &str {
        &self.run_identity_sha256
    }

    pub const fn authorizes_live(&self) -> bool {
        false
    }

    pub const fn durable_claim_observed(&self) -> bool {
        false
    }

    pub const fn host_observed(&self) -> bool {
        false
    }

    pub const fn execution_ready(&self) -> bool {
        false
    }

    pub const fn flake_check_output_observed(&self) -> bool {
        false
    }

    pub const fn source_materialized(&self) -> bool {
        false
    }

    pub const fn artifacts_staged(&self) -> bool {
        false
    }

    pub const fn plan_digest_joined_to_prepared_claim(&self) -> bool {
        false
    }

    pub const fn builder_and_verifier_isolation_observed(&self) -> bool {
        false
    }

    pub const fn immutable_evidence_collected(&self) -> bool {
        false
    }

    pub const fn store_provisioned(&self) -> bool {
        false
    }

    pub const fn seccomp_applied(&self) -> bool {
        false
    }

    pub const fn typed_final_freeze_bound(&self) -> bool {
        false
    }

    pub const fn launch_grant_available(&self) -> bool {
        false
    }

    pub const fn launch_performed(&self) -> bool {
        false
    }

    pub const fn wall_clock_verified(&self) -> bool {
        false
    }
}

impl JoinedNixClosedRunPlanPreparedClaimInspectionV1 {
    pub(crate) fn into_parts(
        self,
    ) -> (
        InspectedNixClosedRunPlanV1,
        MatchedPreparedPreRunReplayClaimInspectionV1,
    ) {
        (self.plan, self.matched_claim)
    }

    pub fn plan(&self) -> &InspectedNixClosedRunPlanV1 {
        &self.plan
    }

    pub fn matched_claim(&self) -> &MatchedPreparedPreRunReplayClaimInspectionV1 {
        &self.matched_claim
    }

    pub const fn authorizes_live(&self) -> bool {
        false
    }

    pub const fn launch_grant_available(&self) -> bool {
        false
    }

    pub const fn durable_claim_observed(&self) -> bool {
        false
    }

    pub const fn wall_clock_verified(&self) -> bool {
        false
    }

    pub const fn launch_performed(&self) -> bool {
        false
    }
}

pub fn derive_nix_closed_run_plan(
    binding: NixClosedRunPlanBindingV1,
) -> Result<NixClosedRunPlanWireV1, NixMnlError> {
    validate_binding(&binding)?;
    let successor_receipt_identity_contract = successor_receipt_identity_contract(&binding);
    let workspace_check_contract = exact_nix_workspace_check_contract(&binding);
    let run_root = format!(
        "{DATA_ROOT}/hepta-nix-mnl-v1-runs/{}",
        binding.run_identity_sha256
    );
    let source_root = format!("{run_root}/source");
    let driver_root = format!("{run_root}/driver");
    let supervisor_root = format!("{run_root}/supervisor");
    let input_root = format!("{run_root}/input");
    let control_root = format!("{run_root}/control");
    let external_freeze_root = format!(
        "{DATA_ROOT}/hepta-nix-mnl-v1/frozen-artifacts/{}",
        binding.final_artifact_freeze_payload_sha256
    );
    let volume_name = format!("hepta_nix_mnl_v1_store_{}", binding.run_identity_sha256);
    let sandbox = binding.isolation_mode == NixIsolationModeV1::NixSandboxEnabled;
    let disposition = if sandbox {
        ClosedRunPlanDispositionV1::FreshSandboxBuildInspectionOnlyNoLaunchAuthority
    } else {
        ClosedRunPlanDispositionV1::PresealedOfflineArtifactInspectionOnlyNotFreshBuild
    };
    let artifacts = vec![
        artifact_path(
            ClosedArtifactRoleV1::SourceArchive,
            &external_freeze_root,
            &input_root,
            "source.tar.zst",
            &binding.source_archive,
        ),
        artifact_path(
            ClosedArtifactRoleV1::SeccompProfile,
            &external_freeze_root,
            &input_root,
            "seccomp.json",
            &binding.seccomp_profile,
        ),
        artifact_path(
            ClosedArtifactRoleV1::NixStoreSeedBundle,
            &external_freeze_root,
            &input_root,
            "nix-store-seed.nar.zst",
            &binding.nix_store_seed_bundle,
        ),
        artifact_path(
            ClosedArtifactRoleV1::CollectorBinary,
            &external_freeze_root,
            &supervisor_root,
            "codex-hepta-nix-mnl-collector-v1",
            &binding.collector_binary,
        ),
        artifact_path(
            ClosedArtifactRoleV1::DriverBinary,
            &external_freeze_root,
            &driver_root,
            "codex-hepta-nix-mnl-driver-v1",
            &binding.driver_binary,
        ),
        artifact_path(
            ClosedArtifactRoleV1::RunnerBinary,
            &external_freeze_root,
            &supervisor_root,
            "codex-hepta-nix-mnl-runner-v1",
            &binding.runner_binary,
        ),
        artifact_path(
            ClosedArtifactRoleV1::VerifierBinary,
            &external_freeze_root,
            &supervisor_root,
            "codex-hepta-nix-mnl-verifier-v1",
            &binding.verifier_binary,
        ),
    ];
    let closed_plan_host_path = format!("{control_root}/closed-plan.json");
    let closed_plan_digest_host_path = format!("{control_root}/closed-plan.sha256");
    let builder_container = sandbox.then(|| {
        exact_container(
            &binding,
            &source_root,
            &driver_root,
            &closed_plan_host_path,
            &closed_plan_digest_host_path,
            &volume_name,
            artifacts.clone(),
            ClosedContainerRoleV1::Builder,
            false,
            "0:0",
        )
    });
    let verifier_container = exact_container(
        &binding,
        &source_root,
        &driver_root,
        &closed_plan_host_path,
        &closed_plan_digest_host_path,
        &volume_name,
        artifacts.clone(),
        ClosedContainerRoleV1::ReadOnlyVerifierSmoke,
        true,
        "65532:65532",
    );
    let source_archive_host_path = format!("{input_root}/source.tar.zst");
    let seccomp_host_path = format!("{input_root}/seccomp.json");
    let seed_bundle_host_path = format!("{input_root}/nix-store-seed.nar.zst");
    Ok(NixClosedRunPlanWireV1 {
        authority: ClosedAuthorityV1::exact(),
        artifact_staging: ClosedArtifactStagingPlanV1 {
            artifacts,
            atomic_no_replace_publish: true,
            destination_run_root: run_root.clone(),
            directories: exact_run_directories(
                &run_root,
                &input_root,
                &control_root,
                &supervisor_root,
                &driver_root,
                &source_root,
            ),
            directory_fsync_after_each_publish: true,
            external_freeze_parent_gid: 0,
            external_freeze_parent_mode: "0555".to_string(),
            external_freeze_parent_uid: 0,
            external_freeze_root: external_freeze_root.clone(),
            external_sources_exact_pin_mode_uid_gid: true,
            external_sources_openat2_no_symlink_no_magiclink_no_xdev: true,
            file_fsync_before_publish: true,
            reject_existing_or_residual_destination: true,
            reopen_and_read_back_exact_bytes: true,
            retain_external_source_inodes_until_publish: true,
            retain_published_inodes_until_terminal: true,
        },
        binding: binding.clone(),
        builder_container,
        disposition,
        dynamic_evidence_traversal: exact_dynamic_evidence_traversal(),
        dynamic_output_handoff: exact_dynamic_output_handoff(sandbox),
        evidence_collection: exact_evidence_collection(sandbox),
        evidence_algorithms: exact_evidence_algorithms(),
        failure_policy: ClosedFailurePolicyV1 {
            automatic_cleanup: false,
            automatic_retry: false,
            fallback_isolation_mode: false,
            post_launch_unknown_is_uncertain: true,
            preserve_container_and_volume_on_failure: true,
            truncation_is_failure: true,
        },
        freshness_prerequisite: ClosedFreshnessPrerequisitePlanV1 {
            claim_publication_and_post_clock_deferred_until_launch_ready: true,
            exact_inspected_plan_digest_joined_to_prepared_claim: true,
            external_digest_never_accepted_from_caller: true,
            immediate_launch_is_same_internal_state_machine_step: true,
            indivisible_clock_publication_clock_sequence: true,
            no_delayed_or_serializable_launch_grant: true,
            post_publication_failure_is_uncertain_and_nonce_stays_consumed: true,
            prepared_claim_scope: "nix".to_string(),
            sequence: "signed_plan_join_then_setup_and_container_inspect_then_clock_and_boot_pre_then_durable_no_replace_claim_file_fsync_directory_fsync_reopen_readback_then_clock_and_boot_post_then_immediate_docker_start_v1".to_string(),
            signed_plan_join_occurs_before_setup_without_consuming_nonce: true,
            subsequent_verifier_start_requires_retained_nonserializable_run_state: sandbox,
        },
        global_exclusion: ClosedGlobalExclusionPlanV1 {
            active_candidate_containers_before: 0,
            active_named_volumes_before: 0,
            active_run_markers_before: 0,
            active_run_marker_bytes_derived_only_from_joined_prepared_claim: true,
            active_run_marker_bytes_format: "domain_framed_plan_profile_run_boot_host_sha256s_v1"
                .to_string(),
            active_run_marker_directory_fsync: true,
            active_run_marker_file_fsync: true,
            active_run_marker_gid: 0,
            active_run_marker_host_path: format!("{run_root}/active-run.marker-v1"),
            active_run_marker_mode: "0600".to_string(),
            active_run_marker_publish_no_replace_after_census: true,
            active_run_marker_reject_existing_or_residue: true,
            active_run_marker_reopen_and_read_back_exact_bytes: true,
            active_run_marker_retained_inode_until_terminal: true,
            active_run_marker_uid: 0,
            container_label_selector: "hepta.mnl.family=hepta-nix-mnl-v1".to_string(),
            lock_acquisition: "retained_fd_flock_exclusive_nonblocking_v1".to_string(),
            lock_acquisition_run_nonce_sha256: binding.run_nonce_sha256.clone(),
            lock_creation_file_and_parent_fsync_if_absent: true,
            lock_existing_file_never_truncated_or_replaced: true,
            lock_host_path: GLOBAL_LOCK_PATH.to_string(),
            lock_mode: "0600".to_string(),
            lock_name: GLOBAL_LOCK_NAME.to_string(),
            lock_parent_gid: 0,
            lock_parent_host_path: "/data/hepta-nix-mnl-v1/locks".to_string(),
            lock_parent_mode: "0700".to_string(),
            lock_parent_openat2_no_symlink_no_magiclink_no_xdev: true,
            lock_parent_uid: 0,
            lock_path_opened_from_retained_parent_fd: true,
            lock_regular_file_mode_owner_inode_verified_before_flock: true,
            lock_owner_gid: 0,
            lock_owner_uid: 0,
            lock_retained_until_terminal: true,
            volume_label_selector: "hepta.mnl.family=hepta-nix-mnl-v1".to_string(),
        },
        host_preflight: ClosedHostPreflightPlanV1 {
            architecture: "x86_64".to_string(),
            boot_id_sha256: binding.boot_id_sha256.clone(),
            cpuset_cpu: binding.cpuset_cpu,
            data_root: DATA_ROOT.to_string(),
            docker_api_version: binding.docker_api_version.clone(),
            docker_config_sha256: binding.docker_config_sha256.clone(),
            docker_platform: "linux/amd64".to_string(),
            docker_socket: DOCKER_SOCKET.to_string(),
            host_identity_sha256: binding.host_identity_sha256.clone(),
            image: PINNED_IMAGE.to_string(),
            image_config_id_sha256: binding.docker_platform_config_image_id_sha256.clone(),
            image_manifest_sha256: PINNED_IMAGE_SHA256.to_string(),
            kernel_system: "Linux".to_string(),
            minimum_data_volume_free_bytes: MINIMUM_DATA_VOLUME_FREE_BYTES,
            nix_system: "x86_64-linux".to_string(),
            nix_version: NIX_VERSION.to_string(),
            selected_cpu_must_be_online: true,
        },
        nix_store_provisioning: ClosedNixStoreProvisioningPlanV1 {
            create_run_unique_volume_no_replace: true,
            initial_seed_bundle_host_path: seed_bundle_host_path,
            initial_seed_bundle_role: ClosedArtifactRoleV1::NixStoreSeedBundle,
            initial_seed_inventory_sha256: binding.nix_store_seed_inventory_sha256.clone(),
            no_image_copy_up: true,
            pre_builder_inventory_retained_before_freshness_barrier: sandbox,
            fresh_product_check_and_target_derivation_outputs_must_be_absent_from_pre_builder_inventory: sandbox,
            seed_inventory_read_back_before_any_candidate_container: true,
            reopen_same_volume_read_only_before_verifier: true,
            seed_bundle_format: "zstd_single_frame_nar_stream_set_v1".to_string(),
            seed_extraction_policy:
                "supervisor_streams_exact_inventory_no_path_escape_no_duplicate_no_special_v1"
                    .to_string(),
            seed_materializer_role: ClosedArtifactRoleV1::RunnerBinary,
            volume_name,
        },
        plan_delivery: ClosedPlanDeliveryV1 {
            atomic_no_replace_publish_after_census: true,
            candidate_can_read_plan: true,
            candidate_recomputes_external_digest_before_first_stage: true,
            container_path: CLOSED_PLAN_CONTAINER_PATH.to_string(),
            external_digest_source: "signed_pre_run_profile_external_digest_field_v1".to_string(),
            external_digest_container_path: CLOSED_PLAN_DIGEST_CONTAINER_PATH.to_string(),
            external_digest_host_path: closed_plan_digest_host_path,
            external_digest_sidecar_format: "lowercase_sha256_hex_64_plus_single_lf_v1".to_string(),
            host_path: closed_plan_host_path,
            host_file_gid: 0,
            host_file_mode: "0444".to_string(),
            host_file_uid: 0,
            immediate_pre_start_revalidate_plan_and_sidecar_canonical_path_inode_mode_uid_gid: true,
            mount_read_only: true,
            plan_must_not_contain_own_digest: true,
            publish_directory_fsync: true,
            publish_file_fsync: true,
            retain_published_inode_through_container_start: true,
            retain_sidecar_inode_through_container_start: true,
            sidecar_atomic_no_replace_publish: true,
            sidecar_bytes_derived_only_from_joined_prepared_claim: true,
            sidecar_candidate_compares_before_any_container_stage: true,
            sidecar_directory_fsync: true,
            sidecar_equals_joined_plan_digest: true,
            sidecar_file_fsync: true,
            sidecar_gid: 0,
            sidecar_mode: "0444".to_string(),
            sidecar_reject_existing_or_residue: true,
            sidecar_reopen_and_read_back_exact_bytes: true,
            sidecar_uid: 0,
            supervisor_reopens_and_verifies_exact_bytes: true,
        },
        run_root,
        schema: NIX_CLOSED_RUN_PLAN_SCHEMA.to_string(),
        schema_version: 3,
        seccomp_input: ClosedSeccompInputV1 {
            artifact_host_path: seccomp_host_path,
            artifact_role: ClosedArtifactRoleV1::SeccompProfile,
            canonical_json_required: true,
            docker_request_uses_exact_verified_bytes: true,
            pin: binding.seccomp_profile.clone(),
        },
        source_materialization: ClosedSourceMaterializationPlanV1 {
            archive_host_path: source_archive_host_path,
            archive_role: ClosedArtifactRoleV1::SourceArchive,
            destination_host_path: source_root,
            destination_mode: "0555".to_string(),
            destination_must_not_exist: true,
            expected_tree_manifest_sha256: binding.source_tree_manifest_sha256,
            extraction_policy:
                "zstd_single_frame_git_archive_no_absolute_dotdot_duplicate_special_or_escaping_link_v1"
                    .to_string(),
            materializer_role: ClosedArtifactRoleV1::RunnerBinary,
            read_back_tree_manifest: true,
            read_only_after_materialization: true,
        },
        stages: exact_stages(sandbox),
        successor_receipt_identity_contract,
        tool_execution: exact_tool_execution(
            &external_freeze_root,
            &supervisor_root,
            &driver_root,
            sandbox,
        ),
        verifier_container,
        verifier_docker_exec_policy: ClosedVerifierDockerExecPolicyV1 {
            attach_stderr: true,
            attach_stdin: false,
            attach_stdout: true,
            docker_exec_create_id_retained: true,
            docker_exec_inspect_binds_same_verifier_container_id: true,
            docker_exec_inspect_started_running_exit_code_read_back: true,
            environment_overrides: Vec::new(),
            inherit_caller_environment: false,
            privileged: false,
            start_detach: false,
            start_tty_matches_create: true,
            tty: false,
            user: "65532:65532".to_string(),
            working_directory: SOURCE_CONTAINER_ROOT.to_string(),
        },
        workspace_check_contract,
    })
}

pub fn inspect_canonical_nix_closed_run_plan(
    bytes: &[u8],
) -> Result<InspectedNixClosedRunPlanV1, NixMnlError> {
    if bytes.is_empty() || bytes.len() > MAX_NIX_CLOSED_RUN_PLAN_BYTES {
        return Err(invalid(
            "closed Nix run plan byte length is outside its bound",
        ));
    }
    let plan: NixClosedRunPlanWireV1 = serde_json::from_slice(bytes)
        .map_err(|error| invalid(format!("closed Nix run plan is malformed: {error}")))?;
    let canonical = serde_json::to_vec(&plan)?;
    if canonical != bytes {
        return Err(invalid("closed Nix run plan is not exact canonical JSON"));
    }
    if plan.schema != NIX_CLOSED_RUN_PLAN_SCHEMA || plan.schema_version != 3 {
        return Err(invalid("closed Nix run plan schema or version differs"));
    }
    if !plan.authority.is_fully_closed() {
        return Err(invalid("closed Nix run plan carries authority"));
    }
    let expected = derive_nix_closed_run_plan(plan.binding.clone())?;
    if plan != expected {
        return Err(invalid(
            "closed Nix run plan differs from its exact derived plan",
        ));
    }
    Ok(InspectedNixClosedRunPlanV1 {
        binding: plan.binding.clone(),
        boot_id_sha256: plan.binding.boot_id_sha256,
        canonical_bytes: canonical,
        challenge_nonce_sha256: plan.binding.challenge_nonce_sha256,
        closed_run_plan_sha256: sha256_hex(bytes),
        disposition: plan.disposition,
        final_artifact_freeze_payload_sha256: plan.binding.final_artifact_freeze_payload_sha256,
        final_artifact_freeze_profile_id: plan.binding.final_artifact_freeze_profile_id,
        host_identity_sha256: plan.binding.host_identity_sha256,
        profile_id: plan.binding.profile_id,
        run_identity_sha256: plan.binding.run_identity_sha256,
        run_nonce_sha256: plan.binding.run_nonce_sha256,
    })
}

fn successor_receipt_identity_contract(
    binding: &NixClosedRunPlanBindingV1,
) -> NixSuccessorReceiptIdentityContractV2 {
    NixSuccessorReceiptIdentityContractV2 {
        boot_id_sha256: binding.boot_id_sha256.clone(),
        legacy_candidate_evidence_v1_accepted: false,
        receipt_schema: NIX_SUCCESSOR_RECEIPT_SCHEMA.to_string(),
        receipt_schema_version: NIX_SUCCESSOR_RECEIPT_SCHEMA_VERSION,
        run_identity_algorithm: NIX_SUCCESSOR_RUN_IDENTITY_ALGORITHM.to_string(),
        run_identity_schema: NIX_SUCCESSOR_RUN_IDENTITY_SCHEMA.to_string(),
        run_identity_sha256: binding.run_identity_sha256.clone(),
        run_nonce_sha256: binding.run_nonce_sha256.clone(),
    }
}

pub(crate) fn exact_nix_workspace_check_contract(
    binding: &NixClosedRunPlanBindingV1,
) -> NixWorkspaceCheckContractV1 {
    NixWorkspaceCheckContractV1 {
        cargo_nextest_version: CARGO_NEXTEST_VERSION.to_string(),
        cargo_version_requirement: "1.95.0".to_string(),
        expected_inventory_digest_algorithm:
            "sha256_exact_utf8_canonical_json_single_lf_file_bytes_v1".to_string(),
        expected_inventory_file_encoding: "utf8_canonical_json_then_exactly_one_lf_v1".to_string(),
        expected_inventory_is_exact_canonical_json: true,
        expected_inventory_maximum_file_bytes_including_trailing_lf:
            MAX_EXPECTED_WORKSPACE_CHECK_INVENTORY_BYTES,
        expected_inventory_schema: NIX_WORKSPACE_EXPECTED_INVENTORY_SCHEMA.to_string(),
        expected_inventory_sha256: binding.workspace_check_expected_inventory_sha256.clone(),
        expected_inventory_sha256_covers_exact_file_bytes_including_trailing_lf: true,
        expected_inventory_source_relative_path: "nix/hepta-expected-check-inventory-v1.json"
            .to_string(),
        expected_nonempty_suite_count: binding.workspace_check_expected_nonempty_suite_count,
        expected_suite_count: binding.workspace_check_expected_suite_count,
        expected_test_count: binding.workspace_check_expected_test_count,
        flake_attribute: CHECK_FLAKE_ATTRIBUTE.to_string(),
        flaky_result: "unreachable_retries_zero_no_cli_option".to_string(),
        ignore_default_filter: true,
        leak_timeout_period: "200ms".to_string(),
        leak_timeout_result: "fail".to_string(),
        list_message_format: "json".to_string(),
        locked: true,
        nextest_config_exact_bytes_required: true,
        nextest_config_relative_path: "nix/hepta-nextest.toml".to_string(),
        nextest_config_sha256: binding.nextest_config_sha256.clone(),
        nix_system: "x86_64-linux".to_string(),
        no_fail_fast: true,
        offline: true,
        outcome: ClosedWorkspaceCheckOutcomeContractV1 {
            discovered_equals_executed_test_identity_set: true,
            executed_equals_passed_test_identity_set: true,
            expected_equals_discovered_test_identity_set: true,
            required_failed_count: 0,
            required_filtered_out_count: 0,
            required_ignored_count: 0,
            required_measured_count: 0,
            required_retried_count: 0,
            required_skipped_count: 0,
            required_timed_out_count: 0,
        },
        package_count: WORKSPACE_CHECK_PACKAGES.len() as u32,
        package_id_normalized_format: "{package} 0.0.0 (workspace-member:{relative})".to_string(),
        packages: WORKSPACE_CHECK_PACKAGES
            .into_iter()
            .map(str::to_string)
            .collect(),
        profile: "default".to_string(),
        raw_nextest_package_id_cargo_path_uri_required: true,
        raw_nextest_package_id_source_must_equal_suite_cwd: true,
        recipe: exact_nix_workspace_check_recipe(),
        retries: 0,
        run_message_format: "libtest-json-plus".to_string(),
        run_message_format_version: "0.1".to_string(),
        runner_name: "cargo-nextest".to_string(),
        rustc_version_requirement: "1.95.0".to_string(),
        schema: NIX_WORKSPACE_CHECK_CONTRACT_SCHEMA.to_string(),
        schema_version: 1,
        selection: ClosedWorkspaceCheckSelectionV1 {
            all: false,
            all_features: false,
            benchmark_mode: "nextest_list_kind_lib_only;reject_any_additional_target_or_suite"
                .to_string(),
            build_target: "native_x86_64-unknown-linux-gnu".to_string(),
            cargo_target_selection_mode:
                "exact_cargo_metadata_single_lib_roster_joined_to_nextest_list_kind_lib_v1"
                    .to_string(),
            doctests: "cargo_metadata_roster_doctest_false;nextest_no_doctests".to_string(),
            exclude: Vec::new(),
            features: Vec::new(),
            filter_expression: "none".to_string(),
            ignored_test_policy: "reject_inventory_and_do_not_run".to_string(),
            nextest_list_suites_must_join_cargo_metadata_projection: true,
            no_default_features: false,
            package_selection_mode: "explicit_exact_allowlist".to_string(),
            packages: WORKSPACE_CHECK_PACKAGES
                .into_iter()
                .map(str::to_string)
                .collect(),
            partition: "none".to_string(),
            target_selector_argv: Vec::new(),
            test_name_filters: Vec::new(),
            workspace: false,
        },
        slow_timeout_grace_period: "10s".to_string(),
        slow_timeout_on_timeout: "fail".to_string(),
        slow_timeout_period: "900s".to_string(),
        slow_timeout_terminate_after: 1,
        supervisor_reparse: ClosedWorkspaceCheckSupervisorReparseV1 {
            candidate_raw_material_retained_in_check_output: false,
            candidate_reported_recipe_authoritative: false,
            candidate_summary_authoritative: false,
            candidate_summary_contains_subject_product_identity: false,
            candidate_summary_must_equal_recomputed_values: true,
            candidate_summary_relative_path: "share/hepta/check-suite-v1.json".to_string(),
            discovered_inventory_relative_path:
                "share/hepta/check-suite-v1/discovered-inventory.json".to_string(),
            product_and_check_output_and_derivation_paths_must_be_distinct: true,
            raw_capture_independent_of_candidate: true,
            raw_inputs_require_eof_and_no_truncation: true,
            reject_duplicate_unknown_or_unfinished_events: true,
            semantic_inventory_digest_alone_authoritative: false,
            source_workspace_check_only: true,
            subject_product_executed_by_workspace_check: false,
            subject_product_binding_must_equal_retained_product: true,
            supervisor_recomputes_inventory_counts_and_outcomes: true,
            supervisor_reparses_raw_cargo_metadata: true,
            supervisor_reparses_raw_list_and_events: true,
            trusted_supervisor_binds_exact_check_derivation_and_wrapper: true,
            trusted_supervisor_capture_paths_bound_by_future_terminal_evidence: true,
            trusted_supervisor_raw_capture_out_of_store: true,
        },
        suite_scope: "hepta_nix_linux_exact_packages_v1".to_string(),
        target_triple: "x86_64-unknown-linux-gnu".to_string(),
        test_identity_inventory_algorithm: WORKSPACE_CHECK_TEST_IDENTITY_ALGORITHM.to_string(),
        test_threads: 1,
        toolchain_manifest_sha256: binding.workspace_check_toolchain_manifest_sha256.clone(),
        user_config_file: "none".to_string(),
    }
}

fn exact_nix_workspace_check_recipe() -> ClosedWorkspaceCheckRecipeV1 {
    let mut common = vec![
        "cargo".to_string(),
        "nextest".to_string(),
        "--user-config-file".to_string(),
        "none".to_string(),
        "--config-file".to_string(),
        "nix/hepta-nextest.toml".to_string(),
        "--profile".to_string(),
        "default".to_string(),
    ];
    let mut list_argv = common.clone();
    list_argv.push("list".to_string());
    append_workspace_check_common_selection_argv(&mut list_argv);
    list_argv.extend([
        "--list-type".to_string(),
        "full".to_string(),
        "--message-format".to_string(),
        "json".to_string(),
    ]);

    common.push("run".to_string());
    append_workspace_check_common_selection_argv(&mut common);
    common.extend([
        "--no-fail-fast".to_string(),
        "--no-tests".to_string(),
        "fail".to_string(),
        "--retries".to_string(),
        "0".to_string(),
        "--test-threads".to_string(),
        "1".to_string(),
        "--message-format".to_string(),
        "libtest-json-plus".to_string(),
        "--message-format-version".to_string(),
        "0.1".to_string(),
    ]);

    ClosedWorkspaceCheckRecipeV1 {
        archive: "none".to_string(),
        binaries_metadata: "none".to_string(),
        build_jobs: 1,
        build_profile: "test".to_string(),
        caller_manifest_allowed: false,
        candidate_verify_revalidates_discovered_inventory_after_run: true,
        candidate_verify_revalidates_tool_versions_after_run: true,
        cargo_metadata_argv: vec![
            "cargo".to_string(),
            "metadata".to_string(),
            "--locked".to_string(),
            "--offline".to_string(),
            "--no-deps".to_string(),
            "--format-version".to_string(),
            "1".to_string(),
            "--manifest-path".to_string(),
            "Cargo.toml".to_string(),
        ],
        cargo_metadata_projection_preflight_before_nextest_list: true,
        execution_order: vec![
            "capture_exact_tool_versions".to_string(),
            "validate_exact_tool_versions".to_string(),
            "cargo_metadata".to_string(),
            "validate_exact_nextest_config".to_string(),
            "parse_and_compare_cargo_target_projection".to_string(),
            "nextest_list".to_string(),
            "canonicalize_discovered_inventory".to_string(),
            "compare_expected_inventory".to_string(),
            "nextest_run".to_string(),
            "verify_candidate_summary_and_discovered_inventory".to_string(),
        ],
        expected_inventory_compare_argv: vec![
            "cmp".to_string(),
            "--silent".to_string(),
            "<run_unique_tmpdir>/hepta-check-suite-v1/discovered-inventory.json".to_string(),
            "nix/hepta-expected-check-inventory-v1.json".to_string(),
        ],
        expected_inventory_compared_before_nextest_run: true,
        list_and_run_share_target_dir: true,
        list_argv,
        manifest_path: "codex-rs/Cargo.toml".to_string(),
        nextest_config_preflight_before_nextest_list: true,
        nextest_list_launches_test_binaries_for_enumeration: true,
        nextest_reuse_build_option: "absent".to_string(),
        no_run: false,
        no_tests_behavior: "fail".to_string(),
        release: false,
        run_argv: common,
        run_environment: vec![ClosedWorkspaceCheckEnvironmentV1 {
            name: "NEXTEST_EXPERIMENTAL_LIBTEST_JSON".to_string(),
            value: "1".to_string(),
        }],
        target_dir_remap: "none".to_string(),
        tool_versions_preflight_before_cargo_metadata_and_nextest_list: true,
        workspace_remap: "none".to_string(),
        workspace_root: "codex-rs".to_string(),
        wrapper_explicit_environment_overrides: vec![
            ClosedWorkspaceCheckEnvironmentV1 {
                name: "CARGO_BUILD_JOBS".to_string(),
                value: "1".to_string(),
            },
            ClosedWorkspaceCheckEnvironmentV1 {
                name: "CARGO_INCREMENTAL".to_string(),
                value: "0".to_string(),
            },
            ClosedWorkspaceCheckEnvironmentV1 {
                name: "CARGO_NET_OFFLINE".to_string(),
                value: "true".to_string(),
            },
            ClosedWorkspaceCheckEnvironmentV1 {
                name: "CARGO_TARGET_DIR".to_string(),
                value: "<run_unique_tmpdir>/hepta-nextest-target".to_string(),
            },
            ClosedWorkspaceCheckEnvironmentV1 {
                name: "CARGO_TERM_COLOR".to_string(),
                value: "never".to_string(),
            },
            ClosedWorkspaceCheckEnvironmentV1 {
                name: "NO_COLOR".to_string(),
                value: "1".to_string(),
            },
            ClosedWorkspaceCheckEnvironmentV1 {
                name: "RUST_BACKTRACE".to_string(),
                value: "0".to_string(),
            },
        ],
    }
}

fn append_workspace_check_common_selection_argv(argv: &mut Vec<String>) {
    argv.extend([
        "--ignore-default-filter".to_string(),
        "--locked".to_string(),
        "--offline".to_string(),
    ]);
    for package in WORKSPACE_CHECK_PACKAGES {
        argv.push("-p".to_string());
        argv.push(package.to_string());
    }
}

pub(crate) fn nix_workspace_check_contract_sha256(
    binding: &NixClosedRunPlanBindingV1,
) -> Result<String, NixMnlError> {
    Ok(sha256_hex(&serde_json::to_vec(
        &exact_nix_workspace_check_contract(binding),
    )?))
}

pub fn join_nix_closed_run_plan_to_prepared_claim(
    plan: InspectedNixClosedRunPlanV1,
    prepared_claim: codex_hepta_mnl_trust_v1::PreparedPreRunReplayClaimV1,
) -> Result<JoinedNixClosedRunPlanPreparedClaimInspectionV1, NixMnlError> {
    let expected = expected_prepared_claim_lineage_for_inspected_plan(&plan);
    let matched_claim =
        inspect_prepared_pre_run_replay_claim_lineage(prepared_claim, &expected).map_err(
            |error| {
                invalid(format!(
                    "inspected Nix closed plan does not equal its prepared signed replay claim lineage: {error}"
                ))
            },
        )?;
    Ok(JoinedNixClosedRunPlanPreparedClaimInspectionV1 {
        plan,
        matched_claim,
    })
}

pub(crate) fn expected_prepared_claim_lineage_for_inspected_plan(
    plan: &InspectedNixClosedRunPlanV1,
) -> ExpectedPreparedPreRunReplayClaimLineageV1 {
    ExpectedPreparedPreRunReplayClaimLineageV1 {
        boot_id_sha256: plan.boot_id_sha256.clone(),
        challenge_nonce_sha256: plan.challenge_nonce_sha256.clone(),
        final_artifact_freeze_payload_sha256: plan.final_artifact_freeze_payload_sha256.clone(),
        final_artifact_freeze_profile_id: plan.final_artifact_freeze_profile_id.clone(),
        host_identity_sha256: plan.host_identity_sha256.clone(),
        platform_closed_run_plan_sha256: plan.closed_run_plan_sha256.clone(),
        platform_scope: ReplayPlatformScopeV1::Nix,
        profile_id: plan.profile_id.clone(),
        run_identity_sha256: plan.run_identity_sha256.clone(),
        run_nonce_sha256: plan.run_nonce_sha256.clone(),
    }
}

fn validate_binding(binding: &NixClosedRunPlanBindingV1) -> Result<(), NixMnlError> {
    for (value, label) in [
        (&binding.boot_id_sha256, "boot identity"),
        (&binding.challenge_nonce_sha256, "challenge nonce"),
        (&binding.docker_config_sha256, "Docker config"),
        (
            &binding.docker_platform_config_image_id_sha256,
            "Docker platform config image ID",
        ),
        (
            &binding.final_artifact_freeze_payload_sha256,
            "final artifact-freeze payload",
        ),
        (&binding.host_identity_sha256, "host identity"),
        (
            &binding.nix_store_seed_inventory_sha256,
            "Nix store seed inventory",
        ),
        (&binding.nextest_config_sha256, "nextest config"),
        (&binding.run_identity_sha256, "run identity"),
        (&binding.run_nonce_sha256, "run nonce"),
        (&binding.source_tree_manifest_sha256, "source tree manifest"),
        (
            &binding.workspace_check_contract_sha256,
            "workspace check contract",
        ),
        (
            &binding.workspace_check_expected_inventory_sha256,
            "workspace check expected inventory",
        ),
        (
            &binding.workspace_check_toolchain_manifest_sha256,
            "workspace check toolchain manifest",
        ),
    ] {
        require_sha256(value, label)?;
    }
    require_git_oid(&binding.final_tooling.head, "final tooling HEAD")?;
    require_git_oid(&binding.final_tooling.tree, "final tooling tree")?;
    require_identifier(&binding.profile_id, "profile id")?;
    require_identifier(
        &binding.final_artifact_freeze_profile_id,
        "final artifact-freeze profile id",
    )?;
    if binding.run_identity_sha256
        != derive_run_identity_sha256(&binding.run_nonce_sha256, &binding.boot_id_sha256)
            .map_err(|error| invalid(format!("shared run identity input differs: {error}")))?
    {
        return Err(invalid(
            "closed Nix run plan does not use the shared run-identity algorithm",
        ));
    }
    if binding.workspace_check_expected_suite_count != EXACT_EXPECTED_WORKSPACE_CHECK_SUITES
        || binding.workspace_check_expected_nonempty_suite_count == 0
        || binding.workspace_check_expected_nonempty_suite_count
            > binding.workspace_check_expected_suite_count
        || binding.workspace_check_expected_test_count == 0
        || binding.workspace_check_expected_test_count > MAX_EXPECTED_WORKSPACE_CHECK_TESTS
        || binding.workspace_check_expected_test_count
            < binding.workspace_check_expected_nonempty_suite_count
    {
        return Err(invalid(
            "workspace check expected suite or test counts are outside their exact bounds",
        ));
    }
    if binding.workspace_check_contract_sha256 != nix_workspace_check_contract_sha256(binding)? {
        return Err(invalid(
            "workspace check contract digest differs from its exact canonical contract",
        ));
    }
    validate_artifact(&binding.source_archive, "0444", "source archive")?;
    validate_artifact(&binding.seccomp_profile, "0444", "seccomp profile")?;
    validate_artifact_with_max(
        &binding.nix_store_seed_bundle,
        "0444",
        "Nix store seed bundle",
        MAX_STORE_SEED_BYTES,
    )?;
    validate_artifact(&binding.collector_binary, "0555", "collector binary")?;
    validate_artifact(&binding.driver_binary, "0555", "driver binary")?;
    validate_artifact(&binding.runner_binary, "0555", "runner binary")?;
    validate_artifact(&binding.verifier_binary, "0555", "verifier binary")?;
    if binding.cpuset_cpu > MAX_CPU_INDEX {
        return Err(invalid("closed Nix run plan CPU index exceeds its bound"));
    }
    validate_docker_api_version(&binding.docker_api_version)?;
    if binding.docker_platform_config_image_id_sha256 == PINNED_IMAGE_SHA256 {
        return Err(invalid(
            "Docker platform config image ID reuses the manifest digest",
        ));
    }
    match binding.isolation_mode {
        NixIsolationModeV1::NixSandboxEnabled => {
            if binding.presealed_offline_closure_sha256.is_some()
                || binding.presealed_check_output_store_path.is_some()
                || binding.presealed_output_store_path.is_some()
            {
                return Err(invalid(
                    "sandbox plan contains a presealed closure or output path",
                ));
            }
        }
        NixIsolationModeV1::PresealedOfflineClosure => {
            let closure = binding
                .presealed_offline_closure_sha256
                .as_deref()
                .ok_or_else(|| invalid("presealed plan lacks its closure digest"))?;
            require_sha256(closure, "presealed closure")?;
            let output = binding
                .presealed_output_store_path
                .as_deref()
                .ok_or_else(|| invalid("presealed plan lacks its exact output store path"))?;
            validate_nix_store_path(output)?;
            let check_output = binding
                .presealed_check_output_store_path
                .as_deref()
                .ok_or_else(|| invalid("presealed plan lacks its exact check output store path"))?;
            validate_nix_store_path(check_output)?;
            if check_output == output {
                return Err(invalid(
                    "presealed product and check output store paths are not distinct",
                ));
            }
        }
    }
    Ok(())
}

fn validate_artifact(
    artifact: &ClosedArtifactPinV1,
    expected_mode: &str,
    label: &str,
) -> Result<(), NixMnlError> {
    validate_artifact_with_max(artifact, expected_mode, label, MAX_ARTIFACT_BYTES)
}

fn validate_artifact_with_max(
    artifact: &ClosedArtifactPinV1,
    expected_mode: &str,
    label: &str,
    maximum_byte_count: u64,
) -> Result<(), NixMnlError> {
    require_sha256(&artifact.sha256, label)?;
    if artifact.mode != expected_mode
        || artifact.byte_count == 0
        || artifact.byte_count > maximum_byte_count
    {
        return Err(invalid(format!("{label} mode or byte count differs")));
    }
    Ok(())
}

fn exact_run_directories(
    run_root: &str,
    input_root: &str,
    control_root: &str,
    supervisor_root: &str,
    driver_root: &str,
    source_root: &str,
) -> Vec<ClosedRunDirectoryPlanV1> {
    vec![
        run_directory(
            ClosedRunDirectoryRoleV1::RunRoot,
            run_root,
            ClosedRunStageKindV1::RunRootEstablishedAndActiveMarkerPublished,
            "0700",
        ),
        run_directory(
            ClosedRunDirectoryRoleV1::InputArtifacts,
            input_root,
            ClosedRunStageKindV1::ArtifactsStaged,
            "0555",
        ),
        run_directory(
            ClosedRunDirectoryRoleV1::ClosedPlanControl,
            control_root,
            ClosedRunStageKindV1::ClosedPlanPublishedAndReadBack,
            "0555",
        ),
        run_directory(
            ClosedRunDirectoryRoleV1::SupervisorTools,
            supervisor_root,
            ClosedRunStageKindV1::ArtifactsStaged,
            "0555",
        ),
        run_directory(
            ClosedRunDirectoryRoleV1::DriverTools,
            driver_root,
            ClosedRunStageKindV1::ArtifactsStaged,
            "0555",
        ),
        run_directory(
            ClosedRunDirectoryRoleV1::MaterializedSource,
            source_root,
            ClosedRunStageKindV1::SourceMaterialized,
            "0555",
        ),
    ]
}

fn run_directory(
    role: ClosedRunDirectoryRoleV1,
    host_path: &str,
    create_at_stage: ClosedRunStageKindV1,
    final_mode: &str,
) -> ClosedRunDirectoryPlanV1 {
    ClosedRunDirectoryPlanV1 {
        create_at_stage,
        create_gid: 0,
        create_mode: "0700".to_string(),
        create_uid: 0,
        final_mode: final_mode.to_string(),
        final_mode_applied_by_retained_fd_before_first_use: true,
        fsync_directory_after_final_mode: true,
        fsync_parent_after_create: true,
        host_path: host_path.to_string(),
        inode_preserved_across_final_mode_application: true,
        must_not_exist_before_create: true,
        openat2_no_symlink_no_magiclink_no_xdev: true,
        reopen_and_verify_uid_gid_mode_inode_before_first_use: true,
        retain_inode_until_terminal: true,
        role,
    }
}

fn artifact_path(
    role: ClosedArtifactRoleV1,
    external_root: &str,
    staged_root: &str,
    leaf: &str,
    pin: &ClosedArtifactPinV1,
) -> ClosedArtifactPathV1 {
    ClosedArtifactPathV1 {
        external_freeze_path: format!("{external_root}/{leaf}"),
        external_source_gid: 0,
        external_source_uid: 0,
        pin: pin.clone(),
        role,
        host_path: format!("{staged_root}/{leaf}"),
        staged_destination_gid: 0,
        staged_destination_mode_equals_pin: true,
        staged_destination_uid: 0,
    }
}

#[allow(clippy::too_many_arguments)]
fn exact_container(
    binding: &NixClosedRunPlanBindingV1,
    source_root: &str,
    driver_root: &str,
    closed_plan_host_path: &str,
    closed_plan_digest_host_path: &str,
    volume_name: &str,
    artifacts: Vec<ClosedArtifactPathV1>,
    role: ClosedContainerRoleV1,
    store_read_only: bool,
    user: &str,
) -> ClosedContainerSpecV1 {
    let suffix = match role {
        ClosedContainerRoleV1::Builder => "builder",
        ClosedContainerRoleV1::ReadOnlyVerifierSmoke => "verifier",
    };
    let runtime_uid_gid = match role {
        ClosedContainerRoleV1::Builder => 0,
        ClosedContainerRoleV1::ReadOnlyVerifierSmoke => 65_532,
    };
    ClosedContainerSpecV1 {
        api_version: binding.docker_api_version.clone(),
        artifact_paths: artifacts,
        attach_stderr: true,
        attach_stdout: true,
        auto_remove: false,
        bind_mounts: vec![
            ClosedBindMountV1 {
                container_path: SOURCE_CONTAINER_ROOT.to_string(),
                docker_inspect_read_back_exact: true,
                host_gid: 0,
                host_mode: "0555".to_string(),
                host_path: source_root.to_string(),
                host_uid: 0,
                immediate_pre_start_revalidate_canonical_path_inode_mode_uid_gid: true,
                read_only: true,
                retained_host_inode_through_container_start: true,
            },
            ClosedBindMountV1 {
                container_path: "/driver".to_string(),
                docker_inspect_read_back_exact: true,
                host_gid: 0,
                host_mode: "0555".to_string(),
                host_path: driver_root.to_string(),
                host_uid: 0,
                immediate_pre_start_revalidate_canonical_path_inode_mode_uid_gid: true,
                read_only: true,
                retained_host_inode_through_container_start: true,
            },
            ClosedBindMountV1 {
                container_path: CLOSED_PLAN_CONTAINER_PATH.to_string(),
                docker_inspect_read_back_exact: true,
                host_gid: 0,
                host_mode: "0444".to_string(),
                host_path: closed_plan_host_path.to_string(),
                host_uid: 0,
                immediate_pre_start_revalidate_canonical_path_inode_mode_uid_gid: true,
                read_only: true,
                retained_host_inode_through_container_start: true,
            },
            ClosedBindMountV1 {
                container_path: CLOSED_PLAN_DIGEST_CONTAINER_PATH.to_string(),
                docker_inspect_read_back_exact: true,
                host_gid: 0,
                host_mode: "0444".to_string(),
                host_path: closed_plan_digest_host_path.to_string(),
                host_uid: 0,
                immediate_pre_start_revalidate_canonical_path_inode_mode_uid_gid: true,
                read_only: true,
                retained_host_inode_through_container_start: true,
            },
        ],
        cap_add: Vec::new(),
        cap_drop: vec!["ALL".to_string()],
        candidate_can_read_closed_plan: true,
        candidate_can_read_copy_store: false,
        candidate_can_read_evidence: false,
        candidate_can_read_profile: false,
        candidate_can_read_replay_store: false,
        command_arguments: vec![
            "--closed-plan".to_string(),
            CLOSED_PLAN_CONTAINER_PATH.to_string(),
            "--closed-plan-sha256-file".to_string(),
            CLOSED_PLAN_DIGEST_CONTAINER_PATH.to_string(),
            "--container-role".to_string(),
            suffix.to_string(),
        ],
        command_executable: DRIVER_CONTAINER_PATH.to_string(),
        container_name: format!("hepta-nix-mnl-v1-{}-{suffix}", binding.run_identity_sha256),
        devices: Vec::new(),
        dns: Vec::new(),
        docker_create_response_id_retained: true,
        docker_inspect_before_execute: true,
        docker_socket: DOCKER_SOCKET.to_string(),
        environment: exact_environment(
            binding.isolation_mode == NixIsolationModeV1::NixSandboxEnabled,
            role,
        ),
        extra_hosts: Vec::new(),
        host_ipc: false,
        host_pid: false,
        hostname: format!("hepta-nix-{}-{suffix}", &binding.run_identity_sha256[..24]),
        image: PINNED_IMAGE.to_string(),
        image_config_id_sha256: binding.docker_platform_config_image_id_sha256.clone(),
        image_manifest_sha256: PINNED_IMAGE_SHA256.to_string(),
        image_manifest_and_config_read_back_exact: true,
        image_pull_policy: ClosedImagePullPolicyV1::Never,
        inherit_environment: false,
        labels: vec![
            ClosedDockerLabelV1::Literal {
                key: "hepta.mnl.family".to_string(),
                value: "hepta-nix-mnl-v1".to_string(),
            },
            ClosedDockerLabelV1::Literal {
                key: "hepta.mnl.profile".to_string(),
                value: binding.profile_id.clone(),
            },
            ClosedDockerLabelV1::Literal {
                key: "hepta.mnl.run".to_string(),
                value: binding.run_identity_sha256.clone(),
            },
            ClosedDockerLabelV1::Literal {
                key: "hepta.mnl.role".to_string(),
                value: suffix.to_string(),
            },
            ClosedDockerLabelV1::ExternalVerifiedPlanDigest {
                key: "hepta.mnl.plan-sha256".to_string(),
            },
        ],
        labels_read_back_exact: true,
        named_volume_mounts: vec![ClosedNamedVolumeMountV1 {
            name: volume_name.to_string(),
            container_path: NIX_STORE_CONTAINER_ROOT.to_string(),
            create_if_missing: false,
            driver: "local".to_string(),
            driver_options: Vec::new(),
            docker_inspect_exact_identity_before_execute: true,
            immediate_pre_start_revalidate_name_driver_options_labels_mountpoint_identity: true,
            labels: vec![
                ClosedDockerLabelV1::Literal {
                    key: "hepta.mnl.family".to_string(),
                    value: "hepta-nix-mnl-v1".to_string(),
                },
                ClosedDockerLabelV1::Literal {
                    key: "hepta.mnl.profile".to_string(),
                    value: binding.profile_id.clone(),
                },
                ClosedDockerLabelV1::Literal {
                    key: "hepta.mnl.run".to_string(),
                    value: binding.run_identity_sha256.clone(),
                },
                ClosedDockerLabelV1::ExternalVerifiedPlanDigest {
                    key: "hepta.mnl.plan-sha256".to_string(),
                },
            ],
            no_copy: true,
            read_only: store_read_only,
            must_exist_exact_identity: true,
        }],
        network_disabled: true,
        network_mode: "none".to_string(),
        no_new_privileges: true,
        open_stdin: false,
        platform: "linux/amd64".to_string(),
        ports: Vec::new(),
        privileged: false,
        read_only_rootfs: true,
        resources: ClosedResourcePlanV1 {
            cpuset_cpus: binding.cpuset_cpu.to_string(),
            effective_cores: 1,
            effective_max_jobs: 1,
            memory_limit_bytes: MEMORY_LIMIT_BYTES,
            nano_cpus: NANO_CPUS,
            pids_limit: PIDS_LIMIT,
        },
        restart_policy: ClosedRestartPolicyV1::Never,
        retry_count: 0,
        role,
        seccomp_profile_sha256: binding.seccomp_profile.sha256.clone(),
        security_options: vec![
            ClosedSecurityOptionV1::NoNewPrivileges,
            ClosedSecurityOptionV1::SeccompFromVerifiedArtifactBytes {
                role: ClosedArtifactRoleV1::SeccompProfile,
            },
        ],
        tmpfs_mounts: vec![ClosedTmpfsMountV1 {
            container_path: "/tmp".to_string(),
            gid: runtime_uid_gid,
            mode: "0700".to_string(),
            nodev: true,
            noexec: true,
            nosuid: true,
            size_bytes: TMPFS_SIZE_BYTES,
            uid: runtime_uid_gid,
        }],
        tty: false,
        user: user.to_string(),
        working_directory: SOURCE_CONTAINER_ROOT.to_string(),
    }
}

fn exact_environment(sandbox: bool, role: ClosedContainerRoleV1) -> Vec<String> {
    let nix_config = if sandbox {
        "accept-flake-config = false\nbuilders =\nconnect-timeout = 1\ncores = 1\nexperimental-features = nix-command flakes\nflake-registry =\nmax-jobs = 1\nrequire-sigs = true\nsandbox = true\nsandbox-fallback = false\nsubstituters ="
    } else {
        "accept-flake-config = false\nbuilders =\nconnect-timeout = 1\ncores = 1\nexperimental-features = nix-command flakes\nflake-registry =\nmax-jobs = 1\nrequire-sigs = true\nsandbox = false\nsandbox-fallback = false\nsubstituters ="
    };
    let user_name = match role {
        ClosedContainerRoleV1::Builder => "root",
        ClosedContainerRoleV1::ReadOnlyVerifierSmoke => "hepta",
    };
    vec![
        "GIT_CONFIG_COUNT=1".to_string(),
        "GIT_CONFIG_KEY_0=safe.directory".to_string(),
        "GIT_CONFIG_NOSYSTEM=1".to_string(),
        "GIT_CONFIG_VALUE_0=/workspace".to_string(),
        "GIT_OPTIONAL_LOCKS=0".to_string(),
        "HOME=/tmp/home".to_string(),
        "LANG=C".to_string(),
        "LC_ALL=C".to_string(),
        format!("NIX_CONFIG={nix_config}"),
        "NIX_PATH=".to_string(),
        "LOGNAME=".to_string() + user_name,
        "PATH=/nix/var/nix/profiles/default/bin:/nix/var/nix/profiles/default/sbin".to_string(),
        "USER=".to_string() + user_name,
        "XDG_CACHE_HOME=/tmp/cache".to_string(),
        "XDG_CONFIG_HOME=/tmp/config".to_string(),
        "XDG_DATA_HOME=/tmp/data".to_string(),
    ]
}

fn exact_stages(sandbox: bool) -> Vec<ClosedRunStageV1> {
    let mut stages = vec![
        supervisor_stage(ClosedRunStageKindV1::Preflight),
        supervisor_stage(ClosedRunStageKindV1::SignedPlanJoinedToPreparedClaim),
        supervisor_stage(ClosedRunStageKindV1::ExclusiveLock),
        supervisor_stage(ClosedRunStageKindV1::CensusClear),
        supervisor_stage(ClosedRunStageKindV1::RunRootEstablishedAndActiveMarkerPublished),
        supervisor_stage(ClosedRunStageKindV1::ArtifactsStaged),
        supervisor_stage(ClosedRunStageKindV1::ClosedPlanPublishedAndReadBack),
        supervisor_stage(ClosedRunStageKindV1::SourceMaterialized),
        supervisor_stage(ClosedRunStageKindV1::NixStoreProvisioned),
    ];
    if sandbox {
        stages.push(supervisor_stage(
            ClosedRunStageKindV1::BuilderCreatedAndInspected,
        ));
        stages.push(supervisor_stage(
            ClosedRunStageKindV1::BuilderIsolationVerified,
        ));
        stages.push(supervisor_stage(
            ClosedRunStageKindV1::PreRunClaimPublishedClockRecheckedAndImmediateLaunch,
        ));
        stages.push(ClosedRunStageV1 {
            commands: vec![
                command(
                    NIX_BIN,
                    &[
                        literal("--offline"),
                        literal("flake"),
                        literal("metadata"),
                        literal("--json"),
                        literal("--no-write-lock-file"),
                        literal(SOURCE_CONTAINER_ROOT),
                    ],
                    300,
                ),
                command(
                    NIX_BIN,
                    &[
                        literal("--offline"),
                        literal("derivation"),
                        literal("show"),
                        literal("--no-write-lock-file"),
                        literal("/workspace#packages.x86_64-linux.default"),
                    ],
                    600,
                ),
                command(
                    NIX_BIN,
                    &[
                        literal("--offline"),
                        literal("build"),
                        literal("--no-link"),
                        literal("--print-out-paths"),
                        literal("--no-write-lock-file"),
                        literal("--option"),
                        literal("max-jobs"),
                        literal("1"),
                        literal("--option"),
                        literal("cores"),
                        literal("1"),
                        literal("/workspace#packages.x86_64-linux.default"),
                    ],
                    7_200,
                ),
            ],
            executor: ClosedRunExecutorV1::BuilderCandidateDriver,
            kind: ClosedRunStageKindV1::FreshBuild,
        });
        stages.push(ClosedRunStageV1 {
            commands: vec![command(
                NIX_BIN,
                &[
                    literal("--offline"),
                    literal("build"),
                    literal("--no-link"),
                    literal("--print-out-paths"),
                    literal("--no-write-lock-file"),
                    literal(CHECK_FLAKE_ATTRIBUTE),
                ],
                7_200,
            )],
            executor: ClosedRunExecutorV1::BuilderCandidateDriver,
            kind: ClosedRunStageKindV1::RealChecks,
        });
        stages.push(supervisor_stage(
            ClosedRunStageKindV1::BuilderOutputsRetainedAndQualified,
        ));
        stages.push(supervisor_stage(
            ClosedRunStageKindV1::BuilderStoppedNoDescendants,
        ));
    }
    stages.push(supervisor_stage(
        ClosedRunStageKindV1::StoreReopenedReadOnly,
    ));
    if !sandbox {
        stages.push(supervisor_stage(
            ClosedRunStageKindV1::PresealedOutputBindingsRetainedAndQualified,
        ));
    }
    stages.push(supervisor_stage(
        ClosedRunStageKindV1::VerifierCreatedAndInspected,
    ));
    stages.push(supervisor_stage(
        ClosedRunStageKindV1::VerifierIsolationVerified,
    ));
    stages.push(supervisor_stage(if sandbox {
        ClosedRunStageKindV1::VerifierStartedWithinRetainedRunState
    } else {
        ClosedRunStageKindV1::PreRunClaimPublishedClockRecheckedAndImmediateLaunch
    }));
    stages.push(ClosedRunStageV1 {
        commands: output_verification_commands(),
        executor: ClosedRunExecutorV1::SupervisorDockerExecIntoReadOnlyVerifier,
        kind: if sandbox {
            ClosedRunStageKindV1::ReadOnlyArtifactVerification
        } else {
            ClosedRunStageKindV1::PresealedArtifactVerification
        },
    });
    stages.push(dynamic_evidence_stage());
    stages.push(supervisor_stage(ClosedRunStageKindV1::PreSmokeReadBack));
    stages.push(ClosedRunStageV1 {
        commands: vec![ClosedCommandSpecV1 {
            arguments: vec![literal("--version")],
            executable: ClosedExecutableV1::QualifiedProductOutputRelative {
                relative_path: "bin/codex".to_string(),
            },
            inherit_environment: false,
            stderr_limit_bytes: MAX_STDERR_BYTES,
            stdin_closed: true,
            stdout_handling: ClosedStdoutHandlingV1::BoundedCapture {
                maximum_bytes: MAX_STDOUT_BYTES,
            },
            timeout_seconds: 60,
            truncation_is_failure: true,
            working_directory: SOURCE_CONTAINER_ROOT.to_string(),
        }],
        executor: ClosedRunExecutorV1::SupervisorDockerExecIntoReadOnlyVerifier,
        kind: ClosedRunStageKindV1::RuntimeSmoke,
    });
    stages.push(supervisor_stage(ClosedRunStageKindV1::PostSmokeReadBack));
    stages.push(supervisor_stage(
        ClosedRunStageKindV1::VerifierStoppedNoDescendants,
    ));
    stages.push(supervisor_stage(ClosedRunStageKindV1::EvidenceClosed));
    stages
}

fn supervisor_stage(kind: ClosedRunStageKindV1) -> ClosedRunStageV1 {
    ClosedRunStageV1 {
        commands: Vec::new(),
        executor: ClosedRunExecutorV1::SupervisorInternalStateMachine,
        kind,
    }
}

fn dynamic_evidence_stage() -> ClosedRunStageV1 {
    ClosedRunStageV1 {
        commands: Vec::new(),
        executor: ClosedRunExecutorV1::SupervisorDockerExecIntoReadOnlyVerifier,
        kind: ClosedRunStageKindV1::ImmutableEvidenceCollected,
    }
}

fn command(
    path: &str,
    arguments: &[ClosedArgumentV1],
    timeout_seconds: u32,
) -> ClosedCommandSpecV1 {
    ClosedCommandSpecV1 {
        arguments: arguments.to_vec(),
        executable: ClosedExecutableV1::FixedAbsolute {
            path: path.to_string(),
        },
        inherit_environment: false,
        stderr_limit_bytes: MAX_STDERR_BYTES,
        stdin_closed: true,
        stdout_handling: ClosedStdoutHandlingV1::BoundedCapture {
            maximum_bytes: MAX_STDOUT_BYTES,
        },
        timeout_seconds,
        truncation_is_failure: true,
        working_directory: SOURCE_CONTAINER_ROOT.to_string(),
    }
}

fn literal(value: &str) -> ClosedArgumentV1 {
    ClosedArgumentV1::Literal {
        value: value.to_string(),
    }
}

fn output_verification_commands() -> Vec<ClosedCommandSpecV1> {
    let mut commands = vec![
        command(
            NIX_BIN,
            &[
                literal("path-info"),
                literal("--json"),
                literal("-S"),
                ClosedArgumentV1::QualifiedProductOutputStorePath,
            ],
            300,
        ),
        command(
            NIX_STORE_BIN,
            &[
                literal("--verify-path"),
                ClosedArgumentV1::QualifiedProductOutputStorePath,
            ],
            600,
        ),
        command(
            NIX_STORE_BIN,
            &[
                literal("--query"),
                literal("--deriver"),
                ClosedArgumentV1::QualifiedProductOutputStorePath,
            ],
            300,
        ),
        command(
            NIX_STORE_BIN,
            &[
                literal("--query"),
                literal("--requisites"),
                ClosedArgumentV1::QualifiedProductOutputStorePath,
            ],
            600,
        ),
        command(
            NIX_BIN,
            &[
                literal("hash"),
                literal("path"),
                literal("--type"),
                literal("sha256"),
                literal("--base16"),
                ClosedArgumentV1::QualifiedProductOutputStorePath,
            ],
            1_800,
        ),
    ];
    commands.extend([
        command(
            NIX_BIN,
            &[
                literal("path-info"),
                literal("--json"),
                literal("-S"),
                ClosedArgumentV1::QualifiedCheckOutputStorePath,
            ],
            300,
        ),
        command(
            NIX_STORE_BIN,
            &[
                literal("--verify-path"),
                ClosedArgumentV1::QualifiedCheckOutputStorePath,
            ],
            600,
        ),
        command(
            NIX_STORE_BIN,
            &[
                literal("--query"),
                literal("--deriver"),
                ClosedArgumentV1::QualifiedCheckOutputStorePath,
            ],
            300,
        ),
        command(
            NIX_STORE_BIN,
            &[
                literal("--query"),
                literal("--requisites"),
                ClosedArgumentV1::QualifiedCheckOutputStorePath,
            ],
            600,
        ),
        command(
            "/bin/cat",
            &[ClosedArgumentV1::QualifiedCheckOutputRelative {
                relative_path: "share/hepta/check-suite-v1.json".to_string(),
            }],
            300,
        ),
    ]);
    let mut discovered_inventory = command(
        "/bin/cat",
        &[ClosedArgumentV1::QualifiedCheckOutputRelative {
            relative_path: "share/hepta/check-suite-v1/discovered-inventory.json".to_string(),
        }],
        300,
    );
    discovered_inventory.stdout_handling = ClosedStdoutHandlingV1::BoundedCapture {
        maximum_bytes: MAX_WORKSPACE_CHECK_INVENTORY_BYTES,
    };
    commands.push(discovered_inventory);
    commands
}

fn streaming_command(
    path: &str,
    arguments: &[ClosedArgumentV1],
    timeout_seconds: u32,
    maximum_bytes: u64,
) -> ClosedCommandSpecV1 {
    let mut spec = command(path, arguments, timeout_seconds);
    spec.stdout_handling = ClosedStdoutHandlingV1::StreamingSha256ToSupervisor {
        maximum_bytes,
        require_eof: true,
    };
    spec
}

fn exact_dynamic_evidence_traversal() -> ClosedDynamicEvidenceTraversalPlanV1 {
    let mut product_derivation_read = command(
        "/bin/cat",
        &[ClosedArgumentV1::QualifiedProductDerivationStorePath],
        300,
    );
    product_derivation_read.stdout_handling = ClosedStdoutHandlingV1::BoundedCapture {
        maximum_bytes: MAX_DERIVATION_BYTES,
    };
    let mut check_derivation_read = command(
        "/bin/cat",
        &[ClosedArgumentV1::QualifiedCheckDerivationStorePath],
        300,
    );
    check_derivation_read.stdout_handling = ClosedStdoutHandlingV1::BoundedCapture {
        maximum_bytes: MAX_DERIVATION_BYTES,
    };
    ClosedDynamicEvidenceTraversalPlanV1 {
        closure_member_commands: vec![
            command(
                NIX_BIN,
                &[
                    literal("path-info"),
                    literal("--json"),
                    literal("-S"),
                    ClosedArgumentV1::QualifiedClosureMemberStorePath,
                ],
                300,
            ),
            command(
                NIX_STORE_BIN,
                &[
                    literal("--verify-path"),
                    ClosedArgumentV1::QualifiedClosureMemberStorePath,
                ],
                600,
            ),
            streaming_command(
                NIX_STORE_BIN,
                &[
                    literal("--dump"),
                    ClosedArgumentV1::QualifiedClosureMemberStorePath,
                ],
                1_800,
                MAX_NAR_STREAM_BYTES,
            ),
        ],
        closure_member_paths_sorted_unique_bytewise: true,
        closure_inventory_is_sorted_unique_union_of_product_and_check_requisites: true,
        closure_requisites_must_include_check_output: true,
        closure_requisites_must_include_product_output: true,
        product_derivation_commands: vec![product_derivation_read],
        check_derivation_commands: vec![check_derivation_read],
        product_and_check_derivation_paths_must_be_distinct_single_canonical_drv_store_paths: true,
        executor: ClosedRunExecutorV1::SupervisorDockerExecIntoReadOnlyVerifier,
        inputs_from_preceding_artifact_verification_retained_output: true,
        maximum_closure_members: MAX_CLOSURE_MEMBERS,
        maximum_total_closure_nar_bytes: MAX_TOTAL_CLOSURE_NAR_BYTES,
        no_shell_or_candidate_selected_commands: true,
        stage_kind: ClosedRunStageKindV1::ImmutableEvidenceCollected,
        supervisor_drives_each_iteration: true,
        traversal_must_complete_before_pre_smoke_read_back: true,
    }
}

fn exact_dynamic_output_handoff(fresh_sandbox_build: bool) -> ClosedDynamicOutputHandoffPlanV1 {
    ClosedDynamicOutputHandoffPlanV1 {
        docker_exec_argv_constructed_by_supervisor_without_shell: true,
        dynamic_derivation_and_closure_queries_retained_after_exit_zero: true,
        fresh_builder_transcript_exit_status_and_no_truncation_bound_before_accept:
            fresh_sandbox_build,
        fresh_outputs_and_target_derivation_outputs_checked_absent_from_pre_builder_inventory:
            fresh_sandbox_build,
        handoff_source: if fresh_sandbox_build {
            ClosedOutputHandoffSourceV1::FreshBuilderAndCheckTranscripts
        } else {
            ClosedOutputHandoffSourceV1::SignedPresealedProductAndCheckBindings
        },
        no_cli_env_file_stdin_or_candidate_selected_dynamic_path: true,
        no_serializable_or_cloneable_handoff_token: true,
        presealed_signed_product_and_check_bindings_verified_before_verifier_create:
            !fresh_sandbox_build,
        product_and_check_paths_must_be_distinct_canonical_store_paths: true,
        qualified_tokens_resolved_only_from_retained_supervisor_state: true,
        retained_state_binds_plan_run_profile_and_verifier_container_id: true,
        fresh_retained_state_also_binds_builder_container_id: fresh_sandbox_build,
        verifier_docker_exec_inspected_for_exit_stdout_stderr_and_truncation: true,
    }
}

fn exact_tool_execution(
    external_freeze_root: &str,
    supervisor_root: &str,
    driver_root: &str,
    fresh_sandbox_build: bool,
) -> ClosedToolExecutionPlanV1 {
    ClosedToolExecutionPlanV1 {
        bootstrap_runner_exec_forbidden_when_typed_final_freeze_unavailable: true,
        bootstrap_runner_inode_retained_before_preflight: true,
        bootstrap_runner_pin_authority_source:
            ClosedBootstrapPinAuthoritySourceV1::FutureTypedVerifiedFinalArtifactFreezeInspectionOnly,
        bootstrap_runner_pin_never_from_closed_plan_or_caller: true,
        bootstrap_runner_verified_by_trusted_launcher_before_exec: true,
        collector_helper_host_path: format!("{supervisor_root}/codex-hepta-nix-mnl-collector-v1"),
        collector_helper_role: ClosedArtifactRoleV1::CollectorBinary,
        collector_helper_stages: if fresh_sandbox_build {
            vec![
                ClosedRunStageKindV1::BuilderIsolationVerified,
                ClosedRunStageKindV1::VerifierIsolationVerified,
                ClosedRunStageKindV1::PreSmokeReadBack,
                ClosedRunStageKindV1::PostSmokeReadBack,
            ]
        } else {
            vec![
                ClosedRunStageKindV1::VerifierIsolationVerified,
                ClosedRunStageKindV1::PreSmokeReadBack,
                ClosedRunStageKindV1::PostSmokeReadBack,
            ]
        },
        driver_container_entrypoint_path: DRIVER_CONTAINER_PATH.to_string(),
        driver_container_role: ClosedArtifactRoleV1::DriverBinary,
        fresh_driver_builder_stages: if fresh_sandbox_build {
            vec![
                ClosedRunStageKindV1::FreshBuild,
                ClosedRunStageKindV1::RealChecks,
            ]
        } else {
            Vec::new()
        },
        driver_is_only_candidate_container_entrypoint: true,
        driver_staged_host_path: format!("{driver_root}/codex-hepta-nix-mnl-driver-v1"),
        driver_verifies_mounted_plan_and_sidecar_before_any_container_stage: true,
        host_tool_inodes_retained_and_reverified_before_each_invocation: true,
        no_unpinned_or_path_searched_tool: true,
        runner_bootstrap_external_freeze_path: format!(
            "{external_freeze_root}/codex-hepta-nix-mnl-runner-v1"
        ),
        runner_role: ClosedArtifactRoleV1::RunnerBinary,
        runner_drives_all_supervisor_and_docker_api_stages: true,
        runner_staged_readback_must_equal_bootstrap_bytes: true,
        runner_staged_readback_path: format!("{supervisor_root}/codex-hepta-nix-mnl-runner-v1"),
        verifier_helper_host_path: format!("{supervisor_root}/codex-hepta-nix-mnl-verifier-v1"),
        verifier_helper_role: ClosedArtifactRoleV1::VerifierBinary,
        verifier_helper_stages: if fresh_sandbox_build {
            vec![
                ClosedRunStageKindV1::BuilderOutputsRetainedAndQualified,
                ClosedRunStageKindV1::ImmutableEvidenceCollected,
                ClosedRunStageKindV1::EvidenceClosed,
            ]
        } else {
            vec![
                ClosedRunStageKindV1::PresealedOutputBindingsRetainedAndQualified,
                ClosedRunStageKindV1::ImmutableEvidenceCollected,
                ClosedRunStageKindV1::EvidenceClosed,
            ]
        },
    }
}

fn exact_evidence_algorithms() -> ClosedEvidenceAlgorithmPlanV1 {
    ClosedEvidenceAlgorithmPlanV1 {
        artifact_bytes: "sha256_exact_regular_file_bytes_plus_u64be_size_and_octal_mode_v1"
            .to_string(),
        closure_inventory: "sha256_domain_count_then_bytewise_sorted_unique_lp_path_u64be_nar_size_raw32_nar_sha256_v1".to_string(),
        derivation_bytes: "sha256_exact_drv_regular_file_bytes_v1".to_string(),
        nar_stream: "sha256_complete_nix_store_dump_stream_no_truncation_v1".to_string(),
        path_bytes: "sha256_exact_utf8_nix_store_path_without_line_ending_v1".to_string(),
        source_archive_bytes: "sha256_exact_source_archive_bytes_v1".to_string(),
        stderr_bytes: "sha256_complete_raw_stderr_plus_u64be_size_no_truncation_v1".to_string(),
        stdout_bytes: "sha256_complete_raw_stdout_plus_u64be_size_no_truncation_v1".to_string(),
        suite_inventory: WORKSPACE_CHECK_TEST_IDENTITY_ALGORITHM.to_string(),
    }
}

fn exact_evidence_collection(fresh_sandbox_build: bool) -> ClosedEvidenceCollectionPlanV1 {
    ClosedEvidenceCollectionPlanV1 {
        binary_retained_fd_pre_and_post_smoke_read_back: true,
        candidate_self_report_authoritative: false,
        check_output_path_source: if fresh_sandbox_build {
            ClosedOutputPathSourceV1::RealChecksStdoutSingleCanonicalStorePath
        } else {
            ClosedOutputPathSourceV1::SignedPresealedCheckOutputBindingExactCanonicalStorePath
        },
        check_manifest_relative_path: "share/hepta/check-suite-v1.json".to_string(),
        check_manifest_schema: "hepta_nix_mnl_check_suite_result_v1".to_string(),
        closure_members_each_require_path_nar_size_and_nar_sha256: true,
        command_transcripts_bind_argv_exit_stdout_stderr_and_truncation: true,
        derivation_exact_regular_file_bytes: true,
        exact_state_surface_pre_post_inventory_required_before_receipt: true,
        lossless_network_attempt_observer_required_before_receipt: true,
        network_access_prevented_by_enforcement: true,
        network_attempted_receipt_field_available: false,
        nix_store_dump_complete_stream_required: true,
        product_and_check_output_paths_are_distinct: true,
        product_output_path_source: if fresh_sandbox_build {
            ClosedOutputPathSourceV1::FreshBuildStdoutSingleCanonicalStorePath
        } else {
            ClosedOutputPathSourceV1::SignedPresealedBindingExactCanonicalStorePath
        },
        fresh_check_output_required: fresh_sandbox_build,
        fresh_realization_proven_after_immediate_launch: fresh_sandbox_build,
        presealed_check_provenance_is_historical_and_not_fresh: !fresh_sandbox_build,
        presealed_product_and_check_paths_must_be_closure_members: !fresh_sandbox_build,
        presealed_signed_closure_digest_must_equal_recomputed_union_inventory: !fresh_sandbox_build,
        single_product_output_path_required: true,
        runtime_smoke_executable_resolves_only_from_product_output: true,
        state_mutated_receipt_field_available: false,
        supervisor_is_evidence_writer: true,
        test_manifest_requires_all_pass_no_ignored_no_filtered: true,
    }
}

fn require_sha256(value: &str, label: &str) -> Result<(), NixMnlError> {
    if value.len() != 64
        || value.bytes().all(|byte| byte == b'0')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(format!("{label} is not canonical SHA-256")));
    }
    Ok(())
}

fn require_git_oid(value: &str, label: &str) -> Result<(), NixMnlError> {
    if value.len() != 40
        || value.bytes().all(|byte| byte == b'0')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(format!("{label} is not a canonical Git OID")));
    }
    Ok(())
}

fn require_identifier(value: &str, label: &str) -> Result<(), NixMnlError> {
    let mut bytes = value.bytes();
    let Some(first) = bytes.next() else {
        return Err(invalid(format!("{label} is empty")));
    };
    if value.len() > 128
        || !(first.is_ascii_lowercase() || first.is_ascii_digit())
        || !bytes.all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'-' | b'_' | b'.' | b':')
        })
    {
        return Err(invalid(format!("{label} is not a canonical identifier")));
    }
    Ok(())
}

fn validate_docker_api_version(value: &str) -> Result<(), NixMnlError> {
    let Some((major, minor)) = value.split_once('.') else {
        return Err(invalid("Docker API version is not canonical"));
    };
    if major != "1"
        || minor.len() < 2
        || minor.len() > 3
        || !minor.bytes().all(|byte| byte.is_ascii_digit())
        || minor.starts_with('0')
    {
        return Err(invalid("Docker API version is not canonical"));
    }
    Ok(())
}

fn validate_nix_store_path(value: &str) -> Result<(), NixMnlError> {
    let Some(component) = value.strip_prefix("/nix/store/") else {
        return Err(invalid("presealed output path is outside the Nix store"));
    };
    let Some((hash, name)) = component.split_once('-') else {
        return Err(invalid("presealed output path lacks a store name"));
    };
    if hash.len() != 32
        || !hash.bytes().all(|byte| {
            byte.is_ascii_digit()
                || matches!(byte, b'a'..=b'd' | b'f'..=b'n' | b'p'..=b's' | b'v'..=b'z')
        })
        || name.is_empty()
        || name.len() > 211
        || name == "."
        || name == ".."
        || name.starts_with('.')
        || !name.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.' | b'_' | b'?' | b'=')
        })
        || component.contains('/')
    {
        return Err(invalid("presealed output path grammar differs"));
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

use serde::Deserialize;
use serde::Serialize;

pub const CONTRACT_SCHEMA: &str = "hepta_nix_exact_mnl_successor_phase_a_contract_v1";
pub const EVIDENCE_SCHEMA: &str = "hepta_nix_exact_mnl_successor_candidate_evidence_v1";
pub const STATUS_SCHEMA: &str = "hepta_nix_exact_mnl_successor_production_status_v1";
pub const AUTHORITY_SCHEMA: &str = "hepta_nix_exact_mnl_successor_closed_authority_v1";

pub const BACKEND_HEAD: &str = "52ec4b3868fc5272e19ed516d00e11e44c549ea4";
pub const BACKEND_TREE: &str = "247e9e7cfcb41dbfcc8c5b3b531b1e1407c0bd5d";
pub const UI_HEAD: &str = "64612c01de811f647d7f113d3104e2c9d8e17656";
pub const UI_TREE: &str = "7cae3967f9ab878bc67be8083beb9308482725f5";
pub const UI_SOURCE_BLOB_OID: &str = "44e19b3fb9f84da67d94b0d4151a0eca1b9a1862";
pub const UI_SOURCE_PATH: &str = "apps/hepta-control-ui/control-ui.js";
pub const UI_SOURCE_SHA256: &str =
    "8e4fdf8264545f3e0f1dd823c617594e5e6994463ed1723f8b3fd65fb04962b5";
pub const UI_ROUTE_INVENTORY_SHA256: &str =
    "3b57324c845f33f8f0f89d5c69ea716fb3dd948b42a959dabcb1f9c412fdd762";

pub const TOOLING_BASELINE_HEAD: &str = "898628204ff60131b8b015555a3f3a5b2ff80987";
pub const TOOLING_BASELINE_TREE: &str = "4977641b9bf4e91e1f548c73bc7622fc4e2874ee";
pub const SUCCESSOR_PHASE_A_HEAD: &str = "91c1185b8d7e52342e64dfd83f1dfdfab6a99748";
pub const SUCCESSOR_PHASE_A_TREE: &str = "2a86ee4eb9deaaa77743d6cd4fe3796511403083";

pub const FREEZE_RECEIPT_ROOT: &str =
    "vnext-main-52ec4b3868-development-tree-freeze-decision-20260813T111625Z";
pub const FREEZE_MANIFEST_SHA256: &str =
    "4d4ec050cd73ef55a52fcb85d15b7a3bfd8e10ec63f1e9d314f2e1150770fc12";
pub const FREEZE_DECISION_SHA256: &str =
    "9316980821e019f91f3da7380ebf9c473f3c6cfb3a2b01b14a0756de21910a79";
pub const STRATEGY_RECEIPT_ROOT: &str = "vnext-52ec4b3868-upstream-ui-strategy-20260814T075430Z";
pub const STRATEGY_MANIFEST_SHA256: &str =
    "80517be5420e31516a0331a5fbb1f97dc1e5228a3b9cbc23089aa75c4460b926";
pub const STRATEGY_DECISION_SHA256: &str =
    "1e4ffbfe6c6603b3a24e9ff4d25187982dd590f7448cb2932cc356ca118e0231";

pub const HISTORICAL_REV7_ROOT: &str =
    "vnext-main-52ec4b3868-nix-exact-reemitted-rev7-prepared-20260813T185012Z";
pub const HISTORICAL_REV7_OUTER_MANIFEST_SHA256: &str =
    "f81c84fe01076307c80816914d696cf2a2b234b90847c6294b0e283d2ba55ab2";
pub const HISTORICAL_REV7_OUTER_MODE_SHA256: &str =
    "c2ed5d64444054d8ec52fe04f511bdd58c86ab6481cfcf79c71dc70a5bbb9012";
pub const HISTORICAL_REV7_INNER_MANIFEST_SHA256: &str =
    "24e6bf9b8b5bd0134b01ea044582570d45a2085cf019fab43dc3c139b1a45a27";
pub const HISTORICAL_REV7_INNER_MODE_SHA256: &str =
    "5f6be3e09d9373ba794ea6456d071e8f00716c1a4107ae3e7bbc1f4e37f3d7cd";
pub const HISTORICAL_REV7_TERMINAL_SHA256: &str =
    "fde51ffb1695a8201dfe2e3162514511e8e591c22361c5675da8a8a1131da8df";

pub const PINNED_IMAGE: &str =
    "nixos/nix@sha256:d78540374f6a886653cba47d5c3f61c5a41d42e2a8db2607b8d68cb226fd463e";
pub const PINNED_IMAGE_SHA256: &str =
    "d78540374f6a886653cba47d5c3f61c5a41d42e2a8db2607b8d68cb226fd463e";
pub const NIX_VERSION: &str = "nix (Nix) 2.35.1";
pub const MINIMUM_DATA_VOLUME_FREE_BYTES: u64 = 69_793_218_560;

// Phase A deliberately contains no final identity or artifact pin. Those
// values must later arrive in a separately frozen, out-of-tree supervisor
// profile authenticated by an opaque trust context; they must never be hashed
// into the candidate source or accepted as caller arguments.

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryIdentityV1 {
    pub head: String,
    pub tree: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UiRouteStrategyV1 {
    TwentyTwoReadOnlyGetProjectionsDeferredToFirstVnextDevelopmentCycle,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UiIdentityV1 {
    pub backend_contains_ui_tree: bool,
    pub catalog_route_count: u32,
    pub freeze_decision_sha256: String,
    pub freeze_manifest_sha256: String,
    pub head: String,
    pub integration_deferred: bool,
    pub inventory_bytes: u64,
    pub inventory_schema: String,
    pub inventory_serialization: String,
    pub inventory_sha256: String,
    pub preservation_ref: String,
    pub projection_count: u32,
    pub route_strategy: UiRouteStrategyV1,
    pub snapshot_route_count: u32,
    pub source_blob_oid: String,
    pub source_path: String,
    pub source_sha256: String,
    pub tree: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProductIdentityV1 {
    pub backend: RepositoryIdentityV1,
    pub ui: UiIdentityV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolingIdentityV1 {
    pub successor_final_tooling: Option<RepositoryIdentityV1>,
    pub successor_phase_a_ancestry: RepositoryIdentityV1,
    pub tooling_baseline: RepositoryIdentityV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionReceiptV1 {
    pub decision_sha256: String,
    pub decision_size_bytes: u64,
    pub manifest_sha256: String,
    pub receipt_root: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HistoricalDispositionV1 {
    HistoricalContentIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HistoricalRev7V1 {
    pub disposition: HistoricalDispositionV1,
    pub fresh_pass: bool,
    pub inner_manifest_sha256: String,
    pub inner_mode_sha256: String,
    pub original_terminal_claim: String,
    pub outer_manifest_sha256: String,
    pub outer_mode_sha256: String,
    pub receipt_root: String,
    pub terminal_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhaseAExternalTrustStatusV1 {
    pub candidate_source_contains_self_pins: bool,
    pub external_supervisor_profile_available: bool,
    pub external_supervisor_trust_root_available: bool,
    pub immutable_stage_one_manifest_available: bool,
    pub immutable_stage_two_manifest_available: bool,
    pub out_of_tree_freeze_required: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HostContractV1 {
    pub architecture: String,
    pub data_volume_root: String,
    pub docker_platform: String,
    pub kernel_system: String,
    pub minimum_data_volume_free_bytes: u64,
    pub nix_system: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImageContractV1 {
    pub docker_image: String,
    pub docker_image_sha256: String,
    pub effective_cores: u32,
    pub effective_flakes: bool,
    pub effective_max_jobs: u32,
    pub effective_nix_command: bool,
    pub nix_version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedAuthorityV1 {
    pub automatic_transition: bool,
    pub candidate_execution: bool,
    pub container_launch: bool,
    pub cutover: bool,
    pub default_ref_change: bool,
    pub deletion: bool,
    pub enforce: bool,
    pub evidence_acceptance: bool,
    pub full_matrix_claim: bool,
    pub ga_claim: bool,
    pub install: bool,
    pub local_ref_change: bool,
    pub mutation: bool,
    pub operator_acceptance: bool,
    pub outbound: bool,
    pub production: bool,
    pub promotion: bool,
    pub qualification_authority: bool,
    pub recutover: bool,
    pub refs: bool,
    pub remote_ref_change: bool,
    pub retirement: bool,
    pub rollback: bool,
    pub writer_control: bool,
}

impl ClosedAuthorityV1 {
    pub(crate) const fn exact() -> Self {
        Self {
            automatic_transition: false,
            candidate_execution: false,
            container_launch: false,
            cutover: false,
            default_ref_change: false,
            deletion: false,
            enforce: false,
            evidence_acceptance: false,
            full_matrix_claim: false,
            ga_claim: false,
            install: false,
            local_ref_change: false,
            mutation: false,
            operator_acceptance: false,
            outbound: false,
            production: false,
            promotion: false,
            qualification_authority: false,
            recutover: false,
            refs: false,
            remote_ref_change: false,
            retirement: false,
            rollback: false,
            writer_control: false,
        }
    }

    pub fn is_fully_closed(&self) -> bool {
        self == &Self::exact()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhaseAContractV1 {
    pub authority: ClosedAuthorityV1,
    pub authority_schema: String,
    pub development_freeze: DecisionReceiptV1,
    pub external_trust_status: PhaseAExternalTrustStatusV1,
    pub historical_rev7: HistoricalRev7V1,
    pub host: HostContractV1,
    pub image: ImageContractV1,
    pub product: ProductIdentityV1,
    pub schema: String,
    pub schema_version: u32,
    pub strategy: DecisionReceiptV1,
    pub tooling: ToolingIdentityV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductionDispositionV1 {
    Blocked,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProductionStatusV1 {
    pub authority: ClosedAuthorityV1,
    pub blockers: Vec<String>,
    pub disposition: ProductionDispositionV1,
    pub historical_rev7_disposition: HistoricalDispositionV1,
    pub ready_to_plan: bool,
    pub schema: String,
    pub schema_version: u32,
}

/// An unconstructable live plan marker. Phase A has no public constructor.
#[derive(Debug)]
pub struct ProductionRunPlanV1 {
    _private: (),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CandidateDispositionV1 {
    AwaitingExternalSupervisorVerificationNoAuthority,
    ShapeOnlyNoFreshPass,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HostEvidenceV1 {
    pub architecture: String,
    pub boot_id_sha256: String,
    pub data_volume_free_bytes: u64,
    pub data_volume_root: String,
    pub docker_config_sha256: String,
    pub docker_platform: String,
    pub host_identity_sha256: String,
    pub kernel_system: String,
    pub nix_system: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RunIdentityV1 {
    pub boot_id_sha256: String,
    pub run_identity_sha256: String,
    pub run_nonce: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExclusiveCensusV1 {
    pub active_candidate_containers_before: u32,
    pub active_named_volumes_before: u32,
    pub active_runs_before: u32,
    pub exclusive_lock_acquired: bool,
    pub lock_name: String,
    pub lock_nonce: String,
    pub named_volume_unique: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NixIsolationModeV1 {
    NixSandboxEnabled,
    PresealedOfflineClosure,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IsolationEvidenceV1 {
    pub cap_drop_all: bool,
    pub candidate_evidence_access: bool,
    pub cores: u32,
    pub cpuset_single_cpu: bool,
    pub devices_exposed: bool,
    pub driver_read_only: bool,
    pub flake_registry_empty: bool,
    pub host_ipc: bool,
    pub host_pid: bool,
    pub isolation_mode: NixIsolationModeV1,
    pub max_jobs: u32,
    pub memory_limit_bytes: u64,
    pub nano_cpus: u64,
    pub network_none: bool,
    pub nix_store_volume_read_write: bool,
    pub no_new_privileges: bool,
    pub pids_limit: u32,
    pub presealed_offline_closure_sha256: Option<String>,
    pub privileged: bool,
    pub rootfs_read_only: bool,
    pub source_read_only: bool,
    pub substituters_empty: bool,
    pub tmpfs_nodev_noexec_nosuid: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InputEvidenceV1 {
    pub candidate_can_read_evidence: bool,
    pub driver_binary_sha256: String,
    pub driver_source_sha256: String,
    pub product: ProductIdentityV1,
    pub source_archive_sha256: String,
    pub tooling: ToolingIdentityV1,
    pub verifier_binary_sha256: String,
    pub verifier_source_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BuildEvidenceV1 {
    pub binary_mode: String,
    pub binary_sha256: String,
    pub closure_sha256: String,
    pub derivation_path: String,
    pub derivation_path_sha256: String,
    pub derivation_sha256: String,
    pub nar_sha256: String,
    pub output_store_path: String,
    pub output_store_path_sha256: String,
    pub source_archive_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckEvidenceV1 {
    pub all_passed: bool,
    pub failed_count: u32,
    pub ignored_count: u32,
    pub subject_binary_sha256: String,
    pub subject_derivation_path_sha256: String,
    pub subject_derivation_sha256: String,
    pub subject_output_store_path_sha256: String,
    pub suite_inventory_sha256: String,
    pub test_count: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeSmokeV1 {
    pub binary_sha256: String,
    pub completed: bool,
    pub exit_code: i32,
    pub network_attempted: bool,
    pub started: bool,
    pub state_mutated: bool,
    pub stderr_sha256: String,
    pub stdout_sha256: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKindV1 {
    Preflight,
    ExclusiveLock,
    CensusClear,
    IsolationVerified,
    BuildCompleted,
    ChecksCompleted,
    SmokeCompleted,
    EvidenceClosed,
    SupervisorSealed,
    CopyAcknowledgedFinal,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EventV1 {
    pub boot_id_sha256: String,
    pub event_index: u32,
    pub event_sha256: String,
    pub kind: EventKindV1,
    pub payload_sha256: String,
    pub predecessor_event_sha256: Option<String>,
    pub run_nonce: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptPinMethodV1 {
    ExternalOutOfTreeManifestSha256,
    GitTrackedSelfReferential,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommitmentDispositionV1 {
    AwaitingExternalSupervisorSignatureVerificationNoAuthority,
    UnattestedShapeOnlyNoAuthenticatedSupervisorSignature,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CopyAckMaterialV1 {
    pub byte_count: u64,
    pub byte_identical: bool,
    pub bundle_format: String,
    pub copy_session_nonce: String,
    pub destination_failure_domain: String,
    pub destination_identity_sha256: String,
    pub destination_read_back: bool,
    pub destination_sha256: String,
    pub freeze_manifest_statement_sha256: String,
    pub pre_ack_sealed_bundle_sha256: String,
    pub receipt_set_sha256: String,
    pub run_identity_sha256: String,
    pub seal_event_sha256: String,
    pub source_sha256: String,
    pub supervisor_seal_statement_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PreflightFreshnessBindingV1 {
    pub challenge_nonce: String,
    pub expires_generation: u64,
    pub generation_epoch_id: String,
    pub issued_generation: u64,
    pub one_shot: bool,
    pub session_nonce: String,
    pub verification_generation: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TwoStageCommitmentV1 {
    pub close_event_sha256: String,
    pub copy_ack_material: CopyAckMaterialV1,
    pub copy_ack_material_sha256: String,
    pub copy_ack_failure_domain: String,
    pub disposition: CommitmentDispositionV1,
    pub excludes_own_hash_and_git_self_reference: bool,
    pub final_event_anchor_sha256: String,
    pub freeze_manifest: SignedStatementShapeV1,
    pub full_evidence_core_sha256: String,
    pub immutable_after_close: bool,
    pub independent_copy_ack: SignedStatementShapeV1,
    pub out_of_tree: bool,
    pub preflight_freshness: PreflightFreshnessBindingV1,
    pub pre_run_authorization_envelope_sha256: String,
    pub pre_run_authorization_payload_sha256: String,
    pub pre_ack_sealed_bundle_sha256: String,
    pub precommit_payload_sha256: String,
    pub receipt_set_sha256: String,
    pub seal_event_sha256: String,
    pub supervisor_seal: SignedStatementShapeV1,
    pub terminal_manifest: SignedStatementShapeV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedStatementShapeV1 {
    pub algorithm: String,
    pub object_sha256: String,
    pub profile_id: String,
    pub signature_sha256: String,
    pub signer_key_id: String,
    pub statement_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupervisorEvidenceV1 {
    pub authority: ClosedAuthorityV1,
    pub candidate_output_closed_before_commit: bool,
    pub candidate_writes_manifests: bool,
    pub commitment: TwoStageCommitmentV1,
    pub driver_source_sha256: String,
    pub receipt_set_excludes_git_self_reference: bool,
    pub receipt_set_pin_method: ReceiptPinMethodV1,
    pub verifier_source_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateEvidenceV1 {
    pub authority: ClosedAuthorityV1,
    pub build: BuildEvidenceV1,
    pub checks: CheckEvidenceV1,
    pub contract: PhaseAContractV1,
    pub disposition: CandidateDispositionV1,
    pub events: Vec<EventV1>,
    pub exclusive: ExclusiveCensusV1,
    pub host: HostEvidenceV1,
    pub image: ImageContractV1,
    pub input: InputEvidenceV1,
    pub isolation: IsolationEvidenceV1,
    pub run: RunIdentityV1,
    pub runtime_smoke: RuntimeSmokeV1,
    pub schema: String,
    pub schema_version: u32,
    pub supervisor: SupervisorEvidenceV1,
}

/// Opaque marker that only the non-authorizing shape checks passed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedCandidateShapeV1 {
    pub(crate) evidence_sha256: String,
}

impl ValidatedCandidateShapeV1 {
    pub fn evidence_sha256(&self) -> &str {
        &self.evidence_sha256
    }
}

/// Unconstructable verified-production marker. Current code always blocks.
#[derive(Debug)]
pub struct VerifiedProductionEvidenceV1 {
    _private: (),
}

/// Opaque boundary for a separately authenticated supervisor profile.
///
/// Phase A has no constructor and no deserializer for this type. Future code
/// may obtain one only after an external trust-root implementation verifies an
/// immutable out-of-tree profile.
#[derive(Debug)]
pub struct SupervisorTrustContextV1 {
    pub(crate) freshness_lease: SupervisorFreshnessLeaseV1,
    pub(crate) post_run_result: PostRunResultEnvelopeV1,
    pub(crate) pre_run_authorization: FrozenSupervisorProfileEnvelopeV1,
    pub(crate) trust_root: SupervisorTrustRootV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct StatementAuthorizationV1 {
    pub profile_id: String,
    pub signer_key_id: String,
}

/// Canonical frozen profile contents. The payload deliberately has no field
/// for its own digest; the enclosing out-of-tree envelope carries that digest.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct FrozenSupervisorProfilePayloadV1 {
    pub copy_ack_authorization: StatementAuthorizationV1,
    pub copy_ack_destination_failure_domain: String,
    pub copy_ack_destination_identity_sha256: String,
    pub driver_binary_sha256: String,
    pub driver_source_sha256: String,
    pub docker_config_sha256: String,
    pub freeze_manifest_authorization: StatementAuthorizationV1,
    pub freshness_challenge_nonce: String,
    pub freshness_generation_epoch_id: String,
    pub freshness_max_generation_span: u64,
    pub freshness_one_shot_required: bool,
    pub host_identity_sha256: String,
    pub isolation_policy: IsolationEvidenceV1,
    pub production_contract: PhaseAContractV1,
    pub profile_id: String,
    pub result_envelope_authorization: StatementAuthorizationV1,
    pub source_archive_sha256: String,
    pub supervisor_seal_authorization: StatementAuthorizationV1,
    pub terminal_manifest_authorization: StatementAuthorizationV1,
    pub verifier_binary_sha256: String,
    pub verifier_source_sha256: String,
}

#[derive(Debug)]
pub(crate) struct FrozenSupervisorProfileEnvelopeV1 {
    pub authorization: SupervisorProfileAuthorizationV1,
    pub envelope_sha256: String,
    pub payload: FrozenSupervisorProfilePayloadV1,
    pub payload_sha256: String,
}

#[derive(Debug)]
pub(crate) struct SupervisorProfileAuthorizationV1 {
    pub algorithm: String,
    pub authorized_payload_sha256: String,
    pub profile_id: String,
    pub signature_sha256: String,
    pub signer_key_id: String,
    pub trust_root_id: String,
}

/// T-downstream result envelope. Its digest never flows back into P or M.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct PostRunResultPayloadV1 {
    pub copy_ack_material: CopyAckMaterialV1,
    pub copy_ack_material_sha256: String,
    pub final_event_anchor_sha256: String,
    pub freeze_manifest: SignedStatementShapeV1,
    pub independent_copy_ack: SignedStatementShapeV1,
    pub pre_ack_sealed_bundle_sha256: String,
    pub pre_run_authorization_envelope_sha256: String,
    pub pre_run_authorization_payload_sha256: String,
    pub receipt_set_sha256: String,
    pub run_identity_sha256: String,
    pub seal_event_sha256: String,
    pub supervisor_seal: SignedStatementShapeV1,
    pub terminal_manifest: SignedStatementShapeV1,
}

#[derive(Debug)]
pub(crate) struct PostRunResultEnvelopeV1 {
    pub authorization: SupervisorProfileAuthorizationV1,
    pub envelope_sha256: String,
    pub payload: PostRunResultPayloadV1,
    pub payload_sha256: String,
}

#[derive(Debug)]
pub(crate) struct SupervisorTrustRootV1 {
    pub authorized_pre_run_envelope_sha256: String,
    pub authorized_pre_run_payload_sha256: String,
    pub copy_ack_destination_failure_domain: String,
    pub copy_ack_destination_identity_sha256: String,
    pub copy_ack_signer_key_id: String,
    pub profile_authorization_signer_key_id: String,
    pub trust_root_id: String,
}

#[derive(Debug)]
pub(crate) struct SupervisorFreshnessLeaseV1 {
    pub authorized_pre_run_envelope_sha256: String,
    pub authorized_pre_run_payload_sha256: String,
    pub challenge_nonce: String,
    pub consumed_in_this_verification: bool,
    pub copy_session_nonce: String,
    pub expires_generation: u64,
    pub generation_epoch_id: String,
    pub issued_generation: u64,
    pub one_shot: bool,
    pub run_identity_sha256: String,
    pub session_nonce: String,
    pub verification_generation: u64,
}

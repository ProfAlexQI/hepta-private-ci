use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::AUTHORITY_SCHEMA;
use crate::BACKEND_HEAD;
use crate::BACKEND_TREE;
use crate::BuildEvidenceV1;
use crate::CONTRACT_SCHEMA;
use crate::CandidateDispositionV1;
use crate::CandidateEvidenceV1;
use crate::CheckEvidenceV1;
use crate::ClosedAuthorityV1;
use crate::CommitmentDispositionV1;
use crate::DecisionReceiptV1;
use crate::EVIDENCE_SCHEMA;
use crate::EventKindV1;
use crate::EventV1;
use crate::FREEZE_DECISION_SHA256;
use crate::FREEZE_MANIFEST_SHA256;
use crate::FREEZE_RECEIPT_ROOT;
use crate::HISTORICAL_REV7_INNER_MANIFEST_SHA256;
use crate::HISTORICAL_REV7_INNER_MODE_SHA256;
use crate::HISTORICAL_REV7_OUTER_MANIFEST_SHA256;
use crate::HISTORICAL_REV7_OUTER_MODE_SHA256;
use crate::HISTORICAL_REV7_ROOT;
use crate::HISTORICAL_REV7_TERMINAL_SHA256;
use crate::HistoricalDispositionV1;
use crate::HistoricalRev7V1;
use crate::HostContractV1;
use crate::ImageContractV1;
use crate::MAX_CANONICAL_BYTES;
use crate::MINIMUM_DATA_VOLUME_FREE_BYTES;
use crate::NIX_VERSION;
use crate::NixIsolationModeV1;
use crate::NixMnlError;
use crate::PINNED_IMAGE;
use crate::PINNED_IMAGE_SHA256;
use crate::PhaseAContractV1;
use crate::PhaseAExternalTrustStatusV1;
use crate::ProductIdentityV1;
use crate::ProductionDispositionV1;
use crate::ProductionRunPlanV1;
use crate::ProductionStatusV1;
use crate::ReceiptPinMethodV1;
use crate::RepositoryIdentityV1;
use crate::RuntimeSmokeV1;
use crate::STATUS_SCHEMA;
use crate::STRATEGY_DECISION_SHA256;
use crate::STRATEGY_MANIFEST_SHA256;
use crate::STRATEGY_RECEIPT_ROOT;
use crate::SUCCESSOR_PHASE_A_HEAD;
use crate::SUCCESSOR_PHASE_A_TREE;
use crate::SupervisorTrustContextV1;
use crate::TOOLING_BASELINE_HEAD;
use crate::TOOLING_BASELINE_TREE;
use crate::ToolingIdentityV1;
use crate::UI_HEAD;
use crate::UI_ROUTE_INVENTORY_SHA256;
use crate::UI_SOURCE_BLOB_OID;
use crate::UI_SOURCE_PATH;
use crate::UI_SOURCE_SHA256;
use crate::UI_TREE;
use crate::UiIdentityV1;
use crate::UiRouteStrategyV1;
use crate::ValidatedCandidateShapeV1;
use crate::VerifiedProductionEvidenceV1;
use crate::blocked;
use crate::invalid;

const MEMORY_LIMIT_BYTES: u64 = 8 * 1024 * 1024 * 1024;
const NANO_CPUS: u64 = 1_000_000_000;
const PIDS_LIMIT: u32 = 256;
const LOCK_NAME: &str = "hepta-nix-mnl-successor-v1-exclusive";

pub fn exact_phase_a_contract() -> PhaseAContractV1 {
    PhaseAContractV1 {
        authority: ClosedAuthorityV1::exact(),
        authority_schema: AUTHORITY_SCHEMA.to_string(),
        development_freeze: DecisionReceiptV1 {
            decision_sha256: FREEZE_DECISION_SHA256.to_string(),
            decision_size_bytes: 1_314,
            manifest_sha256: FREEZE_MANIFEST_SHA256.to_string(),
            receipt_root: FREEZE_RECEIPT_ROOT.to_string(),
        },
        external_trust_status: PhaseAExternalTrustStatusV1 {
            candidate_source_contains_self_pins: false,
            external_supervisor_profile_available: false,
            external_supervisor_trust_root_available: false,
            immutable_stage_one_manifest_available: false,
            immutable_stage_two_manifest_available: false,
            out_of_tree_freeze_required: true,
        },
        historical_rev7: HistoricalRev7V1 {
            disposition: HistoricalDispositionV1::HistoricalContentIdentity,
            fresh_pass: false,
            inner_manifest_sha256: HISTORICAL_REV7_INNER_MANIFEST_SHA256.to_string(),
            inner_mode_sha256: HISTORICAL_REV7_INNER_MODE_SHA256.to_string(),
            original_terminal_claim: "PASS".to_string(),
            outer_manifest_sha256: HISTORICAL_REV7_OUTER_MANIFEST_SHA256.to_string(),
            outer_mode_sha256: HISTORICAL_REV7_OUTER_MODE_SHA256.to_string(),
            receipt_root: HISTORICAL_REV7_ROOT.to_string(),
            terminal_sha256: HISTORICAL_REV7_TERMINAL_SHA256.to_string(),
        },
        host: HostContractV1 {
            architecture: "x86_64".to_string(),
            data_volume_root: "/data".to_string(),
            docker_platform: "linux/amd64".to_string(),
            kernel_system: "Linux".to_string(),
            minimum_data_volume_free_bytes: MINIMUM_DATA_VOLUME_FREE_BYTES,
            nix_system: "x86_64-linux".to_string(),
        },
        image: ImageContractV1 {
            docker_image: PINNED_IMAGE.to_string(),
            docker_image_sha256: PINNED_IMAGE_SHA256.to_string(),
            effective_cores: 1,
            effective_flakes: true,
            effective_max_jobs: 1,
            effective_nix_command: true,
            nix_version: NIX_VERSION.to_string(),
        },
        product: exact_product(),
        schema: CONTRACT_SCHEMA.to_string(),
        schema_version: 1,
        strategy: DecisionReceiptV1 {
            decision_sha256: STRATEGY_DECISION_SHA256.to_string(),
            decision_size_bytes: 1_666,
            manifest_sha256: STRATEGY_MANIFEST_SHA256.to_string(),
            receipt_root: STRATEGY_RECEIPT_ROOT.to_string(),
        },
        tooling: exact_tooling(),
    }
}

pub fn production_status() -> ProductionStatusV1 {
    ProductionStatusV1 {
        authority: ClosedAuthorityV1::exact(),
        blockers: compiled_blockers(),
        disposition: ProductionDispositionV1::Blocked,
        historical_rev7_disposition: HistoricalDispositionV1::HistoricalContentIdentity,
        ready_to_plan: false,
        schema: STATUS_SCHEMA.to_string(),
        schema_version: 1,
    }
}

/// Phase A cannot produce a run plan. No caller-supplied pins are accepted.
pub fn plan_production_run() -> Result<ProductionRunPlanV1, NixMnlError> {
    require_phase_a_trust_context()?;
    Err(blocked(
        "Nix MNL production planner has no host, Docker, Nix, or driver implementation",
    ))
}

/// Phase A has no Docker/Nix/process implementation behind this boundary.
pub fn execute_live_nix_run_v1() -> Result<(), NixMnlError> {
    Err(blocked(
        "Nix MNL live execution has no implementation or authority",
    ))
}

/// Production verification blocks on the unavailable external trust context
/// before reading evidence; no final identity is compiled into Phase A.
pub fn verify_production_evidence(
    bytes: &[u8],
) -> Result<VerifiedProductionEvidenceV1, NixMnlError> {
    let trust_context = require_phase_a_trust_context()?;
    verify_with_trusted_context(bytes, trust_context)
}

/// Parse and validate only a non-authorizing candidate shape.
pub fn validate_canonical_shape_only(
    bytes: &[u8],
) -> Result<ValidatedCandidateShapeV1, NixMnlError> {
    if bytes.is_empty() || bytes.len() > MAX_CANONICAL_BYTES {
        return Err(invalid(format!(
            "candidate evidence must be non-empty and at most {MAX_CANONICAL_BYTES} bytes"
        )));
    }
    let evidence: CandidateEvidenceV1 = serde_json::from_slice(bytes)
        .map_err(|error| invalid(format!("candidate evidence is malformed: {error}")))?;
    if canonical_json(&evidence)? != bytes {
        return Err(invalid("candidate evidence is not exact canonical JSON"));
    }
    validate_candidate_shape(&evidence)
}

pub fn validate_candidate_shape(
    evidence: &CandidateEvidenceV1,
) -> Result<ValidatedCandidateShapeV1, NixMnlError> {
    validate_candidate_common(
        evidence,
        &exact_phase_a_contract(),
        CandidateDispositionV1::ShapeOnlyNoFreshPass,
    )?;
    Ok(ValidatedCandidateShapeV1 {
        evidence_sha256: sha256(&canonical_json(evidence)?),
    })
}

fn validate_candidate_common(
    evidence: &CandidateEvidenceV1,
    expected_contract: &PhaseAContractV1,
    expected_disposition: CandidateDispositionV1,
) -> Result<(), NixMnlError> {
    if evidence.schema != EVIDENCE_SCHEMA || evidence.schema_version != 1 {
        return Err(invalid("candidate evidence schema/version differs"));
    }
    if &evidence.contract != expected_contract {
        return Err(invalid(
            "candidate contract differs from the selected Phase-A or opaque production profile",
        ));
    }
    if evidence.disposition != expected_disposition {
        return Err(invalid(
            "candidate disposition differs from the selected non-authorizing validation path",
        ));
    }
    require_closed(&evidence.authority, "candidate")?;
    require_closed(&evidence.supervisor.authority, "supervisor")?;
    validate_host(evidence)?;
    validate_run(evidence)?;
    validate_exclusive(evidence)?;
    validate_isolation(evidence)?;
    validate_inputs(evidence)?;
    validate_build_checks_smoke(evidence)?;
    let expected_commitment_disposition = match expected_disposition {
        CandidateDispositionV1::AwaitingExternalSupervisorVerificationNoAuthority => {
            CommitmentDispositionV1::AwaitingExternalSupervisorSignatureVerificationNoAuthority
        }
        CandidateDispositionV1::ShapeOnlyNoFreshPass => {
            CommitmentDispositionV1::UnattestedShapeOnlyNoAuthenticatedSupervisorSignature
        }
    };
    validate_commitment_dag(evidence, expected_commitment_disposition)?;
    Ok(())
}

fn exact_product() -> ProductIdentityV1 {
    ProductIdentityV1 {
        backend: repository(BACKEND_HEAD, BACKEND_TREE),
        ui: UiIdentityV1 {
            backend_contains_ui_tree: false,
            catalog_route_count: 21,
            freeze_decision_sha256: FREEZE_DECISION_SHA256.to_string(),
            freeze_manifest_sha256: FREEZE_MANIFEST_SHA256.to_string(),
            head: UI_HEAD.to_string(),
            integration_deferred: true,
            inventory_bytes: 533,
            inventory_schema: "hepta_control_ui_read_only_get_inventory_v1".to_string(),
            inventory_serialization: "utf8_sorted_method_tab_path_lf_v1".to_string(),
            inventory_sha256: UI_ROUTE_INVENTORY_SHA256.to_string(),
            preservation_ref: "refs/heads/ui/vnext-main".to_string(),
            projection_count: 22,
            route_strategy:
                UiRouteStrategyV1::TwentyTwoReadOnlyGetProjectionsDeferredToFirstVnextDevelopmentCycle,
            snapshot_route_count: 1,
            source_blob_oid: UI_SOURCE_BLOB_OID.to_string(),
            source_path: UI_SOURCE_PATH.to_string(),
            source_sha256: UI_SOURCE_SHA256.to_string(),
            tree: UI_TREE.to_string(),
        },
    }
}

fn exact_tooling() -> ToolingIdentityV1 {
    ToolingIdentityV1 {
        successor_final_tooling: None,
        successor_phase_a_ancestry: repository(SUCCESSOR_PHASE_A_HEAD, SUCCESSOR_PHASE_A_TREE),
        tooling_baseline: repository(TOOLING_BASELINE_HEAD, TOOLING_BASELINE_TREE),
    }
}

fn repository(head: &str, tree: &str) -> RepositoryIdentityV1 {
    RepositoryIdentityV1 {
        head: head.to_string(),
        tree: tree.to_string(),
    }
}

fn compiled_blockers() -> Vec<String> {
    [
        "successor_final_tooling_unfrozen",
        "external_supervisor_profile_missing",
        "external_supervisor_trust_root_missing",
        "immutable_stage_one_manifest_missing",
        "immutable_stage_two_manifest_missing",
        "host_identity_pin_missing_from_external_profile",
        "docker_config_pin_missing_from_external_profile",
        "driver_source_and_binary_pins_missing_from_external_profile",
        "verifier_source_and_binary_pins_missing_from_external_profile",
        "external_receipt_set_pin_missing_from_external_profile",
        "independent_copy_ack_statement_missing",
        "independent_copy_ack_signature_missing",
        "production_role_separated_signature_policy_missing",
        "production_durable_replay_policy_and_crash_reboot_qualification_missing",
        "production_wall_clock_immediate_spawn_state_machine_missing",
        "typed_final_artifact_freeze_semantics_not_joined_to_platform_plan",
        "final_tooling_ancestry_proof_not_joined_to_platform_plan",
        "successor_receipt_run_identity_algorithm_migration_missing",
        "live_read_only_host_collector_and_closed_runner_missing",
        "qualified_workspace_flake_check_output_missing",
        "qualified_nix_sandbox_container_feasibility_missing",
        "auditable_network_attempt_observer_missing",
        "exact_state_mutation_pre_post_inventory_diff_missing",
        "independent_bundle_copy_readback_process_identity_missing",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn require_phase_a_trust_context() -> Result<SupervisorTrustContextV1, NixMnlError> {
    // There is deliberately no environment/file/CLI fallback. A later,
    // independently reviewed integration must replace this with an external
    // trust-root verifier; callers can never pass a context or pins here.
    let context: Option<SupervisorTrustContextV1> = None;
    context.ok_or_else(|| {
        blocked(format!(
            "external supervisor trust context is unavailable before evidence read: {}",
            compiled_blockers().join(",")
        ))
    })
}

pub(crate) fn verify_with_trusted_context(
    bytes: &[u8],
    mut context: SupervisorTrustContextV1,
) -> Result<VerifiedProductionEvidenceV1, NixMnlError> {
    validate_trust_context(&context)?;
    consume_freshness_lease(&mut context)?;
    if bytes.is_empty() || bytes.len() > MAX_CANONICAL_BYTES {
        return Err(invalid("production candidate evidence size differs"));
    }
    let evidence: CandidateEvidenceV1 = serde_json::from_slice(bytes)
        .map_err(|error| invalid(format!("production evidence is malformed: {error}")))?;
    if canonical_json(&evidence)? != bytes {
        return Err(invalid("production evidence is not exact canonical JSON"));
    }
    let expected_contract = expected_production_contract(&context)?;
    validate_candidate_common(
        &evidence,
        &expected_contract,
        CandidateDispositionV1::AwaitingExternalSupervisorVerificationNoAuthority,
    )?;
    validate_trusted_equalities(&evidence, &context)?;
    Err(blocked(
        "common structure, DAG, lease binding, and trusted equalities passed; real detached-signature verification and durable atomic one-shot replay storage remain unimplemented",
    ))
}

fn expected_production_contract(
    context: &SupervisorTrustContextV1,
) -> Result<PhaseAContractV1, NixMnlError> {
    let final_tooling = context
        .pre_run_authorization
        .payload
        .production_contract
        .tooling
        .successor_final_tooling
        .as_ref()
        .ok_or_else(|| invalid("pre-run profile lacks final tooling identity"))?;
    Ok(derived_production_contract(final_tooling))
}

fn derived_production_contract(final_tooling: &RepositoryIdentityV1) -> PhaseAContractV1 {
    let mut contract = exact_phase_a_contract();
    contract.tooling.successor_final_tooling = Some(final_tooling.clone());
    contract
        .external_trust_status
        .external_supervisor_profile_available = true;
    contract
        .external_trust_status
        .external_supervisor_trust_root_available = true;
    contract
        .external_trust_status
        .immutable_stage_one_manifest_available = true;
    contract
        .external_trust_status
        .immutable_stage_two_manifest_available = true;
    contract
}

pub(crate) fn validate_trust_context(
    context: &SupervisorTrustContextV1,
) -> Result<(), NixMnlError> {
    let pre_run = &context.pre_run_authorization;
    let authorization = &pre_run.authorization;
    let profile = &pre_run.payload;
    let result = &context.post_run_result;
    let root = &context.trust_root;
    for (digest, label) in [
        (&pre_run.envelope_sha256, "pre-run authorization envelope"),
        (&pre_run.payload_sha256, "pre-run authorization payload"),
        (
            &authorization.authorized_payload_sha256,
            "pre-run envelope authorized payload",
        ),
        (
            &root.authorized_pre_run_envelope_sha256,
            "trust-root pre-run envelope",
        ),
        (
            &root.authorized_pre_run_payload_sha256,
            "trust-root pre-run payload",
        ),
        (
            &authorization.signature_sha256,
            "pre-run authorization signature",
        ),
        (&profile.host_identity_sha256, "profile host identity"),
        (&profile.driver_source_sha256, "profile driver source"),
        (&profile.driver_binary_sha256, "profile driver binary"),
        (&profile.docker_config_sha256, "profile Docker config"),
        (&profile.verifier_source_sha256, "profile verifier source"),
        (&profile.verifier_binary_sha256, "profile verifier binary"),
        (&profile.source_archive_sha256, "profile source archive"),
        (
            &profile.copy_ack_destination_identity_sha256,
            "profile copy destination identity",
        ),
        (
            &root.copy_ack_destination_identity_sha256,
            "trust-root copy destination identity",
        ),
        (
            &profile.freshness_challenge_nonce,
            "profile freshness challenge",
        ),
        (&result.envelope_sha256, "post-run result envelope"),
        (&result.payload_sha256, "post-run result payload"),
        (
            &result.authorization.authorized_payload_sha256,
            "post-run result authorized payload",
        ),
        (
            &result.authorization.signature_sha256,
            "post-run result authorization signature",
        ),
    ] {
        require_digest(digest, label)?;
    }
    let final_tooling = profile
        .production_contract
        .tooling
        .successor_final_tooling
        .as_ref()
        .ok_or_else(|| invalid("pre-run profile lacks final tooling identity"))?;
    require_git_oid(&final_tooling.head, "profile final tooling head")?;
    require_git_oid(&final_tooling.tree, "profile final tooling tree")?;
    let recomputed_payload_sha256 = frozen_profile_payload_sha256(profile)?;
    let recomputed_envelope_sha256 = frozen_profile_envelope_sha256(pre_run)?;
    let recomputed_result_payload_sha256 = post_run_result_payload_sha256(&result.payload)?;
    let recomputed_result_envelope_sha256 = post_run_result_envelope_sha256(result)?;
    let trust_status = &profile.production_contract.external_trust_status;
    let derived_contract = derived_production_contract(final_tooling);
    if pre_run.payload_sha256 != recomputed_payload_sha256
        || pre_run.envelope_sha256 != recomputed_envelope_sha256
        || authorization.authorized_payload_sha256 != recomputed_payload_sha256
        || root.authorized_pre_run_payload_sha256 != recomputed_payload_sha256
        || root.authorized_pre_run_envelope_sha256 != recomputed_envelope_sha256
        || authorization.algorithm != "ed25519-detached-sha256"
        || authorization.profile_id != profile.profile_id
        || authorization.signer_key_id != root.profile_authorization_signer_key_id
        || authorization.trust_root_id != root.trust_root_id
        || profile.copy_ack_authorization.signer_key_id != root.copy_ack_signer_key_id
        || root.copy_ack_signer_key_id == root.profile_authorization_signer_key_id
        || root.copy_ack_signer_key_id == profile.result_envelope_authorization.signer_key_id
        || profile.copy_ack_destination_failure_domain != root.copy_ack_destination_failure_domain
        || profile.copy_ack_destination_identity_sha256 != root.copy_ack_destination_identity_sha256
        || profile.copy_ack_destination_failure_domain.is_empty()
        || root.trust_root_id.is_empty()
        || root.profile_authorization_signer_key_id.is_empty()
        || profile.profile_id.is_empty()
        || profile.freshness_generation_epoch_id.is_empty()
        || profile.freshness_max_generation_span == 0
        || !profile.freshness_one_shot_required
        || trust_status.candidate_source_contains_self_pins
        || !trust_status.external_supervisor_profile_available
        || !trust_status.external_supervisor_trust_root_available
        || !trust_status.immutable_stage_one_manifest_available
        || !trust_status.immutable_stage_two_manifest_available
        || !trust_status.out_of_tree_freeze_required
        || profile.production_contract != derived_contract
        || !statement_authorization_is_nonempty(&profile.freeze_manifest_authorization)
        || !statement_authorization_is_nonempty(&profile.supervisor_seal_authorization)
        || !statement_authorization_is_nonempty(&profile.copy_ack_authorization)
        || !statement_authorization_is_nonempty(&profile.terminal_manifest_authorization)
        || !statement_authorization_is_nonempty(&profile.result_envelope_authorization)
        || result.payload_sha256 != recomputed_result_payload_sha256
        || result.envelope_sha256 != recomputed_result_envelope_sha256
        || result.authorization.authorized_payload_sha256 != recomputed_result_payload_sha256
        || result.authorization.algorithm != "ed25519-detached-sha256"
        || result.authorization.profile_id != profile.result_envelope_authorization.profile_id
        || result.authorization.signer_key_id != profile.result_envelope_authorization.signer_key_id
        || result.authorization.trust_root_id != root.trust_root_id
        || result.payload.pre_run_authorization_payload_sha256 != recomputed_payload_sha256
        || result.payload.pre_run_authorization_envelope_sha256 != recomputed_envelope_sha256
    {
        return Err(invalid(
            "pre-run authorization or T-downstream result envelope is not domain-separated and equal to its authorized role",
        ));
    }
    validate_freshness_lease(&context.freshness_lease)?;
    if context.freshness_lease.authorized_pre_run_payload_sha256 != recomputed_payload_sha256
        || context.freshness_lease.authorized_pre_run_envelope_sha256 != recomputed_envelope_sha256
        || context.freshness_lease.challenge_nonce != profile.freshness_challenge_nonce
        || context.freshness_lease.generation_epoch_id != profile.freshness_generation_epoch_id
        || context
            .freshness_lease
            .expires_generation
            .saturating_sub(context.freshness_lease.issued_generation)
            > profile.freshness_max_generation_span
        || context.freshness_lease.one_shot != profile.freshness_one_shot_required
    {
        return Err(invalid(
            "freshness lease is not bound to the pre-run authorization and lease policy",
        ));
    }
    Ok(())
}

fn statement_authorization_is_nonempty(value: &crate::StatementAuthorizationV1) -> bool {
    !value.profile_id.is_empty() && !value.signer_key_id.is_empty()
}

fn validate_freshness_lease(lease: &crate::SupervisorFreshnessLeaseV1) -> Result<(), NixMnlError> {
    require_digest(&lease.challenge_nonce, "freshness challenge nonce")?;
    require_digest(&lease.session_nonce, "freshness session nonce")?;
    require_digest(
        &lease.authorized_pre_run_payload_sha256,
        "freshness authorized pre-run payload",
    )?;
    require_digest(
        &lease.authorized_pre_run_envelope_sha256,
        "freshness authorized pre-run envelope",
    )?;
    require_digest(&lease.copy_session_nonce, "freshness copy session nonce")?;
    require_digest(&lease.run_identity_sha256, "freshness run identity")?;
    if lease.challenge_nonce == lease.session_nonce
        || lease.challenge_nonce == lease.copy_session_nonce
        || lease.session_nonce == lease.copy_session_nonce
        || !lease.one_shot
        || lease.consumed_in_this_verification
        || lease.generation_epoch_id.is_empty()
        || lease.issued_generation > lease.verification_generation
        || lease.verification_generation >= lease.expires_generation
        || lease
            .expires_generation
            .saturating_sub(lease.issued_generation)
            > 1
    {
        return Err(invalid(
            "opaque freshness lease is replayed, expired, not one-shot, or generation-invalid",
        ));
    }
    Ok(())
}

fn consume_freshness_lease(context: &mut SupervisorTrustContextV1) -> Result<(), NixMnlError> {
    validate_freshness_lease(&context.freshness_lease)?;
    // This only prevents reuse inside this consumed value. Phase A has no
    // durable atomic replay store, so this mutation is never treated as
    // production freshness authority.
    context.freshness_lease.consumed_in_this_verification = true;
    Ok(())
}

pub(crate) fn validate_trusted_equalities(
    evidence: &CandidateEvidenceV1,
    context: &SupervisorTrustContextV1,
) -> Result<(), NixMnlError> {
    let pre_run = &context.pre_run_authorization;
    let profile = &pre_run.payload;
    let result = &context.post_run_result.payload;
    let commitment = &evidence.supervisor.commitment;
    let freeze = &commitment.freeze_manifest;
    let seal = &commitment.supervisor_seal;
    let copy_ack = &commitment.independent_copy_ack;
    let terminal = &commitment.terminal_manifest;
    let lease = &context.freshness_lease;
    if evidence.contract != profile.production_contract
        || evidence.input.tooling != profile.production_contract.tooling
        || evidence.host.host_identity_sha256 != profile.host_identity_sha256
        || evidence.host.docker_config_sha256 != profile.docker_config_sha256
        || evidence.input.source_archive_sha256 != profile.source_archive_sha256
        || evidence.input.driver_source_sha256 != profile.driver_source_sha256
        || evidence.input.driver_binary_sha256 != profile.driver_binary_sha256
        || evidence.input.verifier_source_sha256 != profile.verifier_source_sha256
        || evidence.input.verifier_binary_sha256 != profile.verifier_binary_sha256
        || evidence.isolation != profile.isolation_policy
        || commitment.pre_run_authorization_payload_sha256 != pre_run.payload_sha256
        || commitment.pre_run_authorization_envelope_sha256 != pre_run.envelope_sha256
        || commitment.copy_ack_failure_domain != profile.copy_ack_destination_failure_domain
        || commitment.copy_ack_material.destination_failure_domain
            != profile.copy_ack_destination_failure_domain
        || commitment.copy_ack_material.destination_identity_sha256
            != profile.copy_ack_destination_identity_sha256
        || freeze.profile_id != profile.freeze_manifest_authorization.profile_id
        || freeze.signer_key_id != profile.freeze_manifest_authorization.signer_key_id
        || seal.profile_id != profile.supervisor_seal_authorization.profile_id
        || seal.signer_key_id != profile.supervisor_seal_authorization.signer_key_id
        || copy_ack.profile_id != profile.copy_ack_authorization.profile_id
        || copy_ack.signer_key_id != profile.copy_ack_authorization.signer_key_id
        || copy_ack.object_sha256 != commitment.copy_ack_material_sha256
        || terminal.profile_id != profile.terminal_manifest_authorization.profile_id
        || terminal.signer_key_id != profile.terminal_manifest_authorization.signer_key_id
        || result.pre_run_authorization_payload_sha256 != pre_run.payload_sha256
        || result.pre_run_authorization_envelope_sha256 != pre_run.envelope_sha256
        || result.run_identity_sha256 != evidence.run.run_identity_sha256
        || result.receipt_set_sha256 != commitment.receipt_set_sha256
        || result.freeze_manifest != *freeze
        || result.supervisor_seal != *seal
        || result.seal_event_sha256 != commitment.seal_event_sha256
        || result.pre_ack_sealed_bundle_sha256 != commitment.pre_ack_sealed_bundle_sha256
        || result.copy_ack_material != commitment.copy_ack_material
        || result.copy_ack_material_sha256 != commitment.copy_ack_material_sha256
        || result.independent_copy_ack != *copy_ack
        || result.final_event_anchor_sha256 != commitment.final_event_anchor_sha256
        || result.terminal_manifest != *terminal
        || lease.authorized_pre_run_payload_sha256
            != commitment.pre_run_authorization_payload_sha256
        || lease.authorized_pre_run_envelope_sha256
            != commitment.pre_run_authorization_envelope_sha256
        || lease.challenge_nonce != commitment.preflight_freshness.challenge_nonce
        || lease.session_nonce != commitment.preflight_freshness.session_nonce
        || lease.generation_epoch_id != commitment.preflight_freshness.generation_epoch_id
        || lease.issued_generation != commitment.preflight_freshness.issued_generation
        || lease.verification_generation != commitment.preflight_freshness.verification_generation
        || lease.expires_generation != commitment.preflight_freshness.expires_generation
        || lease.one_shot != commitment.preflight_freshness.one_shot
        || lease.run_identity_sha256 != evidence.run.run_identity_sha256
        || lease.session_nonce != evidence.run.run_nonce
        || lease.copy_session_nonce != commitment.copy_ack_material.copy_session_nonce
    {
        return Err(invalid(
            "candidate host/driver/verifier/receipt-set/final-tooling/copy-ack is not byte-equal to the authenticated external supervisor profile",
        ));
    }
    Ok(())
}

fn validate_host(evidence: &CandidateEvidenceV1) -> Result<(), NixMnlError> {
    let host = &evidence.host;
    let contract = &evidence.contract.host;
    if host.architecture != contract.architecture
        || host.kernel_system != contract.kernel_system
        || host.nix_system != contract.nix_system
        || host.docker_platform != contract.docker_platform
        || host.data_volume_root != contract.data_volume_root
    {
        return Err(invalid(
            "host is not the exact positive x86_64-linux profile",
        ));
    }
    if host.data_volume_free_bytes < contract.minimum_data_volume_free_bytes {
        return Err(invalid("data volume has less than 69793218560 free bytes"));
    }
    for (digest, label) in [
        (&host.host_identity_sha256, "host identity"),
        (&host.boot_id_sha256, "boot identity"),
        (&host.docker_config_sha256, "Docker config"),
    ] {
        require_digest(digest, label)?;
    }
    if evidence.image != evidence.contract.image {
        return Err(invalid("image or effective Nix configuration differs"));
    }
    Ok(())
}

fn validate_run(evidence: &CandidateEvidenceV1) -> Result<(), NixMnlError> {
    require_digest(&evidence.run.run_nonce, "run nonce")?;
    require_digest(&evidence.run.boot_id_sha256, "run boot identity")?;
    if evidence.run.boot_id_sha256 != evidence.host.boot_id_sha256 {
        return Err(invalid("run/host boot identity transplant"));
    }
    let expected =
        legacy_receipt_run_identity_sha256(&evidence.run.run_nonce, &evidence.run.boot_id_sha256)?;
    if evidence.run.run_identity_sha256 != expected {
        return Err(invalid("run identity digest transplant"));
    }
    Ok(())
}

fn validate_exclusive(evidence: &CandidateEvidenceV1) -> Result<(), NixMnlError> {
    let value = &evidence.exclusive;
    if !value.exclusive_lock_acquired
        || value.lock_name != LOCK_NAME
        || value.lock_nonce != evidence.run.run_nonce
        || value.active_candidate_containers_before != 0
        || value.active_named_volumes_before != 0
        || value.active_runs_before != 0
        || !value.named_volume_unique
    {
        return Err(invalid("exclusive lock or pre-run census differs"));
    }
    Ok(())
}

fn validate_isolation(evidence: &CandidateEvidenceV1) -> Result<(), NixMnlError> {
    let value = &evidence.isolation;
    if !value.network_none
        || !value.rootfs_read_only
        || !value.source_read_only
        || !value.driver_read_only
        || !value.nix_store_volume_read_write
        || value.candidate_evidence_access
        || !value.cap_drop_all
        || !value.no_new_privileges
        || value.privileged
        || value.devices_exposed
        || value.host_pid
        || value.host_ipc
        || !value.tmpfs_nodev_noexec_nosuid
        || value.nano_cpus != NANO_CPUS
        || !value.cpuset_single_cpu
        || value.max_jobs != 1
        || value.cores != 1
        || value.memory_limit_bytes != MEMORY_LIMIT_BYTES
        || value.pids_limit != PIDS_LIMIT
        || !value.substituters_empty
        || !value.flake_registry_empty
    {
        return Err(invalid("container isolation or resource contract differs"));
    }
    match value.isolation_mode {
        NixIsolationModeV1::NixSandboxEnabled => {
            if value.presealed_offline_closure_sha256.is_some() {
                return Err(invalid("sandbox mode cannot smuggle an unpinned closure"));
            }
        }
        NixIsolationModeV1::PresealedOfflineClosure => {
            let digest = value
                .presealed_offline_closure_sha256
                .as_deref()
                .ok_or_else(|| invalid("offline mode lacks presealed closure"))?;
            require_digest(digest, "presealed offline closure")?;
        }
    }
    Ok(())
}

fn validate_inputs(evidence: &CandidateEvidenceV1) -> Result<(), NixMnlError> {
    let input = &evidence.input;
    if input.product != evidence.contract.product || input.tooling != evidence.contract.tooling {
        return Err(invalid("product/tooling identity transplant"));
    }
    if input.candidate_can_read_evidence {
        return Err(invalid("candidate can access supervisor evidence"));
    }
    for (digest, label) in [
        (&input.source_archive_sha256, "source archive"),
        (&input.driver_source_sha256, "driver source"),
        (&input.driver_binary_sha256, "driver binary"),
        (&input.verifier_source_sha256, "verifier source"),
        (&input.verifier_binary_sha256, "verifier binary"),
    ] {
        require_digest(digest, label)?;
    }
    Ok(())
}

fn validate_build_checks_smoke(evidence: &CandidateEvidenceV1) -> Result<(), NixMnlError> {
    let build = &evidence.build;
    for (digest, label) in [
        (&build.derivation_sha256, "derivation"),
        (&build.derivation_path_sha256, "derivation path"),
        (&build.output_store_path_sha256, "output store path"),
        (&build.nar_sha256, "NAR"),
        (&build.closure_sha256, "closure"),
        (&build.binary_sha256, "binary"),
    ] {
        require_digest(digest, label)?;
    }
    let derivation_name = validate_nix_store_path(&build.derivation_path, "derivation path")?;
    let output_name = validate_nix_store_path(&build.output_store_path, "output store path")?;
    let derivation_basename = derivation_name
        .strip_suffix(".drv")
        .ok_or_else(|| invalid("derivation path lacks a single .drv suffix"))?;
    // The Nix store-name grammar permits `.drv` inside a name, but this
    // protocol's derivation role deliberately permits exactly one final
    // `.drv` marker; therefore a basename that itself ends in `.drv` fails.
    if derivation_basename.is_empty()
        || derivation_basename.ends_with(".drv")
        || output_name.ends_with(".drv")
        || build.binary_mode != "0555"
        || build.source_archive_sha256 != evidence.input.source_archive_sha256
        || build.derivation_path_sha256 != sha256(build.derivation_path.as_bytes())
        || build.output_store_path_sha256 != sha256(build.output_store_path.as_bytes())
    {
        return Err(invalid("drv/store/NAR/closure/binary binding differs"));
    }
    let checks = &evidence.checks;
    for (digest, label) in [
        (&checks.suite_inventory_sha256, "check inventory"),
        (&checks.subject_binary_sha256, "check subject binary"),
        (
            &checks.subject_derivation_path_sha256,
            "check subject derivation path",
        ),
        (
            &checks.subject_derivation_sha256,
            "check subject derivation",
        ),
        (
            &checks.subject_output_store_path_sha256,
            "check subject output store path",
        ),
    ] {
        require_digest(digest, label)?;
    }
    if !checks.all_passed
        || checks.test_count == 0
        || checks.failed_count != 0
        || checks.ignored_count != 0
        || checks.subject_binary_sha256 != build.binary_sha256
        || checks.subject_derivation_path_sha256 != build.derivation_path_sha256
        || checks.subject_derivation_sha256 != build.derivation_sha256
        || checks.subject_output_store_path_sha256 != build.output_store_path_sha256
    {
        return Err(invalid(
            "candidate check suite is not exact all-pass/no-skip",
        ));
    }
    let smoke = &evidence.runtime_smoke;
    for (digest, label) in [
        (&smoke.binary_sha256, "smoke binary"),
        (&smoke.stdout_sha256, "smoke stdout"),
        (&smoke.stderr_sha256, "smoke stderr"),
    ] {
        require_digest(digest, label)?;
    }
    if smoke.binary_sha256 != build.binary_sha256
        || !smoke.started
        || !smoke.completed
        || smoke.exit_code != 0
        || smoke.network_attempted
        || smoke.state_mutated
    {
        return Err(invalid(
            "runtime smoke is incomplete, transplanted, or effectful",
        ));
    }
    Ok(())
}

fn validate_commitment_dag(
    evidence: &CandidateEvidenceV1,
    expected_disposition: CommitmentDispositionV1,
) -> Result<(), NixMnlError> {
    let supervisor = &evidence.supervisor;
    let commitment = &supervisor.commitment;
    if !supervisor.candidate_output_closed_before_commit
        || supervisor.candidate_writes_manifests
        || !supervisor.receipt_set_excludes_git_self_reference
        || supervisor.receipt_set_pin_method != ReceiptPinMethodV1::ExternalOutOfTreeManifestSha256
        || supervisor.driver_source_sha256 != evidence.input.driver_source_sha256
        || supervisor.verifier_source_sha256 != evidence.input.verifier_source_sha256
        || commitment.disposition != expected_disposition
        || commitment.copy_ack_failure_domain != "independent-supervisor-copy-ack-failure-domain-v1"
        || !commitment.out_of_tree
        || !commitment.immutable_after_close
        || !commitment.excludes_own_hash_and_git_self_reference
        || commitment.independent_copy_ack.signer_key_id == commitment.freeze_manifest.signer_key_id
        || commitment.independent_copy_ack.signer_key_id == commitment.supervisor_seal.signer_key_id
        || commitment.independent_copy_ack.signer_key_id
            == commitment.terminal_manifest.signer_key_id
    {
        return Err(invalid(
            "shape-only commitment is not an immutable, out-of-tree, non-self-referential boundary",
        ));
    }
    if evidence.events.len() != 10 {
        return Err(invalid("event chain has missing or replayed events"));
    }
    require_digest(
        &commitment.pre_run_authorization_payload_sha256,
        "pre-run authorization payload",
    )?;
    require_digest(
        &commitment.pre_run_authorization_envelope_sha256,
        "pre-run authorization envelope",
    )?;
    let freshness = &commitment.preflight_freshness;
    require_digest(&freshness.challenge_nonce, "preflight freshness challenge")?;
    require_digest(&freshness.session_nonce, "preflight freshness session")?;
    if freshness.challenge_nonce == freshness.session_nonce
        || freshness.session_nonce != evidence.run.run_nonce
        || freshness.generation_epoch_id.is_empty()
        || !freshness.one_shot
        || freshness.issued_generation > freshness.verification_generation
        || freshness.verification_generation >= freshness.expires_generation
        || freshness
            .expires_generation
            .saturating_sub(freshness.issued_generation)
            > 1
    {
        return Err(invalid(
            "preflight freshness binding is not a valid one-shot generation lease",
        ));
    }

    // P -> M: P contains the complete pre-freeze candidate state; M is an
    // explicitly unattested shape of the future independently signed freeze
    // manifest. Neither includes its own or any downstream digest.
    let precommit = precommit_payload_sha256(evidence)?;
    if commitment.precommit_payload_sha256 != precommit {
        return Err(invalid("P precommit payload digest transplant"));
    }
    validate_statement_shape(&commitment.freeze_manifest, &precommit, "M freeze manifest")?;

    // C contains the full evidence core again (not merely P), plus M.
    let core = full_evidence_core_sha256(evidence, &precommit)?;
    if commitment.full_evidence_core_sha256 != core {
        return Err(invalid("C full evidence core digest transplant"));
    }

    // E0 explicitly binds M plus the pre-run authorization and freshness
    // lease, establishing the cryptographic M -> E0 dependency before any
    // ordinary run event. Phase A cannot prove physical wall-clock ordering;
    // the missing live supervisor remains a compiled production blocker.
    // Eclose then closes the complete E0..E6 prefix over C.
    let preclose_payloads = vec![
        preflight_payload_sha256(evidence)?,
        exclusive_lock_payload_sha256(&evidence.exclusive)?,
        exclusive_census_payload_sha256(&evidence.exclusive)?,
        sha256_json(&evidence.isolation)?,
        sha256_json(&evidence.build)?,
        sha256_json(&evidence.checks)?,
        sha256_json(&evidence.runtime_smoke)?,
        core.clone(),
    ];
    validate_event_segment(
        evidence,
        0,
        &[
            EventKindV1::Preflight,
            EventKindV1::ExclusiveLock,
            EventKindV1::CensusClear,
            EventKindV1::IsolationVerified,
            EventKindV1::BuildCompleted,
            EventKindV1::ChecksCompleted,
            EventKindV1::SmokeCompleted,
            EventKindV1::EvidenceClosed,
        ],
        &preclose_payloads,
        None,
    )?;
    let close_event = evidence.events[7].event_sha256.clone();
    if commitment.close_event_sha256 != close_event {
        return Err(invalid("Eclose event anchor transplant"));
    }

    // R covers C and the complete closed prefix. S is the future signed
    // supervisor seal statement shape over R.
    let receipt_set = receipt_set_sha256(evidence, &core, &close_event)?;
    if commitment.receipt_set_sha256 != receipt_set {
        return Err(invalid("R receipt-set digest transplant"));
    }
    validate_statement_shape(
        &commitment.supervisor_seal,
        &receipt_set,
        "S supervisor seal",
    )?;

    // Eseal binds S into the same boot/nonce/predecessor chain.
    validate_event_segment(
        evidence,
        8,
        &[EventKindV1::SupervisorSealed],
        std::slice::from_ref(&commitment.supervisor_seal.statement_sha256),
        Some(close_event),
    )?;
    let seal_event = evidence.events[8].event_sha256.clone();
    if commitment.seal_event_sha256 != seal_event {
        return Err(invalid("Eseal event anchor transplant"));
    }

    // B is the sealed bundle presented out-of-tree. A signs a separate copy
    // material object that binds B to a destination read-back in a distinct
    // failure domain; A never signs B directly and is never a local success bit.
    let bundle_bytes = pre_ack_bundle_bytes(evidence, &receipt_set, &seal_event)?;
    let bundle = sha256(&bundle_bytes);
    let bundle_byte_count = u64::try_from(bundle_bytes.len())
        .map_err(|_| invalid("canonical pre-ack bundle byte length exceeds u64"))?;
    if commitment.pre_ack_sealed_bundle_sha256 != bundle {
        return Err(invalid("B pre-ack sealed bundle digest transplant"));
    }
    let copy_material = &commitment.copy_ack_material;
    let copy_material_sha256 = copy_ack_material_sha256(copy_material)?;
    if commitment.copy_ack_material_sha256 != copy_material_sha256
        || copy_material.freeze_manifest_statement_sha256
            != commitment.freeze_manifest.statement_sha256
        || copy_material.run_identity_sha256 != evidence.run.run_identity_sha256
        || copy_material.receipt_set_sha256 != receipt_set
        || copy_material.supervisor_seal_statement_sha256
            != commitment.supervisor_seal.statement_sha256
        || copy_material.seal_event_sha256 != seal_event
        || copy_material.pre_ack_sealed_bundle_sha256 != bundle
        || copy_material.source_sha256 != bundle
        || copy_material.destination_sha256 != bundle
        || copy_material.byte_count != bundle_byte_count
        || copy_material.bundle_format != "canonical-single-bundle-bytes-v1"
        || copy_material.destination_failure_domain != commitment.copy_ack_failure_domain
        || !copy_material.destination_read_back
        || !copy_material.byte_identical
    {
        return Err(invalid(
            "independent copy material does not bind M/run/R/S/Eseal/B to an identical destination read-back",
        ));
    }
    require_digest(
        &copy_material.copy_session_nonce,
        "independent copy session nonce",
    )?;
    require_digest(
        &copy_material.destination_identity_sha256,
        "independent copy destination identity",
    )?;
    validate_statement_shape(
        &commitment.independent_copy_ack,
        &copy_material_sha256,
        "A independent copy acknowledgement",
    )?;

    // Efinal anchors A. T then covers every upstream object and the full event
    // chain, including the final event. T contains no self hash.
    validate_event_segment(
        evidence,
        9,
        &[EventKindV1::CopyAcknowledgedFinal],
        std::slice::from_ref(&commitment.independent_copy_ack.statement_sha256),
        Some(seal_event),
    )?;
    let final_event = evidence.events[9].event_sha256.clone();
    if commitment.final_event_anchor_sha256 != final_event {
        return Err(invalid("Efinal event anchor transplant"));
    }
    let terminal_payload = terminal_payload_sha256(evidence, &precommit, &core)?;
    validate_statement_shape(
        &commitment.terminal_manifest,
        &terminal_payload,
        "T terminal manifest",
    )?;
    Ok(())
}

fn validate_event_segment(
    evidence: &CandidateEvidenceV1,
    start: usize,
    kinds: &[EventKindV1],
    payloads: &[String],
    mut predecessor: Option<String>,
) -> Result<(), NixMnlError> {
    if kinds.len() != payloads.len() || start + kinds.len() > evidence.events.len() {
        return Err(invalid("event segment shape differs"));
    }
    for (offset, (kind, payload)) in kinds.iter().zip(payloads).enumerate() {
        let index = start + offset;
        let event = &evidence.events[index];
        if event.event_index != index as u32
            || &event.kind != kind
            || event.run_nonce != evidence.run.run_nonce
            || event.boot_id_sha256 != evidence.run.boot_id_sha256
            || event.predecessor_event_sha256 != predecessor
            || &event.payload_sha256 != payload
        {
            return Err(invalid(
                "event index, kind, run, boot, predecessor, or payload differs",
            ));
        }
        let expected = event_sha256(event)?;
        if event.event_sha256 != expected {
            return Err(invalid("event digest transplant"));
        }
        predecessor = Some(expected);
    }
    Ok(())
}

#[derive(Serialize)]
struct RunHashMaterial<'a> {
    boot_id_sha256: &'a str,
    run_nonce: &'a str,
    schema: &'static str,
}

// Frozen V1 candidate receipts use this canonical-JSON identity. The successor
// closed plan deliberately uses the shared trust-domain framing instead; a new
// receipt revision must migrate explicitly rather than reinterpret frozen V1.
pub(crate) fn legacy_receipt_run_identity_sha256(
    run_nonce: &str,
    boot_id_sha256: &str,
) -> Result<String, NixMnlError> {
    sha256_json(&RunHashMaterial {
        boot_id_sha256,
        run_nonce,
        schema: "hepta_nix_mnl_run_identity_v1",
    })
}

#[derive(Serialize)]
struct ExclusiveLockPayloadMaterial<'a> {
    exclusive_lock_acquired: bool,
    lock_name: &'a str,
    lock_nonce: &'a str,
    named_volume_unique: bool,
    schema: &'static str,
}

fn exclusive_lock_payload_sha256(
    exclusive: &crate::ExclusiveCensusV1,
) -> Result<String, NixMnlError> {
    sha256_json(&ExclusiveLockPayloadMaterial {
        exclusive_lock_acquired: exclusive.exclusive_lock_acquired,
        lock_name: &exclusive.lock_name,
        lock_nonce: &exclusive.lock_nonce,
        named_volume_unique: exclusive.named_volume_unique,
        schema: "hepta_nix_mnl_exclusive_lock_acquisition_v1",
    })
}

#[derive(Serialize)]
struct ExclusiveCensusPayloadMaterial {
    active_candidate_containers_before: u32,
    active_named_volumes_before: u32,
    active_runs_before: u32,
    schema: &'static str,
}

fn exclusive_census_payload_sha256(
    exclusive: &crate::ExclusiveCensusV1,
) -> Result<String, NixMnlError> {
    sha256_json(&ExclusiveCensusPayloadMaterial {
        active_candidate_containers_before: exclusive.active_candidate_containers_before,
        active_named_volumes_before: exclusive.active_named_volumes_before,
        active_runs_before: exclusive.active_runs_before,
        schema: "hepta_nix_mnl_pre_run_census_v1",
    })
}

#[derive(Serialize)]
struct StatementIdentityMaterial<'a> {
    algorithm: &'a str,
    profile_id: &'a str,
    signer_key_id: &'a str,
}

fn statement_identity(statement: &crate::SignedStatementShapeV1) -> StatementIdentityMaterial<'_> {
    StatementIdentityMaterial {
        algorithm: &statement.algorithm,
        profile_id: &statement.profile_id,
        signer_key_id: &statement.signer_key_id,
    }
}

#[derive(Serialize)]
struct PrecommitPayloadMaterial<'a> {
    authority: &'a ClosedAuthorityV1,
    approved_host_identity_sha256: &'a str,
    commitment_disposition: CommitmentDispositionV1,
    contract: &'a PhaseAContractV1,
    copy_ack_failure_domain: &'a str,
    copy_ack_identity: StatementIdentityMaterial<'a>,
    disposition: CandidateDispositionV1,
    driver_binary_sha256: &'a str,
    driver_source_sha256: &'a str,
    docker_config_sha256: &'a str,
    evidence_schema: &'a str,
    excludes_own_hash_and_git_self_reference: bool,
    freeze_manifest_identity: StatementIdentityMaterial<'a>,
    freshness_challenge_nonce: &'a str,
    freshness_generation_epoch_id: &'a str,
    image: &'a ImageContractV1,
    immutable_after_close: bool,
    isolation_policy: &'a crate::IsolationEvidenceV1,
    out_of_tree: bool,
    product: &'a ProductIdentityV1,
    pre_run_authorization_envelope_sha256: &'a str,
    pre_run_authorization_payload_sha256: &'a str,
    receipt_set_excludes_git_self_reference: bool,
    receipt_set_pin_method: ReceiptPinMethodV1,
    schema: &'static str,
    schema_version: u32,
    source_archive_sha256: &'a str,
    supervisor_authority: &'a ClosedAuthorityV1,
    supervisor_candidate_writes_manifests: bool,
    supervisor_driver_source_sha256: &'a str,
    supervisor_seal_identity: StatementIdentityMaterial<'a>,
    supervisor_verifier_source_sha256: &'a str,
    terminal_manifest_identity: StatementIdentityMaterial<'a>,
    tooling: &'a ToolingIdentityV1,
    verifier_binary_sha256: &'a str,
    verifier_source_sha256: &'a str,
}

fn precommit_material(evidence: &CandidateEvidenceV1) -> PrecommitPayloadMaterial<'_> {
    let commitment = &evidence.supervisor.commitment;
    PrecommitPayloadMaterial {
        authority: &evidence.authority,
        approved_host_identity_sha256: &evidence.host.host_identity_sha256,
        commitment_disposition: commitment.disposition,
        contract: &evidence.contract,
        copy_ack_failure_domain: &commitment.copy_ack_failure_domain,
        copy_ack_identity: statement_identity(&commitment.independent_copy_ack),
        disposition: evidence.disposition,
        driver_binary_sha256: &evidence.input.driver_binary_sha256,
        driver_source_sha256: &evidence.input.driver_source_sha256,
        docker_config_sha256: &evidence.host.docker_config_sha256,
        evidence_schema: &evidence.schema,
        excludes_own_hash_and_git_self_reference: commitment
            .excludes_own_hash_and_git_self_reference,
        freeze_manifest_identity: statement_identity(&commitment.freeze_manifest),
        freshness_challenge_nonce: &commitment.preflight_freshness.challenge_nonce,
        freshness_generation_epoch_id: &commitment.preflight_freshness.generation_epoch_id,
        image: &evidence.image,
        immutable_after_close: commitment.immutable_after_close,
        isolation_policy: &evidence.isolation,
        out_of_tree: commitment.out_of_tree,
        product: &evidence.input.product,
        pre_run_authorization_envelope_sha256: &commitment.pre_run_authorization_envelope_sha256,
        pre_run_authorization_payload_sha256: &commitment.pre_run_authorization_payload_sha256,
        receipt_set_excludes_git_self_reference: evidence
            .supervisor
            .receipt_set_excludes_git_self_reference,
        receipt_set_pin_method: evidence.supervisor.receipt_set_pin_method,
        schema: "hepta_nix_mnl_precommit_payload_v1",
        schema_version: evidence.schema_version,
        source_archive_sha256: &evidence.input.source_archive_sha256,
        supervisor_authority: &evidence.supervisor.authority,
        supervisor_candidate_writes_manifests: evidence.supervisor.candidate_writes_manifests,
        supervisor_driver_source_sha256: &evidence.supervisor.driver_source_sha256,
        supervisor_seal_identity: statement_identity(&commitment.supervisor_seal),
        supervisor_verifier_source_sha256: &evidence.supervisor.verifier_source_sha256,
        terminal_manifest_identity: statement_identity(&commitment.terminal_manifest),
        tooling: &evidence.input.tooling,
        verifier_binary_sha256: &evidence.input.verifier_binary_sha256,
        verifier_source_sha256: &evidence.input.verifier_source_sha256,
    }
}

#[derive(Serialize)]
struct FullEvidenceCoreMaterial<'a> {
    authority: &'a ClosedAuthorityV1,
    build: &'a BuildEvidenceV1,
    candidate_output_closed_before_commit: bool,
    checks: &'a CheckEvidenceV1,
    contract: &'a PhaseAContractV1,
    disposition: CandidateDispositionV1,
    evidence_schema: &'a str,
    evidence_schema_version: u32,
    exclusive: &'a crate::ExclusiveCensusV1,
    freeze_manifest: &'a crate::SignedStatementShapeV1,
    host: &'a crate::HostEvidenceV1,
    image: &'a ImageContractV1,
    input: &'a crate::InputEvidenceV1,
    isolation: &'a crate::IsolationEvidenceV1,
    preclose_event_prefix: &'a [EventV1],
    precommit_payload_sha256: &'a str,
    run: &'a crate::RunIdentityV1,
    runtime_smoke: &'a RuntimeSmokeV1,
    schema: &'static str,
    supervisor_authority: &'a ClosedAuthorityV1,
    supervisor_candidate_writes_manifests: bool,
    supervisor_driver_source_sha256: &'a str,
    supervisor_receipt_set_excludes_git_self_reference: bool,
    supervisor_receipt_set_pin_method: ReceiptPinMethodV1,
    supervisor_verifier_source_sha256: &'a str,
}

fn precommit_payload_sha256(evidence: &CandidateEvidenceV1) -> Result<String, NixMnlError> {
    sha256_json(&precommit_material(evidence))
}

#[derive(Serialize)]
struct PreflightPayloadMaterial<'a> {
    freeze_manifest_statement_sha256: &'a str,
    freshness: &'a crate::PreflightFreshnessBindingV1,
    host: &'a crate::HostEvidenceV1,
    pre_run_authorization_envelope_sha256: &'a str,
    pre_run_authorization_payload_sha256: &'a str,
    run_identity_sha256: &'a str,
    schema: &'static str,
}

fn preflight_payload_sha256(evidence: &CandidateEvidenceV1) -> Result<String, NixMnlError> {
    let commitment = &evidence.supervisor.commitment;
    sha256_json(&PreflightPayloadMaterial {
        freeze_manifest_statement_sha256: &commitment.freeze_manifest.statement_sha256,
        freshness: &commitment.preflight_freshness,
        host: &evidence.host,
        pre_run_authorization_envelope_sha256: &commitment.pre_run_authorization_envelope_sha256,
        pre_run_authorization_payload_sha256: &commitment.pre_run_authorization_payload_sha256,
        run_identity_sha256: &evidence.run.run_identity_sha256,
        schema: "hepta_nix_mnl_preflight_after_freeze_v1",
    })
}

fn full_evidence_core_sha256(
    evidence: &CandidateEvidenceV1,
    precommit_payload_sha256: &str,
) -> Result<String, NixMnlError> {
    sha256_json(&FullEvidenceCoreMaterial {
        authority: &evidence.authority,
        build: &evidence.build,
        candidate_output_closed_before_commit: evidence
            .supervisor
            .candidate_output_closed_before_commit,
        checks: &evidence.checks,
        contract: &evidence.contract,
        disposition: evidence.disposition,
        evidence_schema: &evidence.schema,
        evidence_schema_version: evidence.schema_version,
        exclusive: &evidence.exclusive,
        freeze_manifest: &evidence.supervisor.commitment.freeze_manifest,
        host: &evidence.host,
        image: &evidence.image,
        input: &evidence.input,
        isolation: &evidence.isolation,
        preclose_event_prefix: &evidence.events[..7],
        precommit_payload_sha256,
        run: &evidence.run,
        runtime_smoke: &evidence.runtime_smoke,
        schema: "hepta_nix_mnl_full_evidence_core_v1",
        supervisor_authority: &evidence.supervisor.authority,
        supervisor_candidate_writes_manifests: evidence.supervisor.candidate_writes_manifests,
        supervisor_driver_source_sha256: &evidence.supervisor.driver_source_sha256,
        supervisor_receipt_set_excludes_git_self_reference: evidence
            .supervisor
            .receipt_set_excludes_git_self_reference,
        supervisor_receipt_set_pin_method: evidence.supervisor.receipt_set_pin_method,
        supervisor_verifier_source_sha256: &evidence.supervisor.verifier_source_sha256,
    })
}

#[derive(Serialize)]
struct ReceiptSetMaterial<'a> {
    close_event_sha256: &'a str,
    closed_event_prefix: &'a [EventV1],
    full_evidence_core_sha256: &'a str,
    schema: &'static str,
}

fn receipt_set_sha256(
    evidence: &CandidateEvidenceV1,
    full_evidence_core_sha256: &str,
    close_event_sha256: &str,
) -> Result<String, NixMnlError> {
    sha256_json(&ReceiptSetMaterial {
        close_event_sha256,
        closed_event_prefix: &evidence.events[..8],
        full_evidence_core_sha256,
        schema: "hepta_nix_mnl_receipt_set_v1",
    })
}

#[derive(Serialize)]
struct PreAckBundleMaterial<'a> {
    bundle_format: &'static str,
    receipt_set_sha256: &'a str,
    schema: &'static str,
    seal_event_sha256: &'a str,
    sealed_event_prefix: &'a [EventV1],
    supervisor_seal: &'a crate::SignedStatementShapeV1,
}

fn pre_ack_bundle_bytes(
    evidence: &CandidateEvidenceV1,
    receipt_set_sha256: &str,
    seal_event_sha256: &str,
) -> Result<Vec<u8>, NixMnlError> {
    canonical_json(&PreAckBundleMaterial {
        bundle_format: "canonical-single-bundle-bytes-v1",
        receipt_set_sha256,
        schema: "hepta_nix_mnl_pre_ack_sealed_bundle_v1",
        seal_event_sha256,
        sealed_event_prefix: &evidence.events[..9],
        supervisor_seal: &evidence.supervisor.commitment.supervisor_seal,
    })
}

#[derive(Serialize)]
struct CopyAckHashMaterial<'a> {
    material: &'a crate::CopyAckMaterialV1,
    schema: &'static str,
}

fn copy_ack_material_sha256(material: &crate::CopyAckMaterialV1) -> Result<String, NixMnlError> {
    sha256_json(&CopyAckHashMaterial {
        material,
        schema: "hepta_nix_mnl_independent_copy_ack_material_v1",
    })
}

#[derive(Serialize)]
struct TerminalPayloadMaterial<'a> {
    close_event_sha256: &'a str,
    commitment_disposition: CommitmentDispositionV1,
    copy_ack_failure_domain: &'a str,
    copy_ack: &'a crate::SignedStatementShapeV1,
    copy_ack_material: &'a crate::CopyAckMaterialV1,
    copy_ack_material_sha256: &'a str,
    excludes_own_hash_and_git_self_reference: bool,
    final_event_anchor_sha256: &'a str,
    freeze_manifest: &'a crate::SignedStatementShapeV1,
    full_event_chain: &'a [EventV1],
    full_evidence_core_sha256: &'a str,
    immutable_after_close: bool,
    out_of_tree: bool,
    pre_ack_sealed_bundle_sha256: &'a str,
    precommit_payload_sha256: &'a str,
    receipt_set_sha256: &'a str,
    schema: &'static str,
    seal_event_sha256: &'a str,
    supervisor_seal: &'a crate::SignedStatementShapeV1,
}

fn terminal_payload_sha256(
    evidence: &CandidateEvidenceV1,
    precommit_payload_sha256: &str,
    full_evidence_core_sha256: &str,
) -> Result<String, NixMnlError> {
    let commitment = &evidence.supervisor.commitment;
    sha256_json(&TerminalPayloadMaterial {
        close_event_sha256: &commitment.close_event_sha256,
        commitment_disposition: commitment.disposition,
        copy_ack_failure_domain: &commitment.copy_ack_failure_domain,
        copy_ack: &commitment.independent_copy_ack,
        copy_ack_material: &commitment.copy_ack_material,
        copy_ack_material_sha256: &commitment.copy_ack_material_sha256,
        excludes_own_hash_and_git_self_reference: commitment
            .excludes_own_hash_and_git_self_reference,
        final_event_anchor_sha256: &commitment.final_event_anchor_sha256,
        freeze_manifest: &commitment.freeze_manifest,
        full_event_chain: &evidence.events,
        full_evidence_core_sha256,
        immutable_after_close: commitment.immutable_after_close,
        out_of_tree: commitment.out_of_tree,
        pre_ack_sealed_bundle_sha256: &commitment.pre_ack_sealed_bundle_sha256,
        precommit_payload_sha256,
        receipt_set_sha256: &commitment.receipt_set_sha256,
        schema: "hepta_nix_mnl_terminal_manifest_payload_v1",
        seal_event_sha256: &commitment.seal_event_sha256,
        supervisor_seal: &commitment.supervisor_seal,
    })
}

#[derive(Serialize)]
struct FrozenProfilePayloadHashMaterial<'a> {
    domain: &'static str,
    payload: &'a crate::FrozenSupervisorProfilePayloadV1,
}

pub(crate) fn frozen_profile_payload_sha256(
    payload: &crate::FrozenSupervisorProfilePayloadV1,
) -> Result<String, NixMnlError> {
    sha256_json(&FrozenProfilePayloadHashMaterial {
        domain: "hepta_nix_mnl_frozen_supervisor_profile_payload_v1",
        payload,
    })
}

#[derive(Serialize)]
struct FrozenProfileEnvelopeHashMaterial<'a> {
    algorithm: &'a str,
    authorized_payload_sha256: &'a str,
    domain: &'static str,
    payload_sha256: &'a str,
    profile_id: &'a str,
    signature_sha256: &'a str,
    signer_key_id: &'a str,
    trust_root_id: &'a str,
}

pub(crate) fn frozen_profile_envelope_sha256(
    envelope: &crate::FrozenSupervisorProfileEnvelopeV1,
) -> Result<String, NixMnlError> {
    sha256_json(&FrozenProfileEnvelopeHashMaterial {
        algorithm: &envelope.authorization.algorithm,
        authorized_payload_sha256: &envelope.authorization.authorized_payload_sha256,
        domain: "hepta_nix_mnl_frozen_supervisor_profile_envelope_v1",
        payload_sha256: &envelope.payload_sha256,
        profile_id: &envelope.authorization.profile_id,
        signature_sha256: &envelope.authorization.signature_sha256,
        signer_key_id: &envelope.authorization.signer_key_id,
        trust_root_id: &envelope.authorization.trust_root_id,
    })
}

#[derive(Serialize)]
struct PostRunResultPayloadHashMaterial<'a> {
    domain: &'static str,
    payload: &'a crate::PostRunResultPayloadV1,
}

pub(crate) fn post_run_result_payload_sha256(
    payload: &crate::PostRunResultPayloadV1,
) -> Result<String, NixMnlError> {
    sha256_json(&PostRunResultPayloadHashMaterial {
        domain: "hepta_nix_mnl_post_run_result_payload_v1",
        payload,
    })
}

pub(crate) fn post_run_result_envelope_sha256(
    envelope: &crate::PostRunResultEnvelopeV1,
) -> Result<String, NixMnlError> {
    sha256_json(&FrozenProfileEnvelopeHashMaterial {
        algorithm: &envelope.authorization.algorithm,
        authorized_payload_sha256: &envelope.authorization.authorized_payload_sha256,
        domain: "hepta_nix_mnl_post_run_result_envelope_v1",
        payload_sha256: &envelope.payload_sha256,
        profile_id: &envelope.authorization.profile_id,
        signature_sha256: &envelope.authorization.signature_sha256,
        signer_key_id: &envelope.authorization.signer_key_id,
        trust_root_id: &envelope.authorization.trust_root_id,
    })
}

#[derive(Serialize)]
struct StatementShapeHashMaterial<'a> {
    algorithm: &'a str,
    object_sha256: &'a str,
    profile_id: &'a str,
    schema: &'static str,
    signature_sha256: &'a str,
    signer_key_id: &'a str,
}

fn statement_shape_sha256(
    statement: &crate::SignedStatementShapeV1,
) -> Result<String, NixMnlError> {
    sha256_json(&StatementShapeHashMaterial {
        algorithm: &statement.algorithm,
        object_sha256: &statement.object_sha256,
        profile_id: &statement.profile_id,
        schema: "hepta_nix_mnl_unattested_signed_statement_shape_v1",
        signature_sha256: &statement.signature_sha256,
        signer_key_id: &statement.signer_key_id,
    })
}

fn validate_statement_shape(
    statement: &crate::SignedStatementShapeV1,
    object_sha256: &str,
    label: &str,
) -> Result<(), NixMnlError> {
    require_digest(object_sha256, &format!("{label} object"))?;
    require_digest(
        &statement.signature_sha256,
        &format!("{label} signature shape"),
    )?;
    require_digest(&statement.statement_sha256, &format!("{label} statement"))?;
    if statement.algorithm != "ed25519-detached-sha256"
        || statement.object_sha256 != object_sha256
        || statement.profile_id.is_empty()
        || statement.signer_key_id.is_empty()
        || statement.statement_sha256 != statement_shape_sha256(statement)?
    {
        return Err(invalid(format!(
            "{label} is not an exact unattested signed-statement shape"
        )));
    }
    Ok(())
}

#[derive(Serialize)]
struct EventHashMaterial<'a> {
    boot_id_sha256: &'a str,
    event_index: u32,
    kind: EventKindV1,
    payload_sha256: &'a str,
    predecessor_event_sha256: &'a Option<String>,
    run_nonce: &'a str,
    schema: &'static str,
}

fn event_sha256(event: &EventV1) -> Result<String, NixMnlError> {
    sha256_json(&EventHashMaterial {
        boot_id_sha256: &event.boot_id_sha256,
        event_index: event.event_index,
        kind: event.kind,
        payload_sha256: &event.payload_sha256,
        predecessor_event_sha256: &event.predecessor_event_sha256,
        run_nonce: &event.run_nonce,
        schema: "hepta_nix_mnl_event_v1",
    })
}

#[cfg(test)]
pub(crate) fn populate_derived_fields(
    evidence: &mut CandidateEvidenceV1,
) -> Result<(), NixMnlError> {
    evidence.run.run_identity_sha256 =
        legacy_receipt_run_identity_sha256(&evidence.run.run_nonce, &evidence.run.boot_id_sha256)?;
    evidence.build.derivation_path_sha256 = sha256(evidence.build.derivation_path.as_bytes());
    evidence.build.output_store_path_sha256 = sha256(evidence.build.output_store_path.as_bytes());
    evidence.checks.subject_binary_sha256 = evidence.build.binary_sha256.clone();
    evidence.checks.subject_derivation_path_sha256 = evidence.build.derivation_path_sha256.clone();
    evidence.checks.subject_derivation_sha256 = evidence.build.derivation_sha256.clone();
    evidence.checks.subject_output_store_path_sha256 =
        evidence.build.output_store_path_sha256.clone();
    let precommit = precommit_payload_sha256(evidence)?;
    evidence.supervisor.commitment.precommit_payload_sha256 = precommit.clone();
    evidence.supervisor.commitment.freeze_manifest.object_sha256 = precommit.clone();
    evidence
        .supervisor
        .commitment
        .freeze_manifest
        .statement_sha256 =
        statement_shape_sha256(&evidence.supervisor.commitment.freeze_manifest)?;
    evidence.events.clear();
    let ordinary_kinds = [
        EventKindV1::Preflight,
        EventKindV1::ExclusiveLock,
        EventKindV1::CensusClear,
        EventKindV1::IsolationVerified,
        EventKindV1::BuildCompleted,
        EventKindV1::ChecksCompleted,
        EventKindV1::SmokeCompleted,
    ];
    let ordinary_payloads = vec![
        preflight_payload_sha256(evidence)?,
        exclusive_lock_payload_sha256(&evidence.exclusive)?,
        exclusive_census_payload_sha256(&evidence.exclusive)?,
        sha256_json(&evidence.isolation)?,
        sha256_json(&evidence.build)?,
        sha256_json(&evidence.checks)?,
        sha256_json(&evidence.runtime_smoke)?,
    ];
    let mut predecessor = None;
    append_events(
        evidence,
        &ordinary_kinds,
        &ordinary_payloads,
        &mut predecessor,
    )?;
    let core = full_evidence_core_sha256(evidence, &precommit)?;
    evidence.supervisor.commitment.full_evidence_core_sha256 = core.clone();
    append_events(
        evidence,
        &[EventKindV1::EvidenceClosed],
        std::slice::from_ref(&core),
        &mut predecessor,
    )?;
    let close_event = evidence.events[7].event_sha256.clone();
    evidence.supervisor.commitment.close_event_sha256 = close_event.clone();
    let receipt = receipt_set_sha256(evidence, &core, &close_event)?;
    evidence.supervisor.commitment.receipt_set_sha256 = receipt.clone();
    evidence.supervisor.commitment.supervisor_seal.object_sha256 = receipt.clone();
    evidence
        .supervisor
        .commitment
        .supervisor_seal
        .statement_sha256 =
        statement_shape_sha256(&evidence.supervisor.commitment.supervisor_seal)?;
    let seal_statement_sha256 = evidence
        .supervisor
        .commitment
        .supervisor_seal
        .statement_sha256
        .clone();
    append_events(
        evidence,
        &[EventKindV1::SupervisorSealed],
        &[seal_statement_sha256],
        &mut predecessor,
    )?;
    let seal_event = evidence.events[8].event_sha256.clone();
    evidence.supervisor.commitment.seal_event_sha256 = seal_event.clone();
    let bundle_bytes = pre_ack_bundle_bytes(evidence, &receipt, &seal_event)?;
    let bundle = sha256(&bundle_bytes);
    let bundle_byte_count = u64::try_from(bundle_bytes.len())
        .map_err(|_| invalid("canonical pre-ack bundle byte length exceeds u64"))?;
    evidence.supervisor.commitment.pre_ack_sealed_bundle_sha256 = bundle.clone();
    let freeze_statement_sha256 = evidence
        .supervisor
        .commitment
        .freeze_manifest
        .statement_sha256
        .clone();
    let supervisor_seal_statement_sha256 = evidence
        .supervisor
        .commitment
        .supervisor_seal
        .statement_sha256
        .clone();
    let run_identity_sha256 = evidence.run.run_identity_sha256.clone();
    let copy_material = &mut evidence.supervisor.commitment.copy_ack_material;
    copy_material.byte_count = bundle_byte_count;
    copy_material.destination_sha256 = bundle.clone();
    copy_material.freeze_manifest_statement_sha256 = freeze_statement_sha256;
    copy_material.pre_ack_sealed_bundle_sha256 = bundle.clone();
    copy_material.receipt_set_sha256 = receipt;
    copy_material.run_identity_sha256 = run_identity_sha256;
    copy_material.seal_event_sha256 = seal_event;
    copy_material.source_sha256 = bundle;
    copy_material.supervisor_seal_statement_sha256 = supervisor_seal_statement_sha256;
    let copy_material_sha256 = copy_ack_material_sha256(copy_material)?;
    evidence.supervisor.commitment.copy_ack_material_sha256 = copy_material_sha256.clone();
    evidence
        .supervisor
        .commitment
        .independent_copy_ack
        .object_sha256 = copy_material_sha256;
    evidence
        .supervisor
        .commitment
        .independent_copy_ack
        .statement_sha256 =
        statement_shape_sha256(&evidence.supervisor.commitment.independent_copy_ack)?;
    let copy_ack_statement_sha256 = evidence
        .supervisor
        .commitment
        .independent_copy_ack
        .statement_sha256
        .clone();
    append_events(
        evidence,
        &[EventKindV1::CopyAcknowledgedFinal],
        &[copy_ack_statement_sha256],
        &mut predecessor,
    )?;
    evidence.supervisor.commitment.final_event_anchor_sha256 =
        evidence.events[9].event_sha256.clone();
    let terminal_payload = terminal_payload_sha256(evidence, &precommit, &core)?;
    evidence
        .supervisor
        .commitment
        .terminal_manifest
        .object_sha256 = terminal_payload;
    evidence
        .supervisor
        .commitment
        .terminal_manifest
        .statement_sha256 =
        statement_shape_sha256(&evidence.supervisor.commitment.terminal_manifest)?;
    Ok(())
}

#[cfg(test)]
fn append_events(
    evidence: &mut CandidateEvidenceV1,
    kinds: &[EventKindV1],
    payloads: &[String],
    predecessor: &mut Option<String>,
) -> Result<(), NixMnlError> {
    for (kind, payload_sha256) in kinds.iter().zip(payloads) {
        let mut event = EventV1 {
            boot_id_sha256: evidence.run.boot_id_sha256.clone(),
            event_index: evidence.events.len() as u32,
            event_sha256: String::new(),
            kind: *kind,
            payload_sha256: payload_sha256.clone(),
            predecessor_event_sha256: predecessor.clone(),
            run_nonce: evidence.run.run_nonce.clone(),
        };
        event.event_sha256 = event_sha256(&event)?;
        *predecessor = Some(event.event_sha256.clone());
        evidence.events.push(event);
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn test_trust_context_for(
    evidence: &mut CandidateEvidenceV1,
) -> Result<SupervisorTrustContextV1, NixMnlError> {
    evidence
        .contract
        .tooling
        .successor_final_tooling
        .as_ref()
        .ok_or_else(|| invalid("test production claim lacks final tooling"))?;
    let commitment = &evidence.supervisor.commitment;
    let freeze = &commitment.freeze_manifest;
    let seal = &commitment.supervisor_seal;
    let copy_ack = &commitment.independent_copy_ack;
    let terminal = &evidence.supervisor.commitment.terminal_manifest;
    let payload = crate::FrozenSupervisorProfilePayloadV1 {
        copy_ack_authorization: crate::StatementAuthorizationV1 {
            profile_id: copy_ack.profile_id.clone(),
            signer_key_id: copy_ack.signer_key_id.clone(),
        },
        copy_ack_destination_failure_domain: commitment.copy_ack_failure_domain.clone(),
        copy_ack_destination_identity_sha256: commitment
            .copy_ack_material
            .destination_identity_sha256
            .clone(),
        driver_binary_sha256: evidence.input.driver_binary_sha256.clone(),
        driver_source_sha256: evidence.input.driver_source_sha256.clone(),
        docker_config_sha256: evidence.host.docker_config_sha256.clone(),
        freeze_manifest_authorization: crate::StatementAuthorizationV1 {
            profile_id: freeze.profile_id.clone(),
            signer_key_id: freeze.signer_key_id.clone(),
        },
        freshness_challenge_nonce: commitment.preflight_freshness.challenge_nonce.clone(),
        freshness_generation_epoch_id: commitment.preflight_freshness.generation_epoch_id.clone(),
        freshness_max_generation_span: 1,
        freshness_one_shot_required: true,
        host_identity_sha256: evidence.host.host_identity_sha256.clone(),
        isolation_policy: evidence.isolation.clone(),
        production_contract: evidence.contract.clone(),
        profile_id: "test-only-pre-run-supervisor-profile-v1".to_string(),
        result_envelope_authorization: crate::StatementAuthorizationV1 {
            profile_id: "test-only-post-run-result-profile-v1".to_string(),
            signer_key_id: "test-only-post-run-result-key-v1".to_string(),
        },
        source_archive_sha256: evidence.input.source_archive_sha256.clone(),
        supervisor_seal_authorization: crate::StatementAuthorizationV1 {
            profile_id: seal.profile_id.clone(),
            signer_key_id: seal.signer_key_id.clone(),
        },
        terminal_manifest_authorization: crate::StatementAuthorizationV1 {
            profile_id: terminal.profile_id.clone(),
            signer_key_id: terminal.signer_key_id.clone(),
        },
        verifier_binary_sha256: evidence.input.verifier_binary_sha256.clone(),
        verifier_source_sha256: evidence.input.verifier_source_sha256.clone(),
    };
    let payload_sha256 = frozen_profile_payload_sha256(&payload)?;
    let trust_root_id = "test-only-external-trust-root-v1".to_string();
    let profile_authorization_signer_key_id = "test-only-profile-authorization-key-v1".to_string();
    let authorization = crate::SupervisorProfileAuthorizationV1 {
        algorithm: "ed25519-detached-sha256".to_string(),
        authorized_payload_sha256: payload_sha256.clone(),
        profile_id: payload.profile_id.clone(),
        signature_sha256: "e".repeat(64),
        signer_key_id: profile_authorization_signer_key_id.clone(),
        trust_root_id: trust_root_id.clone(),
    };
    let mut pre_run_authorization = crate::FrozenSupervisorProfileEnvelopeV1 {
        authorization,
        envelope_sha256: String::new(),
        payload,
        payload_sha256: payload_sha256.clone(),
    };
    let envelope_sha256 = frozen_profile_envelope_sha256(&pre_run_authorization)?;
    pre_run_authorization.envelope_sha256 = envelope_sha256.clone();

    evidence
        .supervisor
        .commitment
        .pre_run_authorization_payload_sha256 = payload_sha256.clone();
    evidence
        .supervisor
        .commitment
        .pre_run_authorization_envelope_sha256 = envelope_sha256.clone();
    populate_derived_fields(evidence)?;

    let commitment = &evidence.supervisor.commitment;
    let result_payload = crate::PostRunResultPayloadV1 {
        copy_ack_material: commitment.copy_ack_material.clone(),
        copy_ack_material_sha256: commitment.copy_ack_material_sha256.clone(),
        final_event_anchor_sha256: commitment.final_event_anchor_sha256.clone(),
        freeze_manifest: commitment.freeze_manifest.clone(),
        independent_copy_ack: commitment.independent_copy_ack.clone(),
        pre_ack_sealed_bundle_sha256: commitment.pre_ack_sealed_bundle_sha256.clone(),
        pre_run_authorization_envelope_sha256: envelope_sha256.clone(),
        pre_run_authorization_payload_sha256: payload_sha256.clone(),
        receipt_set_sha256: commitment.receipt_set_sha256.clone(),
        run_identity_sha256: evidence.run.run_identity_sha256.clone(),
        seal_event_sha256: commitment.seal_event_sha256.clone(),
        supervisor_seal: commitment.supervisor_seal.clone(),
        terminal_manifest: commitment.terminal_manifest.clone(),
    };
    let result_payload_sha256 = post_run_result_payload_sha256(&result_payload)?;
    let result_authorization = crate::SupervisorProfileAuthorizationV1 {
        algorithm: "ed25519-detached-sha256".to_string(),
        authorized_payload_sha256: result_payload_sha256.clone(),
        profile_id: pre_run_authorization
            .payload
            .result_envelope_authorization
            .profile_id
            .clone(),
        signature_sha256: "f".repeat(64),
        signer_key_id: pre_run_authorization
            .payload
            .result_envelope_authorization
            .signer_key_id
            .clone(),
        trust_root_id: trust_root_id.clone(),
    };
    let mut post_run_result = crate::PostRunResultEnvelopeV1 {
        authorization: result_authorization,
        envelope_sha256: String::new(),
        payload: result_payload,
        payload_sha256: result_payload_sha256,
    };
    post_run_result.envelope_sha256 = post_run_result_envelope_sha256(&post_run_result)?;
    Ok(SupervisorTrustContextV1 {
        freshness_lease: crate::SupervisorFreshnessLeaseV1 {
            authorized_pre_run_envelope_sha256: envelope_sha256.clone(),
            authorized_pre_run_payload_sha256: payload_sha256.clone(),
            challenge_nonce: commitment.preflight_freshness.challenge_nonce.clone(),
            consumed_in_this_verification: false,
            copy_session_nonce: commitment.copy_ack_material.copy_session_nonce.clone(),
            expires_generation: commitment.preflight_freshness.expires_generation,
            generation_epoch_id: commitment.preflight_freshness.generation_epoch_id.clone(),
            issued_generation: commitment.preflight_freshness.issued_generation,
            one_shot: commitment.preflight_freshness.one_shot,
            run_identity_sha256: evidence.run.run_identity_sha256.clone(),
            session_nonce: commitment.preflight_freshness.session_nonce.clone(),
            verification_generation: commitment.preflight_freshness.verification_generation,
        },
        post_run_result,
        pre_run_authorization,
        trust_root: crate::SupervisorTrustRootV1 {
            authorized_pre_run_envelope_sha256: envelope_sha256,
            authorized_pre_run_payload_sha256: payload_sha256,
            copy_ack_destination_failure_domain: commitment.copy_ack_failure_domain.clone(),
            copy_ack_destination_identity_sha256: commitment
                .copy_ack_material
                .destination_identity_sha256
                .clone(),
            copy_ack_signer_key_id: commitment.independent_copy_ack.signer_key_id.clone(),
            profile_authorization_signer_key_id,
            trust_root_id,
        },
    })
}

#[cfg(test)]
pub(crate) fn rebind_test_context_hashes(
    context: &mut SupervisorTrustContextV1,
) -> Result<(), NixMnlError> {
    let payload_sha256 = frozen_profile_payload_sha256(&context.pre_run_authorization.payload)?;
    context.pre_run_authorization.payload_sha256 = payload_sha256.clone();
    context
        .pre_run_authorization
        .authorization
        .authorized_payload_sha256 = payload_sha256.clone();
    let envelope_sha256 = frozen_profile_envelope_sha256(&context.pre_run_authorization)?;
    context.pre_run_authorization.envelope_sha256 = envelope_sha256.clone();
    context.trust_root.authorized_pre_run_payload_sha256 = payload_sha256.clone();
    context.trust_root.authorized_pre_run_envelope_sha256 = envelope_sha256.clone();
    context.freshness_lease.authorized_pre_run_payload_sha256 = payload_sha256.clone();
    context.freshness_lease.authorized_pre_run_envelope_sha256 = envelope_sha256.clone();
    context
        .post_run_result
        .payload
        .pre_run_authorization_payload_sha256 = payload_sha256;
    context
        .post_run_result
        .payload
        .pre_run_authorization_envelope_sha256 = envelope_sha256;
    let result_payload_sha256 = post_run_result_payload_sha256(&context.post_run_result.payload)?;
    context.post_run_result.payload_sha256 = result_payload_sha256.clone();
    context
        .post_run_result
        .authorization
        .authorized_payload_sha256 = result_payload_sha256;
    context.post_run_result.envelope_sha256 =
        post_run_result_envelope_sha256(&context.post_run_result)?;
    Ok(())
}

#[cfg(test)]
pub(crate) fn rebind_test_post_run_result_to_evidence(
    context: &mut SupervisorTrustContextV1,
    evidence: &CandidateEvidenceV1,
) -> Result<(), NixMnlError> {
    let commitment = &evidence.supervisor.commitment;
    context.post_run_result.payload = crate::PostRunResultPayloadV1 {
        copy_ack_material: commitment.copy_ack_material.clone(),
        copy_ack_material_sha256: commitment.copy_ack_material_sha256.clone(),
        final_event_anchor_sha256: commitment.final_event_anchor_sha256.clone(),
        freeze_manifest: commitment.freeze_manifest.clone(),
        independent_copy_ack: commitment.independent_copy_ack.clone(),
        pre_ack_sealed_bundle_sha256: commitment.pre_ack_sealed_bundle_sha256.clone(),
        pre_run_authorization_envelope_sha256: context
            .pre_run_authorization
            .envelope_sha256
            .clone(),
        pre_run_authorization_payload_sha256: context.pre_run_authorization.payload_sha256.clone(),
        receipt_set_sha256: commitment.receipt_set_sha256.clone(),
        run_identity_sha256: evidence.run.run_identity_sha256.clone(),
        seal_event_sha256: commitment.seal_event_sha256.clone(),
        supervisor_seal: commitment.supervisor_seal.clone(),
        terminal_manifest: commitment.terminal_manifest.clone(),
    };
    let result_payload_sha256 = post_run_result_payload_sha256(&context.post_run_result.payload)?;
    context.post_run_result.payload_sha256 = result_payload_sha256.clone();
    context
        .post_run_result
        .authorization
        .authorized_payload_sha256 = result_payload_sha256;
    context.post_run_result.envelope_sha256 =
        post_run_result_envelope_sha256(&context.post_run_result)?;
    Ok(())
}

#[cfg(test)]
pub(crate) fn rebind_test_copy_ack_downstream_preserving_byte_count(
    evidence: &mut CandidateEvidenceV1,
) -> Result<(), NixMnlError> {
    let copy_material_sha256 =
        copy_ack_material_sha256(&evidence.supervisor.commitment.copy_ack_material)?;
    evidence.supervisor.commitment.copy_ack_material_sha256 = copy_material_sha256.clone();
    evidence
        .supervisor
        .commitment
        .independent_copy_ack
        .object_sha256 = copy_material_sha256;
    evidence
        .supervisor
        .commitment
        .independent_copy_ack
        .statement_sha256 =
        statement_shape_sha256(&evidence.supervisor.commitment.independent_copy_ack)?;
    evidence.events[9].payload_sha256 = evidence
        .supervisor
        .commitment
        .independent_copy_ack
        .statement_sha256
        .clone();
    evidence.events[9].event_sha256 = event_sha256(&evidence.events[9])?;
    evidence.supervisor.commitment.final_event_anchor_sha256 =
        evidence.events[9].event_sha256.clone();
    let terminal_payload = terminal_payload_sha256(
        evidence,
        &evidence.supervisor.commitment.precommit_payload_sha256,
        &evidence.supervisor.commitment.full_evidence_core_sha256,
    )?;
    evidence
        .supervisor
        .commitment
        .terminal_manifest
        .object_sha256 = terminal_payload;
    evidence
        .supervisor
        .commitment
        .terminal_manifest
        .statement_sha256 =
        statement_shape_sha256(&evidence.supervisor.commitment.terminal_manifest)?;
    Ok(())
}

fn require_closed(value: &ClosedAuthorityV1, label: &str) -> Result<(), NixMnlError> {
    if !value.is_fully_closed() {
        return Err(invalid(format!("{label} authority is not fully closed")));
    }
    Ok(())
}

fn validate_nix_store_path<'a>(value: &'a str, label: &str) -> Result<&'a str, NixMnlError> {
    const STORE_PREFIX: &str = "/nix/store/";
    const NIX_BASE32: &[u8] = b"0123456789abcdfghijklmnpqrsvwxyz";
    let rest = value
        .strip_prefix(STORE_PREFIX)
        .ok_or_else(|| invalid(format!("{label} lacks exact /nix/store/ prefix")))?;
    if rest.contains('/') {
        return Err(invalid(format!("{label} contains an extra path component")));
    }
    let (store_hash, name) = rest
        .split_once('-')
        .ok_or_else(|| invalid(format!("{label} lacks store hash/name separator")))?;
    let hash_is_canonical =
        store_hash.len() == 32 && store_hash.bytes().all(|byte| NIX_BASE32.contains(&byte));
    let name_is_canonical = !name.is_empty()
        && name.len() <= 211
        && name != "."
        && name != ".."
        && name.bytes().next().is_some_and(|byte| byte != b'.')
        && name.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.' | b'_' | b'?' | b'=')
        });
    if !hash_is_canonical || !name_is_canonical {
        return Err(invalid(format!(
            "{label} is not /nix/store/<32-char-nix-base32>-<single-canonical-name>"
        )));
    }
    Ok(name)
}

fn require_digest(value: &str, label: &str) -> Result<(), NixMnlError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(format!(
            "{label} SHA-256 must be 64 lowercase hex characters"
        )));
    }
    Ok(())
}

fn require_git_oid(value: &str, label: &str) -> Result<(), NixMnlError> {
    if value.len() != 40
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(format!(
            "{label} must be 40 lowercase hex characters"
        )));
    }
    Ok(())
}

fn canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, NixMnlError> {
    Ok(serde_json::to_vec(value)?)
}

fn sha256_json<T: Serialize>(value: &T) -> Result<String, NixMnlError> {
    Ok(sha256(&canonical_json(value)?))
}

fn sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(64);
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in digest {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

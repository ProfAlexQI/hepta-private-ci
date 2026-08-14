//! Inert, deterministic post-hoc audit projection of a completed install-epoch
//! model proof.
//!
//! This completed-only projection is created after modeled provider I/O. It
//! retains the complete bounded current-tip attempt/closure history, but it
//! still cannot prove that an Intent or Outbox was durably published before
//! I/O. It is therefore a terminal audit artifact, not executable bridge
//! input. It exports exact claim, bundle, phase, attempt, closure, and
//! provider-receipt identities without granting provider I/O, publication,
//! trusted-state-root establishment, installation, daemon activation, or
//! qualification authority.

use sha2::Digest as _;

use crate::ExternalWatermarkPredecessorV1;
use crate::ExternalWatermarkRecordV1;
use crate::InstallEpochBindingV1;
use crate::MAX_EXTERNAL_WATERMARK_CURRENT_TIP_RETRY_COUNT_V1;
use crate::QualificationError;
use crate::VerifiedCommittedCurrentTipPreparationV1;
use crate::external_watermark_record_sha256_v1;
use crate::invalid;

#[cfg(test)]
#[path = "install_epoch_durable_projection_v1_tests.rs"]
mod tests;

/// Frozen identifier for the legacy terminal-only wire schema. New
/// projections are never emitted under this identity.
#[deprecated(
    note = "the v1 wire schema omitted bounded current-tip attempt history; use INSTALL_EPOCH_DURABLE_PROJECTION_SCHEMA_V2"
)]
pub const INSTALL_EPOCH_DURABLE_PROJECTION_SCHEMA_V1: &str =
    "hepta_linux_v8_install_epoch_durable_projection_v1";

/// Canonical wire schema for the complete bounded attempt/closure projection.
pub const INSTALL_EPOCH_DURABLE_PROJECTION_SCHEMA_V2: &str =
    "hepta_linux_v8_install_epoch_durable_projection_v2";
/// Canonical semantic profile bound into every v2 job and projection digest.
pub const INSTALL_EPOCH_DURABLE_PROJECTION_PROFILE_V2: &str =
    "hepta_linux_v8_install_epoch_durable_projection_profile_v2";

const INSTALL_EPOCH_DURABLE_PROJECTION_JOB_DOMAIN_V2: &[u8] =
    b"hepta_linux_v8_install_epoch_durable_projection_job_v2\0";
const INSTALL_EPOCH_DURABLE_PROJECTION_BINDING_DOMAIN_V2: &[u8] =
    b"hepta_linux_v8_install_epoch_durable_projection_binding_v2\0";
const EXPECTED_CLAIM_DOMAIN_V1: &str = "hepta-linux-v8-install-epoch-global-claim-domain-v1";
const EXPECTED_AUTHORITY_CLAIM_SCOPE_V1: &str = "hepta-linux-v8-install-epoch-authority-claim-v1";
const EXPECTED_LEASE_CLAIM_SCOPE_V1: &str = "hepta-linux-v8-external-watermark-lease-claim-v1";
const EXPECTED_COMMIT_CLAIM_SCOPE_V1: &str = "hepta-linux-v8-external-watermark-commit-claim-v1";
const EXPECTED_QUERY_CLAIM_SCOPE_V1: &str = "hepta-linux-v8-external-watermark-query-claim-v1";

/// Exact nonce-claim identity emitted by the qualification model. Fields stay
/// private and the value is neither cloneable nor deserializable.
#[derive(Debug)]
pub struct InstallEpochDurableNonceClaimProjectionV1 {
    pub(crate) binding_sha256: String,
    pub(crate) nonce: String,
    pub(crate) scope: String,
}

impl InstallEpochDurableNonceClaimProjectionV1 {
    pub fn binding_sha256(&self) -> &str {
        &self.binding_sha256
    }

    pub fn nonce(&self) -> &str {
        &self.nonce
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }
}

pub(crate) type RawDurableNonceClaimProjectionV1 = InstallEpochDurableNonceClaimProjectionV1;

/// Exact durable bundle identity emitted by the qualification model. It is an
/// inert binding only, not evidence that the bundle has been published.
#[derive(Debug)]
pub struct InstallEpochDurableBundleProjectionV1 {
    pub(crate) binding_sha256: String,
    pub(crate) id_sha256: String,
}

impl InstallEpochDurableBundleProjectionV1 {
    pub fn binding_sha256(&self) -> &str {
        &self.binding_sha256
    }

    pub fn id_sha256(&self) -> &str {
        &self.id_sha256
    }
}

pub(crate) type RawDurableBundleProjectionV1 = InstallEpochDurableBundleProjectionV1;

/// Complete identity of the typed terminal closure that consumed one
/// current-tip query before the next retry was admitted. The value contains
/// only audit bindings and cannot be constructed outside this crate.
#[derive(Debug)]
pub struct InstallEpochDurableQueryClosureProjectionV2 {
    pub(crate) closure_binding_sha256: String,
    pub(crate) closure_evidence_sha256: String,
    pub(crate) closure_profile_sha256: String,
    pub(crate) completion_operation_binding_sha256: String,
    pub(crate) phase_head_id_sha256: String,
    pub(crate) provider_transaction_sha256: String,
    pub(crate) query_claim_binding_sha256: String,
    pub(crate) query_nonce: String,
    pub(crate) query_phase_revision: u64,
    pub(crate) query_sequence: u64,
    pub(crate) query_state_sha256: String,
}

impl InstallEpochDurableQueryClosureProjectionV2 {
    pub fn closure_binding_sha256(&self) -> &str {
        &self.closure_binding_sha256
    }

    pub fn closure_evidence_sha256(&self) -> &str {
        &self.closure_evidence_sha256
    }

    pub fn closure_profile_sha256(&self) -> &str {
        &self.closure_profile_sha256
    }

    pub fn completion_operation_binding_sha256(&self) -> &str {
        &self.completion_operation_binding_sha256
    }

    pub fn phase_head_id_sha256(&self) -> &str {
        &self.phase_head_id_sha256
    }

    pub fn provider_transaction_sha256(&self) -> &str {
        &self.provider_transaction_sha256
    }

    pub fn query_claim_binding_sha256(&self) -> &str {
        &self.query_claim_binding_sha256
    }

    pub fn query_nonce(&self) -> &str {
        &self.query_nonce
    }

    pub fn query_phase_revision(&self) -> u64 {
        self.query_phase_revision
    }

    pub fn query_sequence(&self) -> u64 {
        self.query_sequence
    }

    pub fn query_state_sha256(&self) -> &str {
        &self.query_state_sha256
    }
}

pub(crate) type RawDurableQueryClosureProjectionV2 = InstallEpochDurableQueryClosureProjectionV2;

/// One ordered current-tip outbox attempt. Attempt 1 has no predecessor
/// closure; each later attempt owns the full verified closure for attempt
/// `sequence - 1`. Query claim and bundle identities are carried together
/// with the exact phase edge they represent.
#[derive(Debug)]
pub struct InstallEpochDurableCurrentTipAttemptProjectionV2 {
    pub(crate) phase_predecessor_revision: u64,
    pub(crate) phase_predecessor_state_sha256: String,
    pub(crate) phase_successor_revision: u64,
    pub(crate) phase_successor_state_sha256: String,
    pub(crate) predecessor_closure: Option<RawDurableQueryClosureProjectionV2>,
    pub(crate) query_bundle: RawDurableBundleProjectionV1,
    pub(crate) query_claim: RawDurableNonceClaimProjectionV1,
    pub(crate) query_sequence: u64,
}

impl InstallEpochDurableCurrentTipAttemptProjectionV2 {
    pub fn phase_predecessor_revision(&self) -> u64 {
        self.phase_predecessor_revision
    }

    pub fn phase_predecessor_state_sha256(&self) -> &str {
        &self.phase_predecessor_state_sha256
    }

    pub fn phase_successor_revision(&self) -> u64 {
        self.phase_successor_revision
    }

    pub fn phase_successor_state_sha256(&self) -> &str {
        &self.phase_successor_state_sha256
    }

    pub fn predecessor_closure(&self) -> Option<&InstallEpochDurableQueryClosureProjectionV2> {
        self.predecessor_closure.as_ref()
    }

    pub fn query_bundle(&self) -> &InstallEpochDurableBundleProjectionV1 {
        &self.query_bundle
    }

    pub fn query_claim(&self) -> &InstallEpochDurableNonceClaimProjectionV1 {
        &self.query_claim
    }

    pub fn query_sequence(&self) -> u64 {
        self.query_sequence
    }
}

pub(crate) type RawDurableCurrentTipAttemptProjectionV2 =
    InstallEpochDurableCurrentTipAttemptProjectionV2;

/// Exact monotonic phase identities that the model verified. Revisions and
/// states are exported together so a bridge cannot silently reconstruct phase
/// arithmetic or substitute a different transition.
#[derive(Debug)]
pub struct InstallEpochDurablePhaseProjectionV1 {
    active_query_revision: u64,
    active_query_sequence: u64,
    active_query_state_sha256: String,
    cas_intent_revision: u64,
    cas_intent_state_sha256: String,
    cas_issue_revision: u64,
    cas_issue_state_sha256: String,
    cas_receipt_revision: u64,
    cas_receipt_state_sha256: String,
    final_revision: u64,
    final_state_sha256: String,
    head_id_sha256: String,
}

impl InstallEpochDurablePhaseProjectionV1 {
    pub fn active_query_revision(&self) -> u64 {
        self.active_query_revision
    }

    pub fn active_query_sequence(&self) -> u64 {
        self.active_query_sequence
    }

    pub fn active_query_state_sha256(&self) -> &str {
        &self.active_query_state_sha256
    }

    pub fn cas_intent_revision(&self) -> u64 {
        self.cas_intent_revision
    }

    pub fn cas_intent_state_sha256(&self) -> &str {
        &self.cas_intent_state_sha256
    }

    pub fn cas_issue_revision(&self) -> u64 {
        self.cas_issue_revision
    }

    pub fn cas_issue_state_sha256(&self) -> &str {
        &self.cas_issue_state_sha256
    }

    pub fn cas_receipt_revision(&self) -> u64 {
        self.cas_receipt_revision
    }

    pub fn cas_receipt_state_sha256(&self) -> &str {
        &self.cas_receipt_state_sha256
    }

    pub fn final_revision(&self) -> u64 {
        self.final_revision
    }

    pub fn final_state_sha256(&self) -> &str {
        &self.final_state_sha256
    }

    pub fn head_id_sha256(&self) -> &str {
        &self.head_id_sha256
    }
}

#[derive(Debug)]
pub(crate) struct InstallEpochDurableProjectionSourceV2 {
    pub(crate) active_query_claim_binding_sha256: String,
    pub(crate) active_query_revision: u64,
    pub(crate) active_query_sequence: u64,
    pub(crate) active_query_state_sha256: String,
    pub(crate) authority_claim: RawDurableNonceClaimProjectionV1,
    pub(crate) cas_intent_revision: u64,
    pub(crate) cas_intent_state_sha256: String,
    pub(crate) cas_issue_revision: u64,
    pub(crate) cas_issue_state_sha256: String,
    pub(crate) cas_receipt_revision: u64,
    pub(crate) cas_receipt_state_sha256: String,
    pub(crate) claim_domain_id: String,
    pub(crate) commit_claim: RawDurableNonceClaimProjectionV1,
    pub(crate) commit_signature_sha256: String,
    pub(crate) commit_statement_sha256: String,
    pub(crate) commit_trust_policy_sha256: String,
    pub(crate) committed_at_unix_seconds: u64,
    pub(crate) completion_bundle: RawDurableBundleProjectionV1,
    pub(crate) completion_operation_binding_sha256: String,
    pub(crate) completion_profile_sha256: String,
    pub(crate) current_tip_expires_at_unix_seconds: u64,
    pub(crate) current_tip_issued_at_unix_seconds: u64,
    pub(crate) current_tip_query_nonce: String,
    pub(crate) current_tip_signature_sha256: String,
    pub(crate) current_tip_statement_sha256: String,
    pub(crate) current_tip_trust_policy_sha256: String,
    pub(crate) epoch: InstallEpochBindingV1,
    pub(crate) final_phase_revision: u64,
    pub(crate) finalized_state_sha256: String,
    pub(crate) current_tip_attempts: Box<[RawDurableCurrentTipAttemptProjectionV2]>,
    pub(crate) initial_query_claim_binding_sha256: String,
    pub(crate) initial_query_nonce: String,
    pub(crate) lease_claim: RawDurableNonceClaimProjectionV1,
    pub(crate) machine_id_sha256: String,
    pub(crate) phase_head_id_sha256: String,
    pub(crate) prepared_epoch_binding_sha256: String,
    pub(crate) preparation_binding_sha256: String,
    pub(crate) preparation_bundle: RawDurableBundleProjectionV1,
    pub(crate) predecessor: ExternalWatermarkPredecessorV1,
    pub(crate) provider_transaction_sha256: String,
    pub(crate) state_root_profile_sha256: String,
    pub(crate) successor_record: ExternalWatermarkRecordV1,
    pub(crate) successor_tip_sha256: String,
}

/// Opaque, non-cloneable, non-serializable terminal audit projection.
/// Construction consumes the completed qualification-model token, but the
/// result is deliberately insufficient for executable pre-I/O dispatch.
#[derive(Debug)]
pub struct InstallEpochDurableProjectionV2 {
    authority_claim: InstallEpochDurableNonceClaimProjectionV1,
    commit_claim: InstallEpochDurableNonceClaimProjectionV1,
    commit_signature_sha256: String,
    commit_statement_sha256: String,
    commit_trust_policy_sha256: String,
    committed_at_unix_seconds: u64,
    completion_bundle: InstallEpochDurableBundleProjectionV1,
    completion_operation_binding_sha256: String,
    completion_profile_sha256: String,
    current_tip_expires_at_unix_seconds: u64,
    current_tip_issued_at_unix_seconds: u64,
    current_tip_query_nonce: String,
    current_tip_signature_sha256: String,
    current_tip_statement_sha256: String,
    current_tip_trust_policy_sha256: String,
    current_tip_attempts: Box<[InstallEpochDurableCurrentTipAttemptProjectionV2]>,
    epoch: InstallEpochBindingV1,
    job_id_sha256: String,
    lease_claim: InstallEpochDurableNonceClaimProjectionV1,
    machine_id_sha256: String,
    phase: InstallEpochDurablePhaseProjectionV1,
    preparation_binding_sha256: String,
    preparation_bundle: InstallEpochDurableBundleProjectionV1,
    predecessor: ExternalWatermarkPredecessorV1,
    projection_sha256: String,
    provider_transaction_sha256: String,
    successor_record: ExternalWatermarkRecordV1,
    successor_tip_sha256: String,
}

impl InstallEpochDurableProjectionV2 {
    pub fn schema(&self) -> &'static str {
        INSTALL_EPOCH_DURABLE_PROJECTION_SCHEMA_V2
    }

    pub fn profile(&self) -> &'static str {
        INSTALL_EPOCH_DURABLE_PROJECTION_PROFILE_V2
    }

    pub fn active_query_bundle(&self) -> Option<&InstallEpochDurableBundleProjectionV1> {
        (self.current_tip_attempts.len() > 1).then(|| {
            &self
                .current_tip_attempts
                .last()
                .expect("validated attempt history")
                .query_bundle
        })
    }

    pub fn active_query_claim(&self) -> &InstallEpochDurableNonceClaimProjectionV1 {
        &self
            .current_tip_attempts
            .last()
            .expect("validated attempt history")
            .query_claim
    }

    pub fn authority_claim(&self) -> &InstallEpochDurableNonceClaimProjectionV1 {
        &self.authority_claim
    }

    pub fn claim_domain_id(&self) -> &str {
        EXPECTED_CLAIM_DOMAIN_V1
    }

    pub fn commit_claim(&self) -> &InstallEpochDurableNonceClaimProjectionV1 {
        &self.commit_claim
    }

    pub fn commit_signature_sha256(&self) -> &str {
        &self.commit_signature_sha256
    }

    pub fn commit_statement_sha256(&self) -> &str {
        &self.commit_statement_sha256
    }

    pub fn commit_trust_policy_sha256(&self) -> &str {
        &self.commit_trust_policy_sha256
    }

    pub fn committed_at_unix_seconds(&self) -> u64 {
        self.committed_at_unix_seconds
    }

    pub fn completion_bundle(&self) -> &InstallEpochDurableBundleProjectionV1 {
        &self.completion_bundle
    }

    pub fn completion_operation_binding_sha256(&self) -> &str {
        &self.completion_operation_binding_sha256
    }

    pub fn completion_profile_sha256(&self) -> &str {
        &self.completion_profile_sha256
    }

    pub fn current_tip_expires_at_unix_seconds(&self) -> u64 {
        self.current_tip_expires_at_unix_seconds
    }

    pub fn current_tip_issued_at_unix_seconds(&self) -> u64 {
        self.current_tip_issued_at_unix_seconds
    }

    pub fn current_tip_query_nonce(&self) -> &str {
        &self.current_tip_query_nonce
    }

    pub fn current_tip_signature_sha256(&self) -> &str {
        &self.current_tip_signature_sha256
    }

    pub fn current_tip_statement_sha256(&self) -> &str {
        &self.current_tip_statement_sha256
    }

    pub fn current_tip_trust_policy_sha256(&self) -> &str {
        &self.current_tip_trust_policy_sha256
    }

    pub fn current_tip_attempts(&self) -> &[InstallEpochDurableCurrentTipAttemptProjectionV2] {
        &self.current_tip_attempts
    }

    pub fn epoch(&self) -> &InstallEpochBindingV1 {
        &self.epoch
    }

    pub fn initial_query_claim(&self) -> &InstallEpochDurableNonceClaimProjectionV1 {
        &self
            .current_tip_attempts
            .first()
            .expect("validated attempt history")
            .query_claim
    }

    pub fn job_id_sha256(&self) -> &str {
        &self.job_id_sha256
    }

    pub fn lease_claim(&self) -> &InstallEpochDurableNonceClaimProjectionV1 {
        &self.lease_claim
    }

    pub fn machine_id_sha256(&self) -> &str {
        &self.machine_id_sha256
    }

    pub fn phase(&self) -> &InstallEpochDurablePhaseProjectionV1 {
        &self.phase
    }

    pub fn preparation_binding_sha256(&self) -> &str {
        &self.preparation_binding_sha256
    }

    pub fn preparation_bundle(&self) -> &InstallEpochDurableBundleProjectionV1 {
        &self.preparation_bundle
    }

    pub fn predecessor(&self) -> &ExternalWatermarkPredecessorV1 {
        &self.predecessor
    }

    pub fn projection_sha256(&self) -> &str {
        &self.projection_sha256
    }

    pub fn provider_transaction_sha256(&self) -> &str {
        &self.provider_transaction_sha256
    }

    pub fn retry_query_claim(&self) -> Option<&InstallEpochDurableNonceClaimProjectionV1> {
        (self.current_tip_attempts.len() > 1).then(|| {
            &self
                .current_tip_attempts
                .last()
                .expect("validated attempt history")
                .query_claim
        })
    }

    pub fn successor_record(&self) -> &ExternalWatermarkRecordV1 {
        &self.successor_record
    }

    pub fn successor_tip_sha256(&self) -> &str {
        &self.successor_tip_sha256
    }

    pub fn durable_publication_complete(&self) -> bool {
        false
    }

    pub fn provider_io_allowed(&self) -> bool {
        false
    }

    pub fn root_install_execution_allowed(&self) -> bool {
        false
    }

    pub fn daemon_reload_enable_or_start_allowed(&self) -> bool {
        false
    }

    pub fn trusted_state_root_established(&self) -> bool {
        false
    }

    pub fn fresh_attempt_allowed(&self) -> bool {
        false
    }
}

/// Consumes one completed model token and produces a deterministic inert,
/// post-hoc terminal audit projection. Model-local completion time and
/// fresh/recovery disposition are intentionally excluded: neither is trusted
/// durable identity. The result must not be used to authorize provider I/O.
pub fn project_install_epoch_completion_for_durable_bridge_v2(
    verified: VerifiedCommittedCurrentTipPreparationV1,
) -> Result<InstallEpochDurableProjectionV2, QualificationError> {
    let source = verified.durable_projection_source_v2();
    let projection = projection_from_source_v2(source)?;
    drop(verified);
    Ok(projection)
}

/// Source-compatibility entry point for callers that used the pre-v2 Rust
/// symbol. It emits only the canonical v2 wire profile.
#[deprecated(
    note = "the v1 wire profile omitted bounded attempt history; use project_install_epoch_completion_for_durable_bridge_v2"
)]
pub fn project_install_epoch_completion_for_durable_bridge_v1(
    verified: VerifiedCommittedCurrentTipPreparationV1,
) -> Result<InstallEpochDurableProjectionV2, QualificationError> {
    project_install_epoch_completion_for_durable_bridge_v2(verified)
}

/// Source-compatibility type name. Values produced now carry the v2 schema,
/// profile, and hash domains; no v1 wire value is constructed.
#[deprecated(
    note = "the v1 wire profile is frozen and no longer emitted; use InstallEpochDurableProjectionV2"
)]
pub type InstallEpochDurableProjectionV1 = InstallEpochDurableProjectionV2;

fn projection_from_source_v2(
    source: InstallEpochDurableProjectionSourceV2,
) -> Result<InstallEpochDurableProjectionV2, QualificationError> {
    validate_source_v2(&source)?;
    let job_id_sha256 = projection_job_id_sha256_v2(&source);
    let projection_sha256 = projection_binding_sha256_v2(&source, &job_id_sha256);
    Ok(InstallEpochDurableProjectionV2 {
        authority_claim: source.authority_claim,
        commit_claim: source.commit_claim,
        commit_signature_sha256: source.commit_signature_sha256,
        commit_statement_sha256: source.commit_statement_sha256,
        commit_trust_policy_sha256: source.commit_trust_policy_sha256,
        committed_at_unix_seconds: source.committed_at_unix_seconds,
        completion_bundle: source.completion_bundle,
        completion_operation_binding_sha256: source.completion_operation_binding_sha256,
        completion_profile_sha256: source.completion_profile_sha256,
        current_tip_expires_at_unix_seconds: source.current_tip_expires_at_unix_seconds,
        current_tip_issued_at_unix_seconds: source.current_tip_issued_at_unix_seconds,
        current_tip_query_nonce: source.current_tip_query_nonce,
        current_tip_signature_sha256: source.current_tip_signature_sha256,
        current_tip_statement_sha256: source.current_tip_statement_sha256,
        current_tip_trust_policy_sha256: source.current_tip_trust_policy_sha256,
        current_tip_attempts: source.current_tip_attempts,
        epoch: source.epoch,
        job_id_sha256,
        lease_claim: source.lease_claim,
        machine_id_sha256: source.machine_id_sha256,
        phase: InstallEpochDurablePhaseProjectionV1 {
            active_query_revision: source.active_query_revision,
            active_query_sequence: source.active_query_sequence,
            active_query_state_sha256: source.active_query_state_sha256,
            cas_intent_revision: source.cas_intent_revision,
            cas_intent_state_sha256: source.cas_intent_state_sha256,
            cas_issue_revision: source.cas_issue_revision,
            cas_issue_state_sha256: source.cas_issue_state_sha256,
            cas_receipt_revision: source.cas_receipt_revision,
            cas_receipt_state_sha256: source.cas_receipt_state_sha256,
            final_revision: source.final_phase_revision,
            final_state_sha256: source.finalized_state_sha256,
            head_id_sha256: source.phase_head_id_sha256,
        },
        preparation_binding_sha256: source.preparation_binding_sha256,
        preparation_bundle: source.preparation_bundle,
        predecessor: source.predecessor,
        projection_sha256,
        provider_transaction_sha256: source.provider_transaction_sha256,
        successor_record: source.successor_record,
        successor_tip_sha256: source.successor_tip_sha256,
    })
}

fn validate_source_v2(
    source: &InstallEpochDurableProjectionSourceV2,
) -> Result<(), QualificationError> {
    if source.claim_domain_id != EXPECTED_CLAIM_DOMAIN_V1 {
        return Err(invalid("durable projection claim domain is not exact"));
    }
    validate_claim_v1(
        "authority",
        &source.authority_claim,
        EXPECTED_AUTHORITY_CLAIM_SCOPE_V1,
    )?;
    validate_claim_v1("lease", &source.lease_claim, EXPECTED_LEASE_CLAIM_SCOPE_V1)?;
    validate_claim_v1(
        "commit",
        &source.commit_claim,
        EXPECTED_COMMIT_CLAIM_SCOPE_V1,
    )?;
    validate_current_tip_attempt_history_v2(source)?;
    validate_bundle_v1("preparation", &source.preparation_bundle)?;
    validate_bundle_v1("completion", &source.completion_bundle)?;
    for (label, digest) in [
        (
            "preparation binding",
            source.preparation_binding_sha256.as_str(),
        ),
        (
            "prepared epoch binding",
            source.prepared_epoch_binding_sha256.as_str(),
        ),
        (
            "completion operation binding",
            source.completion_operation_binding_sha256.as_str(),
        ),
        (
            "completion profile",
            source.completion_profile_sha256.as_str(),
        ),
        ("machine identity", source.machine_id_sha256.as_str()),
        (
            "state-root profile",
            source.state_root_profile_sha256.as_str(),
        ),
        ("phase head", source.phase_head_id_sha256.as_str()),
        ("CAS intent state", source.cas_intent_state_sha256.as_str()),
        ("CAS issue state", source.cas_issue_state_sha256.as_str()),
        (
            "CAS receipt state",
            source.cas_receipt_state_sha256.as_str(),
        ),
        (
            "active query state",
            source.active_query_state_sha256.as_str(),
        ),
        ("final state", source.finalized_state_sha256.as_str()),
        (
            "provider transaction",
            source.provider_transaction_sha256.as_str(),
        ),
        ("commit statement", source.commit_statement_sha256.as_str()),
        ("commit signature", source.commit_signature_sha256.as_str()),
        (
            "commit trust policy",
            source.commit_trust_policy_sha256.as_str(),
        ),
        (
            "current-tip statement",
            source.current_tip_statement_sha256.as_str(),
        ),
        (
            "current-tip signature",
            source.current_tip_signature_sha256.as_str(),
        ),
        (
            "current-tip trust policy",
            source.current_tip_trust_policy_sha256.as_str(),
        ),
        ("successor tip", source.successor_tip_sha256.as_str()),
    ] {
        validate_digest_v1(label, digest)?;
    }
    let expected_active_query_revision = source
        .active_query_sequence
        .checked_add(3)
        .ok_or_else(|| invalid("durable projection active-query revision overflows"))?;
    let expected_final_revision = source
        .active_query_revision
        .checked_add(1)
        .ok_or_else(|| invalid("durable projection final revision overflows"))?;
    if source.cas_intent_revision != 1
        || source.cas_issue_revision != 2
        || source.cas_receipt_revision != 3
        || source.active_query_sequence == 0
        || source.active_query_revision != expected_active_query_revision
        || source.final_phase_revision != expected_final_revision
    {
        return Err(invalid(
            "durable projection phase revision chain is not exact",
        ));
    }
    if source.current_tip_issued_at_unix_seconds >= source.current_tip_expires_at_unix_seconds
        || source.committed_at_unix_seconds > source.current_tip_issued_at_unix_seconds
    {
        return Err(invalid(
            "durable projection provider receipt chronology is invalid",
        ));
    }
    let expected_successor_revision = source
        .predecessor
        .revision()
        .checked_add(1)
        .ok_or_else(|| invalid("durable projection successor revision overflows"))?;
    if source.epoch.epoch_sequence == 0
        || source.successor_record.completion_profile_sha256 != source.completion_profile_sha256
        || source.successor_record.prepared_epoch_binding_sha256
            != source.prepared_epoch_binding_sha256
        || source.successor_record.machine_id_sha256 != source.machine_id_sha256
        || source.successor_record.preparation_binding_sha256 != source.preparation_binding_sha256
        || source.successor_record.predecessor != source.predecessor
        || source.successor_record.provider_profile_sha256
            != source.predecessor.provider_profile_sha256()
        || source.successor_record.state_root_profile_sha256 != source.state_root_profile_sha256
        || source.successor_record.stream_id_sha256 != source.predecessor.stream_id_sha256()
        || source.successor_record.successor_revision != expected_successor_revision
        || source.successor_tip_sha256
            != external_watermark_record_sha256_v1(&source.successor_record)
    {
        return Err(invalid(
            "durable projection successor record is not exactly cross-bound",
        ));
    }
    if source.completion_bundle.binding_sha256 != source.cas_intent_state_sha256 {
        return Err(invalid(
            "durable projection completion bundle is not the CAS intent state",
        ));
    }
    Ok(())
}

fn validate_current_tip_attempt_history_v2(
    source: &InstallEpochDurableProjectionSourceV2,
) -> Result<(), QualificationError> {
    let attempt_count = u64::try_from(source.current_tip_attempts.len())
        .map_err(|_| invalid("durable projection attempt count overflows"))?;
    let maximum_attempt_count = MAX_EXTERNAL_WATERMARK_CURRENT_TIP_RETRY_COUNT_V1
        .checked_add(1)
        .ok_or_else(|| invalid("durable projection attempt bound overflows"))?;
    if attempt_count == 0
        || attempt_count > maximum_attempt_count
        || attempt_count != source.active_query_sequence
    {
        return Err(invalid(
            "durable projection current-tip attempt count is not exact",
        ));
    }

    let mut predecessor_revision = source.cas_receipt_revision;
    let mut predecessor_state = source.cas_receipt_state_sha256.as_str();
    let mut predecessor_attempt: Option<&InstallEpochDurableCurrentTipAttemptProjectionV2> = None;
    let mut nonces = vec![
        source.authority_claim.nonce.as_str(),
        source.lease_claim.nonce.as_str(),
        source.commit_claim.nonce.as_str(),
    ];

    for (index, attempt) in source.current_tip_attempts.iter().enumerate() {
        let query_sequence = u64::try_from(index)
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| invalid("durable projection attempt sequence overflows"))?;
        validate_claim_v1(
            "current-tip attempt",
            &attempt.query_claim,
            EXPECTED_QUERY_CLAIM_SCOPE_V1,
        )?;
        validate_bundle_v1("current-tip attempt", &attempt.query_bundle)?;
        validate_digest_v1(
            "current-tip attempt predecessor state",
            &attempt.phase_predecessor_state_sha256,
        )?;
        validate_digest_v1(
            "current-tip attempt successor state",
            &attempt.phase_successor_state_sha256,
        )?;

        let expected_successor_revision = predecessor_revision
            .checked_add(1)
            .ok_or_else(|| invalid("durable projection attempt revision overflows"))?;
        if attempt.query_sequence != query_sequence
            || attempt.phase_predecessor_revision != predecessor_revision
            || attempt.phase_predecessor_state_sha256 != predecessor_state
            || attempt.phase_successor_revision != expected_successor_revision
            || attempt.query_bundle.id_sha256 != attempt.query_claim.binding_sha256
            || attempt.query_bundle.binding_sha256 != attempt.phase_successor_state_sha256
        {
            return Err(invalid(
                "durable projection current-tip attempt phase edge is not exact",
            ));
        }

        if nonces.contains(&attempt.query_claim.nonce.as_str()) {
            return Err(invalid("durable projection nonce identities collide"));
        }
        nonces.push(attempt.query_claim.nonce.as_str());

        let predecessor_closure_binding = match (&predecessor_attempt, &attempt.predecessor_closure)
        {
            (None, None) => None,
            (Some(predecessor), Some(closure)) => {
                validate_query_closure_v2(source, predecessor, closure)?;
                Some(closure.closure_binding_sha256.as_str())
            }
            _ => {
                return Err(invalid(
                    "durable projection current-tip closure disposition is not exact",
                ));
            }
        };

        let expected_claim_binding = match (predecessor_attempt, predecessor_closure_binding) {
            (None, None) => completion_nonce_claim_binding_sha256_v1(
                EXPECTED_QUERY_CLAIM_SCOPE_V1,
                &attempt.query_claim.nonce,
                &source.commit_claim.nonce,
                &source.completion_operation_binding_sha256,
            ),
            (Some(predecessor), Some(closure_binding)) => {
                completion_retry_nonce_claim_binding_sha256_v1(
                    &attempt.query_claim.nonce,
                    &predecessor.query_claim.nonce,
                    &source.commit_claim.nonce,
                    &source.completion_operation_binding_sha256,
                    closure_binding,
                )
            }
            _ => unreachable!("closure disposition was checked above"),
        };
        let expected_query_state = current_tip_query_state_sha256_v1(
            predecessor_state,
            attempt.phase_successor_revision,
            attempt.query_sequence,
            &attempt.query_claim.binding_sha256,
            predecessor_closure_binding,
        );
        if attempt.query_claim.binding_sha256 != expected_claim_binding
            || attempt.phase_successor_state_sha256 != expected_query_state
        {
            return Err(invalid(
                "durable projection current-tip claim/state binding is not exact",
            ));
        }

        predecessor_revision = attempt.phase_successor_revision;
        predecessor_state = &attempt.phase_successor_state_sha256;
        predecessor_attempt = Some(attempt);
    }

    let initial = source
        .current_tip_attempts
        .first()
        .ok_or_else(|| invalid("durable projection initial attempt is absent"))?;
    let active = source
        .current_tip_attempts
        .last()
        .ok_or_else(|| invalid("durable projection active attempt is absent"))?;
    if initial.query_claim.nonce != source.initial_query_nonce
        || initial.query_claim.binding_sha256 != source.initial_query_claim_binding_sha256
        || active.query_claim.nonce != source.current_tip_query_nonce
        || active.query_claim.binding_sha256 != source.active_query_claim_binding_sha256
        || active.query_sequence != source.active_query_sequence
        || active.phase_successor_revision != source.active_query_revision
        || active.phase_successor_state_sha256 != source.active_query_state_sha256
    {
        return Err(invalid(
            "durable projection attempt endpoints do not match initial/active query identity",
        ));
    }
    Ok(())
}

fn validate_query_closure_v2(
    source: &InstallEpochDurableProjectionSourceV2,
    predecessor: &InstallEpochDurableCurrentTipAttemptProjectionV2,
    closure: &InstallEpochDurableQueryClosureProjectionV2,
) -> Result<(), QualificationError> {
    for (label, digest) in [
        (
            "query-closure binding",
            closure.closure_binding_sha256.as_str(),
        ),
        (
            "query-closure evidence",
            closure.closure_evidence_sha256.as_str(),
        ),
        (
            "query-closure profile",
            closure.closure_profile_sha256.as_str(),
        ),
        (
            "query-closure operation",
            closure.completion_operation_binding_sha256.as_str(),
        ),
        (
            "query-closure phase head",
            closure.phase_head_id_sha256.as_str(),
        ),
        (
            "query-closure provider transaction",
            closure.provider_transaction_sha256.as_str(),
        ),
        (
            "query-closure claim binding",
            closure.query_claim_binding_sha256.as_str(),
        ),
        ("query-closure nonce", closure.query_nonce.as_str()),
        ("query-closure state", closure.query_state_sha256.as_str()),
    ] {
        validate_digest_v1(label, digest)?;
    }
    if closure.completion_operation_binding_sha256 != source.completion_operation_binding_sha256
        || closure.phase_head_id_sha256 != source.phase_head_id_sha256
        || closure.provider_transaction_sha256 != source.provider_transaction_sha256
        || closure.query_claim_binding_sha256 != predecessor.query_claim.binding_sha256
        || closure.query_nonce != predecessor.query_claim.nonce
        || closure.query_phase_revision != predecessor.phase_successor_revision
        || closure.query_sequence != predecessor.query_sequence
        || closure.query_state_sha256 != predecessor.phase_successor_state_sha256
        || closure.closure_binding_sha256 != query_closure_binding_sha256_v1(closure)
    {
        return Err(invalid(
            "durable projection query closure is not exactly cross-bound",
        ));
    }
    Ok(())
}

fn validate_claim_v1(
    label: &str,
    claim: &InstallEpochDurableNonceClaimProjectionV1,
    expected_scope: &str,
) -> Result<(), QualificationError> {
    if claim.scope != expected_scope {
        return Err(invalid(format!(
            "durable projection {label} claim scope is not exact"
        )));
    }
    validate_digest_v1(&format!("{label} claim nonce"), &claim.nonce)?;
    validate_digest_v1(&format!("{label} claim binding"), &claim.binding_sha256)
}

fn validate_bundle_v1(
    label: &str,
    bundle: &InstallEpochDurableBundleProjectionV1,
) -> Result<(), QualificationError> {
    validate_digest_v1(&format!("{label} bundle id"), &bundle.id_sha256)?;
    validate_digest_v1(&format!("{label} bundle binding"), &bundle.binding_sha256)
}

fn validate_digest_v1(label: &str, value: &str) -> Result<(), QualificationError> {
    if value.len() != 64
        || value == "0".repeat(64)
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(format!(
            "durable projection {label} must be a non-zero lowercase SHA-256"
        )));
    }
    Ok(())
}

fn completion_nonce_claim_binding_sha256_v1(
    scope: &str,
    nonce: &str,
    counterpart_nonce: &str,
    completion_operation_binding_sha256: &str,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_nonce_claim_v1\0".to_vec();
    append_field_v1(
        &mut bytes,
        "claim_domain_id",
        EXPECTED_CLAIM_DOMAIN_V1.as_bytes(),
    );
    append_field_v1(&mut bytes, "scope", scope.as_bytes());
    append_field_v1(&mut bytes, "nonce", nonce.as_bytes());
    append_field_v1(
        &mut bytes,
        "counterpart_nonce",
        counterpart_nonce.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "completion_operation_binding_sha256",
        completion_operation_binding_sha256.as_bytes(),
    );
    sha256_v1(&bytes)
}

fn completion_retry_nonce_claim_binding_sha256_v1(
    nonce: &str,
    predecessor_query_nonce: &str,
    commit_nonce: &str,
    completion_operation_binding_sha256: &str,
    predecessor_query_closure_sha256: &str,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_retry_claim_v1\0".to_vec();
    append_field_v1(
        &mut bytes,
        "claim_domain_id",
        EXPECTED_CLAIM_DOMAIN_V1.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "scope",
        EXPECTED_QUERY_CLAIM_SCOPE_V1.as_bytes(),
    );
    append_field_v1(&mut bytes, "nonce", nonce.as_bytes());
    append_field_v1(
        &mut bytes,
        "predecessor_query_nonce",
        predecessor_query_nonce.as_bytes(),
    );
    append_field_v1(&mut bytes, "commit_nonce", commit_nonce.as_bytes());
    append_field_v1(
        &mut bytes,
        "completion_operation_binding_sha256",
        completion_operation_binding_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "predecessor_query_closure_sha256",
        predecessor_query_closure_sha256.as_bytes(),
    );
    sha256_v1(&bytes)
}

fn query_closure_binding_sha256_v1(
    closure: &InstallEpochDurableQueryClosureProjectionV2,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_query_closure_v1\0".to_vec();
    append_field_v1(
        &mut bytes,
        "closure_profile_sha256",
        closure.closure_profile_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "closure_evidence_sha256",
        closure.closure_evidence_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "completion_operation_binding_sha256",
        closure.completion_operation_binding_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "phase_head_id_sha256",
        closure.phase_head_id_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "provider_transaction_sha256",
        closure.provider_transaction_sha256.as_bytes(),
    );
    append_field_v1(&mut bytes, "query_nonce", closure.query_nonce.as_bytes());
    append_field_v1(
        &mut bytes,
        "query_claim_binding_sha256",
        closure.query_claim_binding_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "query_state_sha256",
        closure.query_state_sha256.as_bytes(),
    );
    append_u64_v1(&mut bytes, "query_sequence", closure.query_sequence);
    append_u64_v1(
        &mut bytes,
        "query_phase_revision",
        closure.query_phase_revision,
    );
    sha256_v1(&bytes)
}

fn current_tip_query_state_sha256_v1(
    prior_state_sha256: &str,
    phase_revision: u64,
    query_sequence: u64,
    query_claim_binding_sha256: &str,
    predecessor_query_closure_sha256: Option<&str>,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_query_state_v1\0".to_vec();
    append_u64_v1(&mut bytes, "phase_revision", phase_revision);
    append_u64_v1(&mut bytes, "query_sequence", query_sequence);
    append_field_v1(
        &mut bytes,
        "prior_state_sha256",
        prior_state_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "query_claim_binding_sha256",
        query_claim_binding_sha256.as_bytes(),
    );
    match predecessor_query_closure_sha256 {
        Some(binding) => append_field_v1(
            &mut bytes,
            "predecessor_query_closure_sha256",
            binding.as_bytes(),
        ),
        None => append_field_v1(
            &mut bytes,
            "predecessor_query_closure",
            b"initial_query_no_predecessor",
        ),
    }
    sha256_v1(&bytes)
}

fn projection_job_id_sha256_v2(source: &InstallEpochDurableProjectionSourceV2) -> String {
    let mut bytes = INSTALL_EPOCH_DURABLE_PROJECTION_JOB_DOMAIN_V2.to_vec();
    append_field_v1(
        &mut bytes,
        "schema",
        INSTALL_EPOCH_DURABLE_PROJECTION_SCHEMA_V2.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "profile",
        INSTALL_EPOCH_DURABLE_PROJECTION_PROFILE_V2.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "claim_domain_id",
        source.claim_domain_id.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "preparation_bundle_id_sha256",
        source.preparation_bundle.id_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "completion_bundle_id_sha256",
        source.completion_bundle.id_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "completion_operation_binding_sha256",
        source.completion_operation_binding_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "phase_head_id_sha256",
        source.phase_head_id_sha256.as_bytes(),
    );
    sha256_v1(&bytes)
}

fn projection_binding_sha256_v2(
    source: &InstallEpochDurableProjectionSourceV2,
    job_id_sha256: &str,
) -> String {
    let mut bytes = INSTALL_EPOCH_DURABLE_PROJECTION_BINDING_DOMAIN_V2.to_vec();
    append_field_v1(
        &mut bytes,
        "schema",
        INSTALL_EPOCH_DURABLE_PROJECTION_SCHEMA_V2.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "profile",
        INSTALL_EPOCH_DURABLE_PROJECTION_PROFILE_V2.as_bytes(),
    );
    append_field_v1(&mut bytes, "job_id_sha256", job_id_sha256.as_bytes());
    append_field_v1(
        &mut bytes,
        "claim_domain_id",
        source.claim_domain_id.as_bytes(),
    );
    append_claim_v1(&mut bytes, "authority", &source.authority_claim);
    append_claim_v1(&mut bytes, "lease", &source.lease_claim);
    append_claim_v1(&mut bytes, "commit", &source.commit_claim);
    append_u64_v1(
        &mut bytes,
        "current_tip_attempt_count",
        source.current_tip_attempts.len() as u64,
    );
    for (index, attempt) in source.current_tip_attempts.iter().enumerate() {
        append_current_tip_attempt_v2(&mut bytes, index, attempt);
    }
    append_bundle_v1(&mut bytes, "preparation", &source.preparation_bundle);
    append_bundle_v1(&mut bytes, "completion", &source.completion_bundle);
    append_field_v1(
        &mut bytes,
        "preparation_binding_sha256",
        source.preparation_binding_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "completion_operation_binding_sha256",
        source.completion_operation_binding_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "completion_profile_sha256",
        source.completion_profile_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "machine_id_sha256",
        source.machine_id_sha256.as_bytes(),
    );
    append_u64_v1(&mut bytes, "epoch_sequence", source.epoch.epoch_sequence);
    append_field_v1(
        &mut bytes,
        "epoch_nonce_sha256",
        source.epoch.epoch_nonce_sha256.as_bytes(),
    );
    append_predecessor_v1(&mut bytes, &source.predecessor);
    append_field_v1(
        &mut bytes,
        "phase_head_id_sha256",
        source.phase_head_id_sha256.as_bytes(),
    );
    for (label, revision, state) in [
        (
            "cas_intent",
            source.cas_intent_revision,
            source.cas_intent_state_sha256.as_str(),
        ),
        (
            "cas_issue",
            source.cas_issue_revision,
            source.cas_issue_state_sha256.as_str(),
        ),
        (
            "cas_receipt",
            source.cas_receipt_revision,
            source.cas_receipt_state_sha256.as_str(),
        ),
        (
            "active_query",
            source.active_query_revision,
            source.active_query_state_sha256.as_str(),
        ),
        (
            "final",
            source.final_phase_revision,
            source.finalized_state_sha256.as_str(),
        ),
    ] {
        append_u64_v1(&mut bytes, &format!("{label}_revision"), revision);
        append_field_v1(
            &mut bytes,
            &format!("{label}_state_sha256"),
            state.as_bytes(),
        );
    }
    append_u64_v1(
        &mut bytes,
        "active_query_sequence",
        source.active_query_sequence,
    );
    append_u64_v1(
        &mut bytes,
        "committed_at_unix_seconds",
        source.committed_at_unix_seconds,
    );
    append_field_v1(
        &mut bytes,
        "provider_transaction_sha256",
        source.provider_transaction_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "commit_statement_sha256",
        source.commit_statement_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "commit_signature_sha256",
        source.commit_signature_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "commit_trust_policy_sha256",
        source.commit_trust_policy_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "current_tip_query_nonce",
        source.current_tip_query_nonce.as_bytes(),
    );
    append_u64_v1(
        &mut bytes,
        "current_tip_issued_at_unix_seconds",
        source.current_tip_issued_at_unix_seconds,
    );
    append_u64_v1(
        &mut bytes,
        "current_tip_expires_at_unix_seconds",
        source.current_tip_expires_at_unix_seconds,
    );
    append_field_v1(
        &mut bytes,
        "current_tip_statement_sha256",
        source.current_tip_statement_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "current_tip_signature_sha256",
        source.current_tip_signature_sha256.as_bytes(),
    );
    append_field_v1(
        &mut bytes,
        "current_tip_trust_policy_sha256",
        source.current_tip_trust_policy_sha256.as_bytes(),
    );
    append_record_v1(&mut bytes, &source.successor_record);
    append_field_v1(
        &mut bytes,
        "successor_tip_sha256",
        source.successor_tip_sha256.as_bytes(),
    );
    sha256_v1(&bytes)
}

fn append_current_tip_attempt_v2(
    bytes: &mut Vec<u8>,
    index: usize,
    attempt: &InstallEpochDurableCurrentTipAttemptProjectionV2,
) {
    let label = format!("current_tip_attempt_{index}");
    append_u64_v1(bytes, &format!("{label}_index"), index as u64);
    append_u64_v1(
        bytes,
        &format!("{label}_query_sequence"),
        attempt.query_sequence,
    );
    append_claim_v1(bytes, &format!("{label}_query"), &attempt.query_claim);
    append_bundle_v1(bytes, &format!("{label}_query"), &attempt.query_bundle);
    append_u64_v1(
        bytes,
        &format!("{label}_phase_predecessor_revision"),
        attempt.phase_predecessor_revision,
    );
    append_field_v1(
        bytes,
        &format!("{label}_phase_predecessor_state_sha256"),
        attempt.phase_predecessor_state_sha256.as_bytes(),
    );
    append_u64_v1(
        bytes,
        &format!("{label}_phase_successor_revision"),
        attempt.phase_successor_revision,
    );
    append_field_v1(
        bytes,
        &format!("{label}_phase_successor_state_sha256"),
        attempt.phase_successor_state_sha256.as_bytes(),
    );
    match &attempt.predecessor_closure {
        Some(closure) => append_query_closure_v2(bytes, &label, closure),
        None => append_field_v1(
            bytes,
            &format!("{label}_predecessor_closure"),
            b"initial_query_no_predecessor",
        ),
    }
}

fn append_query_closure_v2(
    bytes: &mut Vec<u8>,
    attempt_label: &str,
    closure: &InstallEpochDurableQueryClosureProjectionV2,
) {
    let label = format!("{attempt_label}_predecessor_closure");
    for (field, value) in [
        ("binding_sha256", closure.closure_binding_sha256.as_str()),
        ("evidence_sha256", closure.closure_evidence_sha256.as_str()),
        ("profile_sha256", closure.closure_profile_sha256.as_str()),
        (
            "completion_operation_binding_sha256",
            closure.completion_operation_binding_sha256.as_str(),
        ),
        (
            "phase_head_id_sha256",
            closure.phase_head_id_sha256.as_str(),
        ),
        (
            "provider_transaction_sha256",
            closure.provider_transaction_sha256.as_str(),
        ),
        (
            "query_claim_binding_sha256",
            closure.query_claim_binding_sha256.as_str(),
        ),
        ("query_nonce", closure.query_nonce.as_str()),
        ("query_state_sha256", closure.query_state_sha256.as_str()),
    ] {
        append_field_v1(bytes, &format!("{label}_{field}"), value.as_bytes());
    }
    append_u64_v1(
        bytes,
        &format!("{label}_query_phase_revision"),
        closure.query_phase_revision,
    );
    append_u64_v1(
        bytes,
        &format!("{label}_query_sequence"),
        closure.query_sequence,
    );
}

fn append_claim_v1(
    bytes: &mut Vec<u8>,
    label: &str,
    claim: &InstallEpochDurableNonceClaimProjectionV1,
) {
    append_field_v1(
        bytes,
        &format!("{label}_claim_scope"),
        claim.scope.as_bytes(),
    );
    append_field_v1(bytes, &format!("{label}_nonce"), claim.nonce.as_bytes());
    append_field_v1(
        bytes,
        &format!("{label}_claim_binding_sha256"),
        claim.binding_sha256.as_bytes(),
    );
}

fn append_bundle_v1(
    bytes: &mut Vec<u8>,
    label: &str,
    bundle: &InstallEpochDurableBundleProjectionV1,
) {
    append_field_v1(
        bytes,
        &format!("{label}_bundle_id_sha256"),
        bundle.id_sha256.as_bytes(),
    );
    append_field_v1(
        bytes,
        &format!("{label}_bundle_binding_sha256"),
        bundle.binding_sha256.as_bytes(),
    );
}

fn append_record_v1(bytes: &mut Vec<u8>, record: &ExternalWatermarkRecordV1) {
    append_field_v1(
        bytes,
        "successor_completion_profile_sha256",
        record.completion_profile_sha256.as_bytes(),
    );
    append_field_v1(
        bytes,
        "successor_prepared_epoch_binding_sha256",
        record.prepared_epoch_binding_sha256.as_bytes(),
    );
    append_field_v1(
        bytes,
        "successor_machine_id_sha256",
        record.machine_id_sha256.as_bytes(),
    );
    append_predecessor_v1(bytes, &record.predecessor);
    append_field_v1(
        bytes,
        "successor_preparation_binding_sha256",
        record.preparation_binding_sha256.as_bytes(),
    );
    append_field_v1(
        bytes,
        "successor_provider_profile_sha256",
        record.provider_profile_sha256.as_bytes(),
    );
    append_field_v1(
        bytes,
        "successor_state_root_profile_sha256",
        record.state_root_profile_sha256.as_bytes(),
    );
    append_field_v1(
        bytes,
        "successor_stream_id_sha256",
        record.stream_id_sha256.as_bytes(),
    );
    append_u64_v1(bytes, "successor_revision", record.successor_revision);
}

fn append_predecessor_v1(bytes: &mut Vec<u8>, predecessor: &ExternalWatermarkPredecessorV1) {
    match predecessor {
        ExternalWatermarkPredecessorV1::GenesisPinnedSentinel {
            genesis_epoch_binding_sha256,
            provider_profile_sha256,
            revision,
            stream_id_sha256,
            tip_sha256,
        } => {
            append_field_v1(bytes, "predecessor_kind", b"genesis_pinned_sentinel");
            append_field_v1(
                bytes,
                "predecessor_epoch_binding_sha256",
                genesis_epoch_binding_sha256.as_bytes(),
            );
            append_field_v1(
                bytes,
                "predecessor_provider_profile_sha256",
                provider_profile_sha256.as_bytes(),
            );
            append_u64_v1(bytes, "predecessor_revision", *revision);
            append_field_v1(
                bytes,
                "predecessor_stream_id_sha256",
                stream_id_sha256.as_bytes(),
            );
            append_field_v1(bytes, "predecessor_tip_sha256", tip_sha256.as_bytes());
        }
        ExternalWatermarkPredecessorV1::Successor {
            installed_epoch_binding_sha256,
            installed_epoch_sequence,
            provider_profile_sha256,
            revision,
            stream_id_sha256,
            tip_sha256,
        } => {
            append_field_v1(bytes, "predecessor_kind", b"successor");
            append_field_v1(
                bytes,
                "predecessor_epoch_binding_sha256",
                installed_epoch_binding_sha256.as_bytes(),
            );
            append_u64_v1(
                bytes,
                "predecessor_installed_epoch_sequence",
                *installed_epoch_sequence,
            );
            append_field_v1(
                bytes,
                "predecessor_provider_profile_sha256",
                provider_profile_sha256.as_bytes(),
            );
            append_u64_v1(bytes, "predecessor_revision", *revision);
            append_field_v1(
                bytes,
                "predecessor_stream_id_sha256",
                stream_id_sha256.as_bytes(),
            );
            append_field_v1(bytes, "predecessor_tip_sha256", tip_sha256.as_bytes());
        }
    }
}

fn append_field_v1(bytes: &mut Vec<u8>, label: &str, value: &[u8]) {
    bytes.extend_from_slice(&(label.len() as u64).to_be_bytes());
    bytes.extend_from_slice(label.as_bytes());
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value);
}

fn append_u64_v1(bytes: &mut Vec<u8>, label: &str, value: u64) {
    append_field_v1(bytes, label, &value.to_be_bytes());
}

fn sha256_v1(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

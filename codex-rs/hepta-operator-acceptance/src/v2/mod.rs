mod evidence;
mod model;

use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::AcceptanceError;
use crate::ceremony::nonce_shape;
use crate::ceremony::path_present;
use crate::ceremony::path_string;
use crate::ceremony::random_hex;
use crate::ceremony::reject_existing;
use crate::ceremony::trusted_time;
use crate::durable::MAX_SMALL_FILE_BYTES;
use crate::durable::canonical_json;
use crate::durable::ensure_disjoint_roots;
use crate::durable::lock_existing_sidecar;
use crate::durable::lock_sidecar;
use crate::durable::secure_canonical_file_path;
use crate::durable::secure_read;
use crate::durable::secure_root;
use crate::durable::sha256;
use crate::durable::write_private_atomic_replace;
use crate::durable::write_private_new;
use crate::model::AuthorityBoundary;
use crate::model::OperatorBinding;
use crate::model::SignatureBinding;
use crate::receipt_store::read_canonical;
use crate::trust::SIGNATURE_ALGORITHM;
use crate::trust::SSHSIG_NAMESPACE_V2;
use crate::trust::TRUST_POLICY_SCOPE_V2;
use crate::trust::TrustAnchor;
use crate::trust::TrustInputs;
use crate::trust::VerifiedSignature;

use evidence::AGGREGATE_ROOT;
use evidence::AggregateEvidence;
use evidence::CANDIDATE_HEAD;
use evidence::CANDIDATE_TREE;
use evidence::LEGACY_PRODUCT_AUDIT_ROOT;
use evidence::load_aggregate_evidence;

pub use model::AcceptanceChallengeV2;
pub use model::AcceptanceReceiptV2;
pub use model::AggregateManifestBinding;
pub use model::AggregateQualificationPacket;
pub use model::CandidateBindingV2;
use model::NonceClaimV2;
pub use model::PlatformGateBinding;
pub use model::PlatformPolicy;
pub use model::PreparedChallengeV2;
pub use model::PrerequisiteReceiptBinding;
pub use model::QualificationAssessment;
pub use model::QualificationDecision;
pub use model::ReceiptManifestBinding;
pub use model::SealedAcceptanceV2;

const CHALLENGE_SCHEMA: &str = "hepta_operator_acceptance_v2";
const RECEIPT_SCHEMA: &str = "hepta_operator_acceptance_receipt_v2";
const CLAIM_SCHEMA: &str = "hepta_operator_acceptance_nonce_claim_v2";
const WATERMARK_SCHEMA: &str = "hepta_operator_acceptance_time_watermark_v2";
const ACCEPTANCE_SCOPE: &str = "exact_09e9_aggregate_qualification_evidence_only";
const DECISION: &str = "accept";
const DECLARATION: &str = "Accept only the exact 09e9 aggregate qualification packet and its fully verified platform receipt manifests. This grants no authority for Enforce, promotion, outbound, retirement, or automatic transition. A GitHub zero-step BLOCKED_EXTERNAL receipt is explicitly excluded from PASS and cannot satisfy the required GitHub gate.";
const CHALLENGE_FILE: &str = "operator-acceptance-v2-challenge.json";
const CLAIM_FILE: &str = "operator-acceptance-v2-nonce-claim.json";
const RECEIPT_FILE: &str = "operator-acceptance-v2-receipt.json";
const WATERMARK_FILE: &str = "operator-acceptance-v2-time-watermark.json";
const DEFAULT_CHALLENGE_LIFETIME_SECONDS: u64 = 900;
const ACCEPTANCE_STORE_PARENT: &str = "/Volumes/T5/hepta-vnext/artifacts/acceptances";
const ACCEPTANCE_STORE_PREFIX: &str = "vnext-main-09e9e9ff7f-operator-acceptance-v2";

const FORMAL_ENVIRONMENT: [(&str, &str); 5] = [
    ("HEPTA_SSD_ROOT", "/Volumes/T5/hepta-vnext"),
    (
        "HEPTA_SSD_VOLUME_UUID",
        "FB804D1B-24CB-4D6E-AEA7-A9E180807758",
    ),
    ("HEPTA_LANE", "operator-acceptance-09e9"),
    (
        "HEPTA_WORKTREE",
        "/Volumes/T5/hepta-vnext/worktrees/operator-acceptance-09e9",
    ),
    ("HEPTA_ARTIFACTS_DIR", "/Volumes/T5/hepta-vnext/artifacts"),
];

pub struct AssessRequest<'a> {
    pub aggregate_manifest_sha256: &'a str,
    pub aggregate_root: &'a Path,
    pub legacy_product_audit_root: &'a Path,
}

pub struct PrepareRequestV2<'a> {
    pub aggregate_manifest_sha256: &'a str,
    pub aggregate_root: &'a Path,
    pub allowed_signers_path: &'a Path,
    pub externally_pinned_trust_policy_sha256: &'a str,
    pub legacy_product_audit_root: &'a Path,
    pub sidecar_root: &'a Path,
    pub trust_policy_path: &'a Path,
}

pub struct VerifyRequestV2<'a> {
    pub aggregate_manifest_sha256: &'a str,
    pub aggregate_root: &'a Path,
    pub allowed_signers_path: &'a Path,
    pub externally_pinned_trust_policy_sha256: &'a str,
    pub legacy_product_audit_root: &'a Path,
    pub sidecar_root: &'a Path,
    pub signature_path: &'a Path,
    pub trust_policy_path: &'a Path,
}

pub struct ReadReceiptRequestV2<'a> {
    pub aggregate_manifest_sha256: &'a str,
    pub aggregate_root: &'a Path,
    pub allowed_signers_path: &'a Path,
    pub externally_pinned_trust_policy_sha256: &'a str,
    pub legacy_product_audit_root: &'a Path,
    pub sidecar_root: &'a Path,
    pub trust_policy_path: &'a Path,
}

struct ValidatedRootsV2 {
    aggregate: PathBuf,
    allowed_signers: PathBuf,
    legacy_product_audit: PathBuf,
    sidecar: PathBuf,
    trust_policy: PathBuf,
}

impl ValidatedRootsV2 {
    fn load(
        aggregate: &Path,
        legacy_product_audit: &Path,
        sidecar: &Path,
        allowed_signers: &Path,
        trust_policy: &Path,
    ) -> Result<Self, AcceptanceError> {
        let aggregate = secure_root(aggregate, "aggregate qualification root")?;
        let legacy_product_audit = secure_root(legacy_product_audit, "legacy product audit root")?;
        let sidecar = secure_root(sidecar, "operator acceptance V2 sidecar root")?;
        if aggregate != Path::new(AGGREGATE_ROOT)
            || legacy_product_audit != Path::new(LEGACY_PRODUCT_AUDIT_ROOT)
        {
            return Err(invalid("V2 evidence roots differ from the exact 09e9 pins"));
        }
        let sidecar_parent = Path::new(ACCEPTANCE_STORE_PARENT);
        let basename = sidecar.file_name().and_then(|value| value.to_str());
        if sidecar.parent() != Some(sidecar_parent)
            || !basename.is_some_and(|value| value.starts_with(ACCEPTANCE_STORE_PREFIX))
        {
            return Err(invalid(
                "V2 sidecar must be a named strict child of the canonical acceptance store",
            ));
        }
        ensure_disjoint_roots(&sidecar, &aggregate, &legacy_product_audit)?;
        let allowed_signers =
            secure_canonical_file_path(allowed_signers, "external allowed_signers")?;
        let trust_policy = secure_canonical_file_path(trust_policy, "external V2 trust policy")?;
        for external in [&allowed_signers, &trust_policy] {
            if external.starts_with(&sidecar)
                || external.starts_with(&aggregate)
                || external.starts_with(&legacy_product_audit)
            {
                return Err(invalid(
                    "external trust material must be outside evidence and sidecar roots",
                ));
            }
        }
        if allowed_signers == trust_policy {
            return Err(invalid(
                "allowed_signers and V2 trust policy must be distinct files",
            ));
        }
        Ok(Self {
            aggregate,
            allowed_signers,
            legacy_product_audit,
            sidecar,
            trust_policy,
        })
    }
}

pub fn require_formal_environment_v2() -> Result<(), AcceptanceError> {
    validate_formal_environment_with(|name| std::env::var_os(name))
}

fn validate_formal_environment_with(
    lookup: impl Fn(&str) -> Option<OsString>,
) -> Result<(), AcceptanceError> {
    for (name, expected) in FORMAL_ENVIRONMENT {
        if lookup(name).as_deref() != Some(OsStr::new(expected)) {
            return Err(invalid(format!(
                "formal V2 operator acceptance requires exact {name} from hepta-ssd-run",
            )));
        }
    }
    Ok(())
}

pub fn assess(request: AssessRequest<'_>) -> Result<QualificationAssessment, AcceptanceError> {
    require_formal_environment_v2()?;
    let evidence = load_aggregate_evidence(
        request.aggregate_root,
        request.aggregate_manifest_sha256,
        request.legacy_product_audit_root,
    )?;
    Ok(evidence.assessment)
}

pub fn prepare_v2(request: PrepareRequestV2<'_>) -> Result<PreparedChallengeV2, AcceptanceError> {
    require_formal_environment_v2()?;
    let roots = ValidatedRootsV2::load(
        request.aggregate_root,
        request.legacy_product_audit_root,
        request.sidecar_root,
        request.allowed_signers_path,
        request.trust_policy_path,
    )?;
    let evidence = load_aggregate_evidence(
        &roots.aggregate,
        request.aggregate_manifest_sha256,
        &roots.legacy_product_audit,
    )?;
    require_ready(&evidence)?;
    let trust = load_trust_v2(&roots, request.externally_pinned_trust_policy_sha256)?;

    let _lock = lock_sidecar(&roots.sidecar)?;
    reject_existing(&roots.sidecar.join(CHALLENGE_FILE), "V2 challenge")?;
    reject_existing(&roots.sidecar.join(CLAIM_FILE), "V2 nonce claim")?;
    reject_existing(&roots.sidecar.join(RECEIPT_FILE), "V2 acceptance receipt")?;
    let issued_at = trusted_time()?;
    advance_time_watermark_v2(
        &roots.sidecar,
        issued_at,
        request.externally_pinned_trust_policy_sha256,
    )?;
    let lifetime = DEFAULT_CHALLENGE_LIFETIME_SECONDS.min(trust.binding.maximum_lifetime_seconds);
    let expires_at = issued_at
        .checked_add(lifetime)
        .ok_or_else(|| invalid("V2 challenge expiration overflows trusted time"))?;
    let challenge = AcceptanceChallengeV2 {
        aggregate_manifest: evidence.aggregate_manifest.clone(),
        automatic_transition: false,
        authority: AuthorityBoundary::evidence_acceptance_only(),
        candidate: evidence.packet.candidate.clone(),
        decision: DECISION.to_string(),
        declaration: DECLARATION.to_string(),
        expires_at_unix_seconds: expires_at,
        issued_at_unix_seconds: issued_at,
        namespace: SSHSIG_NAMESPACE_V2.to_string(),
        nonce: random_hex::<32>()?,
        not_before_unix_seconds: issued_at,
        operator: trust.binding.clone(),
        qualification_packet: evidence.packet.clone(),
        schema: CHALLENGE_SCHEMA.to_string(),
        schema_version: 2,
        scope: ACCEPTANCE_SCOPE.to_string(),
        signature_algorithm: SIGNATURE_ALGORITHM.to_string(),
    };
    validate_challenge_v2(&challenge, &evidence, &trust.binding)?;
    let bytes = canonical_json(&challenge)?;
    let digest = sha256(&bytes);
    let path = roots.sidecar.join(CHALLENGE_FILE);
    write_private_new(&path, &bytes)?;
    Ok(PreparedChallengeV2 {
        challenge_path: path_string(&path)?,
        challenge_sha256: digest,
        expires_at_unix_seconds: expires_at,
    })
}

pub fn verify_and_seal_v2(
    request: VerifyRequestV2<'_>,
) -> Result<SealedAcceptanceV2, AcceptanceError> {
    require_formal_environment_v2()?;
    let roots = ValidatedRootsV2::load(
        request.aggregate_root,
        request.legacy_product_audit_root,
        request.sidecar_root,
        request.allowed_signers_path,
        request.trust_policy_path,
    )?;
    let evidence = load_aggregate_evidence(
        &roots.aggregate,
        request.aggregate_manifest_sha256,
        &roots.legacy_product_audit,
    )?;
    require_ready(&evidence)?;
    let _lock = lock_sidecar(&roots.sidecar)?;
    let first_time = trusted_time()?;
    advance_time_watermark_v2(
        &roots.sidecar,
        first_time,
        request.externally_pinned_trust_policy_sha256,
    )?;
    let trust = load_trust_v2(&roots, request.externally_pinned_trust_policy_sha256)?;
    let challenge_path = roots.sidecar.join(CHALLENGE_FILE);
    let (challenge, challenge_bytes) =
        read_canonical::<AcceptanceChallengeV2>(&challenge_path, "V2 acceptance challenge")?;
    validate_challenge_v2(&challenge, &evidence, &trust.binding)?;
    let challenge_sha256 = sha256(&challenge_bytes);
    let receipt_path = roots.sidecar.join(RECEIPT_FILE);
    let claim_path = roots.sidecar.join(CLAIM_FILE);
    if path_present(&receipt_path)? {
        return verify_stored_acceptance_v2(
            &receipt_path,
            &claim_path,
            &challenge,
            &challenge_bytes,
            &challenge_sha256,
            &trust,
        );
    }
    if path_present(&claim_path)? {
        return Err(invalid(
            "V2 nonce was durably claimed but no PASS receipt exists; fail closed",
        ));
    }
    validate_time_window_v2(&challenge, first_time)?;
    let verified = trust.verify(&challenge_bytes, request.signature_path)?;

    let trust_after = load_trust_v2(&roots, request.externally_pinned_trust_policy_sha256)?;
    if trust_after.binding != trust.binding {
        return Err(invalid(
            "external V2 trust policy changed during verification",
        ));
    }
    let evidence_after = load_aggregate_evidence(
        &roots.aggregate,
        request.aggregate_manifest_sha256,
        &roots.legacy_product_audit,
    )?;
    if evidence_after != evidence {
        return Err(invalid(
            "V2 aggregate evidence changed during signature verification",
        ));
    }
    if secure_read(&challenge_path, MAX_SMALL_FILE_BYTES)? != challenge_bytes {
        return Err(invalid(
            "canonical V2 challenge changed during signature verification",
        ));
    }
    let accepted_at = trusted_time()?;
    advance_time_watermark_v2(
        &roots.sidecar,
        accepted_at,
        request.externally_pinned_trust_policy_sha256,
    )?;
    persist_final_acceptance_v2(
        &receipt_path,
        &claim_path,
        &challenge,
        &challenge_sha256,
        &trust.binding,
        &verified,
        accepted_at,
    )
}

pub fn verify_receipt_v2(
    request: ReadReceiptRequestV2<'_>,
) -> Result<SealedAcceptanceV2, AcceptanceError> {
    require_formal_environment_v2()?;
    let roots = ValidatedRootsV2::load(
        request.aggregate_root,
        request.legacy_product_audit_root,
        request.sidecar_root,
        request.allowed_signers_path,
        request.trust_policy_path,
    )?;
    let _lock = lock_existing_sidecar(&roots.sidecar)?;
    let evidence = load_aggregate_evidence(
        &roots.aggregate,
        request.aggregate_manifest_sha256,
        &roots.legacy_product_audit,
    )?;
    require_ready(&evidence)?;
    let trust = load_trust_v2(&roots, request.externally_pinned_trust_policy_sha256)?;
    let challenge_path = roots.sidecar.join(CHALLENGE_FILE);
    let (challenge, challenge_bytes) =
        read_canonical::<AcceptanceChallengeV2>(&challenge_path, "V2 acceptance challenge")?;
    validate_challenge_v2(&challenge, &evidence, &trust.binding)?;
    let challenge_sha256 = sha256(&challenge_bytes);
    verify_stored_acceptance_v2(
        &roots.sidecar.join(RECEIPT_FILE),
        &roots.sidecar.join(CLAIM_FILE),
        &challenge,
        &challenge_bytes,
        &challenge_sha256,
        &trust,
    )
}

fn require_ready(evidence: &AggregateEvidence) -> Result<(), AcceptanceError> {
    if !evidence.assessment.ready_for_challenge {
        return Err(invalid(format!(
            "aggregate qualification is not ready; required blockers: {}",
            evidence.assessment.blockers.join(",")
        )));
    }
    Ok(())
}

fn load_trust_v2(
    roots: &ValidatedRootsV2,
    externally_pinned_trust_policy_sha256: &str,
) -> Result<TrustAnchor, AcceptanceError> {
    TrustAnchor::load_v2(TrustInputs {
        acceptance_store_root: &roots.sidecar,
        allowed_signers_path: &roots.allowed_signers,
        externally_pinned_trust_policy_sha256,
        trust_policy_path: &roots.trust_policy,
    })
}

fn validate_challenge_v2(
    challenge: &AcceptanceChallengeV2,
    evidence: &AggregateEvidence,
    operator: &OperatorBinding,
) -> Result<(), AcceptanceError> {
    if challenge.schema != CHALLENGE_SCHEMA
        || challenge.schema_version != 2
        || challenge.namespace != SSHSIG_NAMESPACE_V2
        || challenge.signature_algorithm != SIGNATURE_ALGORITHM
        || challenge.scope != ACCEPTANCE_SCOPE
        || challenge.decision != DECISION
        || challenge.declaration != DECLARATION
        || challenge.automatic_transition
        || challenge.authority != AuthorityBoundary::evidence_acceptance_only()
        || challenge.candidate != evidence.packet.candidate
        || challenge.aggregate_manifest != evidence.aggregate_manifest
        || challenge.qualification_packet != evidence.packet
        || challenge.operator != *operator
        || challenge.operator.trust_policy_scope != TRUST_POLICY_SCOPE_V2
        || challenge.qualification_packet.decision.verdict != "PASS"
        || !evidence.assessment.ready_for_challenge
    {
        return Err(invalid(
            "V2 challenge differs from the exact aggregate evidence-acceptance boundary",
        ));
    }
    if challenge.candidate.head != CANDIDATE_HEAD || challenge.candidate.tree != CANDIDATE_TREE {
        return Err(invalid("V2 challenge candidate identity differs from 09e9"));
    }
    if !nonce_shape(&challenge.nonce)
        || challenge.issued_at_unix_seconds == 0
        || challenge.not_before_unix_seconds != challenge.issued_at_unix_seconds
        || challenge.expires_at_unix_seconds <= challenge.issued_at_unix_seconds
    {
        return Err(invalid(
            "V2 challenge nonce or validity interval is malformed",
        ));
    }
    let lifetime = challenge
        .expires_at_unix_seconds
        .checked_sub(challenge.issued_at_unix_seconds)
        .ok_or_else(|| invalid("V2 challenge validity interval underflow"))?;
    if lifetime > DEFAULT_CHALLENGE_LIFETIME_SECONDS || lifetime > operator.maximum_lifetime_seconds
    {
        return Err(invalid("V2 challenge lifetime exceeds the external policy"));
    }
    Ok(())
}

fn validate_time_window_v2(
    challenge: &AcceptanceChallengeV2,
    trusted_now: u64,
) -> Result<(), AcceptanceError> {
    if trusted_now < challenge.not_before_unix_seconds
        || trusted_now < challenge.issued_at_unix_seconds
        || trusted_now >= challenge.expires_at_unix_seconds
    {
        return Err(invalid(
            "trusted host time is outside the signed V2 validity window",
        ));
    }
    Ok(())
}

fn persist_final_acceptance_v2(
    receipt_path: &Path,
    claim_path: &Path,
    challenge: &AcceptanceChallengeV2,
    challenge_sha256: &str,
    operator: &OperatorBinding,
    verified: &VerifiedSignature,
    accepted_at_unix_seconds: u64,
) -> Result<SealedAcceptanceV2, AcceptanceError> {
    if accepted_at_unix_seconds < challenge.not_before_unix_seconds
        || accepted_at_unix_seconds < challenge.issued_at_unix_seconds
        || accepted_at_unix_seconds >= challenge.expires_at_unix_seconds
    {
        return Err(invalid(
            "final trusted host time is outside the signed V2 validity window",
        ));
    }
    if path_present(claim_path)? || path_present(receipt_path)? {
        return Err(invalid(
            "V2 acceptance sidecar changed before nonce consumption",
        ));
    }
    let claim = NonceClaimV2 {
        accepted_at_unix_seconds,
        challenge_sha256: challenge_sha256.to_string(),
        detached_signature_sha256: verified.detached_signature_sha256.clone(),
        nonce: challenge.nonce.clone(),
        schema: CLAIM_SCHEMA.to_string(),
        schema_version: 2,
    };
    write_private_new(claim_path, &canonical_json(&claim)?)?;
    let signature = signature_binding_v2(operator, verified);
    let receipt = AcceptanceReceiptV2 {
        accepted_at_unix_seconds,
        authority: AuthorityBoundary::evidence_acceptance_only(),
        challenge: challenge.clone(),
        challenge_sha256: challenge_sha256.to_string(),
        schema: RECEIPT_SCHEMA.to_string(),
        schema_version: 2,
        signature,
    };
    validate_receipt_v2(&receipt, challenge, challenge_sha256, &receipt.signature)?;
    let receipt_bytes = canonical_json(&receipt)?;
    let receipt_sha256 = sha256(&receipt_bytes);
    write_private_new(receipt_path, &receipt_bytes)?;
    Ok(SealedAcceptanceV2 {
        acceptance_receipt_path: path_string(receipt_path)?,
        acceptance_receipt_sha256: receipt_sha256,
        challenge_sha256: challenge_sha256.to_string(),
    })
}

fn verify_stored_acceptance_v2(
    receipt_path: &Path,
    claim_path: &Path,
    challenge: &AcceptanceChallengeV2,
    challenge_bytes: &[u8],
    challenge_sha256: &str,
    trust: &TrustAnchor,
) -> Result<SealedAcceptanceV2, AcceptanceError> {
    let (stored, _) = read_canonical::<AcceptanceReceiptV2>(receipt_path, "V2 acceptance receipt")?;
    let verified = trust.verify_base64(
        challenge_bytes,
        &stored.signature.detached_signature_sshsig_base64,
    )?;
    let expected_signature = signature_binding_v2(&trust.binding, &verified);
    validate_receipt_v2(&stored, challenge, challenge_sha256, &expected_signature)?;
    let (claim, _) = read_canonical::<NonceClaimV2>(claim_path, "V2 nonce claim")?;
    let expected_claim = NonceClaimV2 {
        accepted_at_unix_seconds: stored.accepted_at_unix_seconds,
        challenge_sha256: challenge_sha256.to_string(),
        detached_signature_sha256: expected_signature.detached_signature_sha256,
        nonce: challenge.nonce.clone(),
        schema: CLAIM_SCHEMA.to_string(),
        schema_version: 2,
    };
    if claim != expected_claim {
        return Err(invalid(
            "stored V2 nonce claim conflicts with the exact replay",
        ));
    }
    Ok(SealedAcceptanceV2 {
        acceptance_receipt_path: path_string(receipt_path)?,
        acceptance_receipt_sha256: sha256(&canonical_json(&stored)?),
        challenge_sha256: challenge_sha256.to_string(),
    })
}

fn validate_receipt_v2(
    receipt: &AcceptanceReceiptV2,
    challenge: &AcceptanceChallengeV2,
    challenge_sha256: &str,
    signature: &SignatureBinding,
) -> Result<(), AcceptanceError> {
    if receipt.schema != RECEIPT_SCHEMA
        || receipt.schema_version != 2
        || receipt.authority != AuthorityBoundary::evidence_acceptance_only()
        || receipt.authority != receipt.challenge.authority
        || receipt.challenge != *challenge
        || receipt.challenge_sha256 != challenge_sha256
        || receipt.signature != *signature
        || receipt.accepted_at_unix_seconds < challenge.issued_at_unix_seconds
        || receipt.accepted_at_unix_seconds >= challenge.expires_at_unix_seconds
    {
        return Err(invalid("stored V2 acceptance receipt is inconsistent"));
    }
    Ok(())
}

fn signature_binding_v2(
    operator: &OperatorBinding,
    verified: &VerifiedSignature,
) -> SignatureBinding {
    SignatureBinding {
        algorithm: SIGNATURE_ALGORITHM.to_string(),
        allowed_signers_sha256: operator.allowed_signers_sha256.clone(),
        detached_signature_sha256: verified.detached_signature_sha256.clone(),
        detached_signature_sshsig_base64: verified.detached_signature_sshsig_base64.clone(),
        key_fingerprint: operator.key_fingerprint.clone(),
        namespace: SSHSIG_NAMESPACE_V2.to_string(),
        principal: operator.principal.clone(),
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct TimeWatermarkV2 {
    last_observed_unix_seconds: u64,
    schema: String,
    schema_version: u32,
    trust_policy_sha256: String,
}

fn advance_time_watermark_v2(
    sidecar: &Path,
    observed: u64,
    trust_policy_sha256: &str,
) -> Result<(), AcceptanceError> {
    let path = sidecar.join(WATERMARK_FILE);
    if path_present(&path)? {
        let (stored, _) = read_canonical::<TimeWatermarkV2>(&path, "V2 trusted-time watermark")?;
        if stored.schema != WATERMARK_SCHEMA
            || stored.schema_version != 2
            || stored.trust_policy_sha256 != trust_policy_sha256
        {
            return Err(invalid(
                "V2 trusted-time watermark has a different policy scope",
            ));
        }
        if observed < stored.last_observed_unix_seconds {
            return Err(invalid(
                "trusted host clock moved behind its durable V2 watermark",
            ));
        }
    }
    let watermark = TimeWatermarkV2 {
        last_observed_unix_seconds: observed,
        schema: WATERMARK_SCHEMA.to_string(),
        schema_version: 2,
        trust_policy_sha256: trust_policy_sha256.to_string(),
    };
    let temporary = sidecar.join(format!(
        ".operator-acceptance-v2-watermark-{}.tmp",
        random_hex::<16>()?
    ));
    write_private_atomic_replace(&path, &temporary, &canonical_json(&watermark)?)
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD;

    use super::*;

    #[test]
    fn formal_environment_is_exact_and_lane_specific() {
        let exact = FORMAL_ENVIRONMENT.into_iter().collect::<BTreeMap<_, _>>();
        validate_formal_environment_with(|name| exact.get(name).map(OsString::from))
            .expect("exact V2 wrapper environment");
        for name in exact.keys() {
            assert!(
                validate_formal_environment_with(|candidate| {
                    if candidate == *name {
                        Some(OsString::from("wrong"))
                    } else {
                        exact.get(candidate).map(OsString::from)
                    }
                })
                .is_err()
            );
        }
    }

    #[test]
    fn signature_binding_has_v2_namespace() {
        let operator = OperatorBinding {
            acceptance_store_root: "/store".to_string(),
            allowed_signers_sha256: "a".repeat(64),
            key_fingerprint: "SHA256:abcdefghijklmnopqrstuvwx".to_string(),
            maximum_lifetime_seconds: 900,
            principal: "operator@example".to_string(),
            trust_policy_scope: TRUST_POLICY_SCOPE_V2.to_string(),
            trust_policy_sha256: "b".repeat(64),
            trust_root_id: "operator-root".to_string(),
            trust_root_revision: 2,
        };
        let verified = VerifiedSignature {
            detached_signature_sha256: "c".repeat(64),
            detached_signature_sshsig_base64: STANDARD.encode(b"fixture"),
        };
        let binding = signature_binding_v2(&operator, &verified);
        assert_eq!(binding.namespace, SSHSIG_NAMESPACE_V2);
        assert_eq!(binding.algorithm, SIGNATURE_ALGORITHM);
    }
}

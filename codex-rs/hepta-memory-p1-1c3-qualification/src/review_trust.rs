use crate::trust::{TrustDomain, TrustRole, TrustStore, VerifiedSignatureReceipt, verify_signed_digest};
use crate::{ContractError, Digest32, SignedDigest, framed_digest, validate_id};
use hepta_memory_p1_1c1_qualification::{
    AdjudicationRecord, CitationLabel, ContradictionLabel, PrivacyDecision, ReviewBatch,
    ReviewRecord, ReviewTuple,
};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_REVIEW_ATTESTATIONS: usize = 16_384;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewAttestation {
    pub item_id: String,
    pub reviewer_key_id: String,
    pub review_record_sha256: Digest32,
    pub signed: SignedDigest,
}

impl ReviewAttestation {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.item_id, "review attestation item ID")?;
        validate_id(&self.reviewer_key_id, "review attestation reviewer key ID")?;
        if self.signed.key_id != self.reviewer_key_id
            || self.signed.payload_sha256 != self.review_record_sha256
        {
            return Err(ContractError::Corrupt(
                "review attestation signature binding mismatch".to_string(),
            ));
        }
        self.signed.validate()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdjudicationAttestation {
    pub item_id: String,
    pub adjudicator_key_id: String,
    pub adjudication_record_sha256: Digest32,
    pub signed: SignedDigest,
}

impl AdjudicationAttestation {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.item_id, "adjudication attestation item ID")?;
        validate_id(
            &self.adjudicator_key_id,
            "adjudication attestation key ID",
        )?;
        if self.signed.key_id != self.adjudicator_key_id
            || self.signed.payload_sha256 != self.adjudication_record_sha256
        {
            return Err(ContractError::Corrupt(
                "adjudication attestation signature binding mismatch".to_string(),
            ));
        }
        self.signed.validate()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewTrustBundle {
    pub review_attestations: Vec<ReviewAttestation>,
    pub adjudication_attestations: Vec<AdjudicationAttestation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewTrustPolicy {
    pub policy_id: String,
    pub expected_trust_store_sha256: Digest32,
    pub require_external_signers: bool,
    pub require_distinct_reviewer_affiliations: bool,
    pub require_independent_adjudicator_affiliation: bool,
    pub policy_sha256: Digest32,
}

impl ReviewTrustPolicy {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.policy_id, "review trust policy ID")?;
        if self.policy_sha256 != review_trust_policy_digest(self) {
            return Err(ContractError::Corrupt(
                "review trust policy digest mismatch".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewTrustReceipt {
    pub policy_id: String,
    pub trust_store_sha256: Digest32,
    pub review_batch_sha256: Digest32,
    pub adjudication_batch_sha256: Digest32,
    pub reviewer_set_sha256: Digest32,
    pub signed_review_count: u32,
    pub signed_adjudication_count: u32,
    pub item_count: u32,
    pub reviewer_affiliations_independent: bool,
    pub adjudicator_affiliations_independent: bool,
    pub all_signers_external_attested: bool,
    pub signature_receipts_sha256: Digest32,
    pub receipt_sha256: Digest32,
    verified: bool,
}

impl ReviewTrustReceipt {
    pub fn validate(&self) -> Result<(), ContractError> {
        if !self.verified {
            return Err(ContractError::Corrupt(
                "review trust receipt is not verified".to_string(),
            ));
        }
        if self.receipt_sha256 != review_trust_receipt_digest(self) {
            return Err(ContractError::Corrupt(
                "review trust receipt digest mismatch".to_string(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.verified
    }
}

pub fn verify_review_trust(
    batch: &ReviewBatch,
    bundle: &ReviewTrustBundle,
    policy: &ReviewTrustPolicy,
    trust_store: &TrustStore,
    now_unix_seconds: u64,
) -> Result<ReviewTrustReceipt, ContractError> {
    batch
        .validate()
        .map_err(|error| ContractError::Invalid(error.to_string()))?;
    policy.validate()?;
    trust_store.validate()?;
    if trust_store.store_sha256 != policy.expected_trust_store_sha256 {
        return Err(ContractError::Invalid(
            "review trust store does not match policy".to_string(),
        ));
    }
    if bundle.review_attestations.len() != batch.reviews.len()
        || bundle.adjudication_attestations.len() != batch.adjudications.len()
        || bundle.review_attestations.len() > MAX_REVIEW_ATTESTATIONS
        || bundle.adjudication_attestations.len() > MAX_REVIEW_ATTESTATIONS
    {
        return Err(ContractError::Invalid(
            "review trust attestation counts do not match the review batch".to_string(),
        ));
    }

    let mut review_attestations = BTreeMap::new();
    for attestation in &bundle.review_attestations {
        attestation.validate()?;
        let key = (attestation.item_id.as_str(), attestation.reviewer_key_id.as_str());
        if review_attestations.insert(key, attestation).is_some() {
            return Err(ContractError::Duplicate(format!(
                "review attestation {}:{}",
                attestation.item_id, attestation.reviewer_key_id
            )));
        }
    }
    let mut adjudication_attestations = BTreeMap::new();
    for attestation in &bundle.adjudication_attestations {
        attestation.validate()?;
        if adjudication_attestations
            .insert(attestation.item_id.as_str(), attestation)
            .is_some()
        {
            return Err(ContractError::Duplicate(format!(
                "adjudication attestation {}",
                attestation.item_id
            )));
        }
    }

    let mut signature_receipts = Vec::<VerifiedSignatureReceipt>::new();
    let mut item_affiliations = BTreeMap::<&str, BTreeSet<String>>::new();
    let mut reviewer_commitments = BTreeSet::new();
    for review in &batch.reviews {
        let key = trust_store
            .keys
            .iter()
            .find(|trusted| {
                trusted.role == TrustRole::Reviewer
                    && trusted.commitment_sha256().to_hex()
                        == review.reviewer_commitment.to_string()
            })
            .ok_or_else(|| {
                ContractError::Missing(format!(
                    "reviewer key for item {} commitment {}",
                    review.item_id, review.reviewer_commitment
                ))
            })?;
        let attestation = review_attestations
            .get(&(review.item_id.as_str(), key.key_id.as_str()))
            .ok_or_else(|| {
                ContractError::Missing(format!(
                    "review signature {}:{}",
                    review.item_id, key.key_id
                ))
            })?;
        let expected_digest = review_record_digest(review)?;
        if attestation.review_record_sha256 != expected_digest {
            return Err(ContractError::Corrupt(format!(
                "review signature digest mismatch for item {}",
                review.item_id
            )));
        }
        let verified = verify_signed_digest(
            trust_store,
            &attestation.signed,
            TrustRole::Reviewer,
            Some(&review.locale),
            now_unix_seconds,
            policy.require_external_signers,
        )?;
        reviewer_commitments.insert(key.commitment_sha256());
        item_affiliations
            .entry(&review.item_id)
            .or_default()
            .insert(verified.affiliation_id.clone());
        signature_receipts.push(verified);
    }

    let reviewer_affiliations_independent = item_affiliations
        .values()
        .all(|affiliations| affiliations.len() == 2);
    if policy.require_distinct_reviewer_affiliations && !reviewer_affiliations_independent {
        return Err(ContractError::Invalid(
            "review pair affiliations are not independent".to_string(),
        ));
    }

    let mut adjudicator_affiliations_independent = true;
    for adjudication in &batch.adjudications {
        let key = trust_store
            .keys
            .iter()
            .find(|trusted| {
                trusted.role == TrustRole::Adjudicator
                    && trusted.commitment_sha256().to_hex()
                        == adjudication.adjudicator_commitment.to_string()
            })
            .ok_or_else(|| {
                ContractError::Missing(format!(
                    "adjudicator key for item {}",
                    adjudication.item_id
                ))
            })?;
        let attestation = adjudication_attestations
            .get(adjudication.item_id.as_str())
            .ok_or_else(|| {
                ContractError::Missing(format!(
                    "adjudication signature {}",
                    adjudication.item_id
                ))
            })?;
        if attestation.adjudicator_key_id != key.key_id
            || attestation.adjudication_record_sha256 != adjudication_record_digest(adjudication)?
        {
            return Err(ContractError::Corrupt(format!(
                "adjudication signature binding mismatch for item {}",
                adjudication.item_id
            )));
        }
        let verified = verify_signed_digest(
            trust_store,
            &attestation.signed,
            TrustRole::Adjudicator,
            None,
            now_unix_seconds,
            policy.require_external_signers,
        )?;
        if item_affiliations
            .get(adjudication.item_id.as_str())
            .is_some_and(|affiliations| affiliations.contains(&verified.affiliation_id))
        {
            adjudicator_affiliations_independent = false;
        }
        signature_receipts.push(verified);
    }
    if policy.require_independent_adjudicator_affiliation
        && !adjudicator_affiliations_independent
    {
        return Err(ContractError::Invalid(
            "adjudicator affiliation is not independent from the review pair".to_string(),
        ));
    }

    signature_receipts.sort_by(|left, right| {
        left.key_id
            .cmp(&right.key_id)
            .then_with(|| left.payload_sha256.cmp(&right.payload_sha256))
    });
    let signature_receipts_sha256 = framed_digest(
        b"hepta:intelligence:p1.1c3:review-signature-receipts:v1",
        &signature_receipts
            .iter()
            .map(|receipt| receipt.receipt_sha256.as_bytes().as_slice())
            .collect::<Vec<_>>(),
    );
    let all_signers_external_attested = signature_receipts
        .iter()
        .all(|receipt| receipt.domain == TrustDomain::ExternalAttested);
    let mut receipt = ReviewTrustReceipt {
        policy_id: policy.policy_id.clone(),
        trust_store_sha256: trust_store.store_sha256,
        review_batch_sha256: review_batch_digest(batch)?,
        adjudication_batch_sha256: adjudication_batch_digest(batch)?,
        reviewer_set_sha256: framed_digest(
            b"hepta:intelligence:p1.1c3:reviewer-set:v1",
            &reviewer_commitments
                .iter()
                .map(|digest| digest.as_bytes().as_slice())
                .collect::<Vec<_>>(),
        ),
        signed_review_count: u32::try_from(batch.reviews.len()).map_err(|_| ContractError::Overflow)?,
        signed_adjudication_count: u32::try_from(batch.adjudications.len())
            .map_err(|_| ContractError::Overflow)?,
        item_count: u32::try_from(batch.item_count()).map_err(|_| ContractError::Overflow)?,
        reviewer_affiliations_independent,
        adjudicator_affiliations_independent,
        all_signers_external_attested,
        signature_receipts_sha256,
        receipt_sha256: Digest32::for_bytes(b"uncomputed"),
        verified: true,
    };
    receipt.receipt_sha256 = review_trust_receipt_digest(&receipt);
    receipt.validate()?;
    Ok(receipt)
}

#[must_use]
pub fn review_trust_policy_digest(policy: &ReviewTrustPolicy) -> Digest32 {
    framed_digest(
        b"hepta:intelligence:p1.1c3:review-trust-policy:v1",
        &[
            policy.policy_id.as_bytes(),
            policy.expected_trust_store_sha256.as_bytes(),
            &[u8::from(policy.require_external_signers)],
            &[u8::from(policy.require_distinct_reviewer_affiliations)],
            &[u8::from(policy.require_independent_adjudicator_affiliation)],
        ],
    )
}

pub fn review_record_digest(review: &ReviewRecord) -> Result<Digest32, ContractError> {
    review
        .validate()
        .map_err(|error| ContractError::Invalid(error.to_string()))?;
    let labels = review_tuple_digest(review.labels);
    Ok(framed_digest(
        b"hepta:intelligence:p1.1c1:review-record:v1",
        &[
            review.item_id.as_bytes(),
            review.locale.as_bytes(),
            p1_digest(review.query_sha256)?.as_bytes(),
            p1_digest(review.candidate_sha256)?.as_bytes(),
            p1_digest(review.reviewer_commitment)?.as_bytes(),
            labels.as_bytes(),
            p1_digest(review.rationale_sha256)?.as_bytes(),
        ],
    ))
}

pub fn adjudication_record_digest(
    adjudication: &AdjudicationRecord,
) -> Result<Digest32, ContractError> {
    adjudication
        .validate()
        .map_err(|error| ContractError::Invalid(error.to_string()))?;
    let labels = review_tuple_digest(adjudication.labels);
    let redaction = adjudication
        .redaction_receipt_sha256
        .map(p1_digest)
        .transpose()?
        .unwrap_or_else(|| Digest32::for_bytes(b"no-redaction"));
    Ok(framed_digest(
        b"hepta:intelligence:p1.1c1:adjudication-record:v1",
        &[
            adjudication.item_id.as_bytes(),
            p1_digest(adjudication.adjudicator_commitment)?.as_bytes(),
            labels.as_bytes(),
            redaction.as_bytes(),
            p1_digest(adjudication.rationale_sha256)?.as_bytes(),
        ],
    ))
}

pub fn review_batch_digest(batch: &ReviewBatch) -> Result<Digest32, ContractError> {
    let digests = batch
        .reviews
        .iter()
        .map(review_record_digest)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(framed_digest(
        b"hepta:intelligence:p1.1c1:review-batch:v1",
        &digests
            .iter()
            .map(|digest| digest.as_bytes().as_slice())
            .collect::<Vec<_>>(),
    ))
}

pub fn adjudication_batch_digest(batch: &ReviewBatch) -> Result<Digest32, ContractError> {
    let digests = batch
        .adjudications
        .iter()
        .map(adjudication_record_digest)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(framed_digest(
        b"hepta:intelligence:p1.1c1:adjudication-batch:v1",
        &digests
            .iter()
            .map(|digest| digest.as_bytes().as_slice())
            .collect::<Vec<_>>(),
    ))
}

fn review_tuple_digest(labels: ReviewTuple) -> Digest32 {
    framed_digest(
        b"hepta:intelligence:p1.1c1:review-tuple:v1",
        &[
            &[labels.relevance],
            &[citation_code(labels.citation)],
            &[contradiction_code(labels.contradiction)],
            &[privacy_code(labels.privacy)],
        ],
    )
}

fn p1_digest(value: hepta_memory_p1_1c1_qualification::Digest32) -> Result<Digest32, ContractError> {
    Digest32::from_hex(&value.to_string())
}

const fn citation_code(value: CitationLabel) -> u8 {
    match value {
        CitationLabel::Unsupported => 0,
        CitationLabel::Partial => 1,
        CitationLabel::Supported => 2,
    }
}

const fn contradiction_code(value: ContradictionLabel) -> u8 {
    match value {
        ContradictionLabel::None => 0,
        ContradictionLabel::Potential => 1,
        ContradictionLabel::Confirmed => 2,
    }
}

const fn privacy_code(value: PrivacyDecision) -> u8 {
    match value {
        PrivacyDecision::Allow => 0,
        PrivacyDecision::Redact => 1,
        PrivacyDecision::Block => 2,
    }
}

fn review_trust_receipt_digest(receipt: &ReviewTrustReceipt) -> Digest32 {
    framed_digest(
        b"hepta:intelligence:p1.1c3:review-trust-receipt:v1",
        &[
            receipt.policy_id.as_bytes(),
            receipt.trust_store_sha256.as_bytes(),
            receipt.review_batch_sha256.as_bytes(),
            receipt.adjudication_batch_sha256.as_bytes(),
            receipt.reviewer_set_sha256.as_bytes(),
            &receipt.signed_review_count.to_be_bytes(),
            &receipt.signed_adjudication_count.to_be_bytes(),
            &receipt.item_count.to_be_bytes(),
            &[u8::from(receipt.reviewer_affiliations_independent)],
            &[u8::from(receipt.adjudicator_affiliations_independent)],
            &[u8::from(receipt.all_signers_external_attested)],
            receipt.signature_receipts_sha256.as_bytes(),
            &[u8::from(receipt.verified)],
        ],
    )
}

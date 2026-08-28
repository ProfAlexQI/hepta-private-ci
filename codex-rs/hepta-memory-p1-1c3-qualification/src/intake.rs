use crate::qualification::VerifiedQualificationReceipt;
use crate::review_trust::ReviewTrustReceipt;
use crate::trust::{
    SignedDigest, TrustRole, TrustStore, VerifiedSignatureReceipt, verify_signed_digest,
};
use crate::{ContractError, Digest32, framed_digest, validate_git_oid, validate_id};
use hepta_memory_p1_1c1_qualification::AcceptanceReceipt;
use std::collections::BTreeSet;

pub const INTAKE_SCHEMA: &str = "hepta.intelligence.p1_1c3.trusted_corpus_intake.v1";
pub const MAX_ALLOWED_LICENSES: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LicenseEvidence {
    pub dataset_sha256: Digest32,
    pub spdx_license_id: String,
    pub license_text_sha256: Digest32,
    pub permits_offline_evaluation: bool,
    pub permits_storage: bool,
    pub permits_derivatives: bool,
    pub fixture_only: bool,
    pub signed: SignedDigest,
}

impl LicenseEvidence {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.spdx_license_id, "dataset SPDX license ID")?;
        self.signed.validate()?;
        if self.signed.payload_sha256 != self.payload_sha256() {
            return Err(ContractError::Corrupt(
                "license signature payload mismatch".to_string(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn payload_sha256(&self) -> Digest32 {
        framed_digest(
            b"hepta:intelligence:p1.1c3:license-evidence:v1",
            &[
                self.dataset_sha256.as_bytes(),
                self.spdx_license_id.as_bytes(),
                self.license_text_sha256.as_bytes(),
                &[u8::from(self.permits_offline_evaluation)],
                &[u8::from(self.permits_storage)],
                &[u8::from(self.permits_derivatives)],
                &[u8::from(self.fixture_only)],
            ],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceEvidence {
    pub dataset_sha256: Digest32,
    pub source_manifest_sha256: Digest32,
    pub acquisition_method: String,
    pub legal_basis: String,
    pub collected_from_unix_seconds: u64,
    pub collected_to_unix_seconds: u64,
    pub human_review_source: bool,
    pub fixture_only: bool,
    pub signed: SignedDigest,
}

impl ProvenanceEvidence {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.acquisition_method, "dataset acquisition method")?;
        validate_id(&self.legal_basis, "dataset legal basis")?;
        if self.collected_from_unix_seconds >= self.collected_to_unix_seconds {
            return Err(ContractError::Invalid(
                "dataset collection window is empty or inverted".to_string(),
            ));
        }
        self.signed.validate()?;
        if self.signed.payload_sha256 != self.payload_sha256() {
            return Err(ContractError::Corrupt(
                "provenance signature payload mismatch".to_string(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn payload_sha256(&self) -> Digest32 {
        framed_digest(
            b"hepta:intelligence:p1.1c3:provenance-evidence:v1",
            &[
                self.dataset_sha256.as_bytes(),
                self.source_manifest_sha256.as_bytes(),
                self.acquisition_method.as_bytes(),
                self.legal_basis.as_bytes(),
                &self.collected_from_unix_seconds.to_be_bytes(),
                &self.collected_to_unix_seconds.to_be_bytes(),
                &[u8::from(self.human_review_source)],
                &[u8::from(self.fixture_only)],
            ],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivacyEvidence {
    pub dataset_sha256: Digest32,
    pub scanner_id: String,
    pub secret_scan_sha256: Digest32,
    pub secret_scan_passed: bool,
    pub pii_assessment_sha256: Digest32,
    pub pii_assessment_passed: bool,
    pub redaction_manifest_sha256: Option<Digest32>,
    pub redaction_complete: bool,
    pub residual_risk_accepted: bool,
    pub fixture_only: bool,
    pub signed: SignedDigest,
}

impl PrivacyEvidence {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.scanner_id, "privacy scanner ID")?;
        if self.redaction_complete != self.redaction_manifest_sha256.is_some() {
            return Err(ContractError::Invalid(
                "privacy redaction completion and manifest state disagree".to_string(),
            ));
        }
        self.signed.validate()?;
        if self.signed.payload_sha256 != self.payload_sha256() {
            return Err(ContractError::Corrupt(
                "privacy signature payload mismatch".to_string(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn payload_sha256(&self) -> Digest32 {
        let redaction = self
            .redaction_manifest_sha256
            .unwrap_or_else(|| Digest32::for_bytes(b"no-redaction-manifest"));
        framed_digest(
            b"hepta:intelligence:p1.1c3:privacy-evidence:v1",
            &[
                self.dataset_sha256.as_bytes(),
                self.scanner_id.as_bytes(),
                self.secret_scan_sha256.as_bytes(),
                &[u8::from(self.secret_scan_passed)],
                self.pii_assessment_sha256.as_bytes(),
                &[u8::from(self.pii_assessment_passed)],
                redaction.as_bytes(),
                &[u8::from(self.redaction_complete)],
                &[u8::from(self.residual_risk_accepted)],
                &[u8::from(self.fixture_only)],
            ],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorApprovalEvidence {
    pub subject_sha256: Digest32,
    pub scope: String,
    pub approved: bool,
    pub approved_at_unix_seconds: u64,
    pub expires_at_unix_seconds: u64,
    pub fixture_only: bool,
    pub signed: SignedDigest,
}

impl OperatorApprovalEvidence {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.scope, "operator approval scope")?;
        if self.approved_at_unix_seconds >= self.expires_at_unix_seconds {
            return Err(ContractError::Invalid(
                "operator approval validity window is empty or inverted".to_string(),
            ));
        }
        self.signed.validate()?;
        if self.signed.payload_sha256 != self.payload_sha256() {
            return Err(ContractError::Corrupt(
                "operator approval signature payload mismatch".to_string(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn payload_sha256(&self) -> Digest32 {
        framed_digest(
            b"hepta:intelligence:p1.1c3:operator-approval:v1",
            &[
                self.subject_sha256.as_bytes(),
                self.scope.as_bytes(),
                &[u8::from(self.approved)],
                &self.approved_at_unix_seconds.to_be_bytes(),
                &self.expires_at_unix_seconds.to_be_bytes(),
                &[u8::from(self.fixture_only)],
            ],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntakePolicy {
    pub policy_id: String,
    pub expected_p1c_commit: String,
    pub expected_p1c_tree: String,
    pub expected_p1c1_commit: String,
    pub expected_p1c1_tree: String,
    pub expected_trust_store_sha256: Digest32,
    pub allowed_spdx_license_ids: Vec<String>,
    pub minimum_items: u32,
    pub minimum_locales: u32,
    pub require_external_signers: bool,
    pub require_non_fixture: bool,
    pub require_derivative_rights: bool,
    pub require_operator_approval: bool,
    pub policy_sha256: Digest32,
}

impl IntakePolicy {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.policy_id, "corpus intake policy ID")?;
        validate_git_oid(&self.expected_p1c_commit, "expected P1.1c commit")?;
        validate_git_oid(&self.expected_p1c_tree, "expected P1.1c tree")?;
        validate_git_oid(&self.expected_p1c1_commit, "expected P1.1c.1 commit")?;
        validate_git_oid(&self.expected_p1c1_tree, "expected P1.1c.1 tree")?;
        if self.allowed_spdx_license_ids.is_empty()
            || self.allowed_spdx_license_ids.len() > MAX_ALLOWED_LICENSES
            || self.minimum_items == 0
            || self.minimum_locales == 0
        {
            return Err(ContractError::Limit(
                "corpus intake policy licenses and coverage minima are outside bounds"
                    .to_string(),
            ));
        }
        let mut previous: Option<&str> = None;
        for license in &self.allowed_spdx_license_ids {
            validate_id(license, "allowed SPDX license ID")?;
            if previous.is_some_and(|value| value >= license.as_str()) {
                return Err(ContractError::Invalid(
                    "allowed SPDX license IDs must be strictly sorted and unique"
                        .to_string(),
                ));
            }
            previous = Some(license);
        }
        if self.policy_sha256 != intake_policy_digest(self) {
            return Err(ContractError::Corrupt(
                "corpus intake policy digest mismatch".to_string(),
            ));
        }
        Ok(())
    }
}

pub struct CorpusIntakeRequest<'a> {
    pub expected_dataset_sha256: Digest32,
    pub item_count: u32,
    pub locale_count: u32,
    pub p1c_qualification: Option<&'a VerifiedQualificationReceipt>,
    pub p1c1_qualification: Option<&'a VerifiedQualificationReceipt>,
    pub acceptance: Option<&'a AcceptanceReceipt>,
    pub review_trust: Option<&'a ReviewTrustReceipt>,
    pub license: Option<&'a LicenseEvidence>,
    pub provenance: Option<&'a ProvenanceEvidence>,
    pub privacy: Option<&'a PrivacyEvidence>,
    pub operator_approval: Option<&'a OperatorApprovalEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustedCorpusIntakeReceipt {
    pub schema: String,
    pub status: String,
    pub policy_id: String,
    pub dataset_sha256: Digest32,
    pub item_count: u32,
    pub locale_count: u32,
    pub p1c_qualification_sha256: Option<Digest32>,
    pub p1c1_qualification_sha256: Option<Digest32>,
    pub acceptance_sha256: Option<Digest32>,
    pub review_trust_sha256: Option<Digest32>,
    pub license_payload_sha256: Option<Digest32>,
    pub provenance_payload_sha256: Option<Digest32>,
    pub privacy_payload_sha256: Option<Digest32>,
    pub operator_payload_sha256: Option<Digest32>,
    pub external_evidence_complete: bool,
    pub mechanically_accepted: bool,
    pub blocked_reasons: Vec<String>,
    pub product_workspace_member: bool,
    pub product_module_registered: bool,
    pub runtime_wired: bool,
    pub default_recall_changed: bool,
    pub federation_recall_changed: bool,
    pub context_attachment: bool,
    pub physical_send: bool,
    pub external_effects: bool,
    pub production_authority: bool,
    pub efficacy_validation: bool,
    pub efficacy_claim: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub callers_ratchet: bool,
    pub receipt_sha256: Digest32,
}

impl TrustedCorpusIntakeReceipt {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.schema != INTAKE_SCHEMA {
            return Err(ContractError::Corrupt(
                "trusted corpus intake receipt schema mismatch".to_string(),
            ));
        }
        let expected_status = if self.mechanically_accepted {
            "PASS_P1_1C3_TRUSTED_CORPUS_INTAKE"
        } else {
            "BLOCKED_P1_1C3_TRUSTED_CORPUS_INTAKE"
        };
        if self.status != expected_status
            || self.mechanically_accepted != self.blocked_reasons.is_empty()
            || self.external_evidence_complete != self.mechanically_accepted
            || (!self.mechanically_accepted && self.blocked_reasons.is_empty())
            || self
                .blocked_reasons
                .windows(2)
                .any(|pair| pair[0].as_str() >= pair[1].as_str())
        {
            return Err(ContractError::Corrupt(
                "trusted corpus intake status and blocker state are noncanonical"
                    .to_string(),
            ));
        }
        if self.product_workspace_member
            || self.product_module_registered
            || self.runtime_wired
            || self.default_recall_changed
            || self.federation_recall_changed
            || self.context_attachment
            || self.physical_send
            || self.external_effects
            || self.production_authority
            || self.efficacy_validation
            || self.efficacy_claim
            || self.operator_acceptance
            || self.promotion
            || self.callers_ratchet
        {
            return Err(ContractError::Corrupt(
                "trusted corpus intake receipt crosses its offline authority boundary"
                    .to_string(),
            ));
        }
        if self.receipt_sha256 != trusted_corpus_intake_receipt_digest(self) {
            return Err(ContractError::Corrupt(
                "trusted corpus intake receipt digest mismatch".to_string(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn to_json_pretty(&self) -> String {
        let blockers = self
            .blocked_reasons
            .iter()
            .map(|reason| format!("\"{}\"", json_escape(reason)))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            concat!(
                "{{\n",
                "  \"schema\": \"{}\",\n",
                "  \"status\": \"{}\",\n",
                "  \"policy_id\": \"{}\",\n",
                "  \"dataset_sha256\": \"{}\",\n",
                "  \"item_count\": {},\n",
                "  \"locale_count\": {},\n",
                "  \"external_evidence_complete\": {},\n",
                "  \"mechanically_accepted\": {},\n",
                "  \"blocked_reasons\": [{}],\n",
                "  \"runtime_wired\": {},\n",
                "  \"production_authority\": {},\n",
                "  \"efficacy_validation\": {},\n",
                "  \"efficacy_claim\": {},\n",
                "  \"promotion\": {},\n",
                "  \"receipt_sha256\": \"{}\"\n",
                "}}\n"
            ),
            self.schema,
            self.status,
            json_escape(&self.policy_id),
            self.dataset_sha256,
            self.item_count,
            self.locale_count,
            self.external_evidence_complete,
            self.mechanically_accepted,
            blockers,
            self.runtime_wired,
            self.production_authority,
            self.efficacy_validation,
            self.efficacy_claim,
            self.promotion,
            self.receipt_sha256,
        )
    }
}

pub fn evaluate_corpus_intake(
    request: &CorpusIntakeRequest<'_>,
    policy: &IntakePolicy,
    trust_store: &TrustStore,
    now_unix_seconds: u64,
) -> Result<TrustedCorpusIntakeReceipt, ContractError> {
    policy.validate()?;
    trust_store.validate()?;
    if trust_store.store_sha256 != policy.expected_trust_store_sha256 {
        return Err(ContractError::Invalid(
            "corpus intake trust store does not match policy".to_string(),
        ));
    }
    let mut blockers = BTreeSet::new();
    if request.item_count < policy.minimum_items {
        blockers.insert("coverage.items_below_policy".to_string());
    }
    if request.locale_count < policy.minimum_locales {
        blockers.insert("coverage.locales_below_policy".to_string());
    }

    let p1c_qualification_sha256 = match request.p1c_qualification {
        Some(receipt) => {
            receipt.validate()?;
            if receipt.source_commit != policy.expected_p1c_commit
                || receipt.source_tree != policy.expected_p1c_tree
            {
                blockers.insert("qualification.p1c_exact_source_mismatch".to_string());
            }
            if policy.require_external_signers && !receipt.external_attested {
                blockers.insert("qualification.p1c_external_attestation_missing".to_string());
            }
            Some(receipt.receipt_sha256)
        }
        None => {
            blockers.insert("qualification.p1c_missing".to_string());
            None
        }
    };
    let p1c1_qualification_sha256 = match request.p1c1_qualification {
        Some(receipt) => {
            receipt.validate()?;
            if receipt.source_commit != policy.expected_p1c1_commit
                || receipt.source_tree != policy.expected_p1c1_tree
            {
                blockers.insert("qualification.p1c1_exact_source_mismatch".to_string());
            }
            if policy.require_external_signers && !receipt.external_attested {
                blockers.insert("qualification.p1c1_external_attestation_missing".to_string());
            }
            Some(receipt.receipt_sha256)
        }
        None => {
            blockers.insert("qualification.p1c1_missing".to_string());
            None
        }
    };

    let acceptance_sha256 = match request.acceptance {
        Some(receipt) => {
            receipt
                .validate()
                .map_err(|error| ContractError::Invalid(error.to_string()))?;
            let dataset = p1_digest(receipt.reviewed_corpus_sha256)?;
            if dataset != request.expected_dataset_sha256 {
                blockers.insert("acceptance.dataset_digest_mismatch".to_string());
            }
            if !receipt.reviewed_corpus_accepted
                || !receipt.corpus_reviewed
                || !receipt.human_review_attested
            {
                blockers.insert("acceptance.reviewed_corpus_not_accepted".to_string());
            }
            if receipt.source_p1_1c_commit != policy.expected_p1c_commit {
                blockers.insert("acceptance.p1c_commit_mismatch".to_string());
            }
            if receipt.item_count != request.item_count || receipt.locale_count != request.locale_count {
                blockers.insert("acceptance.coverage_mismatch".to_string());
            }
            Some(p1_digest(receipt.receipt_sha256)?)
        }
        None => {
            blockers.insert("acceptance.receipt_missing".to_string());
            None
        }
    };

    let review_trust_sha256 = match request.review_trust {
        Some(receipt) => {
            receipt.validate()?;
            if policy.require_external_signers && !receipt.all_signers_external_attested {
                blockers.insert("review.external_attestation_missing".to_string());
            }
            if !receipt.reviewer_affiliations_independent
                || !receipt.adjudicator_affiliations_independent
            {
                blockers.insert("review.affiliation_independence_failed".to_string());
            }
            if receipt.item_count != request.item_count {
                blockers.insert("review.item_count_mismatch".to_string());
            }
            if let Some(acceptance) = request.acceptance {
                if receipt.review_batch_sha256 != p1_digest(acceptance.review_batch_sha256)?
                    || receipt.adjudication_batch_sha256
                        != p1_digest(acceptance.adjudication_batch_sha256)?
                    || receipt.reviewer_set_sha256 != p1_digest(acceptance.reviewer_set_sha256)?
                {
                    blockers.insert("review.acceptance_digest_mismatch".to_string());
                }
            }
            Some(receipt.receipt_sha256)
        }
        None => {
            blockers.insert("review.trust_receipt_missing".to_string());
            None
        }
    };

    let license_payload_sha256 = match request.license {
        Some(evidence) => {
            evidence.validate()?;
            verify_evidence_signature(
                trust_store,
                &evidence.signed,
                TrustRole::LicenseApprover,
                now_unix_seconds,
                policy.require_external_signers,
            )?;
            if evidence.dataset_sha256 != request.expected_dataset_sha256 {
                blockers.insert("license.dataset_digest_mismatch".to_string());
            }
            if policy
                .allowed_spdx_license_ids
                .binary_search(&evidence.spdx_license_id)
                .is_err()
            {
                blockers.insert("license.not_allowed".to_string());
            }
            if !evidence.permits_offline_evaluation || !evidence.permits_storage {
                blockers.insert("license.required_rights_missing".to_string());
            }
            if policy.require_derivative_rights && !evidence.permits_derivatives {
                blockers.insert("license.derivative_rights_missing".to_string());
            }
            if policy.require_non_fixture && evidence.fixture_only {
                blockers.insert("license.fixture_evidence_rejected".to_string());
            }
            Some(evidence.payload_sha256())
        }
        None => {
            blockers.insert("license.evidence_missing".to_string());
            None
        }
    };

    let provenance_payload_sha256 = match request.provenance {
        Some(evidence) => {
            evidence.validate()?;
            verify_evidence_signature(
                trust_store,
                &evidence.signed,
                TrustRole::ProvenanceApprover,
                now_unix_seconds,
                policy.require_external_signers,
            )?;
            if evidence.dataset_sha256 != request.expected_dataset_sha256 {
                blockers.insert("provenance.dataset_digest_mismatch".to_string());
            }
            if !evidence.human_review_source {
                blockers.insert("provenance.human_review_source_missing".to_string());
            }
            if policy.require_non_fixture && evidence.fixture_only {
                blockers.insert("provenance.fixture_evidence_rejected".to_string());
            }
            Some(evidence.payload_sha256())
        }
        None => {
            blockers.insert("provenance.evidence_missing".to_string());
            None
        }
    };

    let privacy_payload_sha256 = match request.privacy {
        Some(evidence) => {
            evidence.validate()?;
            verify_evidence_signature(
                trust_store,
                &evidence.signed,
                TrustRole::PrivacyApprover,
                now_unix_seconds,
                policy.require_external_signers,
            )?;
            if evidence.dataset_sha256 != request.expected_dataset_sha256 {
                blockers.insert("privacy.dataset_digest_mismatch".to_string());
            }
            if !evidence.secret_scan_passed || !evidence.pii_assessment_passed {
                blockers.insert("privacy.scan_or_assessment_failed".to_string());
            }
            if !evidence.redaction_complete || !evidence.residual_risk_accepted {
                blockers.insert("privacy.redaction_or_residual_risk_incomplete".to_string());
            }
            if policy.require_non_fixture && evidence.fixture_only {
                blockers.insert("privacy.fixture_evidence_rejected".to_string());
            }
            Some(evidence.payload_sha256())
        }
        None => {
            blockers.insert("privacy.evidence_missing".to_string());
            None
        }
    };

    let subject_sha256 = intake_subject_digest(
        request.expected_dataset_sha256,
        policy.policy_sha256,
        p1c_qualification_sha256,
        p1c1_qualification_sha256,
        acceptance_sha256,
        review_trust_sha256,
        license_payload_sha256,
        provenance_payload_sha256,
        privacy_payload_sha256,
    );
    let operator_payload_sha256 = match request.operator_approval {
        Some(evidence) => {
            evidence.validate()?;
            verify_evidence_signature(
                trust_store,
                &evidence.signed,
                TrustRole::Operator,
                now_unix_seconds,
                policy.require_external_signers,
            )?;
            if evidence.subject_sha256 != subject_sha256
                || evidence.scope != "offline_corpus_intake"
                || !evidence.approved
                || now_unix_seconds < evidence.approved_at_unix_seconds
                || now_unix_seconds >= evidence.expires_at_unix_seconds
            {
                blockers.insert("operator.approval_binding_invalid".to_string());
            }
            if policy.require_non_fixture && evidence.fixture_only {
                blockers.insert("operator.fixture_approval_rejected".to_string());
            }
            Some(evidence.payload_sha256())
        }
        None if policy.require_operator_approval => {
            blockers.insert("operator.approval_missing".to_string());
            None
        }
        None => None,
    };

    let blocked_reasons = blockers.into_iter().collect::<Vec<_>>();
    let mechanically_accepted = blocked_reasons.is_empty();
    let mut receipt = TrustedCorpusIntakeReceipt {
        schema: INTAKE_SCHEMA.to_string(),
        status: if mechanically_accepted {
            "PASS_P1_1C3_TRUSTED_CORPUS_INTAKE"
        } else {
            "BLOCKED_P1_1C3_TRUSTED_CORPUS_INTAKE"
        }
        .to_string(),
        policy_id: policy.policy_id.clone(),
        dataset_sha256: request.expected_dataset_sha256,
        item_count: request.item_count,
        locale_count: request.locale_count,
        p1c_qualification_sha256,
        p1c1_qualification_sha256,
        acceptance_sha256,
        review_trust_sha256,
        license_payload_sha256,
        provenance_payload_sha256,
        privacy_payload_sha256,
        operator_payload_sha256,
        external_evidence_complete: mechanically_accepted,
        mechanically_accepted,
        blocked_reasons,
        product_workspace_member: false,
        product_module_registered: false,
        runtime_wired: false,
        default_recall_changed: false,
        federation_recall_changed: false,
        context_attachment: false,
        physical_send: false,
        external_effects: false,
        production_authority: false,
        efficacy_validation: false,
        efficacy_claim: false,
        operator_acceptance: false,
        promotion: false,
        callers_ratchet: false,
        receipt_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    receipt.receipt_sha256 = trusted_corpus_intake_receipt_digest(&receipt);
    receipt.validate()?;
    Ok(receipt)
}

#[must_use]
pub fn intake_policy_digest(policy: &IntakePolicy) -> Digest32 {
    let mut licenses = Vec::new();
    for license in &policy.allowed_spdx_license_ids {
        licenses.extend_from_slice(&u64::try_from(license.len()).unwrap_or(u64::MAX).to_be_bytes());
        licenses.extend_from_slice(license.as_bytes());
    }
    framed_digest(
        b"hepta:intelligence:p1.1c3:intake-policy:v1",
        &[
            policy.policy_id.as_bytes(),
            policy.expected_p1c_commit.as_bytes(),
            policy.expected_p1c_tree.as_bytes(),
            policy.expected_p1c1_commit.as_bytes(),
            policy.expected_p1c1_tree.as_bytes(),
            policy.expected_trust_store_sha256.as_bytes(),
            &licenses,
            &policy.minimum_items.to_be_bytes(),
            &policy.minimum_locales.to_be_bytes(),
            &[u8::from(policy.require_external_signers)],
            &[u8::from(policy.require_non_fixture)],
            &[u8::from(policy.require_derivative_rights)],
            &[u8::from(policy.require_operator_approval)],
        ],
    )
}

#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn intake_subject_digest(
    dataset_sha256: Digest32,
    policy_sha256: Digest32,
    p1c_qualification_sha256: Option<Digest32>,
    p1c1_qualification_sha256: Option<Digest32>,
    acceptance_sha256: Option<Digest32>,
    review_trust_sha256: Option<Digest32>,
    license_payload_sha256: Option<Digest32>,
    provenance_payload_sha256: Option<Digest32>,
    privacy_payload_sha256: Option<Digest32>,
) -> Digest32 {
    let absent = Digest32::for_bytes(b"absent");
    framed_digest(
        b"hepta:intelligence:p1.1c3:intake-subject:v1",
        &[
            dataset_sha256.as_bytes(),
            policy_sha256.as_bytes(),
            p1c_qualification_sha256.unwrap_or(absent).as_bytes(),
            p1c1_qualification_sha256.unwrap_or(absent).as_bytes(),
            acceptance_sha256.unwrap_or(absent).as_bytes(),
            review_trust_sha256.unwrap_or(absent).as_bytes(),
            license_payload_sha256.unwrap_or(absent).as_bytes(),
            provenance_payload_sha256.unwrap_or(absent).as_bytes(),
            privacy_payload_sha256.unwrap_or(absent).as_bytes(),
        ],
    )
}

fn verify_evidence_signature(
    trust_store: &TrustStore,
    signed: &SignedDigest,
    role: TrustRole,
    now_unix_seconds: u64,
    require_external: bool,
) -> Result<VerifiedSignatureReceipt, ContractError> {
    verify_signed_digest(
        trust_store,
        signed,
        role,
        None,
        now_unix_seconds,
        require_external,
    )
}

fn p1_digest(value: hepta_memory_p1_1c1_qualification::Digest32) -> Result<Digest32, ContractError> {
    Digest32::from_hex(&value.to_string())
}

fn trusted_corpus_intake_receipt_digest(receipt: &TrustedCorpusIntakeReceipt) -> Digest32 {
    let absent = Digest32::for_bytes(b"absent");
    let blockers = receipt.blocked_reasons.join("\n");
    framed_digest(
        b"hepta:intelligence:p1.1c3:trusted-corpus-intake-receipt:v1",
        &[
            receipt.schema.as_bytes(),
            receipt.status.as_bytes(),
            receipt.policy_id.as_bytes(),
            receipt.dataset_sha256.as_bytes(),
            &receipt.item_count.to_be_bytes(),
            &receipt.locale_count.to_be_bytes(),
            receipt.p1c_qualification_sha256.unwrap_or(absent).as_bytes(),
            receipt.p1c1_qualification_sha256.unwrap_or(absent).as_bytes(),
            receipt.acceptance_sha256.unwrap_or(absent).as_bytes(),
            receipt.review_trust_sha256.unwrap_or(absent).as_bytes(),
            receipt.license_payload_sha256.unwrap_or(absent).as_bytes(),
            receipt.provenance_payload_sha256.unwrap_or(absent).as_bytes(),
            receipt.privacy_payload_sha256.unwrap_or(absent).as_bytes(),
            receipt.operator_payload_sha256.unwrap_or(absent).as_bytes(),
            &[u8::from(receipt.external_evidence_complete)],
            &[u8::from(receipt.mechanically_accepted)],
            blockers.as_bytes(),
            &[u8::from(receipt.product_workspace_member)],
            &[u8::from(receipt.product_module_registered)],
            &[u8::from(receipt.runtime_wired)],
            &[u8::from(receipt.default_recall_changed)],
            &[u8::from(receipt.federation_recall_changed)],
            &[u8::from(receipt.context_attachment)],
            &[u8::from(receipt.physical_send)],
            &[u8::from(receipt.external_effects)],
            &[u8::from(receipt.production_authority)],
            &[u8::from(receipt.efficacy_validation)],
            &[u8::from(receipt.efficacy_claim)],
            &[u8::from(receipt.operator_acceptance)],
            &[u8::from(receipt.promotion)],
            &[u8::from(receipt.callers_ratchet)],
        ],
    )
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

use crate::agreement::labels_match_any;
use crate::{
    AgreementMetrics, ContractError, CorpusProvenance, Digest32,
    P1_1C1_CALLERS_RATCHET, P1_1C1_CONTEXT_ATTACHMENT,
    P1_1C1_DEFAULT_RECALL_CHANGED, P1_1C1_EFFICACY_CLAIM,
    P1_1C1_EFFICACY_VALIDATION, P1_1C1_EXTERNAL_EFFECTS,
    P1_1C1_FEDERATION_RECALL_CHANGED, P1_1C1_MODEL_DOWNLOAD,
    P1_1C1_NETWORK_ACCESS, P1_1C1_OPERATOR_ACCEPTANCE,
    P1_1C1_PHYSICAL_SEND, P1_1C1_PRODUCT_MODULE_REGISTERED,
    P1_1C1_PRODUCT_WORKSPACE_MEMBER, P1_1C1_PRODUCTION_AUTHORITY,
    P1_1C1_PROMOTION, P1_1C1_RUNTIME_WIRED, P1_1C1_SCHEMA_VERSION,
    P1_1C1_SOURCE_QUALIFIED, PrivacyDecision, ReviewBatch, ReviewTuple,
    SCORE_SCALE_PPM, framed_digest, usize_to_u32, validate_commit_oid,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyState {
    pub p1_1c_source_commit: String,
    pub p1_1c_source_qualified: bool,
    pub seed_pipeline_reproducible: bool,
}

impl DependencyState {
    pub fn blocked_seed(source_commit: &str) -> Result<Self, ContractError> {
        let state = Self {
            p1_1c_source_commit: source_commit.to_string(),
            p1_1c_source_qualified: false,
            seed_pipeline_reproducible: false,
        };
        state.validate()?;
        Ok(state)
    }

    pub fn qualified(source_commit: &str) -> Result<Self, ContractError> {
        let state = Self {
            p1_1c_source_commit: source_commit.to_string(),
            p1_1c_source_qualified: true,
            seed_pipeline_reproducible: true,
        };
        state.validate()?;
        Ok(state)
    }

    pub fn validate(&self) -> Result<(), ContractError> {
        validate_commit_oid(&self.p1_1c_source_commit)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptancePolicy {
    pub minimum_items: u32,
    pub minimum_locales: u32,
    pub minimum_exact_tuple_agreement_ppm: u32,
    pub minimum_weighted_relevance_kappa_ppm: i32,
    pub require_zero_unresolved: bool,
    pub require_zero_privacy_blocks: bool,
}

impl Default for AcceptancePolicy {
    fn default() -> Self {
        Self {
            minimum_items: 8,
            minimum_locales: 8,
            minimum_exact_tuple_agreement_ppm: 700_000,
            minimum_weighted_relevance_kappa_ppm: 600_000,
            require_zero_unresolved: true,
            require_zero_privacy_blocks: true,
        }
    }
}

impl AcceptancePolicy {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.minimum_items == 0 || self.minimum_locales == 0 {
            return Err(ContractError::Invalid(
                "acceptance policy item and locale minima must be positive"
                    .to_string(),
            ));
        }
        if self.minimum_exact_tuple_agreement_ppm > SCORE_SCALE_PPM {
            return Err(ContractError::Invalid(
                "exact agreement threshold exceeds one million PPM".to_string(),
            ));
        }
        if !(-1_000_000..=1_000_000)
            .contains(&self.minimum_weighted_relevance_kappa_ppm)
        {
            return Err(ContractError::Invalid(
                "weighted kappa threshold is outside -1,000,000..=1,000,000"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedItemReceipt {
    pub item_id_sha256: Digest32,
    pub locale: String,
    pub final_labels: ReviewTuple,
    pub resolved: bool,
    pub accepted: bool,
    pub review_pair_sha256: Digest32,
    pub adjudication_sha256: Option<Digest32>,
    pub redaction_receipt_sha256: Option<Digest32>,
    pub item_receipt_sha256: Digest32,
}

impl ResolvedItemReceipt {
    pub fn validate(&self) -> Result<(), ContractError> {
        self.final_labels.validate()?;
        if self.accepted && !self.resolved {
            return Err(ContractError::Corrupt(
                "unresolved review item cannot be accepted".to_string(),
            ));
        }
        match self.final_labels.privacy {
            PrivacyDecision::Allow => {
                if self.redaction_receipt_sha256.is_some() {
                    return Err(ContractError::Corrupt(
                        "privacy allow item cannot carry a redaction receipt"
                            .to_string(),
                    ));
                }
            }
            PrivacyDecision::Redact => {
                if self.accepted && self.redaction_receipt_sha256.is_none() {
                    return Err(ContractError::Corrupt(
                        "accepted redacted item requires a redaction receipt"
                            .to_string(),
                    ));
                }
            }
            PrivacyDecision::Block => {
                if self.accepted {
                    return Err(ContractError::Corrupt(
                        "privacy-blocked item cannot be accepted".to_string(),
                    ));
                }
            }
        }
        if self.item_receipt_sha256 != resolved_item_digest(self) {
            return Err(ContractError::Corrupt(
                "resolved item receipt digest mismatch".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptanceReceipt {
    pub schema_version: u32,
    pub namespace: String,
    pub status: String,
    pub corpus_provenance: CorpusProvenance,
    pub header_reviewed: bool,
    pub human_review_attested: bool,
    pub source_p1_1c_commit: String,
    pub dependency_source_qualified: bool,
    pub dependency_seed_pipeline_reproducible: bool,
    pub dependency_commit_matches: bool,
    pub review_pipeline_validated: bool,
    pub reviewed_corpus_accepted: bool,
    pub corpus_reviewed: bool,
    pub item_count: u32,
    pub locale_count: u32,
    pub review_count: u32,
    pub reviewer_count: u32,
    pub adjudication_count: u32,
    pub unresolved_count: u32,
    pub accepted_item_count: u32,
    pub privacy_block_count: u32,
    pub privacy_redact_count: u32,
    pub agreement: AgreementMetrics,
    pub locales: Vec<String>,
    pub blocked_reasons: Vec<String>,
    pub reviewer_set_sha256: Digest32,
    pub review_batch_sha256: Digest32,
    pub adjudication_batch_sha256: Digest32,
    pub reviewed_corpus_sha256: Digest32,
    pub items: Vec<ResolvedItemReceipt>,
    pub source_contract_qualified: bool,
    pub efficacy_validation: bool,
    pub efficacy_claim: bool,
    pub product_workspace_member: bool,
    pub product_module_registered: bool,
    pub runtime_wired: bool,
    pub default_recall_changed: bool,
    pub federation_recall_changed: bool,
    pub context_attachment: bool,
    pub physical_send: bool,
    pub network_access: bool,
    pub model_download: bool,
    pub external_effects: bool,
    pub production_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub callers_ratchet: bool,
    pub receipt_sha256: Digest32,
}

impl AcceptanceReceipt {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.schema_version != P1_1C1_SCHEMA_VERSION
            || self.namespace
                != "hepta_intelligence_p1_1c1_reviewed_corpus_acceptance_v1"
            || !self.review_pipeline_validated
        {
            return Err(ContractError::Corrupt(
                "review acceptance receipt schema or pipeline state is invalid"
                    .to_string(),
            ));
        }
        validate_commit_oid(&self.source_p1_1c_commit)?;
        let counted_accepted = u32::try_from(
            self.items.iter().filter(|item| item.accepted).count(),
        )
        .map_err(|_| ContractError::Overflow)?;
        if usize::try_from(self.item_count).ok() != Some(self.items.len())
            || self.review_count != self.item_count.saturating_mul(2)
            || self.accepted_item_count != counted_accepted
        {
            return Err(ContractError::Corrupt(
                "review acceptance receipt counts are inconsistent".to_string(),
            ));
        }
        for item in &self.items {
            item.validate()?;
        }
        if self.reviewed_corpus_accepted != self.blocked_reasons.is_empty()
            || self.corpus_reviewed != self.reviewed_corpus_accepted
        {
            return Err(ContractError::Corrupt(
                "reviewed-corpus acceptance and blocker state disagree"
                    .to_string(),
            ));
        }
        if self.reviewed_corpus_accepted
            && (self.corpus_provenance != CorpusProvenance::HumanReviewedV1
                || !self.header_reviewed
                || !self.human_review_attested
                || !self.dependency_source_qualified
                || !self.dependency_seed_pipeline_reproducible
                || !self.dependency_commit_matches)
        {
            return Err(ContractError::Corrupt(
                "accepted corpus lacks reviewed provenance or qualified dependency"
                    .to_string(),
            ));
        }
        if self.source_contract_qualified
            || self.efficacy_validation
            || self.efficacy_claim
            || self.product_workspace_member
            || self.product_module_registered
            || self.runtime_wired
            || self.default_recall_changed
            || self.federation_recall_changed
            || self.context_attachment
            || self.physical_send
            || self.network_access
            || self.model_download
            || self.external_effects
            || self.production_authority
            || self.operator_acceptance
            || self.promotion
            || self.callers_ratchet
        {
            return Err(ContractError::Corrupt(
                "review acceptance receipt crosses the source-only authority boundary"
                    .to_string(),
            ));
        }
        if self.receipt_sha256 != acceptance_receipt_digest(self) {
            return Err(ContractError::Corrupt(
                "review acceptance receipt digest mismatch".to_string(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn to_json_pretty(&self) -> String {
        let locales = self
            .locales
            .iter()
            .map(|locale| format!("\"{}\"", json_escape(locale)))
            .collect::<Vec<_>>()
            .join(", ");
        let blockers = self
            .blocked_reasons
            .iter()
            .map(|reason| format!("\"{}\"", json_escape(reason)))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            concat!(
                "{{\n",
                "  \"schema\": \"hepta.intelligence.p1_1c1.reviewed-corpus-acceptance.v1\",\n",
                "  \"status\": \"{}\",\n",
                "  \"corpus_provenance\": \"{}\",\n",
                "  \"header_reviewed\": {},\n",
                "  \"human_review_attested\": {},\n",
                "  \"source_p1_1c_commit\": \"{}\",\n",
                "  \"dependency_source_qualified\": {},\n",
                "  \"dependency_seed_pipeline_reproducible\": {},\n",
                "  \"dependency_commit_matches\": {},\n",
                "  \"review_pipeline_validated\": {},\n",
                "  \"reviewed_corpus_accepted\": {},\n",
                "  \"corpus_reviewed\": {},\n",
                "  \"item_count\": {},\n",
                "  \"locale_count\": {},\n",
                "  \"review_count\": {},\n",
                "  \"reviewer_count\": {},\n",
                "  \"adjudication_count\": {},\n",
                "  \"unresolved_count\": {},\n",
                "  \"accepted_item_count\": {},\n",
                "  \"privacy_block_count\": {},\n",
                "  \"privacy_redact_count\": {},\n",
                "  \"agreement\": {{\n",
                "    \"exact_tuple_agreement_ppm\": {},\n",
                "    \"relevance_agreement_ppm\": {},\n",
                "    \"citation_agreement_ppm\": {},\n",
                "    \"contradiction_agreement_ppm\": {},\n",
                "    \"privacy_agreement_ppm\": {},\n",
                "    \"weighted_relevance_kappa_ppm\": {}\n",
                "  }},\n",
                "  \"locales\": [{}],\n",
                "  \"blocked_reasons\": [{}],\n",
                "  \"reviewer_set_sha256\": \"{}\",\n",
                "  \"review_batch_sha256\": \"{}\",\n",
                "  \"adjudication_batch_sha256\": \"{}\",\n",
                "  \"reviewed_corpus_sha256\": \"{}\",\n",
                "  \"source_contract_qualified\": {},\n",
                "  \"efficacy_validation\": {},\n",
                "  \"efficacy_claim\": {},\n",
                "  \"product_workspace_member\": {},\n",
                "  \"product_module_registered\": {},\n",
                "  \"runtime_wired\": {},\n",
                "  \"default_recall_changed\": {},\n",
                "  \"federation_recall_changed\": {},\n",
                "  \"context_attachment\": {},\n",
                "  \"physical_send\": {},\n",
                "  \"network_access\": {},\n",
                "  \"model_download\": {},\n",
                "  \"external_effects\": {},\n",
                "  \"production_authority\": {},\n",
                "  \"operator_acceptance\": {},\n",
                "  \"promotion\": {},\n",
                "  \"callers_ratchet\": {},\n",
                "  \"receipt_sha256\": \"{}\"\n",
                "}}\n"
            ),
            json_escape(&self.status),
            self.corpus_provenance.as_str(),
            self.header_reviewed,
            self.human_review_attested,
            self.source_p1_1c_commit,
            self.dependency_source_qualified,
            self.dependency_seed_pipeline_reproducible,
            self.dependency_commit_matches,
            self.review_pipeline_validated,
            self.reviewed_corpus_accepted,
            self.corpus_reviewed,
            self.item_count,
            self.locale_count,
            self.review_count,
            self.reviewer_count,
            self.adjudication_count,
            self.unresolved_count,
            self.accepted_item_count,
            self.privacy_block_count,
            self.privacy_redact_count,
            self.agreement.exact_tuple_agreement_ppm,
            self.agreement.relevance_agreement_ppm,
            self.agreement.citation_agreement_ppm,
            self.agreement.contradiction_agreement_ppm,
            self.agreement.privacy_agreement_ppm,
            self.agreement.weighted_relevance_kappa_ppm,
            locales,
            blockers,
            self.reviewer_set_sha256,
            self.review_batch_sha256,
            self.adjudication_batch_sha256,
            self.reviewed_corpus_sha256,
            self.source_contract_qualified,
            self.efficacy_validation,
            self.efficacy_claim,
            self.product_workspace_member,
            self.product_module_registered,
            self.runtime_wired,
            self.default_recall_changed,
            self.federation_recall_changed,
            self.context_attachment,
            self.physical_send,
            self.network_access,
            self.model_download,
            self.external_effects,
            self.production_authority,
            self.operator_acceptance,
            self.promotion,
            self.callers_ratchet,
            self.receipt_sha256,
        )
    }
}

pub fn evaluate_review_batch(
    batch: &ReviewBatch,
    dependency: &DependencyState,
    policy: &AcceptancePolicy,
) -> Result<AcceptanceReceipt, ContractError> {
    batch.validate()?;
    dependency.validate()?;
    policy.validate()?;
    let agreement = AgreementMetrics::calculate(batch)?;
    let grouped = batch.grouped_reviews()?;
    let mut items = Vec::with_capacity(grouped.len());

    for (item_id, pair) in grouped {
        let left = pair[0];
        let right = pair[1];
        let review_pair_sha256 = review_pair_digest(left.digest(), right.digest());
        let disagreement = left.labels != right.labels;
        let adjudication = batch.adjudication_for(item_id);

        let (final_labels, resolved, adjudication_sha256, redaction_receipt_sha256) =
            match (disagreement, adjudication) {
                (false, None) => {
                    let resolved = left.labels.privacy != PrivacyDecision::Redact;
                    (left.labels, resolved, None, None)
                }
                (false, Some(record)) => {
                    validate_adjudication(record, left, right, false)?;
                    (
                        record.labels,
                        true,
                        Some(record.digest()),
                        record.redaction_receipt_sha256,
                    )
                }
                (true, None) => (
                    ReviewTuple::conservative(left.labels, right.labels),
                    false,
                    None,
                    None,
                ),
                (true, Some(record)) => {
                    validate_adjudication(record, left, right, true)?;
                    (
                        record.labels,
                        true,
                        Some(record.digest()),
                        record.redaction_receipt_sha256,
                    )
                }
            };

        let accepted = resolved
            && final_labels.privacy != PrivacyDecision::Block
            && (final_labels.privacy != PrivacyDecision::Redact
                || redaction_receipt_sha256.is_some());
        let mut item = ResolvedItemReceipt {
            item_id_sha256: Digest32::for_bytes(item_id.as_bytes()),
            locale: left.locale.clone(),
            final_labels,
            resolved,
            accepted,
            review_pair_sha256,
            adjudication_sha256,
            redaction_receipt_sha256,
            item_receipt_sha256: Digest32::for_bytes(b"uncomputed"),
        };
        item.item_receipt_sha256 = resolved_item_digest(&item);
        item.validate()?;
        items.push(item);
    }

    items.sort_by(|left, right| {
        left.item_id_sha256
            .cmp(&right.item_id_sha256)
            .then_with(|| left.locale.cmp(&right.locale))
    });

    let item_count = usize_to_u32(items.len())?;
    let locale_count = usize_to_u32(batch.header.locales.len())?;
    let review_count = usize_to_u32(batch.reviews.len())?;
    let reviewer_count = usize_to_u32(batch.reviewer_count())?;
    let adjudication_count = usize_to_u32(batch.adjudications.len())?;
    let unresolved_count =
        usize_to_u32(items.iter().filter(|item| !item.resolved).count())?;
    let accepted_item_count =
        usize_to_u32(items.iter().filter(|item| item.accepted).count())?;
    let privacy_block_count = usize_to_u32(
        items
            .iter()
            .filter(|item| item.final_labels.privacy == PrivacyDecision::Block)
            .count(),
    )?;
    let privacy_redact_count = usize_to_u32(
        items
            .iter()
            .filter(|item| item.final_labels.privacy == PrivacyDecision::Redact)
            .count(),
    )?;
    let dependency_commit_matches =
        dependency.p1_1c_source_commit == batch.header.source_p1_1c_commit;

    let mut blocked_reasons = Vec::new();
    if !dependency.p1_1c_source_qualified {
        blocked_reasons.push("P1_1C_SOURCE_QUALIFICATION_MISSING".to_string());
    }
    if !dependency.seed_pipeline_reproducible {
        blocked_reasons.push("P1_1C_SEED_PIPELINE_EVIDENCE_MISSING".to_string());
    }
    if !dependency_commit_matches {
        blocked_reasons.push("P1_1C_COMMIT_BINDING_MISMATCH".to_string());
    }
    if batch.header.provenance != CorpusProvenance::HumanReviewedV1 {
        blocked_reasons.push("CORPUS_PROVENANCE_NOT_HUMAN_REVIEWED".to_string());
    }
    if !batch.header.reviewed {
        blocked_reasons.push("CORPUS_REVIEWED_FLAG_FALSE".to_string());
    }
    if !batch.header.human_review_attested {
        blocked_reasons.push("HUMAN_REVIEW_ATTESTATION_MISSING".to_string());
    }
    if item_count < policy.minimum_items {
        blocked_reasons.push("INSUFFICIENT_REVIEWED_ITEMS".to_string());
    }
    if locale_count < policy.minimum_locales {
        blocked_reasons.push("INSUFFICIENT_REVIEWED_LOCALES".to_string());
    }
    if reviewer_count < 2 {
        blocked_reasons.push("INDEPENDENT_REVIEWER_SET_MISSING".to_string());
    }
    if agreement.exact_tuple_agreement_ppm
        < policy.minimum_exact_tuple_agreement_ppm
    {
        blocked_reasons.push("EXACT_REVIEW_AGREEMENT_BELOW_THRESHOLD".to_string());
    }
    if agreement.weighted_relevance_kappa_ppm
        < policy.minimum_weighted_relevance_kappa_ppm
    {
        blocked_reasons.push("WEIGHTED_RELEVANCE_KAPPA_BELOW_THRESHOLD".to_string());
    }
    if policy.require_zero_unresolved && unresolved_count != 0 {
        blocked_reasons.push("UNRESOLVED_REVIEW_DISAGREEMENTS".to_string());
    }
    if policy.require_zero_privacy_blocks && privacy_block_count != 0 {
        blocked_reasons.push("PRIVACY_BLOCKED_ITEMS_PRESENT".to_string());
    }
    if accepted_item_count != item_count {
        blocked_reasons.push("NOT_ALL_REVIEWED_ITEMS_ACCEPTED".to_string());
    }

    let reviewed_corpus_accepted = blocked_reasons.is_empty();
    let status = if reviewed_corpus_accepted {
        "PASS_P1_1C1_REVIEWED_CORPUS_ACCEPTED"
    } else if batch.header.provenance == CorpusProvenance::SyntheticReviewSeed {
        "PASS_P1_1C1_REVIEW_PIPELINE_SEED_ONLY"
    } else {
        "BLOCKED_P1_1C1_REVIEWED_CORPUS_ACCEPTANCE"
    };

    let reviewed_corpus_sha256 =
        reviewed_corpus_digest(batch.header.digest(), &items);
    let mut receipt = AcceptanceReceipt {
        schema_version: P1_1C1_SCHEMA_VERSION,
        namespace:
            "hepta_intelligence_p1_1c1_reviewed_corpus_acceptance_v1"
                .to_string(),
        status: status.to_string(),
        corpus_provenance: batch.header.provenance,
        header_reviewed: batch.header.reviewed,
        human_review_attested: batch.header.human_review_attested,
        source_p1_1c_commit: batch.header.source_p1_1c_commit.clone(),
        dependency_source_qualified: dependency.p1_1c_source_qualified,
        dependency_seed_pipeline_reproducible:
            dependency.seed_pipeline_reproducible,
        dependency_commit_matches,
        review_pipeline_validated: true,
        reviewed_corpus_accepted,
        corpus_reviewed: reviewed_corpus_accepted,
        item_count,
        locale_count,
        review_count,
        reviewer_count,
        adjudication_count,
        unresolved_count,
        accepted_item_count,
        privacy_block_count,
        privacy_redact_count,
        agreement,
        locales: batch.header.locales.iter().cloned().collect(),
        blocked_reasons,
        reviewer_set_sha256: batch.reviewer_set_digest(),
        review_batch_sha256: batch.review_batch_digest(),
        adjudication_batch_sha256: batch.adjudication_batch_digest(),
        reviewed_corpus_sha256,
        items,
        source_contract_qualified: P1_1C1_SOURCE_QUALIFIED,
        efficacy_validation: P1_1C1_EFFICACY_VALIDATION,
        efficacy_claim: P1_1C1_EFFICACY_CLAIM,
        product_workspace_member: P1_1C1_PRODUCT_WORKSPACE_MEMBER,
        product_module_registered: P1_1C1_PRODUCT_MODULE_REGISTERED,
        runtime_wired: P1_1C1_RUNTIME_WIRED,
        default_recall_changed: P1_1C1_DEFAULT_RECALL_CHANGED,
        federation_recall_changed: P1_1C1_FEDERATION_RECALL_CHANGED,
        context_attachment: P1_1C1_CONTEXT_ATTACHMENT,
        physical_send: P1_1C1_PHYSICAL_SEND,
        network_access: P1_1C1_NETWORK_ACCESS,
        model_download: P1_1C1_MODEL_DOWNLOAD,
        external_effects: P1_1C1_EXTERNAL_EFFECTS,
        production_authority: P1_1C1_PRODUCTION_AUTHORITY,
        operator_acceptance: P1_1C1_OPERATOR_ACCEPTANCE,
        promotion: P1_1C1_PROMOTION,
        callers_ratchet: P1_1C1_CALLERS_RATCHET,
        receipt_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    receipt.receipt_sha256 = acceptance_receipt_digest(&receipt);
    receipt.validate()?;
    Ok(receipt)
}

fn validate_adjudication(
    record: &crate::AdjudicationRecord,
    left: &crate::ReviewRecord,
    right: &crate::ReviewRecord,
    disagreement: bool,
) -> Result<(), ContractError> {
    record.validate()?;
    if record.adjudicator_commitment == left.reviewer_commitment
        || record.adjudicator_commitment == right.reviewer_commitment
    {
        return Err(ContractError::Invalid(format!(
            "item {} adjudicator is not independent",
            record.item_id
        )));
    }
    let required_privacy =
        PrivacyDecision::fail_closed_max(left.labels.privacy, right.labels.privacy);
    if record.labels.privacy != required_privacy {
        return Err(ContractError::Invalid(format!(
            "item {} adjudication attempted to downgrade fail-closed privacy",
            record.item_id
        )));
    }
    if disagreement {
        if !labels_match_any(record.labels, left.labels, right.labels) {
            return Err(ContractError::Invalid(format!(
                "item {} adjudication introduced labels absent from both reviews",
                record.item_id
            )));
        }
    } else if record.labels != left.labels
        || record.labels.privacy != PrivacyDecision::Redact
    {
        return Err(ContractError::Invalid(format!(
            "item {} unnecessary adjudication is only allowed to bind an agreed redaction",
            record.item_id
        )));
    }
    Ok(())
}

fn review_pair_digest(left: Digest32, right: Digest32) -> Digest32 {
    let (first, second) = if left <= right {
        (left, right)
    } else {
        (right, left)
    };
    framed_digest(
        b"hepta:intelligence:p1.1c1:review-pair:v1",
        &[first.as_bytes(), second.as_bytes()],
    )
}

fn resolved_item_digest(item: &ResolvedItemReceipt) -> Digest32 {
    let labels = item.final_labels.digest();
    let adjudication = item
        .adjudication_sha256
        .unwrap_or_else(|| Digest32::for_bytes(b"no-adjudication"));
    let redaction = item
        .redaction_receipt_sha256
        .unwrap_or_else(|| Digest32::for_bytes(b"no-redaction"));
    framed_digest(
        b"hepta:intelligence:p1.1c1:resolved-item:v1",
        &[
            item.item_id_sha256.as_bytes(),
            item.locale.as_bytes(),
            labels.as_bytes(),
            &[u8::from(item.resolved)],
            &[u8::from(item.accepted)],
            item.review_pair_sha256.as_bytes(),
            adjudication.as_bytes(),
            redaction.as_bytes(),
        ],
    )
}

fn reviewed_corpus_digest(
    header_sha256: Digest32,
    items: &[ResolvedItemReceipt],
) -> Digest32 {
    let mut parts = Vec::<&[u8]>::with_capacity(items.len().saturating_add(1));
    parts.push(header_sha256.as_bytes());
    for item in items {
        parts.push(item.item_receipt_sha256.as_bytes());
    }
    framed_digest(
        b"hepta:intelligence:p1.1c1:reviewed-corpus:v1",
        &parts,
    )
}

fn acceptance_receipt_digest(receipt: &AcceptanceReceipt) -> Digest32 {
    let metrics = receipt.agreement.digest_parts();
    let blocker_text = receipt.blocked_reasons.join("|");
    let locale_text = receipt.locales.join(",");
    framed_digest(
        b"hepta:intelligence:p1.1c1:acceptance-receipt:v1",
        &[
            &receipt.schema_version.to_be_bytes(),
            receipt.namespace.as_bytes(),
            receipt.status.as_bytes(),
            receipt.corpus_provenance.as_str().as_bytes(),
            &[u8::from(receipt.header_reviewed)],
            &[u8::from(receipt.human_review_attested)],
            receipt.source_p1_1c_commit.as_bytes(),
            &[u8::from(receipt.dependency_source_qualified)],
            &[u8::from(
                receipt.dependency_seed_pipeline_reproducible,
            )],
            &[u8::from(receipt.dependency_commit_matches)],
            &[u8::from(receipt.review_pipeline_validated)],
            &[u8::from(receipt.reviewed_corpus_accepted)],
            &[u8::from(receipt.corpus_reviewed)],
            &receipt.item_count.to_be_bytes(),
            &receipt.locale_count.to_be_bytes(),
            &receipt.review_count.to_be_bytes(),
            &receipt.reviewer_count.to_be_bytes(),
            &receipt.adjudication_count.to_be_bytes(),
            &receipt.unresolved_count.to_be_bytes(),
            &receipt.accepted_item_count.to_be_bytes(),
            &receipt.privacy_block_count.to_be_bytes(),
            &receipt.privacy_redact_count.to_be_bytes(),
            &metrics[0],
            &metrics[1],
            &metrics[2],
            &metrics[3],
            &metrics[4],
            &metrics[5],
            &metrics[6],
            locale_text.as_bytes(),
            blocker_text.as_bytes(),
            receipt.reviewer_set_sha256.as_bytes(),
            receipt.review_batch_sha256.as_bytes(),
            receipt.adjudication_batch_sha256.as_bytes(),
            receipt.reviewed_corpus_sha256.as_bytes(),
            &[u8::from(receipt.source_contract_qualified)],
            &[u8::from(receipt.efficacy_validation)],
            &[u8::from(receipt.efficacy_claim)],
            &[u8::from(receipt.product_workspace_member)],
            &[u8::from(receipt.product_module_registered)],
            &[u8::from(receipt.runtime_wired)],
            &[u8::from(receipt.default_recall_changed)],
            &[u8::from(receipt.federation_recall_changed)],
            &[u8::from(receipt.context_attachment)],
            &[u8::from(receipt.physical_send)],
            &[u8::from(receipt.network_access)],
            &[u8::from(receipt.model_download)],
            &[u8::from(receipt.external_effects)],
            &[u8::from(receipt.production_authority)],
            &[u8::from(receipt.operator_acceptance)],
            &[u8::from(receipt.promotion)],
            &[u8::from(receipt.callers_ratchet)],
        ],
    )
}

fn json_escape(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character if character.is_control() => {
                use std::fmt::Write as _;
                write!(&mut output, "\\u{:04x}", u32::from(character))
                    .expect("writing to String cannot fail");
            }
            character => output.push(character),
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_rejects_ppm_overflow() {
        let policy = AcceptancePolicy {
            minimum_exact_tuple_agreement_ppm: 1_000_001,
            ..AcceptancePolicy::default()
        };
        assert!(policy.validate().is_err());
    }

    #[test]
    fn json_escape_redacts_control_characters_safely() {
        assert_eq!(json_escape("a\nb\"c"), "a\\nb\\\"c");
    }
}

use crate::{
    ContractError, Digest32, P1_1C1_SOURCE_COMMIT, P1_1C2_CALLERS_RATCHET,
    P1_1C2_CONTEXT_ATTACHMENT, P1_1C2_DEFAULT_RECALL_CHANGED,
    P1_1C2_EFFICACY_CLAIM, P1_1C2_EXTERNAL_EFFECTS,
    P1_1C2_FEDERATION_RECALL_CHANGED, P1_1C2_MODEL_DOWNLOAD, P1_1C2_NAMESPACE,
    P1_1C2_NETWORK_ACCESS, P1_1C2_OPERATOR_ACCEPTANCE, P1_1C2_PHYSICAL_SEND,
    P1_1C2_PRODUCT_MODULE_REGISTERED, P1_1C2_PRODUCT_WORKSPACE_MEMBER,
    P1_1C2_PRODUCTION_AUTHORITY, P1_1C2_PROMOTION, P1_1C2_RUNTIME_WIRED,
    P1_1C2_SCHEMA_VERSION, P1_1C2_SOURCE_QUALIFIED, ProjectionAudit, ReviewProjection,
    framed_digest, p1c1_digest, p1c_digest, validate_commit_oid,
};
use hepta_memory_p1_1c_qualification::{
    AblationLane, CalibrationContract, CandidateFeatures, CaseMetrics, CorpusProvenance,
    EvaluationReceipt, KgEdge, KgGraph, LaneMetrics, OfflineCorpus, PPM_DENOMINATOR,
    RankedCandidate,
};
use hepta_memory_p1_1c1_qualification::{
    AcceptancePolicy, AcceptanceReceipt, DependencyState, ReviewBatch, evaluate_review_batch,
};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EfficacyPolicy {
    pub minimum_cases: u32,
    pub minimum_locales: u32,
    pub minimum_full_recall_at_4_ppm: u32,
    pub minimum_full_ndcg_at_4_ppm: u32,
    pub minimum_full_citation_precision_ppm: u32,
    pub maximum_full_p95_latency_micros: u64,
    pub maximum_full_mean_token_cost: u64,
    pub policy_sha256: Digest32,
    pub production_calibrated: bool,
}

impl Default for EfficacyPolicy {
    fn default() -> Self {
        let mut policy = Self {
            minimum_cases: 8,
            minimum_locales: 8,
            minimum_full_recall_at_4_ppm: 750_000,
            minimum_full_ndcg_at_4_ppm: 700_000,
            minimum_full_citation_precision_ppm: 400_000,
            maximum_full_p95_latency_micros: 1_000,
            maximum_full_mean_token_cost: 512,
            policy_sha256: Digest32::for_bytes(b"pending"),
            production_calibrated: false,
        };
        policy.policy_sha256 = policy.digest();
        policy
    }
}

impl EfficacyPolicy {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.minimum_cases == 0 || self.minimum_locales == 0 {
            return Err(ContractError::Invalid(
                "efficacy policy case and locale minima must be positive".to_string(),
            ));
        }
        for (value, label) in [
            (self.minimum_full_recall_at_4_ppm, "full Recall@4 threshold"),
            (self.minimum_full_ndcg_at_4_ppm, "full nDCG@4 threshold"),
            (
                self.minimum_full_citation_precision_ppm,
                "full citation precision threshold",
            ),
        ] {
            if value > PPM_DENOMINATOR {
                return Err(ContractError::Invalid(format!(
                    "{label} exceeds {PPM_DENOMINATOR} PPM"
                )));
            }
        }
        if self.maximum_full_p95_latency_micros == 0
            || self.maximum_full_mean_token_cost == 0
            || self.production_calibrated
        {
            return Err(ContractError::Invalid(
                "qualification efficacy policy must use positive bounds and remain uncalibrated"
                    .to_string(),
            ));
        }
        if self.policy_sha256 != self.digest() {
            return Err(ContractError::Corrupt(
                "efficacy policy digest mismatch".to_string(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn permits(&self, metrics: &LaneMetrics, case_count: u32, locale_count: u32) -> bool {
        case_count >= self.minimum_cases
            && locale_count >= self.minimum_locales
            && metrics.mean_recall_at_4_ppm >= self.minimum_full_recall_at_4_ppm
            && metrics.mean_ndcg_at_4_ppm >= self.minimum_full_ndcg_at_4_ppm
            && metrics.mean_citation_precision_ppm
                >= self.minimum_full_citation_precision_ppm
            && metrics.p95_latency_micros <= self.maximum_full_p95_latency_micros
            && metrics.mean_token_cost <= self.maximum_full_mean_token_cost
    }

    fn digest(&self) -> Digest32 {
        framed_digest(
            b"hepta:intelligence:p1.1c2:efficacy-policy:v1",
            &[
                &self.minimum_cases.to_be_bytes(),
                &self.minimum_locales.to_be_bytes(),
                &self.minimum_full_recall_at_4_ppm.to_be_bytes(),
                &self.minimum_full_ndcg_at_4_ppm.to_be_bytes(),
                &self
                    .minimum_full_citation_precision_ppm
                    .to_be_bytes(),
                &self.maximum_full_p95_latency_micros.to_be_bytes(),
                &self.maximum_full_mean_token_cost.to_be_bytes(),
                &[u8::from(self.production_calibrated)],
            ],
        )
    }
}

pub struct EvaluationRequest<'a> {
    pub review_batch: &'a ReviewBatch,
    pub dependency: &'a DependencyState,
    pub acceptance_policy: &'a AcceptancePolicy,
    pub acceptance_receipt: &'a AcceptanceReceipt,
    pub projection: &'a ReviewProjection,
    pub reviewed_corpus: &'a OfflineCorpus,
    pub baseline_receipt: &'a EvaluationReceipt,
    pub calibration: &'a CalibrationContract,
    pub efficacy_policy: &'a EfficacyPolicy,
    pub p1_1c1_source_commit: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaneDeltaReceipt {
    pub lane: AblationLane,
    pub reviewed: LaneMetrics,
    pub baseline: LaneMetrics,
    pub recall_delta_ppm: i64,
    pub ndcg_delta_ppm: i64,
    pub citation_delta_ppm: i64,
    pub p95_latency_delta_micros: i128,
    pub mean_token_cost_delta: i128,
    pub lane_sha256: Digest32,
}

impl LaneDeltaReceipt {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.reviewed.case_count == 0 || self.baseline.case_count == 0 {
            return Err(ContractError::Corrupt(
                "lane delta requires non-empty reviewed and baseline metrics".to_string(),
            ));
        }
        if self.recall_delta_ppm
            != i64::from(self.reviewed.mean_recall_at_4_ppm)
                - i64::from(self.baseline.mean_recall_at_4_ppm)
            || self.ndcg_delta_ppm
                != i64::from(self.reviewed.mean_ndcg_at_4_ppm)
                    - i64::from(self.baseline.mean_ndcg_at_4_ppm)
            || self.citation_delta_ppm
                != i64::from(self.reviewed.mean_citation_precision_ppm)
                    - i64::from(self.baseline.mean_citation_precision_ppm)
            || self.p95_latency_delta_micros
                != i128::from(self.reviewed.p95_latency_micros)
                    - i128::from(self.baseline.p95_latency_micros)
            || self.mean_token_cost_delta
                != i128::from(self.reviewed.mean_token_cost)
                    - i128::from(self.baseline.mean_token_cost)
        {
            return Err(ContractError::Corrupt(
                "lane delta arithmetic mismatch".to_string(),
            ));
        }
        if self.lane_sha256 != self.digest()? {
            return Err(ContractError::Corrupt(
                "lane delta digest mismatch".to_string(),
            ));
        }
        Ok(())
    }

    fn digest(&self) -> Result<Digest32, ContractError> {
        let reviewed = lane_metrics_digest(&self.reviewed)?;
        let baseline = lane_metrics_digest(&self.baseline)?;
        Ok(framed_digest(
            b"hepta:intelligence:p1.1c2:lane-delta:v1",
            &[
                self.lane.as_str().as_bytes(),
                reviewed.as_bytes(),
                baseline.as_bytes(),
                &self.recall_delta_ppm.to_be_bytes(),
                &self.ndcg_delta_ppm.to_be_bytes(),
                &self.citation_delta_ppm.to_be_bytes(),
                &self.p95_latency_delta_micros.to_be_bytes(),
                &self.mean_token_cost_delta.to_be_bytes(),
            ],
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewedCorpusEvaluationReceipt {
    pub schema_version: u32,
    pub namespace: String,
    pub status: String,
    pub p1_1c_source_commit: String,
    pub p1_1c1_source_commit: String,
    pub acceptance_receipt_sha256: Digest32,
    pub reviewed_corpus_sha256: Digest32,
    pub projection_sha256: Digest32,
    pub projection_audit_sha256: Digest32,
    pub evaluation_corpus_sha256: Digest32,
    pub baseline_receipt_sha256: Digest32,
    pub calibration_sha256: Digest32,
    pub efficacy_policy_sha256: Digest32,
    pub acceptance_recomputed: bool,
    pub acceptance_receipt_matches: bool,
    pub reviewed_corpus_present: bool,
    pub projection_complete: bool,
    pub reviewed_corpus_evaluated: bool,
    pub efficacy_thresholds_passed: bool,
    pub case_count: u32,
    pub candidate_count: u32,
    pub locale_count: u32,
    pub lanes: Vec<LaneDeltaReceipt>,
    pub blocked_reasons: Vec<String>,
    pub source_qualified: bool,
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

impl ReviewedCorpusEvaluationReceipt {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.schema_version != P1_1C2_SCHEMA_VERSION
            || self.namespace != P1_1C2_NAMESPACE
        {
            return Err(ContractError::Corrupt(
                "P1.1c.2 evaluation receipt schema mismatch".to_string(),
            ));
        }
        validate_commit_oid(&self.p1_1c_source_commit, "P1.1c source commit")?;
        validate_commit_oid(&self.p1_1c1_source_commit, "P1.1c.1 source commit")?;
        if self.case_count == 0 || self.candidate_count < self.case_count || self.locale_count == 0 {
            return Err(ContractError::Corrupt(
                "P1.1c.2 corpus counts are inconsistent".to_string(),
            ));
        }
        let lane_set = self
            .lanes
            .iter()
            .map(|lane| lane.lane)
            .collect::<BTreeSet<_>>();
        for lane in &self.lanes {
            lane.validate()?;
        }
        if self.reviewed_corpus_evaluated {
            if !self.reviewed_corpus_present
                || !self.acceptance_recomputed
                || !self.acceptance_receipt_matches
                || !self.projection_complete
                || !self.blocked_reasons.is_empty()
                || self.lanes.len() != AblationLane::ALL.len()
                || lane_set.len() != AblationLane::ALL.len()
            {
                return Err(ContractError::Corrupt(
                    "evaluated reviewed corpus lacks a complete prerequisite chain"
                        .to_string(),
                ));
            }
        } else if !self.lanes.is_empty()
            || self.efficacy_thresholds_passed
            || self.efficacy_validation
        {
            return Err(ContractError::Corrupt(
                "blocked reviewed corpus cannot emit lane or efficacy evidence".to_string(),
            ));
        }
        if self.efficacy_validation
            != (self.reviewed_corpus_evaluated && self.efficacy_thresholds_passed)
        {
            return Err(ContractError::Corrupt(
                "efficacy validation state disagrees with executable threshold evidence"
                    .to_string(),
            ));
        }
        if self.source_qualified
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
                "P1.1c.2 receipt crosses the source-only authority boundary".to_string(),
            ));
        }
        if self.receipt_sha256 != self.digest()? {
            return Err(ContractError::Corrupt(
                "P1.1c.2 evaluation receipt digest mismatch".to_string(),
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
        let lanes = self
            .lanes
            .iter()
            .map(|lane| {
                format!(
                    concat!(
                        "    {{\n",
                        "      \"lane\": \"{}\",\n",
                        "      \"reviewed_recall_at_4_ppm\": {},\n",
                        "      \"reviewed_ndcg_at_4_ppm\": {},\n",
                        "      \"reviewed_citation_precision_ppm\": {},\n",
                        "      \"reviewed_p50_latency_micros\": {},\n",
                        "      \"reviewed_p95_latency_micros\": {},\n",
                        "      \"reviewed_mean_token_cost\": {},\n",
                        "      \"recall_delta_ppm\": {},\n",
                        "      \"ndcg_delta_ppm\": {},\n",
                        "      \"citation_delta_ppm\": {},\n",
                        "      \"p95_latency_delta_micros\": {},\n",
                        "      \"mean_token_cost_delta\": {},\n",
                        "      \"lane_sha256\": \"{}\"\n",
                        "    }}"
                    ),
                    lane.lane.as_str(),
                    lane.reviewed.mean_recall_at_4_ppm,
                    lane.reviewed.mean_ndcg_at_4_ppm,
                    lane.reviewed.mean_citation_precision_ppm,
                    lane.reviewed.p50_latency_micros,
                    lane.reviewed.p95_latency_micros,
                    lane.reviewed.mean_token_cost,
                    lane.recall_delta_ppm,
                    lane.ndcg_delta_ppm,
                    lane.citation_delta_ppm,
                    lane.p95_latency_delta_micros,
                    lane.mean_token_cost_delta,
                    lane.lane_sha256
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");
        format!(
            concat!(
                "{{\n",
                "  \"schema\": \"hepta.intelligence.p1_1c2.reviewed-corpus-efficacy.v1\",\n",
                "  \"status\": \"{}\",\n",
                "  \"p1_1c_source_commit\": \"{}\",\n",
                "  \"p1_1c1_source_commit\": \"{}\",\n",
                "  \"acceptance_receipt_sha256\": \"{}\",\n",
                "  \"reviewed_corpus_sha256\": \"{}\",\n",
                "  \"projection_sha256\": \"{}\",\n",
                "  \"projection_audit_sha256\": \"{}\",\n",
                "  \"evaluation_corpus_sha256\": \"{}\",\n",
                "  \"baseline_receipt_sha256\": \"{}\",\n",
                "  \"calibration_sha256\": \"{}\",\n",
                "  \"efficacy_policy_sha256\": \"{}\",\n",
                "  \"acceptance_recomputed\": {},\n",
                "  \"acceptance_receipt_matches\": {},\n",
                "  \"reviewed_corpus_present\": {},\n",
                "  \"projection_complete\": {},\n",
                "  \"reviewed_corpus_evaluated\": {},\n",
                "  \"efficacy_thresholds_passed\": {},\n",
                "  \"case_count\": {},\n",
                "  \"candidate_count\": {},\n",
                "  \"locale_count\": {},\n",
                "  \"lanes\": [\n{}\n  ],\n",
                "  \"blocked_reasons\": [{}],\n",
                "  \"source_qualified\": {},\n",
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
            self.p1_1c_source_commit,
            self.p1_1c1_source_commit,
            self.acceptance_receipt_sha256,
            self.reviewed_corpus_sha256,
            self.projection_sha256,
            self.projection_audit_sha256,
            self.evaluation_corpus_sha256,
            self.baseline_receipt_sha256,
            self.calibration_sha256,
            self.efficacy_policy_sha256,
            self.acceptance_recomputed,
            self.acceptance_receipt_matches,
            self.reviewed_corpus_present,
            self.projection_complete,
            self.reviewed_corpus_evaluated,
            self.efficacy_thresholds_passed,
            self.case_count,
            self.candidate_count,
            self.locale_count,
            lanes,
            blockers,
            self.source_qualified,
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
            self.receipt_sha256
        )
    }

    fn digest(&self) -> Result<Digest32, ContractError> {
        let lane_digests = self
            .lanes
            .iter()
            .map(|lane| lane.lane_sha256.to_string())
            .collect::<Vec<_>>()
            .join("|");
        let blockers = self.blocked_reasons.join("|");
        Ok(framed_digest(
            b"hepta:intelligence:p1.1c2:evaluation-receipt:v1",
            &[
                &self.schema_version.to_be_bytes(),
                self.namespace.as_bytes(),
                self.status.as_bytes(),
                self.p1_1c_source_commit.as_bytes(),
                self.p1_1c1_source_commit.as_bytes(),
                self.acceptance_receipt_sha256.as_bytes(),
                self.reviewed_corpus_sha256.as_bytes(),
                self.projection_sha256.as_bytes(),
                self.projection_audit_sha256.as_bytes(),
                self.evaluation_corpus_sha256.as_bytes(),
                self.baseline_receipt_sha256.as_bytes(),
                self.calibration_sha256.as_bytes(),
                self.efficacy_policy_sha256.as_bytes(),
                &[u8::from(self.acceptance_recomputed)],
                &[u8::from(self.acceptance_receipt_matches)],
                &[u8::from(self.reviewed_corpus_present)],
                &[u8::from(self.projection_complete)],
                &[u8::from(self.reviewed_corpus_evaluated)],
                &[u8::from(self.efficacy_thresholds_passed)],
                &self.case_count.to_be_bytes(),
                &self.candidate_count.to_be_bytes(),
                &self.locale_count.to_be_bytes(),
                lane_digests.as_bytes(),
                blockers.as_bytes(),
                &[u8::from(self.source_qualified)],
                &[u8::from(self.efficacy_validation)],
                &[u8::from(self.efficacy_claim)],
                &[u8::from(self.product_workspace_member)],
                &[u8::from(self.product_module_registered)],
                &[u8::from(self.runtime_wired)],
                &[u8::from(self.default_recall_changed)],
                &[u8::from(self.federation_recall_changed)],
                &[u8::from(self.context_attachment)],
                &[u8::from(self.physical_send)],
                &[u8::from(self.network_access)],
                &[u8::from(self.model_download)],
                &[u8::from(self.external_effects)],
                &[u8::from(self.production_authority)],
                &[u8::from(self.operator_acceptance)],
                &[u8::from(self.promotion)],
                &[u8::from(self.callers_ratchet)],
            ],
        ))
    }
}

pub fn evaluate_reviewed_corpus(
    request: &EvaluationRequest<'_>,
) -> Result<ReviewedCorpusEvaluationReceipt, ContractError> {
    request.review_batch.validate()?;
    request.dependency.validate()?;
    request.acceptance_policy.validate()?;
    request.acceptance_receipt.validate()?;
    request.projection.validate()?;
    request.reviewed_corpus.validate()?;
    request.baseline_receipt.validate()?;
    request.calibration.validate()?;
    request.efficacy_policy.validate()?;
    validate_commit_oid(
        request.p1_1c1_source_commit,
        "P1.1c.1 evaluation source commit",
    )?;

    let recomputed = evaluate_review_batch(
        request.review_batch,
        request.dependency,
        request.acceptance_policy,
    )?;
    let acceptance_receipt_matches = recomputed == *request.acceptance_receipt;
    let projection_audit = request
        .projection
        .audit(request.review_batch, request.reviewed_corpus)?;
    let mut blockers = BTreeSet::new();

    if request.p1_1c1_source_commit != P1_1C1_SOURCE_COMMIT {
        blockers.insert("dependency.p1c1_source_commit_mismatch".to_string());
    }
    if request.acceptance_receipt.source_p1_1c_commit != crate::P1_1C_SOURCE_COMMIT
        || request.review_batch.header.source_p1_1c_commit != crate::P1_1C_SOURCE_COMMIT
    {
        blockers.insert("dependency.p1c_source_commit_mismatch".to_string());
    }
    if !acceptance_receipt_matches {
        blockers.insert("acceptance.receipt_recomputation_mismatch".to_string());
    }
    if !recomputed.reviewed_corpus_accepted {
        blockers.insert("acceptance.reviewed_corpus_not_accepted".to_string());
    }
    if !recomputed.corpus_reviewed {
        blockers.insert("acceptance.corpus_not_reviewed".to_string());
    }
    if !recomputed.human_review_attested {
        blockers.insert("acceptance.human_review_not_attested".to_string());
    }
    if !recomputed.dependency_source_qualified {
        blockers.insert("acceptance.p1c_source_not_qualified".to_string());
    }
    if !recomputed.dependency_seed_pipeline_reproducible {
        blockers.insert("acceptance.seed_pipeline_not_reproducible".to_string());
    }
    if request.reviewed_corpus.header.provenance != CorpusProvenance::ReviewedHuman {
        blockers.insert("corpus.provenance_not_reviewed_human".to_string());
    }
    if !request.reviewed_corpus.header.reviewed {
        blockers.insert("corpus.reviewed_flag_false".to_string());
    }
    if request.baseline_receipt.corpus_reviewed
        || request.baseline_receipt.corpus_provenance != "synthetic_seed"
        || request.baseline_receipt.efficacy_validation
        || request.baseline_receipt.efficacy_claim
    {
        blockers.insert("baseline.seed_boundary_invalid".to_string());
    }
    blockers.extend(projection_audit.blocked_reasons.iter().cloned());

    let case_count = u32::try_from(request.reviewed_corpus.cases.len())
        .map_err(|_| ContractError::Overflow)?;
    let candidate_count = request
        .reviewed_corpus
        .cases
        .iter()
        .try_fold(0_u32, |count, case| {
            let candidate_count =
                u32::try_from(case.candidates.len()).map_err(|_| ContractError::Overflow)?;
            count
                .checked_add(candidate_count)
                .ok_or(ContractError::Overflow)
        })?;
    let locale_count = u32::try_from(request.reviewed_corpus.header.locales.len())
        .map_err(|_| ContractError::Overflow)?;

    let reviewed_corpus_present = recomputed.reviewed_corpus_accepted
        && recomputed.corpus_reviewed
        && recomputed.human_review_attested
        && request.reviewed_corpus.header.provenance == CorpusProvenance::ReviewedHuman
        && request.reviewed_corpus.header.reviewed;
    let projection_complete = projection_audit.eligible_for_reviewed_evaluation;
    let mut lanes = Vec::new();
    let mut reviewed_corpus_evaluated = false;
    let mut efficacy_thresholds_passed = false;

    if blockers.is_empty() {
        let reviewed_lanes = run_seven_lanes(request.reviewed_corpus, request.calibration)?;
        lanes = build_lane_deltas(&reviewed_lanes, request.baseline_receipt)?;
        let full = lanes
            .iter()
            .find(|lane| lane.lane == AblationLane::Full)
            .ok_or_else(|| ContractError::Missing("full reviewed evaluation lane".to_string()))?;
        efficacy_thresholds_passed = request
            .efficacy_policy
            .permits(&full.reviewed, case_count, locale_count);
        reviewed_corpus_evaluated = true;
    }

    let blocked_reasons = blockers.into_iter().collect::<Vec<_>>();
    let status = if !reviewed_corpus_evaluated {
        "BLOCKED_P1_1C2_REVIEWED_CORPUS_DEPENDENCY"
    } else if efficacy_thresholds_passed {
        "PASS_P1_1C2_REVIEWED_CORPUS_EFFICACY_VALIDATION"
    } else {
        "FAIL_P1_1C2_EFFICACY_THRESHOLDS"
    };
    let mut receipt = ReviewedCorpusEvaluationReceipt {
        schema_version: P1_1C2_SCHEMA_VERSION,
        namespace: P1_1C2_NAMESPACE.to_string(),
        status: status.to_string(),
        p1_1c_source_commit: request.acceptance_receipt.source_p1_1c_commit.clone(),
        p1_1c1_source_commit: request.p1_1c1_source_commit.to_string(),
        acceptance_receipt_sha256: p1c1_digest(request.acceptance_receipt.receipt_sha256)?,
        reviewed_corpus_sha256: p1c1_digest(request.acceptance_receipt.reviewed_corpus_sha256)?,
        projection_sha256: request.projection.projection_sha256,
        projection_audit_sha256: projection_audit.audit_sha256,
        evaluation_corpus_sha256: p1c_digest(request.reviewed_corpus.header.corpus_sha256)?,
        baseline_receipt_sha256: p1c_digest(request.baseline_receipt.receipt_sha256)?,
        calibration_sha256: p1c_digest(request.calibration.contract_sha256)?,
        efficacy_policy_sha256: request.efficacy_policy.policy_sha256,
        acceptance_recomputed: true,
        acceptance_receipt_matches,
        reviewed_corpus_present,
        projection_complete,
        reviewed_corpus_evaluated,
        efficacy_thresholds_passed,
        case_count,
        candidate_count,
        locale_count,
        lanes,
        blocked_reasons,
        source_qualified: P1_1C2_SOURCE_QUALIFIED,
        efficacy_validation: reviewed_corpus_evaluated && efficacy_thresholds_passed,
        efficacy_claim: P1_1C2_EFFICACY_CLAIM,
        product_workspace_member: P1_1C2_PRODUCT_WORKSPACE_MEMBER,
        product_module_registered: P1_1C2_PRODUCT_MODULE_REGISTERED,
        runtime_wired: P1_1C2_RUNTIME_WIRED,
        default_recall_changed: P1_1C2_DEFAULT_RECALL_CHANGED,
        federation_recall_changed: P1_1C2_FEDERATION_RECALL_CHANGED,
        context_attachment: P1_1C2_CONTEXT_ATTACHMENT,
        physical_send: P1_1C2_PHYSICAL_SEND,
        network_access: P1_1C2_NETWORK_ACCESS,
        model_download: P1_1C2_MODEL_DOWNLOAD,
        external_effects: P1_1C2_EXTERNAL_EFFECTS,
        production_authority: P1_1C2_PRODUCTION_AUTHORITY,
        operator_acceptance: P1_1C2_OPERATOR_ACCEPTANCE,
        promotion: P1_1C2_PROMOTION,
        callers_ratchet: P1_1C2_CALLERS_RATCHET,
        receipt_sha256: Digest32::for_bytes(b"pending"),
    };
    receipt.receipt_sha256 = receipt.digest()?;
    receipt.validate()?;
    Ok(receipt)
}

fn run_seven_lanes(
    corpus: &OfflineCorpus,
    calibration: &CalibrationContract,
) -> Result<Vec<(AblationLane, LaneMetrics)>, ContractError> {
    let mut lane_cases = vec![Vec::new(); AblationLane::ALL.len()];
    for case in &corpus.cases {
        let mut graph = KgGraph::new();
        for candidate in &case.candidates {
            let first_target = candidate
                .middle_node
                .as_deref()
                .unwrap_or(&candidate.goal_node);
            graph.add_edge(KgEdge {
                source: candidate.start_node.clone(),
                target: first_target.to_string(),
                relation: "supports".to_string(),
                truth_ppm: candidate.edge1_truth_ppm,
                contradiction_ppm: candidate.edge1_contradiction_ppm,
            })?;
            if let Some(middle) = &candidate.middle_node {
                graph.add_edge(KgEdge {
                    source: middle.clone(),
                    target: candidate.goal_node.clone(),
                    relation: "supports".to_string(),
                    truth_ppm: candidate.edge2_truth_ppm,
                    contradiction_ppm: candidate.edge2_contradiction_ppm,
                })?;
            }
        }
        let grades = case
            .candidates
            .iter()
            .map(|candidate| candidate.relevance_grade)
            .collect::<Vec<_>>();
        for (lane_index, lane) in AblationLane::ALL.into_iter().enumerate() {
            let mut ranking = Vec::with_capacity(case.candidates.len());
            for candidate in &case.candidates {
                let evidence = graph.bounded_two_hop(
                    &candidate.start_node,
                    &candidate.goal_node,
                )?;
                let features = CandidateFeatures::new(
                    candidate.lexical_ppm,
                    candidate.vector_ppm,
                    evidence.net_support_ppm,
                    if evidence.found { PPM_DENOMINATOR } else { 0 },
                    evidence.truth_ppm,
                    evidence.contradiction_ppm,
                    if candidate.citation_supported {
                        PPM_DENOMINATOR
                    } else {
                        0
                    },
                )?;
                ranking.push(RankedCandidate {
                    candidate_id: candidate.candidate_id.clone(),
                    relevance_grade: candidate.relevance_grade,
                    citation_supported: candidate.citation_supported,
                    score_ppm: calibration.score(lane, &features),
                    latency_micros: candidate.latency_micros,
                    token_cost: candidate.token_cost,
                    feature_sha256: features.feature_sha256,
                });
            }
            ranking.sort_by(|left, right| {
                right
                    .score_ppm
                    .cmp(&left.score_ppm)
                    .then_with(|| left.candidate_id.cmp(&right.candidate_id))
            });
            lane_cases[lane_index].push(CaseMetrics::from_ranking(&ranking, &grades)?);
        }
    }
    AblationLane::ALL
        .into_iter()
        .zip(lane_cases.iter())
        .map(|(lane, cases)| Ok((lane, LaneMetrics::aggregate(cases)?)))
        .collect()
}

fn build_lane_deltas(
    reviewed_lanes: &[(AblationLane, LaneMetrics)],
    baseline: &EvaluationReceipt,
) -> Result<Vec<LaneDeltaReceipt>, ContractError> {
    let mut output = Vec::with_capacity(AblationLane::ALL.len());
    for (lane, reviewed) in reviewed_lanes {
        let baseline_metrics = baseline
            .lanes
            .iter()
            .find(|receipt| receipt.lane == *lane)
            .map(|receipt| receipt.metrics.clone())
            .ok_or_else(|| {
                ContractError::Missing(format!(
                    "baseline lane {}",
                    lane.as_str()
                ))
            })?;
        let mut delta = LaneDeltaReceipt {
            lane: *lane,
            reviewed: reviewed.clone(),
            baseline: baseline_metrics,
            recall_delta_ppm: i64::from(reviewed.mean_recall_at_4_ppm),
            ndcg_delta_ppm: i64::from(reviewed.mean_ndcg_at_4_ppm),
            citation_delta_ppm: i64::from(reviewed.mean_citation_precision_ppm),
            p95_latency_delta_micros: i128::from(reviewed.p95_latency_micros),
            mean_token_cost_delta: i128::from(reviewed.mean_token_cost),
            lane_sha256: Digest32::for_bytes(b"pending"),
        };
        delta.recall_delta_ppm -= i64::from(delta.baseline.mean_recall_at_4_ppm);
        delta.ndcg_delta_ppm -= i64::from(delta.baseline.mean_ndcg_at_4_ppm);
        delta.citation_delta_ppm -= i64::from(delta.baseline.mean_citation_precision_ppm);
        delta.p95_latency_delta_micros -= i128::from(delta.baseline.p95_latency_micros);
        delta.mean_token_cost_delta -= i128::from(delta.baseline.mean_token_cost);
        delta.lane_sha256 = delta.digest()?;
        delta.validate()?;
        output.push(delta);
    }
    Ok(output)
}

fn lane_metrics_digest(metrics: &LaneMetrics) -> Result<Digest32, ContractError> {
    Ok(framed_digest(
        b"hepta:intelligence:p1.1c2:lane-metrics:v1",
        &[
            &metrics.case_count.to_be_bytes(),
            &metrics.mean_recall_at_4_ppm.to_be_bytes(),
            &metrics.mean_ndcg_at_4_ppm.to_be_bytes(),
            &metrics.mean_citation_precision_ppm.to_be_bytes(),
            &metrics.p50_latency_micros.to_be_bytes(),
            &metrics.p95_latency_micros.to_be_bytes(),
            &metrics.mean_token_cost.to_be_bytes(),
            p1c_digest(metrics.cases_sha256)?.as_bytes(),
        ],
    ))
}

fn json_escape(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for character in input.chars() {
        match character {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            value if value.is_control() => output.push('?'),
            value => output.push(value),
        }
    }
    output
}

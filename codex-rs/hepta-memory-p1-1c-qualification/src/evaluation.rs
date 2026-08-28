use crate::{
    AblationLane, CalibrationContract, CandidateFeatures, ContractError, Digest32, KgEdge, KgGraph,
    LaneMetrics, OfflineCorpus, PPM_DENOMINATOR, RankedCandidate, validate_id,
};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneReceipt {
    pub lane: AblationLane,
    pub metrics: LaneMetrics,
    pub lane_sha256: Digest32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationReceipt {
    pub schema: String,
    pub corpus_id: String,
    pub corpus_version: u32,
    pub corpus_sha256: Digest32,
    pub corpus_provenance: String,
    pub corpus_reviewed: bool,
    pub locales: Vec<String>,
    pub case_count: u32,
    pub calibration_sha256: Digest32,
    pub lanes: Vec<LaneReceipt>,
    pub receipt_sha256: Digest32,
    pub deterministic: bool,
    pub offline: bool,
    pub network_access: bool,
    pub model_download: bool,
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
}

impl EvaluationReceipt {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.schema != crate::P1_1C_SCHEMA {
            return Err(ContractError::Invalid(format!(
                "unexpected evaluation schema {}",
                self.schema
            )));
        }
        validate_id(&self.corpus_id, "receipt corpus ID")?;
        if self.corpus_version == 0 || self.case_count == 0 {
            return Err(ContractError::Invalid(
                "receipt corpus version and case count must be non-zero".to_string(),
            ));
        }
        if self.locales.is_empty() || self.lanes.len() != AblationLane::ALL.len() {
            return Err(ContractError::Invalid(
                "receipt must contain locales and all seven ablation lanes".to_string(),
            ));
        }
        let mut lanes = BTreeSet::new();
        for lane in &self.lanes {
            if !lanes.insert(lane.lane) {
                return Err(ContractError::Invalid(format!(
                    "duplicate lane {}",
                    lane.lane.as_str()
                )));
            }
            if lane.metrics.case_count != self.case_count {
                return Err(ContractError::Corrupt(format!(
                    "lane {} case count does not match receipt",
                    lane.lane.as_str()
                )));
            }
            if lane.lane_sha256 != lane_digest(lane.lane, &lane.metrics) {
                return Err(ContractError::Corrupt(format!(
                    "lane {} digest does not match metrics",
                    lane.lane.as_str()
                )));
            }
        }
        if self.receipt_sha256 != Digest32::for_bytes(self.canonical_payload().as_bytes()) {
            return Err(ContractError::Corrupt(
                "evaluation receipt digest does not match canonical fields".to_string(),
            ));
        }
        if !self.deterministic || !self.offline {
            return Err(ContractError::Invalid(
                "P1.1c qualification must be deterministic and offline".to_string(),
            ));
        }
        if self.network_access
            || self.model_download
            || self.product_workspace_member
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
            return Err(ContractError::Invalid(
                "seed-corpus receipt must keep efficacy and authority claims disabled".to_string(),
            ));
        }
        if self.corpus_reviewed {
            return Err(ContractError::Invalid(
                "P1.1c seed receipt must not claim reviewed-human corpus status".to_string(),
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
        let lanes = self
            .lanes
            .iter()
            .map(|lane| {
                format!(
                    concat!(
                        "    {{\n",
                        "      \"lane\": \"{}\",\n",
                        "      \"case_count\": {},\n",
                        "      \"mean_recall_at_4_ppm\": {},\n",
                        "      \"mean_ndcg_at_4_ppm\": {},\n",
                        "      \"mean_citation_precision_ppm\": {},\n",
                        "      \"p50_latency_micros\": {},\n",
                        "      \"p95_latency_micros\": {},\n",
                        "      \"mean_token_cost\": {},\n",
                        "      \"cases_sha256\": \"{}\",\n",
                        "      \"lane_sha256\": \"{}\"\n",
                        "    }}"
                    ),
                    lane.lane.as_str(),
                    lane.metrics.case_count,
                    lane.metrics.mean_recall_at_4_ppm,
                    lane.metrics.mean_ndcg_at_4_ppm,
                    lane.metrics.mean_citation_precision_ppm,
                    lane.metrics.p50_latency_micros,
                    lane.metrics.p95_latency_micros,
                    lane.metrics.mean_token_cost,
                    lane.metrics.cases_sha256,
                    lane.lane_sha256
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");
        format!(
            concat!(
                "{{\n",
                "  \"schema\": \"{}\",\n",
                "  \"status\": \"PASS_P1_1C_SEED_PIPELINE\",\n",
                "  \"corpus_id\": \"{}\",\n",
                "  \"corpus_version\": {},\n",
                "  \"corpus_sha256\": \"{}\",\n",
                "  \"corpus_provenance\": \"{}\",\n",
                "  \"corpus_reviewed\": {},\n",
                "  \"locales\": [{}],\n",
                "  \"case_count\": {},\n",
                "  \"calibration_sha256\": \"{}\",\n",
                "  \"lanes\": [\n{}\n  ],\n",
                "  \"receipt_sha256\": \"{}\",\n",
                "  \"deterministic\": {},\n",
                "  \"offline\": {},\n",
                "  \"network_access\": {},\n",
                "  \"model_download\": {},\n",
                "  \"product_workspace_member\": {},\n",
                "  \"product_module_registered\": {},\n",
                "  \"runtime_wired\": {},\n",
                "  \"default_recall_changed\": {},\n",
                "  \"federation_recall_changed\": {},\n",
                "  \"context_attachment\": {},\n",
                "  \"physical_send\": {},\n",
                "  \"external_effects\": {},\n",
                "  \"production_authority\": {},\n",
                "  \"efficacy_validation\": {},\n",
                "  \"efficacy_claim\": {},\n",
                "  \"operator_acceptance\": {},\n",
                "  \"promotion\": {},\n",
                "  \"callers_ratchet\": {}\n",
                "}}\n"
            ),
            json_escape(&self.schema),
            json_escape(&self.corpus_id),
            self.corpus_version,
            self.corpus_sha256,
            json_escape(&self.corpus_provenance),
            self.corpus_reviewed,
            locales,
            self.case_count,
            self.calibration_sha256,
            lanes,
            self.receipt_sha256,
            self.deterministic,
            self.offline,
            self.network_access,
            self.model_download,
            self.product_workspace_member,
            self.product_module_registered,
            self.runtime_wired,
            self.default_recall_changed,
            self.federation_recall_changed,
            self.context_attachment,
            self.physical_send,
            self.external_effects,
            self.production_authority,
            self.efficacy_validation,
            self.efficacy_claim,
            self.operator_acceptance,
            self.promotion,
            self.callers_ratchet
        )
    }

    fn canonical_payload(&self) -> String {
        let locale_payload = self.locales.join(",");
        let lane_payload = self
            .lanes
            .iter()
            .map(|lane| format!("{}:{}", lane.lane.as_str(), lane.lane_sha256))
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.schema,
            self.corpus_id,
            self.corpus_version,
            self.corpus_sha256,
            self.corpus_provenance,
            self.corpus_reviewed,
            locale_payload,
            self.case_count,
            self.calibration_sha256,
            lane_payload,
            self.deterministic,
            self.offline,
            self.network_access,
            self.model_download,
            self.product_workspace_member,
            self.product_module_registered,
            self.runtime_wired,
            self.default_recall_changed,
            self.federation_recall_changed,
            self.context_attachment,
            self.physical_send,
            self.external_effects,
            self.production_authority,
            self.efficacy_validation,
            self.efficacy_claim,
            self.operator_acceptance,
            self.promotion,
            self.callers_ratchet,
            false
        )
    }
}

pub fn evaluate_corpus(
    corpus: &OfflineCorpus,
    calibration: &CalibrationContract,
) -> Result<EvaluationReceipt, ContractError> {
    corpus.validate()?;
    calibration.validate()?;
    let mut lane_cases = vec![Vec::new(); AblationLane::ALL.len()];

    for case in &corpus.cases {
        let graph = graph_for_case(case)?;
        let grades: Vec<u8> = case
            .candidates
            .iter()
            .map(|candidate| candidate.relevance_grade)
            .collect();
        for (lane_index, lane) in AblationLane::ALL.into_iter().enumerate() {
            let mut ranking = Vec::with_capacity(case.candidates.len());
            for candidate in &case.candidates {
                let evidence = graph.bounded_two_hop(&candidate.start_node, &candidate.goal_node)?;
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
            lane_cases[lane_index].push(crate::CaseMetrics::from_ranking(&ranking, &grades)?);
        }
    }

    let mut lanes = Vec::with_capacity(AblationLane::ALL.len());
    for (lane, cases) in AblationLane::ALL.into_iter().zip(lane_cases.iter()) {
        let metrics = LaneMetrics::aggregate(cases)?;
        lanes.push(LaneReceipt {
            lane,
            lane_sha256: lane_digest(lane, &metrics),
            metrics,
        });
    }

    let mut receipt = EvaluationReceipt {
        schema: crate::P1_1C_SCHEMA.to_string(),
        corpus_id: corpus.header.corpus_id.clone(),
        corpus_version: corpus.header.version,
        corpus_sha256: corpus.header.corpus_sha256,
        corpus_provenance: corpus.header.provenance.as_str().to_string(),
        corpus_reviewed: corpus.header.reviewed,
        locales: corpus.header.locales.clone(),
        case_count: u32::try_from(corpus.cases.len()).map_err(|_| ContractError::Overflow)?,
        calibration_sha256: calibration.contract_sha256,
        lanes,
        receipt_sha256: Digest32::for_bytes(b"pending"),
        deterministic: true,
        offline: true,
        network_access: false,
        model_download: false,
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
    };
    receipt.receipt_sha256 = Digest32::for_bytes(receipt.canonical_payload().as_bytes());
    receipt.validate()?;
    Ok(receipt)
}

fn graph_for_case(case: &crate::CorpusCase) -> Result<KgGraph, ContractError> {
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
    Ok(graph)
}

fn lane_digest(lane: AblationLane, metrics: &LaneMetrics) -> Digest32 {
    let payload = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}",
        lane.as_str(),
        metrics.case_count,
        metrics.mean_recall_at_4_ppm,
        metrics.mean_ndcg_at_4_ppm,
        metrics.mean_citation_precision_ppm,
        metrics.p50_latency_micros,
        metrics.p95_latency_micros,
        metrics.mean_token_cost,
        metrics.cases_sha256
    );
    Digest32::for_bytes(payload.as_bytes())
}

fn json_escape(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for character in input.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            value if value.is_control() => {
                use std::fmt::Write as _;
                write!(&mut output, "\\u{:04x}", value as u32)
                    .expect("writing to String cannot fail");
            }
            value => output.push(value),
        }
    }
    output
}

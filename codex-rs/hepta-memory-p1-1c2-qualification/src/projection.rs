use crate::{
    ContractError, Digest32, MAX_BLOCKED_REASONS, MAX_PROJECTION_ENTRIES, framed_digest,
    p1c1_digest, p1c_digest, validate_commit_oid, validate_id,
};
use hepta_memory_p1_1c_qualification::{CandidateFixture, CorpusCase, OfflineCorpus};
use hepta_memory_p1_1c1_qualification::ReviewBatch;
use std::collections::{BTreeMap, BTreeSet};

const PROJECTION_SCHEMA: &str = "hepta.intelligence.p1_1c2.review_projection.v1";
const PROJECTION_COLUMNS: &str =
    "item_id\tcase_id\tcandidate_id\tquery_sha256\tcandidate_sha256";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionEntry {
    pub item_id: String,
    pub case_id: String,
    pub candidate_id: String,
    pub query_sha256: Digest32,
    pub candidate_sha256: Digest32,
}

impl ProjectionEntry {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.item_id, "projection item ID")?;
        validate_id(&self.case_id, "projection case ID")?;
        validate_id(&self.candidate_id, "projection candidate ID")
    }

    fn digest(&self) -> Digest32 {
        framed_digest(
            b"hepta:intelligence:p1.1c2:projection-entry:v1",
            &[
                self.item_id.as_bytes(),
                self.case_id.as_bytes(),
                self.candidate_id.as_bytes(),
                self.query_sha256.as_bytes(),
                self.candidate_sha256.as_bytes(),
            ],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewProjection {
    pub schema: String,
    pub fixture_only: bool,
    pub p1_1c1_source_commit: String,
    pub entries: Vec<ProjectionEntry>,
    pub projection_sha256: Digest32,
}

impl ReviewProjection {
    pub fn parse_tsv(input: &str) -> Result<Self, ContractError> {
        let (metadata, rows) = split_metadata_and_rows(input)?;
        let schema = required_metadata(&metadata, "schema")?.to_string();
        let fixture_only = parse_bool(required_metadata(&metadata, "fixture_only")?)?;
        let p1_1c1_source_commit =
            required_metadata(&metadata, "p1_1c1_source_commit")?.to_string();
        let mut lines = rows.into_iter();
        if lines.next() != Some(PROJECTION_COLUMNS) {
            return Err(ContractError::Invalid(
                "review projection TSV column header mismatch".to_string(),
            ));
        }
        let mut entries = Vec::new();
        for line in lines {
            if entries.len() >= MAX_PROJECTION_ENTRIES {
                return Err(ContractError::Invalid(format!(
                    "projection row count exceeds {MAX_PROJECTION_ENTRIES}"
                )));
            }
            let columns = line.split('\t').collect::<Vec<_>>();
            if columns.len() != 5 {
                return Err(ContractError::Invalid(
                    "projection row must contain exactly five columns".to_string(),
                ));
            }
            let entry = ProjectionEntry {
                item_id: columns[0].to_string(),
                case_id: columns[1].to_string(),
                candidate_id: columns[2].to_string(),
                query_sha256: Digest32::from_hex(columns[3])?,
                candidate_sha256: Digest32::from_hex(columns[4])?,
            };
            entry.validate()?;
            entries.push(entry);
        }
        entries.sort_by(|left, right| {
            left.case_id
                .cmp(&right.case_id)
                .then_with(|| left.candidate_id.cmp(&right.candidate_id))
                .then_with(|| left.item_id.cmp(&right.item_id))
        });
        let mut projection = Self {
            schema,
            fixture_only,
            p1_1c1_source_commit,
            entries,
            projection_sha256: Digest32::for_bytes(b"pending"),
        };
        projection.projection_sha256 = projection.digest();
        projection.validate()?;
        Ok(projection)
    }

    pub fn for_corpus(
        corpus: &OfflineCorpus,
        p1_1c1_source_commit: &str,
        fixture_only: bool,
    ) -> Result<Self, ContractError> {
        corpus.validate()?;
        validate_commit_oid(p1_1c1_source_commit, "P1.1c.1 source commit")?;
        let candidate_count = corpus
            .cases
            .iter()
            .try_fold(0_usize, |count, case| {
                count
                    .checked_add(case.candidates.len())
                    .ok_or(ContractError::Overflow)
            })?;
        if candidate_count > MAX_PROJECTION_ENTRIES {
            return Err(ContractError::Invalid(format!(
                "evaluation corpus exceeds {MAX_PROJECTION_ENTRIES} candidates"
            )));
        }
        let mut entries = Vec::with_capacity(candidate_count);
        for case in &corpus.cases {
            for candidate in &case.candidates {
                entries.push(ProjectionEntry {
                    item_id: format!("{}:{}", case.case_id, candidate.candidate_id),
                    case_id: case.case_id.clone(),
                    candidate_id: candidate.candidate_id.clone(),
                    query_sha256: p1c_digest(case.query_sha256)?,
                    candidate_sha256: candidate_projection_digest(case, candidate),
                });
            }
        }
        entries.sort_by(|left, right| {
            left.case_id
                .cmp(&right.case_id)
                .then_with(|| left.candidate_id.cmp(&right.candidate_id))
        });
        let mut projection = Self {
            schema: PROJECTION_SCHEMA.to_string(),
            fixture_only,
            p1_1c1_source_commit: p1_1c1_source_commit.to_string(),
            entries,
            projection_sha256: Digest32::for_bytes(b"pending"),
        };
        projection.projection_sha256 = projection.digest();
        projection.validate()?;
        Ok(projection)
    }

    pub fn validate(&self) -> Result<(), ContractError> {
        if self.schema != PROJECTION_SCHEMA {
            return Err(ContractError::Invalid(
                "review projection schema mismatch".to_string(),
            ));
        }
        validate_commit_oid(
            &self.p1_1c1_source_commit,
            "review projection P1.1c.1 source commit",
        )?;
        if self.entries.is_empty() || self.entries.len() > MAX_PROJECTION_ENTRIES {
            return Err(ContractError::Invalid(format!(
                "projection entries must contain 1..={MAX_PROJECTION_ENTRIES} rows"
            )));
        }
        let mut item_ids = BTreeSet::new();
        let mut candidate_ids = BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            if !item_ids.insert(entry.item_id.as_str()) {
                return Err(ContractError::Duplicate(
                    "projection item ID".to_string(),
                ));
            }
            if !candidate_ids.insert((entry.case_id.as_str(), entry.candidate_id.as_str())) {
                return Err(ContractError::Duplicate(
                    "projection case/candidate pair".to_string(),
                ));
            }
        }
        if self.projection_sha256 != self.digest() {
            return Err(ContractError::Corrupt(
                "review projection digest mismatch".to_string(),
            ));
        }
        Ok(())
    }

    pub fn audit(
        &self,
        reviews: &ReviewBatch,
        corpus: &OfflineCorpus,
    ) -> Result<ProjectionAudit, ContractError> {
        self.validate()?;
        reviews.validate()?;
        corpus.validate()?;

        let mut blockers = BTreeSet::new();
        if self.fixture_only {
            blockers.insert("projection.fixture_only".to_string());
        }
        if self.p1_1c1_source_commit != crate::P1_1C1_SOURCE_COMMIT {
            blockers.insert("projection.p1c1_source_commit_mismatch".to_string());
        }

        let candidate_count = corpus
            .cases
            .iter()
            .try_fold(0_usize, |count, case| {
                count
                    .checked_add(case.candidates.len())
                    .ok_or(ContractError::Overflow)
            })?;
        if self.entries.len() != candidate_count {
            blockers.insert("projection.candidate_coverage_incomplete".to_string());
        }
        if reviews.item_count() != candidate_count {
            blockers.insert("projection.review_coverage_incomplete".to_string());
        }

        let review_map = reviews_by_item(reviews);
        let projection_items = self
            .entries
            .iter()
            .map(|entry| entry.item_id.as_str())
            .collect::<BTreeSet<_>>();
        if review_map.keys().copied().collect::<BTreeSet<_>>() != projection_items {
            blockers.insert("projection.review_item_set_mismatch".to_string());
        }

        let case_map = corpus
            .cases
            .iter()
            .map(|case| (case.case_id.as_str(), case))
            .collect::<BTreeMap<_, _>>();
        let mut matched_entries = 0_usize;
        for entry in &self.entries {
            let Some(case) = case_map.get(entry.case_id.as_str()) else {
                blockers.insert("projection.case_missing".to_string());
                continue;
            };
            let Some(candidate) = case
                .candidates
                .iter()
                .find(|candidate| candidate.candidate_id == entry.candidate_id)
            else {
                blockers.insert("projection.candidate_missing".to_string());
                continue;
            };
            let Some(item_reviews) = review_map.get(entry.item_id.as_str()) else {
                blockers.insert("projection.review_pair_missing".to_string());
                continue;
            };
            if item_reviews.len() != 2 {
                blockers.insert("projection.review_pair_cardinality".to_string());
                continue;
            }

            let expected_query = p1c_digest(case.query_sha256)?;
            let expected_candidate = candidate_projection_digest(case, candidate);
            if entry.query_sha256 != expected_query
                || item_reviews
                    .iter()
                    .any(|review| p1c1_digest(review.query_sha256).ok() != Some(expected_query))
            {
                blockers.insert("projection.query_digest_mismatch".to_string());
                continue;
            }
            if entry.candidate_sha256 != expected_candidate
                || item_reviews.iter().any(|review| {
                    p1c1_digest(review.candidate_sha256).ok() != Some(expected_candidate)
                })
            {
                blockers.insert("projection.candidate_digest_mismatch".to_string());
                continue;
            }
            if item_reviews.iter().any(|review| review.locale != case.locale) {
                blockers.insert("projection.locale_mismatch".to_string());
                continue;
            }
            matched_entries = matched_entries
                .checked_add(1)
                .ok_or(ContractError::Overflow)?;
        }

        let blocked_reasons = blockers
            .into_iter()
            .take(MAX_BLOCKED_REASONS)
            .collect::<Vec<_>>();
        let coverage_complete = self.entries.len() == candidate_count
            && reviews.item_count() == candidate_count
            && projection_items.len() == candidate_count;
        let bindings_match = matched_entries == candidate_count;
        let eligible = blocked_reasons.is_empty() && coverage_complete && bindings_match;
        let mut audit = ProjectionAudit {
            entry_count: u32::try_from(self.entries.len()).map_err(|_| ContractError::Overflow)?,
            review_item_count: u32::try_from(reviews.item_count())
                .map_err(|_| ContractError::Overflow)?,
            evaluation_candidate_count: u32::try_from(candidate_count)
                .map_err(|_| ContractError::Overflow)?,
            matched_entry_count: u32::try_from(matched_entries)
                .map_err(|_| ContractError::Overflow)?,
            coverage_complete,
            bindings_match,
            fixture_only: self.fixture_only,
            eligible_for_reviewed_evaluation: eligible,
            blocked_reasons,
            audit_sha256: Digest32::for_bytes(b"pending"),
        };
        audit.audit_sha256 = audit.digest();
        audit.validate()?;
        Ok(audit)
    }

    fn digest(&self) -> Digest32 {
        let entry_digests = self
            .entries
            .iter()
            .map(ProjectionEntry::digest)
            .collect::<Vec<_>>();
        let mut parts = Vec::with_capacity(entry_digests.len().saturating_add(3));
        parts.push(self.schema.as_bytes());
        parts.push(&[u8::from(self.fixture_only)]);
        parts.push(self.p1_1c1_source_commit.as_bytes());
        for digest in &entry_digests {
            parts.push(digest.as_bytes());
        }
        framed_digest(
            b"hepta:intelligence:p1.1c2:review-projection:v1",
            &parts,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionAudit {
    pub entry_count: u32,
    pub review_item_count: u32,
    pub evaluation_candidate_count: u32,
    pub matched_entry_count: u32,
    pub coverage_complete: bool,
    pub bindings_match: bool,
    pub fixture_only: bool,
    pub eligible_for_reviewed_evaluation: bool,
    pub blocked_reasons: Vec<String>,
    pub audit_sha256: Digest32,
}

impl ProjectionAudit {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.matched_entry_count > self.entry_count
            || self.entry_count > self.evaluation_candidate_count
            || self.eligible_for_reviewed_evaluation
                != (self.coverage_complete
                    && self.bindings_match
                    && !self.fixture_only
                    && self.blocked_reasons.is_empty())
        {
            return Err(ContractError::Corrupt(
                "projection audit counts or eligibility are inconsistent".to_string(),
            ));
        }
        if self.audit_sha256 != self.digest() {
            return Err(ContractError::Corrupt(
                "projection audit digest mismatch".to_string(),
            ));
        }
        Ok(())
    }

    fn digest(&self) -> Digest32 {
        let blockers = self.blocked_reasons.join("|");
        framed_digest(
            b"hepta:intelligence:p1.1c2:projection-audit:v1",
            &[
                &self.entry_count.to_be_bytes(),
                &self.review_item_count.to_be_bytes(),
                &self.evaluation_candidate_count.to_be_bytes(),
                &self.matched_entry_count.to_be_bytes(),
                &[u8::from(self.coverage_complete)],
                &[u8::from(self.bindings_match)],
                &[u8::from(self.fixture_only)],
                &[u8::from(self.eligible_for_reviewed_evaluation)],
                blockers.as_bytes(),
            ],
        )
    }
}

#[must_use]
pub fn candidate_projection_digest(
    case: &CorpusCase,
    candidate: &CandidateFixture,
) -> Digest32 {
    let middle_node = candidate.middle_node.as_deref().unwrap_or("-");
    let relevance = [candidate.relevance_grade];
    let citation = [u8::from(candidate.citation_supported)];
    framed_digest(
        b"hepta:intelligence:p1.1c2:evaluation-candidate:v1",
        &[
            case.case_id.as_bytes(),
            case.locale.as_bytes(),
            candidate.candidate_id.as_bytes(),
            &relevance,
            &candidate.lexical_ppm.to_be_bytes(),
            &candidate.vector_ppm.to_be_bytes(),
            &citation,
            &candidate.latency_micros.to_be_bytes(),
            &candidate.token_cost.to_be_bytes(),
            candidate.start_node.as_bytes(),
            middle_node.as_bytes(),
            candidate.goal_node.as_bytes(),
            &candidate.edge1_truth_ppm.to_be_bytes(),
            &candidate.edge1_contradiction_ppm.to_be_bytes(),
            &candidate.edge2_truth_ppm.to_be_bytes(),
            &candidate.edge2_contradiction_ppm.to_be_bytes(),
        ],
    )
}

fn reviews_by_item(
    reviews: &ReviewBatch,
) -> BTreeMap<&str, Vec<&hepta_memory_p1_1c1_qualification::ReviewRecord>> {
    let mut grouped = BTreeMap::<&str, Vec<_>>::new();
    for review in &reviews.reviews {
        grouped.entry(review.item_id.as_str()).or_default().push(review);
    }
    grouped
}

fn split_metadata_and_rows(
    input: &str,
) -> Result<(BTreeMap<String, String>, Vec<&str>), ContractError> {
    let mut metadata = BTreeMap::new();
    let mut rows = Vec::new();
    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if let Some(comment) = line.strip_prefix('#') {
            let (key, value) = comment.trim().split_once('=').ok_or_else(|| {
                ContractError::Invalid("invalid projection metadata line".to_string())
            })?;
            if metadata
                .insert(key.trim().to_string(), value.trim().to_string())
                .is_some()
            {
                return Err(ContractError::Duplicate(format!(
                    "projection metadata key {}",
                    key.trim()
                )));
            }
        } else {
            rows.push(line);
        }
    }
    Ok((metadata, rows))
}

fn required_metadata<'a>(
    metadata: &'a BTreeMap<String, String>,
    key: &str,
) -> Result<&'a str, ContractError> {
    metadata
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| ContractError::Missing(format!("projection metadata key {key}")))
}

fn parse_bool(value: &str) -> Result<bool, ContractError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ContractError::Invalid(format!(
            "projection boolean must be true or false, found {value}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_rejects_duplicate_items() {
        let input = concat!(
            "# schema=hepta.intelligence.p1_1c2.review_projection.v1\n",
            "# fixture_only=true\n",
            "# p1_1c1_source_commit=f961a056ac0a35c1967a934de7cf5bf7ffb92a05\n",
            "item_id\tcase_id\tcandidate_id\tquery_sha256\tcandidate_sha256\n",
            "item\tcase\tcandidate\t",
            "0000000000000000000000000000000000000000000000000000000000000000\t",
            "1111111111111111111111111111111111111111111111111111111111111111\n",
            "item\tcase\tother\t",
            "0000000000000000000000000000000000000000000000000000000000000000\t",
            "2222222222222222222222222222222222222222222222222222222222222222\n"
        );
        assert!(ReviewProjection::parse_tsv(input).is_err());
    }
}

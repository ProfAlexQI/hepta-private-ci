use std::collections::{BTreeMap, BTreeSet};

use crate::{
    ContractError, Digest32, MAX_ADJUDICATION_ROWS, MAX_LOCALES, MAX_REVIEW_ITEMS,
    MAX_REVIEW_ROWS, framed_digest, validate_commit_oid, validate_id, validate_locale,
};

const REVIEW_SCHEMA: &str = "hepta.intelligence.p1_1c1.review_batch.v1";
const ADJUDICATION_SCHEMA: &str =
    "hepta.intelligence.p1_1c1.adjudication_batch.v1";
const REVIEW_COLUMNS: &str = "item_id\tlocale\tquery_sha256\tcandidate_sha256\
\treviewer_commitment\trelevance\tcitation\tcontradiction\tprivacy\
\trationale_sha256";
const ADJUDICATION_COLUMNS: &str = "item_id\tadjudicator_commitment\trelevance\
\tcitation\tcontradiction\tprivacy\tredaction_receipt_sha256\trationale_sha256";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CorpusProvenance {
    SyntheticReviewSeed,
    HumanReviewedV1,
}

impl CorpusProvenance {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyntheticReviewSeed => "synthetic_review_seed",
            Self::HumanReviewedV1 => "human_reviewed_v1",
        }
    }

    fn parse(value: &str) -> Result<Self, ContractError> {
        match value {
            "synthetic_review_seed" => Ok(Self::SyntheticReviewSeed),
            "human_reviewed_v1" => Ok(Self::HumanReviewedV1),
            _ => Err(ContractError::Invalid(format!(
                "unknown review corpus provenance {value}"
            ))),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CitationLabel {
    Unsupported,
    Partial,
    Supported,
}

impl CitationLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
            Self::Partial => "partial",
            Self::Supported => "supported",
        }
    }

    pub(crate) const fn code(self) -> u8 {
        match self {
            Self::Unsupported => 0,
            Self::Partial => 1,
            Self::Supported => 2,
        }
    }

    fn parse(value: &str) -> Result<Self, ContractError> {
        match value {
            "unsupported" => Ok(Self::Unsupported),
            "partial" => Ok(Self::Partial),
            "supported" => Ok(Self::Supported),
            _ => Err(ContractError::Invalid(format!(
                "unknown citation label {value}"
            ))),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ContradictionLabel {
    None,
    Potential,
    Confirmed,
}

impl ContradictionLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Potential => "potential",
            Self::Confirmed => "confirmed",
        }
    }

    pub(crate) const fn code(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Potential => 1,
            Self::Confirmed => 2,
        }
    }

    fn parse(value: &str) -> Result<Self, ContractError> {
        match value {
            "none" => Ok(Self::None),
            "potential" => Ok(Self::Potential),
            "confirmed" => Ok(Self::Confirmed),
            _ => Err(ContractError::Invalid(format!(
                "unknown contradiction label {value}"
            ))),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PrivacyDecision {
    Allow,
    Redact,
    Block,
}

impl PrivacyDecision {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Redact => "redact",
            Self::Block => "block",
        }
    }

    pub(crate) const fn code(self) -> u8 {
        match self {
            Self::Allow => 0,
            Self::Redact => 1,
            Self::Block => 2,
        }
    }

    pub(crate) fn fail_closed_max(left: Self, right: Self) -> Self {
        left.max(right)
    }

    fn parse(value: &str) -> Result<Self, ContractError> {
        match value {
            "allow" => Ok(Self::Allow),
            "redact" => Ok(Self::Redact),
            "block" => Ok(Self::Block),
            _ => Err(ContractError::Invalid(format!(
                "unknown privacy decision {value}"
            ))),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReviewTuple {
    pub relevance: u8,
    pub citation: CitationLabel,
    pub contradiction: ContradictionLabel,
    pub privacy: PrivacyDecision,
}

impl ReviewTuple {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.relevance > 3 {
            return Err(ContractError::Invalid(
                "relevance grade must contain 0..=3".to_string(),
            ));
        }
        Ok(())
    }

    pub(crate) fn conservative(left: Self, right: Self) -> Self {
        Self {
            relevance: left.relevance.min(right.relevance),
            citation: left.citation.min(right.citation),
            contradiction: left.contradiction.max(right.contradiction),
            privacy: PrivacyDecision::fail_closed_max(left.privacy, right.privacy),
        }
    }

    pub(crate) fn digest(&self) -> Digest32 {
        framed_digest(
            b"hepta:intelligence:p1.1c1:review-tuple:v1",
            &[
                &[self.relevance],
                &[self.citation.code()],
                &[self.contradiction.code()],
                &[self.privacy.code()],
            ],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewHeader {
    pub schema: String,
    pub provenance: CorpusProvenance,
    pub reviewed: bool,
    pub human_review_attested: bool,
    pub source_p1_1c_commit: String,
    pub locales: BTreeSet<String>,
}

impl ReviewHeader {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.schema != REVIEW_SCHEMA {
            return Err(ContractError::Invalid(
                "review batch schema mismatch".to_string(),
            ));
        }
        validate_commit_oid(&self.source_p1_1c_commit)?;
        if self.locales.is_empty() || self.locales.len() > MAX_LOCALES {
            return Err(ContractError::Invalid(format!(
                "review locale count must contain 1..={MAX_LOCALES}"
            )));
        }
        for locale in &self.locales {
            validate_locale(locale)?;
        }
        match self.provenance {
            CorpusProvenance::SyntheticReviewSeed => {
                if self.reviewed || self.human_review_attested {
                    return Err(ContractError::Invalid(
                        "synthetic review seed cannot claim reviewed or human-attested status"
                            .to_string(),
                    ));
                }
            }
            CorpusProvenance::HumanReviewedV1 => {
                if !self.reviewed {
                    return Err(ContractError::Invalid(
                        "human-reviewed provenance requires reviewed=true".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    pub(crate) fn digest(&self) -> Digest32 {
        let locale_list = self
            .locales
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join(",");
        framed_digest(
            b"hepta:intelligence:p1.1c1:review-header:v1",
            &[
                self.schema.as_bytes(),
                self.provenance.as_str().as_bytes(),
                &[u8::from(self.reviewed)],
                &[u8::from(self.human_review_attested)],
                self.source_p1_1c_commit.as_bytes(),
                locale_list.as_bytes(),
            ],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewRecord {
    pub item_id: String,
    pub locale: String,
    pub query_sha256: Digest32,
    pub candidate_sha256: Digest32,
    pub reviewer_commitment: Digest32,
    pub labels: ReviewTuple,
    pub rationale_sha256: Digest32,
}

impl ReviewRecord {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.item_id, "review item id")?;
        validate_locale(&self.locale)?;
        self.labels.validate()
    }

    pub(crate) fn digest(&self) -> Digest32 {
        let labels = self.labels.digest();
        framed_digest(
            b"hepta:intelligence:p1.1c1:review-record:v1",
            &[
                self.item_id.as_bytes(),
                self.locale.as_bytes(),
                self.query_sha256.as_bytes(),
                self.candidate_sha256.as_bytes(),
                self.reviewer_commitment.as_bytes(),
                labels.as_bytes(),
                self.rationale_sha256.as_bytes(),
            ],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdjudicationRecord {
    pub item_id: String,
    pub adjudicator_commitment: Digest32,
    pub labels: ReviewTuple,
    pub redaction_receipt_sha256: Option<Digest32>,
    pub rationale_sha256: Digest32,
}

impl AdjudicationRecord {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.item_id, "adjudication item id")?;
        self.labels.validate()?;
        match self.labels.privacy {
            PrivacyDecision::Redact if self.redaction_receipt_sha256.is_none() => {
                return Err(ContractError::Invalid(
                    "redact adjudication requires a redaction receipt digest".to_string(),
                ));
            }
            PrivacyDecision::Allow | PrivacyDecision::Block
                if self.redaction_receipt_sha256.is_some() =>
            {
                return Err(ContractError::Invalid(
                    "redaction receipt is only valid for privacy=redact".to_string(),
                ));
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) fn digest(&self) -> Digest32 {
        let labels = self.labels.digest();
        let redaction = self
            .redaction_receipt_sha256
            .unwrap_or_else(|| Digest32::for_bytes(b"no-redaction"));
        framed_digest(
            b"hepta:intelligence:p1.1c1:adjudication-record:v1",
            &[
                self.item_id.as_bytes(),
                self.adjudicator_commitment.as_bytes(),
                labels.as_bytes(),
                redaction.as_bytes(),
                self.rationale_sha256.as_bytes(),
            ],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewBatch {
    pub header: ReviewHeader,
    pub reviews: Vec<ReviewRecord>,
    pub adjudications: Vec<AdjudicationRecord>,
}

impl ReviewBatch {
    pub fn parse_tsv(
        review_tsv: &str,
        adjudication_tsv: &str,
    ) -> Result<Self, ContractError> {
        let (metadata, review_rows) = split_metadata_and_rows(review_tsv)?;
        let schema = required_metadata(&metadata, "schema")?.to_string();
        let provenance =
            CorpusProvenance::parse(required_metadata(&metadata, "provenance")?)?;
        let reviewed = parse_bool(required_metadata(&metadata, "reviewed")?)?;
        let human_review_attested =
            parse_bool(required_metadata(&metadata, "human_review_attested")?)?;
        let source_p1_1c_commit =
            required_metadata(&metadata, "source_p1_1c_commit")?.to_string();
        let locales = required_metadata(&metadata, "locales")?
            .split(',')
            .map(str::to_string)
            .collect::<BTreeSet<_>>();
        let header = ReviewHeader {
            schema,
            provenance,
            reviewed,
            human_review_attested,
            source_p1_1c_commit,
            locales,
        };
        header.validate()?;

        let mut review_lines = review_rows.into_iter();
        if review_lines.next() != Some(REVIEW_COLUMNS) {
            return Err(ContractError::Invalid(
                "review TSV column header mismatch".to_string(),
            ));
        }
        let mut reviews = Vec::new();
        for line in review_lines {
            if reviews.len() >= MAX_REVIEW_ROWS {
                return Err(ContractError::Invalid(format!(
                    "review row count exceeds {MAX_REVIEW_ROWS}"
                )));
            }
            let columns = line.split('\t').collect::<Vec<_>>();
            if columns.len() != 10 {
                return Err(ContractError::Invalid(format!(
                    "review row must contain 10 columns: {line}"
                )));
            }
            let record = ReviewRecord {
                item_id: columns[0].to_string(),
                locale: columns[1].to_string(),
                query_sha256: Digest32::from_hex(columns[2])?,
                candidate_sha256: Digest32::from_hex(columns[3])?,
                reviewer_commitment: Digest32::from_hex(columns[4])?,
                labels: ReviewTuple {
                    relevance: parse_relevance(columns[5])?,
                    citation: CitationLabel::parse(columns[6])?,
                    contradiction: ContradictionLabel::parse(columns[7])?,
                    privacy: PrivacyDecision::parse(columns[8])?,
                },
                rationale_sha256: Digest32::from_hex(columns[9])?,
            };
            record.validate()?;
            if !header.locales.contains(&record.locale) {
                return Err(ContractError::Invalid(format!(
                    "review locale {} is absent from the header locale set",
                    record.locale
                )));
            }
            reviews.push(record);
        }

        let (adj_metadata, adjudication_rows) =
            split_metadata_and_rows(adjudication_tsv)?;
        if required_metadata(&adj_metadata, "schema")? != ADJUDICATION_SCHEMA {
            return Err(ContractError::Invalid(
                "adjudication batch schema mismatch".to_string(),
            ));
        }
        let mut adjudication_lines = adjudication_rows.into_iter();
        if adjudication_lines.next() != Some(ADJUDICATION_COLUMNS) {
            return Err(ContractError::Invalid(
                "adjudication TSV column header mismatch".to_string(),
            ));
        }
        let mut adjudications = Vec::new();
        for line in adjudication_lines {
            if adjudications.len() >= MAX_ADJUDICATION_ROWS {
                return Err(ContractError::Invalid(format!(
                    "adjudication row count exceeds {MAX_ADJUDICATION_ROWS}"
                )));
            }
            let columns = line.split('\t').collect::<Vec<_>>();
            if columns.len() != 8 {
                return Err(ContractError::Invalid(format!(
                    "adjudication row must contain 8 columns: {line}"
                )));
            }
            let record = AdjudicationRecord {
                item_id: columns[0].to_string(),
                adjudicator_commitment: Digest32::from_hex(columns[1])?,
                labels: ReviewTuple {
                    relevance: parse_relevance(columns[2])?,
                    citation: CitationLabel::parse(columns[3])?,
                    contradiction: ContradictionLabel::parse(columns[4])?,
                    privacy: PrivacyDecision::parse(columns[5])?,
                },
                redaction_receipt_sha256: if columns[6] == "-" {
                    None
                } else {
                    Some(Digest32::from_hex(columns[6])?)
                },
                rationale_sha256: Digest32::from_hex(columns[7])?,
            };
            record.validate()?;
            adjudications.push(record);
        }

        let mut batch = Self {
            header,
            reviews,
            adjudications,
        };
        batch.reviews.sort_by(|left, right| {
            left.item_id
                .cmp(&right.item_id)
                .then_with(|| left.reviewer_commitment.cmp(&right.reviewer_commitment))
        });
        batch
            .adjudications
            .sort_by(|left, right| left.item_id.cmp(&right.item_id));
        batch.validate()?;
        Ok(batch)
    }

    pub fn validate(&self) -> Result<(), ContractError> {
        self.header.validate()?;
        if self.reviews.is_empty() || self.reviews.len() > MAX_REVIEW_ROWS {
            return Err(ContractError::Invalid(format!(
                "review row count must contain 1..={MAX_REVIEW_ROWS}"
            )));
        }
        let grouped = self.grouped_reviews()?;
        if grouped.len() > MAX_REVIEW_ITEMS {
            return Err(ContractError::Invalid(format!(
                "review item count exceeds {MAX_REVIEW_ITEMS}"
            )));
        }
        let item_ids = grouped.keys().copied().collect::<BTreeSet<_>>();
        let mut seen_adjudications = BTreeSet::new();
        for adjudication in &self.adjudications {
            adjudication.validate()?;
            if !item_ids.contains(adjudication.item_id.as_str()) {
                return Err(ContractError::Invalid(format!(
                    "adjudication references unknown item {}",
                    adjudication.item_id
                )));
            }
            if !seen_adjudications.insert(adjudication.item_id.as_str()) {
                return Err(ContractError::Duplicate(format!(
                    "adjudication for item {}",
                    adjudication.item_id
                )));
            }
        }
        Ok(())
    }

    pub fn item_count(&self) -> usize {
        self.reviews.len() / 2
    }

    pub fn reviewer_count(&self) -> usize {
        self.reviews
            .iter()
            .map(|review| review.reviewer_commitment)
            .collect::<BTreeSet<_>>()
            .len()
    }

    pub(crate) fn grouped_reviews(
        &self,
    ) -> Result<BTreeMap<&str, [&ReviewRecord; 2]>, ContractError> {
        let mut grouped = BTreeMap::<&str, Vec<&ReviewRecord>>::new();
        for review in &self.reviews {
            review.validate()?;
            grouped.entry(&review.item_id).or_default().push(review);
        }
        let mut pairs = BTreeMap::new();
        for (item_id, mut reviews) in grouped {
            if reviews.len() != 2 {
                return Err(ContractError::Invalid(format!(
                    "item {item_id} requires exactly two independent reviews"
                )));
            }
            reviews.sort_by_key(|review| review.reviewer_commitment);
            let left = reviews[0];
            let right = reviews[1];
            if left.reviewer_commitment == right.reviewer_commitment {
                return Err(ContractError::Invalid(format!(
                    "item {item_id} reviewers are not independent"
                )));
            }
            if left.locale != right.locale
                || left.query_sha256 != right.query_sha256
                || left.candidate_sha256 != right.candidate_sha256
            {
                return Err(ContractError::Corrupt(format!(
                    "item {item_id} review bindings disagree"
                )));
            }
            pairs.insert(item_id, [left, right]);
        }
        Ok(pairs)
    }

    pub(crate) fn adjudication_for(
        &self,
        item_id: &str,
    ) -> Option<&AdjudicationRecord> {
        self.adjudications
            .iter()
            .find(|record| record.item_id == item_id)
    }

    pub(crate) fn reviewer_set_digest(&self) -> Digest32 {
        let reviewers = self
            .reviews
            .iter()
            .map(|review| review.reviewer_commitment)
            .collect::<BTreeSet<_>>();
        let parts = reviewers
            .iter()
            .map(|digest| digest.as_bytes().as_slice())
            .collect::<Vec<_>>();
        framed_digest(
            b"hepta:intelligence:p1.1c1:reviewer-set:v1",
            &parts,
        )
    }

    pub(crate) fn review_batch_digest(&self) -> Digest32 {
        let digests = self
            .reviews
            .iter()
            .map(ReviewRecord::digest)
            .collect::<Vec<_>>();
        let parts = digests
            .iter()
            .map(|digest| digest.as_bytes().as_slice())
            .collect::<Vec<_>>();
        framed_digest(
            b"hepta:intelligence:p1.1c1:review-batch:v1",
            &parts,
        )
    }

    pub(crate) fn adjudication_batch_digest(&self) -> Digest32 {
        let digests = self
            .adjudications
            .iter()
            .map(AdjudicationRecord::digest)
            .collect::<Vec<_>>();
        let parts = digests
            .iter()
            .map(|digest| digest.as_bytes().as_slice())
            .collect::<Vec<_>>();
        framed_digest(
            b"hepta:intelligence:p1.1c1:adjudication-batch:v1",
            &parts,
        )
    }
}

fn split_metadata_and_rows(
    input: &str,
) -> Result<(BTreeMap<String, String>, Vec<&str>), ContractError> {
    let mut metadata = BTreeMap::new();
    let mut rows = Vec::new();
    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if let Some(comment) = line.strip_prefix('#') {
            let (key, value) = comment.trim().split_once('=').ok_or_else(|| {
                ContractError::Invalid(format!("invalid metadata line {line}"))
            })?;
            if metadata
                .insert(key.trim().to_string(), value.trim().to_string())
                .is_some()
            {
                return Err(ContractError::Duplicate(format!(
                    "metadata key {}",
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
        .ok_or_else(|| ContractError::Missing(format!("metadata key {key}")))
}

fn parse_bool(value: &str) -> Result<bool, ContractError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ContractError::Invalid(format!(
            "boolean metadata must be true or false, found {value}"
        ))),
    }
}

fn parse_relevance(value: &str) -> Result<u8, ContractError> {
    let relevance = value.parse::<u8>().map_err(|_| {
        ContractError::Invalid(format!("invalid relevance grade {value}"))
    })?;
    if relevance > 3 {
        return Err(ContractError::Invalid(
            "relevance grade must contain 0..=3".to_string(),
        ));
    }
    Ok(relevance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail_closed_privacy_order_is_stable() {
        assert_eq!(
            PrivacyDecision::fail_closed_max(
                PrivacyDecision::Allow,
                PrivacyDecision::Redact
            ),
            PrivacyDecision::Redact
        );
        assert_eq!(
            PrivacyDecision::fail_closed_max(
                PrivacyDecision::Redact,
                PrivacyDecision::Block
            ),
            PrivacyDecision::Block
        );
    }

    #[test]
    fn conservative_tuple_never_upgrades_evidence() {
        let left = ReviewTuple {
            relevance: 3,
            citation: CitationLabel::Supported,
            contradiction: ContradictionLabel::None,
            privacy: PrivacyDecision::Allow,
        };
        let right = ReviewTuple {
            relevance: 1,
            citation: CitationLabel::Partial,
            contradiction: ContradictionLabel::Confirmed,
            privacy: PrivacyDecision::Redact,
        };
        let conservative = ReviewTuple::conservative(left, right);
        assert_eq!(conservative.relevance, 1);
        assert_eq!(conservative.citation, CitationLabel::Partial);
        assert_eq!(
            conservative.contradiction,
            ContradictionLabel::Confirmed
        );
        assert_eq!(conservative.privacy, PrivacyDecision::Redact);
    }
}

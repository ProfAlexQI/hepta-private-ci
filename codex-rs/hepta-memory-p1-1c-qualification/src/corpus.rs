use crate::{ContractError, Digest32, checked_ppm, validate_id};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_CASES: usize = 512;
pub const MAX_CANDIDATES_PER_CASE: usize = 64;
pub const MAX_QUERY_BYTES: usize = 4_096;
pub const MAX_ID_BYTES: usize = 128;
pub const MAX_LOCALES: usize = 64;
const EXPECTED_COLUMNS: usize = 17;
const EXPECTED_HEADER: &str = "case_id\tlocale\tquery\tcandidate_id\trelevance_grade\tlexical_ppm\tvector_ppm\tcitation_supported\tlatency_micros\ttoken_cost\tstart_node\tmiddle_node\tgoal_node\tedge1_truth_ppm\tedge1_contradiction_ppm\tedge2_truth_ppm\tedge2_contradiction_ppm";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorpusProvenance {
    SyntheticSeed,
    ReviewedHuman,
}

impl CorpusProvenance {
    fn parse(value: &str) -> Result<Self, ContractError> {
        match value {
            "synthetic_seed" => Ok(Self::SyntheticSeed),
            "reviewed_human" => Ok(Self::ReviewedHuman),
            _ => Err(ContractError::Invalid(format!(
                "unsupported corpus provenance {value}"
            ))),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyntheticSeed => "synthetic_seed",
            Self::ReviewedHuman => "reviewed_human",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusHeader {
    pub schema: String,
    pub corpus_id: String,
    pub version: u32,
    pub provenance: CorpusProvenance,
    pub reviewed: bool,
    pub corpus_sha256: Digest32,
    pub locales: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateFixture {
    pub candidate_id: String,
    pub relevance_grade: u8,
    pub lexical_ppm: u32,
    pub vector_ppm: u32,
    pub citation_supported: bool,
    pub latency_micros: u32,
    pub token_cost: u32,
    pub start_node: String,
    pub middle_node: Option<String>,
    pub goal_node: String,
    pub edge1_truth_ppm: u32,
    pub edge1_contradiction_ppm: u32,
    pub edge2_truth_ppm: u32,
    pub edge2_contradiction_ppm: u32,
}

impl CandidateFixture {
    fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.candidate_id, "candidate ID")?;
        validate_id(&self.start_node, "KG start node")?;
        validate_id(&self.goal_node, "KG goal node")?;
        if let Some(middle_node) = &self.middle_node {
            validate_id(middle_node, "KG middle node")?;
            if middle_node == &self.start_node || middle_node == &self.goal_node {
                return Err(ContractError::Invalid(
                    "two-hop KG middle node must differ from both endpoints".to_string(),
                ));
            }
        } else if self.edge2_truth_ppm != 0 || self.edge2_contradiction_ppm != 0 {
            return Err(ContractError::Invalid(
                "one-hop fixture must keep second-edge values at zero".to_string(),
            ));
        }
        if self.start_node == self.goal_node {
            return Err(ContractError::Invalid(
                "KG fixture endpoints must differ".to_string(),
            ));
        }
        if self.relevance_grade > 3 {
            return Err(ContractError::Invalid(
                "relevance grade must be in 0..=3".to_string(),
            ));
        }
        checked_ppm(self.lexical_ppm, "lexical score")?;
        checked_ppm(self.vector_ppm, "vector score")?;
        checked_ppm(self.edge1_truth_ppm, "first edge truth")?;
        checked_ppm(self.edge1_contradiction_ppm, "first edge contradiction")?;
        checked_ppm(self.edge2_truth_ppm, "second edge truth")?;
        checked_ppm(self.edge2_contradiction_ppm, "second edge contradiction")?;
        if self.latency_micros == 0 {
            return Err(ContractError::Invalid(
                "candidate latency must be non-zero".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusCase {
    pub case_id: String,
    pub locale: String,
    pub query: String,
    pub query_sha256: Digest32,
    pub candidates: Vec<CandidateFixture>,
}

impl CorpusCase {
    fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.case_id, "case ID")?;
        validate_locale(&self.locale)?;
        if self.query.is_empty() || self.query.len() > MAX_QUERY_BYTES {
            return Err(ContractError::Invalid(format!(
                "query must contain 1..={MAX_QUERY_BYTES} UTF-8 bytes"
            )));
        }
        if self.query_sha256 != Digest32::for_bytes(self.query.as_bytes()) {
            return Err(ContractError::Corrupt(
                "query digest does not match query bytes".to_string(),
            ));
        }
        if self.candidates.is_empty() || self.candidates.len() > MAX_CANDIDATES_PER_CASE {
            return Err(ContractError::Limit(format!(
                "case candidate count must contain 1..={MAX_CANDIDATES_PER_CASE}"
            )));
        }
        let mut candidate_ids = BTreeSet::new();
        let mut relevant = 0_usize;
        for candidate in &self.candidates {
            candidate.validate()?;
            if !candidate_ids.insert(candidate.candidate_id.as_str()) {
                return Err(ContractError::Invalid(format!(
                    "duplicate candidate {} in case {}",
                    candidate.candidate_id, self.case_id
                )));
            }
            if candidate.relevance_grade > 0 {
                relevant = relevant.checked_add(1).ok_or(ContractError::Overflow)?;
            }
        }
        if relevant == 0 {
            return Err(ContractError::Invalid(format!(
                "case {} has no relevant candidates",
                self.case_id
            )));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineCorpus {
    pub header: CorpusHeader,
    pub cases: Vec<CorpusCase>,
}

impl OfflineCorpus {
    pub fn parse_tsv(input: &str) -> Result<Self, ContractError> {
        if input.is_empty() {
            return Err(ContractError::Invalid(
                "offline corpus must not be empty".to_string(),
            ));
        }
        let metadata = parse_metadata(input)?;
        let schema = required_metadata(&metadata, "schema")?.to_string();
        let corpus_id = required_metadata(&metadata, "corpus_id")?.to_string();
        validate_id(&corpus_id, "corpus ID")?;
        let version = parse_u32(required_metadata(&metadata, "version")?, "version")?;
        if version == 0 {
            return Err(ContractError::Invalid(
                "corpus version must be non-zero".to_string(),
            ));
        }
        let provenance = CorpusProvenance::parse(required_metadata(&metadata, "provenance")?)?;
        let reviewed = parse_bool(required_metadata(&metadata, "reviewed")?, "reviewed")?;
        if reviewed != matches!(provenance, CorpusProvenance::ReviewedHuman) {
            return Err(ContractError::Invalid(
                "reviewed flag must agree with corpus provenance".to_string(),
            ));
        }

        let mut saw_header = false;
        let mut drafts: BTreeMap<String, CaseDraft> = BTreeMap::new();
        for (line_index, raw_line) in input.lines().enumerate() {
            let line_number = line_index + 1;
            let line = raw_line.trim_end_matches('\r');
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if !saw_header {
                if line != EXPECTED_HEADER {
                    return Err(ContractError::Invalid(format!(
                        "line {line_number} does not match the governed TSV header"
                    )));
                }
                saw_header = true;
                continue;
            }
            let columns: Vec<&str> = line.split('\t').collect();
            if columns.len() != EXPECTED_COLUMNS {
                return Err(ContractError::Invalid(format!(
                    "line {line_number} contains {} columns; expected {EXPECTED_COLUMNS}",
                    columns.len()
                )));
            }
            let row = ParsedRow::parse(&columns, line_number)?;
            let draft = drafts
                .entry(row.case_id.clone())
                .or_insert_with(|| CaseDraft::new(&row));
            draft.push(row)?;
            if drafts.len() > MAX_CASES {
                return Err(ContractError::Limit(format!(
                    "offline corpus exceeds {MAX_CASES} cases"
                )));
            }
        }
        if !saw_header {
            return Err(ContractError::Invalid(
                "offline corpus is missing its TSV header".to_string(),
            ));
        }
        if drafts.is_empty() {
            return Err(ContractError::Invalid(
                "offline corpus contains no cases".to_string(),
            ));
        }

        let mut cases = Vec::with_capacity(drafts.len());
        let mut locales = BTreeSet::new();
        for (_, draft) in drafts {
            let case = draft.finish()?;
            locales.insert(case.locale.clone());
            cases.push(case);
        }
        if locales.is_empty() || locales.len() > MAX_LOCALES {
            return Err(ContractError::Limit(format!(
                "locale count must contain 1..={MAX_LOCALES}"
            )));
        }

        let corpus = Self {
            header: CorpusHeader {
                schema,
                corpus_id,
                version,
                provenance,
                reviewed,
                corpus_sha256: Digest32::for_bytes(input.as_bytes()),
                locales: locales.into_iter().collect(),
            },
            cases,
        };
        corpus.validate()?;
        Ok(corpus)
    }

    pub fn validate(&self) -> Result<(), ContractError> {
        if self.header.schema != crate::P1_1C_SCHEMA {
            return Err(ContractError::Invalid(format!(
                "unexpected P1.1c schema {}",
                self.header.schema
            )));
        }
        if self.cases.is_empty() || self.cases.len() > MAX_CASES {
            return Err(ContractError::Limit(format!(
                "case count must contain 1..={MAX_CASES}"
            )));
        }
        let mut case_ids = BTreeSet::new();
        let mut locales = BTreeSet::new();
        for case in &self.cases {
            case.validate()?;
            if !case_ids.insert(case.case_id.as_str()) {
                return Err(ContractError::Invalid(format!(
                    "duplicate corpus case {}",
                    case.case_id
                )));
            }
            locales.insert(case.locale.clone());
        }
        let expected_locales: Vec<String> = locales.into_iter().collect();
        if expected_locales != self.header.locales {
            return Err(ContractError::Corrupt(
                "corpus locale manifest does not match case locales".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
struct ParsedRow {
    case_id: String,
    locale: String,
    query: String,
    candidate: CandidateFixture,
}

impl ParsedRow {
    fn parse(columns: &[&str], line_number: usize) -> Result<Self, ContractError> {
        let parse_error = |field: &str, error: ContractError| {
            ContractError::Invalid(format!("line {line_number} {field}: {error}"))
        };
        let middle_node = match columns[11] {
            "-" => None,
            value => Some(value.to_string()),
        };
        let candidate = CandidateFixture {
            candidate_id: columns[3].to_string(),
            relevance_grade: parse_u8(columns[4], "relevance grade")?,
            lexical_ppm: parse_u32(columns[5], "lexical score")?,
            vector_ppm: parse_u32(columns[6], "vector score")?,
            citation_supported: parse_bool(columns[7], "citation supported")?,
            latency_micros: parse_u32(columns[8], "latency")?,
            token_cost: parse_u32(columns[9], "token cost")?,
            start_node: columns[10].to_string(),
            middle_node,
            goal_node: columns[12].to_string(),
            edge1_truth_ppm: parse_u32(columns[13], "first edge truth")?,
            edge1_contradiction_ppm: parse_u32(columns[14], "first edge contradiction")?,
            edge2_truth_ppm: parse_u32(columns[15], "second edge truth")?,
            edge2_contradiction_ppm: parse_u32(columns[16], "second edge contradiction")?,
        };
        candidate
            .validate()
            .map_err(|error| parse_error("candidate", error))?;
        Ok(Self {
            case_id: columns[0].to_string(),
            locale: columns[1].to_string(),
            query: columns[2].to_string(),
            candidate,
        })
    }
}

#[derive(Debug)]
struct CaseDraft {
    case_id: String,
    locale: String,
    query: String,
    candidates: Vec<CandidateFixture>,
}

impl CaseDraft {
    fn new(row: &ParsedRow) -> Self {
        Self {
            case_id: row.case_id.clone(),
            locale: row.locale.clone(),
            query: row.query.clone(),
            candidates: Vec::new(),
        }
    }

    fn push(&mut self, row: ParsedRow) -> Result<(), ContractError> {
        if self.locale != row.locale || self.query != row.query {
            return Err(ContractError::Invalid(format!(
                "case {} repeats with different locale or query bytes",
                self.case_id
            )));
        }
        if self.candidates.len() >= MAX_CANDIDATES_PER_CASE {
            return Err(ContractError::Limit(format!(
                "case {} exceeds {MAX_CANDIDATES_PER_CASE} candidates",
                self.case_id
            )));
        }
        self.candidates.push(row.candidate);
        Ok(())
    }

    fn finish(self) -> Result<CorpusCase, ContractError> {
        let case = CorpusCase {
            case_id: self.case_id,
            locale: self.locale,
            query_sha256: Digest32::for_bytes(self.query.as_bytes()),
            query: self.query,
            candidates: self.candidates,
        };
        case.validate()?;
        Ok(case)
    }
}

fn parse_metadata(input: &str) -> Result<BTreeMap<String, String>, ContractError> {
    let mut metadata = BTreeMap::new();
    for line in input.lines() {
        let line = line.trim_end_matches('\r');
        if !line.starts_with('#') {
            continue;
        }
        let value = line.trim_start_matches('#').trim();
        let Some((key, value)) = value.split_once('=') else {
            continue;
        };
        if metadata.insert(key.to_string(), value.to_string()).is_some() {
            return Err(ContractError::Invalid(format!(
                "duplicate corpus metadata key {key}"
            )));
        }
    }
    Ok(metadata)
}

fn required_metadata<'a>(
    metadata: &'a BTreeMap<String, String>,
    key: &str,
) -> Result<&'a str, ContractError> {
    metadata
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| ContractError::Invalid(format!("missing corpus metadata key {key}")))
}

fn parse_u32(value: &str, field: &str) -> Result<u32, ContractError> {
    value
        .parse::<u32>()
        .map_err(|_| ContractError::Invalid(format!("{field} must be an unsigned 32-bit integer")))
}

fn parse_u8(value: &str, field: &str) -> Result<u8, ContractError> {
    value
        .parse::<u8>()
        .map_err(|_| ContractError::Invalid(format!("{field} must be an unsigned 8-bit integer")))
}

fn parse_bool(value: &str, field: &str) -> Result<bool, ContractError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ContractError::Invalid(format!(
            "{field} must be true or false"
        ))),
    }
}

fn validate_locale(locale: &str) -> Result<(), ContractError> {
    if locale.is_empty()
        || locale.len() > 32
        || !locale
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    {
        return Err(ContractError::Invalid(
            "locale must be a bounded BCP-47-style identifier".to_string(),
        ));
    }
    Ok(())
}

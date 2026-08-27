//! Source-span grounding for structured cognitive facts.
//!
//! A verified memory proves that its source text was witnessed. It does not,
//! by itself, prove that every caller-supplied KG entity and relation is
//! supported by that text. This qualification slice adds a strict writer path
//! that requires every fact to carry byte-exact evidence spans from the bound
//! source before the existing atomic memory/KG writer is invoked.
//!
//! The returned receipt is digest-bound but deliberately non-authoritative and
//! non-durable. A later tranche must persist it in the Agent-local append-only
//! ledger and gate the production projection on persisted grounding receipts.
//! Source grounding proves textual support, not external real-world truth.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::str;

use codex_hepta_contracts::Sha256Digest;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::CognitiveAccess;
use crate::CognitiveScope;
use crate::CognitiveStore;
use crate::CognitiveStoreError;
use crate::CognitiveWriteReceipt;
use crate::KgFactSetDraft;
use crate::MemoryDraft;
use crate::MemoryLifecycleState;
use crate::MemoryRevisionDraft;
use crate::MemoryRevisionId;
use crate::MemoryVerification;
use crate::SourceDraft;
use crate::SourceRevisionId;
use crate::StableMemoryId;
use crate::framing::frame_part;

pub const FACT_GROUNDING_SCHEMA_VERSION: u32 = 1;
pub const FACT_GROUNDING_NAMESPACE: &str = "local_qualification_only";
pub const FACT_GROUNDING_CONTRACT: &str = "source_span_grounding_v1";
pub const MAX_FACT_GROUNDING_SPANS_PER_FACT: usize = 4;
pub const MAX_FACT_GROUNDING_SPANS: usize = 768;

const MAX_GROUNDING_SOURCE_BYTES: usize = 256 * 1024;
const MAX_FACT_KEY_BYTES: usize = 256;
const MAX_ENTITY_LABEL_BYTES: usize = 1024;
const MAX_RELATION_BYTES: usize = 128;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GroundedFactKind {
    Entity,
    Relation,
}

impl GroundedFactKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Entity => "entity",
            Self::Relation => "relation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FactEvidenceSpanDraft {
    pub fact_kind: GroundedFactKind,
    pub fact_key: String,
    pub start_byte: u32,
    pub end_byte: u32,
    pub evidence_sha256: Sha256Digest,
}

impl FactEvidenceSpanDraft {
    pub fn new(
        fact_kind: GroundedFactKind,
        fact_key: impl Into<String>,
        start_byte: u32,
        end_byte: u32,
        evidence_sha256: Sha256Digest,
    ) -> Self {
        Self {
            fact_kind,
            fact_key: fact_key.into(),
            start_byte,
            end_byte,
            evidence_sha256,
        }
    }

    pub fn from_source_text(
        fact_kind: GroundedFactKind,
        fact_key: impl Into<String>,
        source_text: &str,
        start_byte: usize,
        end_byte: usize,
    ) -> Result<Self, FactGroundingError> {
        validate_span_range(source_text, start_byte, end_byte)?;
        let start_byte = u32::try_from(start_byte)
            .map_err(|_| FactGroundingError::InvalidSpan("start byte exceeds u32".to_string()))?;
        let end_byte = u32::try_from(end_byte)
            .map_err(|_| FactGroundingError::InvalidSpan("end byte exceeds u32".to_string()))?;
        Ok(Self {
            fact_kind,
            fact_key: fact_key.into(),
            start_byte,
            end_byte,
            evidence_sha256: Sha256Digest::for_bytes(
                &source_text.as_bytes()[start_byte as usize..end_byte as usize],
            ),
        })
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GroundedKgFactSetDraft {
    pub facts: KgFactSetDraft,
    pub evidence: Vec<FactEvidenceSpanDraft>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FactEvidenceSpanReceipt {
    pub fact_kind: GroundedFactKind,
    pub fact_key: String,
    pub evidence_ordinal: u32,
    pub start_byte: u32,
    pub end_byte: u32,
    pub evidence_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FactGroundingReceipt {
    pub schema_version: u32,
    pub namespace: String,
    pub grounding_contract: String,
    pub memory: MemoryRevisionId,
    pub source: SourceRevisionId,
    pub source_content_sha256: Sha256Digest,
    pub fact_set_sha256: Sha256Digest,
    pub fact_identity_sha256: Sha256Digest,
    pub evidence_count: u32,
    pub evidence: Vec<FactEvidenceSpanReceipt>,
    pub durable_persistence: bool,
    pub production_authority: bool,
    pub projection_gate: bool,
    pub receipt_sha256: Sha256Digest,
}

impl FactGroundingReceipt {
    pub fn validate(&self) -> Result<(), FactGroundingError> {
        if self.schema_version != FACT_GROUNDING_SCHEMA_VERSION
            || self.namespace != FACT_GROUNDING_NAMESPACE
            || self.grounding_contract != FACT_GROUNDING_CONTRACT
        {
            return Err(FactGroundingError::Receipt(
                "unsupported fact-grounding schema, namespace, or contract".to_string(),
            ));
        }
        if self.durable_persistence || self.production_authority || self.projection_gate {
            return Err(FactGroundingError::AuthorityBoundary);
        }
        if usize::try_from(self.evidence_count).ok() != Some(self.evidence.len()) {
            return Err(FactGroundingError::Receipt(
                "evidence count does not match receipt spans".to_string(),
            ));
        }
        if self.evidence.len() > MAX_FACT_GROUNDING_SPANS {
            return Err(FactGroundingError::TooManyEvidenceSpans {
                max: MAX_FACT_GROUNDING_SPANS,
            });
        }
        validate_digest(&self.source_content_sha256, "source content digest")?;
        validate_digest(&self.fact_set_sha256, "fact-set digest")?;
        validate_digest(&self.fact_identity_sha256, "fact identity digest")?;
        validate_digest(&self.receipt_sha256, "grounding receipt digest")?;

        let mut previous_identity: Option<(GroundedFactKind, &str)> = None;
        let mut expected_ordinal = 0_u32;
        for span in &self.evidence {
            validate_fact_key(&span.fact_key)?;
            validate_digest(&span.evidence_sha256, "evidence digest")?;
            if span.start_byte >= span.end_byte {
                return Err(FactGroundingError::Receipt(
                    "receipt contains an inverted or empty evidence span".to_string(),
                ));
            }
            let identity = (span.fact_kind, span.fact_key.as_str());
            match previous_identity {
                None if span.evidence_ordinal != 0 => {
                    return Err(FactGroundingError::Receipt(
                        "first receipt evidence ordinal must be zero".to_string(),
                    ));
                }
                Some(previous) if identity == previous => {
                    if span.evidence_ordinal != expected_ordinal {
                        return Err(FactGroundingError::Receipt(
                            "receipt evidence ordinals are not contiguous".to_string(),
                        ));
                    }
                }
                Some(previous) if identity < previous || span.evidence_ordinal != 0 => {
                    return Err(FactGroundingError::Receipt(
                        "receipt evidence ordering is invalid".to_string(),
                    ));
                }
                _ => {}
            }
            previous_identity = Some(identity);
            expected_ordinal = span.evidence_ordinal.checked_add(1).ok_or_else(|| {
                FactGroundingError::Receipt("evidence ordinal overflow".to_string())
            })?;
        }
        if self.receipt_sha256 != grounding_receipt_digest(self) {
            return Err(FactGroundingError::Receipt(
                "grounding receipt digest does not match its contents".to_string(),
            ));
        }
        Ok(())
    }

    pub fn is_qualification_only(&self) -> bool {
        self.namespace == FACT_GROUNDING_NAMESPACE
            && !self.durable_persistence
            && !self.production_authority
            && !self.projection_gate
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GroundedCognitiveWriteReceipt {
    pub write: CognitiveWriteReceipt,
    pub grounding: FactGroundingReceipt,
}

impl GroundedCognitiveWriteReceipt {
    pub fn validate(&self) -> Result<(), FactGroundingError> {
        self.grounding.validate()?;
        if self.write.memory.id != self.grounding.memory
            || self.write.source != self.grounding.source
            || self.write.memory.content_sha256 != self.grounding.source_content_sha256
            || self.write.projection.fact_set_sha256 != self.grounding.fact_set_sha256
        {
            return Err(FactGroundingError::Receipt(
                "grounding receipt is not bound to the cognitive write".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum FactGroundingError {
    #[error("fact-grounding source must be valid UTF-8")]
    NonUtf8Source,
    #[error("fact-grounding source exceeds {max} bytes")]
    SourceTooLarge { max: usize },
    #[error("source and memory revision scopes differ")]
    SourceScopeMismatch,
    #[error("source content does not exactly bind the memory revision")]
    SourceContentMismatch,
    #[error("grounded facts require a verified active memory revision")]
    IneligibleRevision,
    #[error("grounding evidence references unknown {kind:?} fact `{key}`")]
    UnknownFact {
        kind: GroundedFactKind,
        key: String,
    },
    #[error("{kind:?} fact `{key}` has no grounding evidence")]
    MissingEvidence {
        kind: GroundedFactKind,
        key: String,
    },
    #[error("duplicate evidence span for {kind:?} fact `{key}`")]
    DuplicateEvidence {
        kind: GroundedFactKind,
        key: String,
    },
    #[error("{kind:?} fact `{key}` exceeds {max} evidence spans")]
    TooManyEvidenceForFact {
        kind: GroundedFactKind,
        key: String,
        max: usize,
    },
    #[error("fact grounding exceeds {max} total evidence spans")]
    TooManyEvidenceSpans { max: usize },
    #[error("invalid fact evidence span: {0}")]
    InvalidSpan(String),
    #[error("evidence digest does not match source bytes for {kind:?} fact `{key}`")]
    EvidenceDigestMismatch {
        kind: GroundedFactKind,
        key: String,
    },
    #[error("evidence spans do not textually support {kind:?} fact `{key}`")]
    UnsupportedFact {
        kind: GroundedFactKind,
        key: String,
    },
    #[error("fact grounding input is invalid: {0}")]
    Invalid(String),
    #[error("fact grounding receipt is invalid: {0}")]
    Receipt(String),
    #[error("fact grounding receipt crosses its qualification authority boundary")]
    AuthorityBoundary,
}

#[derive(Debug, thiserror::Error)]
pub enum GroundedCognitiveWriteError {
    #[error(transparent)]
    Grounding(#[from] FactGroundingError),
    #[error(transparent)]
    Store(#[from] CognitiveStoreError),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FactIdentity {
    kind: GroundedFactKind,
    key: String,
}

#[derive(Clone, Debug)]
enum FactSupport {
    Entity {
        label: String,
    },
    Relation {
        from_label: String,
        to_label: String,
        relation: String,
    },
}

#[derive(Clone, Debug)]
struct PreparedGrounding {
    source_content_sha256: Sha256Digest,
    fact_identity_sha256: Sha256Digest,
    evidence: Vec<FactEvidenceSpanReceipt>,
}

impl PreparedGrounding {
    fn into_receipt(
        self,
        write: &CognitiveWriteReceipt,
    ) -> Result<FactGroundingReceipt, FactGroundingError> {
        let evidence_count = u32::try_from(self.evidence.len()).map_err(|_| {
            FactGroundingError::TooManyEvidenceSpans {
                max: MAX_FACT_GROUNDING_SPANS,
            }
        })?;
        let mut receipt = FactGroundingReceipt {
            schema_version: FACT_GROUNDING_SCHEMA_VERSION,
            namespace: FACT_GROUNDING_NAMESPACE.to_string(),
            grounding_contract: FACT_GROUNDING_CONTRACT.to_string(),
            memory: write.memory.id.clone(),
            source: write.source.clone(),
            source_content_sha256: self.source_content_sha256,
            fact_set_sha256: write.projection.fact_set_sha256.clone(),
            fact_identity_sha256: self.fact_identity_sha256,
            evidence_count,
            evidence: self.evidence,
            durable_persistence: false,
            production_authority: false,
            projection_gate: false,
            receipt_sha256: Sha256Digest::for_bytes(b"uncomputed"),
        };
        receipt.receipt_sha256 = grounding_receipt_digest(&receipt);
        receipt.validate()?;
        Ok(receipt)
    }
}

impl CognitiveStore {
    /// Qualification-only fact-grounded wrapper around the atomic memory/KG writer.
    pub async fn remember_with_grounded_kg(
        &self,
        access: &CognitiveAccess,
        source: &SourceDraft,
        draft: &MemoryDraft,
        grounded: &GroundedKgFactSetDraft,
    ) -> Result<GroundedCognitiveWriteReceipt, GroundedCognitiveWriteError> {
        validate_source_binding(source, &draft.revision.scope, &draft.revision.content)?;
        if draft.revision.lifecycle != MemoryLifecycleState::Active {
            return Err(FactGroundingError::IneligibleRevision.into());
        }
        validate_fact_eligibility(&draft.revision, &grounded.facts)?;
        let prepared = prepare_grounding(source, grounded)?;
        let write = self
            .remember_with_kg(access, source, draft, &grounded.facts)
            .await?;
        let grounding = prepared.into_receipt(&write)?;
        let receipt = GroundedCognitiveWriteReceipt { write, grounding };
        receipt.validate()?;
        Ok(receipt)
    }

    /// Qualification-only fact-grounded compare-and-swap correction.
    pub async fn correct_with_grounded_kg(
        &self,
        access: &CognitiveAccess,
        memory_id: &StableMemoryId,
        expected_revision: u64,
        source: &SourceDraft,
        draft: &MemoryRevisionDraft,
        grounded: &GroundedKgFactSetDraft,
    ) -> Result<GroundedCognitiveWriteReceipt, GroundedCognitiveWriteError> {
        validate_source_binding(source, &draft.scope, &draft.content)?;
        if draft.verification != MemoryVerification::Verified
            || draft.lifecycle != MemoryLifecycleState::Active
        {
            return Err(FactGroundingError::IneligibleRevision.into());
        }
        validate_fact_eligibility(draft, &grounded.facts)?;
        let prepared = prepare_grounding(source, grounded)?;
        let write = self
            .correct_with_kg(
                access,
                memory_id,
                expected_revision,
                source,
                draft,
                &grounded.facts,
            )
            .await?;
        let grounding = prepared.into_receipt(&write)?;
        let receipt = GroundedCognitiveWriteReceipt { write, grounding };
        receipt.validate()?;
        Ok(receipt)
    }
}

fn prepare_grounding(
    source: &SourceDraft,
    grounded: &GroundedKgFactSetDraft,
) -> Result<PreparedGrounding, FactGroundingError> {
    if source.content.len() > MAX_GROUNDING_SOURCE_BYTES {
        return Err(FactGroundingError::SourceTooLarge {
            max: MAX_GROUNDING_SOURCE_BYTES,
        });
    }
    let source_text =
        str::from_utf8(&source.content).map_err(|_| FactGroundingError::NonUtf8Source)?;
    if grounded.evidence.len() > MAX_FACT_GROUNDING_SPANS {
        return Err(FactGroundingError::TooManyEvidenceSpans {
            max: MAX_FACT_GROUNDING_SPANS,
        });
    }

    let mut supports = BTreeMap::<FactIdentity, FactSupport>::new();
    let mut entity_labels = BTreeMap::<String, String>::new();
    for entity in &grounded.facts.entities {
        let key = canonical_token(&entity.key, MAX_FACT_KEY_BYTES, "entity key")?;
        let label = canonical_text(&entity.label, MAX_ENTITY_LABEL_BYTES, "entity label")?;
        require_semantic_text(&label, "entity label")?;
        if entity_labels.insert(key.clone(), label.clone()).is_some() {
            return Err(FactGroundingError::Invalid(format!(
                "duplicate entity key `{key}`"
            )));
        }
        supports.insert(
            FactIdentity {
                kind: GroundedFactKind::Entity,
                key,
            },
            FactSupport::Entity { label },
        );
    }
    for relation in &grounded.facts.relations {
        let key = canonical_token(&relation.key, MAX_FACT_KEY_BYTES, "relation key")?;
        let from_key = canonical_token(
            &relation.from_entity_key,
            MAX_FACT_KEY_BYTES,
            "relation source key",
        )?;
        let to_key = canonical_token(
            &relation.to_entity_key,
            MAX_FACT_KEY_BYTES,
            "relation target key",
        )?;
        let relation_name =
            canonical_token(&relation.relation, MAX_RELATION_BYTES, "relation predicate")?;
        require_semantic_text(&relation_name, "relation predicate")?;
        let from_label = entity_labels.get(&from_key).cloned().ok_or_else(|| {
            FactGroundingError::Invalid(format!(
                "relation `{key}` references undeclared source entity `{from_key}`"
            ))
        })?;
        let to_label = entity_labels.get(&to_key).cloned().ok_or_else(|| {
            FactGroundingError::Invalid(format!(
                "relation `{key}` references undeclared target entity `{to_key}`"
            ))
        })?;
        if supports
            .insert(
                FactIdentity {
                    kind: GroundedFactKind::Relation,
                    key: key.clone(),
                },
                FactSupport::Relation {
                    from_label,
                    to_label,
                    relation: relation_name,
                },
            )
            .is_some()
        {
            return Err(FactGroundingError::Invalid(format!(
                "duplicate relation key `{key}`"
            )));
        }
    }

    let mut seen = BTreeSet::new();
    let mut counts = BTreeMap::<FactIdentity, usize>::new();
    let mut support_text = BTreeMap::<FactIdentity, String>::new();
    let mut prepared = Vec::with_capacity(grounded.evidence.len());
    for evidence in &grounded.evidence {
        let key = canonical_token(&evidence.fact_key, MAX_FACT_KEY_BYTES, "evidence fact key")?;
        let identity = FactIdentity {
            kind: evidence.fact_kind,
            key,
        };
        if !supports.contains_key(&identity) {
            return Err(FactGroundingError::UnknownFact {
                kind: identity.kind,
                key: identity.key,
            });
        }
        let start = usize::try_from(evidence.start_byte)
            .map_err(|_| FactGroundingError::InvalidSpan("start byte exceeds usize".to_string()))?;
        let end = usize::try_from(evidence.end_byte)
            .map_err(|_| FactGroundingError::InvalidSpan("end byte exceeds usize".to_string()))?;
        validate_span_range(source_text, start, end)?;
        validate_digest(&evidence.evidence_sha256, "evidence digest")?;
        let actual = Sha256Digest::for_bytes(&source.content[start..end]);
        if actual != evidence.evidence_sha256 {
            return Err(FactGroundingError::EvidenceDigestMismatch {
                kind: identity.kind,
                key: identity.key,
            });
        }
        let duplicate_key = (
            identity.clone(),
            evidence.start_byte,
            evidence.end_byte,
            evidence.evidence_sha256.as_str().to_string(),
        );
        if !seen.insert(duplicate_key) {
            return Err(FactGroundingError::DuplicateEvidence {
                kind: identity.kind,
                key: identity.key,
            });
        }
        let count = counts.entry(identity.clone()).or_default();
        *count += 1;
        if *count > MAX_FACT_GROUNDING_SPANS_PER_FACT {
            return Err(FactGroundingError::TooManyEvidenceForFact {
                kind: identity.kind,
                key: identity.key,
                max: MAX_FACT_GROUNDING_SPANS_PER_FACT,
            });
        }
        let normalized = semantic_normalize(&source_text[start..end]);
        if normalized.is_empty() {
            return Err(FactGroundingError::InvalidSpan(
                "evidence span contains no semantic text".to_string(),
            ));
        }
        support_text
            .entry(identity.clone())
            .and_modify(|text| {
                text.push(' ');
                text.push_str(&normalized);
            })
            .or_insert(normalized);
        prepared.push((
            identity,
            evidence.start_byte,
            evidence.end_byte,
            evidence.evidence_sha256.clone(),
        ));
    }

    for (identity, support) in &supports {
        let Some(text) = support_text.get(identity) else {
            return Err(FactGroundingError::MissingEvidence {
                kind: identity.kind,
                key: identity.key.clone(),
            });
        };
        if !support_is_sufficient(text, support) {
            return Err(FactGroundingError::UnsupportedFact {
                kind: identity.kind,
                key: identity.key.clone(),
            });
        }
    }

    prepared.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| left.1.cmp(&right.1))
            .then_with(|| left.2.cmp(&right.2))
            .then_with(|| left.3.as_str().cmp(right.3.as_str()))
    });
    let mut ordinals = BTreeMap::<FactIdentity, u32>::new();
    let evidence = prepared
        .into_iter()
        .map(|(identity, start_byte, end_byte, evidence_sha256)| {
            let ordinal = ordinals.entry(identity.clone()).or_default();
            let receipt = FactEvidenceSpanReceipt {
                fact_kind: identity.kind,
                fact_key: identity.key,
                evidence_ordinal: *ordinal,
                start_byte,
                end_byte,
                evidence_sha256,
            };
            *ordinal = ordinal.checked_add(1).unwrap_or(u32::MAX);
            receipt
        })
        .collect::<Vec<_>>();
    let fact_identity_sha256 = fact_identity_digest(supports.keys());

    Ok(PreparedGrounding {
        source_content_sha256: Sha256Digest::for_bytes(&source.content),
        fact_identity_sha256,
        evidence,
    })
}

fn validate_source_binding(
    source: &SourceDraft,
    scope: &CognitiveScope,
    expected_content: &str,
) -> Result<(), FactGroundingError> {
    if &source.scope != scope {
        return Err(FactGroundingError::SourceScopeMismatch);
    }
    if source.content != expected_content.as_bytes() {
        return Err(FactGroundingError::SourceContentMismatch);
    }
    Ok(())
}

fn validate_fact_eligibility(
    revision: &MemoryRevisionDraft,
    facts: &KgFactSetDraft,
) -> Result<(), FactGroundingError> {
    if (revision.verification != MemoryVerification::Verified
        || revision.lifecycle != MemoryLifecycleState::Active)
        && (!facts.entities.is_empty() || !facts.relations.is_empty())
    {
        return Err(FactGroundingError::IneligibleRevision);
    }
    Ok(())
}

fn validate_span_range(
    source_text: &str,
    start_byte: usize,
    end_byte: usize,
) -> Result<(), FactGroundingError> {
    if start_byte >= end_byte || end_byte > source_text.len() {
        return Err(FactGroundingError::InvalidSpan(format!(
            "range {start_byte}..{end_byte} is outside the source"
        )));
    }
    if !source_text.is_char_boundary(start_byte) || !source_text.is_char_boundary(end_byte) {
        return Err(FactGroundingError::InvalidSpan(
            "range does not align to UTF-8 character boundaries".to_string(),
        ));
    }
    Ok(())
}

fn support_is_sufficient(text: &str, support: &FactSupport) -> bool {
    match support {
        FactSupport::Entity { label } => text.contains(&semantic_normalize(label)),
        FactSupport::Relation {
            from_label,
            to_label,
            relation,
        } => {
            text.contains(&semantic_normalize(from_label))
                && text.contains(&semantic_normalize(to_label))
                && text.contains(&semantic_normalize(relation))
        }
    }
}

fn semantic_normalize(value: &str) -> String {
    let mut normalized = String::with_capacity(value.len());
    let mut pending_space = false;
    for character in value.chars().flat_map(char::to_lowercase) {
        if character.is_alphanumeric() {
            if pending_space && !normalized.is_empty() {
                normalized.push(' ');
            }
            normalized.push(character);
            pending_space = false;
        } else {
            pending_space = true;
        }
    }
    normalized
}

fn canonical_token(
    value: &str,
    max_bytes: usize,
    label: &str,
) -> Result<String, FactGroundingError> {
    let value = canonical_text(value, max_bytes, label)?.to_ascii_lowercase();
    if value.len() > max_bytes {
        return Err(FactGroundingError::Invalid(format!(
            "{label} exceeds {max_bytes} bytes after canonicalization"
        )));
    }
    Ok(value)
}

fn canonical_text(
    value: &str,
    max_bytes: usize,
    label: &str,
) -> Result<String, FactGroundingError> {
    if value.as_bytes().contains(&0) {
        return Err(FactGroundingError::Invalid(format!(
            "{label} contains a NUL byte"
        )));
    }
    let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if value.is_empty() || value.len() > max_bytes {
        return Err(FactGroundingError::Invalid(format!(
            "{label} must contain 1..={max_bytes} bytes"
        )));
    }
    Ok(value)
}

fn require_semantic_text(value: &str, label: &str) -> Result<(), FactGroundingError> {
    if semantic_normalize(value).is_empty() {
        return Err(FactGroundingError::Invalid(format!(
            "{label} contains no semantic characters"
        )));
    }
    Ok(())
}

fn validate_fact_key(value: &str) -> Result<(), FactGroundingError> {
    let canonical = canonical_token(value, MAX_FACT_KEY_BYTES, "fact key")?;
    if canonical != value {
        return Err(FactGroundingError::Receipt(
            "receipt fact keys must be canonical".to_string(),
        ));
    }
    Ok(())
}

fn validate_digest(digest: &Sha256Digest, label: &str) -> Result<(), FactGroundingError> {
    Sha256Digest::parse(digest.as_str().to_string()).map_err(|_| {
        FactGroundingError::Receipt(format!("{label} must be a lowercase SHA-256 digest"))
    })?;
    Ok(())
}

fn fact_identity_digest<'a>(identities: impl Iterator<Item = &'a FactIdentity>) -> Sha256Digest {
    let identities = identities.collect::<Vec<_>>();
    let mut hasher = Sha256::new();
    frame_part(&mut hasher, b"hepta:cognitive:fact-identities:v1");
    frame_part(
        &mut hasher,
        &u64::try_from(identities.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for identity in identities {
        frame_part(&mut hasher, identity.kind.as_str().as_bytes());
        frame_part(&mut hasher, identity.key.as_bytes());
    }
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn grounding_receipt_digest(receipt: &FactGroundingReceipt) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(&mut hasher, b"hepta:cognitive:fact-grounding-receipt:v1");
    frame_part(&mut hasher, &receipt.schema_version.to_be_bytes());
    frame_part(&mut hasher, receipt.namespace.as_bytes());
    frame_part(&mut hasher, receipt.grounding_contract.as_bytes());
    frame_part(&mut hasher, receipt.memory.memory_id.as_str().as_bytes());
    frame_part(&mut hasher, &receipt.memory.revision.to_be_bytes());
    frame_part(&mut hasher, receipt.source.source_id.as_str().as_bytes());
    frame_part(&mut hasher, &receipt.source.revision.to_be_bytes());
    frame_part(
        &mut hasher,
        receipt.source_content_sha256.as_str().as_bytes(),
    );
    frame_part(&mut hasher, receipt.fact_set_sha256.as_str().as_bytes());
    frame_part(
        &mut hasher,
        receipt.fact_identity_sha256.as_str().as_bytes(),
    );
    frame_part(&mut hasher, &receipt.evidence_count.to_be_bytes());
    for span in &receipt.evidence {
        frame_part(&mut hasher, span.fact_kind.as_str().as_bytes());
        frame_part(&mut hasher, span.fact_key.as_bytes());
        frame_part(&mut hasher, &span.evidence_ordinal.to_be_bytes());
        frame_part(&mut hasher, &span.start_byte.to_be_bytes());
        frame_part(&mut hasher, &span.end_byte.to_be_bytes());
        frame_part(&mut hasher, span.evidence_sha256.as_str().as_bytes());
    }
    frame_part(&mut hasher, &[0, 0, 0]);
    Sha256Digest::from_sha256_output(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KgEntityFactDraft;
    use crate::KgRelationFactDraft;
    use crate::LedgerSourceKind;
    use crate::cognitive_test_support::agent_id;
    use crate::cognitive_test_support::layout;
    use tempfile::TempDir;

    fn fact_set() -> KgFactSetDraft {
        KgFactSetDraft {
            entities: vec![
                KgEntityFactDraft {
                    key: "aurora".to_string(),
                    entity_type: "project".to_string(),
                    label: "Project Aurora".to_string(),
                },
                KgEntityFactDraft {
                    key: "rust".to_string(),
                    entity_type: "language".to_string(),
                    label: "Rust".to_string(),
                },
            ],
            relations: vec![KgRelationFactDraft {
                key: "aurora-uses-rust".to_string(),
                from_entity_key: "aurora".to_string(),
                to_entity_key: "rust".to_string(),
                relation: "uses".to_string(),
            }],
        }
    }

    fn span(
        source: &str,
        fact_kind: GroundedFactKind,
        fact_key: &str,
        needle: &str,
    ) -> FactEvidenceSpanDraft {
        let start = source.find(needle).expect("needle");
        FactEvidenceSpanDraft::from_source_text(
            fact_kind,
            fact_key,
            source,
            start,
            start + needle.len(),
        )
        .expect("span")
    }

    fn grounded(source: &str) -> GroundedKgFactSetDraft {
        GroundedKgFactSetDraft {
            facts: fact_set(),
            evidence: vec![
                span(
                    source,
                    GroundedFactKind::Entity,
                    "aurora",
                    "Project Aurora uses Rust",
                ),
                span(
                    source,
                    GroundedFactKind::Entity,
                    "rust",
                    "Project Aurora uses Rust",
                ),
                span(
                    source,
                    GroundedFactKind::Relation,
                    "aurora-uses-rust",
                    "Project Aurora uses Rust",
                ),
            ],
        }
    }

    fn source(text: &str, event_key: &str) -> SourceDraft {
        SourceDraft {
            scope: CognitiveScope::AgentPrivate,
            kind: LedgerSourceKind::ExplicitMemoryDirective,
            event_key: event_key.to_string(),
            content: text.as_bytes().to_vec(),
            observed_at_unix_seconds: 100,
        }
    }

    fn memory(text: &str, stable_key: &str) -> MemoryDraft {
        MemoryDraft {
            stable_key: stable_key.to_string(),
            revision: MemoryRevisionDraft {
                scope: CognitiveScope::AgentPrivate,
                content: text.to_string(),
                verification: MemoryVerification::Verified,
                lifecycle: MemoryLifecycleState::Active,
                valid_from_unix_seconds: 100,
                valid_to_unix_seconds: None,
                citations: Vec::new(),
            },
        }
    }

    #[test]
    fn strict_grounding_accepts_source_bound_entities_and_relation() {
        let text = "Project Aurora uses Rust for deployment.";
        let prepared = prepare_grounding(&source(text, "event:1"), &grounded(text))
            .expect("grounding must validate");
        assert_eq!(prepared.evidence.len(), 3);
        assert_eq!(prepared.evidence[0].evidence_ordinal, 0);
    }

    #[test]
    fn strict_grounding_rejects_missing_unknown_drift_and_utf8_split() {
        let text = "Project Aurora uses Rust for deployment.";
        let mut missing = grounded(text);
        missing
            .evidence
            .retain(|span| span.fact_kind != GroundedFactKind::Relation);
        assert!(matches!(
            prepare_grounding(&source(text, "event:2"), &missing),
            Err(FactGroundingError::MissingEvidence {
                kind: GroundedFactKind::Relation,
                ..
            })
        ));

        let mut unknown = grounded(text);
        unknown.evidence.push(span(
            text,
            GroundedFactKind::Entity,
            "not-declared",
            "Project Aurora",
        ));
        assert!(matches!(
            prepare_grounding(&source(text, "event:3"), &unknown),
            Err(FactGroundingError::UnknownFact { .. })
        ));

        let mut drift = grounded(text);
        drift.evidence[0].evidence_sha256 = Sha256Digest::for_bytes(b"wrong");
        assert!(matches!(
            prepare_grounding(&source(text, "event:4"), &drift),
            Err(FactGroundingError::EvidenceDigestMismatch { .. })
        ));

        let utf8_text = "项目 Aurora uses Rust.";
        let split = GroundedKgFactSetDraft {
            facts: KgFactSetDraft {
                entities: vec![KgEntityFactDraft {
                    key: "aurora".to_string(),
                    entity_type: "project".to_string(),
                    label: "Aurora".to_string(),
                }],
                relations: Vec::new(),
            },
            evidence: vec![FactEvidenceSpanDraft::new(
                GroundedFactKind::Entity,
                "aurora",
                1,
                4,
                Sha256Digest::for_bytes(&utf8_text.as_bytes()[1..4]),
            )],
        };
        assert!(matches!(
            prepare_grounding(&source(utf8_text, "event:5"), &split),
            Err(FactGroundingError::InvalidSpan(_))
        ));
    }

    #[test]
    fn strict_grounding_rejects_textually_unsupported_fact() {
        let source_text = "The deadline is Friday. Project Aurora uses Rust.";
        let unsupported = GroundedKgFactSetDraft {
            facts: fact_set(),
            evidence: vec![
                span(
                    source_text,
                    GroundedFactKind::Entity,
                    "aurora",
                    "The deadline is Friday",
                ),
                span(
                    source_text,
                    GroundedFactKind::Entity,
                    "rust",
                    "Project Aurora uses Rust",
                ),
                span(
                    source_text,
                    GroundedFactKind::Relation,
                    "aurora-uses-rust",
                    "Project Aurora uses Rust",
                ),
            ],
        };
        assert!(matches!(
            prepare_grounding(&source(source_text, "event:6"), &unsupported),
            Err(FactGroundingError::UnsupportedFact {
                kind: GroundedFactKind::Entity,
                ..
            })
        ));
    }

    #[tokio::test]
    async fn grounded_writer_binds_receipt_and_rejects_tampering() {
        let temp = TempDir::new().expect("temp");
        let owner = agent_id(201);
        let store = CognitiveStore::open(&layout(&temp, &owner))
            .await
            .expect("store");
        let text = "Project Aurora uses Rust for deployment.";
        let receipt = store
            .remember_with_grounded_kg(
                &CognitiveAccess::agent_private(owner),
                &source(text, "grounding:event:1"),
                &memory(text, "grounding-memory-1"),
                &grounded(text),
            )
            .await
            .expect("grounded write");
        receipt.validate().expect("receipt");
        assert!(receipt.grounding.is_qualification_only());
        assert_eq!(receipt.write.projection.entity_count, 2);
        assert_eq!(receipt.write.projection.relation_count, 1);

        let mut escalated = receipt.grounding.clone();
        escalated.production_authority = true;
        assert_eq!(
            escalated.validate(),
            Err(FactGroundingError::AuthorityBoundary)
        );
        let mut tampered = receipt.grounding;
        tampered.evidence[0].end_byte -= 1;
        assert!(matches!(
            tampered.validate(),
            Err(FactGroundingError::Receipt(_))
        ));
    }

    #[tokio::test]
    async fn invalid_grounding_does_not_mutate_the_store() {
        let temp = TempDir::new().expect("temp");
        let owner = agent_id(202);
        let store = CognitiveStore::open(&layout(&temp, &owner))
            .await
            .expect("store");
        let text = "Project Aurora uses Rust for deployment.";
        let mut invalid = grounded(text);
        invalid.evidence.pop();
        let result = store
            .remember_with_grounded_kg(
                &CognitiveAccess::agent_private(owner),
                &source(text, "grounding:event:invalid"),
                &memory(text, "grounding-memory-invalid"),
                &invalid,
            )
            .await;
        assert!(matches!(
            result,
            Err(GroundedCognitiveWriteError::Grounding(
                FactGroundingError::MissingEvidence { .. }
            ))
        ));
        let source_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM source_ledger")
            .fetch_one(&store.pool)
            .await
            .expect("source count");
        let memory_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM memory_revisions")
            .fetch_one(&store.pool)
            .await
            .expect("memory count");
        assert_eq!(source_count, 0);
        assert_eq!(memory_count, 0);
    }
}

use serde::Deserialize;
use serde::Serialize;

use super::CONTEXT_MEMORY_FORMATION_QUEUE_SCHEMA_VERSION;
use super::ContextRecallInspection;
use super::privacy_class_is_payload_light;
use super::stable_receipt_hash;
use super::stable_receipt_hash_is_valid;

/// Candidate type for future background memory formation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryFormationCandidateType {
    Fact,
    Task,
    Preference,
    Decision,
    Summary,
    #[default]
    Unknown,
}

impl ContextMemoryFormationCandidateType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fact => "fact",
            Self::Task => "task",
            Self::Preference => "preference",
            Self::Decision => "decision",
            Self::Summary => "summary",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light receipt for a background memory-formation enqueue decision.
///
/// This describes that transcript evidence is eligible for later background
/// processing. It deliberately omits transcript text, memory text, source
/// identifiers, and candidate content.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryFormationReceipt {
    pub candidate_type: ContextMemoryFormationCandidateType,
    pub transcript_span_count: usize,
    pub provenance_span_count: usize,
    pub confidence_basis_points: u32,
    pub idempotency_key_hash: String,
    pub privacy_class: String,
    pub queued_for_background: bool,
    pub production_write: bool,
}

impl ContextMemoryFormationReceipt {
    pub fn has_receipt_integrity(&self) -> bool {
        !self.candidate_type.is_unknown()
            && self.transcript_span_count > 0
            && self.provenance_span_count > 0
            && self.provenance_span_count <= self.transcript_span_count
            && self.confidence_basis_points <= 10_000
            && stable_receipt_hash_is_valid(&self.idempotency_key_hash)
            && privacy_class_is_payload_light(&self.privacy_class)
            && self.queued_for_background
            && !self.production_write
    }
}

/// Receipt-only report for future background memory formation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryFormationReceiptReport {
    pub receipts: Vec<ContextMemoryFormationReceipt>,
}

impl ContextMemoryFormationReceiptReport {
    pub fn from_recall_inspection(inspection: &ContextRecallInspection) -> Self {
        let provenance = inspection.transcript_provenance_summary();
        if provenance.span_count == 0 {
            return Self::default();
        }

        let transcript_span_count = provenance.span_count;
        let provenance_span_count = provenance.span_count;
        let privacy_class = "user_private";
        let receipt_specs = [
            (ContextMemoryFormationCandidateType::Fact, 6400),
            (ContextMemoryFormationCandidateType::Task, 5200),
            (ContextMemoryFormationCandidateType::Preference, 5200),
            (ContextMemoryFormationCandidateType::Decision, 5800),
            (ContextMemoryFormationCandidateType::Summary, 7000),
        ];
        let receipts = receipt_specs
            .into_iter()
            .map(|(candidate_type, confidence_basis_points)| {
                let idempotency_key_hash = stable_receipt_hash(&[
                    "memory_formation",
                    candidate_type.as_str(),
                    &inspection.report.request.session_id.0,
                    &transcript_span_count.to_string(),
                    &provenance_span_count.to_string(),
                ]);
                ContextMemoryFormationReceipt {
                    candidate_type,
                    transcript_span_count,
                    provenance_span_count,
                    confidence_basis_points,
                    idempotency_key_hash,
                    privacy_class: privacy_class.to_string(),
                    queued_for_background: true,
                    production_write: false,
                }
            })
            .collect();

        Self { receipts }
    }

    pub fn has_receipt_integrity(&self) -> bool {
        self.receipts
            .iter()
            .all(ContextMemoryFormationReceipt::has_receipt_integrity)
    }

    pub fn is_empty(&self) -> bool {
        self.receipts.is_empty()
    }
}

/// Operator policy attached to a dry-run memory-formation queue item.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryFormationQueueOperatorPolicy {
    OperatorReviewRequired,
    #[default]
    Unknown,
}

impl ContextMemoryFormationQueueOperatorPolicy {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light write-before queue item for future background memory formation.
///
/// Queue items deliberately carry only counters, controlled classes, and stable
/// hashes. They do not carry candidate text, transcript text, source ids, or a
/// production write instruction.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryFormationQueueItem {
    pub candidate_type: ContextMemoryFormationCandidateType,
    pub transcript_span_count: usize,
    pub provenance_span_count: usize,
    pub confidence_basis_points: u32,
    pub idempotency_key_hash: String,
    pub source_receipt_hash: String,
    pub revocation_key_hash: String,
    pub privacy_class: String,
    pub operator_policy: ContextMemoryFormationQueueOperatorPolicy,
    pub retention_ttl_turns: u32,
    pub queued_for_background: bool,
    pub dry_run_only: bool,
    pub idempotency_enforced: bool,
    pub can_revoke_before_commit: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub hot_path_write: bool,
}

impl ContextMemoryFormationQueueItem {
    fn from_receipt(receipt: &ContextMemoryFormationReceipt) -> Self {
        let source_receipt_hash = stable_receipt_hash(&[
            "memory_formation_queue_source_receipt",
            receipt.candidate_type.as_str(),
            &receipt.idempotency_key_hash,
        ]);
        let revocation_key_hash = stable_receipt_hash(&[
            "memory_formation_queue_revocation",
            receipt.candidate_type.as_str(),
            &receipt.idempotency_key_hash,
        ]);

        Self {
            candidate_type: receipt.candidate_type,
            transcript_span_count: receipt.transcript_span_count,
            provenance_span_count: receipt.provenance_span_count,
            confidence_basis_points: receipt.confidence_basis_points,
            idempotency_key_hash: receipt.idempotency_key_hash.clone(),
            source_receipt_hash,
            revocation_key_hash,
            privacy_class: receipt.privacy_class.clone(),
            operator_policy: ContextMemoryFormationQueueOperatorPolicy::OperatorReviewRequired,
            retention_ttl_turns: 64,
            queued_for_background: receipt.queued_for_background,
            dry_run_only: true,
            idempotency_enforced: true,
            can_revoke_before_commit: true,
            production_write: false,
            graph_write: false,
            hot_path_write: false,
        }
    }

    pub fn has_queue_integrity(&self) -> bool {
        !self.candidate_type.is_unknown()
            && self.transcript_span_count > 0
            && self.provenance_span_count > 0
            && self.provenance_span_count <= self.transcript_span_count
            && self.confidence_basis_points <= 10_000
            && stable_receipt_hash_is_valid(&self.idempotency_key_hash)
            && stable_receipt_hash_is_valid(&self.source_receipt_hash)
            && stable_receipt_hash_is_valid(&self.revocation_key_hash)
            && privacy_class_is_payload_light(&self.privacy_class)
            && !self.operator_policy.is_unknown()
            && self.retention_ttl_turns > 0
            && self.queued_for_background
            && self.dry_run_only
            && self.idempotency_enforced
            && self.can_revoke_before_commit
            && !self.production_write
            && !self.graph_write
            && !self.hot_path_write
    }
}

/// Dry-run queue report for future background memory formation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryFormationQueueReport {
    pub schema_version: u32,
    pub items: Vec<ContextMemoryFormationQueueItem>,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub hot_path_write: bool,
}

impl Default for ContextMemoryFormationQueueReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_FORMATION_QUEUE_SCHEMA_VERSION,
            items: Vec::new(),
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            hot_path_write: false,
        }
    }
}

impl ContextMemoryFormationQueueReport {
    pub fn from_receipts(receipts: &ContextMemoryFormationReceiptReport) -> Self {
        Self {
            items: receipts
                .receipts
                .iter()
                .map(ContextMemoryFormationQueueItem::from_receipt)
                .collect(),
            ..Self::default()
        }
    }

    pub fn has_queue_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_FORMATION_QUEUE_SCHEMA_VERSION
            && self
                .items
                .iter()
                .all(ContextMemoryFormationQueueItem::has_queue_integrity)
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.hot_path_write
    }

    pub fn queued_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.queued_for_background)
            .count()
    }

    pub fn revocable_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.can_revoke_before_commit)
            .count()
    }

    pub fn operator_review_required_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| {
                item.operator_policy
                    == ContextMemoryFormationQueueOperatorPolicy::OperatorReviewRequired
            })
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

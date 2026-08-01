use super::compression::TurnContextAdaptiveBudgetAllocation;
use super::compression::TurnContextCompressionCandidate;
use super::compression::TurnContextCompressionLossCheckStatus;
use super::compression::TurnContextCompressionProtectedTierInvariant;
use super::compression::TurnContextCompressionStage;
use super::decision::TurnContextDecisionEntry;
use super::manifest::TurnContextManifestEntry;
use super::memory::TurnContextMemoryFormationReceipt;
use super::memory::TurnContextMemoryTaxonomyBucket;
use super::memory::TurnContextMemoryTemporalFact;
use super::recall::TurnContextRecallSelectedSnippetEnvelope;
use super::recall::TurnContextRecallSelectionSummary;

/// Returns the stable 16-hex replay hash used by turn-context manifest
/// payload-light fields. This hash is deterministic for replay/debug
/// comparison only; it is intentionally not a cryptographic trust digest.
pub fn stable_turn_context_manifest_replay_hash(value: &str) -> String {
    let mut hash = StableManifestReplayHash::new();
    hash.update_str(value);
    hash.finish()
}

/// Backwards-compatible name for entry text hashes. New code that needs to
/// emphasize the trust boundary should call
/// [`stable_turn_context_manifest_replay_hash`] instead.
pub fn stable_turn_context_manifest_text_hash(value: &str) -> String {
    stable_turn_context_manifest_replay_hash(value)
}

pub(super) fn compute_decision_ledger_hash(entries: &[TurnContextDecisionEntry]) -> String {
    let mut hash = StableManifestReplayHash::new();
    hash.update_vec_decisions(entries);
    hash.finish()
}

pub(super) struct StableManifestReplayHash(u64);

impl StableManifestReplayHash {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;

    pub(super) fn new() -> Self {
        Self(Self::OFFSET)
    }

    pub(super) fn update_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
        self.0 ^= 0xff;
        self.0 = self.0.wrapping_mul(Self::PRIME);
    }

    pub(super) fn update_str(&mut self, value: &str) {
        self.update_bytes(value.as_bytes());
    }

    pub(super) fn update_bool(&mut self, value: bool) {
        self.update_str(if value { "true" } else { "false" });
    }

    pub(super) fn update_u32(&mut self, value: u32) {
        self.update_str(&value.to_string());
    }

    pub(super) fn update_option_u32(&mut self, value: Option<u32>) {
        match value {
            Some(value) => {
                self.update_str("some");
                self.update_u32(value);
            }
            None => self.update_str("none"),
        }
    }

    pub(super) fn update_option_str(&mut self, value: Option<&str>) {
        match value {
            Some(value) => {
                self.update_str("some");
                self.update_str(value);
            }
            None => self.update_str("none"),
        }
    }

    pub(super) fn update_vec_str(&mut self, values: &[String]) {
        self.update_u32(u32::try_from(values.len()).unwrap_or(u32::MAX));
        for value in values {
            self.update_str(value);
        }
    }

    pub(super) fn update_vec_entries(&mut self, entries: &[TurnContextManifestEntry]) {
        self.update_u32(u32::try_from(entries.len()).unwrap_or(u32::MAX));
        let include_tiers = entries.iter().any(|entry| !entry.tier.is_unknown());
        for entry in entries {
            self.update_str(&entry.role);
            if include_tiers {
                self.update_str(entry.tier.as_str());
            }
            self.update_str(&entry.source);
            self.update_str(&entry.replay_key);
            self.update_str(&entry.text_hash);
            self.update_u32(entry.estimated_tokens);
        }
    }

    pub(super) fn update_vec_decisions(&mut self, entries: &[TurnContextDecisionEntry]) {
        self.update_u32(u32::try_from(entries.len()).unwrap_or(u32::MAX));
        for entry in entries {
            self.update_str(&entry.source);
            self.update_str(&entry.decision);
            if let Some(reason_hash) = &entry.reason_hash {
                self.update_str("some");
                self.update_str(reason_hash);
            } else {
                self.update_str("none");
            }
        }
    }

    pub(super) fn update_recall_selection(
        &mut self,
        recall_selection: Option<&TurnContextRecallSelectionSummary>,
    ) {
        let Some(recall_selection) = recall_selection else {
            self.update_str("none");
            return;
        };
        self.update_str("some");
        self.update_u32(recall_selection.returned_source_count);
        self.update_u32(recall_selection.selected_source_count);
        self.update_u32(recall_selection.ranked_source_count);
        self.update_u32(recall_selection.returned_unselected_source_count);
        self.update_bool(recall_selection.source_diversity_met);
        self.update_u32(recall_selection.source_diversity_target);
        self.update_u32(recall_selection.max_per_source);
        self.update_u32(recall_selection.ranked_item_count);
        self.update_u32(recall_selection.omitted_by_budget_count);
        self.update_u32(recall_selection.memory_control_omitted_count);
        self.update_u32(recall_selection.low_trust_ranked_item_count);
        self.update_u32(recall_selection.low_recency_ranked_item_count);
    }

    pub(super) fn update_recall_selected_snippets(
        &mut self,
        envelope: Option<&TurnContextRecallSelectedSnippetEnvelope>,
    ) {
        let Some(envelope) = envelope else {
            self.update_str("none");
            return;
        };
        self.update_str("some");
        self.update_u32(envelope.version);
        self.update_u32(envelope.max_snippets);
        self.update_u32(envelope.max_snippet_chars);
        self.update_u32(envelope.selected_snippet_count);
        self.update_u32(envelope.omitted_snippet_count);
        self.update_u32(envelope.redacted_snippet_count);
        self.update_u32(envelope.truncated_snippet_count);
        self.update_u32(u32::try_from(envelope.snippets.len()).unwrap_or(u32::MAX));
        for snippet in &envelope.snippets {
            self.update_str(&snippet.snippet_hash);
            self.update_str(&snippet.text);
            self.update_u32(snippet.estimated_tokens);
            self.update_bool(snippet.redacted);
            self.update_bool(snippet.truncated);
        }
        self.update_bool(envelope.safety.ready_for_shadow_handoff);
        self.update_bool(envelope.safety.bounded);
        self.update_bool(envelope.safety.origin_identifiers_exposed);
        self.update_bool(envelope.safety.raw_ranked_payload_exposed);
        self.update_bool(envelope.safety.rank_explanation_exposed);
        self.update_bool(envelope.safety.control_marker_exposed);
        self.update_bool(envelope.safety.query_payload_exposed);
        self.update_bool(envelope.safety.per_origin_list_exposed);
    }

    pub(super) fn update_memory_taxonomy(&mut self, buckets: &[TurnContextMemoryTaxonomyBucket]) {
        if buckets.is_empty() {
            return;
        }
        self.update_u32(u32::try_from(buckets.len()).unwrap_or(u32::MAX));
        for bucket in buckets {
            self.update_str(bucket.class.as_str());
            self.update_u32(bucket.source_count);
            self.update_u32(bucket.returned_count);
            self.update_u32(bucket.available_count);
            self.update_u32(bucket.omitted_count);
            self.update_u32(bucket.provenance_span_count);
        }
    }

    pub(super) fn update_memory_formation_receipts(
        &mut self,
        receipts: &[TurnContextMemoryFormationReceipt],
    ) {
        if receipts.is_empty() {
            return;
        }
        self.update_u32(u32::try_from(receipts.len()).unwrap_or(u32::MAX));
        for receipt in receipts {
            self.update_str(receipt.candidate_type.as_str());
            self.update_u32(receipt.transcript_span_count);
            self.update_u32(receipt.provenance_span_count);
            self.update_u32(receipt.confidence_basis_points);
            self.update_str(&receipt.idempotency_key_hash);
            self.update_str(&receipt.privacy_class);
            self.update_bool(receipt.queued_for_background);
            self.update_bool(receipt.production_write);
        }
    }

    pub(super) fn update_memory_temporal_facts(&mut self, facts: &[TurnContextMemoryTemporalFact]) {
        if facts.is_empty() {
            return;
        }
        self.update_u32(u32::try_from(facts.len()).unwrap_or(u32::MAX));
        for fact in facts {
            self.update_str(fact.fact_type.as_str());
            self.update_str(&fact.entity_hash);
            self.update_u32(fact.provenance_span_count);
            self.update_u32(fact.valid_from_sequence);
            if let Some(sequence) = fact.invalid_at_sequence {
                self.update_str("some");
                self.update_u32(sequence);
            } else {
                self.update_str("none");
            }
            self.update_u32(fact.confidence_basis_points);
            if let Some(supersedes_fact_hash) = &fact.supersedes_fact_hash {
                self.update_str("some");
                self.update_str(supersedes_fact_hash);
            } else {
                self.update_str("none");
            }
            self.update_str(&fact.privacy_class);
            self.update_bool(fact.dry_run_only);
            self.update_bool(fact.production_write);
        }
    }

    pub(super) fn update_compression_candidates(
        &mut self,
        candidates: &[TurnContextCompressionCandidate],
    ) {
        if candidates.is_empty() {
            return;
        }
        self.update_u32(u32::try_from(candidates.len()).unwrap_or(u32::MAX));
        for candidate in candidates {
            self.update_str(candidate.kind.as_str());
            self.update_str(candidate.tier.as_str());
            self.update_str(&candidate.source_id);
            self.update_u32(candidate.input_tokens);
            self.update_u32(candidate.estimated_output_tokens);
            self.update_u32(candidate.affected_entries);
            self.update_str(candidate.not_executed_reason.as_str());
        }
    }

    pub(super) fn update_adaptive_budget_allocations(
        &mut self,
        allocations: &[TurnContextAdaptiveBudgetAllocation],
    ) {
        if allocations.is_empty() {
            return;
        }
        self.update_u32(u32::try_from(allocations.len()).unwrap_or(u32::MAX));
        for allocation in allocations {
            self.update_str(allocation.tier.as_str());
            self.update_str(&allocation.source_id);
            self.update_str(&allocation.budget_class);
            self.update_u32(allocation.input_tokens);
            self.update_u32(allocation.reserve_tokens);
            self.update_u32(allocation.proposed_budget_tokens);
            self.update_u32(allocation.overflow_tokens);
            self.update_option_u32(allocation.omit_priority);
            match allocation.compression_kind {
                Some(kind) => {
                    self.update_str("some");
                    self.update_str(kind.as_str());
                }
                None => self.update_str("none"),
            }
            self.update_option_u32(allocation.estimated_compressed_tokens);
            self.update_str(allocation.current_heuristic_action.as_str());
            self.update_str(allocation.proposed_action.as_str());
            self.update_bool(allocation.would_drop);
            self.update_bool(allocation.would_compress);
        }
    }

    pub(super) fn update_compression_stages(&mut self, stages: &[TurnContextCompressionStage]) {
        if stages.is_empty() {
            return;
        }
        self.update_u32(u32::try_from(stages.len()).unwrap_or(u32::MAX));
        for stage in stages {
            self.update_str(stage.kind.as_str());
            self.update_u32(stage.input_tokens);
            self.update_u32(stage.output_tokens);
            self.update_u32(stage.affected_entries);
            self.update_option_str(
                stage
                    .loss_check_status
                    .map(TurnContextCompressionLossCheckStatus::as_str),
            );
            self.update_option_str(stage.rollback_source_text_hash.as_deref());
            self.update_option_str(
                stage
                    .protected_tier_invariant
                    .map(TurnContextCompressionProtectedTierInvariant::as_str),
            );
        }
    }

    pub(super) fn finish(self) -> String {
        format!("{:016x}", self.0)
    }
}

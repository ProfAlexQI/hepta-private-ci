use crate::StoreSnapshot;
use hepta_core::ContextBudget;
use hepta_core::ContextMemoryFormationQueueReport;
use hepta_core::ContextMemoryFormationReceiptReport;
use hepta_core::ContextMemoryNamespacePolicyReport;
use hepta_core::ContextMemoryTaxonomyReport;
use hepta_core::ContextMemoryTemporalFactGraphReport;
use hepta_core::ContextMemoryTemporalFactReport;
use hepta_core::ContextMemoryTemporalGraphShadowReplayReport;
use hepta_core::ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport;
use hepta_core::ContextMemoryTemporalGraphShadowStoreReport;
use hepta_core::ContextMemoryTemporalGraphShadowTraversalDiffReport;
use hepta_core::ContextMemoryTemporalGraphShadowTraversalQualityReport;
use hepta_core::ContextMemoryWriteChainReadinessReport;
use hepta_core::ContextMemoryWriteChainReceiptFreshnessReport;
use hepta_core::ContextRecallAvailability;
use hepta_core::ContextRecallBundle;
use hepta_core::ContextRecallCoverage;
use hepta_core::ContextRecallInspection;
use hepta_core::ContextRecallLimitPressure;
use hepta_core::ContextRecallOmissionCounts;
use hepta_core::ContextRecallReport;
use hepta_core::ContextRecallRequest;
use hepta_core::ContextRecallSourceAvailability;
use hepta_core::ContextRecallTranscriptProvenanceSummary;
use hepta_core::MemoryQuery;
use hepta_core::MemoryQueryReport;
use hepta_core::MemoryScope;
use hepta_core::QueryReportCoverage;
use hepta_core::QueryReportLimitPressure;
use hepta_core::TranscriptQuery;
use hepta_core::TranscriptQueryReport;

use super::query::memory_records_matching_recall_query;
use super::query::transcript_query_hits;
use super::ranking::ranked_recall_items;

impl StoreSnapshot {
    /// Builds the portable memory query report directly from the snapshot.
    pub fn search_report(&self, query: &MemoryQuery) -> MemoryQueryReport {
        let (matched, omitted_control_count) =
            memory_records_matching_recall_query(&self.memories, &query.text);
        let matched_count = matched.len();
        let mut hits = matched;
        hits.truncate(query.limit);

        MemoryQueryReport::from_hits_with_omitted_control_count(
            query.clone(),
            matched_count,
            hits,
            omitted_control_count,
        )
    }

    /// Builds the compact returned-vs-matched memory query coverage summary.
    pub fn search_coverage(&self, query: &MemoryQuery) -> QueryReportCoverage {
        self.search_report(query).coverage()
    }

    /// Builds the compact omission-focused memory query pressure summary.
    pub fn search_limit_pressure(&self, query: &MemoryQuery) -> QueryReportLimitPressure {
        self.search_report(query).limit_pressure()
    }

    /// Builds the portable transcript query report directly from the snapshot.
    ///
    /// Returned hits are newest-first so bounded query recall favors fresh
    /// transcript evidence over older matches.
    pub fn transcript_search_report(&self, query: &TranscriptQuery) -> TranscriptQueryReport {
        let (matched_count, hits) = transcript_query_hits(&self.transcripts, query);

        TranscriptQueryReport::from_hits(query.clone(), matched_count, hits)
    }

    /// Builds the compact returned-vs-matched transcript query coverage
    /// summary.
    pub fn transcript_search_coverage(&self, query: &TranscriptQuery) -> QueryReportCoverage {
        self.transcript_search_report(query).coverage()
    }

    /// Builds the compact omission-focused transcript query pressure summary.
    pub fn transcript_search_limit_pressure(
        &self,
        query: &TranscriptQuery,
    ) -> QueryReportLimitPressure {
        self.transcript_search_report(query).limit_pressure()
    }

    fn recall_context_parts(
        &self,
        request: &ContextRecallRequest,
    ) -> (
        ContextRecallBundle,
        ContextRecallAvailability,
        ContextRecallSourceAvailability,
        usize,
    ) {
        let mut recent_entries = self
            .transcripts
            .iter()
            .filter(|entry| entry.session_id == request.session_id)
            .cloned()
            .collect::<Vec<_>>();
        recent_entries.sort_by_key(|entry| entry.sequence);
        let total_recent_entry_count = recent_entries.len();
        if recent_entries.len() > request.recent_window_limit {
            recent_entries =
                recent_entries.split_off(recent_entries.len() - request.recent_window_limit);
        }

        let transcript_query = request.transcript_query();
        let (total_transcript_match_count, transcript_hits) = if request.has_query_text() {
            transcript_query_hits(&self.transcripts, &transcript_query)
        } else {
            (0, Vec::new())
        };

        let memory_query = request.memory_query();
        let (memory_hits, memory_control_omitted_count) =
            memory_records_matching_recall_query(&self.memories, &memory_query.text);
        let total_memory_match_count = memory_hits.len();
        let total_durable_memory_match_count = memory_hits
            .iter()
            .filter(|record| record.scope == MemoryScope::LongTerm)
            .count();
        let total_summary_memory_match_count = memory_hits
            .iter()
            .filter(|record| record.scope == MemoryScope::Session)
            .count();
        let mut limited_memory_hits = memory_hits;
        limited_memory_hits.truncate(memory_query.limit);

        let durable_memory_hits = limited_memory_hits
            .iter()
            .filter(|record| record.scope == MemoryScope::LongTerm)
            .cloned()
            .collect();
        let summary_hits = limited_memory_hits
            .into_iter()
            .filter(|record| record.scope == MemoryScope::Session)
            .collect();

        let mut bundle = ContextRecallBundle {
            request: request.clone(),
            recent_entries,
            transcript_hits,
            durable_memory_hits,
            summary_hits,
            active_topic_sessions: vec![],
            active_neurons: Vec::new(),
            budget: ContextBudget::from_request(request),
            ranked_items: Vec::new(),
            omitted_by_budget: 0,
            truncated: total_transcript_match_count > transcript_query.limit
                || total_memory_match_count > memory_query.limit,
        };
        let (ranked_items, omitted_by_budget) = ranked_recall_items(&bundle);
        bundle.ranked_items = ranked_items;
        bundle.omitted_by_budget = omitted_by_budget;

        (
            bundle,
            ContextRecallAvailability {
                total_recent_entry_count,
                total_transcript_match_count,
                total_memory_match_count,
            },
            ContextRecallSourceAvailability {
                recent_entry_count: total_recent_entry_count,
                transcript_match_count: total_transcript_match_count,
                durable_memory_match_count: total_durable_memory_match_count,
                summary_memory_match_count: total_summary_memory_match_count,
            },
            memory_control_omitted_count,
        )
    }

    /// Builds a portable reference recall bundle directly from the snapshot.
    ///
    /// This mirrors the lightweight in-memory search semantics used by the
    /// store: transcript hits are session-scoped and query-driven, while memory
    /// hits use simple substring matching across the snapshot's memory records
    /// after filtering explicit recall control records.
    pub fn recall_context(&self, request: &ContextRecallRequest) -> ContextRecallBundle {
        self.recall_context_parts(request).0
    }

    /// Builds the compact pre-limit recall availability summary for
    /// `request` without carrying returned item payloads.
    pub fn recall_context_availability(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextRecallAvailability {
        self.recall_context_parts(request).1
    }

    /// Builds the compact pre-limit recall availability summary with
    /// durable-memory and session-summary matches split into separate counts.
    pub fn recall_context_source_availability(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextRecallSourceAvailability {
        self.recall_context_parts(request).2
    }

    /// Builds the payload-light recall report for `request` without embedding
    /// the full transcript and memory payloads.
    pub fn recall_context_report(&self, request: &ContextRecallRequest) -> ContextRecallReport {
        self.recall_context(request).report()
    }

    /// Builds a payload-light recall inspection view that includes pre-limit
    /// availability counts for recent entries, transcript matches, and memory
    /// matches.
    pub fn recall_context_inspection(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextRecallInspection {
        let (bundle, availability, _, _) = self.recall_context_parts(request);

        bundle.inspection(availability)
    }

    /// Builds the compact transcript-provenance summary for `request`
    /// without carrying the individual transcript span refs.
    pub fn recall_context_transcript_provenance_summary(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextRecallTranscriptProvenanceSummary {
        self.recall_context_inspection(request)
            .transcript_provenance_summary()
    }

    /// Builds a payload-light recall coverage summary for `request`.
    pub fn recall_context_coverage(&self, request: &ContextRecallRequest) -> ContextRecallCoverage {
        self.recall_context_inspection(request).coverage()
    }

    /// Builds a compact omission summary for recall sources and totals.
    pub fn recall_context_omission_counts(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextRecallOmissionCounts {
        self.recall_context_coverage(request).omission_counts()
    }

    /// Builds a compact limit-pressure summary for recall sources and totals.
    pub fn recall_context_limit_pressure(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextRecallLimitPressure {
        self.recall_context_coverage(request).limit_pressure()
    }

    /// Builds a payload-light taxonomy report that maps recall counts into
    /// semantic, episodic, control, and transcript buckets.
    pub fn recall_context_memory_taxonomy_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryTaxonomyReport {
        let (bundle, availability, source_availability, memory_control_omitted_count) =
            self.recall_context_parts(request);
        bundle
            .inspection(availability)
            .memory_taxonomy_report(&source_availability, memory_control_omitted_count)
    }

    /// Builds the fixed namespace/block shadow policy without inspecting or
    /// writing memory payloads.
    pub fn context_memory_namespace_policy_report(&self) -> ContextMemoryNamespacePolicyReport {
        ContextMemoryNamespacePolicyReport::seeded()
    }

    /// Builds the fixed namespace write-chain readiness/readback surface
    /// without inspecting or writing memory payloads.
    pub fn context_memory_write_chain_readiness_report(
        &self,
    ) -> ContextMemoryWriteChainReadinessReport {
        ContextMemoryWriteChainReadinessReport::from_namespace_policy(
            &self.context_memory_namespace_policy_report(),
        )
    }

    /// Builds the fixed namespace write-chain projected receipt
    /// freshness/digest surface without recording or persisting receipts.
    pub fn context_memory_write_chain_receipt_freshness_report(
        &self,
    ) -> ContextMemoryWriteChainReceiptFreshnessReport {
        ContextMemoryWriteChainReceiptFreshnessReport::from_readiness(
            &self.context_memory_write_chain_readiness_report(),
        )
    }

    /// Builds receipt-only background memory-formation metadata for transcript
    /// evidence without writing or forming production memory.
    pub fn recall_context_memory_formation_receipt_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryFormationReceiptReport {
        self.recall_context_inspection(request)
            .memory_formation_receipt_report()
    }

    /// Builds the dry-run background memory-formation queue from receipt
    /// metadata without writing production memory.
    pub fn recall_context_memory_formation_queue_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryFormationQueueReport {
        self.recall_context_inspection(request)
            .memory_formation_queue_report()
    }

    /// Builds temporal fact dry-run metadata for transcript evidence without
    /// writing graph facts or production memory.
    pub fn recall_context_memory_temporal_fact_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryTemporalFactReport {
        self.recall_context_inspection(request)
            .memory_temporal_fact_report()
    }

    /// Builds temporal fact graph dry-run topology from temporal facts without
    /// graph writes or production memory writes.
    pub fn recall_context_memory_temporal_fact_graph_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryTemporalFactGraphReport {
        self.recall_context_inspection(request)
            .memory_temporal_fact_graph_report()
    }

    /// Builds an approval-gated temporal graph shadow store skeleton from the
    /// dry-run graph topology without persisting graph facts or receipts.
    pub fn recall_context_memory_temporal_graph_shadow_store_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryTemporalGraphShadowStoreReport {
        ContextMemoryTemporalGraphShadowStoreReport::from_fact_graph(
            &self.recall_context_memory_temporal_fact_graph_report(request),
        )
    }

    /// Builds a payload-light shadow WAL replay evidence surface from the
    /// temporal graph shadow store without recording or persisting receipts.
    pub fn recall_context_memory_temporal_graph_shadow_replay_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryTemporalGraphShadowReplayReport {
        ContextMemoryTemporalGraphShadowReplayReport::from_shadow_store(
            &self.recall_context_memory_temporal_graph_shadow_store_report(request),
        )
    }

    /// Builds a payload-light temporal graph retrieval/traversal diff surface
    /// from replay evidence without graph traversal activation or reranking.
    pub fn recall_context_memory_temporal_graph_shadow_traversal_diff_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryTemporalGraphShadowTraversalDiffReport {
        ContextMemoryTemporalGraphShadowTraversalDiffReport::from_shadow_replay(
            &self.recall_context_memory_temporal_graph_shadow_replay_report(request),
        )
    }

    /// Builds a payload-light temporal graph traversal quality/SLO surface
    /// from diff evidence without graph traversal activation or reranking.
    pub fn recall_context_memory_temporal_graph_shadow_traversal_quality_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryTemporalGraphShadowTraversalQualityReport {
        ContextMemoryTemporalGraphShadowTraversalQualityReport::from_traversal_diff(
            &self.recall_context_memory_temporal_graph_shadow_traversal_diff_report(request),
        )
    }

    /// Builds a payload-light temporal graph retrieval canary guard surface
    /// from traversal quality without opening routes or writing rollback state.
    pub fn recall_context_memory_temporal_graph_shadow_retrieval_canary_guard_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport {
        ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport::from_traversal_quality(
            &self.recall_context_memory_temporal_graph_shadow_traversal_quality_report(request),
        )
    }
}

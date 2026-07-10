use crate::InMemoryStore;
use hepta_core::ContextMemoryFormationQueueReport;
use hepta_core::ContextMemoryFormationReceiptReport;
use hepta_core::ContextMemoryNamespacePolicyReport;
use hepta_core::ContextMemoryTaxonomyReport;
use hepta_core::ContextMemoryTemporalFactGraphReport;
use hepta_core::ContextMemoryTemporalFactReport;
use hepta_core::ContextMemoryTemporalGraphShadowReplayReport;
use hepta_core::ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport;
use hepta_core::ContextMemoryTemporalGraphShadowRetrievalPromotionReadinessReport;
use hepta_core::ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport;
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
use hepta_core::QueryReportCoverage;
use hepta_core::QueryReportLimitPressure;
use hepta_core::TranscriptQuery;
use hepta_core::TranscriptQueryReport;

use super::query::memory_records_matching_recall_query;
use super::query::transcript_query_hits;

impl InMemoryStore {
    pub fn recall_context(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallBundle, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context(&request))
    }

    pub fn recall_context_availability(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallAvailability, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context_availability(&request))
    }

    pub fn recall_context_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallReport, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context_report(&request))
    }

    pub fn recall_context_source_availability(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallSourceAvailability, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .recall_context_source_availability(&request))
    }

    pub fn recall_context_inspection(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallInspection, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context_inspection(&request))
    }

    pub fn recall_context_transcript_provenance_summary(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallTranscriptProvenanceSummary, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .recall_context_transcript_provenance_summary(&request))
    }

    pub fn recall_context_coverage(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallCoverage, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context_coverage(&request))
    }

    pub fn recall_context_omission_counts(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallOmissionCounts, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context_omission_counts(&request))
    }

    pub fn recall_context_limit_pressure(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallLimitPressure, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context_limit_pressure(&request))
    }

    pub fn recall_context_memory_taxonomy_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryTaxonomyReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .recall_context_memory_taxonomy_report(&request))
    }

    pub fn context_memory_namespace_policy_report(
        &self,
    ) -> Result<ContextMemoryNamespacePolicyReport, hepta_core::MemoryError> {
        Ok(self.snapshot()?.context_memory_namespace_policy_report())
    }

    pub fn context_memory_write_chain_readiness_report(
        &self,
    ) -> Result<ContextMemoryWriteChainReadinessReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .context_memory_write_chain_readiness_report())
    }

    pub fn context_memory_write_chain_receipt_freshness_report(
        &self,
    ) -> Result<ContextMemoryWriteChainReceiptFreshnessReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .context_memory_write_chain_receipt_freshness_report())
    }

    pub fn recall_context_memory_formation_receipt_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryFormationReceiptReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .recall_context_memory_formation_receipt_report(&request))
    }

    pub fn recall_context_memory_formation_queue_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryFormationQueueReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .recall_context_memory_formation_queue_report(&request))
    }

    pub fn recall_context_memory_temporal_fact_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryTemporalFactReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .recall_context_memory_temporal_fact_report(&request))
    }

    pub fn recall_context_memory_temporal_fact_graph_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryTemporalFactGraphReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .recall_context_memory_temporal_fact_graph_report(&request))
    }

    pub fn recall_context_memory_temporal_graph_shadow_store_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryTemporalGraphShadowStoreReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .recall_context_memory_temporal_graph_shadow_store_report(&request))
    }

    pub fn recall_context_memory_temporal_graph_shadow_replay_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryTemporalGraphShadowReplayReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .recall_context_memory_temporal_graph_shadow_replay_report(&request))
    }

    pub fn recall_context_memory_temporal_graph_shadow_traversal_diff_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryTemporalGraphShadowTraversalDiffReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .recall_context_memory_temporal_graph_shadow_traversal_diff_report(&request))
    }

    pub fn recall_context_memory_temporal_graph_shadow_traversal_quality_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryTemporalGraphShadowTraversalQualityReport, hepta_core::MemoryError>
    {
        Ok(self
            .snapshot()?
            .recall_context_memory_temporal_graph_shadow_traversal_quality_report(&request))
    }

    pub fn recall_context_memory_temporal_graph_shadow_retrieval_canary_guard_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport, hepta_core::MemoryError>
    {
        Ok(self
            .snapshot()?
            .recall_context_memory_temporal_graph_shadow_retrieval_canary_guard_report(&request))
    }

    pub fn recall_context_memory_temporal_graph_shadow_retrieval_rollback_kill_switch_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<
        ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport,
        hepta_core::MemoryError,
    > {
        Ok(self
            .snapshot()?
            .recall_context_memory_temporal_graph_shadow_retrieval_rollback_kill_switch_report(
                &request,
            ))
    }

    pub fn recall_context_memory_temporal_graph_shadow_retrieval_promotion_readiness_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<
        ContextMemoryTemporalGraphShadowRetrievalPromotionReadinessReport,
        hepta_core::MemoryError,
    > {
        Ok(self
            .snapshot()?
            .recall_context_memory_temporal_graph_shadow_retrieval_promotion_readiness_report(
                &request,
            ))
    }

    pub fn search_report(
        &self,
        query: MemoryQuery,
    ) -> Result<MemoryQueryReport, hepta_core::MemoryError> {
        let guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("memory store mutex poisoned".into()))?;
        let (matched, omitted_control_count) =
            memory_records_matching_recall_query(&guard.memories, &query.text);
        let matched_count = matched.len();
        let mut hits = matched;
        hits.truncate(query.limit);

        Ok(MemoryQueryReport::from_hits_with_omitted_control_count(
            query,
            matched_count,
            hits,
            omitted_control_count,
        ))
    }

    pub fn search_coverage(
        &self,
        query: MemoryQuery,
    ) -> Result<QueryReportCoverage, hepta_core::MemoryError> {
        Ok(self.search_report(query)?.coverage())
    }

    pub fn search_limit_pressure(
        &self,
        query: MemoryQuery,
    ) -> Result<QueryReportLimitPressure, hepta_core::MemoryError> {
        Ok(self.search_report(query)?.limit_pressure())
    }

    /// Builds the portable transcript query report directly from the store.
    ///
    /// Returned hits are newest-first so bounded query recall favors fresh
    /// transcript evidence over older matches.
    pub fn transcript_search_report(
        &self,
        query: TranscriptQuery,
    ) -> Result<TranscriptQueryReport, hepta_core::MemoryError> {
        let guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("transcript store mutex poisoned".into()))?;
        let (matched_count, hits) = transcript_query_hits(&guard.transcripts, &query);

        Ok(TranscriptQueryReport::from_hits(query, matched_count, hits))
    }

    pub fn transcript_search_coverage(
        &self,
        query: TranscriptQuery,
    ) -> Result<QueryReportCoverage, hepta_core::MemoryError> {
        Ok(self.transcript_search_report(query)?.coverage())
    }

    pub fn transcript_search_limit_pressure(
        &self,
        query: TranscriptQuery,
    ) -> Result<QueryReportLimitPressure, hepta_core::MemoryError> {
        Ok(self.transcript_search_report(query)?.limit_pressure())
    }
}

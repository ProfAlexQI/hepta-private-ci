use crate::InMemoryStore;
use crate::StoreSnapshot;
use hepta_core::ContextMemoryAdaptiveAllocatorEvalShadowReport;
use hepta_core::ContextMemoryEvalHarnessReport;
use hepta_core::ContextMemoryFormationQueueReport;
use hepta_core::ContextMemoryRankedRecallShadowEvalReport;
use hepta_core::ContextMemoryRecallQualityGateReport;
use hepta_core::ContextMemorySelectedRecallSummaryCanaryEvalReport;
use hepta_core::ContextMemoryShadowCanaryPromotionReadinessReport;
use hepta_core::ContextMemoryShadowQualitySummaryReport;
use hepta_core::ContextMemoryShadowQualityTrendSnapshotReport;
use hepta_core::ContextMemoryShadowRegressionDashboardReport;
use hepta_core::ContextMemoryTemporalFactGraphReport;
use hepta_core::ContextMemoryTemporalGraphShadowEvalReport;
use hepta_core::ContextPlaneActivationBlockerMatrix;
use hepta_core::ContextPlaneOperatorApprovalPacket;
use hepta_core::ContextPlaneStatusReport;
use hepta_core::ContextPlaneStatusReportInput;
use hepta_core::ContextRecallRequest;
use hepta_core::MemoryProviderAddReport;
use hepta_core::MemoryProviderClearReport;
use hepta_core::MemoryProviderClearScope;
use hepta_core::MemoryProviderCloseReport;
use hepta_core::MemoryProviderContextUpdateEnvelope;
use hepta_core::MemoryProviderDescriptor;
use hepta_core::MemoryProviderReport;
use hepta_core::MemoryProviderV2AuditReport;
use hepta_core::MemoryProviderWriteProposalReport;

impl StoreSnapshot {
    /// Builds the offline, payload-light eval harness seed report without
    /// reading or writing production memory.
    pub fn context_memory_eval_harness_seed_report(&self) -> ContextMemoryEvalHarnessReport {
        ContextMemoryEvalHarnessReport::seeded()
    }

    /// Builds the offline adaptive-allocator eval shadow report without
    /// changing runtime allocation or prompt assembly.
    pub fn context_memory_adaptive_allocator_eval_shadow_report(
        &self,
    ) -> ContextMemoryAdaptiveAllocatorEvalShadowReport {
        ContextMemoryAdaptiveAllocatorEvalShadowReport::from_seed(
            &self.context_memory_eval_harness_seed_report(),
        )
    }

    /// Builds the offline recall-quality gate report without activating
    /// adaptive allocation, source-aware compression, or prompt assembly.
    pub fn context_memory_recall_quality_gate_report(
        &self,
    ) -> ContextMemoryRecallQualityGateReport {
        ContextMemoryRecallQualityGateReport::from_shadow(
            &self.context_memory_adaptive_allocator_eval_shadow_report(),
        )
    }

    /// Builds the offline memory-provider boundary report without changing
    /// prompt assembly, clearing store state, or enabling a runtime route.
    pub fn context_memory_provider_report(
        &self,
        request: &ContextRecallRequest,
    ) -> MemoryProviderReport {
        let bundle = self.recall_context(request);
        let limit_pressure = self.recall_context_limit_pressure(request);
        MemoryProviderReport::from_update(
            MemoryProviderDescriptor::builtin(),
            MemoryProviderContextUpdateEnvelope::from_bundle("builtin", &bundle, limit_pressure),
        )
    }

    /// Builds the offline ranked-recall shadow eval report without exporting
    /// ranked payloads, activating runtime routes, or changing prompt assembly.
    pub fn context_memory_ranked_recall_shadow_eval_report(
        &self,
    ) -> ContextMemoryRankedRecallShadowEvalReport {
        ContextMemoryRankedRecallShadowEvalReport::seeded()
    }

    /// Builds the offline selected-recall summary canary eval replay report
    /// without activating a production route or changing prompt assembly.
    pub fn context_memory_selected_recall_summary_canary_eval_report(
        &self,
    ) -> ContextMemorySelectedRecallSummaryCanaryEvalReport {
        ContextMemorySelectedRecallSummaryCanaryEvalReport::seeded()
    }

    /// Builds the offline temporal-graph shadow eval report without writing
    /// graph facts, activating runtime routes, or changing prompt assembly.
    pub fn context_memory_temporal_graph_shadow_eval_report(
        &self,
    ) -> ContextMemoryTemporalGraphShadowEvalReport {
        ContextMemoryTemporalGraphShadowEvalReport::seeded()
    }

    /// Builds a payload-light dashboard that closes the loop across recall,
    /// temporal graph, quality, and provider shadow reports.
    pub fn context_memory_shadow_regression_dashboard_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryShadowRegressionDashboardReport {
        let ranked_recall = self.context_memory_ranked_recall_shadow_eval_report();
        let temporal_graph = self.context_memory_temporal_graph_shadow_eval_report();
        let recall_quality = self.context_memory_recall_quality_gate_report();
        let provider = self.context_memory_provider_report(request);

        ContextMemoryShadowRegressionDashboardReport::from_reports(
            &ranked_recall,
            &temporal_graph,
            &recall_quality,
            &provider,
        )
    }

    /// Builds a payload-light operator-readable quality summary from the
    /// shadow regression dashboard without enabling a production route.
    pub fn context_memory_shadow_quality_summary_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryShadowQualitySummaryReport {
        ContextMemoryShadowQualitySummaryReport::from_dashboard(
            &self.context_memory_shadow_regression_dashboard_report(request),
        )
    }

    /// Builds a payload-light trend snapshot from the shadow quality summary
    /// without persisting history or enabling a production route.
    pub fn context_memory_shadow_quality_trend_snapshot_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryShadowQualityTrendSnapshotReport {
        ContextMemoryShadowQualityTrendSnapshotReport::from_summary(
            &self.context_memory_shadow_quality_summary_report(request),
        )
    }

    /// Builds a payload-light canary-promotion readiness rehearsal from the
    /// shadow trend snapshot without opening any production route.
    pub fn context_memory_shadow_canary_promotion_readiness_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextMemoryShadowCanaryPromotionReadinessReport {
        ContextMemoryShadowCanaryPromotionReadinessReport::from_trend_snapshot(
            &self.context_memory_shadow_quality_trend_snapshot_report(request),
        )
    }

    /// Builds a unified, payload-light context-plane status report for
    /// operator readiness without changing runtime behavior.
    pub fn context_plane_status_report(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextPlaneStatusReport {
        let taxonomy = self.recall_context_memory_taxonomy_report(request);
        let formation_receipts = self.recall_context_memory_formation_receipt_report(request);
        let formation_queue = ContextMemoryFormationQueueReport::from_receipts(&formation_receipts);
        let temporal_facts = self.recall_context_memory_temporal_fact_report(request);
        let temporal_fact_graph =
            ContextMemoryTemporalFactGraphReport::from_temporal_facts(&temporal_facts);
        let temporal_graph_shadow_eval = self.context_memory_temporal_graph_shadow_eval_report();
        let eval_seed = self.context_memory_eval_harness_seed_report();
        let allocator_shadow =
            ContextMemoryAdaptiveAllocatorEvalShadowReport::from_seed(&eval_seed);
        let recall_quality_gate =
            ContextMemoryRecallQualityGateReport::from_shadow(&allocator_shadow);
        let ranked_recall = self.context_memory_ranked_recall_shadow_eval_report();
        let provider_report = self.context_memory_provider_report(request);
        let provider_v2_write_proposal =
            MemoryProviderWriteProposalReport::from_formation_queue("builtin", &formation_queue);
        let provider_v2_audit = MemoryProviderV2AuditReport::from_parts(
            provider_report.descriptor.clone(),
            provider_report.update_context.clone(),
            provider_v2_write_proposal.clone(),
            MemoryProviderAddReport::blocked(&provider_v2_write_proposal),
            MemoryProviderClearReport::blocked("builtin", MemoryProviderClearScope::All),
            MemoryProviderCloseReport::shadow_noop("builtin"),
        );
        let shadow_quality_trend_snapshot =
            self.context_memory_shadow_quality_trend_snapshot_report(request);
        let shadow_canary_promotion_readiness =
            self.context_memory_shadow_canary_promotion_readiness_report(request);

        ContextPlaneStatusReport::from_reports(ContextPlaneStatusReportInput {
            taxonomy: &taxonomy,
            formation_receipts: &formation_receipts,
            formation_queue: &formation_queue,
            temporal_facts: &temporal_facts,
            temporal_fact_graph: &temporal_fact_graph,
            temporal_graph_shadow_eval: &temporal_graph_shadow_eval,
            eval_seed: &eval_seed,
            allocator_shadow: &allocator_shadow,
            recall_quality_gate: &recall_quality_gate,
            ranked_recall: &ranked_recall,
            provider_report: &provider_report,
            provider_v2_audit: &provider_v2_audit,
            shadow_quality_trend_snapshot: &shadow_quality_trend_snapshot,
            shadow_canary_promotion_readiness: &shadow_canary_promotion_readiness,
        })
    }

    /// Builds a payload-light activation-blocker matrix from the unified
    /// context-plane status report without enabling runtime behavior.
    pub fn context_plane_activation_blocker_matrix(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextPlaneActivationBlockerMatrix {
        ContextPlaneActivationBlockerMatrix::from_status(&self.context_plane_status_report(request))
    }

    /// Builds a payload-light operator approval dry-run packet from the
    /// activation-blocker matrix without enabling runtime behavior.
    pub fn context_plane_operator_approval_packet(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextPlaneOperatorApprovalPacket {
        ContextPlaneOperatorApprovalPacket::from_matrix(
            &self.context_plane_activation_blocker_matrix(request),
        )
    }
}

impl InMemoryStore {
    pub fn context_memory_eval_harness_seed_report(
        &self,
    ) -> Result<ContextMemoryEvalHarnessReport, hepta_core::MemoryError> {
        Ok(self.snapshot()?.context_memory_eval_harness_seed_report())
    }

    pub fn context_memory_adaptive_allocator_eval_shadow_report(
        &self,
    ) -> Result<ContextMemoryAdaptiveAllocatorEvalShadowReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .context_memory_adaptive_allocator_eval_shadow_report())
    }

    pub fn context_memory_recall_quality_gate_report(
        &self,
    ) -> Result<ContextMemoryRecallQualityGateReport, hepta_core::MemoryError> {
        Ok(self.snapshot()?.context_memory_recall_quality_gate_report())
    }

    pub fn context_memory_provider_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<MemoryProviderReport, hepta_core::MemoryError> {
        Ok(self.snapshot()?.context_memory_provider_report(&request))
    }

    pub fn context_memory_ranked_recall_shadow_eval_report(
        &self,
    ) -> Result<ContextMemoryRankedRecallShadowEvalReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .context_memory_ranked_recall_shadow_eval_report())
    }

    pub fn context_memory_selected_recall_summary_canary_eval_report(
        &self,
    ) -> Result<ContextMemorySelectedRecallSummaryCanaryEvalReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .context_memory_selected_recall_summary_canary_eval_report())
    }

    pub fn context_memory_temporal_graph_shadow_eval_report(
        &self,
    ) -> Result<ContextMemoryTemporalGraphShadowEvalReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .context_memory_temporal_graph_shadow_eval_report())
    }

    pub fn context_memory_shadow_regression_dashboard_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryShadowRegressionDashboardReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .context_memory_shadow_regression_dashboard_report(&request))
    }

    pub fn context_memory_shadow_quality_summary_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryShadowQualitySummaryReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .context_memory_shadow_quality_summary_report(&request))
    }

    pub fn context_memory_shadow_quality_trend_snapshot_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryShadowQualityTrendSnapshotReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .context_memory_shadow_quality_trend_snapshot_report(&request))
    }

    pub fn context_memory_shadow_canary_promotion_readiness_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextMemoryShadowCanaryPromotionReadinessReport, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .context_memory_shadow_canary_promotion_readiness_report(&request))
    }

    pub fn context_plane_status_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextPlaneStatusReport, hepta_core::MemoryError> {
        Ok(self.snapshot()?.context_plane_status_report(&request))
    }

    pub fn context_plane_activation_blocker_matrix(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextPlaneActivationBlockerMatrix, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .context_plane_activation_blocker_matrix(&request))
    }

    pub fn context_plane_operator_approval_packet(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextPlaneOperatorApprovalPacket, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .context_plane_operator_approval_packet(&request))
    }
}

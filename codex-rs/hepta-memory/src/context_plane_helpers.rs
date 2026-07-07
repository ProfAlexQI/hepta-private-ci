use crate::InMemoryStore;
use crate::StoreSnapshot;
use hepta_core::ContextMemoryAdaptiveAllocatorEvalShadowReport;
use hepta_core::ContextMemoryEvalHarnessReport;
use hepta_core::ContextMemoryFormationQueueReport;
use hepta_core::ContextMemoryRankedRecallShadowEvalReport;
use hepta_core::ContextMemoryRecallQualityGateReport;
use hepta_core::ContextMemorySelectedRecallSummaryCanaryEvalReport;
use hepta_core::ContextMemoryTemporalFactGraphReport;
use hepta_core::ContextPlaneActivationBlockerMatrix;
use hepta_core::ContextPlaneOperatorApprovalPacket;
use hepta_core::ContextPlaneStatusReport;
use hepta_core::ContextPlaneStatusReportInput;
use hepta_core::ContextRecallRequest;
use hepta_core::MemoryProviderContextUpdateEnvelope;
use hepta_core::MemoryProviderDescriptor;
use hepta_core::MemoryProviderReport;

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
        let eval_seed = self.context_memory_eval_harness_seed_report();
        let allocator_shadow =
            ContextMemoryAdaptiveAllocatorEvalShadowReport::from_seed(&eval_seed);
        let recall_quality_gate =
            ContextMemoryRecallQualityGateReport::from_shadow(&allocator_shadow);
        let provider_report = self.context_memory_provider_report(request);

        ContextPlaneStatusReport::from_reports(ContextPlaneStatusReportInput {
            taxonomy: &taxonomy,
            formation_receipts: &formation_receipts,
            formation_queue: &formation_queue,
            temporal_facts: &temporal_facts,
            temporal_fact_graph: &temporal_fact_graph,
            eval_seed: &eval_seed,
            allocator_shadow: &allocator_shadow,
            recall_quality_gate: &recall_quality_gate,
            provider_report: &provider_report,
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

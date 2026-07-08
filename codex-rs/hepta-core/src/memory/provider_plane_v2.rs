use serde::Deserialize;
use serde::Serialize;

use super::formation::ContextMemoryFormationQueueReport;
use super::provider_plane::MemoryProviderClearReport;
use super::provider_plane::MemoryProviderClearRequest;
use super::provider_plane::MemoryProviderContextUpdateEnvelope;
use super::provider_plane::MemoryProviderDescriptor;
use super::recall::ContextRecallBundle;
use super::recall::ContextRecallRequest;

pub const MEMORY_PROVIDER_V2_SHADOW_BOUNDARY_SCHEMA_VERSION: u32 = 1;

/// Write lifecycle posture for the provider V2 boundary.
///
/// The first V2 contract is intentionally shadow-only: providers may propose,
/// audit, and reject durable-memory writes, but this does not grant a
/// production memory or graph write route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryProviderWriteMode {
    ShadowOnly,
}

/// Payload-light provider write proposal derived from the memory-formation
/// queue.
///
/// The proposal exposes counts and safety posture only. It deliberately omits
/// candidate text, transcript text, query text, prompt text, source ids,
/// session ids, memory ids, trace ids, and provider-specific payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderWriteProposalReport {
    pub schema_version: u32,
    pub provider_id: String,
    pub mode: MemoryProviderWriteMode,
    pub candidate_count: usize,
    pub queued_candidate_count: usize,
    pub revocable_candidate_count: usize,
    pub operator_review_required_count: usize,
    pub source_queue_integrity: bool,
    pub payload_light: bool,
    pub operator_approval_required: bool,
    pub prompt_payload_exported: bool,
    pub query_payload_exported: bool,
    pub candidate_payload_exported: bool,
    pub source_payload_exported: bool,
    pub write_performed: bool,
    pub graph_write_performed: bool,
    pub runtime_activation: bool,
}

impl MemoryProviderWriteProposalReport {
    pub fn from_formation_queue(
        provider_id: impl Into<String>,
        queue: &ContextMemoryFormationQueueReport,
    ) -> Self {
        Self {
            schema_version: MEMORY_PROVIDER_V2_SHADOW_BOUNDARY_SCHEMA_VERSION,
            provider_id: provider_id.into(),
            mode: MemoryProviderWriteMode::ShadowOnly,
            candidate_count: queue.items.len(),
            queued_candidate_count: queue.queued_count(),
            revocable_candidate_count: queue.revocable_count(),
            operator_review_required_count: queue.operator_review_required_count(),
            source_queue_integrity: queue.has_queue_integrity(),
            payload_light: true,
            operator_approval_required: true,
            prompt_payload_exported: false,
            query_payload_exported: false,
            candidate_payload_exported: false,
            source_payload_exported: false,
            write_performed: false,
            graph_write_performed: false,
            runtime_activation: false,
        }
    }

    pub fn has_shadow_boundary_integrity(&self) -> bool {
        self.schema_version == MEMORY_PROVIDER_V2_SHADOW_BOUNDARY_SCHEMA_VERSION
            && self.mode == MemoryProviderWriteMode::ShadowOnly
            && self.source_queue_integrity
            && self.queued_candidate_count <= self.candidate_count
            && self.revocable_candidate_count <= self.candidate_count
            && self.operator_review_required_count <= self.candidate_count
            && self.payload_light
            && self.operator_approval_required
            && !self.prompt_payload_exported
            && !self.query_payload_exported
            && !self.candidate_payload_exported
            && !self.source_payload_exported
            && !self.write_performed
            && !self.graph_write_performed
            && !self.runtime_activation
    }
}

/// Request shape for an attempted provider add operation.
///
/// The request carries the payload-light proposal report rather than candidate
/// content. Until the V2 boundary is promoted, add attempts return a dry-run or
/// blocked report and do not mutate memory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderAddRequest {
    pub proposal: MemoryProviderWriteProposalReport,
    pub dry_run: bool,
    pub operator_approval_granted: bool,
}

/// Payload-light result for provider add attempts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderAddReport {
    pub schema_version: u32,
    pub provider_id: String,
    pub dry_run: bool,
    pub operator_approval_required: bool,
    pub blocked: bool,
    pub candidate_count: usize,
    pub accepted_candidate_count: usize,
    pub affected_record_count: usize,
    pub payload_light: bool,
    pub prompt_payload_exported: bool,
    pub candidate_payload_exported: bool,
    pub write_performed: bool,
    pub graph_write_performed: bool,
    pub runtime_activation: bool,
}

impl MemoryProviderAddReport {
    pub fn dry_run(proposal: &MemoryProviderWriteProposalReport) -> Self {
        Self {
            schema_version: MEMORY_PROVIDER_V2_SHADOW_BOUNDARY_SCHEMA_VERSION,
            provider_id: proposal.provider_id.clone(),
            dry_run: true,
            operator_approval_required: true,
            blocked: false,
            candidate_count: proposal.candidate_count,
            accepted_candidate_count: 0,
            affected_record_count: 0,
            payload_light: true,
            prompt_payload_exported: false,
            candidate_payload_exported: false,
            write_performed: false,
            graph_write_performed: false,
            runtime_activation: false,
        }
    }

    pub fn blocked(proposal: &MemoryProviderWriteProposalReport) -> Self {
        Self {
            schema_version: MEMORY_PROVIDER_V2_SHADOW_BOUNDARY_SCHEMA_VERSION,
            provider_id: proposal.provider_id.clone(),
            dry_run: false,
            operator_approval_required: true,
            blocked: true,
            candidate_count: proposal.candidate_count,
            accepted_candidate_count: 0,
            affected_record_count: 0,
            payload_light: true,
            prompt_payload_exported: false,
            candidate_payload_exported: false,
            write_performed: false,
            graph_write_performed: false,
            runtime_activation: false,
        }
    }

    pub fn has_no_side_effects(&self) -> bool {
        self.schema_version == MEMORY_PROVIDER_V2_SHADOW_BOUNDARY_SCHEMA_VERSION
            && self.accepted_candidate_count == 0
            && self.affected_record_count == 0
            && self.payload_light
            && !self.prompt_payload_exported
            && !self.candidate_payload_exported
            && !self.write_performed
            && !self.graph_write_performed
            && !self.runtime_activation
    }
}

/// Payload-light result for provider close attempts.
///
/// Close is modelled explicitly so future provider implementations have a
/// lifecycle boundary without hiding cleanup, write, or activation behavior in
/// drop handlers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderCloseReport {
    pub schema_version: u32,
    pub provider_id: String,
    pub payload_light: bool,
    pub close_performed: bool,
    pub prompt_payload_exported: bool,
    pub write_performed: bool,
    pub runtime_activation: bool,
}

impl MemoryProviderCloseReport {
    pub fn shadow_noop(provider_id: impl Into<String>) -> Self {
        Self {
            schema_version: MEMORY_PROVIDER_V2_SHADOW_BOUNDARY_SCHEMA_VERSION,
            provider_id: provider_id.into(),
            payload_light: true,
            close_performed: false,
            prompt_payload_exported: false,
            write_performed: false,
            runtime_activation: false,
        }
    }

    pub fn has_no_side_effects(&self) -> bool {
        self.schema_version == MEMORY_PROVIDER_V2_SHADOW_BOUNDARY_SCHEMA_VERSION
            && self.payload_light
            && !self.close_performed
            && !self.prompt_payload_exported
            && !self.write_performed
            && !self.runtime_activation
    }
}

/// Full payload-light audit bundle for one provider V2 lifecycle rehearsal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderV2AuditReport {
    pub schema_version: u32,
    pub descriptor: MemoryProviderDescriptor,
    pub update_context: MemoryProviderContextUpdateEnvelope,
    pub write_proposal: MemoryProviderWriteProposalReport,
    pub add: MemoryProviderAddReport,
    pub clear: MemoryProviderClearReport,
    pub close: MemoryProviderCloseReport,
}

impl MemoryProviderV2AuditReport {
    pub fn from_parts(
        descriptor: MemoryProviderDescriptor,
        update_context: MemoryProviderContextUpdateEnvelope,
        write_proposal: MemoryProviderWriteProposalReport,
        add: MemoryProviderAddReport,
        clear: MemoryProviderClearReport,
        close: MemoryProviderCloseReport,
    ) -> Self {
        Self {
            schema_version: MEMORY_PROVIDER_V2_SHADOW_BOUNDARY_SCHEMA_VERSION,
            descriptor,
            update_context,
            write_proposal,
            add,
            clear,
            close,
        }
    }

    pub fn has_shadow_boundary_integrity(&self) -> bool {
        self.schema_version == MEMORY_PROVIDER_V2_SHADOW_BOUNDARY_SCHEMA_VERSION
            && self.descriptor.context_fencing_required
            && self.descriptor.provenance_required
            && self.update_context.has_payload_light_boundary()
            && self.write_proposal.has_shadow_boundary_integrity()
            && self.add.has_no_side_effects()
            && self.clear.has_no_side_effects()
            && self.close.has_no_side_effects()
    }
}

/// Runtime-facing provider V2 boundary for recall and memory-write lifecycle
/// attempts.
///
/// Implementations must own query, context update, write proposal, add, clear,
/// and close attempts in one typed surface. The initial contract is
/// shadow-only: write proposal/add/clear/close reports are payload-light audit
/// evidence and must not mutate memory, write graph state, export raw payloads,
/// or activate a production route.
pub trait MemoryProviderV2: Send + Sync {
    fn query(
        &self,
        request: ContextRecallRequest,
    ) -> impl std::future::Future<Output = Result<ContextRecallBundle, crate::MemoryError>> + Send;

    fn update_context(
        &self,
        request: ContextRecallRequest,
    ) -> impl std::future::Future<
        Output = Result<MemoryProviderContextUpdateEnvelope, crate::MemoryError>,
    > + Send;

    fn propose_write(
        &self,
        queue: ContextMemoryFormationQueueReport,
    ) -> impl std::future::Future<
        Output = Result<MemoryProviderWriteProposalReport, crate::MemoryError>,
    > + Send;

    fn add(
        &self,
        request: MemoryProviderAddRequest,
    ) -> impl std::future::Future<Output = Result<MemoryProviderAddReport, crate::MemoryError>> + Send;

    fn clear(
        &self,
        request: MemoryProviderClearRequest,
    ) -> impl std::future::Future<Output = Result<MemoryProviderClearReport, crate::MemoryError>> + Send;

    fn close(
        &self,
    ) -> impl std::future::Future<Output = Result<MemoryProviderCloseReport, crate::MemoryError>> + Send;
}

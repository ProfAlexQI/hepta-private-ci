//! Exact, single-use execution authority assembled immediately before dispatch.

use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::Mutex;

use hepta_contracts::CapabilityDescriptor;
use hepta_contracts::CapabilityManifestRef;
use hepta_contracts::ContentHash;
use hepta_contracts::PrincipalId;
use hepta_core::CorrelationId;
use hepta_core::HeptaError;
use hepta_core::SessionId;
use hepta_core::ToolExecutionMetadata;
use hepta_kernel::HeptaKernelSafetyAuthorization;
use hepta_memory::ExecutionEffectAck;
use hepta_memory::ExecutionIntent;
use hepta_memory::ExecutionIntentParts;
#[cfg(test)]
use hepta_memory::OutcomeRecord;
use hepta_memory::candidate_reference_hash;
use serde::Serialize;
use uuid::Uuid;

use super::approval_state::ExactApprovalMaterial;
use super::context_freezer::current_capability_descriptor;
use super::context_freezer::framed_hash;
use super::context_freezer::now_ms;
use super::execution_lease::ResourceBoundExecutionLease;
use super::outcome_recorder::OutcomeRecorder;
use super::outcome_sink::OutcomeBreakerState;
use super::outcome_sink::SharedOutcomeReceiptSink;
use crate::RuntimeKernel;

const ATTEMPT_ID_RETRY_LIMIT: usize = 32;

/// Private registry for live attempts and the independent receipt breaker.
#[derive(Debug, Default)]
pub(crate) struct ExecutionOutcomeState {
    pub(super) active_attempts: BTreeSet<String>,
    pub(super) breaker: OutcomeBreakerState,
    pub(super) finalized_attempts: u64,
}

/// Stable identity of the concrete capability executor selected for dispatch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExecutorBinding {
    principal: PrincipalId,
    provider: String,
    operation: String,
    manifest_hash: ContentHash,
}

impl ExecutorBinding {
    fn from_descriptor(descriptor: &CapabilityDescriptor) -> Self {
        let provider = descriptor.provider().as_str().to_string();
        let operation = descriptor.operation().to_string();
        let manifest_hash = descriptor.manifest_hash().clone();
        let principal_hash = framed_hash(
            "hepta.runtime.executor-principal.v1",
            &[
                ("provider", provider.as_bytes()),
                ("operation", operation.as_bytes()),
                ("manifest_hash", manifest_hash.as_str().as_bytes()),
            ],
        );
        Self {
            principal: PrincipalId::new(format!("executor:{}", principal_hash.as_str())),
            provider,
            operation,
            manifest_hash,
        }
    }

    pub(crate) fn principal(&self) -> &PrincipalId {
        &self.principal
    }

    pub(crate) fn provider(&self) -> &str {
        &self.provider
    }

    pub(crate) fn operation(&self) -> &str {
        &self.operation
    }

    pub(crate) fn manifest_hash(&self) -> &ContentHash {
        &self.manifest_hash
    }
}

/// Reservation proving that one unpredictable attempt identity is live once.
#[derive(Debug)]
pub(crate) struct ExecutionAttemptReservation {
    pub(super) state: Arc<Mutex<ExecutionOutcomeState>>,
    attempt_id: String,
    active: bool,
}

impl ExecutionAttemptReservation {
    pub(crate) fn attempt_id(&self) -> &str {
        &self.attempt_id
    }

    pub(super) fn mark_finalized(&mut self) {
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        };
        state.active_attempts.remove(&self.attempt_id);
        state.finalized_attempts = state.finalized_attempts.saturating_add(1);
        self.active = false;
    }
}

impl Drop for ExecutionAttemptReservation {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        };
        state.active_attempts.remove(&self.attempt_id);
        self.active = false;
    }
}

/// Exact dispatch material frozen before commit-time kernel authorization.
#[derive(Debug)]
pub(crate) struct PreparedExecutionAttempt {
    reservation: ExecutionAttemptReservation,
    execution_lease: ResourceBoundExecutionLease,
    session_id: SessionId,
    correlation_id: CorrelationId,
    tool_name: String,
    canonical_arguments: String,
    payload_hash: ContentHash,
    capability: CapabilityManifestRef,
    executor: ExecutorBinding,
    metadata: ToolExecutionMetadata,
    started_at_unix_ms: u64,
}

impl PreparedExecutionAttempt {
    pub(super) fn prepare(
        runtime: &RuntimeKernel,
        session_id: &SessionId,
        correlation_id: &CorrelationId,
        approved: &ExactApprovalMaterial,
        presented: &ExactApprovalMaterial,
        execution_lease: ResourceBoundExecutionLease,
    ) -> Result<Self, HeptaError> {
        validate_capability_material(approved, presented)?;
        let requests = presented.candidate.capability_requests();
        let [request] = requests else {
            return Err(HeptaError(
                "exact execution candidate must contain one capability request".into(),
            ));
        };
        if request.payload_hash() != &presented.payload_hash {
            return Err(HeptaError(
                "exact execution request payload differs from candidate payload".into(),
            ));
        }
        let capability = request.capability().clone();
        let current_descriptor =
            current_capability_descriptor(runtime, &presented.tool_name, capability.catalog())?;
        if current_descriptor != presented.capability_descriptor {
            return Err(HeptaError(
                "capability binding denied: registry_descriptor_drift".into(),
            ));
        }
        let executor = ExecutorBinding::from_descriptor(&current_descriptor);

        Ok(Self {
            reservation: runtime.reserve_execution_attempt()?,
            execution_lease,
            session_id: session_id.clone(),
            correlation_id: correlation_id.clone(),
            tool_name: presented.tool_name.clone(),
            canonical_arguments: presented.canonical_arguments.clone(),
            payload_hash: presented.payload_hash.clone(),
            capability,
            executor,
            metadata: runtime.tools.execution_metadata(&presented.tool_name)?,
            started_at_unix_ms: now_ms(),
        })
    }

    pub(crate) fn attempt_id(&self) -> &str {
        self.reservation.attempt_id()
    }

    pub(crate) fn executor(&self) -> &ExecutorBinding {
        &self.executor
    }

    pub(crate) fn capability(&self) -> &CapabilityManifestRef {
        &self.capability
    }

    pub(super) fn authorize(
        self,
        authorization: HeptaKernelSafetyAuthorization,
        outcome_sink: SharedOutcomeReceiptSink,
    ) -> AuthorizedToolExecution {
        AuthorizedToolExecution {
            reservation: self.reservation,
            authorization,
            execution_lease: self.execution_lease,
            outcome_sink,
            execution_intent: None,
            execution_effect_ack: None,
            receipt_guard_armed: true,
            session_id: self.session_id,
            correlation_id: self.correlation_id,
            tool_name: self.tool_name,
            canonical_arguments: self.canonical_arguments,
            payload_hash: self.payload_hash,
            capability: self.capability,
            executor: self.executor,
            metadata: self.metadata,
            started_at_unix_ms: self.started_at_unix_ms,
        }
    }
}

/// Non-cloneable proof carrying all exact inputs required for one dispatch.
pub(crate) struct AuthorizedToolExecution {
    pub(super) reservation: ExecutionAttemptReservation,
    authorization: HeptaKernelSafetyAuthorization,
    execution_lease: ResourceBoundExecutionLease,
    outcome_sink: SharedOutcomeReceiptSink,
    execution_intent: Option<ExecutionIntent>,
    execution_effect_ack: Option<ExecutionEffectAck>,
    receipt_guard_armed: bool,
    session_id: SessionId,
    correlation_id: CorrelationId,
    tool_name: String,
    canonical_arguments: String,
    payload_hash: ContentHash,
    capability: CapabilityManifestRef,
    executor: ExecutorBinding,
    metadata: ToolExecutionMetadata,
    started_at_unix_ms: u64,
}

impl AuthorizedToolExecution {
    pub(crate) fn attempt_id(&self) -> &str {
        self.reservation.attempt_id()
    }

    pub(crate) fn authorization(&self) -> &HeptaKernelSafetyAuthorization {
        &self.authorization
    }

    pub(crate) fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub(crate) fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }

    pub(crate) fn tool_name(&self) -> &str {
        &self.tool_name
    }

    pub(crate) fn canonical_arguments(&self) -> &str {
        &self.canonical_arguments
    }

    pub(crate) fn payload_hash(&self) -> &ContentHash {
        &self.payload_hash
    }

    pub(crate) fn capability(&self) -> &CapabilityManifestRef {
        &self.capability
    }

    pub(crate) fn executor(&self) -> &ExecutorBinding {
        &self.executor
    }

    pub(crate) const fn metadata(&self) -> ToolExecutionMetadata {
        self.metadata
    }

    pub(crate) const fn started_at_unix_ms(&self) -> u64 {
        self.started_at_unix_ms
    }

    pub(super) fn validate_dispatch_selector(
        &self,
        runtime: &RuntimeKernel,
    ) -> Result<(), HeptaError> {
        let current =
            current_capability_descriptor(runtime, &self.tool_name, self.capability.catalog())
                .map_err(|error| {
                    HeptaError(format!(
                        "dispatch selector denied: registry_descriptor_drift: {}",
                        error.0
                    ))
                })?;
        let current_executor = ExecutorBinding::from_descriptor(&current);
        if current.reference() != self.capability || current_executor != self.executor {
            return Err(HeptaError(
                "dispatch selector denied: exact executor binding changed after authorization"
                    .into(),
            ));
        }
        Ok(())
    }

    pub(super) fn stage_execution_intent(&mut self) -> Result<(), HeptaError> {
        let intent = self.build_execution_intent()?;
        match self.outcome_sink.stage_execution_intent(&intent) {
            Ok(_) => {
                self.execution_intent = Some(intent);
                Ok(())
            }
            Err(error) if error.is_commit_ambiguous() => {
                match self
                    .outcome_sink
                    .pending_execution_intent(intent.attempt_id())
                {
                    Ok(Some(recovered)) if recovered == intent => {
                        self.execution_intent = Some(intent);
                        Ok(())
                    }
                    Ok(Some(_)) => {
                        let reason = format!(
                            "execution-intent stage acknowledgement was ambiguous and read-back conflicted for attempt {}",
                            intent.attempt_id()
                        );
                        self.trip_outcome_breaker(reason.clone());
                        Err(HeptaError(reason))
                    }
                    Ok(None) => {
                        let reason = format!(
                            "execution-intent stage acknowledgement was ambiguous and read-back did not confirm attempt {}",
                            intent.attempt_id()
                        );
                        self.trip_outcome_breaker(reason.clone());
                        Err(HeptaError(reason))
                    }
                    Err(read_error) => {
                        let reason = format!(
                            "execution-intent stage acknowledgement was ambiguous and read-back failed for attempt {}: {read_error}",
                            intent.attempt_id()
                        );
                        self.trip_outcome_breaker(reason.clone());
                        Err(HeptaError(reason))
                    }
                }
            }
            Err(error) => Err(HeptaError(format!(
                "provider dispatch blocked because execution intent was not durably staged: {error}"
            ))),
        }
    }

    pub(crate) fn execution_intent(&self) -> Option<&ExecutionIntent> {
        self.execution_intent.as_ref()
    }

    pub(crate) fn execution_effect_ack(&self) -> Option<&ExecutionEffectAck> {
        self.execution_effect_ack.as_ref()
    }

    pub(super) fn idempotency_key(&self) -> Option<&str> {
        self.execution_intent
            .as_ref()
            .map(ExecutionIntent::idempotency_key)
    }

    /// Releases frozen-context exclusion after provider dispatch. Tool-specific
    /// reservations stay owned by the resource-bound lease until this witness
    /// is dropped after terminal receipt finalization.
    pub(super) fn release_execution_lease(&mut self) {
        self.execution_lease.release_context_exclusion();
    }

    pub(super) fn prepared_write_transactions(&self) -> &[crate::PreparedWriteTransaction] {
        self.execution_lease.prepared_write_transactions()
    }

    pub(super) fn prepared_read_capability(&self) -> Option<&crate::PreparedReadCapability> {
        self.execution_lease.prepared_read_capability()
    }

    #[cfg(test)]
    pub(crate) fn holds_write_target_reservation(&self) -> bool {
        self.execution_lease.holds_write_target_reservation()
    }

    #[cfg(test)]
    pub(crate) fn holds_read_capability(&self) -> bool {
        self.execution_lease.holds_read_capability()
    }

    pub(super) fn outcome_sink(&self) -> &SharedOutcomeReceiptSink {
        &self.outcome_sink
    }

    pub(super) fn confirm_provider_effect_ack(
        &mut self,
        result: Option<&hepta_core::ToolResult>,
    ) -> Result<(), HeptaError> {
        let intent = self.execution_intent.as_ref().ok_or_else(|| {
            HeptaError("provider effect confirmation lost its staged execution intent".into())
        })?;
        let ack = super::provider_effect::confirm_provider_effect_ack(
            intent,
            self.prepared_write_transactions(),
            result,
        )?;
        self.execution_effect_ack = ack;
        Ok(())
    }

    pub(super) fn stage_provider_completion(
        &self,
        exact: &super::outcome_sink::ExactOutcomeRecord,
    ) -> Result<(), HeptaError> {
        let Some(ack) = self.execution_effect_ack.as_ref() else {
            return Ok(());
        };
        self.outcome_sink
            .stage_provider_completion(ack, exact)
            .map(|_| ())
            .map_err(|error| {
                HeptaError(format!(
                    "provider completion was not atomically persisted: {error}"
                ))
            })
    }

    fn build_execution_intent(&self) -> Result<ExecutionIntent, HeptaError> {
        let canonical_resource_summary =
            serde_json::to_string(&self.resource_summary()).map_err(|error| {
                HeptaError(format!(
                    "failed to canonicalize execution resource summary: {error}"
                ))
            })?;
        let authorization = self.authorization.authorization();
        let binding = self.authorization.binding();
        let canonical_effect_plan = super::provider_effect::canonical_effect_plan_for(
            &self.tool_name,
            self.prepared_write_transactions(),
            &self.canonical_arguments,
        )?;
        ExecutionIntent::try_new(ExecutionIntentParts {
            attempt_id: self.attempt_id().to_owned(),
            session_id: self.session_id.0.clone(),
            correlation_id: self.correlation_id.0.clone(),
            tool_name: self.tool_name.clone(),
            payload_hash: self.payload_hash.clone(),
            candidate_hash: authorization.candidate().content_hash().clone(),
            candidate_reference_hash: candidate_reference_hash(authorization.candidate()),
            kernel_candidate_hash: binding.candidate_hash().clone(),
            payload_set_hash: binding.payload_set_hash().clone(),
            capability_id: self.capability.id().as_str().to_owned(),
            capability_revision: self.capability.revision().get(),
            capability_provider: self.executor.provider.clone(),
            capability_operation: self.executor.operation.clone(),
            capability_manifest_hash: self.executor.manifest_hash.clone(),
            executor_principal: self.executor.principal.as_str().to_owned(),
            authorization_digest: authorization.content_hash().clone(),
            admission_id: authorization.admission().id().as_str().to_owned(),
            admission_revision: authorization.admission().revision().get(),
            admission_digest: authorization.admission().content_hash().clone(),
            canonical_resource_summary,
            canonical_effect_plan,
        })
        .map_err(|error| HeptaError(format!("failed to bind execution intent: {error}")))
    }

    fn resource_summary(&self) -> PlannedResourceSummary<'_> {
        let writes = self
            .prepared_write_transactions()
            .iter()
            .map(|transaction| PlannedWriteResource {
                operation: transaction.operation.as_str(),
                requested_path: transaction.requested_path.as_str(),
                target_path: transaction.target_path.as_str(),
                mode_requested: transaction.mode_requested.as_str(),
                preview_only: transaction.preview_only,
                target_existed_before: transaction.target_existed_before,
                canonical_namespace: transaction
                    .sealed_target
                    .canonical_path
                    .to_string_lossy()
                    .into_owned(),
                anchor_device: transaction.sealed_target.anchor_identity.device,
                anchor_inode: transaction.sealed_target.anchor_identity.inode,
                target_device: transaction
                    .sealed_target
                    .target_identity
                    .map(|identity| identity.device),
                target_inode: transaction
                    .sealed_target
                    .target_identity
                    .map(|identity| identity.inode),
            })
            .collect();
        let read = self
            .prepared_read_capability()
            .map(|capability| PlannedReadResource {
                tool_name: capability.tool_name.as_str(),
                argument_name: capability.argument_name.as_str(),
                requested_path: capability.requested_path.as_str(),
                resolved_path: capability.resolved_path.to_string_lossy().into_owned(),
                anchor_device: capability.anchor_identity.device,
                anchor_inode: capability.anchor_identity.inode,
                parent_device: capability.parent_identity.device,
                parent_inode: capability.parent_identity.inode,
                file_device: capability.file_identity.device,
                file_inode: capability.file_identity.inode,
                content_hash: capability.content_hash.as_str(),
            });
        PlannedResourceSummary {
            schema_version: 1,
            writes,
            read,
        }
    }

    pub(super) fn mark_receipt_finalized(&mut self) {
        self.reservation.mark_finalized();
        self.receipt_guard_armed = false;
    }

    pub(super) fn disarm_receipt_guard(&mut self) {
        self.receipt_guard_armed = false;
    }

    pub(super) fn trip_outcome_breaker(&self, reason: impl Into<String>) {
        let reason = reason.into();
        match self.reservation.state.lock() {
            Ok(mut state) => state.breaker.trip_fatal(reason),
            Err(poisoned) => poisoned.into_inner().breaker.trip_fatal(reason),
        }
    }
}

#[derive(Serialize)]
struct PlannedResourceSummary<'a> {
    schema_version: u32,
    writes: Vec<PlannedWriteResource<'a>>,
    read: Option<PlannedReadResource<'a>>,
}

#[derive(Serialize)]
struct PlannedWriteResource<'a> {
    operation: &'a str,
    requested_path: &'a str,
    target_path: &'a str,
    mode_requested: &'a str,
    preview_only: bool,
    target_existed_before: bool,
    canonical_namespace: String,
    anchor_device: u64,
    anchor_inode: u64,
    target_device: Option<u64>,
    target_inode: Option<u64>,
}

#[derive(Serialize)]
struct PlannedReadResource<'a> {
    tool_name: &'a str,
    argument_name: &'a str,
    requested_path: &'a str,
    resolved_path: String,
    anchor_device: u64,
    anchor_inode: u64,
    parent_device: u64,
    parent_inode: u64,
    file_device: u64,
    file_inode: u64,
    content_hash: &'a str,
}

impl std::fmt::Debug for AuthorizedToolExecution {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AuthorizedToolExecution")
            .field("attempt_id", &self.attempt_id())
            .field("tool_name", &self.tool_name)
            .field("executor", &self.executor)
            .field(
                "write_target_reserved",
                &self.execution_lease.holds_write_target_reservation(),
            )
            .field("receipt_guard_armed", &self.receipt_guard_armed)
            .finish_non_exhaustive()
    }
}

impl Drop for AuthorizedToolExecution {
    fn drop(&mut self) {
        if self.receipt_guard_armed {
            OutcomeRecorder::finalize_authorized_drop(self);
        }
    }
}

pub(super) fn validate_capability_material(
    approved: &ExactApprovalMaterial,
    presented: &ExactApprovalMaterial,
) -> Result<(), HeptaError> {
    if approved.capability_descriptor != presented.capability_descriptor {
        return Err(HeptaError(
            "capability binding denied: approval_descriptor_substitution".into(),
        ));
    }
    for (label, material) in [("approved", approved), ("presented", presented)] {
        let [request] = material.candidate.capability_requests() else {
            return Err(HeptaError(format!(
                "capability binding denied: {label}_candidate_request_count"
            )));
        };
        if request.capability() != &material.capability_descriptor.reference() {
            return Err(HeptaError(format!(
                "capability binding denied: {label}_manifest_reference_mismatch"
            )));
        }
    }
    Ok(())
}

mod runtime;

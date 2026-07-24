#[path = "safety_gate_client/admission.rs"]
mod admission;
use super::approval_state::ApprovalState;
use super::approval_state::EXACT_PENDING_TTL;
use super::approval_state::ExactApprovalMaterial;
use super::context_freezer::add_ttl;
use super::context_freezer::authorization_scope_hash;
use super::context_freezer::framed_hash;
use super::context_freezer::freeze_tool_inputs;
use super::context_freezer::now_ms;
use super::execution_attempt::AuthorizedToolExecution;
use super::execution_attempt::PreparedExecutionAttempt;
#[cfg(test)]
use super::execution_attempt::validate_capability_material;
use super::execution_lease::ResourceBoundExecutionLease;
use crate::CorrelationId;
use crate::HeptaError;
use crate::ModelRef;
use crate::PolicyDecision;
use crate::RiskTier;
use crate::RuntimeKernel;
use crate::SessionId;
use admission::admit_exact_tool_candidate;
use hepta_contracts::AuthorizationId;
use hepta_contracts::PrincipalId;
use hepta_intelligence::ToolCandidateProposalInput;
use hepta_intelligence::propose_tool_candidate;
use hepta_kernel::HeptaKernelSafetyAuthorization;
use hepta_kernel::HeptaKernelSafetyGate;

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct SafetyGateClient;

impl SafetyGateClient {
    pub(crate) fn prepare_candidate(
        runtime: &RuntimeKernel,
        session_id: &str,
        active_model: &ModelRef,
        tool_name: &str,
        arguments_json: &str,
        decision: &PolicyDecision,
    ) -> Result<ExactApprovalMaterial, HeptaError> {
        let frozen = freeze_tool_inputs(
            runtime,
            session_id,
            active_model,
            tool_name,
            arguments_json,
            decision,
        )?;
        let requester = PrincipalId::new(format!(
            "model:{}/{}",
            active_model.provider, active_model.model
        ));
        let capability_descriptor = frozen.capability;
        let candidate = propose_tool_candidate(ToolCandidateProposalInput {
            context: frozen.context,
            capability: capability_descriptor.reference(),
            requester: requester.clone(),
            payload_hash: frozen.payload_hash.clone(),
            metacontrol_hash: frozen.metacontrol_hash,
            contributors: vec![requester, PrincipalId::new("runtime:turn-coordinator")],
        })
        .map_err(|error| HeptaError(format!("failed to propose exact tool candidate: {error}")))?;
        let admission = admit_exact_tool_candidate(
            runtime,
            session_id,
            active_model,
            tool_name,
            decision,
            &candidate,
            capability_descriptor.clone(),
            frozen.payload_hash.clone(),
        )?;
        Ok(ExactApprovalMaterial {
            tool_name: tool_name.to_string(),
            reason: decision.reason.clone(),
            canonical_arguments: frozen.canonical_arguments,
            payload_hash: frozen.payload_hash,
            capability_descriptor,
            candidate,
            admission,
            expires_at_unix_ms: add_ttl(now_ms(), EXACT_PENDING_TTL),
        })
    }

    pub(crate) fn proactive_allowed(
        runtime: &RuntimeKernel,
        tool_name: &str,
    ) -> Result<bool, HeptaError> {
        let metadata = runtime.tools.execution_metadata(tool_name)?;
        let risk = runtime.tools.risk_tier(tool_name)?;
        let explicitly_local_read = matches!(
            tool_name,
            "echo"
                | "read"
                | "read_file"
                | "list_dir"
                | "search_text"
                | "disk_junk_audit"
                | "json_get"
                | "skill_scan"
                | "tool_manifest_validate"
                | "agents_list"
                | "sessions_list"
                | "sessions_history"
                | "session_status"
                | "image"
                | "pdf"
                | "memory_search"
                | "memory_get"
        );
        Ok(explicitly_local_read
            && metadata.read_only
            && !metadata.destructive
            && metadata.idempotent
            && risk != RiskTier::High)
    }

    #[cfg(test)]
    pub(crate) fn authorize_without_grant(
        approved: &ExactApprovalMaterial,
        presented: &ExactApprovalMaterial,
    ) -> Result<HeptaKernelSafetyAuthorization, HeptaError> {
        Self::authorize_test_only(approved, presented)
    }

    pub(crate) fn authorize_execution_without_grant(
        runtime: &RuntimeKernel,
        session_id: &SessionId,
        correlation_id: &CorrelationId,
        approved: &ExactApprovalMaterial,
        presented: &ExactApprovalMaterial,
        execution_lease: ResourceBoundExecutionLease,
    ) -> Result<AuthorizedToolExecution, HeptaError> {
        let attempt = PreparedExecutionAttempt::prepare(
            runtime,
            session_id,
            correlation_id,
            approved,
            presented,
            execution_lease,
        )?;
        let authorization = Self::authorize(approved, presented, &attempt)?;
        Ok(attempt.authorize(authorization, runtime.outcome_sink.clone()))
    }

    pub(crate) fn authorize_execution_and_consume(
        runtime: &RuntimeKernel,
        approvals: &mut ApprovalState,
        session_id: &str,
        typed_session_id: &SessionId,
        correlation_id: &CorrelationId,
        approved: &ExactApprovalMaterial,
        presented: &ExactApprovalMaterial,
        execution_lease: ResourceBoundExecutionLease,
    ) -> Result<AuthorizedToolExecution, HeptaError> {
        let attempt = PreparedExecutionAttempt::prepare(
            runtime,
            typed_session_id,
            correlation_id,
            approved,
            presented,
            execution_lease,
        )?;
        let now = now_ms();
        approvals.purge_expired(now);
        let Some((session_index, grant_index)) = approvals.grant_index(session_id, approved, now)
        else {
            return Err(HeptaError(
                "exact approval is missing, expired, or already consumed".into(),
            ));
        };
        let authorization = Self::authorize(approved, presented, &attempt);
        approvals.remove_grant(session_index, grant_index);
        Ok(attempt.authorize(authorization?, runtime.outcome_sink.clone()))
    }

    fn authorize(
        approved: &ExactApprovalMaterial,
        presented: &ExactApprovalMaterial,
        attempt: &PreparedExecutionAttempt,
    ) -> Result<HeptaKernelSafetyAuthorization, HeptaError> {
        let current_context = presented.candidate.context();
        let scope_hash = authorization_scope_hash(
            attempt.attempt_id(),
            &approved.tool_name,
            &presented.payload_hash,
            attempt.capability(),
            attempt.executor(),
            current_context,
        );
        let authorization_hash = framed_hash(
            "hepta.runtime.authorization-id.v2",
            &[
                ("attempt_id", attempt.attempt_id().as_bytes()),
                (
                    "admission",
                    approved
                        .admission
                        .admission()
                        .content_hash()
                        .as_str()
                        .as_bytes(),
                ),
                (
                    "candidate_binding",
                    approved.binding_hash().as_str().as_bytes(),
                ),
                (
                    "executor_principal",
                    attempt.executor().principal().as_str().as_bytes(),
                ),
                (
                    "executor_provider",
                    attempt.executor().provider().as_bytes(),
                ),
                (
                    "executor_operation",
                    attempt.executor().operation().as_bytes(),
                ),
                (
                    "capability_manifest",
                    attempt.capability().manifest_hash().as_str().as_bytes(),
                ),
                ("scope", scope_hash.as_str().as_bytes()),
            ],
        );
        HeptaKernelSafetyGate::new()
            .authorize_at_commit(
                AuthorizationId::new(format!("authorization:{}", authorization_hash.as_str())),
                presented.candidate.revision(),
                &approved.admission,
                approved.admission.binding(),
                &presented.candidate,
                &presented.payload_hash,
                current_context,
                PrincipalId::new("kernel:safety-gate"),
                scope_hash,
            )
            .map_err(|error| {
                HeptaError(format!("commit-time safety gate denied: {}", error.code()))
            })
    }

    #[cfg(test)]
    fn authorize_test_only(
        approved: &ExactApprovalMaterial,
        presented: &ExactApprovalMaterial,
    ) -> Result<HeptaKernelSafetyAuthorization, HeptaError> {
        validate_capability_material(approved, presented)?;
        let current_context = presented.candidate.context();
        let scope_hash = framed_hash(
            "hepta.runtime.authorization-scope.test-only.v1",
            &[
                ("tool_name", approved.tool_name.as_bytes()),
                ("payload_hash", presented.payload_hash.as_str().as_bytes()),
            ],
        );
        let authorization_hash = framed_hash(
            "hepta.runtime.authorization-id.test-only.v1",
            &[
                (
                    "admission",
                    approved
                        .admission
                        .admission()
                        .content_hash()
                        .as_str()
                        .as_bytes(),
                ),
                (
                    "candidate_binding",
                    approved.binding_hash().as_str().as_bytes(),
                ),
                ("scope", scope_hash.as_str().as_bytes()),
            ],
        );
        HeptaKernelSafetyGate::new()
            .authorize_at_commit(
                AuthorizationId::new(format!("authorization:{}", authorization_hash.as_str())),
                presented.candidate.revision(),
                &approved.admission,
                approved.admission.binding(),
                &presented.candidate,
                &presented.payload_hash,
                current_context,
                PrincipalId::new("kernel:safety-gate"),
                scope_hash,
            )
            .map_err(|error| {
                HeptaError(format!("commit-time safety gate denied: {}", error.code()))
            })
    }

    pub(crate) fn reset_context_for_session(
        runtime: &RuntimeKernel,
        session_id: &str,
    ) -> Result<(), HeptaError> {
        let mut revisions = runtime
            .context_revision_state
            .lock()
            .map_err(|_| HeptaError("context revision state mutex poisoned".into()))?;
        revisions.remove_session(session_id);
        Ok(())
    }

    pub(crate) fn reset_all_context(runtime: &RuntimeKernel) -> Result<(), HeptaError> {
        let mut revisions = runtime
            .context_revision_state
            .lock()
            .map_err(|_| HeptaError("context revision state mutex poisoned".into()))?;
        revisions.clear();
        Ok(())
    }
}

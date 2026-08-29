use std::collections::HashMap;
use std::collections::HashSet;

use crate::AcceptedEvent;
use crate::Controller;
use crate::ControllerConfig;
use crate::ControllerSnapshot;
use crate::Digest;
use crate::EventFence;
use crate::InferError;
use crate::InferenceRequest;
use crate::RequestId;
use crate::Result;
use crate::StateEvent;
use crate::TerminalReceipt;

const OLLAMA_GRANITE4_1B_MODEL_ID_DIGEST: &str =
    "sha256:5e3d0ccafe264414f936b0e80d3ff9478da9509cce105f414878295902f82ba4";
const LMSTUDIO_GRANITE4_MICRO_MODEL_ID_DIGEST: &str =
    "sha256:834047c1a83968ab6d6b52dc1e00dc6cce748733cfad05bf27d05d85a0039900";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AdapterId {
    Ollama,
    LmStudio,
}

impl AdapterId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ollama => "ollama",
            Self::LmStudio => "lmstudio",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityEvidence {
    Qualified,
    UnsupportedFailClosed,
}

impl CapabilityEvidence {
    const fn is_qualified(self) -> bool {
        matches!(self, Self::Qualified)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdapterCapabilities {
    pub semantic_text: CapabilityEvidence,
    pub native_tool_call: CapabilityEvidence,
    pub strict_sse: CapabilityEvidence,
    pub direct_provider_cancel: CapabilityEvidence,
}

impl AdapterCapabilities {
    pub const fn fixed_ollama_granite4_1b() -> Self {
        Self {
            semantic_text: CapabilityEvidence::Qualified,
            native_tool_call: CapabilityEvidence::Qualified,
            strict_sse: CapabilityEvidence::UnsupportedFailClosed,
            direct_provider_cancel: CapabilityEvidence::UnsupportedFailClosed,
        }
    }

    pub const fn fixed_lmstudio_granite4_micro() -> Self {
        Self {
            semantic_text: CapabilityEvidence::Qualified,
            native_tool_call: CapabilityEvidence::UnsupportedFailClosed,
            strict_sse: CapabilityEvidence::UnsupportedFailClosed,
            direct_provider_cancel: CapabilityEvidence::UnsupportedFailClosed,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DispatchRequirements {
    pub semantic_text: bool,
    pub native_tool_call: bool,
    pub strict_sse: bool,
    pub provider_cancel_acknowledgement: bool,
}

impl DispatchRequirements {
    pub const fn semantic_text() -> Self {
        Self {
            semantic_text: true,
            native_tool_call: false,
            strict_sse: false,
            provider_cancel_acknowledgement: false,
        }
    }

    pub const fn native_tool_call() -> Self {
        Self {
            semantic_text: true,
            native_tool_call: true,
            strict_sse: false,
            provider_cancel_acknowledgement: false,
        }
    }

    pub const fn strict_sse() -> Self {
        Self {
            semantic_text: true,
            native_tool_call: false,
            strict_sse: true,
            provider_cancel_acknowledgement: false,
        }
    }

    pub const fn cancel_required() -> Self {
        Self {
            semantic_text: true,
            native_tool_call: false,
            strict_sse: false,
            provider_cancel_acknowledgement: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FallbackPolicy {
    pub text_fallback: bool,
    pub remote_fallback: bool,
    pub implicit_model_switch: bool,
    pub implicit_model_install: bool,
}

impl FallbackPolicy {
    pub const fn closed() -> Self {
        Self {
            text_fallback: false,
            remote_fallback: false,
            implicit_model_switch: false,
            implicit_model_install: false,
        }
    }

    fn validate(self) -> Result<()> {
        if self == Self::closed() {
            Ok(())
        } else {
            Err(InferError::AdapterFallbackEnabled)
        }
    }
}

impl Default for FallbackPolicy {
    fn default() -> Self {
        Self::closed()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactAdapterTuple {
    pub tuple_digest: Digest,
    pub model_id_digest: Digest,
    pub adapter: AdapterId,
    pub capabilities: AdapterCapabilities,
}

impl ExactAdapterTuple {
    pub fn fixed_ollama_granite4_1b(tuple_digest: Digest) -> Result<Self> {
        Ok(Self {
            tuple_digest,
            model_id_digest: Digest::parse(OLLAMA_GRANITE4_1B_MODEL_ID_DIGEST)?,
            adapter: AdapterId::Ollama,
            capabilities: AdapterCapabilities::fixed_ollama_granite4_1b(),
        })
    }

    pub fn fixed_lmstudio_granite4_micro(tuple_digest: Digest) -> Result<Self> {
        Ok(Self {
            tuple_digest,
            model_id_digest: Digest::parse(LMSTUDIO_GRANITE4_MICRO_MODEL_ID_DIGEST)?,
            adapter: AdapterId::LmStudio,
            capabilities: AdapterCapabilities::fixed_lmstudio_granite4_micro(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyProfile {
    pub policy_digest: Digest,
    pub requirements: DispatchRequirements,
}

impl PolicyProfile {
    pub fn new(policy_digest: Digest, requirements: DispatchRequirements) -> Self {
        Self {
            policy_digest,
            requirements,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterAdmission {
    pub adapter: AdapterId,
    pub tuple_digest: Digest,
    pub model_id_digest: Digest,
    pub policy_digest: Digest,
    pub fallback_attempts: u8,
}

#[derive(Clone, Debug)]
pub struct AdapterRegistry {
    tuples: HashMap<Digest, ExactAdapterTuple>,
    policies: HashMap<Digest, DispatchRequirements>,
    fallback: FallbackPolicy,
}

impl AdapterRegistry {
    pub fn new(
        tuples: impl IntoIterator<Item = ExactAdapterTuple>,
        policies: impl IntoIterator<Item = PolicyProfile>,
        fallback: FallbackPolicy,
    ) -> Result<Self> {
        fallback.validate()?;
        let mut tuple_map = HashMap::new();
        for tuple in tuples {
            if tuple_map
                .insert(tuple.tuple_digest.clone(), tuple)
                .is_some()
            {
                return Err(InferError::AdapterConfigInvalid);
            }
        }
        let mut policy_map = HashMap::new();
        for policy in policies {
            if policy_map
                .insert(policy.policy_digest, policy.requirements)
                .is_some()
            {
                return Err(InferError::AdapterConfigInvalid);
            }
        }
        if tuple_map.is_empty() || policy_map.is_empty() {
            return Err(InferError::AdapterConfigInvalid);
        }
        Ok(Self {
            tuples: tuple_map,
            policies: policy_map,
            fallback,
        })
    }

    pub fn tuple_digests(&self) -> HashSet<Digest> {
        self.tuples.keys().cloned().collect()
    }

    pub fn validate_for_controller(&self, controller: &ControllerConfig) -> Result<()> {
        self.fallback.validate()?;
        if self.tuple_digests() == controller.registered_tuples {
            Ok(())
        } else {
            Err(InferError::AdapterConfigInvalid)
        }
    }

    pub fn admit(&self, request: &InferenceRequest) -> Result<AdapterAdmission> {
        self.fallback.validate()?;
        let tuple = self
            .tuples
            .get(&request.model_tuple_digest)
            .ok_or(InferError::UnknownModelTuple)?;
        let requirements = self
            .policies
            .get(&request.policy_digest)
            .ok_or(InferError::AdapterPolicyUnknown)?;
        let capabilities = tuple.capabilities;
        if requirements.semantic_text && !capabilities.semantic_text.is_qualified() {
            return Err(InferError::AdapterSemanticTextUnsupported);
        }
        if requirements.native_tool_call && !capabilities.native_tool_call.is_qualified() {
            return Err(InferError::AdapterToolCallUnsupported);
        }
        if requirements.strict_sse && !capabilities.strict_sse.is_qualified() {
            return Err(InferError::AdapterStrictSseUnsupported);
        }
        if requirements.provider_cancel_acknowledgement
            && !capabilities.direct_provider_cancel.is_qualified()
        {
            return Err(InferError::AdapterProviderCancelUnsupported);
        }
        Ok(AdapterAdmission {
            adapter: tuple.adapter,
            tuple_digest: tuple.tuple_digest.clone(),
            model_id_digest: tuple.model_id_digest.clone(),
            policy_digest: request.policy_digest.clone(),
            fallback_attempts: 0,
        })
    }
}

#[derive(Debug)]
pub struct QualifiedController {
    controller: Controller,
    registry: AdapterRegistry,
    admissions: HashMap<RequestId, AdapterAdmission>,
    requests: HashMap<RequestId, InferenceRequest>,
}

impl QualifiedController {
    pub fn new(
        config: ControllerConfig,
        backend_generation: u64,
        registry: AdapterRegistry,
    ) -> Result<Self> {
        registry.validate_for_controller(&config)?;
        Ok(Self {
            controller: Controller::new(config, backend_generation)?,
            registry,
            admissions: HashMap::new(),
            requests: HashMap::new(),
        })
    }

    pub fn backend_generation(&self) -> u64 {
        self.controller.backend_generation()
    }

    pub fn admit(&mut self, request: InferenceRequest, now_unix_ms: u64) -> Result<AcceptedEvent> {
        let admission = self.registry.admit(&request)?;
        let request_id = request.identity.request_id.clone();
        if self.admissions.contains_key(&request_id) || self.requests.contains_key(&request_id) {
            return Err(InferError::AdapterConfigInvalid);
        }
        let retained_request = request.clone();
        let event = self.controller.admit(request, now_unix_ms)?;
        self.admissions.insert(request_id.clone(), admission);
        self.requests.insert(request_id, retained_request);
        Ok(event)
    }

    pub fn adapter_admission(&self, request_id: &RequestId) -> Result<&AdapterAdmission> {
        self.admissions
            .get(request_id)
            .ok_or(InferError::UnknownRequest)
    }

    pub fn request(&self, request_id: &RequestId) -> Result<&InferenceRequest> {
        self.requests
            .get(request_id)
            .ok_or(InferError::UnknownRequest)
    }

    pub fn start(
        &mut self,
        request_id: &RequestId,
        request_generation: u64,
        backend_generation: u64,
    ) -> Result<StateEvent> {
        self.controller
            .start(request_id, request_generation, backend_generation)
    }

    pub fn publish_token(
        &mut self,
        fence: EventFence<'_>,
        token_digest: &Digest,
        token_byte_length: u64,
    ) -> Result<StateEvent> {
        self.controller
            .publish_token(fence, token_digest, token_byte_length)
    }

    pub fn complete(
        &mut self,
        fence: EventFence<'_>,
        result_digest: Digest,
        output_tokens: u32,
    ) -> Result<TerminalReceipt> {
        self.controller
            .complete(fence, result_digest, output_tokens)
    }

    pub fn cancel(
        &mut self,
        request_id: &RequestId,
        request_generation: u64,
        cancel_generation: u64,
        backend_generation: u64,
    ) -> Result<TerminalReceipt> {
        self.controller.cancel(
            request_id,
            request_generation,
            cancel_generation,
            backend_generation,
        )
    }

    pub fn restart_backend(&mut self, expected_generation: u64) -> Result<Vec<TerminalReceipt>> {
        let receipts = self.controller.restart_backend(expected_generation)?;
        self.admissions.clear();
        self.requests.clear();
        Ok(receipts)
    }

    pub fn terminal_receipt(&self, request_id: &RequestId) -> Result<&TerminalReceipt> {
        self.controller.terminal_receipt(request_id)
    }

    pub fn terminal_receipt_fenced(
        &self,
        request_id: &RequestId,
        request_generation: u64,
        backend_generation: u64,
        minimum_sequence: u64,
    ) -> Result<&TerminalReceipt> {
        self.controller.terminal_receipt_fenced(
            request_id,
            request_generation,
            backend_generation,
            minimum_sequence,
        )
    }

    pub fn snapshot(&self) -> ControllerSnapshot {
        self.controller.snapshot()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::AgentId;
    use crate::AuthoritySnapshot;
    use crate::RequestIdentity;
    use crate::ResourceBudgetId;
    use crate::TaskId;
    use crate::TenantId;
    use crate::WorkspaceId;

    fn must<T>(result: Result<T>) -> T {
        match result {
            Ok(value) => value,
            Err(error) => panic!("unexpected error: {error}"),
        }
    }

    fn digest(fill: char) -> Digest {
        must(Digest::parse(&format!(
            "sha256:{}",
            fill.to_string().repeat(64)
        )))
    }

    fn request(tuple: Digest, policy: Digest, name: &str) -> InferenceRequest {
        InferenceRequest {
            identity: RequestIdentity {
                tenant_id: must(TenantId::parse("tenant-a")),
                workspace_id: must(WorkspaceId::parse("workspace-a")),
                agent_id: must(AgentId::parse("agent-a")),
                task_id: must(TaskId::parse("task-a")),
                request_id: must(RequestId::parse(name)),
            },
            agent_generation: 1,
            request_generation: 1,
            cancel_generation: 0,
            deadline_unix_ms: 10_000,
            model_tuple_digest: tuple,
            policy_digest: policy,
            resource_budget_id: must(ResourceBudgetId::parse("budget-a")),
            prompt_digest: digest('c'),
            prompt_byte_length: 12,
            output_token_limit: 32,
            authority: AuthoritySnapshot::qualification_only_closed(),
        }
    }

    fn controller_config(tuple: Digest) -> ControllerConfig {
        let mut tuples = HashSet::new();
        tuples.insert(tuple);
        ControllerConfig {
            max_queue: 4,
            max_per_tenant: 2,
            registered_tuples: tuples,
            authority: AuthoritySnapshot::qualification_only_closed(),
        }
    }

    #[test]
    fn exact_policy_and_tuple_select_one_adapter_without_fallback() {
        let tuple = digest('a');
        let policy = digest('b');
        let registry = must(AdapterRegistry::new(
            [must(ExactAdapterTuple::fixed_ollama_granite4_1b(
                tuple.clone(),
            ))],
            [PolicyProfile::new(
                policy.clone(),
                DispatchRequirements::semantic_text(),
            )],
            FallbackPolicy::closed(),
        ));
        let admission = must(registry.admit(&request(tuple, policy, "request-admit")));
        assert_eq!(admission.adapter, AdapterId::Ollama);
        assert_eq!(admission.fallback_attempts, 0);
    }

    #[test]
    fn unsupported_capabilities_reject_before_controller_queueing() {
        let tuple = digest('a');
        let tool_policy = digest('b');
        let cancel_policy = digest('d');
        let registry = must(AdapterRegistry::new(
            [must(ExactAdapterTuple::fixed_lmstudio_granite4_micro(
                tuple.clone(),
            ))],
            [
                PolicyProfile::new(
                    tool_policy.clone(),
                    DispatchRequirements::native_tool_call(),
                ),
                PolicyProfile::new(
                    cancel_policy.clone(),
                    DispatchRequirements::cancel_required(),
                ),
            ],
            FallbackPolicy::closed(),
        ));
        let mut controller = must(QualifiedController::new(
            controller_config(tuple.clone()),
            7,
            registry,
        ));
        assert_eq!(
            controller.admit(request(tuple.clone(), tool_policy, "request-tool"), 1),
            Err(InferError::AdapterToolCallUnsupported)
        );
        assert_eq!(
            controller.admit(request(tuple, cancel_policy, "request-cancel"), 1),
            Err(InferError::AdapterProviderCancelUnsupported)
        );
        assert_eq!(controller.snapshot().queued_requests, 0);
    }

    #[test]
    fn strict_sse_is_fail_closed_for_both_fixed_adapters() {
        for (index, tuple) in [
            must(ExactAdapterTuple::fixed_ollama_granite4_1b(digest('a'))),
            must(ExactAdapterTuple::fixed_lmstudio_granite4_micro(digest(
                'd',
            ))),
        ]
        .into_iter()
        .enumerate()
        {
            let policy = digest(if index == 0 { 'b' } else { 'e' });
            let request = request(tuple.tuple_digest.clone(), policy.clone(), "request-sse");
            let registry = must(AdapterRegistry::new(
                [tuple],
                [PolicyProfile::new(
                    policy,
                    DispatchRequirements::strict_sse(),
                )],
                FallbackPolicy::closed(),
            ));
            assert_eq!(
                registry.admit(&request),
                Err(InferError::AdapterStrictSseUnsupported)
            );
        }
    }

    #[test]
    fn unknown_policy_and_any_fallback_fail_closed() {
        let tuple = digest('a');
        let registry = must(AdapterRegistry::new(
            [must(ExactAdapterTuple::fixed_ollama_granite4_1b(
                tuple.clone(),
            ))],
            [PolicyProfile::new(
                digest('b'),
                DispatchRequirements::semantic_text(),
            )],
            FallbackPolicy::closed(),
        ));
        assert_eq!(
            registry.admit(&request(tuple.clone(), digest('c'), "request-policy")),
            Err(InferError::AdapterPolicyUnknown)
        );
        assert!(matches!(
            AdapterRegistry::new(
                [must(ExactAdapterTuple::fixed_ollama_granite4_1b(tuple))],
                [PolicyProfile::new(
                    digest('b'),
                    DispatchRequirements::semantic_text(),
                )],
                FallbackPolicy {
                    text_fallback: true,
                    ..FallbackPolicy::closed()
                },
            ),
            Err(InferError::AdapterFallbackEnabled)
        ));
    }

    #[test]
    fn registry_and_controller_tuple_sets_must_match_exactly() {
        let controller_tuple = digest('a');
        let registry_tuple = digest('d');
        let registry = must(AdapterRegistry::new(
            [must(ExactAdapterTuple::fixed_ollama_granite4_1b(
                registry_tuple,
            ))],
            [PolicyProfile::new(
                digest('b'),
                DispatchRequirements::semantic_text(),
            )],
            FallbackPolicy::closed(),
        ));
        assert!(matches!(
            QualifiedController::new(controller_config(controller_tuple), 7, registry),
            Err(InferError::AdapterConfigInvalid)
        ));
    }
}

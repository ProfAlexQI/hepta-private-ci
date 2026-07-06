use super::catalog::context_source_registry_entries;
use super::entry::ContextSourceActivationGuard;
use super::entry::ContextSourceRedactionPolicy;
use super::entry::ContextSourceTtl;

pub(crate) struct ContextSourceRegistryHealthReport {
    pub(crate) source_count: usize,
    pub(crate) descriptor_field_count: usize,
    pub(crate) turn_ttl_count: usize,
    pub(crate) session_ttl_count: usize,
    pub(crate) prompt_hash_only_count: usize,
    pub(crate) guarded_envelope_count: usize,
    pub(crate) metadata_only_count: usize,
    pub(crate) compression_candidate_count: usize,
    pub(crate) operator_approval_required_count: usize,
    pub(crate) live_activation_route_count: usize,
    pub(crate) runtime_activation: &'static str,
}

pub(crate) fn context_source_registry_health_report() -> ContextSourceRegistryHealthReport {
    let entries = context_source_registry_entries();
    ContextSourceRegistryHealthReport {
        source_count: entries.len(),
        descriptor_field_count: 14,
        turn_ttl_count: entries
            .iter()
            .filter(|entry| entry.ttl == ContextSourceTtl::Turn)
            .count(),
        session_ttl_count: entries
            .iter()
            .filter(|entry| entry.ttl == ContextSourceTtl::Session)
            .count(),
        prompt_hash_only_count: entries
            .iter()
            .filter(|entry| entry.redaction_policy == ContextSourceRedactionPolicy::PromptHashOnly)
            .count(),
        guarded_envelope_count: entries
            .iter()
            .filter(|entry| entry.redaction_policy == ContextSourceRedactionPolicy::GuardedEnvelope)
            .count(),
        metadata_only_count: entries
            .iter()
            .filter(|entry| entry.redaction_policy == ContextSourceRedactionPolicy::MetadataOnly)
            .count(),
        compression_candidate_count: entries
            .iter()
            .filter(|entry| entry.default_compression_kind().is_some())
            .count(),
        operator_approval_required_count: entries
            .iter()
            .filter(|entry| {
                entry.activation_guard == ContextSourceActivationGuard::OperatorApprovalRequired
            })
            .count(),
        live_activation_route_count: entries
            .iter()
            .filter(|entry| entry.activation_guard.allows_live_activation())
            .count(),
        runtime_activation: "disabled",
    }
}

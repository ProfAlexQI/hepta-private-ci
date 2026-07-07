use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use super::recall::ContextBudget;
use super::recall::ContextRecallBundle;
use super::recall::ContextRecallLimitPressure;
use super::recall::ContextRecallRequest;
use super::recall::ContextRecallSourceCounts;

/// Runtime role for a memory provider exposed through the Hepta-native memory
/// plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryProviderKind {
    Builtin,
    External,
}

/// Activation state for a provider descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryProviderStatus {
    Active,
    Available,
    Rejected,
}

/// Capability advertised by a memory provider without binding Hepta to any
/// particular third-party backend implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryProviderCapability {
    ProfileCard,
    SemanticSearch,
    Reasoning,
    ContextSnapshot,
    Conclusions,
    Prefetch,
    Sync,
    Delete,
}

/// Stable descriptor for one memory backend in the provider plane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderDescriptor {
    pub id: String,
    pub kind: MemoryProviderKind,
    pub status: MemoryProviderStatus,
    #[serde(default)]
    pub capabilities: Vec<MemoryProviderCapability>,
    #[serde(default)]
    pub tool_names: Vec<String>,
    pub context_fencing_required: bool,
    pub prefetch_enabled: bool,
    pub sync_enabled: bool,
    pub external_exclusive: bool,
    pub provenance_required: bool,
    pub deletion_supported: bool,
    pub summary: String,
}

impl MemoryProviderDescriptor {
    pub fn builtin() -> Self {
        Self {
            id: "builtin".into(),
            kind: MemoryProviderKind::Builtin,
            status: MemoryProviderStatus::Active,
            capabilities: vec![
                MemoryProviderCapability::SemanticSearch,
                MemoryProviderCapability::ContextSnapshot,
                MemoryProviderCapability::Prefetch,
                MemoryProviderCapability::Sync,
                MemoryProviderCapability::Delete,
            ],
            tool_names: vec!["memory".into(), "recall".into()],
            context_fencing_required: true,
            prefetch_enabled: true,
            sync_enabled: true,
            external_exclusive: false,
            provenance_required: true,
            deletion_supported: true,
            summary: "Hepta builtin transcript/memory recall provider".into(),
        }
    }

    pub fn external_slot(id: impl Into<String>, status: MemoryProviderStatus) -> Self {
        Self {
            id: id.into(),
            kind: MemoryProviderKind::External,
            status,
            capabilities: vec![
                MemoryProviderCapability::ProfileCard,
                MemoryProviderCapability::SemanticSearch,
                MemoryProviderCapability::Reasoning,
                MemoryProviderCapability::ContextSnapshot,
                MemoryProviderCapability::Conclusions,
                MemoryProviderCapability::Prefetch,
                MemoryProviderCapability::Sync,
                MemoryProviderCapability::Delete,
            ],
            tool_names: vec![
                "profile".into(),
                "search".into(),
                "reasoning".into(),
                "context".into(),
                "conclude".into(),
            ],
            context_fencing_required: true,
            prefetch_enabled: status == MemoryProviderStatus::Active,
            sync_enabled: status == MemoryProviderStatus::Active,
            external_exclusive: true,
            provenance_required: true,
            deletion_supported: true,
            summary: "External user-modeling provider slot; at most one may be active".into(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == MemoryProviderStatus::Active
    }
}

/// Machine-readable summary of the Hepta-native memory provider plane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderPlaneReport {
    pub provider_count: usize,
    pub active_provider_count: usize,
    pub external_provider_count: usize,
    pub active_external_provider_count: usize,
    pub builtin_present: bool,
    pub exactly_one_external_active_or_none: bool,
    pub context_fencing_required: bool,
    pub all_active_providers_prefetch: bool,
    pub all_active_providers_sync: bool,
    pub provenance_required: bool,
    pub deletion_path_available: bool,
    pub capability_count: usize,
    #[serde(default)]
    pub capabilities: Vec<MemoryProviderCapability>,
    #[serde(default)]
    pub providers: Vec<MemoryProviderDescriptor>,
}

impl MemoryProviderPlaneReport {
    pub fn from_providers(providers: Vec<MemoryProviderDescriptor>) -> Self {
        let provider_count = providers.len();
        let active_provider_count = providers
            .iter()
            .filter(|provider| provider.is_active())
            .count();
        let external_provider_count = providers
            .iter()
            .filter(|provider| provider.kind == MemoryProviderKind::External)
            .count();
        let active_external_provider_count = providers
            .iter()
            .filter(|provider| {
                provider.kind == MemoryProviderKind::External && provider.is_active()
            })
            .count();
        let active = providers
            .iter()
            .filter(|provider| provider.is_active())
            .collect::<Vec<_>>();
        let mut capabilities = BTreeSet::new();
        for provider in &providers {
            for capability in &provider.capabilities {
                capabilities.insert(*capability);
            }
        }

        Self {
            provider_count,
            active_provider_count,
            external_provider_count,
            active_external_provider_count,
            builtin_present: providers.iter().any(|provider| {
                provider.kind == MemoryProviderKind::Builtin && provider.is_active()
            }),
            exactly_one_external_active_or_none: active_external_provider_count <= 1,
            context_fencing_required: active
                .iter()
                .all(|provider| provider.context_fencing_required),
            all_active_providers_prefetch: active.iter().all(|provider| provider.prefetch_enabled),
            all_active_providers_sync: active.iter().all(|provider| provider.sync_enabled),
            provenance_required: active.iter().all(|provider| provider.provenance_required),
            deletion_path_available: active.iter().any(|provider| provider.deletion_supported),
            capability_count: capabilities.len(),
            capabilities: capabilities.into_iter().collect(),
            providers,
        }
    }

    pub fn native_default() -> Self {
        Self::from_providers(vec![
            MemoryProviderDescriptor::builtin(),
            MemoryProviderDescriptor::external_slot(
                "external-user-modeling-slot",
                MemoryProviderStatus::Available,
            ),
        ])
    }

    pub fn contract_ready(&self) -> bool {
        self.builtin_present
            && self.exactly_one_external_active_or_none
            && self.context_fencing_required
            && self.all_active_providers_prefetch
            && self.all_active_providers_sync
            && self.provenance_required
            && self.deletion_path_available
    }
}

/// Context update posture for a memory provider.
///
/// The initial provider boundary is intentionally shadow-only: providers may
/// plan and report what would be attached, but this contract does not grant a
/// production prompt-injection route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryProviderContextUpdateMode {
    ShadowOnly,
}

/// Compact provider-owned context update envelope.
///
/// This is the runtime-facing handoff shape for future `update_context` work:
/// it carries counts, pressure, and safety booleans, not prompt text, query
/// text, transcript payloads, memory payloads, or ranked item payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderContextUpdateEnvelope {
    pub provider_id: String,
    pub mode: MemoryProviderContextUpdateMode,
    pub source_counts: ContextRecallSourceCounts,
    pub limit_pressure: ContextRecallLimitPressure,
    pub ranked_item_count: usize,
    pub selected_item_count: usize,
    pub estimated_token_count: usize,
    pub payload_light: bool,
    pub operator_approval_required: bool,
    pub prompt_payload_exported: bool,
    pub query_payload_exported: bool,
    pub ranked_payload_exported: bool,
    pub write_performed: bool,
    pub runtime_activation: bool,
}

impl MemoryProviderContextUpdateEnvelope {
    pub fn from_bundle(
        provider_id: impl Into<String>,
        bundle: &ContextRecallBundle,
        limit_pressure: ContextRecallLimitPressure,
    ) -> Self {
        let budget = if bundle.budget.max_items == 0 {
            ContextBudget::from_request(&bundle.request)
        } else {
            bundle.budget
        };

        Self {
            provider_id: provider_id.into(),
            mode: MemoryProviderContextUpdateMode::ShadowOnly,
            source_counts: bundle.source_counts(),
            limit_pressure,
            ranked_item_count: bundle.ranked_items.len(),
            selected_item_count: bundle.total_item_count(),
            estimated_token_count: budget.max_tokens_estimate,
            payload_light: true,
            operator_approval_required: true,
            prompt_payload_exported: false,
            query_payload_exported: false,
            ranked_payload_exported: false,
            write_performed: false,
            runtime_activation: false,
        }
    }

    pub fn has_payload_light_boundary(&self) -> bool {
        self.payload_light
            && self.operator_approval_required
            && !self.prompt_payload_exported
            && !self.query_payload_exported
            && !self.ranked_payload_exported
            && !self.write_performed
            && !self.runtime_activation
    }
}

/// Provider query/update report that pairs a descriptor with the compact
/// context-update envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderReport {
    pub descriptor: MemoryProviderDescriptor,
    pub update_context: MemoryProviderContextUpdateEnvelope,
}

impl MemoryProviderReport {
    pub fn from_update(
        descriptor: MemoryProviderDescriptor,
        update_context: MemoryProviderContextUpdateEnvelope,
    ) -> Self {
        Self {
            descriptor,
            update_context,
        }
    }

    pub fn has_provider_boundary_integrity(&self) -> bool {
        self.descriptor.context_fencing_required
            && self.descriptor.provenance_required
            && self.update_context.has_payload_light_boundary()
    }
}

/// Clear scope requested at the provider boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryProviderClearScope {
    Session,
    LongTerm,
    All,
}

/// Clear request for a provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderClearRequest {
    pub scope: MemoryProviderClearScope,
    pub dry_run: bool,
    pub operator_approval_granted: bool,
}

/// Payload-light result for provider clear attempts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderClearReport {
    pub provider_id: String,
    pub scope: MemoryProviderClearScope,
    pub dry_run: bool,
    pub operator_approval_required: bool,
    pub blocked: bool,
    pub clear_performed: bool,
    pub affected_record_count: usize,
    pub prompt_payload_exported: bool,
    pub write_performed: bool,
    pub runtime_activation: bool,
}

impl MemoryProviderClearReport {
    pub fn dry_run(provider_id: impl Into<String>, scope: MemoryProviderClearScope) -> Self {
        Self {
            provider_id: provider_id.into(),
            scope,
            dry_run: true,
            operator_approval_required: true,
            blocked: false,
            clear_performed: false,
            affected_record_count: 0,
            prompt_payload_exported: false,
            write_performed: false,
            runtime_activation: false,
        }
    }

    pub fn blocked(provider_id: impl Into<String>, scope: MemoryProviderClearScope) -> Self {
        Self {
            provider_id: provider_id.into(),
            scope,
            dry_run: false,
            operator_approval_required: true,
            blocked: true,
            clear_performed: false,
            affected_record_count: 0,
            prompt_payload_exported: false,
            write_performed: false,
            runtime_activation: false,
        }
    }

    pub fn has_no_side_effects(&self) -> bool {
        !self.clear_performed
            && self.affected_record_count == 0
            && !self.prompt_payload_exported
            && !self.write_performed
            && !self.runtime_activation
    }
}

/// Runtime-facing provider boundary for context recall.
///
/// Implementations own query, update envelope planning, reporting, and clear
/// attempts so recall injection cannot be scattered across callers.
pub trait MemoryProvider: Send + Sync {
    async fn query(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallBundle, crate::MemoryError>;

    async fn update_context(
        &self,
        request: ContextRecallRequest,
    ) -> Result<MemoryProviderContextUpdateEnvelope, crate::MemoryError>;

    async fn report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<MemoryProviderReport, crate::MemoryError>;

    async fn clear(
        &self,
        request: MemoryProviderClearRequest,
    ) -> Result<MemoryProviderClearReport, crate::MemoryError>;
}

use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

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

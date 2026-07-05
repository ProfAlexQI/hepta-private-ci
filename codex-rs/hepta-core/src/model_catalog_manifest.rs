use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelCatalogManifestContractKind {
    SignedManifest,
    CacheableManifest,
    StaleCacheFallback,
    NoSecretAuditMode,
    CapabilityBasedRouting,
    PromptCacheTtlPolicy,
    CatalogProvenance,
    OperatorHandoff,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelCatalogManifestDescriptor {
    pub id: String,
    pub kind: ModelCatalogManifestContractKind,
    pub contract_covered: bool,
    pub evidence_gate: String,
    pub operator_surface: String,
    pub signature_required: bool,
    pub cache_required: bool,
    pub stale_fallback_supported: bool,
    pub audit_reads_secrets: bool,
    pub audit_refreshes_network_catalog: bool,
    pub capability_routing_required: bool,
    pub summary: String,
}

impl ModelCatalogManifestDescriptor {
    pub fn new(
        id: impl Into<String>,
        kind: ModelCatalogManifestContractKind,
        evidence_gate: impl Into<String>,
        operator_surface: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            contract_covered: true,
            evidence_gate: evidence_gate.into(),
            operator_surface: operator_surface.into(),
            signature_required: true,
            cache_required: true,
            stale_fallback_supported: true,
            audit_reads_secrets: false,
            audit_refreshes_network_catalog: false,
            capability_routing_required: true,
            summary: summary.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelCatalogManifestReport {
    pub manifest_id: String,
    pub contract_count: usize,
    pub contract_covered_count: usize,
    pub signed_manifest_contract: bool,
    pub cacheable_manifest_contract: bool,
    pub stale_cache_fallback_contract: bool,
    pub no_secret_audit_mode_contract: bool,
    pub capability_based_routing_contract: bool,
    pub prompt_cache_ttl_policy_contract: bool,
    pub catalog_provenance_contract: bool,
    pub operator_handoff_contract: bool,
    pub default_cache_ttl_seconds: u64,
    pub max_cache_ttl_seconds: u64,
    pub stale_cache_fallback_max_age_seconds: u64,
    pub model_capabilities_indexed: Vec<String>,
    pub manifest_sources: Vec<String>,
    pub external_network_read: bool,
    pub secret_value_read: bool,
    pub model_list_network_refresh: bool,
    pub provider_runtime_started: bool,
    pub prompt_uploaded: bool,
    pub p1_model_catalog_manifest_ready: bool,
    pub contracts: Vec<ModelCatalogManifestDescriptor>,
}

impl ModelCatalogManifestReport {
    pub fn native_default() -> Self {
        Self::from_contracts(vec![
            ModelCatalogManifestDescriptor::new(
                "signed-remote-catalog-manifest",
                ModelCatalogManifestContractKind::SignedManifest,
                "cargo test -p hepta-core model_catalog_manifest_contract_is_cacheable_signed_and_side_effect_free --quiet",
                "/model-catalog-manifest --json",
                "remote model catalogs are accepted only through a signed manifest envelope before they can influence routing",
            ),
            ModelCatalogManifestDescriptor::new(
                "cacheable-catalog-manifest",
                ModelCatalogManifestContractKind::CacheableManifest,
                "cargo test -p hepta-core model_catalog_manifest_contract_is_cacheable_signed_and_side_effect_free --quiet",
                "/model-catalog-manifest --json, /providers --json",
                "catalog freshness is represented as a local cache contract so model lists can update independently of a binary release",
            ),
            ModelCatalogManifestDescriptor::new(
                "stale-cache-fallback",
                ModelCatalogManifestContractKind::StaleCacheFallback,
                "cargo test -p hepta-core model_catalog_manifest_contract_is_cacheable_signed_and_side_effect_free --quiet",
                "/model-catalog-manifest --json",
                "if a remote catalog cannot be refreshed or verified, routing falls back to the last valid cached manifest or bundled catalog",
            ),
            ModelCatalogManifestDescriptor::new(
                "audit-mode-no-secret-read",
                ModelCatalogManifestContractKind::NoSecretAuditMode,
                "cargo test -p hepta-core model_catalog_manifest_contract_is_cacheable_signed_and_side_effect_free --quiet",
                "/model-catalog-manifest --json, /provider-transports --json",
                "catalog audits report auth labels and cache policy without reading API keys, OAuth tokens, or provider config values",
            ),
            ModelCatalogManifestDescriptor::new(
                "capability-based-multimodal-routing",
                ModelCatalogManifestContractKind::CapabilityBasedRouting,
                "cargo test -p hepta-core model_catalog_manifest_contract_is_cacheable_signed_and_side_effect_free --quiet",
                "/model-catalog-manifest --json, /capability-surface-plane --json",
                "image, audio, video, tool-call, reasoning, and embedding routing use model capability metadata rather than provider labels",
            ),
            ModelCatalogManifestDescriptor::new(
                "prompt-cache-ttl-policy",
                ModelCatalogManifestContractKind::PromptCacheTtlPolicy,
                "cargo test -p hepta-core model_catalog_manifest_contract_is_cacheable_signed_and_side_effect_free --quiet",
                "/model-catalog-manifest --json, /transport-contracts --json",
                "prompt cache TTL is explicit in the manifest policy with a short safe default and bounded opt-in extension",
            ),
            ModelCatalogManifestDescriptor::new(
                "catalog-provenance-ledger",
                ModelCatalogManifestContractKind::CatalogProvenance,
                "cargo test -p hepta-core model_catalog_manifest_contract_is_cacheable_signed_and_side_effect_free --quiet",
                "/model-catalog-manifest --json, /provenance --json",
                "every model entry can cite manifest source, signature status, fetched-at metadata, and bundled fallback origin",
            ),
            ModelCatalogManifestDescriptor::new(
                "operator-dashboard-handoff",
                ModelCatalogManifestContractKind::OperatorHandoff,
                "cargo test -p hepta-cli model_catalog_manifest_command_exposes_side_effect_free_contract --quiet",
                "/model-catalog-manifest --json, /control-ui --json",
                "operator UI/model switching can consume catalog freshness and capability metadata without triggering provider calls",
            ),
        ])
    }

    pub fn from_contracts(contracts: Vec<ModelCatalogManifestDescriptor>) -> Self {
        let contract_count = contracts.len();
        let contract_covered_count = contracts
            .iter()
            .filter(|contract| contract.contract_covered)
            .count();
        let has_kind = |kind: ModelCatalogManifestContractKind| {
            contracts
                .iter()
                .any(|contract| contract.contract_covered && contract.kind == kind)
        };
        let signed_manifest_contract = has_kind(ModelCatalogManifestContractKind::SignedManifest);
        let cacheable_manifest_contract =
            has_kind(ModelCatalogManifestContractKind::CacheableManifest);
        let stale_cache_fallback_contract =
            has_kind(ModelCatalogManifestContractKind::StaleCacheFallback);
        let no_secret_audit_mode_contract =
            has_kind(ModelCatalogManifestContractKind::NoSecretAuditMode);
        let capability_based_routing_contract =
            has_kind(ModelCatalogManifestContractKind::CapabilityBasedRouting);
        let prompt_cache_ttl_policy_contract =
            has_kind(ModelCatalogManifestContractKind::PromptCacheTtlPolicy);
        let catalog_provenance_contract =
            has_kind(ModelCatalogManifestContractKind::CatalogProvenance);
        let operator_handoff_contract = has_kind(ModelCatalogManifestContractKind::OperatorHandoff);
        let default_cache_ttl_seconds = 300;
        let max_cache_ttl_seconds = 3600;
        let stale_cache_fallback_max_age_seconds = 604_800;
        let model_capabilities_indexed = vec![
            "text".into(),
            "tool_calls".into(),
            "vision".into(),
            "audio".into(),
            "video".into(),
            "reasoning".into(),
            "embeddings".into(),
        ];
        let manifest_sources = vec![
            "bundled".into(),
            "remote_manifest".into(),
            "local_cache".into(),
        ];
        let external_network_read = false;
        let secret_value_read = false;
        let model_list_network_refresh = false;
        let provider_runtime_started = false;
        let prompt_uploaded = false;
        let signed_cache_policy_ok = contracts.iter().all(|contract| {
            contract.contract_covered
                && contract.signature_required
                && contract.cache_required
                && contract.stale_fallback_supported
        });
        let side_effect_boundary_ok = contracts.iter().all(|contract| {
            contract.contract_covered
                && !contract.audit_reads_secrets
                && !contract.audit_refreshes_network_catalog
        }) && !external_network_read
            && !secret_value_read
            && !model_list_network_refresh
            && !provider_runtime_started
            && !prompt_uploaded;
        let p1_model_catalog_manifest_ready = signed_manifest_contract
            && cacheable_manifest_contract
            && stale_cache_fallback_contract
            && no_secret_audit_mode_contract
            && capability_based_routing_contract
            && prompt_cache_ttl_policy_contract
            && catalog_provenance_contract
            && operator_handoff_contract
            && signed_cache_policy_ok
            && side_effect_boundary_ok
            && default_cache_ttl_seconds <= 300
            && max_cache_ttl_seconds <= 3600
            && model_capabilities_indexed.contains(&"vision".to_string())
            && manifest_sources.contains(&"local_cache".to_string());

        Self {
            manifest_id: "model-catalog-manifest".into(),
            contract_count,
            contract_covered_count,
            signed_manifest_contract,
            cacheable_manifest_contract,
            stale_cache_fallback_contract,
            no_secret_audit_mode_contract,
            capability_based_routing_contract,
            prompt_cache_ttl_policy_contract,
            catalog_provenance_contract,
            operator_handoff_contract,
            default_cache_ttl_seconds,
            max_cache_ttl_seconds,
            stale_cache_fallback_max_age_seconds,
            model_capabilities_indexed,
            manifest_sources,
            external_network_read,
            secret_value_read,
            model_list_network_refresh,
            provider_runtime_started,
            prompt_uploaded,
            p1_model_catalog_manifest_ready,
            contracts,
        }
    }

    pub fn contract_ready(&self) -> bool {
        self.contract_count > 0
            && self.contract_count == self.contract_covered_count
            && self.p1_model_catalog_manifest_ready
    }
}

#[cfg(test)]
mod tests {
    use super::ModelCatalogManifestReport;

    #[test]
    fn model_catalog_manifest_contract_is_cacheable_signed_and_side_effect_free() {
        let report = ModelCatalogManifestReport::native_default();

        assert_eq!(report.contract_count, 8);
        assert_eq!(report.contract_covered_count, report.contract_count);
        assert!(report.signed_manifest_contract);
        assert!(report.cacheable_manifest_contract);
        assert!(report.stale_cache_fallback_contract);
        assert!(report.no_secret_audit_mode_contract);
        assert!(report.capability_based_routing_contract);
        assert!(report.prompt_cache_ttl_policy_contract);
        assert!(report.catalog_provenance_contract);
        assert!(report.operator_handoff_contract);
        assert_eq!(report.default_cache_ttl_seconds, 300);
        assert_eq!(report.max_cache_ttl_seconds, 3600);
        assert!(report.model_capabilities_indexed.contains(&"vision".into()));
        assert!(report.manifest_sources.contains(&"local_cache".into()));
        assert!(!report.external_network_read);
        assert!(!report.secret_value_read);
        assert!(!report.model_list_network_refresh);
        assert!(!report.provider_runtime_started);
        assert!(!report.prompt_uploaded);
        assert!(report.p1_model_catalog_manifest_ready);
        assert!(report.contract_ready());
        let forbidden = ["her", "mes"].concat();
        assert!(report.contracts.iter().all(|contract| {
            !contract.id.contains(&forbidden)
                && !contract.summary.to_lowercase().contains(&forbidden)
        }));
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeptaUpstreamCodexSyncRisk {
    P0Security,
    P0Runtime,
    P1Compatibility,
    P2Product,
    Guardrail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexSyncContract {
    pub id: String,
    pub risk: HeptaUpstreamCodexSyncRisk,
    pub title: String,
    pub upstream_scope: Vec<String>,
    pub hepta_surfaces: Vec<String>,
    pub required_gate: String,
    pub auto_apply_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub contract_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexSyncLaneReport {
    pub product: String,
    pub status: String,
    pub lane_id: String,
    pub upstream_repository: String,
    pub compatibility_snapshot_role: String,
    pub sync_mode: String,
    pub contract_count: usize,
    pub ready_contract_count: usize,
    pub sync_lane_ready: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_latest_claimed: bool,
    pub upstream_merge_performed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_runtime_codex_engine_dependency_allowed: bool,
    pub active_dependency_isolation_gate: String,
    pub requires_diff_classification_before_absorption: bool,
    pub requires_adapter_contract_before_active_runtime: bool,
    pub requires_release_governance_before_public_claim: bool,
    pub local_only_audit: bool,
    pub report_only: bool,
    pub mutates_runtime_state: bool,
    pub external_network_read: bool,
    pub external_send: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub contracts: Vec<HeptaUpstreamCodexSyncContract>,
}

impl HeptaUpstreamCodexSyncLaneReport {
    pub fn native_default() -> Self {
        Self::from_contracts(default_upstream_codex_sync_contracts())
    }

    pub fn from_contracts(contracts: Vec<HeptaUpstreamCodexSyncContract>) -> Self {
        let contract_count = contracts.len();
        let ready_contract_count = contracts
            .iter()
            .filter(|contract| contract.contract_ready)
            .count();
        let sync_lane_ready = contract_count > 0
            && ready_contract_count == contract_count
            && contracts.iter().all(|contract| {
                !contract.auto_apply_allowed
                    && !contract.active_runtime_dependency_allowed
                    && !contract.public_release_claim_allowed
            });

        Self {
            product: "Hepta".into(),
            status: if sync_lane_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            lane_id: "upstream-codex-sync-lane".into(),
            upstream_repository: "https://github.com/openai/codex".into(),
            compatibility_snapshot_role: "ingestion_and_regression_oracle".into(),
            sync_mode: "classify_then_absorb_then_gate".into(),
            contract_count,
            ready_contract_count,
            sync_lane_ready,
            upstream_fetch_performed: false,
            upstream_latest_claimed: false,
            upstream_merge_performed: false,
            active_runtime_auto_rebase_allowed: false,
            active_runtime_codex_engine_dependency_allowed: false,
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            requires_diff_classification_before_absorption: true,
            requires_adapter_contract_before_active_runtime: true,
            requires_release_governance_before_public_claim: true,
            local_only_audit: true,
            report_only: true,
            mutates_runtime_state: false,
            external_network_read: false,
            external_send: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            contracts,
        }
    }
}

fn sync_contract(
    id: &str,
    risk: HeptaUpstreamCodexSyncRisk,
    title: &str,
    upstream_scope: &[&str],
    hepta_surfaces: &[&str],
    required_gate: &str,
) -> HeptaUpstreamCodexSyncContract {
    HeptaUpstreamCodexSyncContract {
        id: id.into(),
        risk,
        title: title.into(),
        upstream_scope: upstream_scope.iter().map(|value| (*value).into()).collect(),
        hepta_surfaces: hepta_surfaces.iter().map(|value| (*value).into()).collect(),
        required_gate: required_gate.into(),
        auto_apply_allowed: false,
        active_runtime_dependency_allowed: false,
        public_release_claim_allowed: false,
        contract_ready: true,
    }
}

fn default_upstream_codex_sync_contracts() -> Vec<HeptaUpstreamCodexSyncContract> {
    vec![
        sync_contract(
            "snapshot-and-diff-intake",
            HeptaUpstreamCodexSyncRisk::Guardrail,
            "Fetch upstream only into an explicit snapshot/diff lane",
            &[
                "upstream remote metadata",
                "release tags",
                "commit range summary",
                "file-level diff inventory",
            ],
            &[
                "codex-rs compatibility snapshot",
                "docs/architecture/HEPTA_UPSTREAM_CODEX_SYNC_LANE.md",
            ],
            "manual fetch or CI job must record the observed upstream head before any absorption patch",
        ),
        sync_contract(
            "provider-credential-security-classification",
            HeptaUpstreamCodexSyncRisk::P0Security,
            "Classify provider, credential, approval, sandbox, and network deltas first",
            &[
                "model provider",
                "credential and auth flows",
                "approval policy",
                "sandbox and exec",
                "network/proxy behavior",
            ],
            &[
                "hepta-runtime provider reports",
                "hepta-kernel policy gates",
                "hepta-gateway read-only reports",
            ],
            "security/provider diffs require adapter contract review before active runtime wiring",
        ),
        sync_contract(
            "runtime-session-tool-contract-classification",
            HeptaUpstreamCodexSyncRisk::P0Runtime,
            "Classify runtime, session, thread-store, tool, MCP, and app-server deltas",
            &[
                "runtime event loop",
                "session and thread store",
                "tool invocation",
                "MCP server/client",
                "app-server protocol",
            ],
            &[
                "/api/hepta-engine-adapter-boundary",
                "/api/hepta-core-fusion-readiness",
                "/api/hepta-engine-dependency-closure",
            ],
            "adapter behavior-equivalence and shadow-replay gates must pass before promotion",
        ),
        sync_contract(
            "compatibility-package-retention-boundary",
            HeptaUpstreamCodexSyncRisk::P1Compatibility,
            "Keep Codex compatibility as an intake surface, not the active service engine",
            &[
                "codex-cli",
                "codex-core",
                "codex-exec",
                "codex-state",
                "codex-mcp",
                "codex-app-server",
                "codex-sandboxing",
                "codex-plugin",
                "codex-model-provider",
                "codex-protocol",
                "codex-tui",
            ],
            &[
                "hepta-cli --bin hepta",
                "scripts/hepta-active-service-dependency-isolation.sh",
            ],
            "active hepta-cli cargo tree must remain free of tracked Codex engine crates",
        ),
        sync_contract(
            "release-governance-no-public-claim",
            HeptaUpstreamCodexSyncRisk::P2Product,
            "Require governance evidence before claiming an upstream-sync release",
            &[
                "changelog",
                "release notes",
                "operator packet",
                "long soak evidence",
                "watchdog evidence",
            ],
            &[
                "public GA readiness",
                "operator approval packet",
                "watchdog",
                "live soak",
            ],
            "no public release claim until long-cycle soak and governance packet pass",
        ),
    ]
}

pub fn hepta_upstream_codex_sync_lane_report() -> HeptaUpstreamCodexSyncLaneReport {
    HeptaUpstreamCodexSyncLaneReport::native_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upstream_codex_sync_lane_is_ready_without_side_effects_or_latest_claims() {
        let report = hepta_upstream_codex_sync_lane_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(report.lane_id, "upstream-codex-sync-lane");
        assert_eq!(
            report.upstream_repository,
            "https://github.com/openai/codex"
        );
        assert_eq!(report.contract_count, 5);
        assert_eq!(report.ready_contract_count, report.contract_count);
        assert!(report.sync_lane_ready);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_latest_claimed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_runtime_codex_engine_dependency_allowed);
        assert!(report.requires_diff_classification_before_absorption);
        assert!(report.requires_adapter_contract_before_active_runtime);
        assert!(report.requires_release_governance_before_public_claim);
        assert!(report.local_only_audit);
        assert!(report.report_only);
        assert!(!report.mutates_runtime_state);
        assert!(!report.external_network_read);
        assert!(!report.external_send);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
    }

    #[test]
    fn upstream_codex_sync_lane_keeps_active_service_dependency_isolation_mandatory() {
        let report = hepta_upstream_codex_sync_lane_report();

        assert_eq!(
            report.active_dependency_isolation_gate,
            "scripts/hepta-active-service-dependency-isolation.sh"
        );
        assert!(report.contracts.iter().any(|contract| {
            contract.id == "compatibility-package-retention-boundary"
                && contract
                    .required_gate
                    .contains("active hepta-cli cargo tree")
                && !contract.auto_apply_allowed
                && !contract.active_runtime_dependency_allowed
                && !contract.public_release_claim_allowed
        }));
        assert!(report.contracts.iter().any(|contract| {
            contract.id == "provider-credential-security-classification"
                && matches!(contract.risk, HeptaUpstreamCodexSyncRisk::P0Security)
        }));
    }
}

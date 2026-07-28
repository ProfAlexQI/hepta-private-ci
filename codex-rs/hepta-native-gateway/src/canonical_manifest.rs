use anyhow::Result;
use hepta_core::current_reality_capability_registry_count;
use hepta_core::hepta_native_absorption_report;
use hepta_core::production_surface_report;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::gate_runner;
use crate::gate_spec::ReceiptStateMachine;
use crate::route_manifest::ROUTE_EFFECT_GATE_MANIFEST_SCHEMA;
use crate::route_manifest::route_effect_gate_manifest;
use crate::route_manifest::validate_route_manifest;
use crate::route_registry::CONTROL_UI_ROUTE_SPECS;

const MANIFEST_SCHEMA: &str = "hepta_canonical_integration_manifest_v1";
const READINESS_DAG_SCHEMA: &str = "hepta_release_readiness_dag_v1";
const CAPABILITY_REGISTRY_SCHEMA: &str = "hepta_native_capability_registry_v1";
const CURRENT_REALITY_CAPABILITY_REGISTRY_SCHEMA: &str =
    "hepta_current_reality_capability_registry_v1";
const IMMUTABLE_RELEASE_MANIFEST_SCHEMA: &str = "hepta_immutable_release_manifest_v1";
const ROUTE_REGISTRY_NORMALIZED_SHA256: &str =
    "aabbfa3b6a873716afb5ad49bfd1e1d4fa7717ce495dde7bc94d8674aba320a1";

#[derive(Debug, Serialize)]
struct RegistryBinding {
    schema_version: &'static str,
    source: &'static str,
    derived_entry_count: usize,
}

#[derive(Debug, Serialize)]
struct ReadinessNode {
    id: &'static str,
    requires: &'static [&'static str],
    evidence: &'static str,
}

pub fn canonical_manifest_json() -> Result<String> {
    let capabilities = hepta_native_absorption_report();
    let production = production_surface_report();
    let (gate_pair_schema, migrated_gate_pair_count) =
        gate_runner::migrated_pair_registry_summary()?;
    validate_route_manifest()?;
    let route_manifest = route_effect_gate_manifest()?;

    let manifest = serde_json::json!({
        "product": "Hepta",
        "schema_version": MANIFEST_SCHEMA,
        "canonical_channel": "main-backend-p1-integration",
        "source_git_head": production.source_git_head,
        "source_version": production.source_version,
        "source_commit_bound": production.source_git_head != "unknown",
        "immutable_release_tree_required": true,
        "dirty_worktree_release_forbidden": true,
        "registries": {
            "current_reality_capabilities": RegistryBinding {
                schema_version: CURRENT_REALITY_CAPABILITY_REGISTRY_SCHEMA,
                source: "hepta_core::current_reality_capability_registry_count",
                derived_entry_count: current_reality_capability_registry_count(),
            },
            "native_absorption_capabilities": RegistryBinding {
                schema_version: CAPABILITY_REGISTRY_SCHEMA,
                source: "hepta_core::hepta_native_capability_registry",
                derived_entry_count: capabilities.capability_count,
            },
            "routes": RegistryBinding {
                schema_version: ROUTE_EFFECT_GATE_MANIFEST_SCHEMA,
                source: "hepta_native_gateway::route_manifest",
                derived_entry_count: route_manifest.entry_count,
            },
            "migrated_gate_pairs": RegistryBinding {
                schema_version: gate_pair_schema,
                source: "scripts/hepta-gate-pair-specs-v1.json",
                derived_entry_count: migrated_gate_pair_count,
            },
            "receipt_states": RegistryBinding {
                schema_version: "hepta_receipt_state_machine_v1",
                source: "hepta_native_gateway::gate_spec::ReceiptStateMachine",
                derived_entry_count: ReceiptStateMachine::ORDERED_STATES.len(),
            },
        },
        "route_effect_gate_manifest": route_manifest,
        "readiness_dag": {
            "schema_version": READINESS_DAG_SCHEMA,
            "nodes": readiness_nodes(),
        },
        "release_policy": {
            "immutable_release_manifest_schema": IMMUTABLE_RELEASE_MANIFEST_SCHEMA,
            "immutable_release_tool": "scripts/hepta-immutable-release-tree",
            "immutable_release_self_test": "scripts/hepta-immutable-release-tree self-test",
            "backend_preflight_required": true,
            "native_profile_required": true,
            "release_profile_required": true,
            "artifact_sha256_required": true,
            "source_artifact_online_commit_equality_required": true,
            "operator_acceptance_required": true,
            "install_or_restart_authorized": false,
            "production_mutation_authorized": false,
        },
        "contract_freeze": {
            "route_registry_normalized_sha256": route_registry_normalized_sha256(),
            "route_registry_expected_sha256": ROUTE_REGISTRY_NORMALIZED_SHA256,
            "route_registry_hash_matches": route_registry_normalized_sha256()
                == ROUTE_REGISTRY_NORMALIZED_SHA256,
            "suffix_ladder_frozen": true,
            "suffix_ladder_gate": "scripts/check-hepta-suffix-ladder-freeze.sh",
        },
        "side_effects": {
            "provider_invoked": false,
            "credentials_read": false,
            "memory_or_kg_written": false,
            "channel_message_sent": false,
            "service_installed_or_restarted": false,
        },
    });

    Ok(serde_json::to_string(&manifest)?)
}

fn route_registry_normalized_sha256() -> String {
    serde_json::to_vec(CONTROL_UI_ROUTE_SPECS).map_or_else(
        |_| "serialization_failed".to_string(),
        |canonical| format!("{:x}", Sha256::digest(canonical)),
    )
}

fn readiness_nodes() -> [ReadinessNode; 6] {
    [
        ReadinessNode {
            id: "registry_snapshot",
            requires: &[],
            evidence: "hepta manifest",
        },
        ReadinessNode {
            id: "backend_preflight",
            requires: &["registry_snapshot"],
            evidence: "scripts/hepta-preflight.sh",
        },
        ReadinessNode {
            id: "native_profile",
            requires: &["backend_preflight"],
            evidence: "Hepta Native app validation",
        },
        ReadinessNode {
            id: "release_profile",
            requires: &["backend_preflight", "native_profile"],
            evidence: "release build and package validation",
        },
        ReadinessNode {
            id: "immutable_release_tree",
            requires: &["release_profile"],
            evidence: "scripts/hepta-immutable-release-tree verify --manifest PATH",
        },
        ReadinessNode {
            id: "operator_cutover",
            requires: &["immutable_release_tree"],
            evidence: "explicit operator acceptance and production blocker closure",
        },
    ]
}

#[cfg(test)]
#[path = "../tests/unit/canonical_manifest.rs"]
mod tests;

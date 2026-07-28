use super::*;
use crate::route_manifest::route_manifest_registry;

#[test]
fn canonical_manifest_uses_registry_derived_counts_and_fail_closed_release_policy() {
    let value: serde_json::Value =
        serde_json::from_str(&canonical_manifest_json().expect("canonical manifest JSON"))
            .expect("canonical manifest value");

    assert_eq!(value["schema_version"], MANIFEST_SCHEMA);
    assert_eq!(
        value["registries"]["native_absorption_capabilities"]["derived_entry_count"],
        hepta_native_absorption_report().capability_count
    );
    assert_eq!(
        value["registries"]["current_reality_capabilities"]["derived_entry_count"],
        current_reality_capability_registry_count()
    );
    assert_eq!(
        value["registries"]["routes"]["derived_entry_count"],
        route_manifest_registry().len()
    );
    assert_eq!(
        value["registries"]["routes"]["schema_version"],
        ROUTE_EFFECT_GATE_MANIFEST_SCHEMA
    );
    assert_eq!(
        value["route_effect_gate_manifest"]["entry_count"],
        route_manifest_registry().len()
    );
    assert_eq!(
        value["readiness_dag"]["nodes"].as_array().map(Vec::len),
        Some(6)
    );
    assert_eq!(
        value["release_policy"]["install_or_restart_authorized"],
        false
    );
    assert_eq!(
        value["release_policy"]["immutable_release_manifest_schema"],
        IMMUTABLE_RELEASE_MANIFEST_SCHEMA
    );
    assert_eq!(
        value["release_policy"]["production_mutation_authorized"],
        false
    );
    assert_eq!(
        value["contract_freeze"]["route_registry_normalized_sha256"],
        ROUTE_REGISTRY_NORMALIZED_SHA256
    );
    assert_eq!(
        value["contract_freeze"]["route_registry_hash_matches"],
        true
    );
}

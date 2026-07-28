use serde::Deserialize;

const CURRENT_REALITY_CAPABILITY_REGISTRY: &str = include_str!(
    "../../../scripts/lib/hepta-gate-pair-compat-v2/current-reality-capabilities.json"
);

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CurrentRealityCapabilityRegistry {
    schema: String,
    source_payload: String,
    source_sha256: String,
    capability_ids: Vec<String>,
}

/// Count the capability rows in the current-reality matrix's generated registry.
///
/// The payload bundle generator owns this projection and binds it to the
/// authenticated legacy source payload.
pub fn current_reality_capability_registry_count() -> usize {
    let registry: CurrentRealityCapabilityRegistry =
        serde_json::from_str(CURRENT_REALITY_CAPABILITY_REGISTRY)
            .expect("current reality capability registry must be valid");
    assert_eq!(
        registry.schema,
        "hepta_current_reality_capability_registry_v1"
    );
    assert!(
        registry
            .source_payload
            .starts_with("scripts/lib/hepta-gate-pair-compat-v1/")
    );
    assert_eq!(registry.source_sha256.len(), 64);
    registry.capability_ids.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_count_is_derived_from_the_matrix_registry() {
        let first = current_reality_capability_registry_count();
        let second = current_reality_capability_registry_count();

        assert!(first > 0);
        assert_eq!(first, second);
    }
}

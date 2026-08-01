use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::OnceLock;

use crate::route_definition::RouteDefinition;

static DIRECT_CALL_COUNTS: OnceLock<Mutex<BTreeMap<&'static str, u64>>> = OnceLock::new();

pub(super) fn record_direct_call(definition: RouteDefinition) {
    if !definition.legacy_compatibility_route {
        return;
    }
    let mut counts = direct_call_counts()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let count = counts.entry(definition.lifecycle.path_pattern).or_default();
    *count = count.saturating_add(1);
}

pub(super) fn direct_call_count(path: &str) -> u64 {
    direct_call_counts()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(path)
        .copied()
        .unwrap_or_default()
}

pub(super) fn total_direct_call_count() -> u64 {
    direct_call_counts()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .values()
        .copied()
        .fold(0_u64, u64::saturating_add)
}

fn direct_call_counts() -> &'static Mutex<BTreeMap<&'static str, u64>> {
    DIRECT_CALL_COUNTS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_only_legacy_direct_calls() {
        let legacy = crate::route_definition::route_definition_registry()
            .into_iter()
            .find(|definition| definition.legacy_compatibility_route)
            .expect("legacy route");
        let canonical = crate::route_definition::route_definition(
            "GET",
            crate::route_registry::EVIDENCE_INDEX_ENDPOINT,
        )
        .expect("canonical evidence route");
        let before = direct_call_count(legacy.lifecycle.path_pattern);

        record_direct_call(legacy);
        record_direct_call(canonical);

        assert_eq!(
            direct_call_count(legacy.lifecycle.path_pattern),
            before.saturating_add(1)
        );
        assert_eq!(direct_call_count(canonical.lifecycle.path_pattern), 0);
    }
}

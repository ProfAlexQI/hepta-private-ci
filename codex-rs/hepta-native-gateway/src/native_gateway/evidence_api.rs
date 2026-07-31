use serde::Serialize;

use crate::route_manifest::RouteDefinition;
use crate::route_manifest::route_definition_registry;
#[cfg(test)]
use crate::route_registry::CONTROL_UI_ROUTE_SPECS;
use crate::route_registry::EVIDENCE_INDEX_ENDPOINT;
#[cfg(test)]
use crate::route_registry::TELEGRAM_LIVE_SOAK_ROUTE;

#[derive(Debug, Serialize)]
pub(super) struct EvidenceIndex {
    schema: &'static str,
    status: &'static str,
    canonical_endpoint: &'static str,
    pagination: EvidencePagination,
    entry_count: usize,
    legacy_compatibility_route_count: usize,
    entries: Vec<EvidenceEntry>,
}

#[derive(Debug, Serialize)]
struct EvidencePagination {
    mode: &'static str,
    snapshot_bound: bool,
    transport_encoding: &'static str,
}

#[derive(Debug, Serialize)]
struct EvidenceEntry {
    route: &'static str,
    source_command: &'static str,
    capability: &'static str,
    evidence_state: &'static str,
    effect_class: &'static str,
    side_effect_boundary: &'static str,
    legacy_compatibility_route: bool,
}

pub(super) fn evidence_index_report() -> EvidenceIndex {
    let entries = route_definition_registry()
        .into_iter()
        .filter_map(evidence_entry)
        .collect::<Vec<_>>();
    EvidenceIndex {
        schema: "hepta_evidence_index_v1",
        status: "ready",
        canonical_endpoint: EVIDENCE_INDEX_ENDPOINT,
        pagination: EvidencePagination {
            mode: "digest_bound_cursor",
            snapshot_bound: true,
            transport_encoding: "base64_json_bytes",
        },
        entry_count: entries.len(),
        legacy_compatibility_route_count: entries
            .iter()
            .filter(|entry| entry.legacy_compatibility_route)
            .count(),
        entries,
    }
}

fn evidence_entry(definition: RouteDefinition) -> Option<EvidenceEntry> {
    let evidence_state = definition.receipt_state?.as_str();
    Some(EvidenceEntry {
        route: definition.lifecycle.path_pattern,
        source_command: definition.source_command?,
        capability: definition.capability?,
        evidence_state,
        effect_class: definition.evidence_effect_class?,
        side_effect_boundary: definition.side_effect_boundary?,
        legacy_compatibility_route: definition.legacy_compatibility_route,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_index_is_canonical_paginated_and_side_effect_classified() {
        let report = evidence_index_report();
        assert_eq!(report.canonical_endpoint, EVIDENCE_INDEX_ENDPOINT);
        assert_eq!(report.pagination.mode, "digest_bound_cursor");
        assert!(report.pagination.snapshot_bound);
        assert_eq!(report.entry_count, report.entries.len());
        assert!(report.entry_count > 100);
        assert!(
            report
                .entries
                .iter()
                .all(|entry| !entry.evidence_state.is_empty()
                    && !entry.effect_class.is_empty()
                    && !entry.side_effect_boundary.is_empty())
        );
        assert_eq!(
            report.legacy_compatibility_route_count,
            report.entries.len() - 1
        );
    }

    #[test]
    fn route_definitions_bind_evidence_permissions_and_aliases_once() {
        let definitions = route_definition_registry();
        assert!(definitions.len() > CONTROL_UI_ROUTE_SPECS.len());
        for spec in CONTROL_UI_ROUTE_SPECS {
            let definition = crate::route_manifest::route_definition(spec.method, spec.pattern)
                .unwrap_or_else(|| {
                    panic!(
                        "missing route definition for {} {}",
                        spec.method, spec.pattern
                    )
                });
            assert_eq!(definition.source_command, Some(spec.source_command));
            assert_eq!(definition.capability, Some(spec.capability));
            assert_eq!(
                definition.side_effect_boundary,
                Some(spec.side_effect_boundary)
            );
            assert_eq!(definition.receipt_state, spec.receipt_state());
        }
        assert_eq!(
            definitions
                .iter()
                .filter(|definition| definition.receipt_state.is_some())
                .count(),
            evidence_index_report().entry_count
        );
        assert!(definitions.iter().all(|definition| {
            definition.lifecycle.path_pattern != TELEGRAM_LIVE_SOAK_ROUTE.canonical
                || definition.aliases == TELEGRAM_LIVE_SOAK_ROUTE.aliases
        }));
    }
}

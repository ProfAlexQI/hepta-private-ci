use serde::Serialize;

use crate::gate_spec::GateSpec;
use crate::route_registry::CONTROL_UI_ROUTE_SPECS;
use crate::route_registry::EVIDENCE_INDEX_ENDPOINT;

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
    let entries = CONTROL_UI_ROUTE_SPECS
        .iter()
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

fn evidence_entry(spec: &GateSpec) -> Option<EvidenceEntry> {
    let evidence_state = spec.receipt_state()?.as_str();
    Some(EvidenceEntry {
        route: spec.pattern,
        source_command: spec.source_command,
        capability: spec.capability,
        evidence_state,
        effect_class: effect_class(spec),
        side_effect_boundary: spec.side_effect_boundary,
        legacy_compatibility_route: spec.pattern != EVIDENCE_INDEX_ENDPOINT,
    })
}

fn effect_class(spec: &GateSpec) -> &'static str {
    if spec.is_read_only() {
        "read_only"
    } else if spec.is_dry_run() {
        "dry_run"
    } else if spec.requires_confirmation() {
        "confirmation_required"
    } else {
        "declared_no_effect"
    }
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
}

use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;

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
    evidence_id: String,
    route: &'static str,
    source_command: &'static str,
    capability: &'static str,
    evidence_state: &'static str,
    effect_class: &'static str,
    side_effect_boundary: &'static str,
    legacy_compatibility_route: bool,
}

#[derive(Debug, Serialize)]
pub(super) struct EvidenceDocument {
    schema: &'static str,
    status: &'static str,
    canonical_endpoint: &'static str,
    selected_evidence_id: String,
    selected_route: &'static str,
    source_http_status: &'static str,
    source_content_sha256: String,
    evidence: EvidenceEntry,
    payload: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EvidenceSelector<'a> {
    Id(&'a str),
    Route(&'a str),
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

pub(super) fn requested_evidence_selector(
    query: Option<&str>,
) -> Result<Option<EvidenceSelector<'_>>, &'static str> {
    let Some(query) = query.filter(|query| !query.is_empty()) else {
        return Ok(None);
    };
    let mut selector = None;
    for pair in query.split('&') {
        let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
        let requested = match name {
            "id" => {
                let digest = value
                    .strip_prefix("ev_")
                    .ok_or("evidence id must use the canonical ev_<sha256> representation")?;
                if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                    return Err("evidence id must use the canonical ev_<sha256> representation");
                }
                Some(EvidenceSelector::Id(value))
            }
            "route" => {
                if !value.starts_with("/api/") || value.contains(['?', '#', '%']) {
                    return Err("evidence route must be an unescaped canonical /api/ path");
                }
                Some(EvidenceSelector::Route(value))
            }
            _ => None,
        };
        if let Some(requested) = requested {
            if selector.replace(requested).is_some() {
                return Err("exactly one evidence id or route may be provided");
            }
        }
    }
    Ok(selector)
}

pub(super) fn evidence_definition(selector: EvidenceSelector<'_>) -> Option<RouteDefinition> {
    route_definition_registry().into_iter().find(|definition| {
        let Some(entry) = evidence_entry(*definition) else {
            return false;
        };
        match selector {
            EvidenceSelector::Id(evidence_id) => entry.evidence_id == evidence_id,
            EvidenceSelector::Route(route) => entry.route == route,
        }
    })
}

pub(super) fn evidence_document_report(
    definition: RouteDefinition,
    source_http_status: &'static str,
    source_content_type: &'static str,
    source_body: String,
) -> Result<EvidenceDocument, &'static str> {
    if !definition.legacy_compatibility_route
        || definition.lifecycle.method != "GET"
        || definition.lifecycle.path_pattern.contains('<')
    {
        return Err("selected route is not a canonical legacy evidence report");
    }
    if !source_content_type.starts_with("application/json") {
        return Err("selected evidence report did not return JSON");
    }
    let payload = serde_json::from_str(&source_body)
        .map_err(|_| "selected evidence report returned invalid JSON")?;
    let evidence = evidence_entry(definition)
        .ok_or("selected route does not expose receipt-state evidence")?;
    Ok(EvidenceDocument {
        schema: "hepta_evidence_document_v1",
        status: "ready",
        canonical_endpoint: EVIDENCE_INDEX_ENDPOINT,
        selected_evidence_id: evidence.evidence_id.clone(),
        selected_route: definition.lifecycle.path_pattern,
        source_http_status,
        source_content_sha256: format!("{:x}", Sha256::digest(source_body.as_bytes())),
        evidence,
        payload,
    })
}

fn evidence_entry(definition: RouteDefinition) -> Option<EvidenceEntry> {
    let evidence_state = definition.receipt_state?.as_str();
    let capability = definition.capability?;
    let evidence_id = format!(
        "ev_{:x}",
        Sha256::digest(
            format!(
                "{}\0{}\0{}\0{}",
                definition.lifecycle.method,
                definition.lifecycle.path_pattern,
                capability,
                evidence_state
            )
            .as_bytes()
        )
    );
    Some(EvidenceEntry {
        evidence_id,
        route: definition.lifecycle.path_pattern,
        source_command: definition.source_command?,
        capability,
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

    #[test]
    fn evidence_route_query_is_canonical_and_unambiguous() {
        let evidence_id = evidence_index_report().entries[0].evidence_id.clone();
        assert_eq!(requested_evidence_selector(None), Ok(None));
        assert_eq!(
            requested_evidence_selector(Some("route=/api/example&detail=full")),
            Ok(Some(EvidenceSelector::Route("/api/example")))
        );
        assert_eq!(
            requested_evidence_selector(Some(&format!("id={evidence_id}"))),
            Ok(Some(EvidenceSelector::Id(&evidence_id)))
        );
        assert!(requested_evidence_selector(Some("route=/api/a&route=/api/b")).is_err());
        assert!(requested_evidence_selector(Some("route=/api/a&id=ev_0000000000000000000000000000000000000000000000000000000000000000")).is_err());
        assert!(requested_evidence_selector(Some("route=%2Fapi%2Fexample")).is_err());
        assert!(requested_evidence_selector(Some("route=/health")).is_err());
        assert!(requested_evidence_selector(Some("id=ev_not-a-digest")).is_err());
    }

    #[test]
    fn evidence_ids_are_unique_stable_and_resolvable() {
        let report = evidence_index_report();
        let unique = report
            .entries
            .iter()
            .map(|entry| entry.evidence_id.as_str())
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(unique.len(), report.entries.len());
        for entry in &report.entries {
            let definition = evidence_definition(EvidenceSelector::Id(&entry.evidence_id))
                .expect("evidence id must resolve");
            assert_eq!(definition.lifecycle.path_pattern, entry.route);
        }
    }
}

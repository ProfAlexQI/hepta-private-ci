use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;

#[cfg(test)]
use crate::route_registry::CONTROL_UI_ROUTE_SPECS;
use crate::route_registry::EVIDENCE_INDEX_ENDPOINT;

use super::generated_evidence_registry::EVIDENCE_DEFINITIONS;
use super::generated_evidence_registry::EvidenceDefinition;
use super::generated_evidence_registry::evidence_definition_by_id;
use super::generated_evidence_registry::evidence_definition_by_route;

#[derive(Debug, Serialize)]
pub(super) struct EvidenceIndex {
    schema: &'static str,
    status: &'static str,
    canonical_endpoint: &'static str,
    pagination: EvidencePagination,
    entry_count: usize,
    legacy_compatibility_route_count: usize,
    retired_direct_route_count: usize,
    legacy_direct_call_count_since_start: u64,
    legacy_route_telemetry: super::legacy_route_usage::LegacyRouteTelemetryHealth,
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
    canonical_selector: String,
    direct_call_count_since_start: u64,
    migration_state: &'static str,
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
    let entries = EVIDENCE_DEFINITIONS
        .iter()
        .copied()
        .map(evidence_entry)
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
        retired_direct_route_count: EVIDENCE_DEFINITIONS
            .iter()
            .filter(|definition| definition.retired_direct_route)
            .count(),
        legacy_direct_call_count_since_start: super::legacy_route_usage::total_direct_call_count(),
        legacy_route_telemetry: super::legacy_route_usage::telemetry_health(),
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
        if let Some(requested) = requested
            && selector.replace(requested).is_some()
        {
            return Err("exactly one evidence id or route may be provided");
        }
    }
    Ok(selector)
}

pub(super) fn evidence_definition(selector: EvidenceSelector<'_>) -> Option<EvidenceDefinition> {
    match selector {
        EvidenceSelector::Id(evidence_id) => evidence_definition_by_id(evidence_id),
        EvidenceSelector::Route(route) => evidence_definition_by_route(route),
    }
}

pub(super) fn evidence_document_report(
    definition: EvidenceDefinition,
    source_http_status: &'static str,
    source_content_type: &'static str,
    source_body: String,
) -> Result<EvidenceDocument, &'static str> {
    if !definition.legacy_compatibility_route
        || definition.method != "GET"
        || definition.route_selector.contains('<')
        || definition.renderer_key.is_none()
    {
        return Err("selected route is not a canonical legacy evidence report");
    }
    if !source_content_type.starts_with("application/json") {
        return Err("selected evidence report did not return JSON");
    }
    let payload = serde_json::from_str(&source_body)
        .map_err(|_| "selected evidence report returned invalid JSON")?;
    let evidence = evidence_entry(definition);
    Ok(EvidenceDocument {
        schema: "hepta_evidence_document_v1",
        status: "ready",
        canonical_endpoint: EVIDENCE_INDEX_ENDPOINT,
        selected_evidence_id: evidence.evidence_id.clone(),
        selected_route: definition.route_selector,
        source_http_status,
        source_content_sha256: format!("{:x}", Sha256::digest(source_body.as_bytes())),
        evidence,
        payload,
    })
}

fn evidence_entry(definition: EvidenceDefinition) -> EvidenceEntry {
    EvidenceEntry {
        evidence_id: definition.evidence_id.to_string(),
        route: definition.route_selector,
        source_command: definition.source_command,
        capability: definition.capability,
        evidence_state: definition.evidence_state,
        effect_class: definition.effect_class,
        side_effect_boundary: definition.side_effect_boundary,
        legacy_compatibility_route: definition.legacy_compatibility_route,
        canonical_selector: format!(
            "{EVIDENCE_INDEX_ENDPOINT}?route={}",
            definition.route_selector
        ),
        direct_call_count_since_start: super::legacy_route_usage::direct_call_count(
            definition.route_selector,
        ),
        migration_state: if definition.legacy_compatibility_route {
            "observing_direct_calls"
        } else {
            "canonical"
        },
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
        assert_eq!(
            report.legacy_direct_call_count_since_start,
            report
                .entries
                .iter()
                .map(|entry| entry.direct_call_count_since_start)
                .sum::<u64>()
        );
        assert!(report.entry_count > 100);
        let telemetry = serde_json::to_value(&report.legacy_route_telemetry)
            .expect("legacy route telemetry health JSON");
        assert_eq!(
            telemetry["schema"],
            "hepta_legacy_route_telemetry_health_v2"
        );
        assert_eq!(
            telemetry["enable_env"],
            "HEPTA_CONTROL_UI_LEGACY_ROUTE_TELEMETRY"
        );
        assert_eq!(telemetry["observation_window_complete"], false);
        assert_eq!(telemetry["file_contents_fully_validated"], false);
        assert_eq!(telemetry["summary_producer_available"], true);
        assert_eq!(telemetry["zero_usage_claim_allowed"], false);
        assert_eq!(telemetry["retirement_evidence_ready"], false);
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
        assert_eq!(report.retired_direct_route_count, 166);
        assert!(report.entries.iter().all(|entry| {
            entry.canonical_selector == format!("{EVIDENCE_INDEX_ENDPOINT}?route={}", entry.route)
        }));
    }

    #[test]
    fn generated_evidence_registry_matches_typed_catalog_receipt_states() {
        let mut expected_evidence_count = 0;
        for spec in CONTROL_UI_ROUTE_SPECS {
            let definition = evidence_definition_by_route(spec.pattern);
            match spec.receipt_state() {
                Some(evidence_state) => {
                    expected_evidence_count += 1;
                    let definition = definition.unwrap_or_else(|| {
                        panic!("missing generated evidence definition for {}", spec.pattern)
                    });
                    assert_eq!(definition.method, spec.method);
                    assert_eq!(definition.source_command, spec.source_command);
                    assert_eq!(definition.capability, spec.capability);
                    assert_eq!(definition.side_effect_boundary, spec.side_effect_boundary);
                    assert_eq!(definition.evidence_state, evidence_state.as_str());
                }
                None => assert!(definition.is_none(), "unexpected evidence definition"),
            }
        }
        assert_eq!(expected_evidence_count, EVIDENCE_DEFINITIONS.len());
        assert_eq!(expected_evidence_count, evidence_index_report().entry_count);
        assert_eq!(EVIDENCE_DEFINITIONS.len(), 207);
        assert_eq!(
            EVIDENCE_DEFINITIONS
                .iter()
                .filter(|definition| definition.renderer_key.is_some())
                .count(),
            206
        );
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
            assert_eq!(definition.route_selector, entry.route);
            let expected_id = format!(
                "ev_{:x}",
                Sha256::digest(
                    format!(
                        "{}\0{}\0{}\0{}",
                        definition.method,
                        definition.route_selector,
                        definition.capability,
                        definition.evidence_state
                    )
                    .as_bytes()
                )
            );
            assert_eq!(definition.evidence_id, expected_id);
        }
    }

    #[test]
    fn evidence_selector_and_renderer_survive_simulated_http_path_removal() {
        let definition = EVIDENCE_DEFINITIONS
            .iter()
            .copied()
            .find(|definition| {
                definition.legacy_compatibility_route
                    && definition.renderer_key == Some("native_report_024")
            })
            .expect("stable renderable evidence definition");
        let simulated_http_registry = crate::route_registry::registered_native_report_paths()
            .filter(|path| *path != definition.route_selector)
            .collect::<Vec<_>>();
        assert!(!simulated_http_registry.contains(&definition.route_selector));

        let resolved = evidence_definition(EvidenceSelector::Id(definition.evidence_id))
            .expect("evidence id must resolve without an HTTP path lookup");
        assert_eq!(resolved, definition);
        assert_eq!(
            evidence_definition(EvidenceSelector::Route(definition.route_selector)),
            Some(definition)
        );

        let options = crate::NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: false,
            telegram_plugin_poll_ms: 1_500,
        };
        let plugin = crate::native_telegram::telegram_plugin_status(false, 1_500);
        let (status, content_type, body) =
            super::super::native_report_registry::render_registered_evidence_report(
                definition.renderer_key.expect("generated renderer key"),
                &options,
                plugin,
            )
            .expect("renderer key must resolve independently of the HTTP registry");
        assert_eq!(status, "200 OK");
        assert!(content_type.starts_with("application/json"));
        assert!(serde_json::from_str::<Value>(&body).is_ok());
    }
}

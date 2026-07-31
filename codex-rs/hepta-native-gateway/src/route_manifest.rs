use std::collections::HashSet;

use anyhow::Result;
use sha2::Digest;
use sha2::Sha256;

pub(crate) use crate::route_definition::RouteDefinition;
pub(crate) use crate::route_definition::RouteDispatchHandler;
pub(crate) use crate::route_definition::RouteReportBinding;
pub(crate) use crate::route_definition::RouteResponsePolicy;
pub(crate) use crate::route_definition::WATCHDOG_PROBE_PATHS;
pub(crate) use crate::route_definition::route_definition;
pub(crate) use crate::route_definition::route_definition_registry;
#[cfg(test)]
pub(crate) use crate::route_report_descriptor::ReportRenderer;
use crate::runtime_ingress::IngressDefaultEnablement;
use crate::runtime_ingress::route_pattern_matches;

pub(crate) const ROUTE_EFFECT_GATE_MANIFEST_SCHEMA: &str = "hepta_route_effect_gate_manifest_v1";

#[derive(Debug, serde::Serialize)]
pub(crate) struct RouteEffectGateManifest {
    schema_version: &'static str,
    source: &'static str,
    pub(crate) entry_count: usize,
    watchdog_probe_count: usize,
    sha256: String,
    entries: Vec<RouteDefinition>,
}

pub(crate) fn route_manifest_entry(method: &str, path: &str) -> Option<RouteDefinition> {
    route_definition(method, path)
}

pub(crate) fn route_manifest_registry() -> Vec<RouteDefinition> {
    route_definition_registry()
}

pub(crate) fn route_manifest_digest() -> Result<String> {
    let canonical = serde_json::to_vec(&route_manifest_registry())?;
    Ok(format!("{:x}", Sha256::digest(canonical)))
}

pub(crate) fn route_effect_gate_manifest() -> Result<RouteEffectGateManifest> {
    let entries = route_manifest_registry();
    let watchdog_probe_count = entries.iter().filter(|entry| entry.watchdog_probe).count();
    let sha256 = route_manifest_digest()?;
    Ok(RouteEffectGateManifest {
        schema_version: ROUTE_EFFECT_GATE_MANIFEST_SCHEMA,
        source: "hepta_native_gateway::route_definition_registry",
        entry_count: entries.len(),
        watchdog_probe_count,
        sha256,
        entries,
    })
}

pub(crate) fn validate_route_manifest() -> Result<()> {
    let entries = route_manifest_registry();
    let mut routes = HashSet::with_capacity(entries.len());
    for entry in &entries {
        if !routes.insert((entry.lifecycle.method, entry.lifecycle.path_pattern)) {
            anyhow::bail!(
                "duplicate route manifest entry: {} {}",
                entry.lifecycle.method,
                entry.lifecycle.path_pattern
            );
        }
        if entry.lifecycle.default_enablement
            == IngressDefaultEnablement::DisabledUnlessExplicitGate
            && entry.required_gate.is_none()
            && entry.dispatch_handler != RouteDispatchHandler::RetiredCompatibility
            && !matches!(
                entry.dispatch_handler,
                RouteDispatchHandler::PreferenceIngress
                    | RouteDispatchHandler::EffectReconciliation
            )
        {
            anyhow::bail!(
                "disabled route lacks an explicit gate: {} {}",
                entry.lifecycle.method,
                entry.lifecycle.path_pattern
            );
        }
        if entry.watchdog_probe
            && (entry.lifecycle.method != "GET"
                || entry.lifecycle.path_pattern.contains('<')
                || entry.dispatch_handler == RouteDispatchHandler::RetiredCompatibility)
        {
            anyhow::bail!(
                "invalid watchdog route: {} {}",
                entry.lifecycle.method,
                entry.lifecycle.path_pattern
            );
        }
        if entry.response_policy == RouteResponsePolicy::DigestBoundPagination
            && entry.lifecycle.method != "GET"
        {
            anyhow::bail!(
                "digest-bound pagination is only valid for GET routes: {} {}",
                entry.lifecycle.method,
                entry.lifecycle.path_pattern
            );
        }
        if entry.lifecycle.method == "GET"
            && entry.dispatch_handler == RouteDispatchHandler::NativeGateway
            && entry.report_binding == RouteReportBinding::None
        {
            anyhow::bail!(
                "native GET route lacks a typed report binding: {}",
                entry.lifecycle.path_pattern
            );
        }
    }
    for path in WATCHDOG_PROBE_PATHS {
        let count = entries
            .iter()
            .filter(|entry| {
                entry.lifecycle.method == "GET"
                    && route_pattern_matches(entry.lifecycle.path_pattern, path)
                    && entry.watchdog_probe
            })
            .count();
        if count != 1 {
            anyhow::bail!("watchdog route must resolve exactly once: GET {path}");
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "../tests/unit/route_manifest.rs"]
mod tests;

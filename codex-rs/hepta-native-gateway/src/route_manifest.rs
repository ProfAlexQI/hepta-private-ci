use std::collections::HashSet;

use anyhow::Result;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::native_telegram::TELEGRAM_LIVE_READ_ENV;
use crate::operator_mutation::OPERATOR_MUTATION_ENABLED_ENV;
pub(crate) use crate::route_report_descriptor::ReportRenderer;
use crate::runtime_ingress::IngressDefaultEnablement;
use crate::runtime_ingress::IngressLifecycleSpec;
use crate::runtime_ingress::declared_runtime_ingress_lifecycle;
use crate::runtime_ingress::route_pattern_matches;
use crate::runtime_ingress::runtime_ingress_lifecycle_registry;
use crate::runtime_mutation::RUNTIME_MUTATION_CANARY_ENV;
use crate::telegram_authority::TELEGRAM_AUTHORITY_ENABLED_ENV;

pub(crate) const ROUTE_EFFECT_GATE_MANIFEST_SCHEMA: &str = "hepta_route_effect_gate_manifest_v1";

const WATCHDOG_PROBE_PATHS: &[&str] = &[
    "/health",
    "/api/watchdog-state",
    "/api/control-ui-route-parity",
    "/api/operator-security",
    "/api/telegram-owner-handoff",
    "/api/telegram-poll-loop",
    "/api/native-post-activation-plan",
    "/api/native-post-execution-stores",
    "/api/hepta-engine-adapter-boundary",
    "/api/hepta-codex-engine-adapter-boundary",
    "/api/hepta-core-fusion-readiness",
    "/api/hepta-name-repository-closure",
    "/api/hepta-engine-dependency-closure",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RouteDispatchHandler {
    NativeGateway,
    PreferenceIngress,
    EffectReconciliation,
    TelegramReconciliation,
    RuntimeKernelCanary,
    RuntimeMutationCanary,
    OperatorExecution,
    TelegramReceiveOnce,
    RetiredCompatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RouteResponsePolicy {
    Passthrough,
    DigestBoundPagination,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub(crate) struct RouteManifestEntry {
    #[serde(flatten)]
    pub(crate) lifecycle: IngressLifecycleSpec,
    pub(crate) dispatch_handler: RouteDispatchHandler,
    pub(crate) required_gate: Option<&'static str>,
    pub(crate) watchdog_probe: bool,
    pub(crate) response_policy: RouteResponsePolicy,
}

#[derive(Debug, Serialize)]
pub(crate) struct RouteEffectGateManifest {
    schema_version: &'static str,
    source: &'static str,
    pub(crate) entry_count: usize,
    watchdog_probe_count: usize,
    sha256: String,
    entries: Vec<RouteManifestEntry>,
}

pub(crate) fn route_manifest_entry(method: &str, path: &str) -> Option<RouteManifestEntry> {
    declared_runtime_ingress_lifecycle(method, path).map(route_manifest_entry_from_lifecycle)
}

pub(crate) fn route_manifest_registry() -> Vec<RouteManifestEntry> {
    runtime_ingress_lifecycle_registry()
        .into_iter()
        .map(route_manifest_entry_from_lifecycle)
        .collect()
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
        source: "hepta_native_gateway::route_manifest",
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

fn route_manifest_entry_from_lifecycle(lifecycle: IngressLifecycleSpec) -> RouteManifestEntry {
    let dispatch_handler = match lifecycle.source {
        "trusted_preference_ingress" => RouteDispatchHandler::PreferenceIngress,
        "effect_reconciliation" => RouteDispatchHandler::EffectReconciliation,
        "telegram_terminal_reconciliation" => RouteDispatchHandler::TelegramReconciliation,
        "runtime_kernel_canary" => RouteDispatchHandler::RuntimeKernelCanary,
        "runtime_mutation_canary" => RouteDispatchHandler::RuntimeMutationCanary,
        "operator_authority_challenge"
        | "operator_mutation"
        | "operator_mutation_reconciliation"
        | "telegram_operator_authority" => RouteDispatchHandler::OperatorExecution,
        "telegram_receive_once" => RouteDispatchHandler::TelegramReceiveOnce,
        "control_ui_transitive_effect_quarantine" => RouteDispatchHandler::RetiredCompatibility,
        "control_ui_route_specs" | "special_native_gateway_route" => {
            RouteDispatchHandler::NativeGateway
        }
        _ => RouteDispatchHandler::RetiredCompatibility,
    };
    let required_gate = match lifecycle.source {
        "telegram_receive_once" => Some(TELEGRAM_LIVE_READ_ENV),
        "telegram_operator_authority" | "telegram_terminal_reconciliation" => {
            Some(TELEGRAM_AUTHORITY_ENABLED_ENV)
        }
        "runtime_mutation_canary" => Some(RUNTIME_MUTATION_CANARY_ENV),
        "operator_mutation" | "operator_mutation_reconciliation" => {
            Some(OPERATOR_MUTATION_ENABLED_ENV)
        }
        _ => None,
    };
    RouteManifestEntry {
        lifecycle,
        dispatch_handler,
        required_gate,
        watchdog_probe: lifecycle.method == "GET"
            && WATCHDOG_PROBE_PATHS.contains(&lifecycle.path_pattern),
        response_policy: response_policy(lifecycle),
    }
}

fn response_policy(lifecycle: IngressLifecycleSpec) -> RouteResponsePolicy {
    if lifecycle.method == "GET"
        && (lifecycle.source == "control_ui_route_specs"
            || lifecycle.path_pattern == crate::route_registry::CONTROL_UI_ROUTE_PARITY_ENDPOINT)
    {
        RouteResponsePolicy::DigestBoundPagination
    } else {
        RouteResponsePolicy::Passthrough
    }
}

#[cfg(test)]
#[path = "../tests/unit/route_manifest.rs"]
mod tests;

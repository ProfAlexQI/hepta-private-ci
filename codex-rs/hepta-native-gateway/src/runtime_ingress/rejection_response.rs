use super::RuntimeIngressResponse;
use super::route_pattern_matches;
use super::runtime_ingress_lifecycle;
use crate::route_manifest::RouteDispatchHandler;
use crate::route_manifest::route_manifest_entry;
use crate::route_registry::CONTROL_UI_ROUTE_SPECS;

pub(crate) fn runtime_ingress_rejection_response(
    method: &str,
    path: &str,
) -> RuntimeIngressResponse {
    if CONTROL_UI_ROUTE_SPECS.iter().any(|route| {
        route.method == method
            && route.is_quarantined_transitive_effect()
            && route_pattern_matches(route.pattern, path)
    }) {
        return RuntimeIngressResponse {
            status: "410 Gone",
            body: r#"{"error":"runtime_ingress.route_retired","reason":"legacy_get_route_has_transitive_effects","replacement":"use an explicitly admitted POST mutation route"}"#.to_string(),
        };
    }
    if route_manifest_entry(method, path)
        .is_some_and(|entry| entry.dispatch_handler == RouteDispatchHandler::RetiredCompatibility)
    {
        return RuntimeIngressResponse {
            status: "410 Gone",
            body: r#"{"error":"runtime_ingress.route_retired","reason":"legacy_evidence_route_is_canonical_only","replacement":"use /api/evidence with the route selector"}"#.to_string(),
        };
    }
    if runtime_ingress_lifecycle(method, path).is_none() {
        return RuntimeIngressResponse {
            status: "404 Not Found",
            body: r#"{"error":"runtime_ingress.route_not_found"}"#.to_string(),
        };
    }
    RuntimeIngressResponse {
        status: "503 Service Unavailable",
        body: r#"{"error":"runtime_ingress.preflight_unavailable"}"#.to_string(),
    }
}

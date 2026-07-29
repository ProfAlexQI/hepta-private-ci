use super::*;
use crate::operator_mutation::OPERATOR_MUTATION_COMMIT_ENDPOINT;
use crate::route_registry::CONTROL_UI_ROUTE_SPECS;
use crate::route_registry::WATCHDOG_STATE_ENDPOINT;
use crate::runtime_ingress::TELEGRAM_RECEIVE_ONCE_ENDPOINT;
use crate::runtime_mutation::RUNTIME_MUTATION_CANARY_ENDPOINT;
use crate::telegram_authority::TELEGRAM_AUTHORITY_COMMIT_ENDPOINT;

#[test]
fn manifest_is_unique_complete_and_digestible() {
    validate_route_manifest().expect("valid route manifest");
    let entries = route_manifest_registry();
    assert_eq!(entries.len(), runtime_ingress_lifecycle_registry().len());
    assert_eq!(route_manifest_digest().expect("manifest digest").len(), 64);
    assert_eq!(
        entries.iter().filter(|entry| entry.watchdog_probe).count(),
        WATCHDOG_PROBE_PATHS.len()
    );
}

#[test]
fn manifest_generates_dispatch_and_gate_bindings() {
    for (method, path, handler, gate) in [
        ("GET", "/health", RouteDispatchHandler::NativeGateway, None),
        (
            "POST",
            TELEGRAM_RECEIVE_ONCE_ENDPOINT,
            RouteDispatchHandler::TelegramReceiveOnce,
            Some(TELEGRAM_LIVE_READ_ENV),
        ),
        (
            "POST",
            RUNTIME_MUTATION_CANARY_ENDPOINT,
            RouteDispatchHandler::RuntimeMutationCanary,
            Some(RUNTIME_MUTATION_CANARY_ENV),
        ),
        (
            "POST",
            OPERATOR_MUTATION_COMMIT_ENDPOINT,
            RouteDispatchHandler::OperatorExecution,
            Some(OPERATOR_MUTATION_ENABLED_ENV),
        ),
        (
            "POST",
            TELEGRAM_AUTHORITY_COMMIT_ENDPOINT,
            RouteDispatchHandler::OperatorExecution,
            Some(TELEGRAM_AUTHORITY_ENABLED_ENV),
        ),
    ] {
        let entry = route_manifest_entry(method, path)
            .unwrap_or_else(|| panic!("missing manifest entry for {method} {path}"));
        assert_eq!(entry.dispatch_handler, handler);
        assert_eq!(entry.required_gate, gate);
    }
}

#[test]
fn manifest_assigns_pagination_to_reports_and_exempts_stable_projections() {
    let operator = route_manifest_entry("GET", "/api/operator-security")
        .expect("operator security manifest entry");
    assert_eq!(
        operator.response_policy,
        RouteResponsePolicy::DigestBoundPagination
    );
    let parity = route_manifest_entry("GET", "/api/control-ui-route-parity")
        .expect("route parity manifest entry");
    assert_eq!(
        parity.response_policy,
        RouteResponsePolicy::DigestBoundPagination
    );
    for path in ["/health", WATCHDOG_STATE_ENDPOINT] {
        let entry =
            route_manifest_entry("GET", path).unwrap_or_else(|| panic!("missing route {path}"));
        assert_eq!(entry.response_policy, RouteResponsePolicy::Passthrough);
    }
    let owner = route_manifest_entry("GET", "/api/telegram-owner-handoff")
        .expect("owner handoff manifest entry");
    assert_eq!(
        owner.response_policy,
        RouteResponsePolicy::DigestBoundPagination
    );
}

#[test]
fn manifest_derives_typed_report_descriptors_only_for_native_gets() {
    let report = route_manifest_entry("GET", "/api/operator-security")
        .expect("operator security manifest entry")
        .report_descriptor()
        .expect("native GET report descriptor");
    assert_eq!(report.renderer, ReportRenderer::NativeGatewayJson);
    assert_eq!(
        report.response_policy,
        RouteResponsePolicy::DigestBoundPagination
    );

    let mutation = route_manifest_entry("POST", OPERATOR_MUTATION_COMMIT_ENDPOINT)
        .expect("operator mutation manifest entry");
    assert_eq!(mutation.report_descriptor(), None);
}

#[test]
fn manifest_marks_quarantined_legacy_effects_as_retired_dispatch() {
    for route in CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.is_quarantined_transitive_effect())
    {
        let entry = route_manifest_entry(route.method, route.pattern)
            .unwrap_or_else(|| panic!("missing quarantine entry: {}", route.pattern));
        assert_eq!(
            entry.dispatch_handler,
            RouteDispatchHandler::RetiredCompatibility
        );
        assert!(!entry.watchdog_probe);
    }
}

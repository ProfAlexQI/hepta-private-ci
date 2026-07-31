use serde::Serialize;

use crate::native_telegram::TELEGRAM_LIVE_READ_ENV;
use crate::operator_mutation::OPERATOR_MUTATION_ENABLED_ENV;
use crate::route_registry::CONTROL_UI_ROUTE_SPECS;
use crate::route_registry::EVIDENCE_INDEX_ENDPOINT;
use crate::route_registry::TELEGRAM_LIVE_SOAK_ROUTE;
use crate::runtime_ingress::IngressLifecycleSpec;
use crate::runtime_ingress::declared_runtime_ingress_lifecycle;
use crate::runtime_ingress::runtime_ingress_lifecycle_registry;
use crate::runtime_mutation::RUNTIME_MUTATION_CANARY_ENV;
use crate::telegram_authority::TELEGRAM_AUTHORITY_ENABLED_ENV;

pub(crate) const WATCHDOG_PROBE_PATHS: &[&str] = &[
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
    EvidenceIndex,
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
#[serde(rename_all = "snake_case")]
pub(crate) enum RouteReportBinding {
    None,
    NativeExact,
    NativeParameterized,
    NativeBinaryAsset,
    CanonicalEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub(crate) struct NativeReportId(pub(crate) u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub(crate) struct RouteDefinition {
    #[serde(flatten)]
    pub(crate) lifecycle: IngressLifecycleSpec,
    pub(crate) dispatch_handler: RouteDispatchHandler,
    pub(crate) required_gate: Option<&'static str>,
    pub(crate) watchdog_probe: bool,
    pub(crate) response_policy: RouteResponsePolicy,
    pub(crate) report_binding: RouteReportBinding,
    pub(crate) native_report_id: Option<NativeReportId>,
    pub(crate) source_command: Option<&'static str>,
    pub(crate) capability: Option<&'static str>,
    pub(crate) side_effect_boundary: Option<&'static str>,
    pub(crate) receipt_state: Option<crate::gate_spec::ReceiptState>,
    pub(crate) evidence_effect_class: Option<&'static str>,
    pub(crate) aliases: &'static [&'static str],
    pub(crate) legacy_compatibility_route: bool,
}

pub(crate) fn route_definition(method: &str, path: &str) -> Option<RouteDefinition> {
    declared_runtime_ingress_lifecycle(method, path).map(route_definition_from_lifecycle)
}

pub(crate) fn route_definition_registry() -> Vec<RouteDefinition> {
    runtime_ingress_lifecycle_registry()
        .into_iter()
        .map(route_definition_from_lifecycle)
        .collect()
}

fn route_definition_from_lifecycle(lifecycle: IngressLifecycleSpec) -> RouteDefinition {
    let dispatch_handler = dispatch_handler(lifecycle);
    let required_gate = required_gate(lifecycle);
    let gate = CONTROL_UI_ROUTE_SPECS
        .iter()
        .find(|gate| gate.method == lifecycle.method && gate.pattern == lifecycle.path_pattern);
    let receipt_state = gate.and_then(crate::gate_spec::GateSpec::receipt_state);
    let (report_binding, native_report_id) = report_binding(lifecycle, dispatch_handler);
    RouteDefinition {
        lifecycle,
        dispatch_handler,
        required_gate,
        watchdog_probe: lifecycle.method == "GET"
            && WATCHDOG_PROBE_PATHS.contains(&lifecycle.path_pattern),
        response_policy: response_policy(lifecycle),
        report_binding,
        native_report_id,
        source_command: gate.map(|gate| gate.source_command),
        capability: gate.map(|gate| gate.capability),
        side_effect_boundary: gate.map(|gate| gate.side_effect_boundary),
        receipt_state,
        evidence_effect_class: gate.map(evidence_effect_class),
        aliases: if lifecycle.path_pattern == TELEGRAM_LIVE_SOAK_ROUTE.canonical {
            TELEGRAM_LIVE_SOAK_ROUTE.aliases
        } else {
            &[]
        },
        legacy_compatibility_route: receipt_state.is_some()
            && lifecycle.path_pattern != EVIDENCE_INDEX_ENDPOINT,
    }
}

fn report_binding(
    lifecycle: IngressLifecycleSpec,
    dispatch_handler: RouteDispatchHandler,
) -> (RouteReportBinding, Option<NativeReportId>) {
    if lifecycle.method != "GET" {
        return (RouteReportBinding::None, None);
    }
    match dispatch_handler {
        RouteDispatchHandler::EvidenceIndex => (RouteReportBinding::CanonicalEvidence, None),
        RouteDispatchHandler::NativeGateway => {
            if let Some(report_id) = crate::native_gateway::native_report_registry::native_report_id(
                lifecycle.path_pattern,
            ) {
                return (RouteReportBinding::NativeExact, Some(report_id));
            }
            if crate::ui_domain::NATIVE_GATEWAY_BINARY_ASSET_PATHS.contains(&lifecycle.path_pattern)
            {
                return (RouteReportBinding::NativeBinaryAsset, None);
            }
            if lifecycle.path_pattern.contains('<') {
                return (RouteReportBinding::NativeParameterized, None);
            }
            (RouteReportBinding::None, None)
        }
        _ => (RouteReportBinding::None, None),
    }
}

fn dispatch_handler(lifecycle: IngressLifecycleSpec) -> RouteDispatchHandler {
    if lifecycle.path_pattern == EVIDENCE_INDEX_ENDPOINT {
        return RouteDispatchHandler::EvidenceIndex;
    }
    match lifecycle.source {
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
    }
}

fn required_gate(lifecycle: IngressLifecycleSpec) -> Option<&'static str> {
    match lifecycle.source {
        "telegram_receive_once" => Some(TELEGRAM_LIVE_READ_ENV),
        "telegram_operator_authority" | "telegram_terminal_reconciliation" => {
            Some(TELEGRAM_AUTHORITY_ENABLED_ENV)
        }
        "runtime_mutation_canary" => Some(RUNTIME_MUTATION_CANARY_ENV),
        "operator_mutation" | "operator_mutation_reconciliation" => {
            Some(OPERATOR_MUTATION_ENABLED_ENV)
        }
        _ => None,
    }
}

fn evidence_effect_class(spec: &crate::gate_spec::GateSpec) -> &'static str {
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

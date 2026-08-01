use crate::route_registry::CONTROL_UI_ROUTE_SPECS;
use crate::runtime_ingress::IngressLifecycleSpec;

const CANONICAL_ONLY_EVIDENCE_FAMILY_PREFIXES: &[&str] = &[
    "/api/hepta-memory-intelligence-kg-full-live-activation-",
    "/api/hepta-memory-live-mutation-operator-write-execution-",
    "/api/hepta-memory-intelligence-kg-full-enablement-",
];

const RETAINED_DIRECT_EVIDENCE_ROUTES: &[&str] = &[
    "/api/hepta-memory-intelligence-kg-full-enablement-runtime-readiness",
    "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-readiness",
    "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence",
    "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-authority-denial",
    "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-separation",
    "/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-acceptance-receipt-lane",
    "/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-readback-audit-receipt-lane",
    "/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-receipt-audit-lane",
    "/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-precondition-lane",
];

pub(super) fn is_canonical_only_evidence_route(lifecycle: IngressLifecycleSpec) -> bool {
    if lifecycle.method != "GET"
        || RETAINED_DIRECT_EVIDENCE_ROUTES.contains(&lifecycle.path_pattern)
        || !CANONICAL_ONLY_EVIDENCE_FAMILY_PREFIXES
            .iter()
            .any(|prefix| lifecycle.path_pattern.starts_with(prefix))
    {
        return false;
    }
    CONTROL_UI_ROUTE_SPECS
        .iter()
        .find(|spec| spec.method == lifecycle.method && spec.pattern == lifecycle.path_pattern)
        .and_then(crate::gate_spec::GateSpec::receipt_state)
        .is_some()
}

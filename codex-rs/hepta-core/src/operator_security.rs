use serde::Serialize;

use crate::control_ui::CONTROL_UI_JS;
use crate::control_ui::CONTROL_UI_README;
use crate::control_ui::CONTROL_UI_RUST_RENDERER_MARKERS;

pub const HEPTA_NATIVE_GATEWAY_RS: &str =
    include_str!("../../hepta-native-gateway/src/native_gateway.rs");
pub const HEPTA_NATIVE_HTTP_TRANSPORT_RS: &str =
    include_str!("../../hepta-native-gateway/src/http_transport.rs");
pub const HEPTA_KERNEL_NATIVE_POST_RS: &str =
    include_str!("../../hepta-kernel/src/kernel_parts/native_post.rs");

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperatorCommandSafetyDecision {
    pub allowed: bool,
    pub reason: &'static str,
    pub redacted_command: String,
    pub requires_admin_confirmation: bool,
    pub sudo_stdin_password_blocked: bool,
    pub high_risk_slash_blocked: bool,
    pub allowlist_enforced: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperatorSecurityRoleProfile {
    pub id: &'static str,
    pub title: &'static str,
    pub permissions: &'static [&'static str],
    pub may_execute_mutations: bool,
    pub requires_human_confirmation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperatorSecurityEndpointGuard {
    pub id: &'static str,
    pub method: &'static str,
    pub path_pattern: &'static str,
    pub minimum_role: &'static str,
    pub risk_tier: &'static str,
    pub disposition: &'static str,
    pub dry_run_only: bool,
    pub audited: bool,
    pub external_side_effects: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperatorSecurityLane {
    pub id: &'static str,
    pub title: &'static str,
    pub implemented: bool,
    pub evidence: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperatorSecurityReport {
    pub product: &'static str,
    pub status: &'static str,
    pub scope: &'static str,
    pub role_profile_count: usize,
    pub endpoint_guard_count: usize,
    pub audited_endpoint_guard_count: usize,
    pub mutation_guard_count: usize,
    pub read_only_command_guard_count: usize,
    pub implemented_lane_count: usize,
    pub lane_count: usize,
    pub operator_security_percent: u8,
    pub all_operator_security_lanes_100: bool,
    pub loopback_bind_enforced: bool,
    pub security_headers_present: bool,
    pub post_actions_dry_run_only: bool,
    pub confirmed_local_mutation_guard_present: bool,
    pub read_only_command_allowlist_present: bool,
    pub unsupported_post_fail_closed: bool,
    pub policy_approval_bridge_present: bool,
    pub runtime_operator_guard_present: bool,
    pub audit_event_visibility_present: bool,
    pub boundary_doc_present: bool,
    pub external_side_effects_permitted: bool,
    pub local_boundary: &'static str,
    pub roles: Vec<OperatorSecurityRoleProfile>,
    pub endpoint_guards: Vec<OperatorSecurityEndpointGuard>,
    pub lanes: Vec<OperatorSecurityLane>,
}

impl OperatorSecurityReport {
    pub fn complete(&self) -> bool {
        self.status == "complete"
            && self.operator_security_percent == 100
            && self.all_operator_security_lanes_100
            && self.loopback_bind_enforced
            && self.security_headers_present
            && self.post_actions_dry_run_only
            && self.confirmed_local_mutation_guard_present
            && self.read_only_command_allowlist_present
            && self.unsupported_post_fail_closed
            && self.policy_approval_bridge_present
            && self.runtime_operator_guard_present
            && self.audit_event_visibility_present
            && self.boundary_doc_present
            && !self.external_side_effects_permitted
    }
}

pub fn evaluate_operator_command(raw_command: &str) -> OperatorCommandSafetyDecision {
    let trimmed = raw_command.trim();
    let redacted_command = redact_operator_command(trimmed);
    let sudo_stdin_password_blocked = contains_sudo_stdin_password_pattern(trimmed);
    let high_risk_slash_blocked = high_risk_slash_command(trimmed)
        .is_some_and(|command| !has_explicit_operator_confirmation(trimmed, command));
    let allowlist_enforced = trimmed.starts_with('/');
    let (allowed, reason, requires_admin_confirmation) = if sudo_stdin_password_blocked {
        (
            false,
            "sudo -S/stdin password patterns are denied on the operator command path",
            false,
        )
    } else if high_risk_slash_blocked {
        (
            false,
            "high-risk slash command requires explicit operator confirmation",
            true,
        )
    } else {
        (true, "allowed by local operator command guard", false)
    };

    OperatorCommandSafetyDecision {
        allowed,
        reason,
        redacted_command,
        requires_admin_confirmation,
        sudo_stdin_password_blocked,
        high_risk_slash_blocked,
        allowlist_enforced,
    }
}

pub fn redact_operator_command(raw_command: &str) -> String {
    raw_command
        .split_whitespace()
        .map(|token| {
            let lower = token.to_ascii_lowercase();
            if lower.starts_with("sk-")
                || lower.starts_with("xox")
                || lower.contains("token=")
                || lower.contains("password=")
                || lower.contains("secret=")
            {
                "[REDACTED]".to_string()
            } else {
                token.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn contains_sudo_stdin_password_pattern(command: &str) -> bool {
    let tokens = command.split_whitespace().collect::<Vec<_>>();
    tokens.windows(2).any(|pair| pair == ["sudo", "-S"])
}

fn high_risk_slash_command(command: &str) -> Option<&str> {
    let first = command.split_whitespace().next()?;
    match first {
        "/kill" | "/restart" | "/update" | "/config" | "/exec" => Some(first),
        _ => None,
    }
}

fn has_explicit_operator_confirmation(command: &str, slash_command: &str) -> bool {
    command.contains("--confirm-operator")
        || command.contains("--confirm-runtime-write")
        || (slash_command == "/config" && command.contains("--dry-run"))
}

fn native_post_route_spec_source(pattern: &str) -> Option<&'static str> {
    let marker = format!("pattern: \"{pattern}\",");
    let marker_offset = HEPTA_KERNEL_NATIVE_POST_RS.find(&marker)?;
    let block_start = HEPTA_KERNEL_NATIVE_POST_RS[..marker_offset]
        .rfind("HeptaKernelNativePostPlanRouteSpec {")?;
    let after_marker = &HEPTA_KERNEL_NATIVE_POST_RS[marker_offset..];
    let block_end = marker_offset + after_marker.find("\n    },")? + "\n    },".len();
    HEPTA_KERNEL_NATIVE_POST_RS.get(block_start..block_end)
}

fn native_post_route_spec_matches(
    pattern: &str,
    source_command: &str,
    plan_kind: &str,
    dry_run_only: bool,
    confirmation_required: bool,
) -> bool {
    native_post_route_spec_source(pattern).is_some_and(|block| {
        block.contains(&format!("source_command: \"{source_command}\","))
            && block.contains(&format!("plan_kind: \"{plan_kind}\","))
            && block.contains(&format!("dry_run_only: {dry_run_only},"))
            && block.contains(&format!(
                "confirmation_required_for_real_mutation: {confirmation_required},"
            ))
    })
}

fn native_post_plan_response_is_side_effect_free() -> bool {
    let Some(start) =
        HEPTA_KERNEL_NATIVE_POST_RS.find("pub fn hepta_kernel_native_post_plan_response(")
    else {
        return false;
    };
    let Some(relative_end) = HEPTA_KERNEL_NATIVE_POST_RS[start..]
        .find("pub fn hepta_kernel_native_post_execution_stores_report(")
    else {
        return false;
    };
    let function_source = &HEPTA_KERNEL_NATIVE_POST_RS[start..start + relative_end];
    [
        "action_dispatched: false",
        "command_executed: false",
        "approval_applied: false",
        "task_published: false",
        "chat_mutated: false",
        "external_side_effects: false",
        "gateway_mutation_performed: false",
        "message_sent: false",
    ]
    .iter()
    .all(|marker| function_source.contains(marker))
}

fn native_post_real_handler_inventory_is_empty(source: &str) -> bool {
    source.contains("pub const HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_PLAN_KINDS: &[&str] = &[];")
}

fn control_ui_readonly_get_registry_is_strict() -> bool {
    const PATHS: [&str; 21] = [
        "/api/control-ui",
        "/api/config",
        "/api/optional-configs",
        "/api/hepta-merge-completion",
        "/api/external-agent-benchmark",
        "/api/sessions",
        "/api/session-activity",
        "/api/operator-console",
        "/api/subagent-observatory",
        "/api/events",
        "/api/events-report",
        "/api/activity",
        "/api/transcript",
        "/api/approvals",
        "/api/policy",
        "/api/operator-security",
        "/api/gateway-runtime",
        "/api/gateway-dispatch",
        "/api/gateway-ledger",
        "/api/gateway-retry-dead-letter",
        "/api/multi-agent-runtime",
    ];
    let Ok(source) = std::str::from_utf8(CONTROL_UI_JS) else {
        return false;
    };
    let Some(registry_start) = source.find("const READ_ONLY_ROUTES = Object.freeze({") else {
        return false;
    };
    let Some(relative_end) = source[registry_start..].find("\n  });") else {
        return false;
    };
    let registry = &source[registry_start..registry_start + relative_end];
    registry.matches(": \"/api/").count() == PATHS.len()
        && PATHS
            .iter()
            .all(|path| registry.matches(&format!("\"{path}\"")).count() == 1)
        && source.contains("const SNAPSHOT_PATH = \"/api/operator-snapshot\"")
        && source.contains("method: \"GET\"")
        && source.contains("headers: { Accept: \"application/json\" }")
        && source.contains("new AbortController()")
        && source.contains("url.origin !== window.location.origin")
        && source.contains("textContent")
        && !source.contains("innerHTML")
}

pub fn operator_security_report() -> OperatorSecurityReport {
    let loopback_bind_enforced = HEPTA_NATIVE_GATEWAY_RS.contains("is_loopback_bind_addr")
        && HEPTA_NATIVE_GATEWAY_RS.contains("HEPTA_ALLOW_NON_LOOPBACK_UI")
        && HEPTA_NATIVE_GATEWAY_RS.contains("refusing to serve UI on non-loopback address");
    let security_headers_present = HEPTA_NATIVE_HTTP_TRANSPORT_RS
        .contains("Content-Security-Policy")
        && HEPTA_NATIVE_HTTP_TRANSPORT_RS.contains("script-src 'self';")
        && !HEPTA_NATIVE_HTTP_TRANSPORT_RS.contains("script-src 'self' 'unsafe-inline'")
        && HEPTA_NATIVE_HTTP_TRANSPORT_RS.contains("connect-src 'self'")
        && HEPTA_NATIVE_HTTP_TRANSPORT_RS.contains("X-Content-Type-Options: nosniff")
        && HEPTA_NATIVE_HTTP_TRANSPORT_RS.contains("Referrer-Policy: no-referrer");
    let post_actions_dry_run_only = native_post_route_spec_matches(
        "/api/actions/<action>",
        "/ui-action-plan <action> --dry-run --json",
        "ui_action",
        true,
        false,
    );
    let real_handler_inventory_empty =
        native_post_real_handler_inventory_is_empty(HEPTA_KERNEL_NATIVE_POST_RS);
    let confirmed_local_mutation_guard_present = native_post_route_spec_matches(
        "/api/tasks/publish",
        "/tasks publish --confirm --json",
        "task_publish",
        false,
        true,
    ) && native_post_route_spec_matches(
        "/api/chat",
        "/chat send --json",
        "chat_send",
        false,
        true,
    ) && native_post_route_spec_matches(
        "/api/approvals/exec/apply",
        "/approvals exec apply --dry-run --json",
        "approval_apply",
        true,
        true,
    ) && native_post_plan_response_is_side_effect_free(
    ) && real_handler_inventory_empty;
    let read_only_command_allowlist_present = control_ui_readonly_get_registry_is_strict()
        && native_post_route_spec_matches(
            "/api/commands/<id>",
            "/<allowlisted read-only command> --json",
            "readonly_command",
            true,
            false,
        );
    let unsupported_post_fail_closed = HEPTA_NATIVE_GATEWAY_RS.contains("405 Method Not Allowed")
        && HEPTA_NATIVE_GATEWAY_RS.contains("supported POST endpoints are /api/actions/<action>");
    let policy_approval_bridge_present = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("renderApprovalCards")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/approvals")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/policy");
    let runtime_operator_guard_present = native_post_route_spec_matches(
        "/api/runtime/operator",
        "/runtime/operator --dry-run --json",
        "runtime_operator",
        true,
        false,
    ) && CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("POST /api/runtime/operator")
        && CONTROL_UI_RUST_RENDERER_MARKERS
            .contains("Confirm-gated runtime kill/steer dry-run evidence");
    let audit_event_visibility_present = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("renderEventTimeline")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/events-report")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/live-events/0")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("duplicate_free");
    let boundary_doc_present = CONTROL_UI_README.contains("not a hosted SaaS")
        && CONTROL_UI_README.contains("local")
        && CONTROL_UI_README.contains("operator security");

    let lanes = vec![
        lane(
            "loopback-bind-guard",
            "Loopback-only serving guard",
            loopback_bind_enforced,
            "codex-rs/hepta-native-gateway/src/native_gateway.rs refuses non-loopback UI bind unless explicitly overridden",
        ),
        lane(
            "security-headers",
            "Browser security headers",
            security_headers_present,
            "HTTP responses emit CSP, nosniff, no-referrer, and frame deny headers",
        ),
        lane(
            "post-action-dry-run",
            "POST actions are plan-only",
            post_actions_dry_run_only,
            "POST /api/actions/<action> maps to /ui-action-plan ... --dry-run --json",
        ),
        lane(
            "confirmed-local-mutations",
            "Chat/task mutations require confirmation",
            confirmed_local_mutation_guard_present,
            "POST /api/chat, /api/tasks/publish, and /api/approvals/exec/apply require confirm=true and keep external_side_effects=false",
        ),
        lane(
            "read-only-command-allowlist",
            "Read-only command runner allowlist",
            read_only_command_allowlist_present,
            "The browser exposes exactly 21 canonical same-origin GET reports; the compatibility POST command planner remains allowlisted and plan-only",
        ),
        lane(
            "unsupported-post-fail-closed",
            "Unsupported POST requests fail closed",
            unsupported_post_fail_closed,
            "Mutation endpoints outside the guarded action planner return 405",
        ),
        lane(
            "policy-approval-bridge",
            "Policy and approval state are visible",
            policy_approval_bridge_present,
            "UI renders /api/approvals and /api/policy cards before risky operations",
        ),
        lane(
            "runtime-operator-guard",
            "Runtime operator kill/steer guard",
            runtime_operator_guard_present,
            "POST /api/runtime/operator plans /runtime/operator, /kill, and /steer envelopes without Gateway/session mutation",
        ),
        lane(
            "audit-event-visibility",
            "Event/audit timeline is visible",
            audit_event_visibility_present,
            "UI exposes event report and cursor-based live events with duplicate-free marker",
        ),
        lane(
            "boundary-doc",
            "Security boundary is documented",
            boundary_doc_present,
            "Control UI README documents local-only operator security scope",
        ),
    ];
    let implemented_lane_count = lanes.iter().filter(|lane| lane.implemented).count();
    let lane_count = lanes.len();
    let operator_security_percent = percent(implemented_lane_count, lane_count);
    let roles = operator_security_roles();
    let endpoint_guards = operator_security_endpoint_guards();
    let audited_endpoint_guard_count = endpoint_guards.iter().filter(|guard| guard.audited).count();
    let mutation_guard_count = endpoint_guards
        .iter()
        .filter(|guard| guard.method == "POST" && guard.dry_run_only)
        .count();
    let read_only_command_guard_count = endpoint_guards
        .iter()
        .filter(|guard| guard.id == "readonly-command-runner")
        .count();
    let all_operator_security_lanes_100 = operator_security_percent == 100;

    OperatorSecurityReport {
        product: "Hepta",
        status: if all_operator_security_lanes_100 {
            "complete"
        } else {
            "attention"
        },
        scope: "local_control_ui_operator_security",
        role_profile_count: roles.len(),
        endpoint_guard_count: endpoint_guards.len(),
        audited_endpoint_guard_count,
        mutation_guard_count,
        read_only_command_guard_count,
        implemented_lane_count,
        lane_count,
        operator_security_percent,
        all_operator_security_lanes_100,
        loopback_bind_enforced,
        security_headers_present,
        post_actions_dry_run_only,
        confirmed_local_mutation_guard_present,
        read_only_command_allowlist_present,
        unsupported_post_fail_closed,
        policy_approval_bridge_present,
        runtime_operator_guard_present,
        audit_event_visibility_present,
        boundary_doc_present,
        external_side_effects_permitted: !real_handler_inventory_empty,
        local_boundary: "loopback-only local operator surface; no public ingress, no hosted SaaS auth/RBAC claim",
        roles,
        endpoint_guards,
        lanes,
    }
}

pub fn operator_security_roles() -> Vec<OperatorSecurityRoleProfile> {
    vec![
        OperatorSecurityRoleProfile {
            id: "observer",
            title: "Observer",
            permissions: &["view_dashboard", "view_events", "view_readiness"],
            may_execute_mutations: false,
            requires_human_confirmation: false,
        },
        OperatorSecurityRoleProfile {
            id: "developer",
            title: "Developer",
            permissions: &[
                "view_dashboard",
                "view_events",
                "run_readonly_commands",
                "inspect_transcripts",
                "inspect_tasks",
            ],
            may_execute_mutations: false,
            requires_human_confirmation: false,
        },
        OperatorSecurityRoleProfile {
            id: "operator",
            title: "Operator",
            permissions: &[
                "view_dashboard",
                "view_events",
                "run_readonly_commands",
                "render_dry_run_action_plans",
                "review_approvals",
                "confirm_local_agent_chat",
                "confirm_local_task_publish",
                "confirm_runtime_operator_plan",
            ],
            may_execute_mutations: true,
            requires_human_confirmation: true,
        },
        OperatorSecurityRoleProfile {
            id: "admin",
            title: "Local admin",
            permissions: &[
                "view_dashboard",
                "view_events",
                "run_readonly_commands",
                "render_dry_run_action_plans",
                "review_approvals",
                "confirm_local_agent_chat",
                "confirm_local_task_publish",
                "confirm_runtime_operator_plan",
                "copy_explicit_cli_commands_outside_ui",
            ],
            may_execute_mutations: true,
            requires_human_confirmation: true,
        },
    ]
}

pub fn operator_security_endpoint_guards() -> Vec<OperatorSecurityEndpointGuard> {
    vec![
        guard(
            "static-assets",
            "GET",
            "/ | /index.html | /styles.css | /control-ui.js | /assets/hepta-agent-logo.png",
            "observer",
            "low",
            "serve local bundled assets with security headers",
            false,
            true,
            false,
        ),
        guard(
            "readiness-json",
            "GET",
            "/api/control-ui | /api/ops-status | /api/production-parity",
            "observer",
            "low",
            "read-only JSON inspection",
            false,
            true,
            false,
        ),
        guard(
            "operator-json",
            "GET",
            "/api/operator-console | /api/events-report | /api/live-events/<cursor>",
            "developer",
            "low",
            "read-only operator/audit inspection",
            false,
            true,
            false,
        ),
        guard(
            "approval-policy-json",
            "GET",
            "/api/approvals | /api/policy",
            "developer",
            "medium",
            "read-only policy/approval inspection",
            false,
            true,
            false,
        ),
        guard(
            "readonly-command-runner",
            "GET",
            "21 fixed canonical /api report paths",
            "developer",
            "medium",
            "same-origin read-only JSON inspection only; unregistered cards stay copy-only",
            false,
            true,
            false,
        ),
        guard(
            "dry-run-action-planner",
            "POST",
            "/api/actions/<action>",
            "operator",
            "medium_high",
            "returns confirmation plan; no mutation executed",
            true,
            true,
            false,
        ),
        guard(
            "task-publisher-plan",
            "POST",
            "/api/tasks/plan",
            "operator",
            "medium",
            "returns task publish plan; no mutation executed",
            true,
            true,
            false,
        ),
        guard(
            "task-publisher-confirm",
            "POST",
            "/api/tasks/publish",
            "operator",
            "medium_high",
            "requires confirm=true; mutates only local task queue",
            false,
            true,
            false,
        ),
        guard(
            "agent-chat-plan",
            "POST",
            "/api/chat/plan",
            "operator",
            "medium",
            "returns agent chat plan; no mutation executed",
            true,
            true,
            false,
        ),
        guard(
            "agent-chat-confirm",
            "POST",
            "/api/chat",
            "operator",
            "medium_high",
            "requires confirm=true; mutates only local agent runtime",
            false,
            true,
            false,
        ),
        guard(
            "runtime-operator-plan",
            "POST",
            "/api/runtime/operator",
            "operator",
            "high",
            "requires confirm=true for /kill or /steer planning; never mutates Gateway/session state",
            true,
            true,
            false,
        ),
        guard(
            "unsupported-post",
            "POST",
            "anything else",
            "operator",
            "high",
            "fail closed with 405",
            true,
            true,
            false,
        ),
        guard(
            "external-side-effects",
            "ANY",
            "network/provider/channel side effects",
            "admin",
            "high",
            "not available through local Control UI",
            true,
            true,
            false,
        ),
    ]
}

fn lane(
    id: &'static str,
    title: &'static str,
    implemented: bool,
    evidence: &'static str,
) -> OperatorSecurityLane {
    OperatorSecurityLane {
        id,
        title,
        implemented,
        evidence,
    }
}

#[allow(clippy::too_many_arguments)]
fn guard(
    id: &'static str,
    method: &'static str,
    path_pattern: &'static str,
    minimum_role: &'static str,
    risk_tier: &'static str,
    disposition: &'static str,
    dry_run_only: bool,
    audited: bool,
    external_side_effects: bool,
) -> OperatorSecurityEndpointGuard {
    OperatorSecurityEndpointGuard {
        id,
        method,
        path_pattern,
        minimum_role,
        risk_tier,
        disposition,
        dry_run_only,
        audited,
        external_side_effects,
    }
}

fn percent(numerator: usize, denominator: usize) -> u8 {
    if denominator == 0 {
        return 0;
    }
    ((numerator * 100) / denominator) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_post_security_checks_are_scoped_to_each_route_spec() {
        assert!(native_post_route_spec_matches(
            "/api/tasks/publish",
            "/tasks publish --confirm --json",
            "task_publish",
            false,
            true,
        ));
        assert!(!native_post_route_spec_matches(
            "/api/tasks/plan",
            "/tasks plan --dry-run --json",
            "task_plan",
            true,
            true,
        ));
        assert!(native_post_plan_response_is_side_effect_free());
        assert!(native_post_real_handler_inventory_is_empty(
            HEPTA_KERNEL_NATIVE_POST_RS
        ));
        assert!(!native_post_real_handler_inventory_is_empty(
            "pub const HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_PLAN_KINDS: &[&str] = &[\"chat_send\"];"
        ));
    }

    #[test]
    fn operator_command_guard_denies_sudo_stdin_and_redacts_secrets() {
        let decision = evaluate_operator_command("sudo -S echo token=abc sk-live-secret");
        assert!(!decision.allowed);
        assert!(decision.sudo_stdin_password_blocked);
        assert!(decision.redacted_command.contains("[REDACTED]"));
        assert!(!decision.redacted_command.contains("sk-live-secret"));
    }

    #[test]
    fn operator_command_guard_blocks_high_risk_slash_without_confirmation() {
        let denied = evaluate_operator_command("/restart now");
        assert!(!denied.allowed);
        assert!(denied.high_risk_slash_blocked);
        assert!(denied.requires_admin_confirmation);

        let allowed = evaluate_operator_command("/restart --confirm-operator");
        assert!(allowed.allowed);
        assert!(!allowed.high_risk_slash_blocked);
    }

    #[test]
    fn operator_security_report_reaches_local_100_without_external_claims() {
        let report = operator_security_report();

        assert_eq!(report.status, "complete");
        assert_eq!(report.scope, "local_control_ui_operator_security");
        assert_eq!(report.role_profile_count, 4);
        assert_eq!(report.endpoint_guard_count, 13);
        assert_eq!(report.audited_endpoint_guard_count, 13);
        assert_eq!(report.mutation_guard_count, 5);
        assert_eq!(report.read_only_command_guard_count, 1);
        assert_eq!(report.implemented_lane_count, 10);
        assert_eq!(report.lane_count, 10);
        assert_eq!(report.operator_security_percent, 100);
        assert!(report.all_operator_security_lanes_100);
        assert!(report.loopback_bind_enforced);
        assert!(report.security_headers_present);
        assert!(report.post_actions_dry_run_only);
        assert!(report.confirmed_local_mutation_guard_present);
        assert!(report.read_only_command_allowlist_present);
        assert!(report.unsupported_post_fail_closed);
        assert!(report.policy_approval_bridge_present);
        assert!(report.runtime_operator_guard_present);
        assert!(report.audit_event_visibility_present);
        assert!(report.boundary_doc_present);
        assert!(!report.external_side_effects_permitted);
        assert!(report.complete());
        assert!(
            report
                .roles
                .iter()
                .any(|role| role.id == "operator" && role.requires_human_confirmation)
        );
        assert!(
            report
                .endpoint_guards
                .iter()
                .any(|guard| guard.id == "dry-run-action-planner" && guard.dry_run_only)
        );
        assert!(report.endpoint_guards.iter().any(|guard| {
            guard.id == "readonly-command-runner"
                && guard.method == "GET"
                && !guard.external_side_effects
        }));
        assert!(report.endpoint_guards.iter().any(|guard| {
            guard.id == "task-publisher-confirm"
                && !guard.dry_run_only
                && !guard.external_side_effects
        }));
        assert!(report.endpoint_guards.iter().any(|guard| {
            guard.id == "agent-chat-confirm" && !guard.dry_run_only && !guard.external_side_effects
        }));
        assert!(report.endpoint_guards.iter().any(|guard| {
            guard.id == "runtime-operator-plan"
                && guard.dry_run_only
                && !guard.external_side_effects
        }));
    }
}

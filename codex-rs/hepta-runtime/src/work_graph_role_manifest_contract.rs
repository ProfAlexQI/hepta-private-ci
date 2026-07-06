use serde::Serialize;

pub const WORK_GRAPH_ROLE_MANIFEST_CONTRACT_PREVIEW_GATE: &str =
    "hepta_work_graph_role_manifest_contract_preview_gate";
pub const WORK_GRAPH_ROLE_MANIFEST_CONTRACT_SCHEMA_VERSION: &str =
    "work_graph_role_manifest_contract_preview_v1";
pub const WORK_GRAPH_ROLE_MANIFEST_CONTRACT_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_unified_state_store_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestContractPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub required_field_count: usize,
    pub capability_count: usize,
    pub permission_mode_count: usize,
    pub invariant_count: usize,
    pub adapter_preview_count: usize,
    pub required_fields: Vec<WorkGraphRoleManifestFieldPreview>,
    pub capabilities: Vec<WorkGraphRoleCapabilityPreview>,
    pub permission_modes: Vec<WorkGraphRolePermissionModePreview>,
    pub invariants: Vec<WorkGraphRoleManifestInvariantPreview>,
    pub adapter_previews: Vec<WorkGraphRoleManifestAdapterPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_unified_state_store_preview: bool,
    pub ready_for_role_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphRoleManifestContractPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestFieldPreview {
    pub wire_name: &'static str,
    pub required: bool,
    pub purpose: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleCapabilityPreview {
    pub id: &'static str,
    pub requires_tool_permission: bool,
    pub requires_verifier: bool,
    pub purpose: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRolePermissionModePreview {
    pub id: &'static str,
    pub can_mutate_runtime: bool,
    pub requires_approval: bool,
    pub purpose: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestAdapterPreview {
    pub source_surface_id: &'static str,
    pub projected_role_kind: &'static str,
    pub covered_wire_fields: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub blocker_ids: Vec<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestContractPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub role_enforcement_enabled: bool,
    pub tool_permission_changed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_role_manifest_contract_preview_report()
-> WorkGraphRoleManifestContractPreviewReport {
    let required_fields = work_graph_role_manifest_required_fields();
    let capabilities = work_graph_role_manifest_capabilities();
    let permission_modes = work_graph_role_manifest_permission_modes();
    let invariants = work_graph_role_manifest_invariants();
    let adapter_previews = work_graph_role_manifest_adapter_previews();

    WorkGraphRoleManifestContractPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_ROLE_MANIFEST_CONTRACT_PREVIEW_GATE,
        schema_version: WORK_GRAPH_ROLE_MANIFEST_CONTRACT_SCHEMA_VERSION,
        preview_mode: "read_only_role_manifest_contract_preview_no_enforcement",
        required_field_count: required_fields.len(),
        capability_count: capabilities.len(),
        permission_mode_count: permission_modes.len(),
        invariant_count: invariants.len(),
        adapter_preview_count: adapter_previews.len(),
        required_fields,
        capabilities,
        permission_modes,
        invariants,
        adapter_previews,
        recommended_next_gate: WORK_GRAPH_ROLE_MANIFEST_CONTRACT_RECOMMENDED_NEXT_GATE,
        ready_for_unified_state_store_preview: true,
        ready_for_role_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphRoleManifestContractPreviewSideEffects::none(),
    }
}

pub fn work_graph_role_manifest_required_fields() -> Vec<WorkGraphRoleManifestFieldPreview> {
    vec![
        field(
            "roleId",
            "stable role identity for task admission and audit",
        ),
        field(
            "roleKind",
            "planner, builder, reviewer, verifier, operator, or handoff role category",
        ),
        field("agentPath", "canonical agent path or role binding target"),
        field(
            "capabilities",
            "declared capabilities that the role may exercise",
        ),
        field(
            "toolPermissions",
            "tool families, scopes, and permission modes",
        ),
        field(
            "outputSchemaRef",
            "TaskResult or domain output schema expected from the role",
        ),
        field(
            "verifierRef",
            "gate, reducer, or reviewer that can accept the role output",
        ),
        field(
            "budget",
            "token, wall-clock, attempt, and command budget envelope",
        ),
        field(
            "concurrency",
            "max concurrent tasks, child agents, and tool slots",
        ),
        field("lane", "lane ownership and workspace routing boundary"),
        field("approvalPolicy", "operator approval and escalation rules"),
        field("tracePolicy", "trace, redaction, and evidence requirements"),
    ]
}

pub fn work_graph_role_manifest_capabilities() -> Vec<WorkGraphRoleCapabilityPreview> {
    vec![
        capability(
            "planning",
            false,
            true,
            "may create or update plan_step nodes without executing work",
        ),
        capability(
            "agent_delegation",
            true,
            true,
            "may propose or spawn agent_task nodes under bounded role rules",
        ),
        capability(
            "code_editing",
            true,
            true,
            "may produce patches or artifacts in a scoped workspace",
        ),
        capability(
            "verification",
            true,
            true,
            "may run local gates and produce verification_gate evidence",
        ),
        capability(
            "research",
            true,
            true,
            "may gather read-only evidence from approved tools or sources",
        ),
        capability(
            "scheduler_control",
            true,
            true,
            "may propose scheduler_run admission or retry decisions",
        ),
        capability(
            "external_handoff_proposal",
            true,
            true,
            "may prepare external_handoff previews without delivery authority",
        ),
    ]
}

pub fn work_graph_role_manifest_permission_modes() -> Vec<WorkGraphRolePermissionModePreview> {
    vec![
        permission_mode(
            "deny",
            false,
            false,
            "capability or tool family is unavailable",
        ),
        permission_mode(
            "preview",
            false,
            false,
            "role may produce a side-effect-free preview",
        ),
        permission_mode(
            "read_only",
            false,
            false,
            "role may inspect local state without mutation",
        ),
        permission_mode(
            "write_scoped",
            true,
            true,
            "role may mutate only after scoped approval and admission checks",
        ),
        permission_mode(
            "approval_required",
            false,
            true,
            "role must pause for explicit operator approval before execution",
        ),
    ]
}

pub fn work_graph_role_manifest_invariants() -> Vec<WorkGraphRoleManifestInvariantPreview> {
    vec![
        invariant(
            "capability_requires_permission_mode",
            "every declared capability must map to an explicit tool permission mode",
        ),
        invariant(
            "mutation_requires_approval_and_lane",
            "runtime mutation authority cannot exist without approval policy and lane binding",
        ),
        invariant(
            "terminal_output_requires_schema_and_verifier",
            "roles that complete work must declare output schema and verifier references",
        ),
        invariant(
            "budget_and_concurrency_are_required",
            "roles cannot be admitted without bounded budget and concurrency limits",
        ),
        invariant(
            "trace_policy_is_required",
            "role outputs must be joinable to WorkGraph trace and redacted evidence",
        ),
        invariant(
            "preview_gate_does_not_change_permissions",
            "this preview cannot enable role enforcement or alter tool permissions",
        ),
    ]
}

pub fn work_graph_role_manifest_adapter_previews() -> Vec<WorkGraphRoleManifestAdapterPreview> {
    vec![
        adapter(
            "multi_agent_v2_thread_spawn",
            "agent_task_role",
            vec![
                "roleId",
                "agentPath",
                "capabilities",
                "toolPermissions",
                "budget",
                "concurrency",
                "lane",
                "tracePolicy",
            ],
            vec!["multi_agent_v2_role_manifest_not_enforced"],
        ),
        adapter(
            "agent_jobs_batch_workers",
            "batch_worker_role",
            vec![
                "roleId",
                "capabilities",
                "outputSchemaRef",
                "verifierRef",
                "budget",
                "tracePolicy",
            ],
            vec!["agent_jobs_role_manifest_not_enforced"],
        ),
        adapter(
            "hepta_runtime_worker_tasks",
            "runtime_worker_role",
            vec![
                "roleId",
                "toolPermissions",
                "outputSchemaRef",
                "verifierRef",
                "budget",
                "concurrency",
                "lane",
            ],
            vec!["worker_task_role_manifest_not_enforced"],
        ),
        adapter(
            "hepta_runtime_agent_harness",
            "external_handoff_role",
            vec![
                "roleId",
                "capabilities",
                "approvalPolicy",
                "verifierRef",
                "budget",
                "tracePolicy",
            ],
            vec!["agent_harness_role_manifest_not_enforced"],
        ),
    ]
}

impl WorkGraphRoleManifestContractPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            runtime_mutation_performed: false,
            scheduler_cutover_performed: false,
            role_enforcement_enabled: false,
            tool_permission_changed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn field(wire_name: &'static str, purpose: &'static str) -> WorkGraphRoleManifestFieldPreview {
    WorkGraphRoleManifestFieldPreview {
        wire_name,
        required: true,
        purpose,
    }
}

fn capability(
    id: &'static str,
    requires_tool_permission: bool,
    requires_verifier: bool,
    purpose: &'static str,
) -> WorkGraphRoleCapabilityPreview {
    WorkGraphRoleCapabilityPreview {
        id,
        requires_tool_permission,
        requires_verifier,
        purpose,
    }
}

fn permission_mode(
    id: &'static str,
    can_mutate_runtime: bool,
    requires_approval: bool,
    purpose: &'static str,
) -> WorkGraphRolePermissionModePreview {
    WorkGraphRolePermissionModePreview {
        id,
        can_mutate_runtime,
        requires_approval,
        purpose,
    }
}

fn invariant(id: &'static str, reason: &'static str) -> WorkGraphRoleManifestInvariantPreview {
    WorkGraphRoleManifestInvariantPreview {
        id,
        required: true,
        reason,
    }
}

fn adapter(
    source_surface_id: &'static str,
    projected_role_kind: &'static str,
    covered_wire_fields: Vec<&'static str>,
    blocker_ids: Vec<&'static str>,
) -> WorkGraphRoleManifestAdapterPreview {
    WorkGraphRoleManifestAdapterPreview {
        source_surface_id,
        projected_role_kind,
        covered_wire_fields,
        enforcement_enabled: false,
        blocker_ids,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_manifest_contract_declares_required_fields() {
        let report = hepta_work_graph_role_manifest_contract_preview_report();
        let field_ids = report
            .required_fields
            .iter()
            .map(|field| field.wire_name)
            .collect::<Vec<_>>();

        assert_eq!(
            field_ids,
            [
                "roleId",
                "roleKind",
                "agentPath",
                "capabilities",
                "toolPermissions",
                "outputSchemaRef",
                "verifierRef",
                "budget",
                "concurrency",
                "lane",
                "approvalPolicy",
                "tracePolicy",
            ]
        );
        assert_eq!(report.required_field_count, 12);
        assert!(report.required_fields.iter().all(|field| field.required));
    }

    #[test]
    fn role_manifest_contract_keeps_permission_enforcement_disabled() {
        let report = hepta_work_graph_role_manifest_contract_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphRoleManifestContractPreviewSideEffects::none()
        );
        assert!(report.ready_for_unified_state_store_preview);
        assert!(!report.ready_for_role_enforcement);
        assert!(!report.ready_for_live_execution);
        assert!(
            report
                .adapter_previews
                .iter()
                .all(|adapter| !adapter.enforcement_enabled)
        );
    }

    #[test]
    fn role_manifest_contract_names_capabilities_and_permission_modes() {
        let report = hepta_work_graph_role_manifest_contract_preview_report();
        let capability_ids = report
            .capabilities
            .iter()
            .map(|capability| capability.id)
            .collect::<Vec<_>>();
        let permission_ids = report
            .permission_modes
            .iter()
            .map(|mode| mode.id)
            .collect::<Vec<_>>();

        assert_eq!(
            capability_ids,
            [
                "planning",
                "agent_delegation",
                "code_editing",
                "verification",
                "research",
                "scheduler_control",
                "external_handoff_proposal",
            ]
        );
        assert_eq!(
            permission_ids,
            [
                "deny",
                "preview",
                "read_only",
                "write_scoped",
                "approval_required",
            ]
        );
        assert_eq!(report.capability_count, 7);
        assert_eq!(report.permission_mode_count, 5);
    }

    #[test]
    fn role_manifest_contract_projects_current_role_surfaces() {
        let report = hepta_work_graph_role_manifest_contract_preview_report();
        let adapter_ids = report
            .adapter_previews
            .iter()
            .map(|adapter| adapter.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            adapter_ids,
            [
                "multi_agent_v2_thread_spawn",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_agent_harness",
            ]
        );
        assert_eq!(report.adapter_preview_count, 4);
        assert_eq!(report.invariant_count, 6);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_ROLE_MANIFEST_CONTRACT_RECOMMENDED_NEXT_GATE
        );
    }
}

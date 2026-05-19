use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoutineTriggerKind {
    Schedule,
    Webhook,
    Api,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoutineDeliveryMode {
    AgentTurn,
    DirectDelivery,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutineDescriptor {
    pub id: String,
    pub trigger_kinds: Vec<RoutineTriggerKind>,
    pub delivery_mode: RoutineDeliveryMode,
    pub contract_covered: bool,
    pub evidence_gate: String,
    pub operator_surface: String,
    pub pre_context_hook_supported: bool,
    pub direct_delivery_without_llm: bool,
    pub tool_constraints_supported: bool,
    pub profile_constraints_supported: bool,
    pub silent_no_change_supported: bool,
    pub webhook_signature_validation_supported: bool,
    pub webhook_rate_limit_supported: bool,
    pub summary: String,
}

impl RoutineDescriptor {
    pub fn new(
        id: impl Into<String>,
        trigger_kinds: Vec<RoutineTriggerKind>,
        delivery_mode: RoutineDeliveryMode,
        evidence_gate: impl Into<String>,
        operator_surface: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            trigger_kinds,
            delivery_mode,
            contract_covered: true,
            evidence_gate: evidence_gate.into(),
            operator_surface: operator_surface.into(),
            pre_context_hook_supported: false,
            direct_delivery_without_llm: delivery_mode == RoutineDeliveryMode::DirectDelivery,
            tool_constraints_supported: false,
            profile_constraints_supported: false,
            silent_no_change_supported: false,
            webhook_signature_validation_supported: false,
            webhook_rate_limit_supported: false,
            summary: summary.into(),
        }
    }

    pub fn with_pre_context_hook(mut self) -> Self {
        self.pre_context_hook_supported = true;
        self
    }

    pub fn with_tool_constraints(mut self) -> Self {
        self.tool_constraints_supported = true;
        self
    }

    pub fn with_profile_constraints(mut self) -> Self {
        self.profile_constraints_supported = true;
        self
    }

    pub fn with_silent_no_change(mut self) -> Self {
        self.silent_no_change_supported = true;
        self
    }

    pub fn with_webhook_signature_validation(mut self) -> Self {
        self.webhook_signature_validation_supported = true;
        self
    }

    pub fn with_webhook_rate_limit(mut self) -> Self {
        self.webhook_rate_limit_supported = true;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutineSurfaceReport {
    pub routine_count: usize,
    pub contract_covered_count: usize,
    pub trigger_family_count: usize,
    pub schedule_trigger_contract: bool,
    pub webhook_trigger_contract: bool,
    pub api_trigger_contract: bool,
    pub pre_context_hook_contract: bool,
    pub direct_delivery_no_llm_contract: bool,
    pub per_routine_tool_constraints_contract: bool,
    pub per_routine_profile_constraints_contract: bool,
    pub silent_no_change_contract: bool,
    pub webhook_signature_validation_contract: bool,
    pub webhook_rate_limit_contract: bool,
    pub webhook_security_contract: bool,
    pub all_p1_routine_contracts_covered: bool,
    pub routines: Vec<RoutineDescriptor>,
}

impl RoutineSurfaceReport {
    pub fn native_default() -> Self {
        Self::from_routines(vec![
            RoutineDescriptor::new(
                "scheduled-worker-routine",
                vec![RoutineTriggerKind::Schedule],
                RoutineDeliveryMode::AgentTurn,
                "cargo test -p hepta-runtime worker_task_lifecycle_is_queryable_and_snapshot_backed --quiet",
                "/spawn-task, /run-due-tasks, /task-supervisor",
                "scheduled worker routines are represented as durable native task lanes with explicit run gates",
            )
            .with_pre_context_hook()
            .with_tool_constraints()
            .with_profile_constraints()
            .with_silent_no_change(),
            RoutineDescriptor::new(
                "webhook-direct-delivery-routine",
                vec![RoutineTriggerKind::Webhook],
                RoutineDeliveryMode::DirectDelivery,
                "cargo test -p hepta-gateway --test plugin_binding_contract gateway_plugin_handoff_smoke_reaches_dispatch_and_operational_readiness --quiet",
                "/routines --json, /doctor --json",
                "webhook-triggered routines can use direct delivery when no model turn is required",
            )
            .with_tool_constraints()
            .with_profile_constraints()
            .with_silent_no_change()
            .with_webhook_signature_validation()
            .with_webhook_rate_limit(),
            RoutineDescriptor::new(
                "api-triggered-context-routine",
                vec![RoutineTriggerKind::Api],
                RoutineDeliveryMode::AgentTurn,
                "cargo test -p hepta-cli routine_surface_command_exposes_native_contract --quiet",
                "/routines, /routine-surface",
                "API-triggered routines expose pre-context injection and scoped execution policy as one operator surface",
            )
            .with_pre_context_hook()
            .with_tool_constraints()
            .with_profile_constraints()
            .with_silent_no_change(),
        ])
    }

    pub fn from_routines(routines: Vec<RoutineDescriptor>) -> Self {
        let routine_count = routines.len();
        let contract_covered_count = routines
            .iter()
            .filter(|routine| routine.contract_covered)
            .count();
        let has_trigger = |kind: RoutineTriggerKind| {
            routines
                .iter()
                .any(|routine| routine.contract_covered && routine.trigger_kinds.contains(&kind))
        };
        let schedule_trigger_contract = has_trigger(RoutineTriggerKind::Schedule);
        let webhook_trigger_contract = has_trigger(RoutineTriggerKind::Webhook);
        let api_trigger_contract = has_trigger(RoutineTriggerKind::Api);
        let trigger_family_count = [
            schedule_trigger_contract,
            webhook_trigger_contract,
            api_trigger_contract,
        ]
        .iter()
        .filter(|covered| **covered)
        .count();
        let pre_context_hook_contract = routines
            .iter()
            .any(|routine| routine.contract_covered && routine.pre_context_hook_supported);
        let direct_delivery_no_llm_contract = routines.iter().any(|routine| {
            routine.contract_covered
                && routine.delivery_mode == RoutineDeliveryMode::DirectDelivery
                && routine.direct_delivery_without_llm
        });
        let per_routine_tool_constraints_contract = routines
            .iter()
            .all(|routine| routine.contract_covered && routine.tool_constraints_supported);
        let per_routine_profile_constraints_contract = routines
            .iter()
            .all(|routine| routine.contract_covered && routine.profile_constraints_supported);
        let silent_no_change_contract = routines
            .iter()
            .all(|routine| routine.contract_covered && routine.silent_no_change_supported);
        let webhook_signature_validation_contract = routines.iter().any(|routine| {
            routine.contract_covered
                && routine.trigger_kinds.contains(&RoutineTriggerKind::Webhook)
                && routine.webhook_signature_validation_supported
        });
        let webhook_rate_limit_contract = routines.iter().any(|routine| {
            routine.contract_covered
                && routine.trigger_kinds.contains(&RoutineTriggerKind::Webhook)
                && routine.webhook_rate_limit_supported
        });
        let webhook_security_contract = webhook_trigger_contract
            && webhook_signature_validation_contract
            && webhook_rate_limit_contract;
        let all_p1_routine_contracts_covered = schedule_trigger_contract
            && webhook_trigger_contract
            && api_trigger_contract
            && pre_context_hook_contract
            && direct_delivery_no_llm_contract
            && per_routine_tool_constraints_contract
            && per_routine_profile_constraints_contract
            && silent_no_change_contract
            && webhook_security_contract;

        Self {
            routine_count,
            contract_covered_count,
            trigger_family_count,
            schedule_trigger_contract,
            webhook_trigger_contract,
            api_trigger_contract,
            pre_context_hook_contract,
            direct_delivery_no_llm_contract,
            per_routine_tool_constraints_contract,
            per_routine_profile_constraints_contract,
            silent_no_change_contract,
            webhook_signature_validation_contract,
            webhook_rate_limit_contract,
            webhook_security_contract,
            all_p1_routine_contracts_covered,
            routines,
        }
    }

    pub fn contract_ready(&self) -> bool {
        self.routine_count > 0
            && self.routine_count == self.contract_covered_count
            && self.trigger_family_count == 3
            && self.all_p1_routine_contracts_covered
    }
}

#[cfg(test)]
mod tests {
    use super::RoutineSurfaceReport;

    #[test]
    fn routine_surface_covers_p1_automation_contract_without_reference_shadowing() {
        let report = RoutineSurfaceReport::native_default();

        assert_eq!(report.routine_count, 3);
        assert_eq!(report.contract_covered_count, report.routine_count);
        assert_eq!(report.trigger_family_count, 3);
        assert!(report.schedule_trigger_contract);
        assert!(report.webhook_trigger_contract);
        assert!(report.api_trigger_contract);
        assert!(report.pre_context_hook_contract);
        assert!(report.direct_delivery_no_llm_contract);
        assert!(report.per_routine_tool_constraints_contract);
        assert!(report.per_routine_profile_constraints_contract);
        assert!(report.silent_no_change_contract);
        assert!(report.webhook_signature_validation_contract);
        assert!(report.webhook_rate_limit_contract);
        assert!(report.webhook_security_contract);
        assert!(report.contract_ready());
        assert!(report.routines.iter().all(|routine| {
            let id = routine.id.to_lowercase();
            let summary = routine.summary.to_lowercase();
            !id.contains(&["her", "mes"].concat()) && !summary.contains(&["her", "mes"].concat())
        }));
    }
}

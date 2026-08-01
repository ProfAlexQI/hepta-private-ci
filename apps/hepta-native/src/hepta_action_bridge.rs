//! Local action-bridge policy for Hepta Native.
//!
//! This module is deliberately side-effect free. It describes whether a planned
//! UI action may stay local, produce a read-only/draft preview, or must remain
//! blocked until later confirmation/runtime wiring exists.

use serde_json::{json, Value};

pub const MUTATION_LOCAL_UI_STATE: &str = "local_ui_state";
pub const MUTATION_READ_ONLY_RUNTIME_COMMAND: &str = "read_only_runtime_command";
pub const MUTATION_DRAFT_TASK_PLAN: &str = "draft_task_plan";
pub const MUTATION_DRAFT_AGENT_INSTRUCTION: &str = "draft_agent_instruction";
pub const MUTATION_DRAFT_TOOL_CALL: &str = "draft_tool_call";
pub const MUTATION_DRAFT_APPROVAL_DECISION: &str = "draft_approval_decision";
pub const MUTATION_SEND_CURRENT_SESSION_MESSAGE: &str = "send_current_session_message";
pub const MUTATION_SUBAGENT_CONTROL: &str = "subagent_control";
pub const MUTATION_APPROVE_TOOL_EXEC: &str = "approve_tool_exec";
pub const MUTATION_TASK_REGISTRY_UPDATE: &str = "task_registry_update";
pub const MUTATION_GATEWAY_CONFIG: &str = "gateway_config_mutation";

pub const CONTROLLED_MUTATION_ORDER: &[&str] = &[
    MUTATION_LOCAL_UI_STATE,
    MUTATION_READ_ONLY_RUNTIME_COMMAND,
    MUTATION_DRAFT_TASK_PLAN,
    MUTATION_SEND_CURRENT_SESSION_MESSAGE,
    MUTATION_SUBAGENT_CONTROL,
    MUTATION_APPROVE_TOOL_EXEC,
    MUTATION_TASK_REGISTRY_UPDATE,
    MUTATION_GATEWAY_CONFIG,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeptaActionDisposition {
    LocalOnly,
    ReadOnlyPreview,
    DraftPreview,
    BlockedUntilConfirmation,
    BlockedUntilPolicyGate,
}

impl HeptaActionDisposition {
    pub fn label(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ReadOnlyPreview => "read_only_preview",
            Self::DraftPreview => "draft_preview",
            Self::BlockedUntilConfirmation => "blocked_until_confirmation",
            Self::BlockedUntilPolicyGate => "blocked_until_policy_gate",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaActionBridgeDecision {
    pub mutation_class: String,
    pub disposition: HeptaActionDisposition,
    pub requires_confirmation: bool,
    pub external_mutation_enabled: bool,
    pub confirmed: bool,
    pub exact_payload_preview_required: bool,
    pub result_readback_required: bool,
    pub redacted_evidence_required: bool,
    pub reason: &'static str,
}

impl HeptaActionBridgeDecision {
    pub fn preview_only(mutation_class: &str, requires_confirmation: bool) -> Self {
        decide_hepta_action(HeptaActionBridgeRequest {
            mutation_class,
            requires_confirmation,
            external_mutation_enabled: false,
            confirmed: false,
        })
    }

    pub fn as_payload_value(&self) -> Value {
        json!({
            "mutation_class": self.mutation_class,
            "disposition": self.disposition.label(),
            "requires_confirmation": self.requires_confirmation,
            "external_mutation_enabled": self.external_mutation_enabled,
            "confirmed": self.confirmed,
            "exact_payload_preview_required": self.exact_payload_preview_required,
            "result_readback_required": self.result_readback_required,
            "redacted_evidence_required": self.redacted_evidence_required,
            "reason": self.reason,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeptaActionBridgeRequest<'a> {
    pub mutation_class: &'a str,
    pub requires_confirmation: bool,
    pub external_mutation_enabled: bool,
    pub confirmed: bool,
}

pub fn decide_hepta_action(request: HeptaActionBridgeRequest<'_>) -> HeptaActionBridgeDecision {
    let disposition = if !request.external_mutation_enabled {
        preview_disposition(request.mutation_class)
    } else if request.requires_confirmation && !request.confirmed {
        HeptaActionDisposition::BlockedUntilConfirmation
    } else if mutation_class_is_currently_gateable(request.mutation_class) {
        preview_disposition(request.mutation_class)
    } else {
        HeptaActionDisposition::BlockedUntilPolicyGate
    };

    HeptaActionBridgeDecision {
        mutation_class: request.mutation_class.to_string(),
        disposition,
        requires_confirmation: request.requires_confirmation,
        external_mutation_enabled: request.external_mutation_enabled,
        confirmed: request.confirmed,
        exact_payload_preview_required: request.requires_confirmation
            || !matches!(disposition, HeptaActionDisposition::LocalOnly),
        result_readback_required: request.external_mutation_enabled,
        redacted_evidence_required: true,
        reason: reason_for_disposition(disposition),
    }
}

fn preview_disposition(mutation_class: &str) -> HeptaActionDisposition {
    match mutation_class {
        MUTATION_LOCAL_UI_STATE => HeptaActionDisposition::LocalOnly,
        MUTATION_READ_ONLY_RUNTIME_COMMAND => HeptaActionDisposition::ReadOnlyPreview,
        MUTATION_DRAFT_TASK_PLAN
        | MUTATION_DRAFT_AGENT_INSTRUCTION
        | MUTATION_DRAFT_TOOL_CALL
        | MUTATION_DRAFT_APPROVAL_DECISION => HeptaActionDisposition::DraftPreview,
        _ => HeptaActionDisposition::BlockedUntilPolicyGate,
    }
}

fn mutation_class_is_currently_gateable(mutation_class: &str) -> bool {
    matches!(
        mutation_class,
        MUTATION_LOCAL_UI_STATE | MUTATION_READ_ONLY_RUNTIME_COMMAND | MUTATION_DRAFT_TASK_PLAN
    )
}

fn reason_for_disposition(disposition: HeptaActionDisposition) -> &'static str {
    match disposition {
        HeptaActionDisposition::LocalOnly => "local UI-only state is safe without runtime mutation",
        HeptaActionDisposition::ReadOnlyPreview => {
            "read-only runtime command may be previewed locally before live runtime wiring"
        }
        HeptaActionDisposition::DraftPreview => {
            "draft action is staged as a preview event; no external mutation is enabled"
        }
        HeptaActionDisposition::BlockedUntilConfirmation => {
            "mutation requires explicit payload confirmation before execution"
        }
        HeptaActionDisposition::BlockedUntilPolicyGate => {
            "mutation class is not enabled by the current Hepta Native policy gate"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_policy_keeps_draft_actions_local() {
        let decision = HeptaActionBridgeDecision::preview_only(MUTATION_DRAFT_TASK_PLAN, true);
        assert_eq!(decision.disposition, HeptaActionDisposition::DraftPreview);
        assert!(!decision.external_mutation_enabled);
        assert!(decision.exact_payload_preview_required);
        assert!(!decision.result_readback_required);
        assert!(decision.redacted_evidence_required);
    }

    #[test]
    fn read_only_status_can_be_previewed_without_confirmation() {
        let decision =
            HeptaActionBridgeDecision::preview_only(MUTATION_READ_ONLY_RUNTIME_COMMAND, false);
        assert_eq!(
            decision.disposition,
            HeptaActionDisposition::ReadOnlyPreview
        );
        assert!(!decision.requires_confirmation);
    }

    #[test]
    fn external_mutation_requires_confirmation_before_any_execution_gate() {
        let decision = decide_hepta_action(HeptaActionBridgeRequest {
            mutation_class: MUTATION_APPROVE_TOOL_EXEC,
            requires_confirmation: true,
            external_mutation_enabled: true,
            confirmed: false,
        });
        assert_eq!(
            decision.disposition,
            HeptaActionDisposition::BlockedUntilConfirmation
        );
        assert!(decision.result_readback_required);
    }

    #[test]
    fn approval_execution_is_policy_blocked_until_phase_gate() {
        let decision = decide_hepta_action(HeptaActionBridgeRequest {
            mutation_class: MUTATION_APPROVE_TOOL_EXEC,
            requires_confirmation: true,
            external_mutation_enabled: false,
            confirmed: false,
        });
        assert_eq!(
            decision.disposition,
            HeptaActionDisposition::BlockedUntilPolicyGate
        );
        assert!(decision.exact_payload_preview_required);
        assert_eq!(decision.mutation_class, MUTATION_APPROVE_TOOL_EXEC);
    }

    #[test]
    fn gateway_config_is_last_and_policy_blocked() {
        assert_eq!(
            CONTROLLED_MUTATION_ORDER.last(),
            Some(&MUTATION_GATEWAY_CONFIG)
        );
        let decision = decide_hepta_action(HeptaActionBridgeRequest {
            mutation_class: MUTATION_GATEWAY_CONFIG,
            requires_confirmation: true,
            external_mutation_enabled: true,
            confirmed: true,
        });
        assert_eq!(
            decision.disposition,
            HeptaActionDisposition::BlockedUntilPolicyGate
        );
    }
}

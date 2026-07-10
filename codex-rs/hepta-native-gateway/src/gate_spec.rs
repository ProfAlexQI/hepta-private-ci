use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub(crate) struct GateSpec {
    pub(crate) method: &'static str,
    pub(crate) pattern: &'static str,
    pub(crate) source_command: &'static str,
    pub(crate) capability: &'static str,
    pub(crate) side_effect_boundary: &'static str,
}

impl GateSpec {
    pub(crate) fn is_post(&self) -> bool {
        self.method == "POST"
    }

    pub(crate) fn is_read_only(&self) -> bool {
        self.side_effect_boundary.contains("read-only")
    }

    pub(crate) fn is_dry_run(&self) -> bool {
        self.side_effect_boundary.contains("dry-run")
            || self.side_effect_boundary.contains("plan only")
    }

    pub(crate) fn is_guarded(&self) -> bool {
        let boundary = self.side_effect_boundary;
        self.is_dry_run()
            || boundary.contains("not executed")
            || boundary.contains("confirm-required")
            || boundary.contains("requires confirmation")
            || boundary.contains("never mutates")
            || boundary.contains("never publishes")
            || boundary.contains("never sends")
    }

    pub(crate) fn requires_confirmation(&self) -> bool {
        self.is_post() && !self.is_read_only()
    }

    pub(crate) fn receipt_state(&self) -> Option<ReceiptState> {
        ReceiptStateMachine::classify(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReceiptState {
    Precondition,
    Denial,
    Receipt,
    Persistence,
    Retention,
    Terminal,
}

impl ReceiptState {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Precondition => "precondition",
            Self::Denial => "denial",
            Self::Receipt => "receipt",
            Self::Persistence => "persistence",
            Self::Retention => "retention",
            Self::Terminal => "terminal",
        }
    }
}

pub(crate) struct ReceiptStateMachine;

impl ReceiptStateMachine {
    pub(crate) const ORDERED_STATES: [ReceiptState; 6] = [
        ReceiptState::Precondition,
        ReceiptState::Denial,
        ReceiptState::Receipt,
        ReceiptState::Persistence,
        ReceiptState::Retention,
        ReceiptState::Terminal,
    ];

    pub(crate) fn classify(spec: &GateSpec) -> Option<ReceiptState> {
        Self::classify_fields(
            spec.capability,
            spec.source_command,
            spec.side_effect_boundary,
        )
    }

    pub(crate) fn classify_fields(
        capability: &str,
        source_command: &str,
        side_effect_boundary: &str,
    ) -> Option<ReceiptState> {
        let fields = [capability, source_command, side_effect_boundary];
        let contains = |needle: &str| fields.iter().any(|field| field.contains(needle));

        if contains("terminal") {
            Some(ReceiptState::Terminal)
        } else if contains("retention") || contains("expiry") || contains("garbage-collection") {
            Some(ReceiptState::Retention)
        } else if contains("persistence") || contains("persist") {
            Some(ReceiptState::Persistence)
        } else if contains("receipt") {
            Some(ReceiptState::Receipt)
        } else if contains("denial") || contains("denied") || contains("blocked") {
            Some(ReceiptState::Denial)
        } else if contains("precondition") || contains("readiness") || contains("approval-packet") {
            Some(ReceiptState::Precondition)
        } else {
            None
        }
    }

    pub(crate) fn contains_label(label: &str) -> bool {
        Self::ORDERED_STATES
            .iter()
            .any(|state| state.as_str() == label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(capability: &'static str, boundary: &'static str) -> GateSpec {
        GateSpec {
            method: "GET",
            pattern: "/api/test",
            source_command: "/test --json",
            capability,
            side_effect_boundary: boundary,
        }
    }

    #[test]
    fn gate_spec_centralizes_route_guard_semantics() {
        let dry_run = GateSpec {
            method: "POST",
            pattern: "/api/actions/test",
            source_command: "/actions test --dry-run --json",
            capability: "test-action",
            side_effect_boundary: "dry-run plan only; no mutation",
        };

        assert!(dry_run.is_post());
        assert!(dry_run.is_dry_run());
        assert!(dry_run.is_guarded());
        assert!(dry_run.requires_confirmation());
        assert!(!dry_run.is_read_only());
    }

    #[test]
    fn receipt_state_machine_prefers_the_most_advanced_declared_state() {
        assert_eq!(
            spec("packet-precondition", "read-only precondition").receipt_state(),
            Some(ReceiptState::Precondition)
        );
        assert_eq!(
            spec("result-receipt-denial", "read-only denial").receipt_state(),
            Some(ReceiptState::Receipt)
        );
        assert_eq!(
            spec(
                "result-receipt-retention-expiry-denial",
                "read-only receipt retention denial"
            )
            .receipt_state(),
            Some(ReceiptState::Retention)
        );
        assert_eq!(
            spec(
                "result-receipt-terminal-decision-denial",
                "read-only receipt terminal denial"
            )
            .receipt_state(),
            Some(ReceiptState::Terminal)
        );
        assert_eq!(ReceiptStateMachine::ORDERED_STATES.len(), 6);
        assert!(ReceiptStateMachine::contains_label("terminal"));
        assert!(!ReceiptStateMachine::contains_label("unknown"));
    }
}

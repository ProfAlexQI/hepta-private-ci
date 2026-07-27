use serde::Serialize;

const MINIMAL_SCOPED_MEMORY_EFFECT_ROUTE_PREFIX: &str = "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-";
const PRODUCTION_MEMORY_EFFECT_ROUTE_PREFIX: &str = "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-";

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

    pub(crate) fn is_quarantined_transitive_effect(&self) -> bool {
        if self.method != "GET" {
            return false;
        }
        if self
            .pattern
            .starts_with(PRODUCTION_MEMORY_EFFECT_ROUTE_PREFIX)
        {
            return true;
        }
        let Some(suffix) = self
            .pattern
            .strip_prefix(MINIMAL_SCOPED_MEMORY_EFFECT_ROUTE_PREFIX)
        else {
            return false;
        };
        !matches!(
            suffix,
            "operator-approval-nonce-command-accepted-gate-boundary"
                | "wal-receipt-binding-boundary"
                | "post-write-readback-binding-boundary"
                | "rollback-tombstone-proof-boundary"
        )
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
#[path = "../tests/unit/gate_spec.rs"]
mod tests;

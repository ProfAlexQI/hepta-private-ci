use codex_hepta_types::Digest32;
use codex_hepta_types::Generation;
use codex_hepta_types::Revision;
use codex_hepta_types::StableId;

/// Stable operation identity plus the exact final payload digest.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OperationKey {
    pub id: StableId,
    pub payload_digest: Digest32,
}

/// Short-lived, independently issued witness consumed before adapter entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityWitness {
    pub operation_id: StableId,
    pub final_payload_digest: Digest32,
    pub authority_generation: Generation,
    pub expires_at_unix_ms: u64,
    pub witness_digest: Digest32,
}

impl AuthorityWitness {
    pub fn validates(&self, key: &OperationKey, now_unix_ms: u64) -> bool {
        self.operation_id == key.id
            && self.final_payload_digest == key.payload_digest
            && now_unix_ms < self.expires_at_unix_ms
            && !self.witness_digest.is_zero()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReconciliationOutcome {
    Applied,
    NotApplied,
    Quarantined,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperationState {
    Pending,
    Authorized {
        witness_digest: Digest32,
        authority_generation: Generation,
    },
    Dispatched {
        dispatch_digest: Digest32,
    },
    Indeterminate {
        reason_digest: Digest32,
    },
    Applied {
        outcome_digest: Digest32,
    },
    NotApplied {
        outcome_digest: Digest32,
    },
    Quarantined {
        reason_digest: Digest32,
    },
}

impl OperationState {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Authorized { .. } => "authorized",
            Self::Dispatched { .. } => "dispatched",
            Self::Indeterminate { .. } => "indeterminate",
            Self::Applied { .. } => "applied",
            Self::NotApplied { .. } => "not_applied",
            Self::Quarantined { .. } => "quarantined",
        }
    }

    pub const fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Applied { .. } | Self::NotApplied { .. } | Self::Quarantined { .. }
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationRecord {
    pub key: OperationKey,
    pub owner_generation: Generation,
    pub revision: Revision,
    pub state: OperationState,
}

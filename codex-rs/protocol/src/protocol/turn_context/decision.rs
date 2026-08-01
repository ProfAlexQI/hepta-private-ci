use super::common::TURN_CONTEXT_DECISION_SCHEMA_VERSION;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextDecisionEntry {
    pub source: String,
    pub decision: String,
    /// Stable 16-hex replay identity for the local decision reason. This is not
    /// a cryptographic trust digest and must not be used for approval integrity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub reason_hash: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TurnContextDecisionKind {
    Included {
        policy_class: String,
    },
    Policy {
        strategy: String,
        budget_state: String,
    },
    CandidateOmit {
        source_id: String,
        priority: u32,
        tokens: u32,
    },
    CandidateTruncate {
        source_id: String,
        remaining_over_budget: u32,
        tokens: u32,
    },
    Omitted {
        source_id: String,
        priority: u32,
        tokens: u32,
    },
    Truncated {
        source_id: String,
        original_tokens: u32,
        tokens: u32,
    },
    Unknown {
        raw: String,
    },
}

impl TurnContextDecisionKind {
    pub fn schema_version(&self) -> Option<u32> {
        self.is_known()
            .then_some(TURN_CONTEXT_DECISION_SCHEMA_VERSION)
    }

    pub fn to_legacy_decision_string(&self) -> String {
        match self {
            Self::Included { policy_class } => format!("included:{policy_class}"),
            Self::Policy {
                strategy,
                budget_state,
            } => format!("policy:{strategy}:{budget_state}"),
            Self::CandidateOmit {
                source_id,
                priority,
                tokens,
            } => format!("candidate_omit:{source_id}:priority:{priority}:tokens:{tokens}"),
            Self::CandidateTruncate {
                source_id,
                remaining_over_budget,
                tokens,
            } => format!(
                "candidate_truncate:{source_id}:remaining_over_budget:{remaining_over_budget}:tokens:{tokens}"
            ),
            Self::Omitted {
                source_id,
                priority,
                tokens,
            } => format!("omitted:{source_id}:priority:{priority}:tokens:{tokens}"),
            Self::Truncated {
                source_id,
                original_tokens,
                tokens,
            } => format!("truncated:{source_id}:original_tokens:{original_tokens}:tokens:{tokens}"),
            Self::Unknown { raw } => raw.clone(),
        }
    }

    pub fn is_truncation(&self) -> bool {
        matches!(self, Self::Truncated { .. })
    }

    pub fn is_candidate_truncation(&self) -> bool {
        matches!(self, Self::CandidateTruncate { .. })
    }

    pub fn is_known(&self) -> bool {
        !matches!(self, Self::Unknown { .. })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TurnContextDecisionLedgerSummary {
    pub schema_version: u32,
    pub included_count: u32,
    pub policy_count: u32,
    pub candidate_omit_count: u32,
    pub candidate_truncate_count: u32,
    pub omitted_count: u32,
    pub truncated_count: u32,
    pub unknown_count: u32,
}

impl TurnContextDecisionLedgerSummary {
    pub fn known_count(&self) -> u32 {
        self.included_count
            .saturating_add(self.policy_count)
            .saturating_add(self.candidate_omit_count)
            .saturating_add(self.candidate_truncate_count)
            .saturating_add(self.omitted_count)
            .saturating_add(self.truncated_count)
    }
}

impl TurnContextDecisionEntry {
    pub fn from_kind(
        source: impl Into<String>,
        kind: TurnContextDecisionKind,
        reason_hash: Option<String>,
    ) -> Self {
        Self {
            source: source.into(),
            decision: kind.to_legacy_decision_string(),
            reason_hash,
        }
    }

    pub fn included(
        source: impl Into<String>,
        policy_class: impl Into<String>,
        reason_hash: Option<String>,
    ) -> Self {
        Self::from_kind(
            source,
            TurnContextDecisionKind::Included {
                policy_class: policy_class.into(),
            },
            reason_hash,
        )
    }

    pub fn policy(
        source: impl Into<String>,
        strategy: impl Into<String>,
        budget_state: impl Into<String>,
        reason_hash: Option<String>,
    ) -> Self {
        Self::from_kind(
            source,
            TurnContextDecisionKind::Policy {
                strategy: strategy.into(),
                budget_state: budget_state.into(),
            },
            reason_hash,
        )
    }

    pub fn candidate_omit(
        source: impl Into<String>,
        source_id: impl Into<String>,
        priority: u32,
        tokens: u32,
        reason_hash: Option<String>,
    ) -> Self {
        Self::from_kind(
            source,
            TurnContextDecisionKind::CandidateOmit {
                source_id: source_id.into(),
                priority,
                tokens,
            },
            reason_hash,
        )
    }

    pub fn candidate_truncate(
        source: impl Into<String>,
        source_id: impl Into<String>,
        remaining_over_budget: u32,
        tokens: u32,
        reason_hash: Option<String>,
    ) -> Self {
        Self::from_kind(
            source,
            TurnContextDecisionKind::CandidateTruncate {
                source_id: source_id.into(),
                remaining_over_budget,
                tokens,
            },
            reason_hash,
        )
    }

    pub fn omitted(
        source: impl Into<String>,
        source_id: impl Into<String>,
        priority: u32,
        tokens: u32,
        reason_hash: Option<String>,
    ) -> Self {
        Self::from_kind(
            source,
            TurnContextDecisionKind::Omitted {
                source_id: source_id.into(),
                priority,
                tokens,
            },
            reason_hash,
        )
    }

    pub fn truncated(
        source: impl Into<String>,
        source_id: impl Into<String>,
        original_tokens: u32,
        tokens: u32,
        reason_hash: Option<String>,
    ) -> Self {
        Self::from_kind(
            source,
            TurnContextDecisionKind::Truncated {
                source_id: source_id.into(),
                original_tokens,
                tokens,
            },
            reason_hash,
        )
    }

    pub fn kind(&self) -> TurnContextDecisionKind {
        parse_turn_context_decision_kind(&self.decision).unwrap_or_else(|| {
            TurnContextDecisionKind::Unknown {
                raw: self.decision.clone(),
            }
        })
    }
}

pub fn summarize_turn_context_decision_ledger(
    entries: &[TurnContextDecisionEntry],
) -> TurnContextDecisionLedgerSummary {
    let mut summary = TurnContextDecisionLedgerSummary::default();
    for entry in entries {
        match entry.kind() {
            TurnContextDecisionKind::Included { .. } => {
                summary.included_count = summary.included_count.saturating_add(1);
            }
            TurnContextDecisionKind::Policy { .. } => {
                summary.policy_count = summary.policy_count.saturating_add(1);
            }
            TurnContextDecisionKind::CandidateOmit { .. } => {
                summary.candidate_omit_count = summary.candidate_omit_count.saturating_add(1);
            }
            TurnContextDecisionKind::CandidateTruncate { .. } => {
                summary.candidate_truncate_count =
                    summary.candidate_truncate_count.saturating_add(1);
            }
            TurnContextDecisionKind::Omitted { .. } => {
                summary.omitted_count = summary.omitted_count.saturating_add(1);
            }
            TurnContextDecisionKind::Truncated { .. } => {
                summary.truncated_count = summary.truncated_count.saturating_add(1);
            }
            TurnContextDecisionKind::Unknown { .. } => {
                summary.unknown_count = summary.unknown_count.saturating_add(1);
            }
        }
    }
    if summary.known_count() > 0 {
        summary.schema_version = TURN_CONTEXT_DECISION_SCHEMA_VERSION;
    }
    summary
}

fn parse_turn_context_decision_kind(decision: &str) -> Option<TurnContextDecisionKind> {
    let parts = decision.split(':').collect::<Vec<_>>();
    match parts.as_slice() {
        ["included", policy_class] if !policy_class.is_empty() => {
            Some(TurnContextDecisionKind::Included {
                policy_class: (*policy_class).to_string(),
            })
        }
        ["policy", strategy, budget_state] if !strategy.is_empty() && !budget_state.is_empty() => {
            Some(TurnContextDecisionKind::Policy {
                strategy: (*strategy).to_string(),
                budget_state: (*budget_state).to_string(),
            })
        }
        [
            "candidate_omit",
            source_id,
            "priority",
            priority,
            "tokens",
            tokens,
        ] if !source_id.is_empty() => Some(TurnContextDecisionKind::CandidateOmit {
            source_id: (*source_id).to_string(),
            priority: priority.parse().ok()?,
            tokens: tokens.parse().ok()?,
        }),
        [
            "candidate_truncate",
            source_id,
            "remaining_over_budget",
            remaining_over_budget,
            "tokens",
            tokens,
        ] if !source_id.is_empty() => Some(TurnContextDecisionKind::CandidateTruncate {
            source_id: (*source_id).to_string(),
            remaining_over_budget: remaining_over_budget.parse().ok()?,
            tokens: tokens.parse().ok()?,
        }),
        ["omitted", source_id, "priority", priority, "tokens", tokens] if !source_id.is_empty() => {
            Some(TurnContextDecisionKind::Omitted {
                source_id: (*source_id).to_string(),
                priority: priority.parse().ok()?,
                tokens: tokens.parse().ok()?,
            })
        }
        [
            "truncated",
            source_id,
            "original_tokens",
            original_tokens,
            "tokens",
            tokens,
        ] if !source_id.is_empty() => Some(TurnContextDecisionKind::Truncated {
            source_id: (*source_id).to_string(),
            original_tokens: original_tokens.parse().ok()?,
            tokens: tokens.parse().ok()?,
        }),
        _ => None,
    }
}

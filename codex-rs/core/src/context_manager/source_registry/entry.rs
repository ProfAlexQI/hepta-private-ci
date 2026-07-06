use codex_protocol::protocol::TurnContextCompressionStageKind;
use codex_protocol::protocol::TurnContextTier;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContextSourcePrivacyClass {
    PromptVisible,
    BoundedRecallPayload,
}

impl ContextSourcePrivacyClass {
    #[cfg(test)]
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::PromptVisible => "prompt_visible",
            Self::BoundedRecallPayload => "bounded_recall_payload",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContextSourceBudgetClass {
    ToolInventory,
    ToolCapability,
    BoundedRecall,
    ProtectedDeveloper,
    FallbackContext,
    ProtectedUser,
    RuntimeState,
    NonTextContext,
    ProtectedSystem,
    SessionState,
}

impl ContextSourceBudgetClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::ToolInventory => "tool_inventory",
            Self::ToolCapability => "tool_capability",
            Self::BoundedRecall => "bounded_recall",
            Self::ProtectedDeveloper => "protected_developer",
            Self::FallbackContext => "fallback_context",
            Self::ProtectedUser => "protected_user",
            Self::RuntimeState => "runtime_state",
            Self::NonTextContext => "non_text_context",
            Self::ProtectedSystem => "protected_system",
            Self::SessionState => "session_state",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContextSourceTtl {
    Turn,
    Session,
}

impl ContextSourceTtl {
    #[cfg(test)]
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Turn => "turn",
            Self::Session => "session",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContextSourceVolatility {
    Low,
    Medium,
    High,
}

impl ContextSourceVolatility {
    #[cfg(test)]
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContextSourceTrustClass {
    SystemOwned,
    DeveloperOwned,
    UserOwned,
    ExtensionOwned,
    RuntimeObserved,
    RetrievedMemory,
}

impl ContextSourceTrustClass {
    #[cfg(test)]
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::SystemOwned => "system_owned",
            Self::DeveloperOwned => "developer_owned",
            Self::UserOwned => "user_owned",
            Self::ExtensionOwned => "extension_owned",
            Self::RuntimeObserved => "runtime_observed",
            Self::RetrievedMemory => "retrieved_memory",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContextSourceRedactionPolicy {
    PromptHashOnly,
    GuardedEnvelope,
    MetadataOnly,
}

impl ContextSourceRedactionPolicy {
    #[cfg(test)]
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::PromptHashOnly => "prompt_hash_only",
            Self::GuardedEnvelope => "guarded_envelope",
            Self::MetadataOnly => "metadata_only",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContextSourceQualityMetric {
    Presence,
    Freshness,
    InventoryDigest,
    ExtensionDigest,
    RecallQuality,
    PolicyDigest,
}

impl ContextSourceQualityMetric {
    #[cfg(test)]
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Presence => "presence",
            Self::Freshness => "freshness",
            Self::InventoryDigest => "inventory_digest",
            Self::ExtensionDigest => "extension_digest",
            Self::RecallQuality => "recall_quality",
            Self::PolicyDigest => "policy_digest",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContextSourceActivationGuard {
    Protected,
    CandidateOnly,
    OperatorApprovalRequired,
}

impl ContextSourceActivationGuard {
    #[cfg(test)]
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Protected => "protected",
            Self::CandidateOnly => "candidate_only",
            Self::OperatorApprovalRequired => "operator_approval_required",
        }
    }

    #[cfg(test)]
    pub(super) fn allows_live_activation(self) -> bool {
        match self {
            Self::Protected | Self::CandidateOnly | Self::OperatorApprovalRequired => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContextSourceRollbackPolicy {
    RebuildFromSource,
    RestorePrevious,
    DropTurnFragment,
    RerunRecall,
    NotMutable,
}

impl ContextSourceRollbackPolicy {
    #[cfg(test)]
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::RebuildFromSource => "rebuild_from_source",
            Self::RestorePrevious => "restore_previous",
            Self::DropTurnFragment => "drop_turn_fragment",
            Self::RerunRecall => "rerun_recall",
            Self::NotMutable => "not_mutable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ContextSourceRegistryEntry {
    pub(crate) source_id: &'static str,
    pub(crate) tier: TurnContextTier,
    pub(crate) owner_lane: &'static str,
    pub(crate) privacy_class: ContextSourcePrivacyClass,
    pub(crate) budget_class: ContextSourceBudgetClass,
    pub(crate) ttl: ContextSourceTtl,
    pub(crate) volatility: ContextSourceVolatility,
    pub(crate) trust_class: ContextSourceTrustClass,
    pub(crate) redaction_policy: ContextSourceRedactionPolicy,
    pub(crate) quality_metric: ContextSourceQualityMetric,
    pub(crate) activation_guard: ContextSourceActivationGuard,
    pub(crate) rollback_policy: ContextSourceRollbackPolicy,
    pub(crate) omit_priority: Option<u8>,
    pub(super) allowed_compression_actions: &'static [TurnContextCompressionStageKind],
}

impl ContextSourceRegistryEntry {
    pub(crate) fn default_compression_kind(self) -> Option<TurnContextCompressionStageKind> {
        self.allowed_compression_actions.first().copied()
    }

    #[cfg(test)]
    pub(super) fn as_tsv_row(self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.source_id,
            self.tier.as_str(),
            self.owner_lane,
            self.privacy_class.as_str(),
            self.budget_class.as_str(),
            self.ttl.as_str(),
            self.volatility.as_str(),
            self.trust_class.as_str(),
            self.redaction_policy.as_str(),
            self.quality_metric.as_str(),
            self.activation_guard.as_str(),
            self.rollback_policy.as_str(),
            self.omit_priority_tsv(),
            self.compression_action_list(),
        )
    }

    #[cfg(test)]
    fn compression_action_list(self) -> String {
        if self.allowed_compression_actions.is_empty() {
            return "-".to_string();
        }

        self.allowed_compression_actions
            .iter()
            .map(|kind| kind.as_str())
            .collect::<Vec<_>>()
            .join(",")
    }

    #[cfg(test)]
    fn omit_priority_tsv(self) -> String {
        self.omit_priority
            .map(|priority| priority.to_string())
            .unwrap_or_else(|| "-".to_string())
    }
}

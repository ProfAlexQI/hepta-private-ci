#![forbid(unsafe_code)]
//! Shared authority journal, gate, and bounded-storage policy.
//!
//! Plugin, operator, and Telegram mutations keep their domain-specific event
//! schemas, while this crate owns the limits, external-anchor requirements,
//! gate names, and default-off product boundary they must all obey.

/// Persistent authority journal family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityJournalKind {
    /// App Server plugin share mutations.
    PluginMutation,
    /// Native operator note mutations.
    OperatorMutation,
    /// Native Telegram read/model/send authority.
    Telegram,
}

/// Rollback anchor placement required by an authority journal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityAnchorStrategy {
    /// A dedicated anchor outside the journal state root.
    DedicatedExternalFile,
    /// The shared external monotonic anchor maintained by the Native Gateway.
    SharedExternalMonotonicAnchor,
}

/// Bounded persistence policy shared across one journal implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorityJournalPolicy {
    /// Journal family.
    pub kind: AuthorityJournalKind,
    /// Current authenticated state schema.
    pub schema: &'static str,
    /// Maximum active records or events.
    pub max_active_records: usize,
    /// Maximum compacted terminal authorities retained for replay rejection.
    pub max_checkpointed_authorities: usize,
    /// Maximum serialized journal size.
    pub max_journal_bytes: u64,
    /// Maximum serialized record, event, response, or error payload.
    pub max_record_bytes: usize,
    /// Required rollback anchor placement.
    pub anchor_strategy: AuthorityAnchorStrategy,
    /// Whether real effects are enabled by default.
    pub default_effects_enabled: bool,
}

/// Plugin mutation external anchor override.
pub const PLUGIN_MUTATION_EXTERNAL_ANCHOR_FILE_ENV: &str =
    "HEPTA_PLUGIN_MUTATION_EXTERNAL_ANCHOR_FILE";
/// Native operator mutation enable gate.
pub const OPERATOR_MUTATION_ENABLED_ENV: &str = "HEPTA_OPERATOR_MUTATION_ENABLED";
/// Native operator mutation HMAC key path.
pub const OPERATOR_MUTATION_KEY_FILE_ENV: &str = "HEPTA_OPERATOR_MUTATION_AUTH_KEY_FILE";
/// Telegram operator authority enable gate.
pub const TELEGRAM_AUTHORITY_ENABLED_ENV: &str = "HEPTA_NATIVE_TELEGRAM_OPERATOR_AUTHORITY";
/// Telegram operator authority HMAC key path.
pub const TELEGRAM_AUTHORITY_KEY_FILE_ENV: &str = "HEPTA_NATIVE_TELEGRAM_OPERATOR_AUTH_KEY_FILE";
/// Telegram operator authority journal path.
pub const TELEGRAM_AUTHORITY_JOURNAL_FILE_ENV: &str =
    "HEPTA_NATIVE_TELEGRAM_AUTHORITY_JOURNAL_FILE";
/// Legacy Telegram external read gate.
pub const TELEGRAM_LIVE_READ_ENV: &str = "HEPTA_NATIVE_TELEGRAM_LIVE_READ";
/// Legacy Telegram model execution gate.
pub const TELEGRAM_MODEL_TURN_GATE_ENV: &str = "HEPTA_NATIVE_TELEGRAM_MODEL_TURN";
/// Legacy Telegram external send gate.
pub const TELEGRAM_SEND_GATE_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SEND";
/// Legacy Telegram background poller gate.
pub const TELEGRAM_POLL_LOOP_ENV: &str = "HEPTA_NATIVE_TELEGRAM_POLL_LOOP";
/// Legacy Telegram operator delivery approval gate.
pub const TELEGRAM_DELIVERY_APPROVED_ENV: &str = "HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED";

/// Plugin mutation journal policy.
pub const PLUGIN_MUTATION_JOURNAL_POLICY: AuthorityJournalPolicy = AuthorityJournalPolicy {
    kind: AuthorityJournalKind::PluginMutation,
    schema: "hepta-plugin-mutation-journal-v2",
    max_active_records: 4096,
    max_checkpointed_authorities: 20_000,
    max_journal_bytes: 64 * 1024 * 1024,
    max_record_bytes: 256 * 1024,
    anchor_strategy: AuthorityAnchorStrategy::DedicatedExternalFile,
    default_effects_enabled: false,
};

/// Operator mutation journal policy.
pub const OPERATOR_MUTATION_JOURNAL_POLICY: AuthorityJournalPolicy = AuthorityJournalPolicy {
    kind: AuthorityJournalKind::OperatorMutation,
    schema: "hepta.native.operator-mutation-journal.v2",
    max_active_records: 4096,
    max_checkpointed_authorities: 20_000,
    max_journal_bytes: 4 * 1024 * 1024,
    max_record_bytes: 64 * 1024,
    anchor_strategy: AuthorityAnchorStrategy::SharedExternalMonotonicAnchor,
    default_effects_enabled: false,
};

/// Telegram authority journal policy.
pub const TELEGRAM_AUTHORITY_JOURNAL_POLICY: AuthorityJournalPolicy = AuthorityJournalPolicy {
    kind: AuthorityJournalKind::Telegram,
    schema: "hepta.telegram.operator-authority.v1",
    max_active_records: 4096,
    max_checkpointed_authorities: 20_000,
    max_journal_bytes: 16 * 1024 * 1024,
    max_record_bytes: 8192,
    anchor_strategy: AuthorityAnchorStrategy::SharedExternalMonotonicAnchor,
    default_effects_enabled: false,
};

/// All production authority journals.
pub const AUTHORITY_JOURNAL_POLICIES: &[AuthorityJournalPolicy] = &[
    PLUGIN_MUTATION_JOURNAL_POLICY,
    OPERATOR_MUTATION_JOURNAL_POLICY,
    TELEGRAM_AUTHORITY_JOURNAL_POLICY,
];

/// Typed authority or effect gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorityGateSpec {
    /// Environment variable that controls the gate.
    pub env: &'static str,
    /// Product subsystem that owns the gate.
    pub owner: &'static str,
    /// Effect unlocked when true.
    pub effect: &'static str,
    /// Gate that must also be enabled, when one exists.
    pub requires: Option<&'static str>,
    /// Product-boundary default.
    pub default_enabled: bool,
}

/// Native operator mutation gate.
pub const OPERATOR_MUTATION_GATE: AuthorityGateSpec = AuthorityGateSpec {
    env: OPERATOR_MUTATION_ENABLED_ENV,
    owner: "openclaw",
    effect: "bounded local operator note mutation",
    requires: None,
    default_enabled: false,
};
/// Telegram operator authority gate.
pub const TELEGRAM_AUTHORITY_GATE: AuthorityGateSpec = AuthorityGateSpec {
    env: TELEGRAM_AUTHORITY_ENABLED_ENV,
    owner: "openclaw",
    effect: "phase-bound Telegram read/model/send authority",
    requires: None,
    default_enabled: false,
};
/// Telegram external read gate.
pub const TELEGRAM_LIVE_READ_GATE: AuthorityGateSpec = AuthorityGateSpec {
    env: TELEGRAM_LIVE_READ_ENV,
    owner: "legacy_openclaw",
    effect: "Telegram getUpdates",
    requires: Some(TELEGRAM_AUTHORITY_ENABLED_ENV),
    default_enabled: false,
};
/// Telegram model gate.
pub const TELEGRAM_MODEL_TURN_GATE: AuthorityGateSpec = AuthorityGateSpec {
    env: TELEGRAM_MODEL_TURN_GATE_ENV,
    owner: "legacy_openclaw",
    effect: "Telegram model invocation",
    requires: Some(TELEGRAM_AUTHORITY_ENABLED_ENV),
    default_enabled: false,
};
/// Telegram send gate.
pub const TELEGRAM_SEND_GATE: AuthorityGateSpec = AuthorityGateSpec {
    env: TELEGRAM_SEND_GATE_ENV,
    owner: "legacy_openclaw",
    effect: "Telegram sendMessage",
    requires: Some(TELEGRAM_AUTHORITY_ENABLED_ENV),
    default_enabled: false,
};
/// Telegram poll-loop gate.
pub const TELEGRAM_POLL_LOOP_GATE: AuthorityGateSpec = AuthorityGateSpec {
    env: TELEGRAM_POLL_LOOP_ENV,
    owner: "legacy_openclaw",
    effect: "Telegram background polling",
    requires: Some(TELEGRAM_AUTHORITY_ENABLED_ENV),
    default_enabled: false,
};
/// Telegram operator approval gate.
pub const TELEGRAM_DELIVERY_APPROVED_GATE: AuthorityGateSpec = AuthorityGateSpec {
    env: TELEGRAM_DELIVERY_APPROVED_ENV,
    owner: "openclaw",
    effect: "Telegram controlled delivery approval",
    requires: Some(TELEGRAM_AUTHORITY_ENABLED_ENV),
    default_enabled: false,
};

/// All production effect gates.
pub const AUTHORITY_GATE_SPECS: &[AuthorityGateSpec] = &[
    OPERATOR_MUTATION_GATE,
    TELEGRAM_AUTHORITY_GATE,
    TELEGRAM_LIVE_READ_GATE,
    TELEGRAM_MODEL_TURN_GATE,
    TELEGRAM_SEND_GATE,
    TELEGRAM_POLL_LOOP_GATE,
    TELEGRAM_DELIVERY_APPROVED_GATE,
];

/// Product disposition for a real mutation family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernedMutationDisposition {
    /// Kept because OpenClaw uses the capability through Hepta's governed backend.
    RequiredBackend,
    /// Implemented for controlled-live evaluation but excluded from the default product.
    ControlledLiveDeferred,
}

/// Side-effect surface reached by a governed mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernedEffectSurface {
    /// Authenticated local durable state.
    LocalDurableState,
    /// Authenticated remote plugin state.
    RemotePluginState,
    /// Credentialed Telegram read, model, or send pipeline.
    TelegramPipeline,
}

/// One admitted real mutation family in the governed-backend product.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GovernedMutationSpec {
    /// Stable operation or endpoint identifier.
    pub id: &'static str,
    /// Product owner.
    pub owner: &'static str,
    /// Journal family when the effect uses a shared authority journal.
    pub journal: Option<AuthorityJournalKind>,
    /// Gate required before the effect, when one exists.
    pub gate: Option<&'static str>,
    /// Effect surface.
    pub effect_surface: GovernedEffectSurface,
    /// Product disposition.
    pub disposition: GovernedMutationDisposition,
    /// Whether the effect is available without an explicit disabled-by-default gate.
    pub default_enabled: bool,
}

/// App Server plugin save operation.
pub const PLUGIN_SHARE_SAVE_OPERATION: &str = "plugin-share-save";
/// App Server plugin target update operation.
pub const PLUGIN_SHARE_UPDATE_TARGETS_OPERATION: &str = "plugin-share-update-targets";
/// App Server plugin checkout operation.
pub const PLUGIN_SHARE_CHECKOUT_OPERATION: &str = "plugin-share-checkout";
/// App Server plugin delete operation.
pub const PLUGIN_SHARE_DELETE_OPERATION: &str = "plugin-share-delete";
/// Native trusted preference commit endpoint.
pub const PREFERENCE_COMMIT_ENDPOINT: &str = "/api/v2/preferences/commit";
/// Native operator note commit endpoint.
pub const OPERATOR_NOTE_COMMIT_ENDPOINT: &str = "/api/v2/operator-mutations/note/commit";
/// Native Telegram authority commit endpoint.
pub const TELEGRAM_AUTHORITY_COMMIT_ENDPOINT: &str = "/api/v2/telegram/drain/authority/commit";

/// Exact real-mutation allowlist for the OpenClaw-governed backend.
pub const GOVERNED_MUTATION_SPECS: &[GovernedMutationSpec] = &[
    GovernedMutationSpec {
        id: PLUGIN_SHARE_SAVE_OPERATION,
        owner: "openclaw_app_server",
        journal: Some(AuthorityJournalKind::PluginMutation),
        gate: None,
        effect_surface: GovernedEffectSurface::RemotePluginState,
        disposition: GovernedMutationDisposition::RequiredBackend,
        default_enabled: false,
    },
    GovernedMutationSpec {
        id: PLUGIN_SHARE_UPDATE_TARGETS_OPERATION,
        owner: "openclaw_app_server",
        journal: Some(AuthorityJournalKind::PluginMutation),
        gate: None,
        effect_surface: GovernedEffectSurface::RemotePluginState,
        disposition: GovernedMutationDisposition::RequiredBackend,
        default_enabled: false,
    },
    GovernedMutationSpec {
        id: PLUGIN_SHARE_CHECKOUT_OPERATION,
        owner: "openclaw_app_server",
        journal: Some(AuthorityJournalKind::PluginMutation),
        gate: None,
        effect_surface: GovernedEffectSurface::RemotePluginState,
        disposition: GovernedMutationDisposition::RequiredBackend,
        default_enabled: false,
    },
    GovernedMutationSpec {
        id: PLUGIN_SHARE_DELETE_OPERATION,
        owner: "openclaw_app_server",
        journal: Some(AuthorityJournalKind::PluginMutation),
        gate: None,
        effect_surface: GovernedEffectSurface::RemotePluginState,
        disposition: GovernedMutationDisposition::RequiredBackend,
        default_enabled: false,
    },
    GovernedMutationSpec {
        id: PREFERENCE_COMMIT_ENDPOINT,
        owner: "openclaw_native_gateway",
        journal: None,
        gate: None,
        effect_surface: GovernedEffectSurface::LocalDurableState,
        disposition: GovernedMutationDisposition::RequiredBackend,
        default_enabled: true,
    },
    GovernedMutationSpec {
        id: OPERATOR_NOTE_COMMIT_ENDPOINT,
        owner: "openclaw_native_gateway",
        journal: Some(AuthorityJournalKind::OperatorMutation),
        gate: Some(OPERATOR_MUTATION_ENABLED_ENV),
        effect_surface: GovernedEffectSurface::LocalDurableState,
        disposition: GovernedMutationDisposition::RequiredBackend,
        default_enabled: false,
    },
    GovernedMutationSpec {
        id: TELEGRAM_AUTHORITY_COMMIT_ENDPOINT,
        owner: "legacy_openclaw",
        journal: Some(AuthorityJournalKind::Telegram),
        gate: Some(TELEGRAM_AUTHORITY_ENABLED_ENV),
        effect_surface: GovernedEffectSurface::TelegramPipeline,
        disposition: GovernedMutationDisposition::ControlledLiveDeferred,
        default_enabled: false,
    },
];

/// Parses the canonical truthy values accepted by authority gates.
pub fn parse_gate_truthy(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

/// Returns the typed gate spec for an environment variable.
pub fn gate_spec(env: &str) -> Option<&'static AuthorityGateSpec> {
    AUTHORITY_GATE_SPECS.iter().find(|spec| spec.env == env)
}

/// Returns the governed mutation spec for an operation or endpoint.
pub fn governed_mutation_spec(id: &str) -> Option<&'static GovernedMutationSpec> {
    GOVERNED_MUTATION_SPECS.iter().find(|spec| spec.id == id)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn policies_are_bounded_externally_anchored_and_default_off() {
        assert_eq!(AUTHORITY_JOURNAL_POLICIES.len(), 3);
        for policy in AUTHORITY_JOURNAL_POLICIES {
            assert!(policy.max_active_records > 0);
            assert!(policy.max_checkpointed_authorities >= policy.max_active_records);
            assert!(policy.max_journal_bytes > policy.max_record_bytes as u64);
            assert!(!policy.default_effects_enabled);
        }
    }

    #[test]
    fn typed_gate_registry_is_unique_and_default_off() {
        let mut envs = HashSet::new();
        for gate in AUTHORITY_GATE_SPECS {
            assert!(envs.insert(gate.env));
            assert!(!gate.default_enabled);
            if let Some(required) = gate.requires {
                assert!(
                    AUTHORITY_GATE_SPECS
                        .iter()
                        .any(|candidate| candidate.env == required)
                );
            }
        }
        assert!(parse_gate_truthy(" YES "));
        assert!(!parse_gate_truthy("disabled"));
        assert_eq!(gate_spec(TELEGRAM_SEND_GATE_ENV), Some(&TELEGRAM_SEND_GATE));
    }

    #[test]
    fn governed_mutation_registry_is_exact_and_product_bounded() {
        let mut ids = HashSet::new();
        assert_eq!(GOVERNED_MUTATION_SPECS.len(), 7);
        for mutation in GOVERNED_MUTATION_SPECS {
            assert!(ids.insert(mutation.id));
            if let Some(gate) = mutation.gate {
                assert_eq!(
                    gate_spec(gate).map(|spec| spec.default_enabled),
                    Some(false)
                );
                assert!(!mutation.default_enabled);
            }
        }
        assert_eq!(
            GOVERNED_MUTATION_SPECS
                .iter()
                .filter(|mutation| mutation.default_enabled)
                .map(|mutation| mutation.id)
                .collect::<Vec<_>>(),
            vec![PREFERENCE_COMMIT_ENDPOINT]
        );
        assert_eq!(
            governed_mutation_spec(TELEGRAM_AUTHORITY_COMMIT_ENDPOINT).map(|spec| spec.disposition),
            Some(GovernedMutationDisposition::ControlledLiveDeferred)
        );
        assert!(governed_mutation_spec("/api/chat").is_none());
    }
}

use crate::effect_reconciliation::EFFECT_RECONCILIATION_INSPECT_ENDPOINT;
use crate::effect_reconciliation::EFFECT_RECONCILIATION_RESOLVE_ENDPOINT;
use crate::native_telegram;
use crate::operator_mutation::OPERATOR_MUTATION_COMMIT_ENDPOINT;
use crate::operator_mutation::OPERATOR_MUTATION_ENABLED_ENV;
use crate::operator_mutation::OPERATOR_MUTATION_PLAN_ENDPOINT;
use crate::operator_mutation::enabled as operator_mutation_enabled;
use crate::operator_mutation_reconciliation::OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT;
use crate::operator_mutation_reconciliation::OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT;
use crate::preference_ingress::PREFERENCE_CHALLENGE_ENDPOINT;
use crate::preference_ingress::PREFERENCE_COMMIT_ENDPOINT;
use crate::route_registry::CONTROL_UI_ROUTE_PARITY_ENDPOINT;
use crate::route_registry::CONTROL_UI_ROUTE_SPECS;
use crate::route_registry::GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT;
use crate::route_registry::GATEWAY_REPLACEMENT_READINESS_ENDPOINT;
use crate::route_registry::TELEGRAM_LIVE_SOAK_ENDPOINT;
use crate::runtime_composition::NativeGatewayRuntime;
use crate::runtime_composition::RUNTIME_KERNEL_CANARY_ACTION_ENDPOINT;
use crate::runtime_composition::RuntimeRequestDisposition;
use crate::runtime_composition::RuntimeRequestPreflightReceipt;
use crate::runtime_mutation::RUNTIME_MUTATION_CANARY_ENDPOINT;
use crate::telegram_authority::TELEGRAM_AUTHORITY_COMMIT_ENDPOINT;
use crate::telegram_authority::TELEGRAM_AUTHORITY_ENABLED_ENV;
use crate::telegram_authority::TELEGRAM_AUTHORITY_PLAN_ENDPOINT;
use crate::telegram_authority::TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT;
use crate::telegram_authority::TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use sha2::Digest;
use sha2::Sha256;

pub(crate) const TELEGRAM_RECEIVE_ONCE_ENDPOINT: &str = "/api/telegram-receive-once";
pub(crate) const OPERATOR_AUTHORITY_CHALLENGE_ENDPOINT: &str =
    "/api/v2/operator-authority/challenge";
pub(crate) const RUNTIME_INGRESS_REGISTRY_SCHEMA_VERSION: &str =
    "hepta_runtime_ingress_lifecycle_registry_v1";

const QUARANTINED_TRANSITIVE_CANARY_EFFECT_PATHS: &[&str] = &[
    "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-readback-receipt-acceptance-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-readiness-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-plan-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-preflight-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-execution-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-tombstone-cleanup-acceptance-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-envelope-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-audit-trail-immutable-evidence-denial-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-cancellation-supersession-denial-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-export-query-observability-denial-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-operator-facing-summary-briefing-non-persistence-denial-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-denial-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-retention-expiry-garbage-collection-denial-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-receipt-boundary",
    "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-preflight-boundary",
];

#[cfg(test)]
const DETACHED_CONTROL_UI_REPORT_PATHS: &[&str] = &[
    CONTROL_UI_ROUTE_PARITY_ENDPOINT,
    GATEWAY_REPLACEMENT_READINESS_ENDPOINT,
    GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT,
    TELEGRAM_LIVE_SOAK_ENDPOINT,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RuntimeIngressKind {
    MetadataRead,
    CredentialedNetworkRead,
    AuthenticatedPreferencePlan,
    AuthenticatedPreferenceCommit,
    RuntimeKernelCanary,
    MutationPlan,
    OperatorEffectInspection,
    TerminalReceiptReconciliation,
    TelegramOperatorPlan,
    TelegramOperatorPipeline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IngressEffectClass {
    StaticAssetRead,
    MetadataRead,
    BoundedEphemeralVerification,
    QuarantinedLegacyMutation,
    CredentialedNetworkRead,
    AuthenticatedPreferencePlan,
    AuthenticatedPreferenceCommit,
    RuntimeKernelReadOnlyCanary,
    RuntimeKernelLocalMutation,
    MutationPlan,
    OperatorMutationPlan,
    OperatorEffectInspection,
    TerminalReceiptReconciliation,
    TelegramOperatorPlan,
    TelegramOperatorPipeline,
}

impl IngressEffectClass {
    const fn mutates_state(self) -> bool {
        matches!(
            self,
            Self::QuarantinedLegacyMutation
                | Self::CredentialedNetworkRead
                | Self::AuthenticatedPreferencePlan
                | Self::AuthenticatedPreferenceCommit
                | Self::RuntimeKernelReadOnlyCanary
                | Self::RuntimeKernelLocalMutation
                | Self::OperatorMutationPlan
                | Self::TerminalReceiptReconciliation
                | Self::TelegramOperatorPlan
                | Self::TelegramOperatorPipeline
        )
    }

    const fn external_effect(self) -> bool {
        matches!(
            self,
            Self::CredentialedNetworkRead
                | Self::RuntimeKernelLocalMutation
                | Self::TelegramOperatorPipeline
        )
    }

    const fn performs_effect(self) -> bool {
        self.mutates_state() || self.external_effect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IngressAuthorityOwner {
    RuntimeKernelRequestBinding,
    UnassignedLegacyMutation,
    RuntimeKernelTelegramRead,
    TrustedPreferenceIngress,
    RuntimeKernelCanary,
    RuntimeKernelMutationCanary,
    OperatorMutation,
    OperatorEffectReconciliation,
    TelegramOperatorPipeline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IngressAccessPolicy {
    Forbidden,
    MetadataOnly,
    AfterExactAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IngressLifecycleRequirement {
    NotRequired,
    RequiredBeforeEffect,
    RequiredAfterEffect,
    RequiredForTerminal,
    RequiredExisting,
    DomainAtomicCommit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IngressDefaultEnablement {
    ReadOnlyEnabled,
    PlanOnlyEnabled,
    AuthenticatedEnabled,
    DisabledUnlessExplicitGate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IngressLifecycleSpec {
    pub(crate) method: &'static str,
    pub(crate) path_pattern: &'static str,
    pub(crate) effect_class: IngressEffectClass,
    pub(crate) authority_owner: IngressAuthorityOwner,
    pub(crate) secret_access: IngressAccessPolicy,
    pub(crate) config_access: IngressAccessPolicy,
    pub(crate) network_access: IngressAccessPolicy,
    pub(crate) durable_intent: IngressLifecycleRequirement,
    pub(crate) effect_ack: IngressLifecycleRequirement,
    pub(crate) terminal_receipt: IngressLifecycleRequirement,
    pub(crate) default_enablement: IngressDefaultEnablement,
    pub(crate) source: &'static str,
}

impl IngressLifecycleSpec {
    const fn credential_access(self) -> bool {
        matches!(self.secret_access, IngressAccessPolicy::AfterExactAuthority)
    }

    pub(crate) const fn ingress_kind(self) -> RuntimeIngressKind {
        match self.effect_class {
            IngressEffectClass::StaticAssetRead
            | IngressEffectClass::MetadataRead
            | IngressEffectClass::BoundedEphemeralVerification => RuntimeIngressKind::MetadataRead,
            IngressEffectClass::QuarantinedLegacyMutation => RuntimeIngressKind::MutationPlan,
            IngressEffectClass::CredentialedNetworkRead => {
                RuntimeIngressKind::CredentialedNetworkRead
            }
            IngressEffectClass::AuthenticatedPreferencePlan => {
                RuntimeIngressKind::AuthenticatedPreferencePlan
            }
            IngressEffectClass::AuthenticatedPreferenceCommit => {
                RuntimeIngressKind::AuthenticatedPreferenceCommit
            }
            IngressEffectClass::RuntimeKernelReadOnlyCanary => {
                RuntimeIngressKind::RuntimeKernelCanary
            }
            IngressEffectClass::RuntimeKernelLocalMutation
            | IngressEffectClass::MutationPlan
            | IngressEffectClass::OperatorMutationPlan => RuntimeIngressKind::MutationPlan,
            IngressEffectClass::OperatorEffectInspection => {
                RuntimeIngressKind::OperatorEffectInspection
            }
            IngressEffectClass::TerminalReceiptReconciliation => {
                RuntimeIngressKind::TerminalReceiptReconciliation
            }
            IngressEffectClass::TelegramOperatorPlan => RuntimeIngressKind::TelegramOperatorPlan,
            IngressEffectClass::TelegramOperatorPipeline => {
                RuntimeIngressKind::TelegramOperatorPipeline
            }
        }
    }

    pub(crate) fn disposition(self) -> RuntimeRequestDisposition {
        if self.method == "GET" {
            RuntimeRequestDisposition::ReadOnlyDispatch
        } else if self.authority_owner != IngressAuthorityOwner::RuntimeKernelRequestBinding {
            RuntimeRequestDisposition::ExactAuthorityDispatch
        } else {
            RuntimeRequestDisposition::PlanOnlyQuarantine
        }
    }
}

impl Serialize for IngressLifecycleSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("IngressLifecycleSpec", 15)?;
        state.serialize_field("method", self.method)?;
        state.serialize_field("path_pattern", self.path_pattern)?;
        state.serialize_field("effect_class", &self.effect_class)?;
        state.serialize_field("mutates_state", &self.effect_class.mutates_state())?;
        state.serialize_field("external_effect", &self.effect_class.external_effect())?;
        state.serialize_field("credential_access", &self.credential_access())?;
        state.serialize_field("authority_owner", &self.authority_owner)?;
        state.serialize_field("secret_access", &self.secret_access)?;
        state.serialize_field("config_access", &self.config_access)?;
        state.serialize_field("network_access", &self.network_access)?;
        state.serialize_field("durable_intent", &self.durable_intent)?;
        state.serialize_field("effect_ack", &self.effect_ack)?;
        state.serialize_field("terminal_receipt", &self.terminal_receipt)?;
        state.serialize_field("default_enablement", &self.default_enablement)?;
        state.serialize_field("source", self.source)?;
        state.end()
    }
}

const fn metadata_read(path_pattern: &'static str) -> IngressLifecycleSpec {
    IngressLifecycleSpec {
        method: "GET",
        path_pattern,
        effect_class: IngressEffectClass::MetadataRead,
        authority_owner: IngressAuthorityOwner::RuntimeKernelRequestBinding,
        secret_access: IngressAccessPolicy::Forbidden,
        config_access: IngressAccessPolicy::MetadataOnly,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::NotRequired,
        effect_ack: IngressLifecycleRequirement::NotRequired,
        terminal_receipt: IngressLifecycleRequirement::NotRequired,
        default_enablement: IngressDefaultEnablement::ReadOnlyEnabled,
        source: "special_native_gateway_route",
    }
}

const fn static_read(path_pattern: &'static str) -> IngressLifecycleSpec {
    IngressLifecycleSpec {
        effect_class: IngressEffectClass::StaticAssetRead,
        config_access: IngressAccessPolicy::Forbidden,
        ..metadata_read(path_pattern)
    }
}

const SPECIAL_INGRESS_LIFECYCLES: &[IngressLifecycleSpec] = &[
    static_read("/"),
    static_read("/index.html"),
    static_read("/styles.css"),
    static_read("/gateway-status"),
    static_read("/gateway-status.html"),
    static_read("/native-gateway.html"),
    static_read("/assets/hepta-agent-logo.png"),
    metadata_read("/health"),
    metadata_read("/api/health"),
    metadata_read("/api/native-gateway"),
    metadata_read("/api/telegram-plugin"),
    metadata_read("/api/telegram-model-turn-plan"),
    metadata_read("/api/telegram-model-bridge"),
    metadata_read("/api/telegram-send-plan"),
    metadata_read("/api/telegram-drain-once"),
    metadata_read("/api/telegram-poll-loop"),
    metadata_read("/api/telegram-cursor"),
    metadata_read(CONTROL_UI_ROUTE_PARITY_ENDPOINT),
    metadata_read(GATEWAY_REPLACEMENT_READINESS_ENDPOINT),
    metadata_read(GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT),
    metadata_read(TELEGRAM_LIVE_SOAK_ENDPOINT),
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: OPERATOR_AUTHORITY_CHALLENGE_ENDPOINT,
        effect_class: IngressEffectClass::BoundedEphemeralVerification,
        authority_owner: IngressAuthorityOwner::RuntimeKernelRequestBinding,
        secret_access: IngressAccessPolicy::Forbidden,
        config_access: IngressAccessPolicy::MetadataOnly,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::NotRequired,
        effect_ack: IngressLifecycleRequirement::NotRequired,
        terminal_receipt: IngressLifecycleRequirement::NotRequired,
        default_enablement: IngressDefaultEnablement::AuthenticatedEnabled,
        source: "operator_authority_challenge",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: TELEGRAM_RECEIVE_ONCE_ENDPOINT,
        effect_class: IngressEffectClass::CredentialedNetworkRead,
        authority_owner: IngressAuthorityOwner::RuntimeKernelTelegramRead,
        secret_access: IngressAccessPolicy::AfterExactAuthority,
        config_access: IngressAccessPolicy::AfterExactAuthority,
        network_access: IngressAccessPolicy::AfterExactAuthority,
        durable_intent: IngressLifecycleRequirement::RequiredBeforeEffect,
        effect_ack: IngressLifecycleRequirement::NotRequired,
        terminal_receipt: IngressLifecycleRequirement::NotRequired,
        default_enablement: IngressDefaultEnablement::DisabledUnlessExplicitGate,
        source: "telegram_receive_once",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: TELEGRAM_AUTHORITY_PLAN_ENDPOINT,
        effect_class: IngressEffectClass::TelegramOperatorPlan,
        authority_owner: IngressAuthorityOwner::TelegramOperatorPipeline,
        secret_access: IngressAccessPolicy::AfterExactAuthority,
        config_access: IngressAccessPolicy::MetadataOnly,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::NotRequired,
        effect_ack: IngressLifecycleRequirement::NotRequired,
        terminal_receipt: IngressLifecycleRequirement::NotRequired,
        default_enablement: IngressDefaultEnablement::DisabledUnlessExplicitGate,
        source: "telegram_operator_authority",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: TELEGRAM_AUTHORITY_COMMIT_ENDPOINT,
        effect_class: IngressEffectClass::TelegramOperatorPipeline,
        authority_owner: IngressAuthorityOwner::TelegramOperatorPipeline,
        secret_access: IngressAccessPolicy::AfterExactAuthority,
        config_access: IngressAccessPolicy::AfterExactAuthority,
        network_access: IngressAccessPolicy::AfterExactAuthority,
        durable_intent: IngressLifecycleRequirement::RequiredBeforeEffect,
        effect_ack: IngressLifecycleRequirement::RequiredAfterEffect,
        terminal_receipt: IngressLifecycleRequirement::RequiredForTerminal,
        default_enablement: IngressDefaultEnablement::DisabledUnlessExplicitGate,
        source: "telegram_operator_authority",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT,
        effect_class: IngressEffectClass::OperatorEffectInspection,
        authority_owner: IngressAuthorityOwner::TelegramOperatorPipeline,
        secret_access: IngressAccessPolicy::AfterExactAuthority,
        config_access: IngressAccessPolicy::Forbidden,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::RequiredExisting,
        effect_ack: IngressLifecycleRequirement::RequiredExisting,
        terminal_receipt: IngressLifecycleRequirement::NotRequired,
        default_enablement: IngressDefaultEnablement::DisabledUnlessExplicitGate,
        source: "telegram_terminal_reconciliation",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
        effect_class: IngressEffectClass::TerminalReceiptReconciliation,
        authority_owner: IngressAuthorityOwner::TelegramOperatorPipeline,
        secret_access: IngressAccessPolicy::AfterExactAuthority,
        config_access: IngressAccessPolicy::Forbidden,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::RequiredExisting,
        effect_ack: IngressLifecycleRequirement::RequiredExisting,
        terminal_receipt: IngressLifecycleRequirement::RequiredForTerminal,
        default_enablement: IngressDefaultEnablement::DisabledUnlessExplicitGate,
        source: "telegram_terminal_reconciliation",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: PREFERENCE_CHALLENGE_ENDPOINT,
        effect_class: IngressEffectClass::AuthenticatedPreferencePlan,
        authority_owner: IngressAuthorityOwner::TrustedPreferenceIngress,
        secret_access: IngressAccessPolicy::AfterExactAuthority,
        config_access: IngressAccessPolicy::Forbidden,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::NotRequired,
        effect_ack: IngressLifecycleRequirement::NotRequired,
        terminal_receipt: IngressLifecycleRequirement::NotRequired,
        default_enablement: IngressDefaultEnablement::AuthenticatedEnabled,
        source: "trusted_preference_ingress",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: PREFERENCE_COMMIT_ENDPOINT,
        effect_class: IngressEffectClass::AuthenticatedPreferenceCommit,
        authority_owner: IngressAuthorityOwner::TrustedPreferenceIngress,
        secret_access: IngressAccessPolicy::AfterExactAuthority,
        config_access: IngressAccessPolicy::Forbidden,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::DomainAtomicCommit,
        effect_ack: IngressLifecycleRequirement::DomainAtomicCommit,
        terminal_receipt: IngressLifecycleRequirement::DomainAtomicCommit,
        default_enablement: IngressDefaultEnablement::AuthenticatedEnabled,
        source: "trusted_preference_ingress",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: RUNTIME_KERNEL_CANARY_ACTION_ENDPOINT,
        effect_class: IngressEffectClass::RuntimeKernelReadOnlyCanary,
        authority_owner: IngressAuthorityOwner::RuntimeKernelCanary,
        secret_access: IngressAccessPolicy::Forbidden,
        config_access: IngressAccessPolicy::Forbidden,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::RequiredBeforeEffect,
        effect_ack: IngressLifecycleRequirement::NotRequired,
        terminal_receipt: IngressLifecycleRequirement::RequiredForTerminal,
        default_enablement: IngressDefaultEnablement::PlanOnlyEnabled,
        source: "runtime_kernel_canary",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: RUNTIME_MUTATION_CANARY_ENDPOINT,
        effect_class: IngressEffectClass::RuntimeKernelLocalMutation,
        authority_owner: IngressAuthorityOwner::RuntimeKernelMutationCanary,
        secret_access: IngressAccessPolicy::Forbidden,
        config_access: IngressAccessPolicy::Forbidden,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::RequiredBeforeEffect,
        effect_ack: IngressLifecycleRequirement::RequiredAfterEffect,
        terminal_receipt: IngressLifecycleRequirement::RequiredForTerminal,
        default_enablement: IngressDefaultEnablement::DisabledUnlessExplicitGate,
        source: "runtime_mutation_canary",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: OPERATOR_MUTATION_PLAN_ENDPOINT,
        effect_class: IngressEffectClass::OperatorMutationPlan,
        authority_owner: IngressAuthorityOwner::OperatorMutation,
        secret_access: IngressAccessPolicy::AfterExactAuthority,
        config_access: IngressAccessPolicy::Forbidden,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::NotRequired,
        effect_ack: IngressLifecycleRequirement::NotRequired,
        terminal_receipt: IngressLifecycleRequirement::NotRequired,
        default_enablement: IngressDefaultEnablement::DisabledUnlessExplicitGate,
        source: "operator_mutation",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: OPERATOR_MUTATION_COMMIT_ENDPOINT,
        effect_class: IngressEffectClass::RuntimeKernelLocalMutation,
        authority_owner: IngressAuthorityOwner::OperatorMutation,
        secret_access: IngressAccessPolicy::AfterExactAuthority,
        config_access: IngressAccessPolicy::Forbidden,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::RequiredBeforeEffect,
        effect_ack: IngressLifecycleRequirement::RequiredAfterEffect,
        terminal_receipt: IngressLifecycleRequirement::RequiredForTerminal,
        default_enablement: IngressDefaultEnablement::DisabledUnlessExplicitGate,
        source: "operator_mutation",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT,
        effect_class: IngressEffectClass::OperatorEffectInspection,
        authority_owner: IngressAuthorityOwner::OperatorMutation,
        secret_access: IngressAccessPolicy::AfterExactAuthority,
        config_access: IngressAccessPolicy::Forbidden,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::RequiredExisting,
        effect_ack: IngressLifecycleRequirement::RequiredExisting,
        terminal_receipt: IngressLifecycleRequirement::RequiredExisting,
        default_enablement: IngressDefaultEnablement::DisabledUnlessExplicitGate,
        source: "operator_mutation_reconciliation",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT,
        effect_class: IngressEffectClass::TerminalReceiptReconciliation,
        authority_owner: IngressAuthorityOwner::OperatorMutation,
        secret_access: IngressAccessPolicy::AfterExactAuthority,
        config_access: IngressAccessPolicy::Forbidden,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::RequiredExisting,
        effect_ack: IngressLifecycleRequirement::RequiredExisting,
        terminal_receipt: IngressLifecycleRequirement::RequiredExisting,
        default_enablement: IngressDefaultEnablement::DisabledUnlessExplicitGate,
        source: "operator_mutation_reconciliation",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: EFFECT_RECONCILIATION_INSPECT_ENDPOINT,
        effect_class: IngressEffectClass::OperatorEffectInspection,
        authority_owner: IngressAuthorityOwner::OperatorEffectReconciliation,
        secret_access: IngressAccessPolicy::AfterExactAuthority,
        config_access: IngressAccessPolicy::Forbidden,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::RequiredExisting,
        effect_ack: IngressLifecycleRequirement::RequiredExisting,
        terminal_receipt: IngressLifecycleRequirement::NotRequired,
        default_enablement: IngressDefaultEnablement::DisabledUnlessExplicitGate,
        source: "effect_reconciliation",
    },
    IngressLifecycleSpec {
        method: "POST",
        path_pattern: EFFECT_RECONCILIATION_RESOLVE_ENDPOINT,
        effect_class: IngressEffectClass::TerminalReceiptReconciliation,
        authority_owner: IngressAuthorityOwner::OperatorEffectReconciliation,
        secret_access: IngressAccessPolicy::AfterExactAuthority,
        config_access: IngressAccessPolicy::Forbidden,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::RequiredExisting,
        effect_ack: IngressLifecycleRequirement::RequiredExisting,
        terminal_receipt: IngressLifecycleRequirement::RequiredForTerminal,
        default_enablement: IngressDefaultEnablement::DisabledUnlessExplicitGate,
        source: "effect_reconciliation",
    },
];

pub(crate) struct RuntimeIngressResponse {
    pub(crate) status: &'static str,
    pub(crate) body: String,
}

pub(crate) fn operator_execution_response(
    runtime: &NativeGatewayRuntime,
    method: &str,
    path: &str,
    body: Option<&str>,
    request_binding_hash: &str,
) -> Option<RuntimeIngressResponse> {
    if method != "POST" {
        return None;
    }
    if path == OPERATOR_AUTHORITY_CHALLENGE_ENDPOINT {
        let admitted = body
            .and_then(|body| serde_json::from_str::<serde_json::Value>(body).ok())
            .is_some_and(|body| {
                body == serde_json::json!({"operation":"inspect-authority-context"})
            });
        if !admitted {
            return Some(RuntimeIngressResponse {
                status: "400 Bad Request",
                body: r#"{"error":"operator_authority_challenge.exact_request_required"}"#
                    .to_string(),
            });
        }
        return Some(runtime.operator_authority_challenge().map_or_else(
            |_| {
                RuntimeIngressResponse {
                    status: "503 Service Unavailable",
                    body: r#"{"error":"operator_authority_challenge.runtime_context_unavailable"}"#
                        .to_string(),
                }
            },
            |receipt| RuntimeIngressResponse {
                status: "200 OK",
                body: json_receipt(receipt).unwrap_or_else(|_| {
                    r#"{"error":"operator_authority_challenge.serialization_failed"}"#.to_string()
                }),
            },
        ));
    }
    if matches!(
        path,
        OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT
            | OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT
    ) {
        let response = runtime.route_operator_mutation_reconciliation(
            method,
            path,
            body,
            request_binding_hash,
        )?;
        return Some(RuntimeIngressResponse {
            status: response.status,
            body: response.body,
        });
    }
    if matches!(
        path,
        OPERATOR_MUTATION_PLAN_ENDPOINT | OPERATOR_MUTATION_COMMIT_ENDPOINT
    ) {
        if !operator_mutation_enabled() {
            return Some(RuntimeIngressResponse {
                status: "403 Forbidden",
                body: serde_json::json!({
                    "error": "operator_mutation.disabled",
                    "required_gate": OPERATOR_MUTATION_ENABLED_ENV,
                    "mutation_authorized": false,
                    "durable_intent_recorded": false,
                    "provider_effect_ack_recorded": false,
                    "terminal_receipt_recorded": false,
                })
                .to_string(),
            });
        }
        let result = if path == OPERATOR_MUTATION_PLAN_ENDPOINT {
            runtime
                .plan_operator_mutation(body, request_binding_hash)
                .and_then(json_receipt)
        } else {
            runtime
                .commit_operator_mutation(body, request_binding_hash)
                .and_then(json_receipt)
        };
        return Some(result.map_or_else(
            |_| RuntimeIngressResponse {
                status: "409 Conflict",
                body: r#"{"error":"operator_mutation.denied_or_incomplete"}"#.to_string(),
            },
            |body| RuntimeIngressResponse {
                status: "200 OK",
                body,
            },
        ));
    }
    if matches!(
        path,
        TELEGRAM_AUTHORITY_PLAN_ENDPOINT | TELEGRAM_AUTHORITY_COMMIT_ENDPOINT
    ) {
        if !runtime.telegram_operator_pipeline_enabled() {
            return Some(RuntimeIngressResponse {
                status: "403 Forbidden",
                body: serde_json::json!({
                    "error": "telegram_operator_pipeline.disabled",
                    "required_gate": TELEGRAM_AUTHORITY_ENABLED_ENV,
                    "live_read_authorized": false,
                    "model_invocation_authorized": false,
                    "send_authorized": false,
                    "durable_intent_recorded": false,
                    "provider_effect_ack_recorded": false,
                    "terminal_receipt_recorded": false,
                })
                .to_string(),
            });
        }
        let result = if path == TELEGRAM_AUTHORITY_PLAN_ENDPOINT {
            runtime
                .plan_operator_telegram_drain(body, request_binding_hash)
                .and_then(json_receipt)
        } else {
            runtime
                .commit_operator_telegram_drain(body, request_binding_hash)
                .and_then(json_receipt)
        };
        return Some(result.map_or_else(
            |_| RuntimeIngressResponse {
                status: "409 Conflict",
                body: r#"{"error":"telegram_operator_pipeline.denied_or_incomplete","terminal_receipt_recorded":false}"#.to_string(),
            },
            |body| RuntimeIngressResponse {
                status: "200 OK",
                body,
            },
        ));
    }
    None
}

fn json_receipt(receipt: impl Serialize) -> anyhow::Result<String> {
    serde_json::to_string(&receipt).map_err(anyhow::Error::msg)
}

pub(crate) fn telegram_receive_once_response(
    runtime: Option<&NativeGatewayRuntime>,
    requested: bool,
    limit: usize,
) -> RuntimeIngressResponse {
    let Some(runtime) = runtime else {
        return RuntimeIngressResponse {
            status: "503 Service Unavailable",
            body: r#"{"error":"telegram_runtime_admission.runtime_unavailable"}"#.to_string(),
        };
    };
    let authority = match runtime.authorize_telegram_receive() {
        Ok(authority) => authority,
        Err(error) => {
            return RuntimeIngressResponse {
                status: "503 Service Unavailable",
                body: serde_json::json!({
                    "error": error.to_string(),
                    "ingress": "credentialed_network_read",
                    "config_observed": false,
                    "token_observed": false,
                    "cursor_observed": false,
                    "external_network_read": false,
                })
                .to_string(),
            };
        }
    };
    let status = native_telegram::telegram_receive_once_status(requested, limit, &authority);
    RuntimeIngressResponse {
        status: "200 OK",
        body: serde_json::to_string(&status).unwrap_or_else(|error| {
            serde_json::json!({"error": format!("serialization failed: {error}")}).to_string()
        }),
    }
}

fn control_ui_lifecycle(spec: &'static crate::gate_spec::GateSpec) -> Option<IngressLifecycleSpec> {
    let (effect_class, authority_owner, default_enablement, source) = match spec.method {
        "GET" if QUARANTINED_TRANSITIVE_CANARY_EFFECT_PATHS.contains(&spec.pattern) => (
            IngressEffectClass::QuarantinedLegacyMutation,
            IngressAuthorityOwner::UnassignedLegacyMutation,
            IngressDefaultEnablement::DisabledUnlessExplicitGate,
            "control_ui_transitive_effect_quarantine",
        ),
        "GET" => (
            IngressEffectClass::MetadataRead,
            IngressAuthorityOwner::RuntimeKernelRequestBinding,
            IngressDefaultEnablement::ReadOnlyEnabled,
            "control_ui_route_specs",
        ),
        "POST" if spec.is_guarded() => (
            IngressEffectClass::MutationPlan,
            IngressAuthorityOwner::RuntimeKernelRequestBinding,
            IngressDefaultEnablement::PlanOnlyEnabled,
            "control_ui_route_specs",
        ),
        _ => return None,
    };
    Some(IngressLifecycleSpec {
        method: spec.method,
        path_pattern: spec.pattern,
        effect_class,
        authority_owner,
        secret_access: IngressAccessPolicy::Forbidden,
        config_access: IngressAccessPolicy::MetadataOnly,
        network_access: IngressAccessPolicy::Forbidden,
        durable_intent: IngressLifecycleRequirement::NotRequired,
        effect_ack: IngressLifecycleRequirement::NotRequired,
        terminal_receipt: IngressLifecycleRequirement::NotRequired,
        default_enablement,
        source,
    })
}

pub(crate) fn runtime_ingress_lifecycle(method: &str, path: &str) -> Option<IngressLifecycleSpec> {
    SPECIAL_INGRESS_LIFECYCLES
        .iter()
        .copied()
        .find(|spec| spec.method == method && route_pattern_matches(spec.path_pattern, path))
        .or_else(|| {
            CONTROL_UI_ROUTE_SPECS.iter().find_map(|route| {
                (route.method == method && route_pattern_matches(route.pattern, path))
                    .then(|| control_ui_lifecycle(route))
                    .flatten()
            })
        })
        .filter(|spec| validate_lifecycle(*spec).is_ok())
}

#[cfg(test)]
pub(crate) fn is_detached_control_ui_report_for_test(method: &str, path: &str) -> bool {
    method == "GET"
        && (QUARANTINED_TRANSITIVE_CANARY_EFFECT_PATHS.contains(&path)
            || DETACHED_CONTROL_UI_REPORT_PATHS.contains(&path))
}

pub(crate) fn runtime_ingress_lifecycle_registry() -> Vec<IngressLifecycleSpec> {
    SPECIAL_INGRESS_LIFECYCLES
        .iter()
        .copied()
        .chain(
            CONTROL_UI_ROUTE_SPECS
                .iter()
                .filter_map(control_ui_lifecycle),
        )
        .collect()
}

pub(crate) fn runtime_ingress_lifecycle_registry_digest() -> Result<String, serde_json::Error> {
    let registry = runtime_ingress_lifecycle_registry();
    let canonical = serde_json::to_vec(&registry)?;
    Ok(format!("{:x}", Sha256::digest(canonical)))
}

fn route_pattern_matches(pattern: &str, path: &str) -> bool {
    let mut pattern_segments = pattern.split('/');
    let mut path_segments = path.split('/');
    loop {
        match (pattern_segments.next(), path_segments.next()) {
            (Some(pattern), Some(value)) => {
                let variable = pattern.starts_with('<') && pattern.ends_with('>');
                if (!variable && pattern != value) || (variable && value.is_empty()) {
                    return false;
                }
            }
            (None, None) => return true,
            _ => return false,
        }
    }
}

fn validate_lifecycle(spec: IngressLifecycleSpec) -> Result<(), &'static str> {
    if !matches!(spec.method, "GET" | "POST") {
        return Err("unsupported_method");
    }
    if spec.effect_class == IngressEffectClass::QuarantinedLegacyMutation {
        return Err("legacy_mutation_route_quarantined");
    }
    if spec.method == "GET" && spec.effect_class.performs_effect() {
        return Err("get_effect_surface");
    }
    if spec.effect_class.performs_effect()
        && spec.authority_owner == IngressAuthorityOwner::RuntimeKernelRequestBinding
    {
        return Err("effect_authority_owner_missing");
    }
    if spec.network_access == IngressAccessPolicy::AfterExactAuthority
        && !matches!(
            spec.authority_owner,
            IngressAuthorityOwner::RuntimeKernelTelegramRead
                | IngressAuthorityOwner::TelegramOperatorPipeline
        )
    {
        return Err("network_authority_owner_mismatch");
    }
    if spec.effect_class == IngressEffectClass::RuntimeKernelLocalMutation
        && (spec.durable_intent != IngressLifecycleRequirement::RequiredBeforeEffect
            || spec.effect_ack != IngressLifecycleRequirement::RequiredAfterEffect
            || spec.terminal_receipt != IngressLifecycleRequirement::RequiredForTerminal)
    {
        return Err("mutation_lifecycle_incomplete");
    }
    if spec.effect_class == IngressEffectClass::TelegramOperatorPipeline
        && (spec.durable_intent != IngressLifecycleRequirement::RequiredBeforeEffect
            || spec.effect_ack != IngressLifecycleRequirement::RequiredAfterEffect
            || spec.terminal_receipt != IngressLifecycleRequirement::RequiredForTerminal)
    {
        return Err("telegram_pipeline_lifecycle_incomplete");
    }
    Ok(())
}

pub(crate) fn runtime_preflight_matches(
    method: &str,
    path: &str,
    preflight: &RuntimeRequestPreflightReceipt,
) -> bool {
    let Some(lifecycle) = runtime_ingress_lifecycle(method, path) else {
        return false;
    };
    !preflight.request_binding_hash.is_empty()
        && preflight.disposition == lifecycle.disposition()
        && preflight.ingress_kind == lifecycle.ingress_kind()
}

#[cfg(test)]
#[path = "../tests/unit/runtime_ingress.rs"]
mod tests;

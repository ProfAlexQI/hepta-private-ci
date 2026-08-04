use std::{collections::BTreeMap, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The only schema version accepted by this build of the product shell.
pub const HEPTA_BRIDGE_SCHEMA_VERSION: u16 = 1;
pub const MAX_BRIDGE_SESSION_ID_BYTES: usize = 1024;
pub const MAX_BRIDGE_CORRELATION_ID_BYTES: usize = 1024;

macro_rules! string_identifier {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self { Self(value.into()) }

            pub fn as_str(&self) -> &str { &self.0 }

            pub fn is_empty(&self) -> bool { self.0.is_empty() }

            pub fn is_blank(&self) -> bool { self.0.trim().is_empty() }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self { Self::new(value) }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self { Self::new(value) }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

string_identifier!(
    /// Stable identifier for a bridge request, update, or domain entity.
    BridgeEntityId
);
string_identifier!(
    /// Stable identifier for a Hepta runtime session.
    SessionId
);
string_identifier!(
    /// Identifier used to correlate UI intent, adapter work, and resulting updates.
    CorrelationId
);

impl SessionId {
    pub fn is_live_transport_safe(&self) -> bool {
        valid_bounded_live_identifier(self.as_str(), MAX_BRIDGE_SESSION_ID_BYTES)
    }
}

impl CorrelationId {
    pub fn is_live_transport_safe(&self) -> bool {
        valid_bounded_live_identifier(self.as_str(), MAX_BRIDGE_CORRELATION_ID_BYTES)
    }
}

fn valid_bounded_live_identifier(value: &str, max_bytes: usize) -> bool {
    !value.is_empty()
        && value.len() <= max_bytes
        && value == value.trim()
        && value.bytes().all(|byte| matches!(byte, 0x20..=0x7e))
}
string_identifier!(
    /// Opaque subscription position. The UI must not interpret its contents.
    Cursor
);
string_identifier!(
    /// Identifier issued by the trusted adapter for a prepared action.
    PreparedActionId
);
string_identifier!(
    /// Stable identifier for an approval request.
    ApprovalId
);
string_identifier!(
    /// Caller-generated key used to make confirmation retries idempotent.
    IdempotencyKey
);
string_identifier!(
    /// Opaque binding issued by the trusted adapter when an action is prepared.
    ///
    /// The UI may retain and echo this value, but must never calculate or alter it.
    OpaquePayloadHash
);

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Revision(pub u64);

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct TimestampMillis(pub i64);

/// The component that originated an envelope.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "component", rename_all = "snake_case")]
pub enum Origin {
    HeptaUi,
    HeptaRuntime,
    BridgeAdapter(String),
    Matrix,
    LocalFixture,
}

impl Origin {
    /// Only the product UI may originate declarative requests.
    pub fn is_request_origin(&self) -> bool {
        matches!(self, Self::HeptaUi)
    }

    /// Only the trusted runtime boundary may produce authoritative updates.
    /// Matrix events and local fixtures are inputs, never Hepta receipts.
    pub fn is_authoritative_update_origin(&self) -> bool {
        matches!(self, Self::HeptaRuntime)
            || matches!(self, Self::BridgeAdapter(component) if !component.trim().is_empty())
    }
}

/// Traceability information retained without exposing raw source payloads.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    pub source: String,
    pub source_entity_id: Option<BridgeEntityId>,
    pub source_revision: Option<Revision>,
    pub observed_at: TimestampMillis,
}

impl Provenance {
    pub fn local(source: impl Into<String>, observed_at: TimestampMillis) -> Self {
        Self {
            source: source.into(),
            source_entity_id: None,
            source_revision: None,
            observed_at,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.source.trim().is_empty()
    }
}

/// Whether human-readable fields have passed the bridge's disclosure boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionStatus {
    /// Sensitive source values were removed or replaced before crossing the bridge.
    Redacted,
    /// The producer asserts that the record contained no sensitive source values.
    NotRequired,
    /// Raw or potentially sensitive values remain. Product UI must not render them.
    Unredacted,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Redaction {
    pub status: RedactionStatus,
    pub policy: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed_fields: Vec<String>,
}

impl Redaction {
    pub fn redacted(policy: impl Into<String>) -> Self {
        Self {
            status: RedactionStatus::Redacted,
            policy: Some(policy.into()),
            removed_fields: Vec::new(),
        }
    }

    pub fn not_required() -> Self {
        Self {
            status: RedactionStatus::NotRequired,
            policy: None,
            removed_fields: Vec::new(),
        }
    }

    pub fn unredacted() -> Self {
        Self {
            status: RedactionStatus::Unredacted,
            policy: None,
            removed_fields: Vec::new(),
        }
    }

    pub fn is_presenter_safe(&self) -> bool {
        matches!(
            self.status,
            RedactionStatus::Redacted | RedactionStatus::NotRequired
        )
    }
}

/// Metadata required on every bridge request and update.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeMetadata {
    pub schema_version: u16,
    pub stable_id: BridgeEntityId,
    pub revision: Revision,
    pub cursor: Option<Cursor>,
    pub timestamp: TimestampMillis,
    pub session_id: SessionId,
    pub correlation_id: CorrelationId,
    pub origin: Origin,
    pub redaction: Redaction,
    pub provenance: Provenance,
}

impl BridgeMetadata {
    #[allow(clippy::too_many_arguments)]
    pub fn current(
        stable_id: impl Into<BridgeEntityId>,
        revision: Revision,
        timestamp: TimestampMillis,
        session_id: impl Into<SessionId>,
        correlation_id: impl Into<CorrelationId>,
        origin: Origin,
        redaction: Redaction,
        provenance: Provenance,
    ) -> Self {
        Self {
            schema_version: HEPTA_BRIDGE_SCHEMA_VERSION,
            stable_id: stable_id.into(),
            revision,
            cursor: None,
            timestamp,
            session_id: session_id.into(),
            correlation_id: correlation_id.into(),
            origin,
            redaction,
            provenance,
        }
    }

    pub fn has_required_ids(&self) -> bool {
        !self.stable_id.is_blank()
            && self.session_id.is_live_transport_safe()
            && self.correlation_id.is_live_transport_safe()
            && self.provenance.is_valid()
    }
}

/// Runtime data is local unless a separate, explicit share decision is represented here.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorPolicy {
    #[default]
    LocalOnly,
    ExplicitShare,
}

/// Explicitly binds a Matrix room view to a Hepta session without conflating the two systems.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationBinding {
    pub matrix_room_id: Option<String>,
    pub hepta_session_id: SessionId,
    pub revision: Revision,
    #[serde(default)]
    pub mirror_policy: MirrorPolicy,
}

impl ConversationBinding {
    pub fn local(hepta_session_id: impl Into<SessionId>, revision: Revision) -> Self {
        Self {
            matrix_room_id: None,
            hepta_session_id: hepta_session_id.into(),
            revision,
            mirror_policy: MirrorPolicy::LocalOnly,
        }
    }

    /// Returns a display-level user preference, never an authorization grant.
    /// A trusted adapter still needs a separately-issued, opaque confirmation.
    pub fn requests_matrix_mirroring(&self) -> bool {
        self.matrix_room_id
            .as_ref()
            .is_some_and(|room_id| !room_id.trim().is_empty())
            && matches!(self.mirror_policy, MirrorPolicy::ExplicitShare)
    }

    pub fn is_valid_for(&self, metadata: &BridgeMetadata) -> bool {
        !self.hepta_session_id.is_blank()
            && self.hepta_session_id == metadata.session_id
            && self.revision == metadata.revision
            && self
                .matrix_room_id
                .as_ref()
                .is_none_or(|room_id| !room_id.trim().is_empty())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum ActionKind {
    CreateTask,
    InvokeTool,
    RespondToApproval,
    Cancel,
    Custom(String),
}

/// A declarative UI intent. It is not permission to execute an action.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionIntent {
    pub intent_id: BridgeEntityId,
    pub kind: ActionKind,
    pub target_id: Option<BridgeEntityId>,
    pub summary: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub arguments: BTreeMap<String, Value>,
}

/// Requests understood by a future trusted adapter.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum BridgeRequestKind {
    Snapshot,
    Subscribe {
        cursor: Option<Cursor>,
    },
    Prepare {
        intent: ActionIntent,
    },
    Confirm {
        prepared_id: PreparedActionId,
        /// Opaque adapter-issued value. The UI only echoes it.
        payload_hash: OpaquePayloadHash,
        policy_revision: Revision,
        idempotency_key: IdempotencyKey,
    },
    Reject {
        approval_id: ApprovalId,
        reason: Option<String>,
    },
    Cancel {
        entity_id: BridgeEntityId,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeRequest {
    pub metadata: BridgeMetadata,
    pub binding: ConversationBinding,
    pub request: BridgeRequestKind,
}

impl BridgeRequest {
    pub fn is_contract_valid(&self) -> bool {
        self.metadata.schema_version == HEPTA_BRIDGE_SCHEMA_VERSION
            && self.metadata.has_required_ids()
            && self.metadata.origin.is_request_origin()
            && self.binding.is_valid_for(&self.metadata)
            && self.request.has_required_fields()
    }
}

impl BridgeRequestKind {
    fn has_required_fields(&self) -> bool {
        match self {
            Self::Snapshot => true,
            Self::Subscribe { cursor } => cursor.as_ref().is_none_or(|value| !value.is_blank()),
            Self::Prepare { intent } => {
                !intent.intent_id.is_blank()
                    && !intent.summary.trim().is_empty()
                    && intent
                        .target_id
                        .as_ref()
                        .is_none_or(|value| !value.is_blank())
                    && match &intent.kind {
                        ActionKind::Custom(value) => !value.trim().is_empty(),
                        _ => true,
                    }
            }
            Self::Confirm {
                prepared_id,
                payload_hash,
                idempotency_key,
                ..
            } => !prepared_id.is_blank() && !payload_hash.is_blank() && !idempotency_key.is_blank(),
            Self::Reject {
                approval_id,
                reason,
            } => {
                !approval_id.is_blank()
                    && reason.as_ref().is_none_or(|value| !value.trim().is_empty())
            }
            Self::Cancel { entity_id } => !entity_id.is_blank(),
        }
    }
}

/// A compact, presentation-oriented view of a canonical Hepta entity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeRecord {
    pub metadata: BridgeMetadata,
    pub state: String,
    pub title: String,
    pub summary: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attributes: BTreeMap<String, Value>,
}

impl BridgeRecord {
    pub fn is_presenter_safe(&self) -> bool {
        self.metadata.schema_version == HEPTA_BRIDGE_SCHEMA_VERSION
            && self.metadata.has_required_ids()
            && self.metadata.redaction.is_presenter_safe()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeSnapshot {
    pub revision: Revision,
    pub cursor: Option<Cursor>,
    pub runtime: Option<BridgeRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tasks: Vec<BridgeRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_invocations: Vec<BridgeRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub approvals: Vec<BridgeRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activities: Vec<BridgeRecord>,
}

impl BridgeSnapshot {
    pub fn is_presenter_safe(&self) -> bool {
        self.runtime
            .iter()
            .chain(self.tasks.iter())
            .chain(self.tool_invocations.iter())
            .chain(self.approvals.iter())
            .chain(self.activities.iter())
            .all(BridgeRecord::is_presenter_safe)
    }
}

/// An authoritative result or an adapter-issued confirmation binding.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum BridgeReceipt {
    Prepared {
        prepared_id: PreparedActionId,
        /// Opaque adapter-issued value. The UI only stores and echoes it.
        payload_hash: OpaquePayloadHash,
        policy_revision: Revision,
        expires_at: Option<TimestampMillis>,
    },
    Accepted {
        entity_id: BridgeEntityId,
        idempotency_key: IdempotencyKey,
    },
    Rejected {
        entity_id: BridgeEntityId,
        code: String,
    },
    Cancelled {
        entity_id: BridgeEntityId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeProblem {
    pub code: String,
    pub user_safe_message: String,
    pub retryable: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum BridgeUpdateKind {
    Snapshot { snapshot: BridgeSnapshot },
    RuntimeChanged { runtime: BridgeRecord },
    TaskUpsert { task: BridgeRecord },
    ToolInvocationUpsert { invocation: BridgeRecord },
    ApprovalUpsert { approval: BridgeRecord },
    ActivityUpsert { activity: BridgeRecord },
    Receipt { receipt: BridgeReceipt },
    Error { problem: BridgeProblem },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeUpdate {
    pub metadata: BridgeMetadata,
    pub binding: ConversationBinding,
    pub update: BridgeUpdateKind,
}

impl BridgeUpdate {
    pub fn is_contract_valid(&self) -> bool {
        if self.metadata.schema_version != HEPTA_BRIDGE_SCHEMA_VERSION
            || !self.metadata.has_required_ids()
            || !self.metadata.origin.is_authoritative_update_origin()
            || !self.binding.is_valid_for(&self.metadata)
        {
            return false;
        }

        let record_is_valid = |record: &BridgeRecord| {
            record.metadata.schema_version == HEPTA_BRIDGE_SCHEMA_VERSION
                && record.metadata.has_required_ids()
                && record.metadata.session_id == self.metadata.session_id
                && record.metadata.correlation_id == self.metadata.correlation_id
                && record.metadata.origin.is_authoritative_update_origin()
        };

        match &self.update {
            BridgeUpdateKind::Snapshot { snapshot } => {
                snapshot.revision == self.metadata.revision
                    && snapshot
                        .runtime
                        .iter()
                        .chain(snapshot.tasks.iter())
                        .chain(snapshot.tool_invocations.iter())
                        .chain(snapshot.approvals.iter())
                        .chain(snapshot.activities.iter())
                        .all(record_is_valid)
            }
            BridgeUpdateKind::RuntimeChanged { runtime } => record_is_valid(runtime),
            BridgeUpdateKind::TaskUpsert { task } => record_is_valid(task),
            BridgeUpdateKind::ToolInvocationUpsert { invocation } => record_is_valid(invocation),
            BridgeUpdateKind::ApprovalUpsert { approval } => record_is_valid(approval),
            BridgeUpdateKind::ActivityUpsert { activity } => record_is_valid(activity),
            BridgeUpdateKind::Receipt { .. } | BridgeUpdateKind::Error { .. } => true,
        }
    }

    pub fn is_presenter_safe(&self) -> bool {
        if !self.is_contract_valid() || !self.metadata.redaction.is_presenter_safe() {
            return false;
        }

        match &self.update {
            BridgeUpdateKind::Snapshot { snapshot } => snapshot.is_presenter_safe(),
            BridgeUpdateKind::RuntimeChanged { runtime } => runtime.is_presenter_safe(),
            BridgeUpdateKind::TaskUpsert { task } => task.is_presenter_safe(),
            BridgeUpdateKind::ToolInvocationUpsert { invocation } => invocation.is_presenter_safe(),
            BridgeUpdateKind::ApprovalUpsert { approval } => approval.is_presenter_safe(),
            BridgeUpdateKind::ActivityUpsert { activity } => activity.is_presenter_safe(),
            BridgeUpdateKind::Receipt { .. } | BridgeUpdateKind::Error { .. } => true,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests_support {
    use super::*;

    pub(crate) fn metadata(id: &str, redaction: Redaction) -> BridgeMetadata {
        BridgeMetadata::current(
            id,
            Revision(3),
            TimestampMillis(1_785_578_400_000),
            "session-7",
            "correlation-11",
            Origin::BridgeAdapter("test".into()),
            redaction,
            Provenance::local("bridge-contract-test", TimestampMillis(1_785_578_399_000)),
        )
    }

    pub(crate) fn request_metadata(id: &str) -> BridgeMetadata {
        BridgeMetadata::current(
            id,
            Revision(3),
            TimestampMillis(1_785_578_400_000),
            "session-7",
            "correlation-11",
            Origin::HeptaUi,
            Redaction::not_required(),
            Provenance::local("hepta-ui-contract-test", TimestampMillis(1_785_578_399_000)),
        )
    }

    pub(crate) fn binding() -> ConversationBinding {
        ConversationBinding::local("session-7", Revision(3))
    }

    pub(crate) fn record(id: &str, redaction: Redaction) -> BridgeRecord {
        BridgeRecord {
            metadata: metadata(id, redaction),
            state: "running".into(),
            title: "Task".into(),
            summary: "A safe summary".into(),
            attributes: BTreeMap::new(),
        }
    }

    pub(crate) fn update(redaction: Redaction) -> BridgeUpdate {
        BridgeUpdate {
            metadata: metadata("update-1", Redaction::not_required()),
            binding: binding(),
            update: BridgeUpdateKind::TaskUpsert {
                task: record("task-1", redaction),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{tests_support::*, *};

    #[test]
    fn request_and_update_round_trip_without_losing_contract_fields() {
        let request = BridgeRequest {
            metadata: request_metadata("request-1"),
            binding: binding(),
            request: BridgeRequestKind::Prepare {
                intent: ActionIntent {
                    intent_id: "intent-1".into(),
                    kind: ActionKind::InvokeTool,
                    target_id: Some("tool-1".into()),
                    summary: "Run the selected tool".into(),
                    arguments: BTreeMap::from([("mode".into(), Value::String("safe".into()))]),
                },
            },
        };

        let request_json = serde_json::to_vec(&request).unwrap();
        assert_eq!(
            serde_json::from_slice::<BridgeRequest>(&request_json).unwrap(),
            request
        );

        let update = update(Redaction::redacted("test-policy"));
        let update_json = serde_json::to_vec(&update).unwrap();
        assert_eq!(
            serde_json::from_slice::<BridgeUpdate>(&update_json).unwrap(),
            update
        );
    }

    #[test]
    fn conversation_binding_is_local_only_by_default() {
        let binding: ConversationBinding = serde_json::from_value(serde_json::json!({
            "matrix_room_id": "!room:example.org",
            "hepta_session_id": "session-7",
            "revision": 1
        }))
        .unwrap();

        assert_eq!(binding.mirror_policy, MirrorPolicy::LocalOnly);
        assert!(!binding.requests_matrix_mirroring());

        let explicit = ConversationBinding {
            mirror_policy: MirrorPolicy::ExplicitShare,
            ..binding
        };
        assert!(explicit.requests_matrix_mirroring());
    }

    #[test]
    fn explicit_share_is_only_a_request_not_an_authority_grant() {
        let binding = ConversationBinding {
            matrix_room_id: Some("!room:example.org".into()),
            hepta_session_id: "session-7".into(),
            revision: Revision(3),
            mirror_policy: MirrorPolicy::ExplicitShare,
        };

        assert!(binding.requests_matrix_mirroring());
        // The bridge contract intentionally exposes no permits/grants method.
        // Authority requires an adapter-issued prepared action and confirmation.
        assert!(binding.is_valid_for(&request_metadata("request-1")));
    }

    #[test]
    fn cross_session_nested_updates_and_untrusted_receipts_are_rejected() {
        let mut cross_session = update(Redaction::redacted("test-policy"));
        let BridgeUpdateKind::TaskUpsert { task } = &mut cross_session.update else {
            unreachable!();
        };
        task.metadata.session_id = "another-session".into();
        assert!(!cross_session.is_contract_valid());

        let mut matrix_receipt = update(Redaction::not_required());
        matrix_receipt.metadata.origin = Origin::Matrix;
        matrix_receipt.update = BridgeUpdateKind::Receipt {
            receipt: BridgeReceipt::Accepted {
                entity_id: "task-1".into(),
                idempotency_key: "idempotency-1".into(),
            },
        };
        assert!(!matrix_receipt.is_contract_valid());

        let mut unnamed_adapter = update(Redaction::redacted("test-policy"));
        unnamed_adapter.metadata.origin = Origin::BridgeAdapter("  ".into());
        assert!(!unnamed_adapter.is_contract_valid());
    }
}

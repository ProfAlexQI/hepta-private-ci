use serde::Deserialize;
use serde::Serialize;

use crate::QualificationError;
use crate::digest::framed_digest;
use crate::request::canonical_json;
use crate::request::valid_dynamic_id;

pub const BROWSER_CONTROL_SCHEMA_VERSION: u32 = 1;
pub const BROWSER_RECEIPT_SCHEMA_VERSION: u32 = 2;
pub const BROWSER_QUALIFICATION_ONLY: bool = true;
pub const BROWSER_PRODUCTION_CALLER: bool = false;
pub const BROWSER_PRODUCTION_WRITER: bool = false;
pub const BROWSER_EFFECT_AUTHORITY: bool = false;
pub const BROWSER_OPERATOR_ACCEPTANCE: bool = false;
pub const BROWSER_PROMOTION: bool = false;
pub const BROWSER_G5_ALLOWED: bool = false;
pub const BROWSER_EXECUTE_ALLOWED: bool = false;
pub const BROWSER_EXTERNAL_EFFECT: bool = false;

const SESSION_ID_DOMAIN: &[u8] = b"hepta.browser.session-id:v1";
const SEMANTIC_REF_DOMAIN: &[u8] = b"hepta.browser.semantic-ref:v1";
const REQUEST_DOMAIN: &[u8] = b"hepta.browser.request:v1";
const SNAPSHOT_DOMAIN: &[u8] = b"hepta.browser.semantic-snapshot:v1";
const OUTCOME_DOMAIN: &[u8] = b"hepta.browser.outcome:v1";
const ACTIVITY_RECEIPT_DOMAIN: &[u8] = b"hepta.browser.activity-receipt:v2";
const EVIDENCE_RECEIPT_DOMAIN: &[u8] = b"hepta.browser.evidence-receipt:v2";
const URL_DOMAIN: &[u8] = b"hepta.browser.url:v1";

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct BrowserSessionId(String);

impl<'de> Deserialize<'de> for BrowserSessionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

impl BrowserSessionId {
    pub fn from_seed(seed: &str) -> Result<Self, QualificationError> {
        if !valid_dynamic_id(seed) {
            return Err(invalid(
                "browser session seed is not a bounded stable identifier",
            ));
        }
        let digest = framed_digest(SESSION_ID_DOMAIN, [seed.as_bytes()]);
        Ok(Self(format!("browser-session:v1:{digest}")))
    }

    pub fn parse(value: impl Into<String>) -> Result<Self, QualificationError> {
        let value = value.into();
        let digest = value
            .strip_prefix("browser-session:v1:")
            .ok_or_else(|| invalid("invalid browser session id prefix"))?;
        if !valid_sha256(digest) {
            return Err(invalid(
                "browser session id digest is not lowercase SHA-256",
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct SemanticRef(String);

impl<'de> Deserialize<'de> for SemanticRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

impl SemanticRef {
    pub(crate) fn derive(
        session_id: &BrowserSessionId,
        page_revision: u64,
        node_key: &str,
    ) -> Result<Self, QualificationError> {
        if node_key.is_empty() || node_key.len() > 256 {
            return Err(invalid("browser semantic node key is empty or too long"));
        }
        let revision = page_revision.to_string();
        let digest = framed_digest(
            SEMANTIC_REF_DOMAIN,
            [
                session_id.as_str().as_bytes(),
                revision.as_bytes(),
                node_key.as_bytes(),
            ],
        );
        Ok(Self(format!("browser-ref:v1:{page_revision}:{digest}")))
    }

    pub fn parse(value: impl Into<String>) -> Result<Self, QualificationError> {
        let value = value.into();
        let mut parts = value.split(':');
        let valid = matches!(parts.next(), Some("browser-ref"))
            && matches!(parts.next(), Some("v1"))
            && parts.next().is_some_and(|revision| {
                revision
                    .parse::<u64>()
                    .is_ok_and(|parsed| parsed.to_string() == revision)
            })
            && parts.next().is_some_and(valid_sha256)
            && parts.next().is_none();
        if !valid {
            return Err(invalid("invalid browser semantic reference"));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserActorKind {
    Agent,
    Human,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserControlMode {
    AgentTurn,
    HumanTurn,
    ChallengePaused,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserCommandKind {
    Navigate,
    Observe,
    Act,
    Wait,
    Extract,
    HumanTakeControl,
    HumanReleaseControl,
    HumanInput,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum BrowserAction {
    Click,
    TypeText { text: String },
    Clear,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum BrowserWaitCondition {
    DocumentReady,
    TextContains { text: String },
    HistoryLengthAtLeast { length: u32 },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum BrowserCommand {
    Navigate {
        url: String,
    },
    Observe {
        max_nodes: u16,
    },
    Act {
        target: SemanticRef,
        action: BrowserAction,
    },
    Wait {
        condition: BrowserWaitCondition,
    },
    Extract {
        query: String,
        max_bytes: u32,
    },
    HumanTakeControl {
        lease_ms: u64,
    },
    HumanReleaseControl,
    HumanInput {
        target: SemanticRef,
        action: BrowserAction,
    },
}

impl BrowserCommand {
    pub fn kind(&self) -> BrowserCommandKind {
        match self {
            Self::Navigate { .. } => BrowserCommandKind::Navigate,
            Self::Observe { .. } => BrowserCommandKind::Observe,
            Self::Act { .. } => BrowserCommandKind::Act,
            Self::Wait { .. } => BrowserCommandKind::Wait,
            Self::Extract { .. } => BrowserCommandKind::Extract,
            Self::HumanTakeControl { .. } => BrowserCommandKind::HumanTakeControl,
            Self::HumanReleaseControl => BrowserCommandKind::HumanReleaseControl,
            Self::HumanInput { .. } => BrowserCommandKind::HumanInput,
        }
    }

    pub fn is_mutation(&self) -> bool {
        matches!(
            self,
            Self::Navigate { .. }
                | Self::Act { .. }
                | Self::HumanTakeControl { .. }
                | Self::HumanReleaseControl
                | Self::HumanInput { .. }
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BrowserRequest {
    pub schema_version: u32,
    pub request_id: u64,
    pub session_id: BrowserSessionId,
    pub actor: BrowserActorKind,
    pub generation: u64,
    pub owner_epoch: u64,
    pub expected_page_revision: u64,
    pub command: BrowserCommand,
}

impl BrowserRequest {
    pub fn new(
        request_id: u64,
        session_id: BrowserSessionId,
        actor: BrowserActorKind,
        generation: u64,
        owner_epoch: u64,
        expected_page_revision: u64,
        command: BrowserCommand,
    ) -> Self {
        Self {
            schema_version: BROWSER_CONTROL_SCHEMA_VERSION,
            request_id,
            session_id,
            actor,
            generation,
            owner_epoch,
            expected_page_revision,
            command,
        }
    }

    pub(crate) fn digest(&self) -> Result<String, QualificationError> {
        let bytes = canonical_json(self)?;
        Ok(framed_digest(REQUEST_DOMAIN, [bytes.as_slice()]))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserDenialCode {
    UnsupportedSchema,
    WrongSession,
    StaleGeneration,
    StaleOwnerEpoch,
    StalePageRevision,
    RequestIdConflict,
    ControlNotOwned,
    InvalidCommand,
    ExternalNavigationDisabled,
    ExternalEffectDisabled,
    SensitiveDataDenied,
    CrossTenantDataDenied,
    StaleSemanticRef,
    ResourceLimit,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserChallengeCode {
    ConsentRequired,
    Captcha,
    RateLimited,
    AntiBot,
    UnsupportedCapability,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserIndeterminateCode {
    EngineTimeout,
    RendererFailure,
    UnknownMutationOutcome,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticNode {
    pub semantic_ref: SemanticRef,
    pub role: String,
    pub name: String,
    pub value: String,
    pub interactive: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticSnapshot {
    pub url: String,
    pub title: String,
    pub page_revision: u64,
    pub nodes: Vec<SemanticNode>,
    pub snapshot_sha256: String,
}

impl SemanticSnapshot {
    pub(crate) fn seal(
        url: String,
        title: String,
        page_revision: u64,
        nodes: Vec<SemanticNode>,
    ) -> Result<Self, QualificationError> {
        let preimage = serde_json::json!({
            "nodes": nodes,
            "page_revision": page_revision,
            "title": title,
            "url": url,
        });
        let bytes = canonical_json(&preimage)?;
        let snapshot_sha256 = framed_digest(SNAPSHOT_DOMAIN, [bytes.as_slice()]);
        let value: SnapshotPreimage = serde_json::from_value(preimage)
            .map_err(|error| QualificationError::Serialization(error.to_string()))?;
        Ok(Self {
            url: value.url,
            title: value.title,
            page_revision: value.page_revision,
            nodes: value.nodes,
            snapshot_sha256,
        })
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotPreimage {
    url: String,
    title: String,
    page_revision: u64,
    nodes: Vec<SemanticNode>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum BrowserOutcome {
    Applied {
        command: BrowserCommandKind,
    },
    Observed {
        snapshot: SemanticSnapshot,
    },
    Extracted {
        query: String,
        value: String,
        truncated: bool,
        snapshot_sha256: String,
    },
    WaitSatisfied {
        condition: BrowserWaitCondition,
    },
    ControlTransferred {
        mode: BrowserControlMode,
    },
    Denied {
        code: BrowserDenialCode,
    },
    Challenge {
        code: BrowserChallengeCode,
    },
    Indeterminate {
        code: BrowserIndeterminateCode,
    },
}

impl BrowserOutcome {
    pub(crate) fn digest(&self) -> Result<String, QualificationError> {
        let bytes = canonical_json(self)?;
        Ok(framed_digest(OUTCOME_DOMAIN, [bytes.as_slice()]))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BrowserAuthorityStatus {
    pub qualification_only: bool,
    pub production_caller: bool,
    pub production_writer: bool,
    pub effect_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub g5_allowed: bool,
    pub execute_allowed: bool,
    pub external_effect: bool,
}

impl Default for BrowserAuthorityStatus {
    fn default() -> Self {
        Self {
            qualification_only: BROWSER_QUALIFICATION_ONLY,
            production_caller: BROWSER_PRODUCTION_CALLER,
            production_writer: BROWSER_PRODUCTION_WRITER,
            effect_authority: BROWSER_EFFECT_AUTHORITY,
            operator_acceptance: BROWSER_OPERATOR_ACCEPTANCE,
            promotion: BROWSER_PROMOTION,
            g5_allowed: BROWSER_G5_ALLOWED,
            execute_allowed: BROWSER_EXECUTE_ALLOWED,
            external_effect: BROWSER_EXTERNAL_EFFECT,
        }
    }
}

impl BrowserAuthorityStatus {
    pub fn is_closed(self) -> bool {
        self.qualification_only
            && !self.production_caller
            && !self.production_writer
            && !self.effect_authority
            && !self.operator_acceptance
            && !self.promotion
            && !self.g5_allowed
            && !self.execute_allowed
            && !self.external_effect
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BrowserActivityReceipt {
    pub schema_version: u32,
    pub request_id: u64,
    pub session_id: BrowserSessionId,
    pub actor: BrowserActorKind,
    pub command: BrowserCommandKind,
    pub request_sha256: String,
    pub outcome_sha256: String,
    pub generation: u64,
    pub owner_epoch_before: u64,
    pub owner_epoch_after: u64,
    pub page_revision_before: u64,
    pub page_revision_after: u64,
    pub authority: BrowserAuthorityStatus,
    pub receipt_sha256: String,
}

impl BrowserActivityReceipt {
    pub(crate) fn seal(
        request: &BrowserRequest,
        actual_session_id: &BrowserSessionId,
        actual_generation: u64,
        request_sha256: String,
        outcome: &BrowserOutcome,
        owner_epoch_before: u64,
        owner_epoch_after: u64,
        page_revision_before: u64,
        page_revision_after: u64,
    ) -> Result<Self, QualificationError> {
        let authority = BrowserAuthorityStatus::default();
        let outcome_sha256 = outcome.digest()?;
        let preimage = serde_json::json!({
            "actor": request.actor,
            "authority": authority,
            "command": request.command.kind(),
            "generation": actual_generation,
            "outcome_sha256": outcome_sha256,
            "owner_epoch_after": owner_epoch_after,
            "owner_epoch_before": owner_epoch_before,
            "page_revision_after": page_revision_after,
            "page_revision_before": page_revision_before,
            "request_id": request.request_id,
            "request_sha256": request_sha256,
            "schema_version": BROWSER_RECEIPT_SCHEMA_VERSION,
            "session_id": actual_session_id,
        });
        let bytes = canonical_json(&preimage)?;
        let receipt_sha256 = framed_digest(ACTIVITY_RECEIPT_DOMAIN, [bytes.as_slice()]);
        let value: ActivityReceiptPreimage = serde_json::from_value(preimage)
            .map_err(|error| QualificationError::Serialization(error.to_string()))?;
        Ok(Self {
            schema_version: value.schema_version,
            request_id: value.request_id,
            session_id: value.session_id,
            actor: value.actor,
            command: value.command,
            request_sha256: value.request_sha256,
            outcome_sha256: value.outcome_sha256,
            generation: value.generation,
            owner_epoch_before: value.owner_epoch_before,
            owner_epoch_after: value.owner_epoch_after,
            page_revision_before: value.page_revision_before,
            page_revision_after: value.page_revision_after,
            authority: value.authority,
            receipt_sha256,
        })
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ActivityReceiptPreimage {
    schema_version: u32,
    request_id: u64,
    session_id: BrowserSessionId,
    actor: BrowserActorKind,
    command: BrowserCommandKind,
    request_sha256: String,
    outcome_sha256: String,
    generation: u64,
    owner_epoch_before: u64,
    owner_epoch_after: u64,
    page_revision_before: u64,
    page_revision_after: u64,
    authority: BrowserAuthorityStatus,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WebEvidenceReceipt {
    pub schema_version: u32,
    pub session_id: BrowserSessionId,
    pub generation: u64,
    pub owner_epoch: u64,
    pub page_revision: u64,
    pub url_sha256: String,
    pub snapshot_sha256: String,
    pub node_count: u32,
    pub raw_secret_bytes_present: bool,
    pub cross_tenant_data_present: bool,
    pub external_effect: bool,
    pub qualification_only: bool,
    pub receipt_sha256: String,
}

impl WebEvidenceReceipt {
    pub(crate) fn seal(
        session_id: &BrowserSessionId,
        generation: u64,
        owner_epoch: u64,
        snapshot: &SemanticSnapshot,
    ) -> Result<Self, QualificationError> {
        let node_count = u32::try_from(snapshot.nodes.len())
            .map_err(|_| invalid("browser semantic snapshot node count overflowed"))?;
        let url_sha256 = framed_digest(URL_DOMAIN, [snapshot.url.as_bytes()]);
        let preimage = serde_json::json!({
            "cross_tenant_data_present": false,
            "external_effect": false,
            "generation": generation,
            "node_count": node_count,
            "owner_epoch": owner_epoch,
            "page_revision": snapshot.page_revision,
            "qualification_only": true,
            "raw_secret_bytes_present": false,
            "schema_version": BROWSER_RECEIPT_SCHEMA_VERSION,
            "session_id": session_id,
            "snapshot_sha256": snapshot.snapshot_sha256,
            "url_sha256": url_sha256,
        });
        let bytes = canonical_json(&preimage)?;
        let receipt_sha256 = framed_digest(EVIDENCE_RECEIPT_DOMAIN, [bytes.as_slice()]);
        let value: EvidenceReceiptPreimage = serde_json::from_value(preimage)
            .map_err(|error| QualificationError::Serialization(error.to_string()))?;
        Ok(Self {
            schema_version: value.schema_version,
            session_id: value.session_id,
            generation: value.generation,
            owner_epoch: value.owner_epoch,
            page_revision: value.page_revision,
            url_sha256: value.url_sha256,
            snapshot_sha256: value.snapshot_sha256,
            node_count: value.node_count,
            raw_secret_bytes_present: value.raw_secret_bytes_present,
            cross_tenant_data_present: value.cross_tenant_data_present,
            external_effect: value.external_effect,
            qualification_only: value.qualification_only,
            receipt_sha256,
        })
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EvidenceReceiptPreimage {
    schema_version: u32,
    session_id: BrowserSessionId,
    generation: u64,
    owner_epoch: u64,
    page_revision: u64,
    url_sha256: String,
    snapshot_sha256: String,
    node_count: u32,
    raw_secret_bytes_present: bool,
    cross_tenant_data_present: bool,
    external_effect: bool,
    qualification_only: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BrowserResponse {
    pub schema_version: u32,
    pub request_id: u64,
    pub session_id: BrowserSessionId,
    pub generation: u64,
    pub owner_epoch: u64,
    pub page_revision: u64,
    pub mode: BrowserControlMode,
    pub outcome: BrowserOutcome,
    pub authority: BrowserAuthorityStatus,
    pub activity_receipt: BrowserActivityReceipt,
    pub evidence_receipt: Option<WebEvidenceReceipt>,
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn invalid(message: impl Into<String>) -> QualificationError {
    QualificationError::Invalid(message.into())
}

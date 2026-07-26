//! Operator-authenticated, phase-bound Telegram delivery authority.
//!
//! The authority is deliberately separate from Telegram transport code. It
//! persists an authenticated append-only state transition before each
//! credentialed read, model invocation, and send. The send transition contains
//! an exact effect plan; a provider-owned acknowledgement must be persisted
//! before the terminal receipt can be published.

use std::collections::HashSet;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

use anyhow::Context;
use anyhow::Result;
use hepta_gateway::telegram_delivery_lifecycle_record;
use hepta_gateway::telegram_next_update_offset;
use hmac::Hmac;
use hmac::Mac;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use zeroize::Zeroizing;

use crate::secure_key_file::read_private_key;
use crate::telegram_durable_files::read_private_state;
use crate::telegram_durable_files::update_private_state_atomically;

pub(crate) const TELEGRAM_AUTHORITY_ENABLED_ENV: &str = "HEPTA_NATIVE_TELEGRAM_OPERATOR_AUTHORITY";
pub(crate) const TELEGRAM_AUTHORITY_KEY_FILE_ENV: &str =
    "HEPTA_NATIVE_TELEGRAM_OPERATOR_AUTH_KEY_FILE";
pub(crate) const TELEGRAM_AUTHORITY_JOURNAL_FILE_ENV: &str =
    "HEPTA_NATIVE_TELEGRAM_AUTHORITY_JOURNAL_FILE";
pub(crate) const TELEGRAM_AUTHORITY_PLAN_ENDPOINT: &str = "/api/v2/telegram/drain/authority/plan";
pub(crate) const TELEGRAM_AUTHORITY_COMMIT_ENDPOINT: &str =
    "/api/v2/telegram/drain/authority/commit";
pub(crate) const TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT: &str =
    "/api/v2/telegram/drain/reconciliation/inspect";
pub(crate) const TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT: &str =
    "/api/v2/telegram/drain/reconciliation/resolve";
pub(crate) const TELEGRAM_PIPELINE_AUTHORITY_OWNER: &str = "TelegramPipelineAuthority";

const SCHEMA: &str = "hepta.telegram.operator-authority.v1";
const CHECKPOINT_SCHEMA: &str = "hepta.telegram.operator-authority.checkpoint.v1";
const MONOTONIC_STATE_SCHEMA: &str = "hepta.telegram.operator-authority.monotonic-state.v1";
const RECONCILIATION_SCHEMA: &str = "hepta.telegram.terminal-reconciliation.v1";
const DELIVERY_ACK_BINDING_SCHEMA: &str = "hepta.telegram.delivery-ack-binding.v1";
const PLAN_PROOF_DOMAIN: &[u8] = b"hepta.telegram.operator-authority.plan.v1";
const COMMIT_PROOF_DOMAIN: &[u8] = b"hepta.telegram.operator-authority.commit.v1";
const RECONCILIATION_PROOF_DOMAIN: &[u8] = b"hepta.telegram.terminal-reconciliation.proof.v1";
const RECONCILED_TERMINAL_DOMAIN: &[u8] = b"hepta.telegram.terminal-reconciliation.receipt.v1";
const DELIVERY_ACK_MAC_DOMAIN: &[u8] = b"hepta.telegram.delivery-ack-binding.mac.v1";
const PLAN_HASH_DOMAIN: &[u8] = b"hepta.telegram.operator-authority.plan-hash.v1";
const EVENT_MAC_DOMAIN: &[u8] = b"hepta.telegram.operator-authority.event-mac.v1";
const EVENT_HASH_DOMAIN: &[u8] = b"hepta.telegram.operator-authority.event-hash.v1";
const CHECKPOINT_MAC_DOMAIN: &[u8] = b"hepta.telegram.operator-authority.checkpoint-mac.v1";
const CHECKPOINT_HASH_DOMAIN: &[u8] = b"hepta.telegram.operator-authority.checkpoint-hash.v1";
const CHECKPOINT_HISTORY_DOMAIN: &[u8] = b"hepta.telegram.operator-authority.checkpoint-history.v1";
const READ_RESULT_DOMAIN: &[u8] = b"hepta.telegram.operator-authority.read-result.v1";
const MODEL_RESULT_DOMAIN: &[u8] = b"hepta.telegram.operator-authority.model-result.v1";
const SEND_PLAN_DOMAIN: &[u8] = b"hepta.telegram.operator-authority.send-plan.v1";
const PROVIDER_ACK_DOMAIN: &[u8] = b"hepta.telegram.operator-authority.provider-ack.v1";
const INTENT_DOMAIN: &[u8] = b"hepta.telegram.operator-authority.intent.v1";
const GENESIS_HASH: &str = "sha256:telegram-authority-genesis";
const MAX_BODY_BYTES: usize = 4096;
const MAX_PROMPT_BYTES: usize = 64 * 1024;
const MAX_REPLY_BYTES: usize = 4096;
const MAX_JOURNAL_BYTES: u64 = 16 * 1024 * 1024;
const MAX_DELIVERY_LEDGER_BYTES: u64 = 16 * 1024 * 1024;
const MAX_JOURNAL_EVENTS: usize = 4096;
const MAX_CHECKPOINTED_AUTHORITIES: usize = 20_000;
const RETAIN_TERMINAL_PIPELINES: usize = 128;
const MAX_EVENT_BYTES: usize = 8192;
const MAX_CHECKPOINT_BYTES: usize = 4 * 1024 * 1024;
const CHECKPOINT_GENESIS_HASH: &str = "sha256:telegram-authority-checkpoint-genesis";
const JOURNAL_STAGING_PREFIX: &str = ".hepta-telegram-authority-journal";
#[cfg(test)]
const PRIVATE_FILE_MODE: u32 = 0o600;
const PRIVATE_DIRECTORY_MODE: u32 = 0o700;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub(crate) struct TelegramAuthorityConfig {
    journal_file: PathBuf,
    key_file: PathBuf,
}

impl TelegramAuthorityConfig {
    pub(crate) fn from_env() -> Result<Option<Self>> {
        Self::from_lookup(|name| env::var_os(name))
    }

    fn from_lookup(mut lookup: impl FnMut(&str) -> Option<OsString>) -> Result<Option<Self>> {
        let enabled = lookup(TELEGRAM_AUTHORITY_ENABLED_ENV)
            .and_then(|value| value.into_string().ok())
            .is_some_and(|value| {
                matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes" | "on"
                )
            });
        let key_file = lookup(TELEGRAM_AUTHORITY_KEY_FILE_ENV).filter(|value| !value.is_empty());
        let journal_file =
            lookup(TELEGRAM_AUTHORITY_JOURNAL_FILE_ENV).filter(|value| !value.is_empty());
        if !enabled {
            if key_file.is_some() || journal_file.is_some() {
                anyhow::bail!(
                    "Telegram authority key/journal configuration requires {TELEGRAM_AUTHORITY_ENABLED_ENV}=1"
                );
            }
            return Ok(None);
        }
        let key_file = PathBuf::from(key_file.context("Telegram authority key file is required")?);
        let journal_file =
            PathBuf::from(journal_file.context("Telegram authority journal file is required")?);
        if !key_file.is_absolute() || !journal_file.is_absolute() || key_file == journal_file {
            anyhow::bail!("Telegram authority key and journal must be distinct absolute paths");
        }
        Ok(Some(Self {
            journal_file,
            key_file,
        }))
    }

    #[cfg(all(test, unix))]
    pub(crate) fn for_test(root: &Path) -> Result<Self> {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(root, fs::Permissions::from_mode(PRIVATE_DIRECTORY_MODE))?;
        let key_file = root.join("telegram-authority.key");
        fs::write(
            &key_file,
            b"404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f",
        )?;
        fs::set_permissions(&key_file, fs::Permissions::from_mode(PRIVATE_FILE_MODE))?;
        Ok(Self {
            journal_file: root.join("telegram-authority.jsonl"),
            key_file,
        })
    }
}

pub(crate) struct TelegramAuthority {
    journal_file: PathBuf,
    key: Zeroizing<[u8; 32]>,
    process_lock: Mutex<()>,
}

impl std::fmt::Debug for TelegramAuthority {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TelegramAuthority")
            .field("journal_file", &self.journal_file)
            .field("key", &"[REDACTED]")
            .finish()
    }
}

#[derive(Debug, Clone)]
struct JournalSnapshot {
    checkpoint: Option<JournalCheckpoint>,
    events: Vec<JournalEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct JournalCheckpoint {
    schema: String,
    revision: u64,
    sequence: u64,
    previous_checkpoint_hash: String,
    compacted_events: u64,
    consumed_authorities: Vec<ConsumedAuthority>,
    history_hash: String,
    mac: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConsumedAuthority {
    request_id: String,
    plan_hash: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PlanRequest {
    request_id: String,
    cursor: Option<i64>,
    proof: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CommitRequest {
    request_id: String,
    plan_hash: String,
    plan_request_binding_hash: String,
    proof: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct TelegramReconciliationRequest {
    request_id: String,
    plan_hash: String,
    plan_request_binding_hash: String,
    commit_request_binding_hash: String,
    session_binding_hash: String,
    cursor: Option<i64>,
    update_id: i64,
    next_update_offset: i64,
    effect_plan_hash: String,
    provider_ack_hash: String,
    #[serde(default)]
    decision: Option<TelegramReconciliationDecision>,
    proof: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum TelegramReconciliationDecision {
    CompleteTerminalReceiptOnly,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthenticatedDeliveryAckBinding {
    schema: String,
    request_id: String,
    plan_hash: String,
    plan_request_binding_hash: String,
    commit_request_binding_hash: String,
    session_binding_hash: String,
    cursor: Option<i64>,
    update_id: i64,
    next_update_offset: i64,
    effect_plan_hash: String,
    provider_ack_hash: String,
    delivery_ledger_path_hash: String,
    cursor_path_hash: String,
    mac: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct TelegramAuthorityPlanReceipt {
    schema: &'static str,
    status: &'static str,
    request_id: String,
    plan_hash: String,
    plan_request_binding_hash: String,
    session_binding_hash: String,
    cursor: Option<i64>,
    operator_authenticated: bool,
    live_read_authorized: bool,
    model_invocation_authorized: bool,
    send_authorized: bool,
    durable_intent_recorded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct TelegramAuthorityCommitReceipt {
    schema: &'static str,
    status: &'static str,
    request_id: String,
    plan_hash: String,
    plan_request_binding_hash: String,
    commit_request_binding_hash: String,
    session_binding_hash: String,
    cursor: Option<i64>,
    owner_binding_hash: String,
    operator_authenticated: bool,
    single_owner_permit_issued: bool,
    external_effect_started: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct TelegramAuthorityMonotonicState {
    pub(crate) schema: &'static str,
    pub(crate) authority_owner: &'static str,
    pub(crate) journal_sequence: u64,
    pub(crate) latest_event_hash: String,
    pub(crate) latest_event_mac: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlanBinding {
    request_id: String,
    plan_hash: String,
    plan_request_binding_hash: String,
    commit_request_binding_hash: Option<String>,
    session_binding_hash: String,
    cursor: Option<i64>,
}

pub(crate) struct TelegramPipelinePermit<'a> {
    authority: &'a TelegramAuthority,
    binding: PlanBinding,
    owner_nonce: String,
    finished: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TelegramReadRequest {
    pub(crate) session_binding_hash: String,
    pub(crate) request_binding_hash: String,
    pub(crate) cursor: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TelegramReadResult {
    pub(crate) update_id: i64,
    pub(crate) chat_id: i64,
    pub(crate) reply_to_message_id: Option<i64>,
    pub(crate) prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TelegramModelRequest {
    pub(crate) session_binding_hash: String,
    pub(crate) request_binding_hash: String,
    pub(crate) update_id: i64,
    pub(crate) prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TelegramSendPlan {
    pub(crate) update_id: i64,
    pub(crate) next_update_offset: i64,
    pub(crate) chat_id: i64,
    pub(crate) reply_to_message_id: Option<i64>,
    pub(crate) message_text: String,
    pub(crate) effect_plan_hash: String,
    pub(crate) idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TelegramProviderAck {
    pub(crate) provider: String,
    pub(crate) provider_message_id: i64,
    pub(crate) chat_id: i64,
    pub(crate) raw_response_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct TelegramPipelineReceipt {
    pub(crate) schema: &'static str,
    pub(crate) status: &'static str,
    pub(crate) durable_intent_owner: &'static str,
    pub(crate) request_id: String,
    pub(crate) plan_hash: String,
    pub(crate) session_binding_hash: String,
    pub(crate) plan_request_binding_hash: String,
    pub(crate) commit_request_binding_hash: String,
    pub(crate) read_intent_hash: String,
    pub(crate) read_result_hash: String,
    pub(crate) model_intent_hash: String,
    pub(crate) model_result_hash: String,
    pub(crate) effect_plan_hash: String,
    pub(crate) provider_effect_ack_hash: String,
    pub(crate) terminal_receipt_hash: String,
    pub(crate) update_id: i64,
    pub(crate) next_update_offset: i64,
    pub(crate) live_read_authorized: bool,
    pub(crate) model_invocation_authorized: bool,
    pub(crate) send_authorized: bool,
    pub(crate) durable_intent_recorded: bool,
    pub(crate) provider_effect_ack_recorded: bool,
    pub(crate) delivery_ack_recorded: bool,
    pub(crate) cursor_written: bool,
    pub(crate) terminal_receipt_recorded: bool,
}

#[derive(Debug, Serialize)]
struct TelegramReconciliationResponse<'a> {
    schema: &'static str,
    authority: &'static str,
    operation: &'static str,
    result: &'static str,
    request_binding_hash: &'a str,
    request_id: &'a str,
    plan_hash: &'a str,
    plan_request_binding_hash: &'a str,
    commit_request_binding_hash: &'a str,
    session_binding_hash: &'a str,
    cursor: Option<i64>,
    update_id: i64,
    next_update_offset: i64,
    effect_plan_hash: &'a str,
    provider_ack_hash: &'a str,
    terminal_receipt_hash: Option<&'a str>,
    provider_replayed: bool,
    read_replayed: bool,
    model_replayed: bool,
    cursor_written: bool,
    cursor_advanced: bool,
    terminal_receipt_recorded: bool,
}

pub(crate) struct TelegramReconciliationHttpResponse {
    pub(crate) status: &'static str,
    pub(crate) body: String,
    pub(crate) outcome_state_changed: bool,
}

struct TelegramReconciliationOutcome {
    result: &'static str,
    terminal_receipt_hash: Option<String>,
    state_changed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Phase {
    Planned,
    Authorized,
    ReadIntent,
    ReadCompleted,
    ModelIntent,
    ModelCompleted,
    SendIntent,
    SendAcknowledged,
    TerminalSucceeded,
    ReconciledTerminalSucceeded,
    InDoubt,
}

impl Phase {
    fn permits(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Planned, Self::Authorized)
                | (Self::Authorized, Self::ReadIntent | Self::InDoubt)
                | (Self::ReadIntent, Self::ReadCompleted | Self::InDoubt)
                | (Self::ReadCompleted, Self::ModelIntent | Self::InDoubt)
                | (Self::ModelIntent, Self::ModelCompleted | Self::InDoubt)
                | (Self::ModelCompleted, Self::SendIntent | Self::InDoubt)
                | (Self::SendIntent, Self::SendAcknowledged | Self::InDoubt)
                | (
                    Self::SendAcknowledged,
                    Self::TerminalSucceeded | Self::ReconciledTerminalSucceeded | Self::InDoubt
                )
        )
    }

    fn is_success_terminal(self) -> bool {
        matches!(
            self,
            Self::TerminalSucceeded | Self::ReconciledTerminalSucceeded
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct JournalEvent {
    schema: String,
    sequence: u64,
    previous_entry_hash: String,
    phase: Phase,
    request_id: String,
    plan_hash: String,
    plan_request_binding_hash: String,
    commit_request_binding_hash: Option<String>,
    session_binding_hash: String,
    cursor: Option<i64>,
    owner_nonce: Option<String>,
    update_id: Option<i64>,
    next_update_offset: Option<i64>,
    read_result_hash: Option<String>,
    model_result_hash: Option<String>,
    effect_plan_hash: Option<String>,
    provider_ack_hash: Option<String>,
    terminal_receipt_hash: Option<String>,
    mac: String,
}

#[derive(Default)]
struct PhaseEvidence {
    update_id: Option<i64>,
    next_update_offset: Option<i64>,
    read_result_hash: Option<String>,
    model_result_hash: Option<String>,
    effect_plan_hash: Option<String>,
    provider_ack_hash: Option<String>,
    terminal_receipt_hash: Option<String>,
}

fn validate_evidence_transition(
    latest: &JournalEvent,
    phase: Phase,
    evidence: &PhaseEvidence,
) -> Result<()> {
    let empty = evidence.update_id.is_none()
        && evidence.next_update_offset.is_none()
        && evidence.read_result_hash.is_none()
        && evidence.model_result_hash.is_none()
        && evidence.effect_plan_hash.is_none()
        && evidence.provider_ack_hash.is_none()
        && evidence.terminal_receipt_hash.is_none();
    let valid = match phase {
        Phase::Authorized | Phase::ReadIntent => empty,
        Phase::ReadCompleted => {
            evidence.update_id.is_some()
                && evidence.next_update_offset.is_some()
                && evidence.read_result_hash.is_some()
                && evidence.model_result_hash.is_none()
                && evidence.effect_plan_hash.is_none()
                && evidence.provider_ack_hash.is_none()
                && evidence.terminal_receipt_hash.is_none()
        }
        Phase::ModelIntent => {
            evidence.update_id == latest.update_id
                && evidence.update_id.is_some()
                && evidence.next_update_offset == latest.next_update_offset
                && evidence.next_update_offset.is_some()
                && evidence.read_result_hash == latest.read_result_hash
                && evidence.read_result_hash.is_some()
                && evidence.model_result_hash.is_none()
                && evidence.effect_plan_hash.is_none()
                && evidence.provider_ack_hash.is_none()
                && evidence.terminal_receipt_hash.is_none()
        }
        Phase::ModelCompleted => {
            evidence.update_id == latest.update_id
                && evidence.next_update_offset == latest.next_update_offset
                && evidence.read_result_hash == latest.read_result_hash
                && evidence.model_result_hash.is_some()
                && evidence.effect_plan_hash.is_none()
                && evidence.provider_ack_hash.is_none()
                && evidence.terminal_receipt_hash.is_none()
        }
        Phase::SendIntent => {
            evidence.update_id == latest.update_id
                && evidence.next_update_offset == latest.next_update_offset
                && evidence.read_result_hash == latest.read_result_hash
                && evidence.model_result_hash == latest.model_result_hash
                && evidence.effect_plan_hash.is_some()
                && evidence.provider_ack_hash.is_none()
                && evidence.terminal_receipt_hash.is_none()
        }
        Phase::SendAcknowledged => {
            evidence.update_id == latest.update_id
                && evidence.next_update_offset == latest.next_update_offset
                && evidence.read_result_hash == latest.read_result_hash
                && evidence.model_result_hash == latest.model_result_hash
                && evidence.effect_plan_hash == latest.effect_plan_hash
                && evidence.provider_ack_hash.is_some()
                && evidence.terminal_receipt_hash.is_none()
        }
        Phase::TerminalSucceeded | Phase::ReconciledTerminalSucceeded => {
            evidence.update_id == latest.update_id
                && evidence.next_update_offset == latest.next_update_offset
                && evidence.read_result_hash == latest.read_result_hash
                && evidence.model_result_hash == latest.model_result_hash
                && evidence.effect_plan_hash == latest.effect_plan_hash
                && evidence.provider_ack_hash == latest.provider_ack_hash
                && evidence.terminal_receipt_hash.is_some()
        }
        Phase::InDoubt => empty,
        Phase::Planned => false,
    };
    if !valid {
        anyhow::bail!("Telegram authority phase evidence is missing, stale, or substituted");
    }
    Ok(())
}

impl TelegramAuthority {
    pub(crate) fn open(config: TelegramAuthorityConfig) -> Result<Self> {
        validate_private_parent(&config.journal_file)?;
        let key = read_private_key(
            &config.key_file,
            TELEGRAM_AUTHORITY_KEY_FILE_ENV,
            "Telegram operator authority",
        )?;
        let authority = Self {
            journal_file: config.journal_file,
            key,
            process_lock: Mutex::new(()),
        };
        authority.inspect_events()?;
        Ok(authority)
    }

    pub(crate) fn prevalidate_plan(
        &self,
        body: Option<&str>,
        request_binding_hash: &str,
        session_binding_hash: &str,
        current_cursor: Option<i64>,
    ) -> Result<()> {
        let binding = self.validated_plan_binding(
            body,
            request_binding_hash,
            session_binding_hash,
            current_cursor,
        )?;
        self.with_locked_events(|snapshot| {
            if snapshot.events.len() >= MAX_JOURNAL_EVENTS
                && compactable_terminal_pipeline_count(snapshot) == 0
            {
                anyhow::bail!("Telegram authority journal reached its bounded event limit");
            }
            if snapshot.events.iter().any(|event| {
                event.plan_hash == binding.plan_hash || event.request_id == binding.request_id
            }) || authority_consumed(snapshot, &binding.request_id, &binding.plan_hash)
            {
                anyhow::bail!("Telegram authority request or plan was already recorded");
            }
            Ok(())
        })
    }

    pub(crate) fn prevalidate_authorize(
        &self,
        body: Option<&str>,
        commit_request_binding_hash: &str,
        current_session_binding_hash: &str,
        current_cursor: Option<i64>,
    ) -> Result<()> {
        let request = self.validated_commit_request(
            body,
            commit_request_binding_hash,
            current_session_binding_hash,
        )?;
        let binding = self.exact_planned_binding(
            &request.request_id,
            &request.plan_hash,
            &request.plan_request_binding_hash,
        )?;
        if binding.session_binding_hash != current_session_binding_hash
            || binding.cursor != current_cursor
        {
            anyhow::bail!("Telegram authority plan became stale before authorization");
        }
        self.with_locked_events(|snapshot| {
            if snapshot.events.len() >= MAX_JOURNAL_EVENTS
                && compactable_terminal_pipeline_count(snapshot) == 0
            {
                anyhow::bail!("Telegram authority journal reached its bounded event limit");
            }
            if snapshot.events.iter().any(|event| {
                event.plan_hash != binding.plan_hash
                    && event.cursor == binding.cursor
                    && event.phase != Phase::Planned
            }) {
                anyhow::bail!(
                    "Telegram cursor already has an authorized or in-doubt pipeline owner"
                );
            }
            Ok(())
        })
    }

    pub(crate) fn plan(
        &self,
        body: Option<&str>,
        request_binding_hash: &str,
        session_binding_hash: &str,
        current_cursor: Option<i64>,
    ) -> Result<TelegramAuthorityPlanReceipt> {
        let binding = self.validated_plan_binding(
            body,
            request_binding_hash,
            session_binding_hash,
            current_cursor,
        )?;
        self.append_new_plan(&binding)?;
        Ok(TelegramAuthorityPlanReceipt {
            schema: SCHEMA,
            status: "planned",
            request_id: binding.request_id,
            plan_hash: binding.plan_hash,
            plan_request_binding_hash: request_binding_hash.to_owned(),
            session_binding_hash: session_binding_hash.to_owned(),
            cursor: binding.cursor,
            operator_authenticated: true,
            live_read_authorized: false,
            model_invocation_authorized: false,
            send_authorized: false,
            durable_intent_recorded: false,
        })
    }

    pub(crate) fn authorize<'a>(
        &'a self,
        body: Option<&str>,
        commit_request_binding_hash: &str,
        current_session_binding_hash: &str,
        current_cursor: Option<i64>,
    ) -> Result<(TelegramAuthorityCommitReceipt, TelegramPipelinePermit<'a>)> {
        let request = self.validated_commit_request(
            body,
            commit_request_binding_hash,
            current_session_binding_hash,
        )?;
        let mut binding = self.exact_planned_binding(
            &request.request_id,
            &request.plan_hash,
            &request.plan_request_binding_hash,
        )?;
        if binding.session_binding_hash != current_session_binding_hash
            || binding.cursor != current_cursor
        {
            anyhow::bail!("Telegram authority plan became stale before authorization");
        }
        binding.commit_request_binding_hash = Some(commit_request_binding_hash.to_owned());
        let owner_nonce = random_owner_nonce()?;
        self.append_transition(
            &binding,
            &owner_nonce,
            Phase::Authorized,
            PhaseEvidence::default(),
        )?;
        let owner_binding_hash =
            digest(INTENT_DOMAIN, &[&binding.plan_hash, &owner_nonce, "owner"]);
        let receipt = TelegramAuthorityCommitReceipt {
            schema: SCHEMA,
            status: "authorized",
            request_id: binding.request_id.clone(),
            plan_hash: binding.plan_hash.clone(),
            plan_request_binding_hash: binding.plan_request_binding_hash.clone(),
            commit_request_binding_hash: commit_request_binding_hash.to_owned(),
            session_binding_hash: binding.session_binding_hash.clone(),
            cursor: binding.cursor,
            owner_binding_hash,
            operator_authenticated: true,
            single_owner_permit_issued: true,
            external_effect_started: false,
        };
        Ok((
            receipt,
            TelegramPipelinePermit {
                authority: self,
                binding,
                owner_nonce,
                finished: false,
            },
        ))
    }

    fn validated_plan_binding(
        &self,
        body: Option<&str>,
        request_binding_hash: &str,
        session_binding_hash: &str,
        current_cursor: Option<i64>,
    ) -> Result<PlanBinding> {
        validate_hash(request_binding_hash, "plan request binding")?;
        validate_hash(session_binding_hash, "session binding")?;
        let request: PlanRequest =
            serde_json::from_str(bounded_body(body)?).context("decode Telegram authority plan")?;
        validate_hash(&request.request_id, "request id")?;
        validate_cursor(request.cursor)?;
        if request.cursor != current_cursor {
            anyhow::bail!("Telegram authority plan cursor is stale");
        }
        verify_proof(
            &self.key,
            PLAN_PROOF_DOMAIN,
            &[
                &request.request_id,
                &cursor_binding(request.cursor),
                request_binding_hash,
                session_binding_hash,
            ],
            &request.proof,
        )?;
        Ok(PlanBinding {
            request_id: request.request_id.clone(),
            plan_hash: digest(
                PLAN_HASH_DOMAIN,
                &[
                    &request.request_id,
                    request_binding_hash,
                    session_binding_hash,
                    &cursor_binding(request.cursor),
                ],
            ),
            plan_request_binding_hash: request_binding_hash.to_owned(),
            commit_request_binding_hash: None,
            session_binding_hash: session_binding_hash.to_owned(),
            cursor: request.cursor,
        })
    }

    fn validated_commit_request(
        &self,
        body: Option<&str>,
        commit_request_binding_hash: &str,
        current_session_binding_hash: &str,
    ) -> Result<CommitRequest> {
        validate_hash(commit_request_binding_hash, "commit request binding")?;
        validate_hash(current_session_binding_hash, "current session binding")?;
        let request: CommitRequest = serde_json::from_str(bounded_body(body)?)
            .context("decode Telegram authority commit")?;
        for (value, name) in [
            (request.request_id.as_str(), "request id"),
            (request.plan_hash.as_str(), "plan hash"),
            (
                request.plan_request_binding_hash.as_str(),
                "plan request binding",
            ),
        ] {
            validate_hash(value, name)?;
        }
        verify_proof(
            &self.key,
            COMMIT_PROOF_DOMAIN,
            &[
                &request.request_id,
                &request.plan_hash,
                &request.plan_request_binding_hash,
                commit_request_binding_hash,
                current_session_binding_hash,
            ],
            &request.proof,
        )?;
        Ok(request)
    }

    fn exact_planned_binding(
        &self,
        request_id: &str,
        plan_hash: &str,
        plan_request_binding_hash: &str,
    ) -> Result<PlanBinding> {
        let events = self.inspect_events()?;
        let matching = events
            .iter()
            .filter(|event| event.plan_hash == plan_hash)
            .collect::<Vec<_>>();
        let [event] = matching.as_slice() else {
            anyhow::bail!("Telegram authority plan is missing, ambiguous, or already replayed");
        };
        if event.phase != Phase::Planned
            || event.request_id != request_id
            || event.plan_request_binding_hash != plan_request_binding_hash
        {
            anyhow::bail!("Telegram authority plan binding is unavailable or stale");
        }
        Ok(PlanBinding {
            request_id: event.request_id.clone(),
            plan_hash: event.plan_hash.clone(),
            plan_request_binding_hash: event.plan_request_binding_hash.clone(),
            commit_request_binding_hash: None,
            session_binding_hash: event.session_binding_hash.clone(),
            cursor: event.cursor,
        })
    }

    fn append_new_plan(&self, binding: &PlanBinding) -> Result<()> {
        self.update_locked_events(|snapshot| {
            compact_if_needed(snapshot, &self.key)?;
            if snapshot.events.iter().any(|event| {
                event.plan_hash == binding.plan_hash || event.request_id == binding.request_id
            }) || authority_consumed(snapshot, &binding.request_id, &binding.plan_hash)
            {
                anyhow::bail!("Telegram authority request or plan was already recorded");
            }
            let event = event_for(
                snapshot,
                binding,
                None,
                Phase::Planned,
                PhaseEvidence::default(),
            );
            append_event(snapshot, event, &self.key).map(|_| ())
        })
    }

    fn append_transition(
        &self,
        binding: &PlanBinding,
        owner_nonce: &str,
        phase: Phase,
        evidence: PhaseEvidence,
    ) -> Result<JournalEvent> {
        self.update_locked_events(|snapshot| {
            compact_if_needed(snapshot, &self.key)?;
            let latest = snapshot
                .events
                .iter()
                .rev()
                .find(|event| event.plan_hash == binding.plan_hash)
                .context("Telegram authority plan disappeared")?;
            if !latest.phase.permits(phase)
                || latest.request_id != binding.request_id
                || latest.session_binding_hash != binding.session_binding_hash
                || latest.cursor != binding.cursor
                || latest.plan_request_binding_hash != binding.plan_request_binding_hash
            {
                anyhow::bail!("Telegram authority phase order or exact binding is invalid");
            }
            if latest.phase == Phase::Planned {
                if latest.owner_nonce.is_some()
                    || binding.commit_request_binding_hash.is_none()
                    || latest.commit_request_binding_hash.is_some()
                {
                    anyhow::bail!("Telegram authority owner minting boundary is invalid");
                }
                if phase == Phase::Authorized
                    && snapshot.events.iter().any(|event| {
                        event.plan_hash != binding.plan_hash
                            && event.cursor == binding.cursor
                            && event.phase != Phase::Planned
                    })
                {
                    anyhow::bail!(
                        "Telegram cursor already has an authorized or in-doubt pipeline owner"
                    );
                }
            } else if latest.owner_nonce.as_deref() != Some(owner_nonce)
                || latest.commit_request_binding_hash != binding.commit_request_binding_hash
            {
                anyhow::bail!("Telegram authority phase owner is stale or mismatched");
            }
            if phase == Phase::ReadCompleted {
                let update_id = evidence
                    .update_id
                    .context("Telegram ReadCompleted transition lacks an update claim")?;
                if snapshot.events.iter().any(|event| {
                    event.plan_hash != binding.plan_hash && event.update_id == Some(update_id)
                }) {
                    anyhow::bail!("Telegram update is already claimed by another authority plan");
                }
            }
            validate_evidence_transition(latest, phase, &evidence)?;
            let event = event_for(snapshot, binding, Some(owner_nonce), phase, evidence);
            append_event(snapshot, event, &self.key)
        })
    }

    fn latest_owned_phase(&self, binding: &PlanBinding, owner_nonce: &str) -> Result<Phase> {
        self.with_locked_events(|snapshot| {
            let latest = snapshot
                .events
                .iter()
                .rev()
                .find(|event| event.plan_hash == binding.plan_hash)
                .context("Telegram authority plan disappeared")?;
            if latest.request_id != binding.request_id
                || latest.session_binding_hash != binding.session_binding_hash
                || latest.cursor != binding.cursor
                || latest.plan_request_binding_hash != binding.plan_request_binding_hash
                || latest.commit_request_binding_hash != binding.commit_request_binding_hash
                || latest.owner_nonce.as_deref() != Some(owner_nonce)
            {
                anyhow::bail!("Telegram authority phase owner is stale or mismatched");
            }
            Ok(latest.phase)
        })
    }

    fn inspect_events(&self) -> Result<Vec<JournalEvent>> {
        self.with_locked_events(|snapshot| Ok(snapshot.events.clone()))
    }

    /// Returns the canonical state that an independently stored monotonic
    /// anchor must bind. No anchor path or value is accepted from ingress.
    pub(crate) fn monotonic_state(&self) -> Result<TelegramAuthorityMonotonicState> {
        self.with_locked_events(|snapshot| {
            let (sequence, hash, mac) = snapshot.monotonic_binding();
            Ok(TelegramAuthorityMonotonicState {
                schema: MONOTONIC_STATE_SCHEMA,
                authority_owner: TELEGRAM_PIPELINE_AUTHORITY_OWNER,
                journal_sequence: sequence,
                latest_event_hash: hash,
                latest_event_mac: mac,
            })
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn prevalidate_reconciliation_resolve_http(
        &self,
        body: Option<&str>,
        request_binding_hash: &str,
        expected_session_binding_hash: &str,
        delivery_ledger_path: &Path,
        cursor_path: &Path,
    ) -> Option<TelegramReconciliationHttpResponse> {
        let request: TelegramReconciliationRequest =
            match serde_json::from_str(bounded_body(body).unwrap_or_default()) {
                Ok(request) => request,
                Err(_) => {
                    return Some(reconciliation_error(
                        "400 Bad Request",
                        "telegram_terminal_reconciliation.body_invalid",
                    ));
                }
            };
        if request.decision != Some(TelegramReconciliationDecision::CompleteTerminalReceiptOnly) {
            return Some(reconciliation_error(
                "422 Unprocessable Entity",
                "telegram_terminal_reconciliation.decision_invalid",
            ));
        }
        if request.session_binding_hash != expected_session_binding_hash
            || validate_reconciliation_request(&request, request_binding_hash).is_err()
        {
            return Some(reconciliation_error(
                "403 Forbidden",
                "telegram_terminal_reconciliation.binding_invalid",
            ));
        }
        let cursor_binding = cursor_binding(request.cursor);
        let update_id = request.update_id.to_string();
        let next_update_offset = request.next_update_offset.to_string();
        if verify_proof(
            &self.key,
            RECONCILIATION_PROOF_DOMAIN,
            &[
                "POST",
                TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
                request_binding_hash,
                &request.request_id,
                &request.plan_hash,
                &request.plan_request_binding_hash,
                &request.commit_request_binding_hash,
                &request.session_binding_hash,
                &cursor_binding,
                &update_id,
                &next_update_offset,
                &request.effect_plan_hash,
                &request.provider_ack_hash,
                "complete_terminal_receipt_only",
            ],
            &request.proof,
        )
        .is_err()
        {
            return Some(reconciliation_error(
                "403 Forbidden",
                "telegram_terminal_reconciliation.authentication_denied",
            ));
        }
        if self
            .reconcile_terminal_only(&request, delivery_ledger_path, cursor_path, false)
            .is_err()
        {
            return Some(reconciliation_error(
                "409 Conflict",
                "telegram_terminal_reconciliation.evidence_incomplete_or_mismatched",
            ));
        }
        None
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn route_reconciliation_http(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
        request_binding_hash: &str,
        expected_session_binding_hash: &str,
        delivery_ledger_path: &Path,
        cursor_path: &Path,
    ) -> Option<TelegramReconciliationHttpResponse> {
        if !matches!(
            path,
            TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT | TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT
        ) {
            return None;
        }
        if method != "POST" {
            return Some(reconciliation_error(
                "405 Method Not Allowed",
                "telegram_terminal_reconciliation.method_not_allowed",
            ));
        }
        let request: TelegramReconciliationRequest =
            match serde_json::from_str(bounded_body(body).unwrap_or_default()) {
                Ok(request) => request,
                Err(_) => {
                    return Some(reconciliation_error(
                        "400 Bad Request",
                        "telegram_terminal_reconciliation.body_invalid",
                    ));
                }
            };
        let resolve = match (path, request.decision) {
            (TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT, None) => false,
            (
                TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
                Some(TelegramReconciliationDecision::CompleteTerminalReceiptOnly),
            ) => true,
            _ => {
                return Some(reconciliation_error(
                    "422 Unprocessable Entity",
                    "telegram_terminal_reconciliation.decision_invalid",
                ));
            }
        };
        if request.session_binding_hash != expected_session_binding_hash
            || validate_reconciliation_request(&request, request_binding_hash).is_err()
        {
            return Some(reconciliation_error(
                "403 Forbidden",
                "telegram_terminal_reconciliation.binding_invalid",
            ));
        }
        let cursor_binding = cursor_binding(request.cursor);
        let update_id = request.update_id.to_string();
        let next_update_offset = request.next_update_offset.to_string();
        let decision = if resolve {
            "complete_terminal_receipt_only"
        } else {
            "inspect_only"
        };
        if verify_proof(
            &self.key,
            RECONCILIATION_PROOF_DOMAIN,
            &[
                method,
                path,
                request_binding_hash,
                &request.request_id,
                &request.plan_hash,
                &request.plan_request_binding_hash,
                &request.commit_request_binding_hash,
                &request.session_binding_hash,
                &cursor_binding,
                &update_id,
                &next_update_offset,
                &request.effect_plan_hash,
                &request.provider_ack_hash,
                decision,
            ],
            &request.proof,
        )
        .is_err()
        {
            return Some(reconciliation_error(
                "403 Forbidden",
                "telegram_terminal_reconciliation.authentication_denied",
            ));
        }
        let outcome = match self.reconcile_terminal_only(
            &request,
            delivery_ledger_path,
            cursor_path,
            resolve,
        ) {
            Ok(outcome) => outcome,
            Err(_) => {
                return Some(reconciliation_error(
                    "409 Conflict",
                    "telegram_terminal_reconciliation.evidence_incomplete_or_mismatched",
                ));
            }
        };
        let response = TelegramReconciliationResponse {
            schema: RECONCILIATION_SCHEMA,
            authority: "exact_telegram_terminal_hmac",
            operation: decision,
            result: outcome.result,
            request_binding_hash,
            request_id: &request.request_id,
            plan_hash: &request.plan_hash,
            plan_request_binding_hash: &request.plan_request_binding_hash,
            commit_request_binding_hash: &request.commit_request_binding_hash,
            session_binding_hash: &request.session_binding_hash,
            cursor: request.cursor,
            update_id: request.update_id,
            next_update_offset: request.next_update_offset,
            effect_plan_hash: &request.effect_plan_hash,
            provider_ack_hash: &request.provider_ack_hash,
            terminal_receipt_hash: outcome.terminal_receipt_hash.as_deref(),
            provider_replayed: false,
            read_replayed: false,
            model_replayed: false,
            cursor_written: false,
            cursor_advanced: false,
            terminal_receipt_recorded: outcome.terminal_receipt_hash.is_some(),
        };
        Some(match serde_json::to_string(&response) {
            Ok(body) => TelegramReconciliationHttpResponse {
                status: "200 OK",
                body,
                outcome_state_changed: outcome.state_changed,
            },
            Err(_) => reconciliation_error(
                "503 Service Unavailable",
                "telegram_terminal_reconciliation.response_encoding_failed",
            ),
        })
    }

    fn reconcile_terminal_only(
        &self,
        request: &TelegramReconciliationRequest,
        delivery_ledger_path: &Path,
        cursor_path: &Path,
        resolve: bool,
    ) -> Result<TelegramReconciliationOutcome> {
        self.update_locked_events(|snapshot| {
            compact_if_needed(snapshot, &self.key)?;
            let latest = snapshot
                .events
                .iter()
                .rev()
                .find(|event| event.plan_hash == request.plan_hash)
                .context("Telegram reconciliation plan is unavailable")?;
            exact_reconciliation_event(latest, request)?;
            let delivery_ack = exact_authenticated_delivery_ack(
                delivery_ledger_path,
                cursor_path,
                request,
                &self.key,
            )?;
            let cursor = crate::telegram_durable_files::cursor_status(
                true,
                cursor_path,
                ".hepta/telegram/ingress-drain-cursor.json",
            );
            if !cursor.cursor_parse_ok
                || cursor.next_update_offset != Some(request.next_update_offset)
            {
                anyhow::bail!("Telegram reconciliation cursor evidence is stale or missing");
            }
            let terminal_receipt_hash =
                reconciled_terminal_receipt_hash(request, &delivery_ack.mac);
            if latest.phase == Phase::ReconciledTerminalSucceeded {
                if latest.terminal_receipt_hash.as_deref() != Some(terminal_receipt_hash.as_str()) {
                    anyhow::bail!("Telegram reconciled terminal receipt is substituted");
                }
                return Ok(TelegramReconciliationOutcome {
                    result: "already_recorded",
                    terminal_receipt_hash: Some(terminal_receipt_hash),
                    state_changed: false,
                });
            }
            if latest.phase != Phase::SendAcknowledged {
                anyhow::bail!("Telegram journal is not exactly send-acknowledged");
            }
            if !resolve {
                return Ok(TelegramReconciliationOutcome {
                    result: "eligible",
                    terminal_receipt_hash: None,
                    state_changed: false,
                });
            }
            let evidence = PhaseEvidence {
                update_id: latest.update_id,
                next_update_offset: latest.next_update_offset,
                read_result_hash: latest.read_result_hash.clone(),
                model_result_hash: latest.model_result_hash.clone(),
                effect_plan_hash: latest.effect_plan_hash.clone(),
                provider_ack_hash: latest.provider_ack_hash.clone(),
                terminal_receipt_hash: Some(terminal_receipt_hash.clone()),
            };
            validate_evidence_transition(latest, Phase::ReconciledTerminalSucceeded, &evidence)?;
            let binding = PlanBinding {
                request_id: request.request_id.clone(),
                plan_hash: request.plan_hash.clone(),
                plan_request_binding_hash: request.plan_request_binding_hash.clone(),
                commit_request_binding_hash: Some(request.commit_request_binding_hash.clone()),
                session_binding_hash: request.session_binding_hash.clone(),
                cursor: request.cursor,
            };
            let owner_nonce = latest
                .owner_nonce
                .as_deref()
                .context("Telegram reconciliation owner binding is missing")?;
            let event = event_for(
                snapshot,
                &binding,
                Some(owner_nonce),
                Phase::ReconciledTerminalSucceeded,
                evidence,
            );
            append_event(snapshot, event, &self.key)?;
            Ok(TelegramReconciliationOutcome {
                result: "recorded",
                terminal_receipt_hash: Some(terminal_receipt_hash),
                state_changed: true,
            })
        })
    }

    fn with_locked_events<T>(
        &self,
        operation: impl FnOnce(&JournalSnapshot) -> Result<T>,
    ) -> Result<T> {
        let _process = self
            .process_lock
            .lock()
            .map_err(|_| anyhow::anyhow!("Telegram authority process mutex poisoned"))?;
        let bytes = read_private_state(&self.journal_file, MAX_JOURNAL_BYTES)?;
        let snapshot = read_journal_snapshot(bytes.as_deref().unwrap_or_default(), &self.key)?;
        operation(&snapshot)
    }

    fn update_locked_events<T>(
        &self,
        operation: impl FnOnce(&mut JournalSnapshot) -> Result<T>,
    ) -> Result<T> {
        let _process = self
            .process_lock
            .lock()
            .map_err(|_| anyhow::anyhow!("Telegram authority process mutex poisoned"))?;
        update_private_state_atomically(
            &self.journal_file,
            MAX_JOURNAL_BYTES,
            JOURNAL_STAGING_PREFIX,
            |current| {
                let mut snapshot = read_journal_snapshot(current.unwrap_or_default(), &self.key)?;
                let output = operation(&mut snapshot)?;
                let bytes = encode_journal_snapshot(&snapshot)?;
                Ok((bytes, output))
            },
        )
    }
}

impl JournalSnapshot {
    fn previous_entry_hash(&self) -> String {
        self.events
            .last()
            .map(event_hash)
            .or_else(|| self.checkpoint.as_ref().map(checkpoint_hash))
            .unwrap_or_else(|| GENESIS_HASH.to_owned())
    }

    fn next_sequence(&self) -> u64 {
        self.events
            .last()
            .map(|event| event.sequence + 1)
            .or_else(|| {
                self.checkpoint
                    .as_ref()
                    .map(|checkpoint| checkpoint.sequence + 1)
            })
            .unwrap_or(1)
    }

    fn monotonic_binding(&self) -> (u64, String, Option<String>) {
        if let Some(event) = self.events.last() {
            return (event.sequence, event_hash(event), Some(event.mac.clone()));
        }
        self.checkpoint.as_ref().map_or_else(
            || (0, GENESIS_HASH.to_owned(), None),
            |checkpoint| {
                (
                    checkpoint.sequence,
                    checkpoint_hash(checkpoint),
                    Some(checkpoint.mac.clone()),
                )
            },
        )
    }
}

impl TelegramPipelinePermit<'_> {
    pub(crate) fn execute_with<R, M, S>(
        mut self,
        delivery_ledger_path: &Path,
        cursor_path: &Path,
        read: R,
        model: M,
        send: S,
    ) -> Result<TelegramPipelineReceipt>
    where
        R: FnOnce(&TelegramReadRequest) -> Result<TelegramReadResult>,
        M: FnOnce(&TelegramModelRequest) -> Result<String>,
        S: FnOnce(&TelegramSendPlan) -> Result<TelegramProviderAck>,
    {
        validate_delivery_targets(delivery_ledger_path, cursor_path)?;
        let read_intent_hash = self.intent(Phase::ReadIntent, "credentialed_read")?;
        let commit_request_binding_hash = self.commit_request_binding_hash()?.to_owned();
        let read_request = TelegramReadRequest {
            session_binding_hash: self.binding.session_binding_hash.clone(),
            request_binding_hash: commit_request_binding_hash.clone(),
            cursor: self.binding.cursor,
        };
        let read_result = read(&read_request).context("authorized Telegram read failed")?;
        validate_read_result(&read_result, self.binding.cursor)?;
        let next_update_offset = telegram_next_update_offset(read_result.update_id)
            .context("Telegram update id cannot advance to a next cursor")?;
        let read_result_hash = hash_read_result(&read_result);
        if self.authority.inspect_events()?.iter().any(|event| {
            event.phase == Phase::TerminalSucceeded
                && (event.update_id == Some(read_result.update_id)
                    || event.read_result_hash.as_deref() == Some(read_result_hash.as_str()))
        }) {
            anyhow::bail!("Telegram update already has a terminal delivery receipt");
        }
        self.authority.append_transition(
            &self.binding,
            &self.owner_nonce,
            Phase::ReadCompleted,
            PhaseEvidence {
                update_id: Some(read_result.update_id),
                next_update_offset: Some(next_update_offset),
                read_result_hash: Some(read_result_hash.clone()),
                ..PhaseEvidence::default()
            },
        )?;

        let model_intent_hash = self.intent_with_evidence(
            Phase::ModelIntent,
            "model_invocation",
            PhaseEvidence {
                update_id: Some(read_result.update_id),
                next_update_offset: Some(next_update_offset),
                read_result_hash: Some(read_result_hash.clone()),
                ..PhaseEvidence::default()
            },
        )?;
        let model_request = TelegramModelRequest {
            session_binding_hash: self.binding.session_binding_hash.clone(),
            request_binding_hash: commit_request_binding_hash.clone(),
            update_id: read_result.update_id,
            prompt: read_result.prompt.clone(),
        };
        let model_output = model(&model_request).context("authorized Telegram model failed")?;
        if model_output.is_empty() || model_output.len() > MAX_REPLY_BYTES {
            anyhow::bail!("Telegram model output must contain 1..={MAX_REPLY_BYTES} bytes");
        }
        let model_result_hash = digest(
            MODEL_RESULT_DOMAIN,
            &[
                &read_result_hash,
                &format!("sha256:{:x}", Sha256::digest(model_output.as_bytes())),
            ],
        );
        self.authority.append_transition(
            &self.binding,
            &self.owner_nonce,
            Phase::ModelCompleted,
            PhaseEvidence {
                update_id: Some(read_result.update_id),
                next_update_offset: Some(next_update_offset),
                read_result_hash: Some(read_result_hash.clone()),
                model_result_hash: Some(model_result_hash.clone()),
                ..PhaseEvidence::default()
            },
        )?;

        let effect_plan_hash = digest(
            SEND_PLAN_DOMAIN,
            &[
                &self.binding.plan_hash,
                &commit_request_binding_hash,
                &self.binding.session_binding_hash,
                &read_result.update_id.to_string(),
                &next_update_offset.to_string(),
                &read_result.chat_id.to_string(),
                &optional_i64(read_result.reply_to_message_id),
                &format!("sha256:{:x}", Sha256::digest(model_output.as_bytes())),
                &path_binding(delivery_ledger_path),
                &path_binding(cursor_path),
            ],
        );
        let idempotency_key = format!(
            "hepta-telegram:{}:{}",
            self.binding.request_id, effect_plan_hash
        );
        self.authority.append_transition(
            &self.binding,
            &self.owner_nonce,
            Phase::SendIntent,
            PhaseEvidence {
                update_id: Some(read_result.update_id),
                next_update_offset: Some(next_update_offset),
                read_result_hash: Some(read_result_hash.clone()),
                model_result_hash: Some(model_result_hash.clone()),
                effect_plan_hash: Some(effect_plan_hash.clone()),
                ..PhaseEvidence::default()
            },
        )?;
        let send_plan = TelegramSendPlan {
            update_id: read_result.update_id,
            next_update_offset,
            chat_id: read_result.chat_id,
            reply_to_message_id: read_result.reply_to_message_id,
            message_text: model_output,
            effect_plan_hash: effect_plan_hash.clone(),
            idempotency_key,
        };
        append_delivery_record(
            delivery_ledger_path,
            "enqueued",
            next_update_offset,
            false,
            None,
            false,
            None,
        )
        .context("record Telegram delivery enqueue before send")?;
        let provider_ack = match send(&send_plan) {
            Ok(provider_ack) => provider_ack,
            Err(error) => {
                append_delivery_record(
                    delivery_ledger_path,
                    "failed",
                    next_update_offset,
                    true,
                    None,
                    false,
                    Some("authorized Telegram send failed"),
                )
                .context("record failed Telegram delivery")?;
                return Err(error).context("authorized Telegram send failed");
            }
        };
        if let Err(error) = validate_provider_ack(&provider_ack, &send_plan) {
            append_delivery_record(
                delivery_ledger_path,
                "failed",
                next_update_offset,
                true,
                Some(false),
                provider_ack.provider_message_id > 0,
                Some("Telegram provider ACK did not match the exact send plan"),
            )
            .context("record mismatched Telegram provider ACK")?;
            return Err(error);
        }
        let provider_ack_hash = digest(
            PROVIDER_ACK_DOMAIN,
            &[
                &effect_plan_hash,
                &provider_ack.provider,
                &provider_ack.provider_message_id.to_string(),
                &provider_ack.chat_id.to_string(),
                &provider_ack.raw_response_hash,
            ],
        );
        self.authority.append_transition(
            &self.binding,
            &self.owner_nonce,
            Phase::SendAcknowledged,
            PhaseEvidence {
                update_id: Some(read_result.update_id),
                next_update_offset: Some(next_update_offset),
                read_result_hash: Some(read_result_hash.clone()),
                model_result_hash: Some(model_result_hash.clone()),
                effect_plan_hash: Some(effect_plan_hash.clone()),
                provider_ack_hash: Some(provider_ack_hash.clone()),
                ..PhaseEvidence::default()
            },
        )?;
        append_authenticated_delivery_ack_record(
            delivery_ledger_path,
            cursor_path,
            &self.binding,
            next_update_offset,
            read_result.update_id,
            &effect_plan_hash,
            &provider_ack_hash,
            &self.authority.key,
        )
        .context("record Telegram delivery ACK before cursor commit")?;
        crate::telegram_durable_files::write_cursor_next_update_offset(
            cursor_path,
            next_update_offset,
        )
        .context("commit Telegram next update cursor after delivery ACK")?;
        let terminal_receipt_hash = digest(
            EVENT_HASH_DOMAIN,
            &[
                &self.binding.plan_hash,
                &read_intent_hash,
                &model_intent_hash,
                &effect_plan_hash,
                &provider_ack_hash,
                &read_result.update_id.to_string(),
                &next_update_offset.to_string(),
                "succeeded",
            ],
        );
        self.authority.append_transition(
            &self.binding,
            &self.owner_nonce,
            Phase::TerminalSucceeded,
            PhaseEvidence {
                update_id: Some(read_result.update_id),
                next_update_offset: Some(next_update_offset),
                read_result_hash: Some(read_result_hash.clone()),
                model_result_hash: Some(model_result_hash.clone()),
                effect_plan_hash: Some(effect_plan_hash.clone()),
                provider_ack_hash: Some(provider_ack_hash.clone()),
                terminal_receipt_hash: Some(terminal_receipt_hash.clone()),
            },
        )?;
        self.finished = true;
        Ok(TelegramPipelineReceipt {
            schema: SCHEMA,
            status: "succeeded",
            durable_intent_owner: TELEGRAM_PIPELINE_AUTHORITY_OWNER,
            request_id: self.binding.request_id.clone(),
            plan_hash: self.binding.plan_hash.clone(),
            session_binding_hash: self.binding.session_binding_hash.clone(),
            plan_request_binding_hash: self.binding.plan_request_binding_hash.clone(),
            commit_request_binding_hash,
            read_intent_hash,
            read_result_hash,
            model_intent_hash,
            model_result_hash,
            effect_plan_hash,
            provider_effect_ack_hash: provider_ack_hash,
            terminal_receipt_hash,
            update_id: read_result.update_id,
            next_update_offset,
            live_read_authorized: true,
            model_invocation_authorized: true,
            send_authorized: true,
            durable_intent_recorded: true,
            provider_effect_ack_recorded: true,
            delivery_ack_recorded: true,
            cursor_written: true,
            terminal_receipt_recorded: true,
        })
    }

    fn intent(&self, phase: Phase, operation: &str) -> Result<String> {
        self.intent_with_evidence(phase, operation, PhaseEvidence::default())
    }

    fn intent_with_evidence(
        &self,
        phase: Phase,
        operation: &str,
        evidence: PhaseEvidence,
    ) -> Result<String> {
        let hash = digest(
            INTENT_DOMAIN,
            &[
                &self.binding.plan_hash,
                self.commit_request_binding_hash()?,
                &self.binding.session_binding_hash,
                &cursor_binding(self.binding.cursor),
                operation,
            ],
        );
        self.authority
            .append_transition(&self.binding, &self.owner_nonce, phase, evidence)?;
        Ok(hash)
    }

    fn commit_request_binding_hash(&self) -> Result<&str> {
        self.binding
            .commit_request_binding_hash
            .as_deref()
            .context("Telegram permit lacks its exact commit request binding")
    }
}

impl Drop for TelegramPipelinePermit<'_> {
    fn drop(&mut self) {
        if self.finished {
            return;
        }
        if matches!(
            self.authority
                .latest_owned_phase(&self.binding, &self.owner_nonce),
            Ok(Phase::SendIntent
                | Phase::SendAcknowledged
                | Phase::TerminalSucceeded
                | Phase::ReconciledTerminalSucceeded)
        ) {
            // Once an exact send plan has been persisted, replacing it with an
            // evidence-free InDoubt event would erase the only safe
            // reconciliation boundary. Never retry the provider here.
            return;
        }
        let _ = self.authority.append_transition(
            &self.binding,
            &self.owner_nonce,
            Phase::InDoubt,
            PhaseEvidence::default(),
        );
    }
}

fn event_for(
    snapshot: &JournalSnapshot,
    binding: &PlanBinding,
    owner_nonce: Option<&str>,
    phase: Phase,
    evidence: PhaseEvidence,
) -> JournalEvent {
    JournalEvent {
        schema: SCHEMA.into(),
        sequence: snapshot.next_sequence(),
        previous_entry_hash: snapshot.previous_entry_hash(),
        phase,
        request_id: binding.request_id.clone(),
        plan_hash: binding.plan_hash.clone(),
        plan_request_binding_hash: binding.plan_request_binding_hash.clone(),
        commit_request_binding_hash: binding.commit_request_binding_hash.clone(),
        session_binding_hash: binding.session_binding_hash.clone(),
        cursor: binding.cursor,
        owner_nonce: owner_nonce.map(ToOwned::to_owned),
        update_id: evidence.update_id,
        next_update_offset: evidence.next_update_offset,
        read_result_hash: evidence.read_result_hash,
        model_result_hash: evidence.model_result_hash,
        effect_plan_hash: evidence.effect_plan_hash,
        provider_ack_hash: evidence.provider_ack_hash,
        terminal_receipt_hash: evidence.terminal_receipt_hash,
        mac: String::new(),
    }
}

fn append_event(
    snapshot: &mut JournalSnapshot,
    mut event: JournalEvent,
    key: &[u8; 32],
) -> Result<JournalEvent> {
    if snapshot.events.len() >= MAX_JOURNAL_EVENTS {
        anyhow::bail!("Telegram authority journal reached its bounded event limit");
    }
    event.mac = event_mac(&event, key)?;
    if serde_json::to_vec(&event)
        .context("encode Telegram authority event")?
        .len()
        > MAX_EVENT_BYTES
    {
        anyhow::bail!("Telegram authority event exceeds its bounded size");
    }
    snapshot.events.push(event.clone());
    Ok(event)
}

fn read_journal_snapshot(bytes: &[u8], key: &[u8; 32]) -> Result<JournalSnapshot> {
    if bytes.len() as u64 > MAX_JOURNAL_BYTES {
        anyhow::bail!("Telegram authority journal exceeds its bounded size");
    }
    let mut lines = bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let checkpoint = if lines.first().is_some_and(|line| {
        serde_json::from_slice::<serde_json::Value>(line)
            .ok()
            .and_then(|value| value.get("schema")?.as_str().map(ToOwned::to_owned))
            .as_deref()
            == Some(CHECKPOINT_SCHEMA)
    }) {
        let checkpoint: JournalCheckpoint = serde_json::from_slice(lines.remove(0))
            .context("decode Telegram authority checkpoint")?;
        validate_checkpoint(&checkpoint, key)?;
        Some(checkpoint)
    } else {
        None
    };
    let mut previous = checkpoint
        .as_ref()
        .map(checkpoint_hash)
        .unwrap_or_else(|| GENESIS_HASH.to_owned());
    let first_sequence = checkpoint.as_ref().map_or(1, |value| value.sequence + 1);
    let mut events = Vec::new();
    for (expected_sequence, line) in (first_sequence..).zip(lines) {
        if line.len() > MAX_EVENT_BYTES || events.len() >= MAX_JOURNAL_EVENTS {
            anyhow::bail!("Telegram authority journal exceeds bounded record limits");
        }
        let event: JournalEvent =
            serde_json::from_slice(line).context("decode Telegram authority event")?;
        if event.schema != SCHEMA
            || event.sequence != expected_sequence
            || event.previous_entry_hash != previous
            || event.mac.len() != 64
            || !constant_time_equal(&event.mac, &event_mac(&event, key)?)
        {
            anyhow::bail!("Telegram authority journal chain or MAC is invalid");
        }
        previous = event_hash(&event);
        events.push(event);
    }
    Ok(JournalSnapshot { checkpoint, events })
}

fn encode_journal_snapshot(snapshot: &JournalSnapshot) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    if let Some(checkpoint) = &snapshot.checkpoint {
        let encoded =
            serde_json::to_vec(checkpoint).context("encode Telegram authority checkpoint")?;
        if encoded.len() > MAX_CHECKPOINT_BYTES {
            anyhow::bail!("Telegram authority checkpoint exceeds its bounded size");
        }
        bytes.extend(encoded);
        bytes.push(b'\n');
    }
    for event in &snapshot.events {
        let encoded = serde_json::to_vec(event).context("encode Telegram authority event")?;
        if encoded.len() > MAX_EVENT_BYTES {
            anyhow::bail!("Telegram authority event exceeds its bounded size");
        }
        bytes.extend(encoded);
        bytes.push(b'\n');
    }
    if bytes.len() as u64 > MAX_JOURNAL_BYTES {
        anyhow::bail!("Telegram authority journal exceeds its bounded size");
    }
    Ok(bytes)
}

fn compactable_terminal_pipeline_count(snapshot: &JournalSnapshot) -> usize {
    terminal_pipeline_hashes(snapshot)
        .len()
        .saturating_sub(RETAIN_TERMINAL_PIPELINES)
}

fn compact_if_needed(snapshot: &mut JournalSnapshot, key: &[u8; 32]) -> Result<()> {
    if snapshot.events.len() < MAX_JOURNAL_EVENTS {
        return Ok(());
    }
    compact_terminal_pipelines(snapshot, key, RETAIN_TERMINAL_PIPELINES)
}

fn compact_terminal_pipelines(
    snapshot: &mut JournalSnapshot,
    key: &[u8; 32],
    retain_terminal_pipelines: usize,
) -> Result<()> {
    let terminal_plans = terminal_pipeline_hashes(snapshot);
    let compact_count = terminal_plans
        .len()
        .saturating_sub(retain_terminal_pipelines);
    if compact_count == 0 {
        anyhow::bail!("Telegram authority journal reached its bounded event limit");
    }
    let compacted_plans = terminal_plans
        .into_iter()
        .take(compact_count)
        .collect::<HashSet<_>>();
    let mut consumed_authorities = snapshot
        .checkpoint
        .as_ref()
        .map(|checkpoint| checkpoint.consumed_authorities.clone())
        .unwrap_or_default();
    for plan_hash in &compacted_plans {
        let event = snapshot
            .events
            .iter()
            .rev()
            .find(|event| event.plan_hash == *plan_hash)
            .context("Telegram terminal pipeline disappeared during checkpoint")?;
        if !consumed_authorities.iter().any(|authority| {
            authority.request_id == event.request_id || authority.plan_hash == event.plan_hash
        }) {
            consumed_authorities.push(ConsumedAuthority {
                request_id: event.request_id.clone(),
                plan_hash: event.plan_hash.clone(),
            });
        }
    }
    if consumed_authorities.len() > MAX_CHECKPOINTED_AUTHORITIES {
        anyhow::bail!("Telegram authority checkpoint reached its bounded authority limit");
    }
    let removed_events = snapshot
        .events
        .iter()
        .filter(|event| compacted_plans.contains(&event.plan_hash))
        .count();
    if removed_events == 0 {
        anyhow::bail!("Telegram authority checkpoint made no progress");
    }
    let (old_sequence, old_state_hash, _) = snapshot.monotonic_binding();
    let old_history_hash = snapshot
        .checkpoint
        .as_ref()
        .map(|checkpoint| checkpoint.history_hash.as_str())
        .unwrap_or(CHECKPOINT_GENESIS_HASH);
    let removed_events_text = removed_events.to_string();
    let compacted_events =
        snapshot
            .checkpoint
            .as_ref()
            .map_or(removed_events as u64, |checkpoint| {
                checkpoint
                    .compacted_events
                    .saturating_add(removed_events as u64)
            });
    let compacted_events_text = compacted_events.to_string();
    let previous_checkpoint_hash = snapshot
        .checkpoint
        .as_ref()
        .map(checkpoint_hash)
        .unwrap_or_else(|| CHECKPOINT_GENESIS_HASH.to_owned());
    let mut checkpoint = JournalCheckpoint {
        schema: CHECKPOINT_SCHEMA.to_owned(),
        revision: snapshot
            .checkpoint
            .as_ref()
            .map_or(1, |checkpoint| checkpoint.revision + 1),
        sequence: old_sequence + 1,
        previous_checkpoint_hash,
        compacted_events,
        consumed_authorities,
        history_hash: digest(
            CHECKPOINT_HISTORY_DOMAIN,
            &[
                old_history_hash,
                &old_state_hash,
                &removed_events_text,
                &compacted_events_text,
            ],
        ),
        mac: String::new(),
    };
    checkpoint.mac = checkpoint_mac(&checkpoint, key)?;
    snapshot
        .events
        .retain(|event| !compacted_plans.contains(&event.plan_hash));
    let mut previous_entry_hash = checkpoint_hash(&checkpoint);
    for (sequence, event) in (checkpoint.sequence + 1..).zip(&mut snapshot.events) {
        event.sequence = sequence;
        event.previous_entry_hash = previous_entry_hash;
        event.mac = event_mac(event, key)?;
        previous_entry_hash = event_hash(event);
    }
    snapshot.checkpoint = Some(checkpoint);
    Ok(())
}

fn terminal_pipeline_hashes(snapshot: &JournalSnapshot) -> Vec<String> {
    snapshot
        .events
        .iter()
        .filter(|event| event.phase.is_success_terminal())
        .map(|event| event.plan_hash.clone())
        .collect()
}

fn authority_consumed(snapshot: &JournalSnapshot, request_id: &str, plan_hash: &str) -> bool {
    snapshot.checkpoint.as_ref().is_some_and(|checkpoint| {
        checkpoint
            .consumed_authorities
            .iter()
            .any(|authority| authority.request_id == request_id || authority.plan_hash == plan_hash)
    })
}

fn validate_checkpoint(checkpoint: &JournalCheckpoint, key: &[u8; 32]) -> Result<()> {
    if checkpoint.schema != CHECKPOINT_SCHEMA
        || checkpoint.revision == 0
        || checkpoint.sequence == 0
        || checkpoint.consumed_authorities.is_empty()
        || checkpoint.consumed_authorities.len() > MAX_CHECKPOINTED_AUTHORITIES
        || checkpoint.compacted_events == 0
        || checkpoint.mac.len() != 64
        || !constant_time_equal(&checkpoint.mac, &checkpoint_mac(checkpoint, key)?)
    {
        anyhow::bail!("Telegram authority checkpoint binding or MAC is invalid");
    }
    if checkpoint.previous_checkpoint_hash != CHECKPOINT_GENESIS_HASH {
        validate_content_hash(
            &checkpoint.previous_checkpoint_hash,
            "Telegram authority previous checkpoint hash",
        )?;
    }
    validate_hash(
        &checkpoint.history_hash,
        "Telegram authority checkpoint history hash",
    )?;
    let mut request_ids = HashSet::new();
    let mut plan_hashes = HashSet::new();
    for authority in &checkpoint.consumed_authorities {
        validate_hash(&authority.request_id, "Telegram checkpoint request id")?;
        validate_hash(&authority.plan_hash, "Telegram checkpoint plan hash")?;
        if !request_ids.insert(authority.request_id.as_str())
            || !plan_hashes.insert(authority.plan_hash.as_str())
        {
            anyhow::bail!("Telegram authority checkpoint contains duplicate authority");
        }
    }
    Ok(())
}

fn checkpoint_mac(checkpoint: &JournalCheckpoint, key: &[u8; 32]) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(key)?;
    update_mac(&mut mac, CHECKPOINT_MAC_DOMAIN);
    for field in checkpoint_fields(checkpoint) {
        update_mac(&mut mac, field.as_bytes());
    }
    Ok(hex_encode(&mac.finalize().into_bytes()))
}

fn checkpoint_hash(checkpoint: &JournalCheckpoint) -> String {
    let mut hasher = Sha256::new();
    update_hash(&mut hasher, CHECKPOINT_HASH_DOMAIN);
    for field in checkpoint_fields(checkpoint) {
        update_hash(&mut hasher, field.as_bytes());
    }
    update_hash(&mut hasher, checkpoint.mac.as_bytes());
    format!("sha256:{:x}", hasher.finalize())
}

fn checkpoint_fields(checkpoint: &JournalCheckpoint) -> Vec<String> {
    let mut fields = vec![
        checkpoint.schema.clone(),
        checkpoint.revision.to_string(),
        checkpoint.sequence.to_string(),
        checkpoint.previous_checkpoint_hash.clone(),
        checkpoint.compacted_events.to_string(),
        checkpoint.history_hash.clone(),
    ];
    for authority in &checkpoint.consumed_authorities {
        fields.push(authority.request_id.clone());
        fields.push(authority.plan_hash.clone());
    }
    fields
}

fn event_mac(event: &JournalEvent, key: &[u8; 32]) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(key)?;
    update_mac(&mut mac, EVENT_MAC_DOMAIN);
    for value in event_fields(event) {
        update_mac(&mut mac, value.as_bytes());
    }
    Ok(hex_encode(&mac.finalize().into_bytes()))
}

fn event_hash(event: &JournalEvent) -> String {
    let mut hasher = Sha256::new();
    update_hash(&mut hasher, EVENT_HASH_DOMAIN);
    for value in event_fields(event) {
        update_hash(&mut hasher, value.as_bytes());
    }
    update_hash(&mut hasher, event.mac.as_bytes());
    format!("sha256:{:x}", hasher.finalize())
}

fn event_fields(event: &JournalEvent) -> Vec<String> {
    vec![
        event.schema.clone(),
        event.sequence.to_string(),
        event.previous_entry_hash.clone(),
        format!("{:?}", event.phase),
        event.request_id.clone(),
        event.plan_hash.clone(),
        event.plan_request_binding_hash.clone(),
        event
            .commit_request_binding_hash
            .clone()
            .unwrap_or_default(),
        event.session_binding_hash.clone(),
        cursor_binding(event.cursor),
        event.owner_nonce.clone().unwrap_or_default(),
        optional_i64(event.update_id),
        optional_i64(event.next_update_offset),
        event.read_result_hash.clone().unwrap_or_default(),
        event.model_result_hash.clone().unwrap_or_default(),
        event.effect_plan_hash.clone().unwrap_or_default(),
        event.provider_ack_hash.clone().unwrap_or_default(),
        event.terminal_receipt_hash.clone().unwrap_or_default(),
    ]
}

fn bounded_body(body: Option<&str>) -> Result<&str> {
    let body = body.context("Telegram authority request body is required")?;
    if body.is_empty() || body.len() > MAX_BODY_BYTES {
        anyhow::bail!("Telegram authority request body exceeds its bounded size");
    }
    Ok(body)
}

fn validate_cursor(cursor: Option<i64>) -> Result<()> {
    if cursor.is_some_and(|cursor| cursor < 0) {
        anyhow::bail!("Telegram cursor must be non-negative");
    }
    Ok(())
}

fn validate_delivery_targets(delivery_ledger_path: &Path, cursor_path: &Path) -> Result<()> {
    if delivery_ledger_path.as_os_str().is_empty()
        || cursor_path.as_os_str().is_empty()
        || delivery_ledger_path == cursor_path
    {
        anyhow::bail!("Telegram delivery ledger and cursor targets must be distinct paths");
    }
    Ok(())
}

fn path_binding(path: &Path) -> String {
    format!(
        "sha256:{:x}",
        Sha256::digest(path.as_os_str().as_encoded_bytes())
    )
}

fn append_delivery_record(
    delivery_ledger_path: &Path,
    stage: &'static str,
    next_update_offset: i64,
    provider_send_attempted: bool,
    bot_api_ack: Option<bool>,
    provider_message_id_present: bool,
    error: Option<&str>,
) -> Result<()> {
    let record = telegram_delivery_lifecycle_record(
        stage,
        Some(next_update_offset),
        true,
        provider_send_attempted,
        bot_api_ack,
        provider_message_id_present,
        error,
    );
    crate::telegram_durable_files::append_delivery_lifecycle_record(delivery_ledger_path, &record)
}

#[allow(clippy::too_many_arguments)]
fn append_authenticated_delivery_ack_record(
    delivery_ledger_path: &Path,
    cursor_path: &Path,
    binding: &PlanBinding,
    next_update_offset: i64,
    update_id: i64,
    effect_plan_hash: &str,
    provider_ack_hash: &str,
    key: &[u8; 32],
) -> Result<()> {
    let mut record = telegram_delivery_lifecycle_record(
        "acked",
        Some(next_update_offset),
        true,
        true,
        Some(true),
        true,
        None,
    );
    let commit_request_binding_hash = binding
        .commit_request_binding_hash
        .as_deref()
        .context("Telegram delivery ACK lacks its commit request binding")?;
    let mut authority = AuthenticatedDeliveryAckBinding {
        schema: DELIVERY_ACK_BINDING_SCHEMA.to_owned(),
        request_id: binding.request_id.clone(),
        plan_hash: binding.plan_hash.clone(),
        plan_request_binding_hash: binding.plan_request_binding_hash.clone(),
        commit_request_binding_hash: commit_request_binding_hash.to_owned(),
        session_binding_hash: binding.session_binding_hash.clone(),
        cursor: binding.cursor,
        update_id,
        next_update_offset,
        effect_plan_hash: effect_plan_hash.to_owned(),
        provider_ack_hash: provider_ack_hash.to_owned(),
        delivery_ledger_path_hash: path_binding(delivery_ledger_path),
        cursor_path_hash: path_binding(cursor_path),
        mac: String::new(),
    };
    authority.mac = delivery_ack_mac(&authority, key)?;
    record
        .as_object_mut()
        .context("Telegram delivery ACK lifecycle record is not an object")?
        .insert(
            "telegram_authority".to_owned(),
            serde_json::to_value(authority).context("encode Telegram delivery ACK authority")?,
        );
    crate::telegram_durable_files::append_delivery_lifecycle_record(delivery_ledger_path, &record)
}

fn exact_authenticated_delivery_ack(
    delivery_ledger_path: &Path,
    cursor_path: &Path,
    request: &TelegramReconciliationRequest,
    key: &[u8; 32],
) -> Result<AuthenticatedDeliveryAckBinding> {
    let bytes = crate::telegram_durable_files::read_private_state(
        delivery_ledger_path,
        MAX_DELIVERY_LEDGER_BYTES,
    )?
    .context("Telegram reconciliation delivery ledger is missing")?;
    let mut matching = None;
    for line in bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
    {
        if line.len() > MAX_EVENT_BYTES {
            anyhow::bail!("Telegram reconciliation delivery record exceeds its bounded size");
        }
        let record: serde_json::Value = serde_json::from_slice(line)
            .context("decode Telegram reconciliation delivery record")?;
        let Some(encoded) = record.get("telegram_authority") else {
            continue;
        };
        let authority: AuthenticatedDeliveryAckBinding = serde_json::from_value(encoded.clone())
            .context("decode authenticated Telegram delivery ACK")?;
        if authority.plan_hash != request.plan_hash {
            continue;
        }
        if matching.is_some()
            || !exact_delivery_ack_binding(&authority, request, delivery_ledger_path, cursor_path)
            || !exact_acked_lifecycle_record(&record, request.next_update_offset)
            || !constant_time_equal(&authority.mac, &delivery_ack_mac(&authority, key)?)
        {
            anyhow::bail!("Telegram reconciliation delivery ACK is ambiguous or substituted");
        }
        matching = Some(authority);
    }
    matching.context("exact authenticated Telegram delivery ACK is missing")
}

fn exact_delivery_ack_binding(
    authority: &AuthenticatedDeliveryAckBinding,
    request: &TelegramReconciliationRequest,
    delivery_ledger_path: &Path,
    cursor_path: &Path,
) -> bool {
    authority.schema == DELIVERY_ACK_BINDING_SCHEMA
        && authority.request_id == request.request_id
        && authority.plan_hash == request.plan_hash
        && authority.plan_request_binding_hash == request.plan_request_binding_hash
        && authority.commit_request_binding_hash == request.commit_request_binding_hash
        && authority.session_binding_hash == request.session_binding_hash
        && authority.cursor == request.cursor
        && authority.update_id == request.update_id
        && authority.next_update_offset == request.next_update_offset
        && authority.effect_plan_hash == request.effect_plan_hash
        && authority.provider_ack_hash == request.provider_ack_hash
        && authority.delivery_ledger_path_hash == path_binding(delivery_ledger_path)
        && authority.cursor_path_hash == path_binding(cursor_path)
}

fn exact_acked_lifecycle_record(record: &serde_json::Value, next_update_offset: i64) -> bool {
    let idempotency_key = format!("telegram:next-offset:{next_update_offset}");
    record
        .get("schema_version")
        .and_then(serde_json::Value::as_u64)
        == Some(1)
        && record.get("stage").and_then(serde_json::Value::as_str) == Some("acked")
        && record.get("entry_id").and_then(serde_json::Value::as_str)
            == Some(idempotency_key.as_str())
        && record
            .get("idempotency_key")
            .and_then(serde_json::Value::as_str)
            == Some(idempotency_key.as_str())
        && record
            .get("provider_send_attempted")
            .and_then(serde_json::Value::as_bool)
            == Some(true)
        && record
            .get("provider_message_id_present")
            .and_then(serde_json::Value::as_bool)
            == Some(true)
        && record
            .get("ack_after_provider_message_id")
            .and_then(serde_json::Value::as_bool)
            == Some(true)
        && record.get("acked").and_then(serde_json::Value::as_bool) == Some(true)
        && record.get("failed").and_then(serde_json::Value::as_bool) == Some(false)
        && record
            .get("retry_scheduled")
            .and_then(serde_json::Value::as_bool)
            == Some(false)
        && record
            .get("external_send_attempted")
            .and_then(serde_json::Value::as_bool)
            == Some(true)
        && record
            .get("raw_chat_id_logged")
            .and_then(serde_json::Value::as_bool)
            == Some(false)
        && record
            .get("raw_message_id_logged")
            .and_then(serde_json::Value::as_bool)
            == Some(false)
        && record
            .get("raw_token_logged")
            .and_then(serde_json::Value::as_bool)
            == Some(false)
        && record.get("error").is_some_and(serde_json::Value::is_null)
}

fn delivery_ack_mac(authority: &AuthenticatedDeliveryAckBinding, key: &[u8; 32]) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(key)?;
    update_mac(&mut mac, DELIVERY_ACK_MAC_DOMAIN);
    for value in [
        authority.schema.as_str(),
        authority.request_id.as_str(),
        authority.plan_hash.as_str(),
        authority.plan_request_binding_hash.as_str(),
        authority.commit_request_binding_hash.as_str(),
        authority.session_binding_hash.as_str(),
        cursor_binding(authority.cursor).as_str(),
        authority.update_id.to_string().as_str(),
        authority.next_update_offset.to_string().as_str(),
        authority.effect_plan_hash.as_str(),
        authority.provider_ack_hash.as_str(),
        authority.delivery_ledger_path_hash.as_str(),
        authority.cursor_path_hash.as_str(),
        "acked",
    ] {
        update_mac(&mut mac, value.as_bytes());
    }
    Ok(hex_encode(&mac.finalize().into_bytes()))
}

fn validate_read_result(result: &TelegramReadResult, cursor: Option<i64>) -> Result<()> {
    if result.update_id < 0
        || cursor.is_some_and(|cursor| result.update_id < cursor)
        || result.chat_id == 0
        || result.prompt.is_empty()
        || result.prompt.len() > MAX_PROMPT_BYTES
    {
        anyhow::bail!("Telegram read result violates its exact bounded plan");
    }
    Ok(())
}

fn validate_provider_ack(ack: &TelegramProviderAck, plan: &TelegramSendPlan) -> Result<()> {
    validate_hash(&ack.raw_response_hash, "Telegram provider response hash")?;
    if ack.provider != "telegram-bot-api"
        || ack.provider_message_id <= 0
        || ack.chat_id != plan.chat_id
    {
        anyhow::bail!("Telegram provider ACK does not match the exact send plan");
    }
    Ok(())
}

fn validate_reconciliation_request(
    request: &TelegramReconciliationRequest,
    request_binding_hash: &str,
) -> Result<()> {
    for (value, name) in [
        (request.request_id.as_str(), "request id"),
        (request.plan_hash.as_str(), "plan hash"),
        (
            request.plan_request_binding_hash.as_str(),
            "plan request binding",
        ),
        (
            request.commit_request_binding_hash.as_str(),
            "commit request binding",
        ),
        (request.session_binding_hash.as_str(), "session binding"),
        (request.effect_plan_hash.as_str(), "effect plan hash"),
        (request.provider_ack_hash.as_str(), "provider ACK hash"),
        (request_binding_hash, "reconciliation request binding"),
    ] {
        validate_hash(value, name)?;
    }
    validate_cursor(request.cursor)?;
    if request.update_id < 0
        || telegram_next_update_offset(request.update_id) != Some(request.next_update_offset)
    {
        anyhow::bail!("Telegram reconciliation update/cursor binding is invalid");
    }
    Ok(())
}

fn exact_reconciliation_event(
    event: &JournalEvent,
    request: &TelegramReconciliationRequest,
) -> Result<()> {
    if !matches!(
        event.phase,
        Phase::SendAcknowledged | Phase::ReconciledTerminalSucceeded
    ) || event.request_id != request.request_id
        || event.plan_hash != request.plan_hash
        || event.plan_request_binding_hash != request.plan_request_binding_hash
        || event.commit_request_binding_hash.as_deref()
            != Some(request.commit_request_binding_hash.as_str())
        || event.session_binding_hash != request.session_binding_hash
        || event.cursor != request.cursor
        || event.update_id != Some(request.update_id)
        || event.next_update_offset != Some(request.next_update_offset)
        || event.effect_plan_hash.as_deref() != Some(request.effect_plan_hash.as_str())
        || event.provider_ack_hash.as_deref() != Some(request.provider_ack_hash.as_str())
        || event.owner_nonce.as_deref().is_none_or(str::is_empty)
    {
        anyhow::bail!("Telegram reconciliation journal binding is incomplete or substituted");
    }
    Ok(())
}

fn reconciled_terminal_receipt_hash(
    request: &TelegramReconciliationRequest,
    delivery_ack_mac: &str,
) -> String {
    digest(
        RECONCILED_TERMINAL_DOMAIN,
        &[
            &request.request_id,
            &request.plan_hash,
            &request.plan_request_binding_hash,
            &request.commit_request_binding_hash,
            &request.session_binding_hash,
            &cursor_binding(request.cursor),
            &request.update_id.to_string(),
            &request.next_update_offset.to_string(),
            &request.effect_plan_hash,
            &request.provider_ack_hash,
            delivery_ack_mac,
            "terminal-only-no-provider-or-cursor-replay",
        ],
    )
}

fn reconciliation_error(
    status: &'static str,
    code: &'static str,
) -> TelegramReconciliationHttpResponse {
    TelegramReconciliationHttpResponse {
        status,
        body: serde_json::json!({"error": code}).to_string(),
        outcome_state_changed: false,
    }
}

fn hash_read_result(result: &TelegramReadResult) -> String {
    digest(
        READ_RESULT_DOMAIN,
        &[
            &result.update_id.to_string(),
            &result.chat_id.to_string(),
            &optional_i64(result.reply_to_message_id),
            &format!("sha256:{:x}", Sha256::digest(result.prompt.as_bytes())),
        ],
    )
}

fn verify_proof(key: &[u8; 32], domain: &[u8], fields: &[&str], proof: &str) -> Result<()> {
    let proof = decode_hex(proof)?;
    let mut mac = HmacSha256::new_from_slice(key)?;
    update_mac(&mut mac, domain);
    for value in fields {
        update_mac(&mut mac, value.as_bytes());
    }
    mac.verify_slice(&proof)
        .map_err(|_| anyhow::anyhow!("Telegram operator proof is invalid"))
}

fn digest(domain: &[u8], fields: &[&str]) -> String {
    let mut hasher = Sha256::new();
    update_hash(&mut hasher, domain);
    for value in fields {
        update_hash(&mut hasher, value.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

fn validate_hash(value: &str, name: &str) -> Result<()> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Ok(());
    }
    anyhow::bail!("{name} must be canonical lowercase SHA-256 hex")
}

fn validate_content_hash(value: &str, name: &str) -> Result<()> {
    let Some(value) = value.strip_prefix("sha256:") else {
        anyhow::bail!("{name} must use the sha256 content-hash scheme");
    };
    validate_hash(value, name)
}

fn cursor_binding(cursor: Option<i64>) -> String {
    cursor.map_or_else(|| "none".into(), |cursor| cursor.to_string())
}

fn optional_i64(value: Option<i64>) -> String {
    value.map_or_else(|| "none".into(), |value| value.to_string())
}

fn decode_hex(value: &str) -> Result<[u8; 32]> {
    validate_hash(value, "proof")?;
    let mut bytes = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        bytes[index] = (hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?;
    }
    Ok(bytes)
}

fn hex_nibble(value: u8) -> Result<u8> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => anyhow::bail!("invalid lowercase hexadecimal value"),
    }
}

fn random_owner_nonce() -> Result<String> {
    let mut random = [0_u8; 32];
    OpenOptions::new()
        .read(true)
        .open("/dev/urandom")
        .context("open operating-system random source")?
        .read_exact(&mut random)
        .context("read Telegram authority owner nonce")?;
    Ok(hex_encode(&random))
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

fn update_mac(mac: &mut HmacSha256, value: &[u8]) {
    mac.update(&(value.len() as u64).to_be_bytes());
    mac.update(value);
}

fn update_hash(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value);
}

fn constant_time_equal(left: &str, right: &str) -> bool {
    left.len() == right.len()
        && left
            .bytes()
            .zip(right.bytes())
            .fold(0_u8, |difference, (left, right)| {
                difference | (left ^ right)
            })
            == 0
}

#[cfg(unix)]
fn validate_private_parent(path: &Path) -> Result<()> {
    let parent = path
        .parent()
        .context("Telegram authority journal has no parent")?;
    let metadata = fs::symlink_metadata(parent)?;
    if !metadata.is_dir()
        || metadata.file_type().is_symlink()
        || metadata.uid() != effective_uid()
        || metadata.mode() & 0o7777 != PRIVATE_DIRECTORY_MODE
    {
        anyhow::bail!("Telegram authority journal parent must be owned mode-0700 directory");
    }
    Ok(())
}

#[cfg(not(unix))]
fn validate_private_parent(_path: &Path) -> Result<()> {
    anyhow::bail!("Telegram authority journal requires Unix secure-file semantics")
}

#[cfg(unix)]
fn effective_uid() -> u32 {
    // SAFETY: geteuid has no preconditions.
    unsafe { libc::geteuid() }
}

#[cfg(test)]
fn proof(key: &[u8; 32], domain: &[u8], fields: &[&str]) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("test HMAC");
    update_mac(&mut mac, domain);
    for value in fields {
        update_mac(&mut mac, value.as_bytes());
    }
    hex_encode(&mac.finalize().into_bytes())
}

#[cfg(all(test, unix))]
#[path = "../tests/unit/telegram_authority.rs"]
mod tests;

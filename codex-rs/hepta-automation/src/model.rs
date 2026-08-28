use std::fmt;
use std::str::FromStr;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::AuthorityAction;
use codex_hepta_contracts::DestinationAcknowledgement;
use codex_hepta_contracts::IdempotencyKey;
use codex_hepta_contracts::OperationBinding;
use codex_hepta_contracts::OperationId;
use codex_hepta_contracts::OutboxEnvelope;
use codex_hepta_contracts::ProductComponentId;
use codex_hepta_contracts::Sha256Digest;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use uuid::Uuid;

use crate::AutomationError;

pub const MAX_AUTOMATION_PROMPT_BYTES: usize = 32 * 1024;
pub const MAX_AUTOMATION_THREAD_ID_BYTES: usize = 512;
pub const MAX_AUTOMATION_INTERVAL_MS: u64 = 31_536_000_000;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AutomationSchedule {
    Once,
    Interval { every_ms: u64 },
}

impl AutomationSchedule {
    pub fn next_after(self, current_ms: u64) -> Result<Option<u64>, AutomationError> {
        match self {
            Self::Once => Ok(None),
            Self::Interval { every_ms } => current_ms
                .checked_add(every_ms)
                .map(Some)
                .ok_or(AutomationError::Invalid),
        }
    }

    pub(crate) fn validate(self) -> Result<(), AutomationError> {
        match self {
            Self::Once => Ok(()),
            Self::Interval { every_ms } if (1_000..=MAX_AUTOMATION_INTERVAL_MS).contains(&every_ms) => {
                Ok(())
            }
            Self::Interval { .. } => Err(AutomationError::Invalid),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationTaskState {
    Enabled,
    Disabled,
    Cancelled,
}

impl AutomationTaskState {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::Cancelled => "cancelled",
        }
    }

    pub(crate) fn parse(value: &str) -> Result<Self, AutomationError> {
        match value {
            "enabled" => Ok(Self::Enabled),
            "disabled" => Ok(Self::Disabled),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(AutomationError::Corrupt),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AutomationTaskId(Uuid);

impl AutomationTaskId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn parse(value: &str) -> Result<Self, AutomationError> {
        Uuid::parse_str(value)
            .map(Self)
            .map_err(|_| AutomationError::Invalid)
    }
}

impl Default for AutomationTaskId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AutomationTaskId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for AutomationTaskId {
    type Err = AutomationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl Serialize for AutomationTaskId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for AutomationTaskId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AutomationTaskDraft {
    pub task_id: AutomationTaskId,
    pub thread_id: String,
    pub prompt: String,
    pub schedule: AutomationSchedule,
    pub first_run_at_ms: u64,
    pub created_at_ms: u64,
}

impl AutomationTaskDraft {
    pub(crate) fn validate(&self) -> Result<(), AutomationError> {
        validate_text(
            &self.thread_id,
            MAX_AUTOMATION_THREAD_ID_BYTES,
            "thread id",
        )?;
        validate_text(&self.prompt, MAX_AUTOMATION_PROMPT_BYTES, "prompt")?;
        if self.first_run_at_ms < self.created_at_ms {
            return Err(AutomationError::Invalid);
        }
        self.schedule.validate()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AutomationTask {
    pub task_id: AutomationTaskId,
    pub owner_agent_id: AgentId,
    pub thread_id: String,
    pub prompt: String,
    pub schedule: AutomationSchedule,
    pub state: AutomationTaskState,
    pub next_run_at_ms: Option<u64>,
    pub next_occurrence: u64,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutomationOperationContext {
    pub authority_epoch: u64,
    pub owner_epoch: u64,
    pub generation: u64,
    pub fencing_token_sha256: Sha256Digest,
}

impl AutomationOperationContext {
    pub fn new(
        authority_epoch: u64,
        owner_epoch: u64,
        generation: u64,
        fencing_token_sha256: Sha256Digest,
    ) -> Result<Self, AutomationError> {
        if authority_epoch == 0 || owner_epoch == 0 || generation == 0 {
            return Err(AutomationError::Invalid);
        }
        Ok(Self {
            authority_epoch,
            owner_epoch,
            generation,
            fencing_token_sha256,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutomationLease {
    pub task: AutomationTask,
    pub occurrence: u64,
    pub scheduled_for_ms: u64,
    pub client_user_message_id: String,
    pub lease_generation: u64,
    pub lease_token: String,
    pub lease_expires_at_ms: u64,
}

impl AutomationLease {
    pub fn admission(
        &self,
        context: &AutomationOperationContext,
    ) -> Result<AutomationAdmission, AutomationError> {
        if context.generation != self.lease_generation {
            return Err(AutomationError::AccessDenied);
        }
        let command = operation_command(self);
        let command_bytes = u64::try_from(command.len()).map_err(|_| AutomationError::Invalid)?;
        let binding = OperationBinding::new(
            OperationId::parse(format!(
                "automation:queue:v2:{}:{}",
                self.task.task_id, self.occurrence
            ))
            .map_err(|_| AutomationError::Invalid)?,
            IdempotencyKey::parse(self.client_user_message_id.clone())
                .map_err(|_| AutomationError::Invalid)?,
            self.task.owner_agent_id.clone(),
            ProductComponentId::AutomationRuntime,
            self.task.owner_agent_id.clone(),
            ProductComponentId::AppServer,
            AuthorityAction::MutateAutomation,
            context.authority_epoch,
            context.owner_epoch,
            context.generation,
            operation_fence(context, &self.lease_token),
            Sha256Digest::for_bytes(&command),
            command_bytes,
        )
        .map_err(|_| AutomationError::Invalid)?;
        let operation = OutboxEnvelope::pending(binding, 1).map_err(|_| AutomationError::Invalid)?;
        Ok(AutomationAdmission {
            owner_agent_id: self.task.owner_agent_id.clone(),
            task_id: self.task.task_id,
            occurrence: self.occurrence,
            scheduled_for_ms: self.scheduled_for_ms,
            thread_id: self.task.thread_id.clone(),
            prompt: self.task.prompt.clone(),
            client_user_message_id: self.client_user_message_id.clone(),
            operation,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutomationAdmission {
    pub owner_agent_id: AgentId,
    pub task_id: AutomationTaskId,
    pub occurrence: u64,
    pub scheduled_for_ms: u64,
    pub thread_id: String,
    pub prompt: String,
    pub client_user_message_id: String,
    pub operation: OutboxEnvelope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutomationQueueReceipt {
    pub client_user_message_id: String,
    pub queued_submission_id: String,
    pub acknowledgement: DestinationAcknowledgement,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutomationDispatchUncertainty {
    pub task_id: AutomationTaskId,
    pub occurrence: u64,
    pub scheduled_for_ms: u64,
    pub client_user_message_id: String,
    pub observed_at_ms: u64,
    /// Legacy pre-v4 rows remain `None` and are quarantined from automatic
    /// reconciliation until an operator/provider proves their identity.
    pub operation_id: Option<String>,
    pub operation_binding_sha256: Option<Sha256Digest>,
    pub operation_sequence: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AutomationTick {
    Idle,
    Submitted {
        task_id: AutomationTaskId,
        occurrence: u64,
        queued_submission_id: String,
    },
    RetryScheduled {
        task_id: AutomationTaskId,
        occurrence: u64,
    },
    DispatchUncertain {
        task_id: AutomationTaskId,
        occurrence: u64,
    },
}

pub(crate) fn client_message_id(
    owner_agent_id: &AgentId,
    task_id: AutomationTaskId,
    occurrence: u64,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"hepta:automation:client-message:v1\0");
    hasher.update(owner_agent_id.as_str().as_bytes());
    hasher.update(b"\0");
    hasher.update(task_id.to_string().as_bytes());
    hasher.update(b"\0");
    hasher.update(occurrence.to_be_bytes());
    format!("auto-{:x}", hasher.finalize())
}

fn operation_command(lease: &AutomationLease) -> Vec<u8> {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:automation:app-server-admission:v2");
    frame(&mut bytes, lease.task.owner_agent_id.as_str().as_bytes());
    frame(&mut bytes, lease.task.task_id.to_string().as_bytes());
    frame(&mut bytes, &lease.occurrence.to_be_bytes());
    frame(&mut bytes, &lease.scheduled_for_ms.to_be_bytes());
    frame(&mut bytes, lease.task.thread_id.as_bytes());
    frame(&mut bytes, lease.task.prompt.as_bytes());
    frame(&mut bytes, lease.client_user_message_id.as_bytes());
    bytes
}

fn operation_fence(
    context: &AutomationOperationContext,
    lease_token: &str,
) -> Sha256Digest {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:automation:operation-fence:v1");
    frame(
        &mut bytes,
        context.fencing_token_sha256.as_str().as_bytes(),
    );
    frame(&mut bytes, lease_token.as_bytes());
    Sha256Digest::for_bytes(&bytes)
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

fn validate_text(value: &str, maximum: usize, _label: &str) -> Result<(), AutomationError> {
    if value.is_empty() || value.len() > maximum || value.as_bytes().contains(&0) {
        Err(AutomationError::Invalid)
    } else {
        Ok(())
    }
}

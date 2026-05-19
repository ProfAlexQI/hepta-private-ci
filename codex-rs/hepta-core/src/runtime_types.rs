use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CorrelationId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThinkingLevel {
    Low,
    Medium,
    High,
    XHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionMode {
    Interactive,
    Background,
    Scheduled,
    Subagent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    SessionStarted,
    SessionSwitched,
    SessionRenamed,
    SessionArchived,
    SessionUnarchived,
    SessionDeleted,
    SessionsPruned,
    MessageReceived,
    ModelCalled,
    ModelSwitched,
    ExecutionProfileChanged,
    FilesystemScopeChanged,
    CapabilityGateChanged,
    WritePathScopeChanged,
    ToolInvoked,
    ApprovalRequested,
    ApprovalGranted,
    PolicyUpdated,
    MemoryWritten,
    SessionExported,
    SessionImported,
    SessionForked,
    SessionMerged,
    SnapshotSaved,
    SnapshotLoaded,
    BackupRestored,
    BackupsPruned,
    WriteTransactionRecorded,
    WriteTransactionGroupOpened,
    WriteTransactionGroupClosed,
    WriteLocksAcquired,
    WriteLocksReleased,
    WriteLocksPruned,
    WriteLockConflict,
    WriteRolledBack,
    WriteGroupRollbackFailed,
    WriteGroupRollbackResumed,
    WriteGroupRolledBack,
    TaskSpawned,
    TaskScheduled,
    TaskStarted,
    TaskSteered,
    TaskPaused,
    TaskResumed,
    TaskCompleted,
    TaskFailed,
    TaskCancelled,
    TaskInterrupted,
    AgentRegistered,
    AgentMessageQueued,
    AgentPaused,
    AgentResumed,
    AgentStopped,
    AgentSteered,
    AgentDrained,
    AgentRunStarted,
    AgentRunCompleted,
    AgentRunFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub kind: EventKind,
    pub session_id: Option<SessionId>,
    pub agent_id: Option<AgentId>,
    pub correlation_id: Option<CorrelationId>,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
}

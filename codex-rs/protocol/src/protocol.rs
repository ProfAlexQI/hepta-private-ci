//! Defines the protocol for a Hepta session between a client and an agent.
//!
//! Uses a SQ (Submission Queue) / EQ (Event Queue) pattern to asynchronously communicate
//! between user and agent.

use std::collections::HashMap;
use std::fmt;
use std::ops::Mul;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use strum_macros::EnumIter;

use crate::AgentPath;
use crate::SessionId;
use crate::ThreadId;
use crate::approvals::ElicitationRequestEvent;
use crate::config_types::ApprovalsReviewer;
use crate::config_types::CollaborationMode;
use crate::config_types::ModeKind;
use crate::config_types::Personality;
use crate::config_types::ReasoningSummary as ReasoningSummaryConfig;
use crate::config_types::WindowsSandboxLevel;
use crate::dynamic_tools::DynamicToolCallOutputContentItem;
use crate::dynamic_tools::DynamicToolCallRequest;
use crate::dynamic_tools::DynamicToolResponse;
use crate::dynamic_tools::DynamicToolSpec;
use crate::items::TurnItem;
use crate::mcp::CallToolResult;
use crate::mcp::RequestId;
use crate::memory_citation::MemoryCitation;
use crate::models::ActivePermissionProfile;
use crate::models::BaseInstructions;
use crate::models::ContentItem;
use crate::models::ImageDetail;
use crate::models::MessagePhase;
use crate::models::PermissionProfile;
use crate::models::ResponseInputItem;
use crate::models::ResponseItem;
use crate::models::SandboxEnforcement;
use crate::models::WebSearchAction;
use crate::num_format::format_with_separators;
use crate::openai_models::ReasoningEffort as ReasoningEffortConfig;
use crate::parse_command::ParsedCommand;
use crate::plan_tool::UpdatePlanArgs;
use crate::request_permissions::RequestPermissionsEvent;
use crate::request_permissions::RequestPermissionsResponse;
use crate::request_user_input::RequestUserInputResponse;
use crate::user_input::UserInput;
use codex_utils_absolute_path::AbsolutePathBuf;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_with::serde_as;
use strum_macros::Display;
use tracing::error;
use ts_rs::TS;

pub use crate::approvals::ApplyPatchApprovalRequestEvent;
pub use crate::approvals::ElicitationAction;
pub use crate::approvals::ExecApprovalRequestEvent;
pub use crate::approvals::ExecPolicyAmendment;
pub use crate::approvals::GuardianAssessmentAction;
pub use crate::approvals::GuardianAssessmentDecisionSource;
pub use crate::approvals::GuardianAssessmentEvent;
pub use crate::approvals::GuardianAssessmentOutcome;
pub use crate::approvals::GuardianAssessmentStatus;
pub use crate::approvals::GuardianCommandSource;
pub use crate::approvals::GuardianRiskLevel;
pub use crate::approvals::GuardianUserAuthorization;
pub use crate::approvals::NetworkApprovalContext;
pub use crate::approvals::NetworkApprovalProtocol;
pub use crate::approvals::NetworkPolicyAmendment;
pub use crate::approvals::NetworkPolicyRuleAction;
pub use crate::permissions::FileSystemAccessMode;
pub use crate::permissions::FileSystemPath;
pub use crate::permissions::FileSystemSandboxEntry;
pub use crate::permissions::FileSystemSandboxKind;
pub use crate::permissions::FileSystemSandboxPolicy;
pub use crate::permissions::FileSystemSpecialPath;
pub use crate::permissions::NetworkSandboxPolicy;
use crate::permissions::default_read_only_subpaths_for_writable_root;
pub use crate::request_permissions::RequestPermissionsArgs;
pub use crate::request_user_input::RequestUserInputEvent;

/// Open/close tags for special user-input blocks. Used across crates to avoid
/// duplicated hardcoded strings.
pub const USER_INSTRUCTIONS_OPEN_TAG: &str = "<user_instructions>";
pub const USER_INSTRUCTIONS_CLOSE_TAG: &str = "</user_instructions>";
pub const ENVIRONMENT_CONTEXT_OPEN_TAG: &str = "<environment_context>";
pub const ENVIRONMENT_CONTEXT_CLOSE_TAG: &str = "</environment_context>";
pub const APPS_INSTRUCTIONS_OPEN_TAG: &str = "<apps_instructions>";
pub const APPS_INSTRUCTIONS_CLOSE_TAG: &str = "</apps_instructions>";
pub const SKILLS_INSTRUCTIONS_OPEN_TAG: &str = "<skills_instructions>";
pub const SKILLS_INSTRUCTIONS_CLOSE_TAG: &str = "</skills_instructions>";
pub const PLUGINS_INSTRUCTIONS_OPEN_TAG: &str = "<plugins_instructions>";
pub const PLUGINS_INSTRUCTIONS_CLOSE_TAG: &str = "</plugins_instructions>";
pub const COLLABORATION_MODE_OPEN_TAG: &str = "<collaboration_mode>";
pub const COLLABORATION_MODE_CLOSE_TAG: &str = "</collaboration_mode>";
pub const REALTIME_CONVERSATION_OPEN_TAG: &str = "<realtime_conversation>";
pub const REALTIME_CONVERSATION_CLOSE_TAG: &str = "</realtime_conversation>";
pub const USER_MESSAGE_BEGIN: &str = "## My request for Hepta:";
pub const LEGACY_USER_MESSAGE_BEGIN: &str = "## My request for Codex:";

pub fn find_user_message_begin(text: &str) -> Option<(usize, &'static str)> {
    [USER_MESSAGE_BEGIN, LEGACY_USER_MESSAGE_BEGIN]
        .into_iter()
        .filter_map(|marker| text.rfind(marker).map(|idx| (idx, marker)))
        .max_by_key(|(idx, _)| *idx)
}

pub fn strip_user_message_context(text: &str) -> &str {
    match find_user_message_begin(text) {
        Some((idx, marker)) => text[idx + marker.len()..].trim(),
        None => text.trim(),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct TurnEnvironmentSelection {
    pub environment_id: String,
    pub cwd: AbsolutePathBuf,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, TS)]
#[serde(transparent)]
#[ts(type = "string")]
pub struct GitSha(pub String);

impl GitSha {
    pub fn new(sha: &str) -> Self {
        Self(sha.to_string())
    }
}

/// Submission Queue Entry - requests from user
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Submission {
    /// Unique id for this Submission to correlate with Events
    pub id: String,
    /// Payload
    pub op: Op,
    /// Optional W3C trace carrier propagated across async submission handoffs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace: Option<W3cTraceContext>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct W3cTraceContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub traceparent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub tracestate: Option<String>,
}

/// Config payload for refreshing MCP servers.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct McpServerRefreshConfig {
    pub mcp_servers: Value,
    pub mcp_oauth_credentials_store_mode: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct ConversationStartParams {
    /// Selects whether the realtime session should produce text or audio output.
    pub output_modality: RealtimeOutputModality,
    #[serde(
        default,
        deserialize_with = "conversation_start_prompt_serde::deserialize",
        serialize_with = "conversation_start_prompt_serde::serialize",
        skip_serializing_if = "Option::is_none"
    )]
    pub prompt: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<ConversationStartTransport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<RealtimeVoice>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[ts(tag = "type")]
pub enum ConversationStartTransport {
    Websocket,
    Webrtc { sdp: String },
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeOutputModality {
    Text,
    Audio,
}

mod conversation_start_prompt_serde {
    use serde::Deserializer;
    use serde::Serializer;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Option<String>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_with::rust::double_option::deserialize(deserializer)
    }

    pub(crate) fn serialize<S>(
        value: &Option<Option<String>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serde_with::rust::double_option::serialize(value, serializer)
    }
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, JsonSchema, TS, Ord, PartialOrd,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum RealtimeVoice {
    Alloy,
    Arbor,
    Ash,
    Ballad,
    Breeze,
    Cedar,
    Coral,
    Cove,
    Echo,
    Ember,
    Juniper,
    Maple,
    Marin,
    Sage,
    Shimmer,
    Sol,
    Spruce,
    Vale,
    Verse,
}

impl RealtimeVoice {
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::Alloy => "alloy",
            Self::Arbor => "arbor",
            Self::Ash => "ash",
            Self::Ballad => "ballad",
            Self::Breeze => "breeze",
            Self::Cedar => "cedar",
            Self::Coral => "coral",
            Self::Cove => "cove",
            Self::Echo => "echo",
            Self::Ember => "ember",
            Self::Juniper => "juniper",
            Self::Maple => "maple",
            Self::Marin => "marin",
            Self::Sage => "sage",
            Self::Shimmer => "shimmer",
            Self::Sol => "sol",
            Self::Spruce => "spruce",
            Self::Vale => "vale",
            Self::Verse => "verse",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename_all = "camelCase")]
pub struct RealtimeVoicesList {
    pub v1: Vec<RealtimeVoice>,
    pub v2: Vec<RealtimeVoice>,
    pub default_v1: RealtimeVoice,
    pub default_v2: RealtimeVoice,
}

impl RealtimeVoicesList {
    pub fn builtin() -> Self {
        Self {
            v1: vec![
                RealtimeVoice::Juniper,
                RealtimeVoice::Maple,
                RealtimeVoice::Spruce,
                RealtimeVoice::Ember,
                RealtimeVoice::Vale,
                RealtimeVoice::Breeze,
                RealtimeVoice::Arbor,
                RealtimeVoice::Sol,
                RealtimeVoice::Cove,
            ],
            v2: vec![
                RealtimeVoice::Alloy,
                RealtimeVoice::Ash,
                RealtimeVoice::Ballad,
                RealtimeVoice::Coral,
                RealtimeVoice::Echo,
                RealtimeVoice::Sage,
                RealtimeVoice::Shimmer,
                RealtimeVoice::Verse,
                RealtimeVoice::Marin,
                RealtimeVoice::Cedar,
            ],
            default_v1: RealtimeVoice::Cove,
            default_v2: RealtimeVoice::Marin,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct RealtimeAudioFrame {
    pub data: String,
    pub sample_rate: u32,
    pub num_channels: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samples_per_channel: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct RealtimeTranscriptDelta {
    pub delta: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct RealtimeTranscriptDone {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct RealtimeTranscriptEntry {
    pub role: String,
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct RealtimeHandoffRequested {
    pub handoff_id: String,
    pub item_id: String,
    pub input_transcript: String,
    pub active_transcript: Vec<RealtimeTranscriptEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct RealtimeNoopRequested {
    pub call_id: String,
    pub item_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct RealtimeInputAudioSpeechStarted {
    pub item_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct RealtimeResponseCancelled {
    pub response_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct RealtimeResponseCreated {
    pub response_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct RealtimeResponseDone {
    pub response_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub enum RealtimeEvent {
    SessionUpdated {
        realtime_session_id: String,
        instructions: Option<String>,
    },
    InputAudioSpeechStarted(RealtimeInputAudioSpeechStarted),
    InputTranscriptDelta(RealtimeTranscriptDelta),
    InputTranscriptDone(RealtimeTranscriptDone),
    OutputTranscriptDelta(RealtimeTranscriptDelta),
    OutputTranscriptDone(RealtimeTranscriptDone),
    AudioOut(RealtimeAudioFrame),
    ResponseCreated(RealtimeResponseCreated),
    ResponseCancelled(RealtimeResponseCancelled),
    ResponseDone(RealtimeResponseDone),
    ConversationItemAdded(Value),
    ConversationItemDone {
        item_id: String,
    },
    HandoffRequested(RealtimeHandoffRequested),
    NoopRequested(RealtimeNoopRequested),
    Error(String),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct ConversationAudioParams {
    pub frame: RealtimeAudioFrame,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct ConversationTextParams {
    pub text: String,
}

/// Submission operation
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum Op {
    /// Abort current task without terminating background terminal processes.
    /// This server sends [`EventMsg::TurnAborted`] in response.
    Interrupt,

    /// Terminate all running background terminal processes for this thread.
    /// Use this when callers intentionally want to stop long-lived background shells.
    CleanBackgroundTerminals,

    /// Start a realtime conversation stream.
    RealtimeConversationStart(ConversationStartParams),

    /// Send audio input to the running realtime conversation stream.
    RealtimeConversationAudio(ConversationAudioParams),

    /// Send text input to the running realtime conversation stream.
    RealtimeConversationText(ConversationTextParams),

    /// Close the running realtime conversation stream.
    RealtimeConversationClose,

    /// Request the list of voices supported by realtime conversation streams.
    RealtimeConversationListVoices,

    /// Legacy user input.
    ///
    /// Prefer [`Op::UserTurn`] so the caller provides full turn context
    /// (cwd/approval/sandbox/model/etc.) for each turn.
    UserInput {
        /// User input items, see `InputItem`
        items: Vec<UserInput>,
        /// Optional turn-scoped environments.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        environments: Option<Vec<TurnEnvironmentSelection>>,
        /// Optional JSON Schema used to constrain the final assistant message for this turn.
        #[serde(skip_serializing_if = "Option::is_none")]
        final_output_json_schema: Option<Value>,
        /// Optional turn-scoped Responses API `client_metadata`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        responsesapi_client_metadata: Option<HashMap<String, String>>,
    },

    /// Similar to [`Op::UserInput`], but first applies persistent turn-context
    /// overrides in the same queued operation. This preserves submission order
    /// and prevents the input from starting if the overrides are rejected.
    UserInputWithTurnContext {
        /// User input items, see `InputItem`
        items: Vec<UserInput>,
        /// Optional turn-scoped environment selections.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        environments: Option<Vec<TurnEnvironmentSelection>>,
        /// Optional JSON Schema used to constrain the final assistant message for this turn.
        #[serde(skip_serializing_if = "Option::is_none")]
        final_output_json_schema: Option<Value>,
        /// Optional turn-scoped Responses API `client_metadata`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        responsesapi_client_metadata: Option<HashMap<String, String>>,
        /// Optional selected recall snippets for the current turn.
        ///
        /// This is a request-path handoff surface only. Core/session validates
        /// the envelope again before manifest persistence or live prompt use.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        context_recall_selected_snippets: Option<TurnContextRecallSelectedSnippetEnvelope>,

        /// Updated `cwd` for sandbox/tool calls.
        #[serde(skip_serializing_if = "Option::is_none")]
        cwd: Option<PathBuf>,

        /// Updated runtime workspace roots used to materialize symbolic
        /// `:workspace_roots` filesystem permissions.
        #[serde(skip_serializing_if = "Option::is_none")]
        workspace_roots: Option<Vec<AbsolutePathBuf>>,

        /// Updated profile-defined workspace roots for status summaries and
        /// per-turn config reconstruction.
        #[serde(skip_serializing_if = "Option::is_none")]
        profile_workspace_roots: Option<Vec<AbsolutePathBuf>>,

        /// Updated command approval policy.
        #[serde(skip_serializing_if = "Option::is_none")]
        approval_policy: Option<AskForApproval>,

        /// Updated approval reviewer for future approval prompts.
        #[serde(skip_serializing_if = "Option::is_none")]
        approvals_reviewer: Option<ApprovalsReviewer>,

        /// Updated sandbox policy for tool calls.
        #[serde(skip_serializing_if = "Option::is_none")]
        sandbox_policy: Option<SandboxPolicy>,

        /// Updated permissions profile for tool calls.
        #[serde(skip_serializing_if = "Option::is_none")]
        permission_profile: Option<PermissionProfile>,

        /// Named or built-in profile that produced `permission_profile`, if
        /// the update selected a profile rather than supplying raw
        /// permissions.
        #[serde(skip_serializing_if = "Option::is_none")]
        active_permission_profile: Option<ActivePermissionProfile>,

        /// Updated Windows sandbox mode for tool execution.
        #[serde(skip_serializing_if = "Option::is_none")]
        windows_sandbox_level: Option<WindowsSandboxLevel>,

        /// Updated model slug. When set, the model info is derived
        /// automatically.
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,

        /// Updated reasoning effort (honored only for reasoning-capable models).
        ///
        /// Use `Some(Some(_))` to set a specific effort, `Some(None)` to clear
        /// the effort, or `None` to leave the existing value unchanged.
        #[serde(skip_serializing_if = "Option::is_none")]
        effort: Option<Option<ReasoningEffortConfig>>,

        /// Updated reasoning summary preference (honored only for reasoning-capable models).
        #[serde(skip_serializing_if = "Option::is_none")]
        summary: Option<ReasoningSummaryConfig>,

        /// Updated service tier preference for future turns.
        ///
        /// Use `Some(Some(_))` to set a specific tier, `Some(None)` to clear the
        /// preference, or `None` to leave the existing value unchanged.
        #[serde(skip_serializing_if = "Option::is_none")]
        service_tier: Option<Option<String>>,

        /// EXPERIMENTAL - set a pre-set collaboration mode.
        /// Takes precedence over model, effort, and developer instructions if set.
        #[serde(skip_serializing_if = "Option::is_none")]
        collaboration_mode: Option<CollaborationMode>,

        /// Updated personality preference.
        #[serde(skip_serializing_if = "Option::is_none")]
        personality: Option<Personality>,
    },

    /// Similar to [`Op::UserInput`], but contains additional context required
    /// for a turn of a [`crate::codex_thread::CodexThread`].
    UserTurn {
        /// User input items, see `InputItem`
        items: Vec<UserInput>,

        /// `cwd` to use with the [`SandboxPolicy`] and potentially tool calls
        /// such as `local_shell`.
        cwd: PathBuf,

        /// Policy to use for command approval.
        approval_policy: AskForApproval,

        /// Reviewer to use for approval requests raised during this turn.
        ///
        /// When omitted, the session keeps the current setting
        approvals_reviewer: Option<ApprovalsReviewer>,

        /// Policy to use for tool calls such as `local_shell`.
        sandbox_policy: SandboxPolicy,

        /// Full permissions profile to use for tool calls such as `local_shell`.
        ///
        /// When omitted, `sandbox_policy` is used as a legacy compatibility
        /// projection.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        permission_profile: Option<PermissionProfile>,

        /// Must be a valid model slug for the configured client session
        /// associated with this conversation.
        model: String,

        /// Will only be honored if the model is configured to use reasoning.
        #[serde(skip_serializing_if = "Option::is_none")]
        effort: Option<ReasoningEffortConfig>,

        /// Will only be honored if the model is configured to use reasoning.
        ///
        /// When omitted, the session keeps the current setting (which allows core to
        /// fall back to the selected model's default on new sessions).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        summary: Option<ReasoningSummaryConfig>,

        /// Optional service tier override for this turn.
        ///
        /// Use `Some(Some(_))` to set a specific tier for this turn, `Some(None)` to
        /// explicitly clear the tier for this turn, or `None` to keep the existing
        /// session preference.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        service_tier: Option<Option<String>>,

        // The JSON schema to use for the final assistant message
        final_output_json_schema: Option<Value>,

        /// EXPERIMENTAL - set a pre-set collaboration mode.
        /// Takes precedence over model, effort, and developer instructions if set.
        #[serde(skip_serializing_if = "Option::is_none")]
        collaboration_mode: Option<CollaborationMode>,

        /// Optional personality override for this turn.
        #[serde(skip_serializing_if = "Option::is_none")]
        personality: Option<Personality>,

        /// Optional turn-scoped environments.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        environments: Option<Vec<TurnEnvironmentSelection>>,
    },

    /// Inter-agent communication that should be recorded as assistant history
    /// while still using the normal thread submission lifecycle.
    InterAgentCommunication {
        communication: InterAgentCommunication,
    },

    /// Override parts of the persistent turn context for subsequent turns.
    ///
    /// All fields are optional; when omitted, the existing value is preserved.
    /// This does not enqueue any input – it only updates defaults used for
    /// turns that rely on persistent session-level context (for example,
    /// [`Op::UserInput`]).
    OverrideTurnContext {
        /// Updated `cwd` for sandbox/tool calls.
        #[serde(skip_serializing_if = "Option::is_none")]
        cwd: Option<PathBuf>,

        /// Updated command approval policy.
        #[serde(skip_serializing_if = "Option::is_none")]
        approval_policy: Option<AskForApproval>,

        /// Updated approval reviewer for future approval prompts.
        #[serde(skip_serializing_if = "Option::is_none")]
        approvals_reviewer: Option<ApprovalsReviewer>,

        /// Updated sandbox policy for tool calls.
        #[serde(skip_serializing_if = "Option::is_none")]
        sandbox_policy: Option<SandboxPolicy>,

        /// Updated permissions profile for tool calls.
        #[serde(skip_serializing_if = "Option::is_none")]
        permission_profile: Option<PermissionProfile>,

        /// Updated Windows sandbox mode for tool execution.
        #[serde(skip_serializing_if = "Option::is_none")]
        windows_sandbox_level: Option<WindowsSandboxLevel>,

        /// Updated model slug. When set, the model info is derived
        /// automatically.
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,

        /// Updated reasoning effort (honored only for reasoning-capable models).
        ///
        /// Use `Some(Some(_))` to set a specific effort, `Some(None)` to clear
        /// the effort, or `None` to leave the existing value unchanged.
        #[serde(skip_serializing_if = "Option::is_none")]
        effort: Option<Option<ReasoningEffortConfig>>,

        /// Updated reasoning summary preference (honored only for reasoning-capable models).
        #[serde(skip_serializing_if = "Option::is_none")]
        summary: Option<ReasoningSummaryConfig>,

        /// Updated service tier preference for future turns.
        ///
        /// Use `Some(Some(_))` to set a specific tier, `Some(None)` to clear the
        /// preference, or `None` to leave the existing value unchanged.
        #[serde(skip_serializing_if = "Option::is_none")]
        service_tier: Option<Option<String>>,

        /// EXPERIMENTAL - set a pre-set collaboration mode.
        /// Takes precedence over model, effort, and developer instructions if set.
        #[serde(skip_serializing_if = "Option::is_none")]
        collaboration_mode: Option<CollaborationMode>,

        /// Updated personality preference.
        #[serde(skip_serializing_if = "Option::is_none")]
        personality: Option<Personality>,
    },

    /// Approve a command execution
    ExecApproval {
        /// The id of the submission we are approving
        id: String,
        /// Turn id associated with the approval event, when available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        turn_id: Option<String>,
        /// The user's decision in response to the request.
        decision: ReviewDecision,
    },

    /// Approve a code patch
    PatchApproval {
        /// The id of the submission we are approving
        id: String,
        /// The user's decision in response to the request.
        decision: ReviewDecision,
    },

    /// Resolve an MCP elicitation request.
    ResolveElicitation {
        /// Name of the MCP server that issued the request.
        server_name: String,
        /// Request identifier from the MCP server.
        request_id: RequestId,
        /// User's decision for the request.
        decision: ElicitationAction,
        /// Structured user input supplied for accepted elicitations.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        content: Option<Value>,
        /// Optional client metadata associated with the elicitation response.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        meta: Option<Value>,
    },

    /// Resolve a request_user_input tool call.
    #[serde(rename = "user_input_answer", alias = "request_user_input_response")]
    UserInputAnswer {
        /// Turn id for the in-flight request.
        id: String,
        /// User-provided answers.
        response: RequestUserInputResponse,
    },

    /// Resolve a request_permissions tool call.
    RequestPermissionsResponse {
        /// Call id for the in-flight request.
        id: String,
        /// User-granted permissions.
        response: RequestPermissionsResponse,
    },

    /// Resolve a dynamic tool call request.
    DynamicToolResponse {
        /// Call id for the in-flight request.
        id: String,
        /// Tool output payload.
        response: DynamicToolResponse,
    },

    /// Request MCP servers to reinitialize and refresh cached tool lists.
    RefreshMcpServers { config: McpServerRefreshConfig },

    /// Reload user config layer overrides for the active session.
    ///
    /// This updates runtime config-derived behavior (for example app
    /// enable/disable state) without restarting the thread.
    ReloadUserConfig,

    /// Request the agent to summarize the current conversation context.
    /// The agent will use its existing context (either conversation history or previous response id)
    /// to generate a summary which will be returned as an AgentMessage event.
    Compact,

    /// Set whether the thread remains eligible for memory generation.
    ///
    /// This persists thread-level memory mode metadata without involving the
    /// model.
    SetThreadMemoryMode { mode: ThreadMemoryMode },

    /// Request Hepta to drop the last N user turns from in-memory context.
    ///
    /// This does not attempt to revert local filesystem changes. Clients are
    /// responsible for undoing any edits on disk.
    ThreadRollback { num_turns: u32 },

    /// Request a code review from the agent.
    Review { review_request: ReviewRequest },

    /// Record that the user approved one retry of a concrete Guardian-denied action.
    ApproveGuardianDeniedAction { event: GuardianAssessmentEvent },

    /// Request to shut down the Hepta instance.
    Shutdown,

    /// Execute a user-initiated one-off shell command (triggered by "!cmd").
    ///
    /// The command string is executed using the user's default shell and may
    /// include shell syntax (pipes, redirects, etc.). Output is streamed via
    /// `ExecCommand*` events and the UI regains control upon `TurnComplete`.
    RunUserShellCommand {
        /// The raw command string after '!'
        command: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ThreadMemoryMode {
    Enabled,
    Disabled,
}

/// Persisted storage contract for a thread's canonical history.
///
/// Legacy remains the default so binaries that predate paginated history keep
/// interpreting omitted metadata exactly as they do today. Callers must fail
/// closed before opening a paginated thread unless they implement that
/// contract end to end.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "lowercase")]
#[ts(rename_all = "lowercase")]
pub enum ThreadHistoryMode {
    #[default]
    Legacy,
    Paginated,
}

impl ThreadHistoryMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Legacy => "legacy",
            Self::Paginated => "paginated",
        }
    }
}

impl FromStr for ThreadHistoryMode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "legacy" => Ok(Self::Legacy),
            "paginated" => Ok(Self::Paginated),
            _ => Err(format!("unknown thread history mode `{value}`")),
        }
    }
}

impl From<Vec<UserInput>> for Op {
    fn from(value: Vec<UserInput>) -> Self {
        Op::UserInput {
            environments: None,
            items: value,
            final_output_json_schema: None,
            responsesapi_client_metadata: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema, TS)]
pub struct InterAgentCommunication {
    pub author: AgentPath,
    pub recipient: AgentPath,
    #[serde(default)]
    pub other_recipients: Vec<AgentPath>,
    pub content: String,
    pub trigger_turn: bool,
}

impl InterAgentCommunication {
    pub fn new(
        author: AgentPath,
        recipient: AgentPath,
        other_recipients: Vec<AgentPath>,
        content: String,
        trigger_turn: bool,
    ) -> Self {
        Self {
            author,
            recipient,
            other_recipients,
            content,
            trigger_turn,
        }
    }

    pub fn to_response_input_item(&self) -> ResponseInputItem {
        ResponseInputItem::Message {
            role: "assistant".to_string(),
            content: vec![ContentItem::OutputText {
                text: serde_json::to_string(self).unwrap_or_default(),
            }],
            phase: Some(MessagePhase::Commentary),
        }
    }

    pub fn is_message_content(content: &[ContentItem]) -> bool {
        Self::from_message_content(content).is_some()
    }

    pub fn from_message_content(content: &[ContentItem]) -> Option<Self> {
        match content {
            [ContentItem::InputText { text }] | [ContentItem::OutputText { text }] => {
                serde_json::from_str(text).ok()
            }
            _ => None,
        }
    }
}

impl Op {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Interrupt => "interrupt",
            Self::CleanBackgroundTerminals => "clean_background_terminals",
            Self::RealtimeConversationStart(_) => "realtime_conversation_start",
            Self::RealtimeConversationAudio(_) => "realtime_conversation_audio",
            Self::RealtimeConversationText(_) => "realtime_conversation_text",
            Self::RealtimeConversationClose => "realtime_conversation_close",
            Self::RealtimeConversationListVoices => "realtime_conversation_list_voices",
            Self::UserInput { .. } => "user_input",
            Self::UserInputWithTurnContext { .. } => "user_input_with_turn_context",
            Self::UserTurn { .. } => "user_turn",
            Self::InterAgentCommunication { .. } => "inter_agent_communication",
            Self::OverrideTurnContext { .. } => "override_turn_context",
            Self::ExecApproval { .. } => "exec_approval",
            Self::PatchApproval { .. } => "patch_approval",
            Self::ResolveElicitation { .. } => "resolve_elicitation",
            Self::UserInputAnswer { .. } => "user_input_answer",
            Self::RequestPermissionsResponse { .. } => "request_permissions_response",
            Self::DynamicToolResponse { .. } => "dynamic_tool_response",
            Self::RefreshMcpServers { .. } => "refresh_mcp_servers",
            Self::ReloadUserConfig => "reload_user_config",
            Self::Compact => "compact",
            Self::SetThreadMemoryMode { .. } => "set_thread_memory_mode",
            Self::ThreadRollback { .. } => "thread_rollback",
            Self::Review { .. } => "review",
            Self::ApproveGuardianDeniedAction { .. } => "approve_guardian_denied_action",
            Self::Shutdown => "shutdown",
            Self::RunUserShellCommand { .. } => "run_user_shell_command",
        }
    }
}

/// Determines the conditions under which the user is consulted to approve
/// running the command proposed by Hepta.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    JsonSchema,
    TS,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum AskForApproval {
    /// Under this policy, only "known safe" commands—as determined by
    /// `is_safe_command()`—that **only read files** are auto‑approved.
    /// Everything else will ask the user to approve.
    #[serde(rename = "untrusted")]
    #[strum(serialize = "untrusted")]
    UnlessTrusted,

    /// DEPRECATED: *All* commands are auto‑approved, but they are expected to
    /// run inside a sandbox where network access is disabled and writes are
    /// confined to a specific set of paths. If the command fails, it will be
    /// escalated to the user to approve execution without a sandbox.
    /// Prefer `OnRequest` for interactive runs or `Never` for non-interactive
    /// runs.
    OnFailure,

    /// The model decides when to ask the user for approval.
    #[default]
    OnRequest,

    /// Fine-grained controls for individual approval flows.
    ///
    /// When a field is `true`, commands in that category are allowed. When it
    /// is `false`, those requests are automatically rejected instead of shown
    /// to the user.
    #[strum(serialize = "granular")]
    Granular(GranularApprovalConfig),

    /// Never ask the user to approve commands. Failures are immediately returned
    /// to the model, and never escalated to the user for approval.
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TS)]
pub struct GranularApprovalConfig {
    /// Whether to allow shell command approval requests, including inline
    /// `with_additional_permissions` and `require_escalated` requests.
    pub sandbox_approval: bool,
    /// Whether to allow prompts triggered by execpolicy `prompt` rules.
    pub rules: bool,
    /// Whether to allow approval prompts triggered by skill script execution.
    #[serde(default)]
    pub skill_approval: bool,
    /// Whether to allow prompts triggered by the `request_permissions` tool.
    #[serde(default)]
    pub request_permissions: bool,
    /// Whether to allow MCP elicitation prompts.
    pub mcp_elicitations: bool,
}

impl GranularApprovalConfig {
    pub const fn allows_sandbox_approval(self) -> bool {
        self.sandbox_approval
    }

    pub const fn allows_rules_approval(self) -> bool {
        self.rules
    }

    pub const fn allows_skill_approval(self) -> bool {
        self.skill_approval
    }

    pub const fn allows_request_permissions(self) -> bool {
        self.request_permissions
    }

    pub const fn allows_mcp_elicitations(self) -> bool {
        self.mcp_elicitations
    }
}

/// Represents whether outbound network access is available to the agent.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, Default, JsonSchema, TS,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum NetworkAccess {
    #[default]
    Restricted,
    Enabled,
}

impl NetworkAccess {
    pub fn is_enabled(self) -> bool {
        matches!(self, NetworkAccess::Enabled)
    }
}

/// Determines execution restrictions for model shell commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, JsonSchema, TS)]
#[strum(serialize_all = "kebab-case")]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum SandboxPolicy {
    /// No restrictions whatsoever. Use with caution.
    #[serde(rename = "danger-full-access")]
    DangerFullAccess,

    /// Read-only access configuration.
    #[serde(rename = "read-only")]
    ReadOnly {
        /// When set to `true`, outbound network access is allowed. `false` by
        /// default.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        network_access: bool,
    },

    /// Indicates the process is already in an external sandbox. Allows full
    /// disk access while honoring the provided network setting.
    #[serde(rename = "external-sandbox")]
    ExternalSandbox {
        /// Whether the external sandbox permits outbound network traffic.
        #[serde(default)]
        network_access: NetworkAccess,
    },

    /// Same as `ReadOnly` but additionally grants write access to the current
    /// working directory ("workspace").
    #[serde(rename = "workspace-write")]
    WorkspaceWrite {
        /// Additional folders (beyond cwd and possibly TMPDIR) that should be
        /// writable from within the sandbox.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        writable_roots: Vec<AbsolutePathBuf>,

        /// When set to `true`, outbound network access is allowed. `false` by
        /// default.
        #[serde(default)]
        network_access: bool,

        /// When set to `true`, will NOT include the per-user `TMPDIR`
        /// environment variable among the default writable roots. Defaults to
        /// `false`.
        #[serde(default)]
        exclude_tmpdir_env_var: bool,

        /// When set to `true`, will NOT include the `/tmp` among the default
        /// writable roots on UNIX. Defaults to `false`.
        #[serde(default)]
        exclude_slash_tmp: bool,
    },
}

/// A writable root path accompanied by a list of subpaths that should remain
/// read‑only even when the root is writable. This is primarily used to ensure
/// that folders containing files that could be modified to escalate the
/// privileges of the agent (e.g. `.codex`, `.git`, notably `.git/hooks`) under
/// a writable root are not modified by the agent.
#[derive(Debug, Clone, PartialEq, Eq, JsonSchema)]
pub struct WritableRoot {
    pub root: AbsolutePathBuf,

    /// By construction, these subpaths are all under `root`.
    pub read_only_subpaths: Vec<AbsolutePathBuf>,

    /// Workspace metadata path names that must not be created or replaced under
    /// `root` unless the policy grants an explicit write rule for that metadata
    /// path.
    pub protected_metadata_names: Vec<String>,
}

impl WritableRoot {
    pub fn is_path_writable(&self, path: &Path) -> bool {
        // Check if the path is under the root.
        if !path.starts_with(&self.root) {
            return false;
        }

        // Check if the path is under any of the read-only subpaths.
        for subpath in &self.read_only_subpaths {
            if path.starts_with(subpath) {
                return false;
            }
        }

        if self.path_contains_protected_metadata_name(path) {
            return false;
        }

        true
    }

    fn path_contains_protected_metadata_name(&self, path: &Path) -> bool {
        let Ok(relative_path) = path.strip_prefix(&self.root) else {
            return false;
        };

        let Some(first_component) = relative_path.components().next() else {
            return false;
        };

        self.protected_metadata_names
            .iter()
            .any(|name| first_component.as_os_str() == std::ffi::OsStr::new(name))
    }
}

impl FromStr for SandboxPolicy {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl FromStr for FileSystemSandboxPolicy {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl FromStr for NetworkSandboxPolicy {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl SandboxPolicy {
    /// Returns a policy with read-only disk access and no network.
    pub fn new_read_only_policy() -> Self {
        SandboxPolicy::ReadOnly {
            network_access: false,
        }
    }

    /// Returns a policy that can read the entire disk, but can only write to
    /// the current working directory and the per-user tmp dir on macOS. It does
    /// not allow network access.
    pub fn new_workspace_write_policy() -> Self {
        SandboxPolicy::WorkspaceWrite {
            writable_roots: vec![],
            network_access: false,
            exclude_tmpdir_env_var: false,
            exclude_slash_tmp: false,
        }
    }

    pub fn has_full_disk_read_access(&self) -> bool {
        true
    }

    pub fn has_full_disk_write_access(&self) -> bool {
        match self {
            SandboxPolicy::DangerFullAccess => true,
            SandboxPolicy::ExternalSandbox { .. } => true,
            SandboxPolicy::ReadOnly { .. } => false,
            SandboxPolicy::WorkspaceWrite { .. } => false,
        }
    }

    pub fn has_full_network_access(&self) -> bool {
        match self {
            SandboxPolicy::DangerFullAccess => true,
            SandboxPolicy::ExternalSandbox { network_access } => network_access.is_enabled(),
            SandboxPolicy::ReadOnly { network_access, .. } => *network_access,
            SandboxPolicy::WorkspaceWrite { network_access, .. } => *network_access,
        }
    }

    /// Returns the list of writable roots (tailored to the current working
    /// directory) together with subpaths that should remain read‑only under
    /// each writable root.
    pub fn get_writable_roots_with_cwd(&self, cwd: &Path) -> Vec<WritableRoot> {
        match self {
            SandboxPolicy::DangerFullAccess => Vec::new(),
            SandboxPolicy::ExternalSandbox { .. } => Vec::new(),
            SandboxPolicy::ReadOnly { .. } => Vec::new(),
            SandboxPolicy::WorkspaceWrite {
                writable_roots,
                exclude_tmpdir_env_var,
                exclude_slash_tmp,
                network_access: _,
            } => {
                // Start from explicitly configured writable roots.
                let mut roots: Vec<AbsolutePathBuf> = writable_roots.clone();

                // Always include defaults: cwd, /tmp (if present on Unix), and
                // on macOS, the per-user TMPDIR unless explicitly excluded.
                // TODO(mbolin): cwd param should be AbsolutePathBuf.
                let cwd_absolute = AbsolutePathBuf::from_absolute_path(cwd);
                match cwd_absolute {
                    Ok(cwd) => {
                        roots.push(cwd);
                    }
                    Err(e) => {
                        error!(
                            "Ignoring invalid cwd {:?} for sandbox writable root: {}",
                            cwd, e
                        );
                    }
                }

                // Include /tmp on Unix unless explicitly excluded.
                if cfg!(unix) && !exclude_slash_tmp {
                    match AbsolutePathBuf::from_absolute_path("/tmp") {
                        Ok(slash_tmp) => {
                            if slash_tmp.as_path().is_dir() {
                                roots.push(slash_tmp);
                            }
                        }
                        Err(e) => {
                            error!("Ignoring invalid /tmp for sandbox writable root: {e}");
                        }
                    }
                }

                // Include $TMPDIR unless explicitly excluded. On macOS, TMPDIR
                // is per-user, so writes to TMPDIR should not be readable by
                // other users on the system.
                //
                // By comparison, TMPDIR is not guaranteed to be defined on
                // Linux or Windows, but supporting it here gives users a way to
                // provide the model with their own temporary directory without
                // having to hardcode it in the config.
                if !exclude_tmpdir_env_var
                    && let Some(tmpdir) = std::env::var_os("TMPDIR")
                    && !tmpdir.is_empty()
                {
                    match AbsolutePathBuf::from_absolute_path(PathBuf::from(&tmpdir)) {
                        Ok(tmpdir_path) => {
                            roots.push(tmpdir_path);
                        }
                        Err(e) => {
                            error!(
                                "Ignoring invalid TMPDIR value {tmpdir:?} for sandbox writable root: {e}",
                            );
                        }
                    }
                }

                // For each root, compute subpaths that should remain read-only.
                let cwd_root = AbsolutePathBuf::from_absolute_path(cwd).ok();
                roots
                    .into_iter()
                    .map(|writable_root| {
                        let protect_missing_dot_codex = cwd_root
                            .as_ref()
                            .is_some_and(|cwd_root| cwd_root == &writable_root);
                        WritableRoot {
                            read_only_subpaths: default_read_only_subpaths_for_writable_root(
                                &writable_root,
                                protect_missing_dot_codex,
                            ),
                            protected_metadata_names: Vec::new(),
                            root: writable_root,
                        }
                    })
                    .collect()
            }
        }
    }
}

/// Event Queue Entry - events from agent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    /// Submission `id` that this event is correlated with.
    pub id: String,
    /// Payload
    pub msg: EventMsg,
}

/// Response event from the agent
/// NOTE: Make sure none of these values have optional types, as it will mess up the extension code-gen.
#[derive(Debug, Clone, Deserialize, Serialize, Display, JsonSchema, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[ts(tag = "type")]
#[strum(serialize_all = "snake_case")]
pub enum EventMsg {
    /// Error while executing a submission
    Error(ErrorEvent),

    /// Warning issued while processing a submission. Unlike `Error`, this
    /// indicates the turn continued but the user should still be notified.
    Warning(WarningEvent),

    /// Warning issued by the guardian automatic approval reviewer.
    GuardianWarning(WarningEvent),

    /// Realtime conversation lifecycle start event.
    RealtimeConversationStarted(RealtimeConversationStartedEvent),

    /// Realtime conversation streaming payload event.
    RealtimeConversationRealtime(RealtimeConversationRealtimeEvent),

    /// Realtime conversation lifecycle close event.
    RealtimeConversationClosed(RealtimeConversationClosedEvent),

    /// Realtime session description protocol payload.
    RealtimeConversationSdp(RealtimeConversationSdpEvent),

    /// Model routing changed from the requested model to a different model.
    ModelReroute(ModelRerouteEvent),

    /// Backend recommends additional account verification for this turn.
    ModelVerification(ModelVerificationEvent),

    /// Conversation history was compacted (either automatically or manually).
    ContextCompacted(ContextCompactedEvent),

    /// Conversation history was rolled back by dropping the last N user turns.
    ThreadRolledBack(ThreadRolledBackEvent),

    /// Agent has started a turn.
    /// v1 wire format uses `task_started`; accept `turn_started` for v2 interop.
    #[serde(rename = "task_started", alias = "turn_started")]
    TurnStarted(TurnStartedEvent),

    /// Agent has completed all actions.
    /// v1 wire format uses `task_complete`; accept `turn_complete` for v2 interop.
    #[serde(rename = "task_complete", alias = "turn_complete")]
    TurnComplete(TurnCompleteEvent),

    /// Usage update for the current session, including totals and last turn.
    /// Optional means unknown — UIs should not display when `None`.
    TokenCount(TokenCountEvent),

    /// Agent text output message
    AgentMessage(AgentMessageEvent),

    /// User/system input message (what was sent to the model)
    UserMessage(UserMessageEvent),

    /// Reasoning event from agent.
    AgentReasoning(AgentReasoningEvent),

    /// Raw chain-of-thought from agent.
    AgentReasoningRawContent(AgentReasoningRawContentEvent),

    /// Signaled when the model begins a new reasoning summary section (e.g., a new titled block).
    AgentReasoningSectionBreak(AgentReasoningSectionBreakEvent),

    /// Ack the client's configure message.
    SessionConfigured(SessionConfiguredEvent),

    /// Updated long-running goal metadata for the thread.
    ThreadGoalUpdated(ThreadGoalUpdatedEvent),

    /// Incremental MCP startup progress updates.
    McpStartupUpdate(McpStartupUpdateEvent),

    /// Aggregate MCP startup completion summary.
    McpStartupComplete(McpStartupCompleteEvent),

    McpToolCallBegin(McpToolCallBeginEvent),

    McpToolCallEnd(McpToolCallEndEvent),

    WebSearchBegin(WebSearchBeginEvent),

    WebSearchEnd(WebSearchEndEvent),

    ImageGenerationBegin(ImageGenerationBeginEvent),

    ImageGenerationEnd(ImageGenerationEndEvent),

    /// Notification that the server is about to execute a command.
    ExecCommandBegin(ExecCommandBeginEvent),

    /// Incremental chunk of output from a running command.
    ExecCommandOutputDelta(ExecCommandOutputDeltaEvent),

    /// Terminal interaction for an in-progress command (stdin sent and stdout observed).
    TerminalInteraction(TerminalInteractionEvent),

    ExecCommandEnd(ExecCommandEndEvent),

    /// Notification that the agent attached a local image via the view_image tool.
    ViewImageToolCall(ViewImageToolCallEvent),

    ExecApprovalRequest(ExecApprovalRequestEvent),

    RequestPermissions(RequestPermissionsEvent),

    RequestUserInput(RequestUserInputEvent),

    DynamicToolCallRequest(DynamicToolCallRequest),

    DynamicToolCallResponse(DynamicToolCallResponseEvent),

    ElicitationRequest(ElicitationRequestEvent),

    ApplyPatchApprovalRequest(ApplyPatchApprovalRequestEvent),

    /// Structured lifecycle event for a guardian-reviewed approval request.
    GuardianAssessment(GuardianAssessmentEvent),

    /// Notification advising the user that something they are using has been
    /// deprecated and should be phased out.
    DeprecationNotice(DeprecationNoticeEvent),

    /// Notification that a model stream experienced an error or disconnect
    /// and the system is handling it (e.g., retrying with backoff).
    StreamError(StreamErrorEvent),

    /// Notification that the agent is about to apply a code patch. Mirrors
    /// `ExecCommandBegin` so front‑ends can show progress indicators.
    PatchApplyBegin(PatchApplyBeginEvent),

    /// Latest model-generated structured changes for an `apply_patch` call.
    PatchApplyUpdated(PatchApplyUpdatedEvent),

    /// Notification that a patch application has finished.
    PatchApplyEnd(PatchApplyEndEvent),

    TurnDiff(TurnDiffEvent),

    /// List of voices supported by realtime conversation streams.
    RealtimeConversationListVoicesResponse(RealtimeConversationListVoicesResponseEvent),

    PlanUpdate(UpdatePlanArgs),

    TurnAborted(TurnAbortedEvent),

    /// Notification that the agent is shutting down.
    ShutdownComplete,

    /// Entered review mode.
    EnteredReviewMode(ReviewRequest),

    /// Exited review mode with an optional final result to apply.
    ExitedReviewMode(ExitedReviewModeEvent),

    RawResponseItem(RawResponseItemEvent),

    ItemStarted(ItemStartedEvent),
    ItemCompleted(ItemCompletedEvent),
    HookStarted(HookStartedEvent),
    HookCompleted(HookCompletedEvent),

    AgentMessageContentDelta(AgentMessageContentDeltaEvent),
    PlanDelta(PlanDeltaEvent),
    ReasoningContentDelta(ReasoningContentDeltaEvent),
    ReasoningRawContentDelta(ReasoningRawContentDeltaEvent),

    /// Collab interaction: agent spawn begin.
    CollabAgentSpawnBegin(CollabAgentSpawnBeginEvent),
    /// Collab interaction: agent spawn end.
    CollabAgentSpawnEnd(CollabAgentSpawnEndEvent),
    /// Collab interaction: agent interaction begin.
    CollabAgentInteractionBegin(CollabAgentInteractionBeginEvent),
    /// Collab interaction: agent interaction end.
    CollabAgentInteractionEnd(CollabAgentInteractionEndEvent),
    /// Collab interaction: waiting begin.
    CollabWaitingBegin(CollabWaitingBeginEvent),
    /// Collab interaction: waiting end.
    CollabWaitingEnd(CollabWaitingEndEvent),
    /// Collab interaction: close begin.
    CollabCloseBegin(CollabCloseBeginEvent),
    /// Collab interaction: close end.
    CollabCloseEnd(CollabCloseEndEvent),
    /// Collab interaction: resume begin.
    CollabResumeBegin(CollabResumeBeginEvent),
    /// Collab interaction: resume end.
    CollabResumeEnd(CollabResumeEndEvent),
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum HookEventName {
    PreToolUse,
    PermissionRequest,
    PostToolUse,
    PreCompact,
    PostCompact,
    SessionStart,
    UserPromptSubmit,
    Stop,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum HookHandlerType {
    Command,
    Prompt,
    Agent,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum HookExecutionMode {
    Sync,
    Async,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum HookScope {
    Thread,
    Turn,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum HookSource {
    System,
    User,
    Project,
    Mdm,
    SessionFlags,
    Plugin,
    CloudRequirements,
    LegacyManagedConfigFile,
    LegacyManagedConfigMdm,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum HookTrustStatus {
    Managed,
    Untrusted,
    Trusted,
    Modified,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum HookRunStatus {
    Running,
    Completed,
    Failed,
    Blocked,
    Stopped,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum HookOutputEntryKind {
    Warning,
    Stop,
    Feedback,
    Context,
    Error,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub struct HookOutputEntry {
    pub kind: HookOutputEntryKind,
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub struct HookRunSummary {
    pub id: String,
    pub event_name: HookEventName,
    pub handler_type: HookHandlerType,
    pub execution_mode: HookExecutionMode,
    pub scope: HookScope,
    pub source_path: AbsolutePathBuf,
    #[serde(default)]
    pub source: HookSource,
    pub display_order: i64,
    pub status: HookRunStatus,
    pub status_message: Option<String>,
    #[ts(type = "number")]
    pub started_at: i64,
    #[ts(type = "number | null")]
    pub completed_at: Option<i64>,
    #[ts(type = "number | null")]
    pub duration_ms: Option<i64>,
    pub entries: Vec<HookOutputEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub struct HookStartedEvent {
    pub turn_id: Option<String>,
    pub run: HookRunSummary,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub struct HookCompletedEvent {
    pub turn_id: Option<String>,
    pub run: HookRunSummary,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeConversationVersion {
    V1,
    #[default]
    V2,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct RealtimeConversationStartedEvent {
    pub realtime_session_id: Option<String>,
    pub version: RealtimeConversationVersion,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct RealtimeConversationRealtimeEvent {
    pub payload: RealtimeEvent,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct RealtimeConversationClosedEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct RealtimeConversationSdpEvent {
    pub sdp: String,
}

impl From<CollabAgentSpawnBeginEvent> for EventMsg {
    fn from(event: CollabAgentSpawnBeginEvent) -> Self {
        EventMsg::CollabAgentSpawnBegin(event)
    }
}

impl From<CollabAgentSpawnEndEvent> for EventMsg {
    fn from(event: CollabAgentSpawnEndEvent) -> Self {
        EventMsg::CollabAgentSpawnEnd(event)
    }
}

impl From<CollabAgentInteractionBeginEvent> for EventMsg {
    fn from(event: CollabAgentInteractionBeginEvent) -> Self {
        EventMsg::CollabAgentInteractionBegin(event)
    }
}

impl From<CollabAgentInteractionEndEvent> for EventMsg {
    fn from(event: CollabAgentInteractionEndEvent) -> Self {
        EventMsg::CollabAgentInteractionEnd(event)
    }
}

impl From<CollabWaitingBeginEvent> for EventMsg {
    fn from(event: CollabWaitingBeginEvent) -> Self {
        EventMsg::CollabWaitingBegin(event)
    }
}

impl From<CollabWaitingEndEvent> for EventMsg {
    fn from(event: CollabWaitingEndEvent) -> Self {
        EventMsg::CollabWaitingEnd(event)
    }
}

impl From<CollabCloseBeginEvent> for EventMsg {
    fn from(event: CollabCloseBeginEvent) -> Self {
        EventMsg::CollabCloseBegin(event)
    }
}

impl From<CollabCloseEndEvent> for EventMsg {
    fn from(event: CollabCloseEndEvent) -> Self {
        EventMsg::CollabCloseEnd(event)
    }
}

impl From<CollabResumeBeginEvent> for EventMsg {
    fn from(event: CollabResumeBeginEvent) -> Self {
        EventMsg::CollabResumeBegin(event)
    }
}

impl From<CollabResumeEndEvent> for EventMsg {
    fn from(event: CollabResumeEndEvent) -> Self {
        EventMsg::CollabResumeEnd(event)
    }
}

/// Agent lifecycle status, derived from emitted events.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS, Default)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum AgentStatus {
    /// Agent is waiting for initialization.
    #[default]
    PendingInit,
    /// Agent is currently running.
    Running,
    /// Agent's current turn was interrupted and it may receive more input.
    Interrupted,
    /// Agent is done. Contains the final assistant message.
    Completed(Option<String>),
    /// Agent encountered an error.
    Errored(String),
    /// Agent has been shutdown.
    Shutdown,
    /// Agent is not found.
    NotFound,
}

/// Turn kinds that reject same-turn steering.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum NonSteerableTurnKind {
    Review,
    Compact,
}

/// Hepta errors that we expose to clients.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum CodexErrorInfo {
    ContextWindowExceeded,
    UsageLimitExceeded,
    ServerOverloaded,
    CyberPolicy,
    HttpConnectionFailed {
        http_status_code: Option<u16>,
    },
    /// Failed to connect to the response SSE stream.
    ResponseStreamConnectionFailed {
        http_status_code: Option<u16>,
    },
    InternalServerError,
    Unauthorized,
    BadRequest,
    SandboxError,
    /// The response SSE stream disconnected in the middle of a turnbefore completion.
    ResponseStreamDisconnected {
        http_status_code: Option<u16>,
    },
    /// Reached the retry limit for responses.
    ResponseTooManyFailedAttempts {
        http_status_code: Option<u16>,
    },
    /// Returned when `turn/start` or `turn/steer` is submitted while the current active turn
    /// cannot accept same-turn steering, for example `/review` or manual `/compact`.
    ActiveTurnNotSteerable {
        turn_kind: NonSteerableTurnKind,
    },
    ThreadRollbackFailed,
    Other,
}

impl CodexErrorInfo {
    /// Whether this error should mark the current turn as failed when replaying history.
    pub fn affects_turn_status(&self) -> bool {
        match self {
            Self::ThreadRollbackFailed | Self::ActiveTurnNotSteerable { .. } => false,
            Self::ContextWindowExceeded
            | Self::UsageLimitExceeded
            | Self::ServerOverloaded
            | Self::CyberPolicy
            | Self::HttpConnectionFailed { .. }
            | Self::ResponseStreamConnectionFailed { .. }
            | Self::InternalServerError
            | Self::Unauthorized
            | Self::BadRequest
            | Self::SandboxError
            | Self::ResponseStreamDisconnected { .. }
            | Self::ResponseTooManyFailedAttempts { .. }
            | Self::Other => true,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, JsonSchema)]
pub struct RawResponseItemEvent {
    pub item: ResponseItem,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, JsonSchema)]
pub struct ItemStartedEvent {
    pub thread_id: ThreadId,
    pub turn_id: String,
    pub item: TurnItem,
    pub started_at_ms: i64,
}

impl HasLegacyEvent for ItemStartedEvent {
    fn as_legacy_events(&self, _: bool) -> Vec<EventMsg> {
        match &self.item {
            TurnItem::WebSearch(item) => vec![EventMsg::WebSearchBegin(WebSearchBeginEvent {
                call_id: item.id.clone(),
            })],
            TurnItem::ImageView(_) => Vec::new(),
            TurnItem::ImageGeneration(item) => {
                vec![EventMsg::ImageGenerationBegin(ImageGenerationBeginEvent {
                    call_id: item.id.clone(),
                })]
            }
            TurnItem::FileChange(item) => vec![item.as_legacy_begin_event(self.turn_id.clone())],
            TurnItem::McpToolCall(item) => vec![item.as_legacy_begin_event()],
            _ => Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, JsonSchema)]
pub struct ItemCompletedEvent {
    pub thread_id: ThreadId,
    pub turn_id: String,
    pub item: TurnItem,
    // Old rollout files may contain ItemCompleted events for PlanItem without
    // this field. Default to 0 so those persisted rollouts still deserialize
    // after tightening the core event contract.
    #[serde(default = "default_item_completed_at_ms")]
    pub completed_at_ms: i64,
}

const fn default_item_completed_at_ms() -> i64 {
    0
}

pub trait HasLegacyEvent {
    fn as_legacy_events(&self, show_raw_agent_reasoning: bool) -> Vec<EventMsg>;
}

impl HasLegacyEvent for ItemCompletedEvent {
    fn as_legacy_events(&self, show_raw_agent_reasoning: bool) -> Vec<EventMsg> {
        match &self.item {
            TurnItem::FileChange(item) => item
                .as_legacy_end_event(self.turn_id.clone())
                .into_iter()
                .collect(),
            _ => self.item.as_legacy_events(show_raw_agent_reasoning),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, JsonSchema)]
pub struct AgentMessageContentDeltaEvent {
    pub thread_id: String,
    pub turn_id: String,
    pub item_id: String,
    pub delta: String,
}

impl HasLegacyEvent for AgentMessageContentDeltaEvent {
    fn as_legacy_events(&self, _: bool) -> Vec<EventMsg> {
        Vec::new()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, JsonSchema)]
pub struct PlanDeltaEvent {
    pub thread_id: String,
    pub turn_id: String,
    pub item_id: String,
    pub delta: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, JsonSchema)]
pub struct ReasoningContentDeltaEvent {
    pub thread_id: String,
    pub turn_id: String,
    pub item_id: String,
    pub delta: String,
    // load with default value so it's backward compatible with the old format.
    #[serde(default)]
    pub summary_index: i64,
}

impl HasLegacyEvent for ReasoningContentDeltaEvent {
    fn as_legacy_events(&self, _: bool) -> Vec<EventMsg> {
        Vec::new()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, JsonSchema)]
pub struct ReasoningRawContentDeltaEvent {
    pub thread_id: String,
    pub turn_id: String,
    pub item_id: String,
    pub delta: String,
    // load with default value so it's backward compatible with the old format.
    #[serde(default)]
    pub content_index: i64,
}

impl HasLegacyEvent for ReasoningRawContentDeltaEvent {
    fn as_legacy_events(&self, _: bool) -> Vec<EventMsg> {
        Vec::new()
    }
}

impl HasLegacyEvent for EventMsg {
    fn as_legacy_events(&self, show_raw_agent_reasoning: bool) -> Vec<EventMsg> {
        match self {
            EventMsg::ItemStarted(event) => event.as_legacy_events(show_raw_agent_reasoning),
            EventMsg::ItemCompleted(event) => event.as_legacy_events(show_raw_agent_reasoning),
            EventMsg::AgentMessageContentDelta(event) => {
                event.as_legacy_events(show_raw_agent_reasoning)
            }
            EventMsg::ReasoningContentDelta(event) => {
                event.as_legacy_events(show_raw_agent_reasoning)
            }
            EventMsg::ReasoningRawContentDelta(event) => {
                event.as_legacy_events(show_raw_agent_reasoning)
            }
            _ => Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct ExitedReviewModeEvent {
    pub review_output: Option<ReviewOutputEvent>,
}

// Individual event payload types matching each `EventMsg` variant.

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct ErrorEvent {
    pub message: String,
    #[serde(default)]
    pub codex_error_info: Option<CodexErrorInfo>,
}

impl ErrorEvent {
    /// Whether this error should mark the current turn as failed when replaying history.
    pub fn affects_turn_status(&self) -> bool {
        self.codex_error_info
            .as_ref()
            .is_none_or(CodexErrorInfo::affects_turn_status)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct WarningEvent {
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum ModelRerouteReason {
    HighRiskCyberActivity,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct ModelRerouteEvent {
    pub from_model: String,
    pub to_model: String,
    pub reason: ModelRerouteReason,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum ModelVerification {
    TrustedAccessForCyber,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct ModelVerificationEvent {
    pub verifications: Vec<ModelVerification>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct ContextCompactedEvent;

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct TurnCompleteEvent {
    pub turn_id: String,
    pub last_agent_message: Option<String>,
    /// Unix timestamp (in seconds) when the turn completed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(type = "number | null", optional)]
    pub completed_at: Option<i64>,
    /// Duration between turn start and completion in milliseconds, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(type = "number | null", optional)]
    pub duration_ms: Option<i64>,
    /// Duration between turn start and the first model token in milliseconds, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(type = "number | null", optional)]
    pub time_to_first_token_ms: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct TurnStartedEvent {
    pub turn_id: String,
    /// Unix timestamp (in seconds) when the turn started.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(type = "number | null", optional)]
    pub started_at: Option<i64>,
    // TODO(aibrahim): make this not optional
    pub model_context_window: Option<i64>,
    #[serde(default)]
    pub collaboration_mode_kind: ModeKind,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, JsonSchema, TS)]
pub struct TokenUsage {
    #[ts(type = "number")]
    pub input_tokens: i64,
    #[ts(type = "number")]
    pub cached_input_tokens: i64,
    #[ts(type = "number")]
    pub output_tokens: i64,
    #[ts(type = "number")]
    pub reasoning_output_tokens: i64,
    #[ts(type = "number")]
    pub total_tokens: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct TokenUsageInfo {
    pub total_token_usage: TokenUsage,
    pub last_token_usage: TokenUsage,
    // TODO(aibrahim): make this not optional
    #[ts(type = "number | null")]
    pub model_context_window: Option<i64>,
}

impl TokenUsageInfo {
    pub fn new_or_append(
        info: &Option<TokenUsageInfo>,
        last: &Option<TokenUsage>,
        model_context_window: Option<i64>,
    ) -> Option<Self> {
        if info.is_none() && last.is_none() {
            return None;
        }

        let mut info = match info {
            Some(info) => info.clone(),
            None => Self {
                total_token_usage: TokenUsage::default(),
                last_token_usage: TokenUsage::default(),
                model_context_window,
            },
        };
        if let Some(last) = last {
            info.append_last_usage(last);
        }
        if let Some(model_context_window) = model_context_window {
            info.model_context_window = Some(model_context_window);
        }
        Some(info)
    }

    pub fn append_last_usage(&mut self, last: &TokenUsage) {
        self.total_token_usage.add_assign(last);
        self.last_token_usage = last.clone();
    }

    pub fn fill_to_context_window(&mut self, context_window: i64) {
        let previous_total = self.total_token_usage.total_tokens;
        let delta = (context_window - previous_total).max(0);

        self.model_context_window = Some(context_window);
        self.total_token_usage = TokenUsage {
            total_tokens: context_window,
            ..TokenUsage::default()
        };
        self.last_token_usage = TokenUsage {
            total_tokens: delta,
            ..TokenUsage::default()
        };
    }

    pub fn full_context_window(context_window: i64) -> Self {
        let mut info = Self {
            total_token_usage: TokenUsage::default(),
            last_token_usage: TokenUsage::default(),
            model_context_window: Some(context_window),
        };
        info.fill_to_context_window(context_window);
        info
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct TokenCountEvent {
    pub info: Option<TokenUsageInfo>,
    pub rate_limits: Option<RateLimitSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema, TS)]
pub struct RateLimitSnapshot {
    pub limit_id: Option<String>,
    pub limit_name: Option<String>,
    pub primary: Option<RateLimitWindow>,
    pub secondary: Option<RateLimitWindow>,
    pub credits: Option<CreditsSnapshot>,
    pub plan_type: Option<crate::account::PlanType>,
    pub rate_limit_reached_type: Option<RateLimitReachedType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum RateLimitReachedType {
    RateLimitReached,
    WorkspaceOwnerCreditsDepleted,
    WorkspaceMemberCreditsDepleted,
    WorkspaceOwnerUsageLimitReached,
    WorkspaceMemberUsageLimitReached,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema, TS)]
pub struct RateLimitWindow {
    /// Percentage (0-100) of the window that has been consumed.
    pub used_percent: f64,
    /// Rolling window duration, in minutes.
    #[ts(type = "number | null")]
    pub window_minutes: Option<i64>,
    /// Unix timestamp (seconds since epoch) when the window resets.
    #[ts(type = "number | null")]
    pub resets_at: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema, TS)]
pub struct CreditsSnapshot {
    pub has_credits: bool,
    pub unlimited: bool,
    pub balance: Option<String>,
}

// Includes prompts, tools and space to call compact.
const BASELINE_TOKENS: i64 = 12000;

impl TokenUsage {
    pub fn is_zero(&self) -> bool {
        self.total_tokens == 0
    }

    pub fn cached_input(&self) -> i64 {
        self.cached_input_tokens.max(0)
    }

    pub fn non_cached_input(&self) -> i64 {
        (self.input_tokens - self.cached_input()).max(0)
    }

    /// Primary count for display as a single absolute value: non-cached input + output.
    pub fn blended_total(&self) -> i64 {
        (self.non_cached_input() + self.output_tokens.max(0)).max(0)
    }

    pub fn tokens_in_context_window(&self) -> i64 {
        self.total_tokens
    }

    /// Estimate the remaining user-controllable percentage of the model's context window.
    ///
    /// `context_window` is the total size of the model's context window.
    /// `BASELINE_TOKENS` should capture tokens that are always present in
    /// the context (e.g., system prompt and fixed tool instructions) so that
    /// the percentage reflects the portion the user can influence.
    ///
    /// This normalizes both the numerator and denominator by subtracting the
    /// baseline, so immediately after the first prompt the UI shows 100% left
    /// and trends toward 0% as the user fills the effective window.
    pub fn percent_of_context_window_remaining(&self, context_window: i64) -> i64 {
        if context_window <= BASELINE_TOKENS {
            return 0;
        }

        let effective_window = context_window - BASELINE_TOKENS;
        let used = (self.tokens_in_context_window() - BASELINE_TOKENS).max(0);
        let remaining = (effective_window - used).max(0);
        ((remaining as f64 / effective_window as f64) * 100.0)
            .clamp(0.0, 100.0)
            .round() as i64
    }

    /// In-place element-wise sum of token counts.
    pub fn add_assign(&mut self, other: &TokenUsage) {
        self.input_tokens += other.input_tokens;
        self.cached_input_tokens += other.cached_input_tokens;
        self.output_tokens += other.output_tokens;
        self.reasoning_output_tokens += other.reasoning_output_tokens;
        self.total_tokens += other.total_tokens;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct FinalOutput {
    pub token_usage: TokenUsage,
}

impl From<TokenUsage> for FinalOutput {
    fn from(token_usage: TokenUsage) -> Self {
        Self { token_usage }
    }
}

impl fmt::Display for FinalOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_usage = &self.token_usage;

        write!(
            f,
            "Token usage: total={} input={}{} output={}{}",
            format_with_separators(token_usage.blended_total()),
            format_with_separators(token_usage.non_cached_input()),
            if token_usage.cached_input() > 0 {
                format!(
                    " (+ {} cached)",
                    format_with_separators(token_usage.cached_input())
                )
            } else {
                String::new()
            },
            format_with_separators(token_usage.output_tokens),
            if token_usage.reasoning_output_tokens > 0 {
                format!(
                    " (reasoning {})",
                    format_with_separators(token_usage.reasoning_output_tokens)
                )
            } else {
                String::new()
            }
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct AgentMessageEvent {
    pub message: String,
    #[serde(default)]
    pub phase: Option<MessagePhase>,
    #[serde(default)]
    pub memory_citation: Option<MemoryCitation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, JsonSchema, TS)]
pub struct UserMessageEvent {
    pub message: String,
    /// Image URLs sourced from `UserInput::Image`. These are safe
    /// to replay in legacy UI history events and correspond to images sent to
    /// the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    /// Detail hints for `images`, indexed in parallel. Missing entries imply
    /// default image detail behavior.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_details: Vec<Option<ImageDetail>>,
    /// Local file paths sourced from `UserInput::LocalImage`. These are kept so
    /// the UI can reattach images when editing history, and should not be sent
    /// to the model or treated as API-ready URLs.
    #[serde(default)]
    pub local_images: Vec<std::path::PathBuf>,
    /// Detail hints for `local_images`, indexed in parallel. Missing entries
    /// imply default image detail behavior.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub local_image_details: Vec<Option<ImageDetail>>,
    /// UI-defined spans within `message` used to render or persist special elements.
    #[serde(default)]
    pub text_elements: Vec<crate::user_input::TextElement>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct AgentReasoningEvent {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct AgentReasoningRawContentEvent {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct AgentReasoningSectionBreakEvent {
    // load with default value so it's backward compatible with the old format.
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub summary_index: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS, PartialEq)]
pub struct McpInvocation {
    /// Name of the MCP server as defined in the config.
    pub server: String,
    /// Name of the tool as given by the MCP server.
    pub tool: String,
    /// Arguments to the tool call.
    pub arguments: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS, PartialEq)]
pub struct McpToolCallBeginEvent {
    /// Identifier so this can be paired with the McpToolCallEnd event.
    pub call_id: String,
    pub invocation: McpInvocation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub mcp_app_resource_uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS, PartialEq)]
pub struct McpToolCallEndEvent {
    /// Identifier for the corresponding McpToolCallBegin that finished.
    pub call_id: String,
    pub invocation: McpInvocation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub mcp_app_resource_uri: Option<String>,
    #[ts(type = "string")]
    pub duration: Duration,
    /// Result of the tool call. Note this could be an error.
    pub result: Result<CallToolResult, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS, PartialEq)]
pub struct DynamicToolCallResponseEvent {
    /// Identifier for the corresponding DynamicToolCallRequest.
    pub call_id: String,
    /// Turn ID that this dynamic tool call belongs to.
    pub turn_id: String,
    #[serde(default)]
    pub completed_at_ms: i64,
    /// Dynamic tool namespace, when one was provided.
    #[serde(default)]
    pub namespace: Option<String>,
    /// Dynamic tool name.
    pub tool: String,
    /// Dynamic tool call arguments.
    pub arguments: serde_json::Value,
    /// Dynamic tool response content items.
    pub content_items: Vec<DynamicToolCallOutputContentItem>,
    /// Whether the tool call succeeded.
    pub success: bool,
    /// Optional error text when the tool call failed before producing a response.
    pub error: Option<String>,
    /// The duration of the dynamic tool call.
    #[ts(type = "string")]
    pub duration: Duration,
}

impl McpToolCallEndEvent {
    pub fn is_success(&self) -> bool {
        match &self.result {
            Ok(result) => !result.is_error.unwrap_or(false),
            Err(_) => false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct WebSearchBeginEvent {
    pub call_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct WebSearchEndEvent {
    pub call_id: String,
    pub query: String,
    pub action: WebSearchAction,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct ImageGenerationBeginEvent {
    pub call_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct ImageGenerationEndEvent {
    pub call_id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub revised_prompt: Option<String>,
    pub result: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub saved_path: Option<AbsolutePathBuf>,
}

// Conversation kept for backward compatibility.
/// Response payload for `Op::GetHistory` containing the current session's
/// in-memory transcript.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct ConversationPathResponseEvent {
    pub conversation_id: ThreadId,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct ResumedHistory {
    pub conversation_id: ThreadId,
    pub history: Vec<RolloutItem>,
    pub rollout_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub enum InitialHistory {
    New,
    Cleared,
    Resumed(ResumedHistory),
    Forked(Vec<RolloutItem>),
}

impl InitialHistory {
    pub fn scan_rollout_items(&self, mut predicate: impl FnMut(&RolloutItem) -> bool) -> bool {
        match self {
            InitialHistory::New | InitialHistory::Cleared => false,
            InitialHistory::Resumed(resumed) => resumed.history.iter().any(&mut predicate),
            InitialHistory::Forked(items) => items.iter().any(predicate),
        }
    }

    pub fn forked_from_id(&self) -> Option<ThreadId> {
        match self {
            InitialHistory::New | InitialHistory::Cleared => None,
            InitialHistory::Resumed(resumed) => {
                resumed.history.iter().find_map(|item| match item {
                    RolloutItem::SessionMeta(meta_line) => meta_line.meta.forked_from_id,
                    _ => None,
                })
            }
            InitialHistory::Forked(items) => items.iter().find_map(|item| match item {
                RolloutItem::SessionMeta(meta_line) => Some(meta_line.meta.id),
                _ => None,
            }),
        }
    }

    pub fn session_cwd(&self) -> Option<PathBuf> {
        match self {
            InitialHistory::New | InitialHistory::Cleared => None,
            InitialHistory::Resumed(resumed) => session_cwd_from_items(&resumed.history),
            InitialHistory::Forked(items) => session_cwd_from_items(items),
        }
    }

    pub fn get_rollout_items(&self) -> Vec<RolloutItem> {
        match self {
            InitialHistory::New | InitialHistory::Cleared => Vec::new(),
            InitialHistory::Resumed(resumed) => resumed.history.clone(),
            InitialHistory::Forked(items) => items.clone(),
        }
    }

    pub fn get_event_msgs(&self) -> Option<Vec<EventMsg>> {
        match self {
            InitialHistory::New | InitialHistory::Cleared => None,
            InitialHistory::Resumed(resumed) => Some(
                resumed
                    .history
                    .iter()
                    .filter_map(|ri| match ri {
                        RolloutItem::EventMsg(ev) => Some(ev.clone()),
                        _ => None,
                    })
                    .collect(),
            ),
            InitialHistory::Forked(items) => Some(
                items
                    .iter()
                    .filter_map(|ri| match ri {
                        RolloutItem::EventMsg(ev) => Some(ev.clone()),
                        _ => None,
                    })
                    .collect(),
            ),
        }
    }

    pub fn get_base_instructions(&self) -> Option<BaseInstructions> {
        // TODO: SessionMeta should (in theory) always be first in the history, so we can probably only check the first item?
        match self {
            InitialHistory::New | InitialHistory::Cleared => None,
            InitialHistory::Resumed(resumed) => {
                resumed.history.iter().find_map(|item| match item {
                    RolloutItem::SessionMeta(meta_line) => meta_line.meta.base_instructions.clone(),
                    _ => None,
                })
            }
            InitialHistory::Forked(items) => items.iter().find_map(|item| match item {
                RolloutItem::SessionMeta(meta_line) => meta_line.meta.base_instructions.clone(),
                _ => None,
            }),
        }
    }

    pub fn get_dynamic_tools(&self) -> Option<Vec<DynamicToolSpec>> {
        match self {
            InitialHistory::New | InitialHistory::Cleared => None,
            InitialHistory::Resumed(resumed) => {
                resumed.history.iter().find_map(|item| match item {
                    RolloutItem::SessionMeta(meta_line) => meta_line.meta.dynamic_tools.clone(),
                    _ => None,
                })
            }
            InitialHistory::Forked(items) => items.iter().find_map(|item| match item {
                RolloutItem::SessionMeta(meta_line) => meta_line.meta.dynamic_tools.clone(),
                _ => None,
            }),
        }
    }

    pub fn get_resumed_thread_source(&self) -> Option<ThreadSource> {
        match self {
            InitialHistory::New | InitialHistory::Cleared | InitialHistory::Forked(_) => None,
            InitialHistory::Resumed(resumed) => {
                resumed.history.iter().find_map(|item| match item {
                    RolloutItem::SessionMeta(meta_line) => meta_line.meta.thread_source,
                    _ => None,
                })
            }
        }
    }
}

fn session_cwd_from_items(items: &[RolloutItem]) -> Option<PathBuf> {
    items.iter().find_map(|item| match item {
        RolloutItem::SessionMeta(meta_line) => Some(meta_line.meta.cwd.clone()),
        _ => None,
    })
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS, Default)]
#[serde(rename_all = "lowercase")]
#[ts(rename_all = "lowercase")]
pub enum SessionSource {
    Cli,
    #[default]
    VSCode,
    Exec,
    Mcp,
    Custom(String),
    Internal(InternalSessionSource),
    SubAgent(SubAgentSource),
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum ThreadSource {
    User,
    Subagent,
    MemoryConsolidation,
}

impl ThreadSource {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreadSource::User => "user",
            ThreadSource::Subagent => "subagent",
            ThreadSource::MemoryConsolidation => "memory_consolidation",
        }
    }
}

impl fmt::Display for ThreadSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ThreadSource {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "user" => Ok(ThreadSource::User),
            "subagent" => Ok(ThreadSource::Subagent),
            "memory_consolidation" => Ok(ThreadSource::MemoryConsolidation),
            other => Err(format!("unknown thread source: {other}")),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum InternalSessionSource {
    MemoryConsolidation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum SubAgentSource {
    Review,
    Compact,
    ThreadSpawn {
        parent_thread_id: ThreadId,
        depth: i32,
        #[serde(default)]
        agent_path: Option<AgentPath>,
        #[serde(default)]
        agent_nickname: Option<String>,
        #[serde(default, alias = "agent_type")]
        agent_role: Option<String>,
    },
    MemoryConsolidation,
    Other(String),
}

impl fmt::Display for SessionSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionSource::Cli => f.write_str("cli"),
            SessionSource::VSCode => f.write_str("vscode"),
            SessionSource::Exec => f.write_str("exec"),
            SessionSource::Mcp => f.write_str("mcp"),
            SessionSource::Custom(source) => f.write_str(source),
            SessionSource::Internal(source) => write!(f, "internal_{source}"),
            SessionSource::SubAgent(sub_source) => write!(f, "subagent_{sub_source}"),
            SessionSource::Unknown => f.write_str("unknown"),
        }
    }
}

impl SessionSource {
    pub fn from_startup_arg(value: &str) -> Result<Self, &'static str> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err("session source must not be empty");
        }

        let normalized = trimmed.to_ascii_lowercase();
        Ok(match normalized.as_str() {
            "cli" => SessionSource::Cli,
            "vscode" => SessionSource::VSCode,
            "exec" => SessionSource::Exec,
            "mcp" | "appserver" | "app-server" | "app_server" => SessionSource::Mcp,
            "unknown" => SessionSource::Unknown,
            _ => SessionSource::Custom(normalized),
        })
    }

    pub fn is_internal(&self) -> bool {
        matches!(self, SessionSource::Internal(_))
    }

    pub fn is_non_root_agent(&self) -> bool {
        matches!(
            self,
            SessionSource::Internal(_) | SessionSource::SubAgent(_)
        )
    }

    pub fn get_nickname(&self) -> Option<String> {
        match self {
            SessionSource::SubAgent(SubAgentSource::ThreadSpawn { agent_nickname, .. }) => {
                agent_nickname.clone()
            }
            _ => None,
        }
    }

    pub fn get_agent_role(&self) -> Option<String> {
        match self {
            SessionSource::SubAgent(SubAgentSource::ThreadSpawn { agent_role, .. }) => {
                agent_role.clone()
            }
            _ => None,
        }
    }

    pub fn get_agent_path(&self) -> Option<AgentPath> {
        match self {
            SessionSource::SubAgent(SubAgentSource::ThreadSpawn { agent_path, .. }) => {
                agent_path.clone()
            }
            _ => None,
        }
    }

    pub fn restriction_product(&self) -> Option<Product> {
        match self {
            SessionSource::Custom(source) => Product::from_session_source_name(source),
            SessionSource::Cli
            | SessionSource::VSCode
            | SessionSource::Exec
            | SessionSource::Mcp
            | SessionSource::Unknown => Some(Product::Hepta),
            SessionSource::Internal(_) | SessionSource::SubAgent(_) => None,
        }
    }

    pub fn matches_product_restriction(&self, products: &[Product]) -> bool {
        products.is_empty()
            || self
                .restriction_product()
                .is_some_and(|product| product.matches_product_restriction(products))
    }
}

impl fmt::Display for SubAgentSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubAgentSource::Review => f.write_str("review"),
            SubAgentSource::Compact => f.write_str("compact"),
            SubAgentSource::MemoryConsolidation => f.write_str("memory_consolidation"),
            SubAgentSource::ThreadSpawn {
                parent_thread_id,
                depth,
                ..
            } => {
                write!(f, "thread_spawn_{parent_thread_id}_d{depth}")
            }
            SubAgentSource::Other(other) => f.write_str(other),
        }
    }
}

impl fmt::Display for InternalSessionSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalSessionSource::MemoryConsolidation => f.write_str("memory_consolidation"),
        }
    }
}

/// SessionMeta contains session-level data that doesn't correspond to a specific turn.
///
/// NOTE: There used to be an `instructions` field here, which stored user_instructions, but we
/// now save that on TurnContext. base_instructions stores the base instructions for the session,
/// and should be used when there is no config override.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, TS)]
pub struct SessionMeta {
    pub id: ThreadId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forked_from_id: Option<ThreadId>,
    pub timestamp: String,
    pub cwd: PathBuf,
    pub originator: String,
    pub cli_version: String,
    #[serde(default)]
    pub source: SessionSource,
    /// Optional analytics source classification for this thread.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_source: Option<ThreadSource>,
    /// Optional random unique nickname assigned to an AgentControl-spawned sub-agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_nickname: Option<String>,
    /// Optional role (agent_role) assigned to an AgentControl-spawned sub-agent.
    #[serde(default, alias = "agent_type", skip_serializing_if = "Option::is_none")]
    pub agent_role: Option<String>,
    /// Optional canonical agent path assigned to an AgentControl-spawned sub-agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_path: Option<String>,
    pub model_provider: Option<String>,
    /// base_instructions for the session. This *should* always be present when creating a new session,
    /// but may be missing for older sessions. If not present, fall back to rendering the base_instructions
    /// from ModelsManager.
    pub base_instructions: Option<BaseInstructions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_tools: Option<Vec<DynamicToolSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_mode: Option<String>,
}

impl Default for SessionMeta {
    fn default() -> Self {
        SessionMeta {
            id: ThreadId::default(),
            forked_from_id: None,
            timestamp: String::new(),
            cwd: PathBuf::new(),
            originator: String::new(),
            cli_version: String::new(),
            source: SessionSource::default(),
            thread_source: None,
            agent_nickname: None,
            agent_role: None,
            agent_path: None,
            model_provider: None,
            base_instructions: None,
            dynamic_tools: None,
            memory_mode: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
pub struct SessionMetaLine {
    #[serde(flatten)]
    pub meta: SessionMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git: Option<GitInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum RolloutItem {
    SessionMeta(SessionMetaLine),
    ResponseItem(ResponseItem),
    Compacted(CompactedItem),
    TurnContext(TurnContextItem),
    EventMsg(EventMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, TS)]
pub struct CompactedItem {
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_history: Option<Vec<ResponseItem>>,
}

impl From<CompactedItem> for ResponseItem {
    fn from(value: CompactedItem) -> Self {
        ResponseItem::Message {
            id: None,
            role: "assistant".to_string(),
            content: vec![ContentItem::OutputText {
                text: value.message,
            }],
            phase: None,
        }
    }
}

pub const TURN_CONTEXT_MANIFEST_VERSION: u32 = 1;
pub const TURN_CONTEXT_DECISION_SCHEMA_VERSION: u32 = 1;
pub const TURN_CONTEXT_COMPRESSION_CANDIDATE_SCHEMA_VERSION: u32 = 1;
pub const TURN_CONTEXT_COMPRESSION_STAGE_SCHEMA_VERSION: u32 = 2;
pub const TURN_CONTEXT_ADAPTIVE_BUDGET_ALLOCATION_SCHEMA_VERSION: u32 = 1;
pub const TURN_CONTEXT_MEMORY_TAXONOMY_SCHEMA_VERSION: u32 = 1;
pub const TURN_CONTEXT_MEMORY_FORMATION_RECEIPT_SCHEMA_VERSION: u32 = 1;
pub const TURN_CONTEXT_MEMORY_TEMPORAL_FACT_SCHEMA_VERSION: u32 = 1;

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextTier {
    System,
    Developer,
    User,
    Tool,
    Runtime,
    SessionState,
    CrossSessionMemory,
    RetrievedSnippets,
    Summary,
    #[default]
    Unknown,
}

impl TurnContextTier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Developer => "developer",
            Self::User => "user",
            Self::Tool => "tool",
            Self::Runtime => "runtime",
            Self::SessionState => "session_state",
            Self::CrossSessionMemory => "cross_session_memory",
            Self::RetrievedSnippets => "retrieved_snippets",
            Self::Summary => "summary",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextManifestEntry {
    pub role: String,
    #[serde(default, skip_serializing_if = "TurnContextTier::is_unknown")]
    pub tier: TurnContextTier,
    pub source: String,
    pub replay_key: String,
    /// Stable 16-hex replay identity for the entry text. This is not a
    /// cryptographic trust digest and must not be used for approval integrity.
    pub text_hash: String,
    pub estimated_tokens: u32,
}

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextRecallSelectionSummary {
    pub returned_source_count: u32,
    pub selected_source_count: u32,
    pub ranked_source_count: u32,
    pub returned_unselected_source_count: u32,
    pub source_diversity_met: bool,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub source_diversity_target: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub max_per_source: u32,
    pub ranked_item_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub omitted_by_budget_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub memory_control_omitted_count: u32,
    pub low_trust_ranked_item_count: u32,
    pub low_recency_ranked_item_count: u32,
}

impl TurnContextRecallSelectionSummary {
    pub fn returned_unselected_source_count_matches(&self) -> bool {
        self.returned_unselected_source_count
            == self
                .returned_source_count
                .saturating_sub(self.selected_source_count)
    }

    pub fn source_diversity_target_matches(&self) -> bool {
        self.source_diversity_target == 0
            || self.source_diversity_met
                == (self.selected_source_count >= self.source_diversity_target)
    }

    pub fn has_count_integrity(&self) -> bool {
        self.selected_source_count <= self.returned_source_count
            && self.ranked_source_count <= self.selected_source_count
            && self.ranked_source_count <= self.ranked_item_count
            && (self.ranked_item_count == 0 || self.ranked_source_count > 0)
            && self.returned_unselected_source_count_matches()
            && self.source_diversity_target_matches()
            && self.low_trust_ranked_item_count <= self.ranked_item_count
            && self.low_recency_ranked_item_count <= self.ranked_item_count
    }
}

pub const TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION: u32 = 1;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextRecallSelectedSnippetEnvelope {
    pub version: u32,
    pub max_snippets: u32,
    pub max_snippet_chars: u32,
    pub selected_snippet_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub omitted_snippet_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub redacted_snippet_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub truncated_snippet_count: u32,
    pub snippets: Vec<TurnContextRecallSelectedSnippet>,
    pub safety: TurnContextRecallSelectedSnippetSafety,
}

impl TurnContextRecallSelectedSnippetEnvelope {
    pub fn counts_match(&self) -> bool {
        self.selected_snippet_count == u32::try_from(self.snippets.len()).unwrap_or(u32::MAX)
            && self.redacted_snippet_count
                == u32::try_from(
                    self.snippets
                        .iter()
                        .filter(|snippet| snippet.redacted)
                        .count(),
                )
                .unwrap_or(u32::MAX)
            && self.truncated_snippet_count
                == u32::try_from(
                    self.snippets
                        .iter()
                        .filter(|snippet| snippet.truncated)
                        .count(),
                )
                .unwrap_or(u32::MAX)
    }

    pub fn bounds_match(&self) -> bool {
        self.selected_snippet_count <= self.max_snippets
            && self.snippets.len() <= usize::try_from(self.max_snippets).unwrap_or(usize::MAX)
            && self.snippets.iter().all(|snippet| {
                !snippet.text.is_empty()
                    && snippet.text.chars().count()
                        <= usize::try_from(self.max_snippet_chars).unwrap_or(usize::MAX)
                    && is_stable_manifest_replay_hash(&snippet.snippet_hash)
            })
    }

    pub fn safety_matches(&self) -> bool {
        let forbidden_exposure = self.safety.origin_identifiers_exposed
            || self.safety.raw_ranked_payload_exposed
            || self.safety.rank_explanation_exposed
            || self.safety.control_marker_exposed
            || self.safety.query_payload_exposed
            || self.safety.per_origin_list_exposed
            || self
                .snippets
                .iter()
                .any(|snippet| snippet.text.contains("[hepta-memory:"));
        self.safety.bounded == self.bounds_match()
            && self.safety.ready_for_shadow_handoff == (self.safety.bounded && !forbidden_exposure)
            && self.safety.ready_for_shadow_handoff
    }

    pub fn has_shadow_integrity(&self) -> bool {
        self.version == TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION
            && self.counts_match()
            && self.bounds_match()
            && self.safety_matches()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextRecallSelectedSnippet {
    pub snippet_hash: String,
    pub text: String,
    pub estimated_tokens: u32,
    #[serde(default, skip_serializing_if = "is_false")]
    pub redacted: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub truncated: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextRecallSelectedSnippetSafety {
    pub ready_for_shadow_handoff: bool,
    pub bounded: bool,
    pub origin_identifiers_exposed: bool,
    pub raw_ranked_payload_exposed: bool,
    pub rank_explanation_exposed: bool,
    pub control_marker_exposed: bool,
    pub query_payload_exposed: bool,
    pub per_origin_list_exposed: bool,
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextMemoryTaxonomyClass {
    Semantic,
    Episodic,
    Procedural,
    Control,
    Transcript,
    #[default]
    Unknown,
}

impl TurnContextMemoryTaxonomyClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Semantic => "semantic",
            Self::Episodic => "episodic",
            Self::Procedural => "procedural",
            Self::Control => "control",
            Self::Transcript => "transcript",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextMemoryTaxonomyBucket {
    pub class: TurnContextMemoryTaxonomyClass,
    pub source_count: u32,
    pub returned_count: u32,
    pub available_count: u32,
    pub omitted_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub provenance_span_count: u32,
}

impl TurnContextMemoryTaxonomyBucket {
    pub fn schema_version(&self) -> Option<u32> {
        self.has_payload_light_integrity()
            .then_some(TURN_CONTEXT_MEMORY_TAXONOMY_SCHEMA_VERSION)
    }

    pub fn has_payload_light_integrity(&self) -> bool {
        !self.class.is_unknown()
            && self.returned_count <= self.available_count
            && self.omitted_count == self.available_count.saturating_sub(self.returned_count)
            && (self.source_count > 0
                || (self.returned_count == 0
                    && self.available_count == 0
                    && self.omitted_count == 0
                    && self.provenance_span_count == 0))
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextMemoryFormationCandidateType {
    Fact,
    Task,
    Preference,
    Decision,
    Summary,
    #[default]
    Unknown,
}

impl TurnContextMemoryFormationCandidateType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fact => "fact",
            Self::Task => "task",
            Self::Preference => "preference",
            Self::Decision => "decision",
            Self::Summary => "summary",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextMemoryFormationReceipt {
    pub candidate_type: TurnContextMemoryFormationCandidateType,
    pub transcript_span_count: u32,
    pub provenance_span_count: u32,
    pub confidence_basis_points: u32,
    pub idempotency_key_hash: String,
    pub privacy_class: String,
    pub queued_for_background: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub production_write: bool,
}

impl TurnContextMemoryFormationReceipt {
    pub fn schema_version(&self) -> Option<u32> {
        self.has_payload_light_integrity()
            .then_some(TURN_CONTEXT_MEMORY_FORMATION_RECEIPT_SCHEMA_VERSION)
    }

    pub fn has_payload_light_integrity(&self) -> bool {
        !self.candidate_type.is_unknown()
            && self.transcript_span_count > 0
            && self.provenance_span_count > 0
            && self.provenance_span_count <= self.transcript_span_count
            && self.confidence_basis_points <= 10_000
            && is_stable_manifest_replay_hash(&self.idempotency_key_hash)
            && compression_candidate_source_id_is_payload_light(&self.privacy_class)
            && self.queued_for_background
            && !self.production_write
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextMemoryTemporalFactType {
    Attribute,
    Preference,
    TaskState,
    Decision,
    Summary,
    #[default]
    Unknown,
}

impl TurnContextMemoryTemporalFactType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Attribute => "attribute",
            Self::Preference => "preference",
            Self::TaskState => "task_state",
            Self::Decision => "decision",
            Self::Summary => "summary",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextMemoryTemporalFact {
    pub fact_type: TurnContextMemoryTemporalFactType,
    pub entity_hash: String,
    pub provenance_span_count: u32,
    pub valid_from_sequence: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub invalid_at_sequence: Option<u32>,
    pub confidence_basis_points: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub supersedes_fact_hash: Option<String>,
    pub privacy_class: String,
    pub dry_run_only: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub production_write: bool,
}

impl TurnContextMemoryTemporalFact {
    pub fn schema_version(&self) -> Option<u32> {
        self.has_payload_light_integrity()
            .then_some(TURN_CONTEXT_MEMORY_TEMPORAL_FACT_SCHEMA_VERSION)
    }

    pub fn has_payload_light_integrity(&self) -> bool {
        !self.fact_type.is_unknown()
            && is_stable_manifest_replay_hash(&self.entity_hash)
            && self.provenance_span_count > 0
            && self.valid_from_sequence > 0
            && self
                .invalid_at_sequence
                .is_none_or(|sequence| sequence > self.valid_from_sequence)
            && self.confidence_basis_points <= 10_000
            && self
                .supersedes_fact_hash
                .as_deref()
                .is_none_or(is_stable_manifest_replay_hash)
            && compression_candidate_source_id_is_payload_light(&self.privacy_class)
            && self.dry_run_only
            && !self.production_write
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextCompressionStageKind {
    Summary,
    Rewrite,
    Defragment,
    Prune,
    #[default]
    Unknown,
}

impl TurnContextCompressionStageKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::Rewrite => "rewrite",
            Self::Defragment => "defragment",
            Self::Prune => "prune",
            Self::Unknown => "unknown",
        }
    }

    pub fn schema_version(self) -> Option<u32> {
        (!self.is_unknown()).then_some(TURN_CONTEXT_COMPRESSION_STAGE_SCHEMA_VERSION)
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextCompressionLossCheckStatus {
    MarkerBoundaryOnly,
    SemanticLossCheckPassed,
    SemanticLossCheckFailed,
    NotEvaluated,
    #[default]
    Unknown,
}

impl TurnContextCompressionLossCheckStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MarkerBoundaryOnly => "marker_boundary_only",
            Self::SemanticLossCheckPassed => "semantic_loss_check_passed",
            Self::SemanticLossCheckFailed => "semantic_loss_check_failed",
            Self::NotEvaluated => "not_evaluated",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextCompressionProtectedTierInvariant {
    Preserved,
    #[default]
    Unknown,
}

impl TurnContextCompressionProtectedTierInvariant {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextCompressionStage {
    pub kind: TurnContextCompressionStageKind,
    pub input_tokens: u32,
    pub output_tokens: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub affected_entries: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub loss_check_status: Option<TurnContextCompressionLossCheckStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub rollback_source_text_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub protected_tier_invariant: Option<TurnContextCompressionProtectedTierInvariant>,
}

impl TurnContextCompressionStage {
    pub fn tokens_saved(&self) -> u32 {
        self.input_tokens.saturating_sub(self.output_tokens)
    }

    pub fn has_payload_light_integrity(&self) -> bool {
        !self.kind.is_unknown()
            && self.output_tokens <= self.input_tokens
            && (self.input_tokens == 0 || self.affected_entries > 0)
            && self
                .loss_check_status
                .is_none_or(|status| !status.is_unknown())
            && self
                .rollback_source_text_hash
                .as_deref()
                .is_none_or(is_stable_manifest_replay_hash)
            && self
                .protected_tier_invariant
                .is_none_or(|invariant| !invariant.is_unknown())
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextCompressionCandidateReason {
    BudgetPressureDryRun,
    #[default]
    Unknown,
}

impl TurnContextCompressionCandidateReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BudgetPressureDryRun => "budget_pressure_dry_run",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextBudgetAllocationAction {
    Keep,
    Drop,
    Compress,
    #[default]
    Unknown,
}

impl TurnContextBudgetAllocationAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Keep => "keep",
            Self::Drop => "drop",
            Self::Compress => "compress",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextAdaptiveBudgetAllocation {
    pub tier: TurnContextTier,
    pub source_id: String,
    pub budget_class: String,
    pub input_tokens: u32,
    pub reserve_tokens: u32,
    pub proposed_budget_tokens: u32,
    pub overflow_tokens: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub omit_priority: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub compression_kind: Option<TurnContextCompressionStageKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub estimated_compressed_tokens: Option<u32>,
    pub current_heuristic_action: TurnContextBudgetAllocationAction,
    pub proposed_action: TurnContextBudgetAllocationAction,
    pub would_drop: bool,
    pub would_compress: bool,
}

impl TurnContextAdaptiveBudgetAllocation {
    pub fn schema_version(&self) -> Option<u32> {
        self.has_payload_light_integrity()
            .then_some(TURN_CONTEXT_ADAPTIVE_BUDGET_ALLOCATION_SCHEMA_VERSION)
    }

    pub fn has_payload_light_integrity(&self) -> bool {
        !self.tier.is_unknown()
            && compression_candidate_source_id_is_payload_light(&self.source_id)
            && !self.budget_class.is_empty()
            && self.proposed_budget_tokens <= self.input_tokens
            && self.reserve_tokens <= self.input_tokens
            && self.overflow_tokens
                == self
                    .input_tokens
                    .saturating_sub(self.proposed_budget_tokens)
            && self
                .estimated_compressed_tokens
                .is_none_or(|tokens| tokens <= self.input_tokens)
            && self.compression_kind.is_none_or(|kind| !kind.is_unknown())
            && !self.current_heuristic_action.is_unknown()
            && !self.proposed_action.is_unknown()
            && self.would_drop == (self.proposed_action == TurnContextBudgetAllocationAction::Drop)
            && self.would_compress
                == (self.proposed_action == TurnContextBudgetAllocationAction::Compress)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextCompressionCandidate {
    pub kind: TurnContextCompressionStageKind,
    pub tier: TurnContextTier,
    pub source_id: String,
    pub input_tokens: u32,
    pub estimated_output_tokens: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub affected_entries: u32,
    pub not_executed_reason: TurnContextCompressionCandidateReason,
}

impl TurnContextCompressionCandidate {
    pub fn schema_version(&self) -> Option<u32> {
        self.has_payload_light_integrity()
            .then_some(TURN_CONTEXT_COMPRESSION_CANDIDATE_SCHEMA_VERSION)
    }

    pub fn estimated_tokens_saved(&self) -> u32 {
        self.input_tokens
            .saturating_sub(self.estimated_output_tokens)
    }

    pub fn has_payload_light_integrity(&self) -> bool {
        !self.kind.is_unknown()
            && !self.tier.is_unknown()
            && compression_candidate_source_id_is_payload_light(&self.source_id)
            && self.estimated_output_tokens <= self.input_tokens
            && (self.input_tokens == 0 || self.affected_entries > 0)
            && !self.not_executed_reason.is_unknown()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextManifestItem {
    pub version: u32,
    pub estimated_tokens: u32,
    /// Stable 16-hex replay identity over the payload-light manifest fields.
    /// This is not a cryptographic trust digest and must not be used for
    /// operator approval, release, or activation integrity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub ledger_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub budget_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub omitted_entries: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub omitted_sources: Vec<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub truncated: bool,
    /// Stable 16-hex replay identity over the payload-light decision ledger.
    /// This is not a cryptographic trust digest and must not be used for
    /// operator approval, release, or activation integrity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub decision_ledger_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub decision_ledger: Vec<TurnContextDecisionEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub recall_selection: Option<TurnContextRecallSelectionSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub recall_selected_snippets: Option<TurnContextRecallSelectedSnippetEnvelope>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub memory_taxonomy: Vec<TurnContextMemoryTaxonomyBucket>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub memory_formation_receipts: Vec<TurnContextMemoryFormationReceipt>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub memory_temporal_facts: Vec<TurnContextMemoryTemporalFact>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compression_candidates: Vec<TurnContextCompressionCandidate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adaptive_budget_allocations: Vec<TurnContextAdaptiveBudgetAllocation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compression_stages: Vec<TurnContextCompressionStage>,
    pub entries: Vec<TurnContextManifestEntry>,
}

impl TurnContextManifestItem {
    pub fn with_refreshed_ledger_hash(mut self) -> Self {
        self.refresh_ledger_hash();
        self
    }

    pub fn refresh_ledger_hash(&mut self) {
        self.ledger_hash = Some(self.compute_ledger_hash());
        self.decision_ledger_hash = (!self.decision_ledger.is_empty())
            .then(|| compute_decision_ledger_hash(&self.decision_ledger));
    }

    pub fn has_supported_version(&self) -> bool {
        self.version == TURN_CONTEXT_MANIFEST_VERSION
    }

    pub fn ledger_hash_matches_manifest(&self) -> bool {
        self.ledger_hash
            .as_deref()
            .is_none_or(|hash| hash == self.compute_ledger_hash())
    }

    pub fn ledger_hash_is_compatible(&self) -> bool {
        self.ledger_hash
            .as_deref()
            .is_none_or(is_stable_manifest_replay_hash)
            && self.ledger_hash_matches_manifest()
    }

    pub fn entries_have_replay_integrity(&self) -> bool {
        !self.entries.is_empty()
            && self.entries.iter().all(|entry| {
                !entry.role.is_empty()
                    && !entry.source.is_empty()
                    && !entry.replay_key.is_empty()
                    && is_stable_manifest_replay_hash(&entry.text_hash)
                    && entry.replay_key.ends_with(&format!(":{}", entry.text_hash))
            })
    }

    pub fn decision_ledger_has_integrity(&self) -> bool {
        self.decision_ledger.iter().all(|entry| {
            !entry.source.is_empty()
                && !entry.decision.is_empty()
                && entry
                    .reason_hash
                    .as_deref()
                    .is_none_or(is_stable_manifest_replay_hash)
                && entry.kind().is_known()
        })
    }

    pub fn decision_ledger_summary(&self) -> TurnContextDecisionLedgerSummary {
        summarize_turn_context_decision_ledger(&self.decision_ledger)
    }

    pub fn decision_ledger_hash_is_compatible(&self) -> bool {
        match (
            self.decision_ledger.is_empty(),
            self.decision_ledger_hash.as_deref(),
        ) {
            (true, None) => true,
            (true, Some(hash)) => is_stable_manifest_replay_hash(hash),
            (false, Some(hash)) => {
                is_stable_manifest_replay_hash(hash)
                    && hash == compute_decision_ledger_hash(&self.decision_ledger)
            }
            (false, None) => true,
        }
    }

    pub fn recall_selection_has_integrity(&self) -> bool {
        self.recall_selection
            .as_ref()
            .is_none_or(TurnContextRecallSelectionSummary::has_count_integrity)
    }

    pub fn recall_selected_snippets_have_integrity(&self) -> bool {
        self.recall_selected_snippets
            .as_ref()
            .is_none_or(TurnContextRecallSelectedSnippetEnvelope::has_shadow_integrity)
    }

    pub fn memory_taxonomy_has_integrity(&self) -> bool {
        self.memory_taxonomy
            .iter()
            .all(TurnContextMemoryTaxonomyBucket::has_payload_light_integrity)
    }

    pub fn memory_formation_receipts_have_integrity(&self) -> bool {
        self.memory_formation_receipts
            .iter()
            .all(TurnContextMemoryFormationReceipt::has_payload_light_integrity)
    }

    pub fn memory_temporal_facts_have_integrity(&self) -> bool {
        self.memory_temporal_facts
            .iter()
            .all(TurnContextMemoryTemporalFact::has_payload_light_integrity)
    }

    pub fn compression_stages_have_integrity(&self) -> bool {
        self.compression_stages
            .iter()
            .all(TurnContextCompressionStage::has_payload_light_integrity)
    }

    pub fn compression_candidates_have_integrity(&self) -> bool {
        self.compression_candidates
            .iter()
            .all(TurnContextCompressionCandidate::has_payload_light_integrity)
    }

    pub fn adaptive_budget_allocations_have_integrity(&self) -> bool {
        self.adaptive_budget_allocations
            .iter()
            .all(TurnContextAdaptiveBudgetAllocation::has_payload_light_integrity)
    }

    pub fn has_replay_integrity(&self) -> bool {
        self.has_supported_version()
            && self.entries_have_replay_integrity()
            && self.ledger_hash_is_compatible()
            && self.decision_ledger_has_integrity()
            && self.decision_ledger_hash_is_compatible()
            && self.recall_selection_has_integrity()
            && self.recall_selected_snippets_have_integrity()
            && self.memory_taxonomy_has_integrity()
            && self.memory_formation_receipts_have_integrity()
            && self.memory_temporal_facts_have_integrity()
            && self.compression_candidates_have_integrity()
            && self.adaptive_budget_allocations_have_integrity()
            && self.compression_stages_have_integrity()
    }

    fn compute_ledger_hash(&self) -> String {
        let mut hash = StableManifestReplayHash::new();
        hash.update_u32(self.version);
        hash.update_u32(self.estimated_tokens);
        hash.update_option_u32(self.budget_tokens);
        hash.update_u32(self.omitted_entries);
        hash.update_vec_str(&self.omitted_sources);
        hash.update_bool(self.truncated);
        hash.update_vec_decisions(&self.decision_ledger);
        hash.update_recall_selection(self.recall_selection.as_ref());
        hash.update_recall_selected_snippets(self.recall_selected_snippets.as_ref());
        hash.update_memory_taxonomy(&self.memory_taxonomy);
        hash.update_memory_formation_receipts(&self.memory_formation_receipts);
        hash.update_memory_temporal_facts(&self.memory_temporal_facts);
        hash.update_compression_candidates(&self.compression_candidates);
        hash.update_adaptive_budget_allocations(&self.adaptive_budget_allocations);
        hash.update_compression_stages(&self.compression_stages);
        hash.update_vec_entries(&self.entries);
        hash.finish()
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero_u32(value: &u32) -> bool {
    *value == 0
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(value: &bool) -> bool {
    !*value
}

fn is_stable_manifest_replay_hash(value: &str) -> bool {
    value.len() == 16 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn compression_candidate_source_id_is_payload_light(value: &str) -> bool {
    const FORBIDDEN_SUBSTRINGS: &[&str] = &[
        "memory_id",
        "neuron_id",
        "prompt_text",
        "query",
        "replay_key",
        "snippet_text",
        "text_hash",
        "topic_id",
    ];

    !value.is_empty()
        && value.len() <= 96
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && !FORBIDDEN_SUBSTRINGS
            .iter()
            .any(|forbidden| value.contains(forbidden))
}

/// Returns the stable 16-hex replay hash used by turn-context manifest
/// payload-light fields. This hash is deterministic for replay/debug
/// comparison only; it is intentionally not a cryptographic trust digest.
pub fn stable_turn_context_manifest_replay_hash(value: &str) -> String {
    let mut hash = StableManifestReplayHash::new();
    hash.update_str(value);
    hash.finish()
}

/// Backwards-compatible name for entry text hashes. New code that needs to
/// emphasize the trust boundary should call
/// [`stable_turn_context_manifest_replay_hash`] instead.
pub fn stable_turn_context_manifest_text_hash(value: &str) -> String {
    stable_turn_context_manifest_replay_hash(value)
}

fn compute_decision_ledger_hash(entries: &[TurnContextDecisionEntry]) -> String {
    let mut hash = StableManifestReplayHash::new();
    hash.update_vec_decisions(entries);
    hash.finish()
}

struct StableManifestReplayHash(u64);

impl StableManifestReplayHash {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;

    fn new() -> Self {
        Self(Self::OFFSET)
    }

    fn update_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
        self.0 ^= 0xff;
        self.0 = self.0.wrapping_mul(Self::PRIME);
    }

    fn update_str(&mut self, value: &str) {
        self.update_bytes(value.as_bytes());
    }

    fn update_bool(&mut self, value: bool) {
        self.update_str(if value { "true" } else { "false" });
    }

    fn update_u32(&mut self, value: u32) {
        self.update_str(&value.to_string());
    }

    fn update_option_u32(&mut self, value: Option<u32>) {
        match value {
            Some(value) => {
                self.update_str("some");
                self.update_u32(value);
            }
            None => self.update_str("none"),
        }
    }

    fn update_option_str(&mut self, value: Option<&str>) {
        match value {
            Some(value) => {
                self.update_str("some");
                self.update_str(value);
            }
            None => self.update_str("none"),
        }
    }

    fn update_vec_str(&mut self, values: &[String]) {
        self.update_u32(u32::try_from(values.len()).unwrap_or(u32::MAX));
        for value in values {
            self.update_str(value);
        }
    }

    fn update_vec_entries(&mut self, entries: &[TurnContextManifestEntry]) {
        self.update_u32(u32::try_from(entries.len()).unwrap_or(u32::MAX));
        let include_tiers = entries.iter().any(|entry| !entry.tier.is_unknown());
        for entry in entries {
            self.update_str(&entry.role);
            if include_tiers {
                self.update_str(entry.tier.as_str());
            }
            self.update_str(&entry.source);
            self.update_str(&entry.replay_key);
            self.update_str(&entry.text_hash);
            self.update_u32(entry.estimated_tokens);
        }
    }

    fn update_vec_decisions(&mut self, entries: &[TurnContextDecisionEntry]) {
        self.update_u32(u32::try_from(entries.len()).unwrap_or(u32::MAX));
        for entry in entries {
            self.update_str(&entry.source);
            self.update_str(&entry.decision);
            if let Some(reason_hash) = &entry.reason_hash {
                self.update_str("some");
                self.update_str(reason_hash);
            } else {
                self.update_str("none");
            }
        }
    }

    fn update_recall_selection(
        &mut self,
        recall_selection: Option<&TurnContextRecallSelectionSummary>,
    ) {
        let Some(recall_selection) = recall_selection else {
            self.update_str("none");
            return;
        };
        self.update_str("some");
        self.update_u32(recall_selection.returned_source_count);
        self.update_u32(recall_selection.selected_source_count);
        self.update_u32(recall_selection.ranked_source_count);
        self.update_u32(recall_selection.returned_unselected_source_count);
        self.update_bool(recall_selection.source_diversity_met);
        self.update_u32(recall_selection.source_diversity_target);
        self.update_u32(recall_selection.max_per_source);
        self.update_u32(recall_selection.ranked_item_count);
        self.update_u32(recall_selection.omitted_by_budget_count);
        self.update_u32(recall_selection.memory_control_omitted_count);
        self.update_u32(recall_selection.low_trust_ranked_item_count);
        self.update_u32(recall_selection.low_recency_ranked_item_count);
    }

    fn update_recall_selected_snippets(
        &mut self,
        envelope: Option<&TurnContextRecallSelectedSnippetEnvelope>,
    ) {
        let Some(envelope) = envelope else {
            self.update_str("none");
            return;
        };
        self.update_str("some");
        self.update_u32(envelope.version);
        self.update_u32(envelope.max_snippets);
        self.update_u32(envelope.max_snippet_chars);
        self.update_u32(envelope.selected_snippet_count);
        self.update_u32(envelope.omitted_snippet_count);
        self.update_u32(envelope.redacted_snippet_count);
        self.update_u32(envelope.truncated_snippet_count);
        self.update_u32(u32::try_from(envelope.snippets.len()).unwrap_or(u32::MAX));
        for snippet in &envelope.snippets {
            self.update_str(&snippet.snippet_hash);
            self.update_str(&snippet.text);
            self.update_u32(snippet.estimated_tokens);
            self.update_bool(snippet.redacted);
            self.update_bool(snippet.truncated);
        }
        self.update_bool(envelope.safety.ready_for_shadow_handoff);
        self.update_bool(envelope.safety.bounded);
        self.update_bool(envelope.safety.origin_identifiers_exposed);
        self.update_bool(envelope.safety.raw_ranked_payload_exposed);
        self.update_bool(envelope.safety.rank_explanation_exposed);
        self.update_bool(envelope.safety.control_marker_exposed);
        self.update_bool(envelope.safety.query_payload_exposed);
        self.update_bool(envelope.safety.per_origin_list_exposed);
    }

    fn update_memory_taxonomy(&mut self, buckets: &[TurnContextMemoryTaxonomyBucket]) {
        if buckets.is_empty() {
            return;
        }
        self.update_u32(u32::try_from(buckets.len()).unwrap_or(u32::MAX));
        for bucket in buckets {
            self.update_str(bucket.class.as_str());
            self.update_u32(bucket.source_count);
            self.update_u32(bucket.returned_count);
            self.update_u32(bucket.available_count);
            self.update_u32(bucket.omitted_count);
            self.update_u32(bucket.provenance_span_count);
        }
    }

    fn update_memory_formation_receipts(&mut self, receipts: &[TurnContextMemoryFormationReceipt]) {
        if receipts.is_empty() {
            return;
        }
        self.update_u32(u32::try_from(receipts.len()).unwrap_or(u32::MAX));
        for receipt in receipts {
            self.update_str(receipt.candidate_type.as_str());
            self.update_u32(receipt.transcript_span_count);
            self.update_u32(receipt.provenance_span_count);
            self.update_u32(receipt.confidence_basis_points);
            self.update_str(&receipt.idempotency_key_hash);
            self.update_str(&receipt.privacy_class);
            self.update_bool(receipt.queued_for_background);
            self.update_bool(receipt.production_write);
        }
    }

    fn update_memory_temporal_facts(&mut self, facts: &[TurnContextMemoryTemporalFact]) {
        if facts.is_empty() {
            return;
        }
        self.update_u32(u32::try_from(facts.len()).unwrap_or(u32::MAX));
        for fact in facts {
            self.update_str(fact.fact_type.as_str());
            self.update_str(&fact.entity_hash);
            self.update_u32(fact.provenance_span_count);
            self.update_u32(fact.valid_from_sequence);
            if let Some(sequence) = fact.invalid_at_sequence {
                self.update_str("some");
                self.update_u32(sequence);
            } else {
                self.update_str("none");
            }
            self.update_u32(fact.confidence_basis_points);
            if let Some(supersedes_fact_hash) = &fact.supersedes_fact_hash {
                self.update_str("some");
                self.update_str(supersedes_fact_hash);
            } else {
                self.update_str("none");
            }
            self.update_str(&fact.privacy_class);
            self.update_bool(fact.dry_run_only);
            self.update_bool(fact.production_write);
        }
    }

    fn update_compression_candidates(&mut self, candidates: &[TurnContextCompressionCandidate]) {
        if candidates.is_empty() {
            return;
        }
        self.update_u32(u32::try_from(candidates.len()).unwrap_or(u32::MAX));
        for candidate in candidates {
            self.update_str(candidate.kind.as_str());
            self.update_str(candidate.tier.as_str());
            self.update_str(&candidate.source_id);
            self.update_u32(candidate.input_tokens);
            self.update_u32(candidate.estimated_output_tokens);
            self.update_u32(candidate.affected_entries);
            self.update_str(candidate.not_executed_reason.as_str());
        }
    }

    fn update_adaptive_budget_allocations(
        &mut self,
        allocations: &[TurnContextAdaptiveBudgetAllocation],
    ) {
        if allocations.is_empty() {
            return;
        }
        self.update_u32(u32::try_from(allocations.len()).unwrap_or(u32::MAX));
        for allocation in allocations {
            self.update_str(allocation.tier.as_str());
            self.update_str(&allocation.source_id);
            self.update_str(&allocation.budget_class);
            self.update_u32(allocation.input_tokens);
            self.update_u32(allocation.reserve_tokens);
            self.update_u32(allocation.proposed_budget_tokens);
            self.update_u32(allocation.overflow_tokens);
            self.update_option_u32(allocation.omit_priority);
            match allocation.compression_kind {
                Some(kind) => {
                    self.update_str("some");
                    self.update_str(kind.as_str());
                }
                None => self.update_str("none"),
            }
            self.update_option_u32(allocation.estimated_compressed_tokens);
            self.update_str(allocation.current_heuristic_action.as_str());
            self.update_str(allocation.proposed_action.as_str());
            self.update_bool(allocation.would_drop);
            self.update_bool(allocation.would_compress);
        }
    }

    fn update_compression_stages(&mut self, stages: &[TurnContextCompressionStage]) {
        if stages.is_empty() {
            return;
        }
        self.update_u32(u32::try_from(stages.len()).unwrap_or(u32::MAX));
        for stage in stages {
            self.update_str(stage.kind.as_str());
            self.update_u32(stage.input_tokens);
            self.update_u32(stage.output_tokens);
            self.update_u32(stage.affected_entries);
            self.update_option_str(
                stage
                    .loss_check_status
                    .map(TurnContextCompressionLossCheckStatus::as_str),
            );
            self.update_option_str(stage.rollback_source_text_hash.as_deref());
            self.update_option_str(
                stage
                    .protected_tier_invariant
                    .map(TurnContextCompressionProtectedTierInvariant::as_str),
            );
        }
    }

    fn finish(self) -> String {
        format!("{:016x}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextNetworkItem {
    pub allowed_domains: Vec<String>,
    pub denied_domains: Vec<String>,
}

/// Persist once per real user turn after computing that turn's model-visible
/// context updates, and again after mid-turn compaction when replacement
/// history re-establishes full context, so resume/fork replay can recover the
/// latest durable baseline.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, TS)]
pub struct TurnContextItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    pub cwd: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    pub approval_policy: AskForApproval,
    pub sandbox_policy: SandboxPolicy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_profile: Option<PermissionProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<TurnContextNetworkItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_system_sandbox_policy: Option<FileSystemSandboxPolicy>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personality: Option<Personality>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collaboration_mode: Option<CollaborationMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub realtime_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffortConfig>,
    pub summary: ReasoningSummaryConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_output_json_schema: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation_policy: Option<TruncationPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub context_manifest: Option<TurnContextManifestItem>,
}

impl TurnContextItem {
    pub fn permission_profile(&self) -> PermissionProfile {
        self.permission_profile.clone().unwrap_or_else(|| {
            let file_system_sandbox_policy =
                self.file_system_sandbox_policy.clone().unwrap_or_else(|| {
                    FileSystemSandboxPolicy::from_legacy_sandbox_policy_for_cwd(
                        &self.sandbox_policy,
                        &self.cwd,
                    )
                });
            PermissionProfile::from_runtime_permissions_with_enforcement(
                SandboxEnforcement::from_legacy_sandbox_policy(&self.sandbox_policy),
                &file_system_sandbox_policy,
                NetworkSandboxPolicy::from(&self.sandbox_policy),
            )
        })
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(tag = "mode", content = "limit", rename_all = "snake_case")]
pub enum TruncationPolicy {
    Bytes(usize),
    Tokens(usize),
}

impl From<crate::openai_models::TruncationPolicyConfig> for TruncationPolicy {
    fn from(config: crate::openai_models::TruncationPolicyConfig) -> Self {
        match config.mode {
            crate::openai_models::TruncationMode::Bytes => Self::Bytes(config.limit as usize),
            crate::openai_models::TruncationMode::Tokens => Self::Tokens(config.limit as usize),
        }
    }
}

impl TruncationPolicy {
    pub fn token_budget(&self) -> usize {
        match self {
            TruncationPolicy::Bytes(bytes) => {
                usize::try_from(codex_utils_string::approx_tokens_from_byte_count(*bytes))
                    .unwrap_or(usize::MAX)
            }
            TruncationPolicy::Tokens(tokens) => *tokens,
        }
    }

    pub fn byte_budget(&self) -> usize {
        match self {
            TruncationPolicy::Bytes(bytes) => *bytes,
            TruncationPolicy::Tokens(tokens) => {
                codex_utils_string::approx_bytes_for_tokens(*tokens)
            }
        }
    }
}

impl Mul<f64> for TruncationPolicy {
    type Output = Self;

    fn mul(self, multiplier: f64) -> Self::Output {
        match self {
            TruncationPolicy::Bytes(bytes) => {
                TruncationPolicy::Bytes((bytes as f64 * multiplier).ceil() as usize)
            }
            TruncationPolicy::Tokens(tokens) => {
                TruncationPolicy::Tokens((tokens as f64 * multiplier).ceil() as usize)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct RolloutLine {
    pub timestamp: String,
    #[serde(flatten)]
    pub item: RolloutItem,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, TS)]
pub struct GitInfo {
    /// Current commit hash (SHA)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_hash: Option<GitSha>,
    /// Current branch name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Repository URL (if available from remote)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDelivery {
    Inline,
    Detached,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, TS)]
#[serde(tag = "type", rename_all = "camelCase")]
#[ts(tag = "type")]
pub enum ReviewTarget {
    /// Review the working tree: staged, unstaged, and untracked files.
    UncommittedChanges,

    /// Review changes between the current branch and the given base branch.
    #[serde(rename_all = "camelCase")]
    #[ts(rename_all = "camelCase")]
    BaseBranch { branch: String },

    /// Review the changes introduced by a specific commit.
    #[serde(rename_all = "camelCase")]
    #[ts(rename_all = "camelCase")]
    Commit {
        sha: String,
        /// Optional human-readable label (e.g., commit subject) for UIs.
        title: Option<String>,
    },

    /// Arbitrary instructions provided by the user.
    #[serde(rename_all = "camelCase")]
    #[ts(rename_all = "camelCase")]
    Custom { instructions: String },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
/// Review request sent to the review session.
pub struct ReviewRequest {
    pub target: ReviewTarget,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub user_facing_hint: Option<String>,
}

/// Structured review result produced by a child review session.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct ReviewOutputEvent {
    pub findings: Vec<ReviewFinding>,
    pub overall_correctness: String,
    pub overall_explanation: String,
    pub overall_confidence_score: f32,
}

impl Default for ReviewOutputEvent {
    fn default() -> Self {
        Self {
            findings: Vec::new(),
            overall_correctness: String::default(),
            overall_explanation: String::default(),
            overall_confidence_score: 0.0,
        }
    }
}

/// A single review finding describing an observed issue or recommendation.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct ReviewFinding {
    pub title: String,
    pub body: String,
    pub confidence_score: f32,
    pub priority: i32,
    pub code_location: ReviewCodeLocation,
}

/// Location of the code related to a review finding.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct ReviewCodeLocation {
    pub absolute_file_path: PathBuf,
    pub line_range: ReviewLineRange,
}

/// Inclusive line range in a file associated with the finding.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct ReviewLineRange {
    pub start: u32,
    pub end: u32,
}

#[derive(
    Debug, Clone, Copy, Display, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum ExecCommandSource {
    #[default]
    Agent,
    UserShell,
    UnifiedExecStartup,
    UnifiedExecInteraction,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum ExecCommandStatus {
    Completed,
    Failed,
    Declined,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct ExecCommandBeginEvent {
    /// Identifier so this can be paired with the ExecCommandEnd event.
    pub call_id: String,
    /// Identifier for the underlying PTY process (when available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub process_id: Option<String>,
    /// Turn ID that this command belongs to.
    pub turn_id: String,
    #[serde(default)]
    pub started_at_ms: i64,
    /// The command to be executed.
    pub command: Vec<String>,
    /// The command's working directory if not the default cwd for the agent.
    pub cwd: AbsolutePathBuf,
    pub parsed_cmd: Vec<ParsedCommand>,
    /// Where the command originated. Defaults to Agent for backward compatibility.
    #[serde(default)]
    pub source: ExecCommandSource,
    /// Raw input sent to a unified exec session (if this is an interaction event).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub interaction_input: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct ExecCommandEndEvent {
    /// Identifier for the ExecCommandBegin that finished.
    pub call_id: String,
    /// Identifier for the underlying PTY process (when available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub process_id: Option<String>,
    /// Turn ID that this command belongs to.
    pub turn_id: String,
    #[serde(default)]
    pub completed_at_ms: i64,
    /// The command that was executed.
    pub command: Vec<String>,
    /// The command's working directory if not the default cwd for the agent.
    pub cwd: AbsolutePathBuf,
    pub parsed_cmd: Vec<ParsedCommand>,
    /// Where the command originated. Defaults to Agent for backward compatibility.
    #[serde(default)]
    pub source: ExecCommandSource,
    /// Raw input sent to a unified exec session (if this is an interaction event).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub interaction_input: Option<String>,

    /// Captured stdout
    pub stdout: String,
    /// Captured stderr
    pub stderr: String,
    /// Captured aggregated output
    #[serde(default)]
    pub aggregated_output: String,
    /// The command's exit code.
    pub exit_code: i32,
    /// The duration of the command execution.
    #[ts(type = "string")]
    pub duration: Duration,
    /// Formatted output from the command, as seen by the model.
    pub formatted_output: String,
    /// Completion status for this command execution.
    pub status: ExecCommandStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct ViewImageToolCallEvent {
    /// Identifier for the originating tool call.
    pub call_id: String,
    /// Local filesystem path provided to the tool.
    pub path: AbsolutePathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum ExecOutputStream {
    Stdout,
    Stderr,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct ExecCommandOutputDeltaEvent {
    /// Identifier for the ExecCommandBegin that produced this chunk.
    pub call_id: String,
    /// Which stream produced this chunk.
    pub stream: ExecOutputStream,
    /// Raw bytes from the stream (may not be valid UTF-8).
    #[serde_as(as = "serde_with::base64::Base64")]
    #[schemars(with = "String")]
    #[ts(type = "string")]
    pub chunk: Vec<u8>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct TerminalInteractionEvent {
    /// Identifier for the ExecCommandBegin that produced this chunk.
    pub call_id: String,
    /// Process id associated with the running command.
    pub process_id: String,
    /// Stdin sent to the running session.
    pub stdin: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct DeprecationNoticeEvent {
    /// Concise summary of what is deprecated.
    pub summary: String,
    /// Optional extra guidance, such as migration steps or rationale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct ThreadRolledBackEvent {
    /// Number of user turns that were removed from context.
    pub num_turns: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct StreamErrorEvent {
    pub message: String,
    #[serde(default)]
    pub codex_error_info: Option<CodexErrorInfo>,
    /// Optional details about the underlying stream failure (often the same
    /// human-readable message that is surfaced as the terminal error if retries
    /// are exhausted).
    #[serde(default)]
    pub additional_details: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct StreamInfoEvent {
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct PatchApplyBeginEvent {
    /// Identifier so this can be paired with the PatchApplyEnd event.
    pub call_id: String,
    /// Turn ID that this patch belongs to.
    /// Uses `#[serde(default)]` for backwards compatibility.
    #[serde(default)]
    pub turn_id: String,
    /// If true, there was no ApplyPatchApprovalRequest for this patch.
    pub auto_approved: bool,
    /// The changes to be applied.
    pub changes: HashMap<PathBuf, FileChange>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct PatchApplyUpdatedEvent {
    /// Identifier for the originating `apply_patch` tool call.
    pub call_id: String,
    /// Structured file changes parsed from the model-generated patch input so far.
    pub changes: HashMap<PathBuf, FileChange>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct PatchApplyEndEvent {
    /// Identifier for the PatchApplyBegin that finished.
    pub call_id: String,
    /// Turn ID that this patch belongs to.
    /// Uses `#[serde(default)]` for backwards compatibility.
    #[serde(default)]
    pub turn_id: String,
    /// Captured stdout (summary printed by apply_patch).
    pub stdout: String,
    /// Captured stderr (parser errors, IO failures, etc.).
    pub stderr: String,
    /// Whether the patch was applied successfully.
    pub success: bool,
    /// The changes that were applied (mirrors PatchApplyBeginEvent::changes).
    #[serde(default)]
    pub changes: HashMap<PathBuf, FileChange>,
    /// Completion status for this patch application.
    pub status: PatchApplyStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum PatchApplyStatus {
    Completed,
    Failed,
    Declined,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct TurnDiffEvent {
    pub unified_diff: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct McpStartupUpdateEvent {
    /// Server name being started.
    pub server: String,
    /// Current startup status.
    pub status: McpStartupStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
#[serde(rename_all = "snake_case", tag = "state")]
#[ts(rename_all = "snake_case", tag = "state")]
pub enum McpStartupStatus {
    Starting,
    Ready,
    Failed { error: String },
    Cancelled,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS, Default)]
pub struct McpStartupCompleteEvent {
    pub ready: Vec<String>,
    pub failed: Vec<McpStartupFailure>,
    pub cancelled: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct McpStartupFailure {
    pub server: String,
    pub error: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum McpAuthStatus {
    Unsupported,
    NotLoggedIn,
    BearerToken,
    OAuth,
}

impl fmt::Display for McpAuthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            McpAuthStatus::Unsupported => "Unsupported",
            McpAuthStatus::NotLoggedIn => "Not logged in",
            McpAuthStatus::BearerToken => "Bearer token",
            McpAuthStatus::OAuth => "OAuth",
        };
        f.write_str(text)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct RealtimeConversationListVoicesResponseEvent {
    pub voices: RealtimeVoicesList,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "lowercase")]
#[ts(rename_all = "lowercase")]
pub enum Product {
    #[serde(alias = "CHATGPT")]
    Chatgpt,
    #[serde(alias = "CODEX")]
    Codex,
    #[serde(alias = "ATLAS")]
    Atlas,
    #[serde(alias = "HEPTA")]
    Hepta,
}
impl Product {
    pub fn to_app_platform(self) -> &'static str {
        match self {
            Self::Chatgpt => "chat",
            Self::Codex => "codex",
            Self::Atlas => "atlas",
            // The upstream featured-plugin API does not have a distinct Hepta
            // platform yet. Hepta is intentionally Codex-compatible here.
            Self::Hepta => "codex",
        }
    }

    pub fn from_session_source_name(value: &str) -> Option<Self> {
        let normalized = value.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "chatgpt" => Some(Self::Chatgpt),
            "codex" => Some(Self::Codex),
            "atlas" => Some(Self::Atlas),
            "hepta" => Some(Self::Hepta),
            _ => None,
        }
    }

    pub fn matches_product_restriction(&self, products: &[Product]) -> bool {
        products.is_empty()
            || products.contains(self)
            || matches!(self, Self::Hepta) && products.contains(&Self::Codex)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum SkillScope {
    User,
    Repo,
    System,
    Admin,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    /// Legacy short_description from SKILL.md. Prefer SKILL.json interface.short_description.
    pub short_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub interface: Option<SkillInterface>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub dependencies: Option<SkillDependencies>,
    pub path: AbsolutePathBuf,
    pub scope: SkillScope,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS, PartialEq, Eq)]
pub struct SkillInterface {
    #[ts(optional)]
    pub display_name: Option<String>,
    #[ts(optional)]
    pub short_description: Option<String>,
    #[ts(optional)]
    pub icon_small: Option<AbsolutePathBuf>,
    #[ts(optional)]
    pub icon_large: Option<AbsolutePathBuf>,
    #[ts(optional)]
    pub brand_color: Option<String>,
    #[ts(optional)]
    pub default_prompt: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS, PartialEq, Eq)]
pub struct SkillDependencies {
    pub tools: Vec<SkillToolDependency>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS, PartialEq, Eq)]
pub struct SkillToolDependency {
    #[serde(rename = "type")]
    #[ts(rename = "type")]
    pub r#type: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub transport: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS, PartialEq, Eq)]
pub struct SessionNetworkProxyRuntime {
    pub http_addr: String,
    pub socks_addr: String,
}

#[derive(Debug, Clone, Serialize, JsonSchema, TS)]
pub struct SessionConfiguredEvent {
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forked_from_id: Option<ThreadId>,
    /// Optional analytics source classification for this thread.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_source: Option<ThreadSource>,

    /// Optional user-facing thread name (may be unset).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub thread_name: Option<String>,

    /// Tell the client what model is being queried.
    pub model: String,

    pub model_provider_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,

    /// When to escalate for approval for execution
    pub approval_policy: AskForApproval,

    /// Configures who approval requests are routed to for review once they have
    /// been escalated. This does not disable separate safety checks such as
    /// ARC.
    #[serde(default)]
    pub approvals_reviewer: ApprovalsReviewer,

    /// Canonical effective permissions for commands executed in the session.
    pub permission_profile: PermissionProfile,

    /// Named or implicit built-in profile that produced `permission_profile`,
    /// when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub active_permission_profile: Option<ActivePermissionProfile>,

    /// Working directory that should be treated as the *root* of the
    /// session.
    pub cwd: AbsolutePathBuf,

    /// The effort the model is putting into reasoning about the user's request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffortConfig>,

    /// Optional initial messages (as events) for resumed sessions.
    /// When present, UIs can use these to seed the history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_messages: Option<Vec<EventMsg>>,

    /// Runtime proxy bind addresses, when the managed proxy was started for this session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub network_proxy: Option<SessionNetworkProxyRuntime>,

    /// Path in which the rollout is stored. Can be `None` for ephemeral threads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollout_path: Option<PathBuf>,
}

impl<'de> Deserialize<'de> for SessionConfiguredEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            session_id: SessionId,
            #[serde(default)]
            thread_id: Option<ThreadId>,
            forked_from_id: Option<ThreadId>,
            #[serde(default)]
            thread_source: Option<ThreadSource>,
            #[serde(default)]
            thread_name: Option<String>,
            model: String,
            model_provider_id: String,
            service_tier: Option<String>,
            approval_policy: AskForApproval,
            #[serde(default)]
            approvals_reviewer: ApprovalsReviewer,
            // `SessionConfiguredEvent` is persisted into rollout history. Older
            // rollouts only have `sandbox_policy`, so accept it on deserialize
            // and immediately project it into the canonical `permission_profile`.
            sandbox_policy: Option<SandboxPolicy>,
            permission_profile: Option<PermissionProfile>,
            #[serde(default)]
            active_permission_profile: Option<ActivePermissionProfile>,
            cwd: AbsolutePathBuf,
            reasoning_effort: Option<ReasoningEffortConfig>,
            initial_messages: Option<Vec<EventMsg>>,
            network_proxy: Option<SessionNetworkProxyRuntime>,
            rollout_path: Option<PathBuf>,
        }

        let wire = Wire::deserialize(deserializer)?;
        let permission_profile = match (wire.permission_profile, wire.sandbox_policy) {
            (Some(permission_profile), _) => permission_profile,
            (None, Some(sandbox_policy)) => PermissionProfile::from_legacy_sandbox_policy_for_cwd(
                &sandbox_policy,
                wire.cwd.as_path(),
            ),
            (None, None) => {
                return Err(serde::de::Error::missing_field("permission_profile"));
            }
        };

        Ok(Self {
            session_id: wire.session_id,
            thread_id: wire.thread_id.unwrap_or_else(|| wire.session_id.into()),
            forked_from_id: wire.forked_from_id,
            thread_source: wire.thread_source,
            thread_name: wire.thread_name,
            model: wire.model,
            model_provider_id: wire.model_provider_id,
            service_tier: wire.service_tier,
            approval_policy: wire.approval_policy,
            approvals_reviewer: wire.approvals_reviewer,
            permission_profile,
            active_permission_profile: wire.active_permission_profile,
            cwd: wire.cwd,
            reasoning_effort: wire.reasoning_effort,
            initial_messages: wire.initial_messages,
            network_proxy: wire.network_proxy,
            rollout_path: wire.rollout_path,
        })
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "protocol/")]
pub enum ThreadGoalStatus {
    Active,
    Paused,
    BudgetLimited,
    Complete,
}

pub const MAX_THREAD_GOAL_OBJECTIVE_CHARS: usize = 4_000;

pub fn validate_thread_goal_objective(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err("goal objective must not be empty".to_string());
    }
    if value.chars().count() > MAX_THREAD_GOAL_OBJECTIVE_CHARS {
        return Err(format!(
            "goal objective must be at most {MAX_THREAD_GOAL_OBJECTIVE_CHARS} characters"
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "protocol/")]
pub struct ThreadGoal {
    pub thread_id: ThreadId,
    pub objective: String,
    pub status: ThreadGoalStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub token_budget: Option<i64>,
    pub tokens_used: i64,
    pub time_used_seconds: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "protocol/")]
pub struct ThreadGoalUpdatedEvent {
    pub thread_id: ThreadId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub turn_id: Option<String>,
    pub goal: ThreadGoal,
}

/// User's decision in response to an ExecApprovalRequest.
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Display, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecision {
    /// User has approved this command and the agent should execute it.
    Approved,

    /// User has approved this command and wants to apply the proposed execpolicy
    /// amendment so future matching commands are permitted.
    ApprovedExecpolicyAmendment {
        proposed_execpolicy_amendment: ExecPolicyAmendment,
    },

    /// User has approved this request and wants future prompts in the same
    /// session-scoped approval cache to be automatically approved for the
    /// remainder of the session.
    ApprovedForSession,

    /// User chose to persist a network policy rule (allow/deny) for future
    /// requests to the same host.
    NetworkPolicyAmendment {
        network_policy_amendment: NetworkPolicyAmendment,
    },

    /// User has denied this command and the agent should not execute it, but
    /// it should continue the session and try something else.
    #[default]
    Denied,

    /// Automatic approval review timed out before reaching a decision.
    TimedOut,

    /// User has denied this command and the agent should not do anything until
    /// the user's next command.
    Abort,
}

impl ReviewDecision {
    /// Returns an opaque version of the decision without PII. We can't use an ignored flag
    /// on `serde` because the serialization is required by some surfaces.
    pub fn to_opaque_string(&self) -> &'static str {
        match self {
            ReviewDecision::Approved => "approved",
            ReviewDecision::ApprovedExecpolicyAmendment { .. } => "approved_with_amendment",
            ReviewDecision::ApprovedForSession => "approved_for_session",
            ReviewDecision::NetworkPolicyAmendment {
                network_policy_amendment,
            } => match network_policy_amendment.action {
                NetworkPolicyRuleAction::Allow => "approved_with_network_policy_allow",
                NetworkPolicyRuleAction::Deny => "denied_with_network_policy_deny",
            },
            ReviewDecision::Denied => "denied",
            ReviewDecision::TimedOut => "timed_out",
            ReviewDecision::Abort => "abort",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[ts(tag = "type")]
pub enum FileChange {
    Add {
        content: String,
    },
    Delete {
        content: String,
    },
    Update {
        unified_diff: String,
        move_path: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct Chunk {
    /// 1-based line index of the first line in the original file
    pub orig_index: u32,
    pub deleted_lines: Vec<String>,
    pub inserted_lines: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, TS)]
pub struct TurnAbortedEvent {
    pub turn_id: Option<String>,
    pub reason: TurnAbortReason,
    /// Unix timestamp (in seconds) when the turn was aborted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(type = "number | null", optional)]
    pub completed_at: Option<i64>,
    /// Duration between turn start and abort in milliseconds, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(type = "number | null", optional)]
    pub duration_ms: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum TurnAbortReason {
    Interrupted,
    Replaced,
    ReviewEnded,
    BudgetLimited,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct CollabAgentSpawnBeginEvent {
    /// Identifier for the collab tool call.
    pub call_id: String,
    #[serde(default)]
    pub started_at_ms: i64,
    /// Thread ID of the sender.
    pub sender_thread_id: ThreadId,
    /// Initial prompt sent to the agent. Can be empty to prevent CoT leaking at the
    /// beginning.
    pub prompt: String,
    pub model: String,
    pub reasoning_effort: ReasoningEffortConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct CollabAgentRef {
    /// Thread ID of the receiver/new agent.
    pub thread_id: ThreadId,
    /// Optional nickname assigned to an AgentControl-spawned sub-agent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_nickname: Option<String>,
    /// Optional role (agent_role) assigned to an AgentControl-spawned sub-agent.
    #[serde(default, alias = "agent_type", skip_serializing_if = "Option::is_none")]
    pub agent_role: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema, TS)]
pub struct CollabAgentStatusEntry {
    /// Thread ID of the receiver/new agent.
    pub thread_id: ThreadId,
    /// Optional nickname assigned to an AgentControl-spawned sub-agent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_nickname: Option<String>,
    /// Optional role (agent_role) assigned to an AgentControl-spawned sub-agent.
    #[serde(default, alias = "agent_type", skip_serializing_if = "Option::is_none")]
    pub agent_role: Option<String>,
    /// Last known status of the agent.
    pub status: AgentStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct CollabAgentSpawnEndEvent {
    /// Identifier for the collab tool call.
    pub call_id: String,
    #[serde(default)]
    pub completed_at_ms: i64,
    /// Thread ID of the sender.
    pub sender_thread_id: ThreadId,
    /// Thread ID of the newly spawned agent, if it was created.
    pub new_thread_id: Option<ThreadId>,
    /// Optional nickname assigned to the new agent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_agent_nickname: Option<String>,
    /// Optional role assigned to the new agent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_agent_role: Option<String>,
    /// Initial prompt sent to the agent. Can be empty to prevent CoT leaking at the
    /// beginning.
    pub prompt: String,
    /// Effective model used by the spawned agent after inheritance and role overrides.
    pub model: String,
    /// Effective reasoning effort used by the spawned agent after inheritance and role overrides.
    pub reasoning_effort: ReasoningEffortConfig,
    /// Last known status of the new agent reported to the sender agent.
    pub status: AgentStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct CollabAgentInteractionBeginEvent {
    /// Identifier for the collab tool call.
    pub call_id: String,
    #[serde(default)]
    pub started_at_ms: i64,
    /// Thread ID of the sender.
    pub sender_thread_id: ThreadId,
    /// Thread ID of the receiver.
    pub receiver_thread_id: ThreadId,
    /// Prompt sent from the sender to the receiver. Can be empty to prevent CoT
    /// leaking at the beginning.
    pub prompt: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct CollabAgentInteractionEndEvent {
    /// Identifier for the collab tool call.
    pub call_id: String,
    #[serde(default)]
    pub completed_at_ms: i64,
    /// Thread ID of the sender.
    pub sender_thread_id: ThreadId,
    /// Thread ID of the receiver.
    pub receiver_thread_id: ThreadId,
    /// Optional nickname assigned to the receiver agent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_agent_nickname: Option<String>,
    /// Optional role assigned to the receiver agent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_agent_role: Option<String>,
    /// Prompt sent from the sender to the receiver. Can be empty to prevent CoT
    /// leaking at the beginning.
    pub prompt: String,
    /// Last known status of the receiver agent reported to the sender agent.
    pub status: AgentStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct CollabWaitingBeginEvent {
    #[serde(default)]
    pub started_at_ms: i64,
    /// Thread ID of the sender.
    pub sender_thread_id: ThreadId,
    /// Thread ID of the receivers.
    pub receiver_thread_ids: Vec<ThreadId>,
    /// Optional nicknames/roles for receivers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub receiver_agents: Vec<CollabAgentRef>,
    /// ID of the waiting call.
    pub call_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct CollabWaitingEndEvent {
    /// Thread ID of the sender.
    pub sender_thread_id: ThreadId,
    /// ID of the waiting call.
    pub call_id: String,
    #[serde(default)]
    pub completed_at_ms: i64,
    /// Optional receiver metadata paired with final statuses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agent_statuses: Vec<CollabAgentStatusEntry>,
    /// Last known status of the receiver agents reported to the sender agent.
    pub statuses: HashMap<ThreadId, AgentStatus>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct CollabCloseBeginEvent {
    /// Identifier for the collab tool call.
    pub call_id: String,
    #[serde(default)]
    pub started_at_ms: i64,
    /// Thread ID of the sender.
    pub sender_thread_id: ThreadId,
    /// Thread ID of the receiver.
    pub receiver_thread_id: ThreadId,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct CollabCloseEndEvent {
    /// Identifier for the collab tool call.
    pub call_id: String,
    #[serde(default)]
    pub completed_at_ms: i64,
    /// Thread ID of the sender.
    pub sender_thread_id: ThreadId,
    /// Thread ID of the receiver.
    pub receiver_thread_id: ThreadId,
    /// Optional nickname assigned to the receiver agent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_agent_nickname: Option<String>,
    /// Optional role assigned to the receiver agent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_agent_role: Option<String>,
    /// Last known status of the receiver agent reported to the sender agent before
    /// the close.
    pub status: AgentStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct CollabResumeBeginEvent {
    /// Identifier for the collab tool call.
    pub call_id: String,
    #[serde(default)]
    pub started_at_ms: i64,
    /// Thread ID of the sender.
    pub sender_thread_id: ThreadId,
    /// Thread ID of the receiver.
    pub receiver_thread_id: ThreadId,
    /// Optional nickname assigned to the receiver agent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_agent_nickname: Option<String>,
    /// Optional role assigned to the receiver agent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_agent_role: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, TS)]
pub struct CollabResumeEndEvent {
    /// Identifier for the collab tool call.
    pub call_id: String,
    #[serde(default)]
    pub completed_at_ms: i64,
    /// Thread ID of the sender.
    pub sender_thread_id: ThreadId,
    /// Thread ID of the receiver.
    pub receiver_thread_id: ThreadId,
    /// Optional nickname assigned to the receiver agent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_agent_nickname: Option<String>,
    /// Optional role assigned to the receiver agent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_agent_role: Option<String>,
    /// Last known status of the receiver agent reported to the sender agent after
    /// resume.
    pub status: AgentStatus,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::items::FileChangeItem;
    use crate::items::ImageGenerationItem;
    use crate::items::McpToolCallItem;
    use crate::items::McpToolCallStatus;
    use crate::items::UserMessageItem;
    use crate::items::WebSearchItem;
    use crate::mcp::CallToolResult;
    use crate::permissions::FileSystemAccessMode;
    use crate::permissions::FileSystemPath;
    use crate::permissions::FileSystemSandboxEntry;
    use crate::permissions::FileSystemSandboxPolicy;
    use crate::permissions::FileSystemSpecialPath;
    use crate::permissions::NetworkSandboxPolicy;
    use anyhow::Result;
    use codex_utils_absolute_path::AbsolutePathBuf;
    use codex_utils_absolute_path::test_support::PathBufExt;
    use codex_utils_absolute_path::test_support::test_path_buf;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;
    use tempfile::TempDir;

    #[test]
    fn thread_history_mode_has_stable_wire_values_and_legacy_default() -> Result<()> {
        assert_eq!(ThreadHistoryMode::default(), ThreadHistoryMode::Legacy);
        assert_eq!(ThreadHistoryMode::Legacy.as_str(), "legacy");
        assert_eq!(ThreadHistoryMode::Paginated.as_str(), "paginated");
        assert_eq!(
            serde_json::to_string(&ThreadHistoryMode::Paginated)?,
            r#""paginated""#
        );
        assert_eq!(
            serde_json::from_str::<ThreadHistoryMode>(r#""legacy""#)?,
            ThreadHistoryMode::Legacy
        );
        assert_eq!(
            ThreadHistoryMode::from_str("paginated"),
            Ok(ThreadHistoryMode::Paginated)
        );
        assert!(ThreadHistoryMode::from_str("future").is_err());
        Ok(())
    }

    #[test]
    fn strip_user_message_context_accepts_current_marker() {
        let message = "context\n## My request for Hepta:\n  optimize this\n";

        assert_eq!(strip_user_message_context(message), "optimize this");
    }

    #[test]
    fn strip_user_message_context_accepts_legacy_marker() {
        let message = "context\n## My request for Codex:\n  optimize this\n";

        assert_eq!(strip_user_message_context(message), "optimize this");
    }

    fn sorted_writable_roots(roots: Vec<WritableRoot>) -> Vec<(PathBuf, Vec<PathBuf>)> {
        let mut sorted_roots: Vec<(PathBuf, Vec<PathBuf>)> = roots
            .into_iter()
            .map(|root| {
                let mut read_only_subpaths: Vec<PathBuf> = root
                    .read_only_subpaths
                    .into_iter()
                    .map(|path| path.to_path_buf())
                    .collect();
                read_only_subpaths.sort();
                (root.root.to_path_buf(), read_only_subpaths)
            })
            .collect();
        sorted_roots.sort_by(|left, right| left.0.cmp(&right.0));
        sorted_roots
    }

    fn sandbox_policy_allows_read(policy: &SandboxPolicy, _path: &Path, _cwd: &Path) -> bool {
        policy.has_full_disk_read_access()
    }

    fn sandbox_policy_allows_write(policy: &SandboxPolicy, path: &Path, cwd: &Path) -> bool {
        if policy.has_full_disk_write_access() {
            return true;
        }

        policy
            .get_writable_roots_with_cwd(cwd)
            .iter()
            .any(|root| root.is_path_writable(path))
    }

    #[test]
    fn session_source_from_startup_arg_maps_known_values() {
        assert_eq!(
            SessionSource::from_startup_arg("vscode").unwrap(),
            SessionSource::VSCode
        );
        assert_eq!(
            SessionSource::from_startup_arg("app-server").unwrap(),
            SessionSource::Mcp
        );
    }

    #[test]
    fn inter_agent_communication_response_input_item_preserves_commentary_phase() {
        let communication = InterAgentCommunication {
            author: AgentPath::root(),
            recipient: AgentPath::root().join("reviewer").expect("recipient path"),
            other_recipients: vec![AgentPath::root().join("worker").expect("recipient path")],
            content: "review the diff".to_string(),
            trigger_turn: true,
        };

        assert_eq!(
            communication.to_response_input_item(),
            ResponseInputItem::Message {
                role: "assistant".to_string(),
                content: vec![ContentItem::OutputText {
                    text: serde_json::to_string(&communication).expect("serialize communication"),
                }],
                phase: Some(MessagePhase::Commentary),
            }
        );
    }

    #[test]
    fn session_source_from_startup_arg_normalizes_custom_values() {
        assert_eq!(
            SessionSource::from_startup_arg("atlas").unwrap(),
            SessionSource::Custom("atlas".to_string())
        );
        assert_eq!(
            SessionSource::from_startup_arg(" Atlas ").unwrap(),
            SessionSource::Custom("atlas".to_string())
        );
    }

    #[test]
    fn session_source_restriction_product_defaults_non_subagent_sources_to_hepta() {
        assert_eq!(
            SessionSource::Cli.restriction_product(),
            Some(Product::Hepta)
        );
        assert_eq!(
            SessionSource::VSCode.restriction_product(),
            Some(Product::Hepta)
        );
        assert_eq!(
            SessionSource::Exec.restriction_product(),
            Some(Product::Hepta)
        );
        assert_eq!(
            SessionSource::Mcp.restriction_product(),
            Some(Product::Hepta)
        );
        assert_eq!(
            SessionSource::Unknown.restriction_product(),
            Some(Product::Hepta)
        );
    }

    #[test]
    fn session_source_restriction_product_does_not_guess_subagent_products() {
        assert_eq!(
            SessionSource::SubAgent(SubAgentSource::Review).restriction_product(),
            None
        );
        assert_eq!(
            SessionSource::Internal(InternalSessionSource::MemoryConsolidation)
                .restriction_product(),
            None
        );
    }

    #[test]
    fn session_source_restriction_product_maps_custom_sources_to_products() {
        assert_eq!(
            SessionSource::Custom("chatgpt".to_string()).restriction_product(),
            Some(Product::Chatgpt)
        );
        assert_eq!(
            SessionSource::Custom("ATLAS".to_string()).restriction_product(),
            Some(Product::Atlas)
        );
        assert_eq!(
            SessionSource::Custom("codex".to_string()).restriction_product(),
            Some(Product::Codex)
        );
        assert_eq!(
            SessionSource::Custom("hepta".to_string()).restriction_product(),
            Some(Product::Hepta)
        );
        assert_eq!(
            SessionSource::Custom("atlas-dev".to_string()).restriction_product(),
            None
        );
    }

    #[test]
    fn session_source_matches_product_restriction() {
        assert!(
            SessionSource::Custom("chatgpt".to_string())
                .matches_product_restriction(&[Product::Chatgpt])
        );
        assert!(
            !SessionSource::Custom("chatgpt".to_string())
                .matches_product_restriction(&[Product::Codex])
        );
        assert!(SessionSource::VSCode.matches_product_restriction(&[Product::Codex]));
        assert!(SessionSource::VSCode.matches_product_restriction(&[Product::Hepta]));
        assert!(
            !SessionSource::Custom("codex".to_string())
                .matches_product_restriction(&[Product::Hepta])
        );
        assert!(
            !SessionSource::Custom("atlas-dev".to_string())
                .matches_product_restriction(&[Product::Atlas])
        );
        assert!(SessionSource::Custom("atlas-dev".to_string()).matches_product_restriction(&[]));
    }

    #[test]
    fn hepta_product_uses_codex_platform_compatibility() {
        assert_eq!(Product::Hepta.to_app_platform(), "codex");
        assert!(Product::Hepta.matches_product_restriction(&[Product::Codex]));
        assert!(Product::Hepta.matches_product_restriction(&[Product::Hepta]));
        assert!(!Product::Codex.matches_product_restriction(&[Product::Hepta]));
    }

    fn sandbox_policy_probe_paths(policy: &SandboxPolicy, cwd: &Path) -> Vec<PathBuf> {
        let mut paths = vec![cwd.to_path_buf()];
        for root in policy.get_writable_roots_with_cwd(cwd) {
            paths.push(root.root.to_path_buf());
            paths.extend(
                root.read_only_subpaths
                    .into_iter()
                    .map(|path| path.to_path_buf()),
            );
        }
        paths.sort();
        paths.dedup();
        paths
    }

    fn assert_same_sandbox_policy_semantics(
        expected: &SandboxPolicy,
        actual: &SandboxPolicy,
        cwd: &Path,
    ) {
        assert_eq!(
            actual.has_full_disk_read_access(),
            expected.has_full_disk_read_access()
        );
        assert_eq!(
            actual.has_full_disk_write_access(),
            expected.has_full_disk_write_access()
        );
        assert_eq!(
            actual.has_full_network_access(),
            expected.has_full_network_access()
        );
        let mut probe_paths = sandbox_policy_probe_paths(expected, cwd);
        probe_paths.extend(sandbox_policy_probe_paths(actual, cwd));
        probe_paths.sort();
        probe_paths.dedup();

        for path in probe_paths {
            assert_eq!(
                sandbox_policy_allows_read(actual, &path, cwd),
                sandbox_policy_allows_read(expected, &path, cwd),
                "read access mismatch for {}",
                path.display()
            );
            assert_eq!(
                sandbox_policy_allows_write(actual, &path, cwd),
                sandbox_policy_allows_write(expected, &path, cwd),
                "write access mismatch for {}",
                path.display()
            );
        }
    }

    #[test]
    fn external_sandbox_reports_full_access_flags() {
        let restricted = SandboxPolicy::ExternalSandbox {
            network_access: NetworkAccess::Restricted,
        };
        assert!(restricted.has_full_disk_write_access());
        assert!(!restricted.has_full_network_access());

        let enabled = SandboxPolicy::ExternalSandbox {
            network_access: NetworkAccess::Enabled,
        };
        assert!(enabled.has_full_disk_write_access());
        assert!(enabled.has_full_network_access());
    }

    #[test]
    fn read_only_reports_network_access_flags() {
        let restricted = SandboxPolicy::new_read_only_policy();
        assert!(!restricted.has_full_network_access());

        let enabled = SandboxPolicy::ReadOnly {
            network_access: true,
        };
        assert!(enabled.has_full_network_access());
    }

    #[test]
    fn granular_approval_config_mcp_elicitation_flag_is_field_driven() {
        assert!(
            GranularApprovalConfig {
                sandbox_approval: false,
                rules: false,
                skill_approval: false,
                request_permissions: false,
                mcp_elicitations: true,
            }
            .allows_mcp_elicitations()
        );
        assert!(
            !GranularApprovalConfig {
                sandbox_approval: false,
                rules: false,
                skill_approval: false,
                request_permissions: false,
                mcp_elicitations: false,
            }
            .allows_mcp_elicitations()
        );
    }

    #[test]
    fn granular_approval_config_skill_approval_flag_is_field_driven() {
        assert!(
            GranularApprovalConfig {
                sandbox_approval: false,
                rules: false,
                skill_approval: true,
                request_permissions: false,
                mcp_elicitations: false,
            }
            .allows_skill_approval()
        );
        assert!(
            !GranularApprovalConfig {
                sandbox_approval: false,
                rules: false,
                skill_approval: false,
                request_permissions: false,
                mcp_elicitations: false,
            }
            .allows_skill_approval()
        );
    }

    #[test]
    fn granular_approval_config_request_permissions_flag_is_field_driven() {
        assert!(
            GranularApprovalConfig {
                sandbox_approval: false,
                rules: false,
                skill_approval: false,
                request_permissions: true,
                mcp_elicitations: false,
            }
            .allows_request_permissions()
        );
        assert!(
            !GranularApprovalConfig {
                sandbox_approval: false,
                rules: false,
                skill_approval: false,
                request_permissions: false,
                mcp_elicitations: false,
            }
            .allows_request_permissions()
        );
    }

    #[test]
    fn granular_approval_config_defaults_missing_optional_flags_to_false() {
        let decoded = serde_json::from_value::<GranularApprovalConfig>(serde_json::json!({
            "sandbox_approval": true,
            "rules": false,
            "mcp_elicitations": true,
        }))
        .expect("granular approval config should deserialize");

        assert_eq!(
            decoded,
            GranularApprovalConfig {
                sandbox_approval: true,
                rules: false,
                skill_approval: false,
                request_permissions: false,
                mcp_elicitations: true,
            }
        );
    }

    #[test]
    fn restricted_file_system_policy_reports_full_access_from_root_entries() {
        let read_only = FileSystemSandboxPolicy::restricted(vec![FileSystemSandboxEntry {
            path: FileSystemPath::Special {
                value: FileSystemSpecialPath::Root,
            },
            access: FileSystemAccessMode::Read,
        }]);
        assert!(read_only.has_full_disk_read_access());
        assert!(!read_only.has_full_disk_write_access());
        assert!(!read_only.include_platform_defaults());

        let writable = FileSystemSandboxPolicy::restricted(vec![FileSystemSandboxEntry {
            path: FileSystemPath::Special {
                value: FileSystemSpecialPath::Root,
            },
            access: FileSystemAccessMode::Write,
        }]);
        assert!(writable.has_full_disk_read_access());
        assert!(writable.has_full_disk_write_access());
    }

    #[test]
    fn restricted_file_system_policy_treats_root_with_carveouts_as_scoped_access() {
        let cwd = TempDir::new().expect("tempdir");
        let canonical_cwd = codex_utils_absolute_path::canonicalize_preserving_symlinks(cwd.path())
            .expect("canonicalize cwd");
        let root = AbsolutePathBuf::from_absolute_path(&canonical_cwd)
            .expect("absolute canonical tempdir")
            .as_path()
            .ancestors()
            .last()
            .and_then(|path| AbsolutePathBuf::from_absolute_path(path).ok())
            .expect("filesystem root");
        let blocked = AbsolutePathBuf::resolve_path_against_base("blocked", cwd.path());
        let expected_blocked = AbsolutePathBuf::from_absolute_path(
            codex_utils_absolute_path::canonicalize_preserving_symlinks(cwd.path())
                .expect("canonicalize cwd")
                .join("blocked"),
        )
        .expect("canonical blocked");
        let policy = FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::Root,
                },
                access: FileSystemAccessMode::Write,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path { path: blocked },
                access: FileSystemAccessMode::None,
            },
        ]);

        assert!(!policy.has_full_disk_read_access());
        assert!(!policy.has_full_disk_write_access());
        assert_eq!(
            policy.get_readable_roots_with_cwd(cwd.path()),
            vec![root.clone()]
        );
        assert_eq!(
            policy.get_unreadable_roots_with_cwd(cwd.path()),
            vec![expected_blocked.clone()]
        );

        let writable_roots = policy.get_writable_roots_with_cwd(cwd.path());
        assert_eq!(writable_roots.len(), 1);
        assert_eq!(writable_roots[0].root, root);
        assert!(
            writable_roots[0]
                .read_only_subpaths
                .iter()
                .any(|path| path.as_path() == expected_blocked.as_path())
        );
    }

    #[test]
    fn restricted_file_system_policy_derives_effective_paths() {
        let cwd = TempDir::new().expect("tempdir");
        std::fs::create_dir_all(cwd.path().join(".agents")).expect("create .agents");
        std::fs::create_dir_all(cwd.path().join(".codex")).expect("create .codex");
        let canonical_cwd = codex_utils_absolute_path::canonicalize_preserving_symlinks(cwd.path())
            .expect("canonicalize cwd");
        let cwd_absolute =
            AbsolutePathBuf::from_absolute_path(&canonical_cwd).expect("absolute tempdir");
        let secret = AbsolutePathBuf::resolve_path_against_base("secret", cwd.path());
        let expected_secret = AbsolutePathBuf::from_absolute_path(canonical_cwd.join("secret"))
            .expect("canonical secret");
        let expected_agents = AbsolutePathBuf::from_absolute_path(canonical_cwd.join(".agents"))
            .expect("canonical .agents");
        let expected_codex = AbsolutePathBuf::from_absolute_path(canonical_cwd.join(".codex"))
            .expect("canonical .codex");
        let policy = FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::Minimal,
                },
                access: FileSystemAccessMode::Read,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::project_roots(/*subpath*/ None),
                },
                access: FileSystemAccessMode::Write,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path { path: secret },
                access: FileSystemAccessMode::None,
            },
        ]);

        assert!(!policy.has_full_disk_read_access());
        assert!(!policy.has_full_disk_write_access());
        assert!(policy.include_platform_defaults());
        assert_eq!(
            policy.get_readable_roots_with_cwd(cwd.path()),
            vec![cwd_absolute.clone()]
        );
        assert_eq!(
            policy.get_unreadable_roots_with_cwd(cwd.path()),
            vec![expected_secret.clone()]
        );

        let writable_roots = policy.get_writable_roots_with_cwd(cwd.path());
        assert_eq!(writable_roots.len(), 1);
        assert_eq!(writable_roots[0].root, cwd_absolute);
        assert!(
            writable_roots[0]
                .read_only_subpaths
                .iter()
                .any(|path| path.as_path() == expected_secret.as_path())
        );
        assert!(
            writable_roots[0]
                .read_only_subpaths
                .iter()
                .any(|path| path.as_path() == expected_agents.as_path())
        );
        assert!(
            writable_roots[0]
                .read_only_subpaths
                .iter()
                .any(|path| path.as_path() == expected_codex.as_path())
        );
    }

    #[test]
    fn restricted_file_system_policy_treats_read_entries_as_read_only_subpaths() {
        let cwd = TempDir::new().expect("tempdir");
        let canonical_cwd = codex_utils_absolute_path::canonicalize_preserving_symlinks(cwd.path())
            .expect("canonicalize cwd");
        let docs = AbsolutePathBuf::resolve_path_against_base("docs", cwd.path());
        let docs_public = AbsolutePathBuf::resolve_path_against_base("docs/public", cwd.path());
        let expected_docs = AbsolutePathBuf::from_absolute_path(canonical_cwd.join("docs"))
            .expect("canonical docs");
        let expected_docs_public =
            AbsolutePathBuf::from_absolute_path(canonical_cwd.join("docs/public"))
                .expect("canonical docs/public");
        let expected_dot_codex = AbsolutePathBuf::from_absolute_path(canonical_cwd.join(".codex"))
            .expect("canonical .codex");
        let policy = FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::project_roots(/*subpath*/ None),
                },
                access: FileSystemAccessMode::Write,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path { path: docs },
                access: FileSystemAccessMode::Read,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path { path: docs_public },
                access: FileSystemAccessMode::Write,
            },
        ]);

        assert!(!policy.has_full_disk_write_access());
        assert_eq!(
            sorted_writable_roots(policy.get_writable_roots_with_cwd(cwd.path())),
            vec![
                (
                    canonical_cwd,
                    vec![
                        expected_dot_codex.to_path_buf(),
                        expected_docs.to_path_buf()
                    ],
                ),
                (expected_docs_public.to_path_buf(), Vec::new()),
            ]
        );
    }

    #[test]
    fn file_system_policy_rejects_legacy_bridge_for_non_workspace_writes() {
        let cwd = if cfg!(windows) {
            Path::new(r"C:\workspace")
        } else {
            Path::new("/tmp/workspace")
        };
        let external_write_path = if cfg!(windows) {
            AbsolutePathBuf::from_absolute_path(r"C:\temp").expect("absolute windows temp path")
        } else {
            AbsolutePathBuf::from_absolute_path("/tmp").expect("absolute tmp path")
        };
        let policy = FileSystemSandboxPolicy::restricted(vec![FileSystemSandboxEntry {
            path: FileSystemPath::Path {
                path: external_write_path,
            },
            access: FileSystemAccessMode::Write,
        }]);

        let err = policy
            .to_legacy_sandbox_policy(NetworkSandboxPolicy::Restricted, cwd)
            .expect_err("non-workspace writes should be rejected");

        assert!(
            err.to_string()
                .contains("filesystem writes outside the workspace root"),
            "{err}"
        );
    }

    #[test]
    fn legacy_sandbox_policy_semantics_survive_split_bridge() {
        let cwd = TempDir::new().expect("tempdir");
        let writable_root = AbsolutePathBuf::resolve_path_against_base("writable", cwd.path());
        let policies = [
            SandboxPolicy::DangerFullAccess,
            SandboxPolicy::ExternalSandbox {
                network_access: NetworkAccess::Restricted,
            },
            SandboxPolicy::ExternalSandbox {
                network_access: NetworkAccess::Enabled,
            },
            SandboxPolicy::ReadOnly {
                network_access: false,
            },
            SandboxPolicy::WorkspaceWrite {
                writable_roots: vec![],
                network_access: false,
                exclude_tmpdir_env_var: true,
                exclude_slash_tmp: true,
            },
            SandboxPolicy::WorkspaceWrite {
                writable_roots: vec![writable_root],
                network_access: true,
                exclude_tmpdir_env_var: false,
                exclude_slash_tmp: true,
            },
        ];

        for expected in policies {
            let actual =
                FileSystemSandboxPolicy::from_legacy_sandbox_policy_for_cwd(&expected, cwd.path())
                    .to_legacy_sandbox_policy(NetworkSandboxPolicy::from(&expected), cwd.path())
                    .expect("legacy bridge should preserve legacy policy semantics");

            assert_same_sandbox_policy_semantics(&expected, &actual, cwd.path());
        }
    }

    #[test]
    fn item_started_event_from_web_search_emits_begin_event() {
        let event = ItemStartedEvent {
            thread_id: ThreadId::new(),
            turn_id: "turn-1".into(),
            item: TurnItem::WebSearch(WebSearchItem {
                id: "search-1".into(),
                query: "find docs".into(),
                action: WebSearchAction::Search {
                    query: Some("find docs".into()),
                    queries: None,
                },
            }),
            started_at_ms: 0,
        };

        let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
        assert_eq!(legacy_events.len(), 1);
        match &legacy_events[0] {
            EventMsg::WebSearchBegin(event) => assert_eq!(event.call_id, "search-1"),
            _ => panic!("expected WebSearchBegin event"),
        }
    }

    #[test]
    fn item_started_event_from_non_web_search_emits_no_legacy_events() {
        let event = ItemStartedEvent {
            thread_id: ThreadId::new(),
            turn_id: "turn-1".into(),
            item: TurnItem::UserMessage(UserMessageItem::new(&[])),
            started_at_ms: 0,
        };

        assert!(
            event
                .as_legacy_events(/*show_raw_agent_reasoning*/ false)
                .is_empty()
        );
    }

    #[test]
    fn item_started_event_from_image_generation_emits_begin_event() {
        let event = ItemStartedEvent {
            thread_id: ThreadId::new(),
            turn_id: "turn-1".into(),
            item: TurnItem::ImageGeneration(ImageGenerationItem {
                id: "ig-1".into(),
                status: "in_progress".into(),
                revised_prompt: None,
                result: String::new(),
                saved_path: None,
            }),
            started_at_ms: 0,
        };

        let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
        assert_eq!(legacy_events.len(), 1);
        match &legacy_events[0] {
            EventMsg::ImageGenerationBegin(event) => assert_eq!(event.call_id, "ig-1"),
            _ => panic!("expected ImageGenerationBegin event"),
        }
    }

    #[test]
    fn item_started_event_from_file_change_emits_patch_begin_event() {
        let event = ItemStartedEvent {
            thread_id: ThreadId::new(),
            turn_id: "turn-1".into(),
            started_at_ms: 0,
            item: TurnItem::FileChange(FileChangeItem {
                id: "patch-1".into(),
                changes: [(
                    PathBuf::from("new.txt"),
                    FileChange::Add {
                        content: "hello".into(),
                    },
                )]
                .into_iter()
                .collect(),
                status: None,
                auto_approved: Some(true),
                stdout: None,
                stderr: None,
            }),
        };

        let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
        assert_eq!(legacy_events.len(), 1);
        match &legacy_events[0] {
            EventMsg::PatchApplyBegin(event) => {
                assert_eq!(event.call_id, "patch-1");
                assert_eq!(event.turn_id, "turn-1");
                assert!(event.auto_approved);
                assert!(event.changes.contains_key(&PathBuf::from("new.txt")));
            }
            _ => panic!("expected PatchApplyBegin event"),
        }
    }

    #[test]
    fn item_started_event_from_mcp_tool_call_emits_begin_event() {
        let event = ItemStartedEvent {
            thread_id: ThreadId::new(),
            turn_id: "turn-1".into(),
            started_at_ms: 0,
            item: TurnItem::McpToolCall(McpToolCallItem {
                id: "mcp-1".into(),
                server: "server".into(),
                tool: "tool".into(),
                arguments: json!({"arg": "value"}),
                mcp_app_resource_uri: Some("app://connector".into()),
                status: McpToolCallStatus::InProgress,
                result: None,
                error: None,
                duration: None,
            }),
        };

        let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
        assert_eq!(legacy_events.len(), 1);
        match &legacy_events[0] {
            EventMsg::McpToolCallBegin(event) => {
                assert_eq!(event.call_id, "mcp-1");
                assert_eq!(event.invocation.server, "server");
                assert_eq!(event.invocation.tool, "tool");
                assert_eq!(
                    event.mcp_app_resource_uri.as_deref(),
                    Some("app://connector")
                );
            }
            _ => panic!("expected McpToolCallBegin event"),
        }
    }

    #[test]
    fn item_completed_event_from_image_generation_emits_end_event() {
        let event = ItemCompletedEvent {
            thread_id: ThreadId::new(),
            turn_id: "turn-1".into(),
            item: TurnItem::ImageGeneration(ImageGenerationItem {
                id: "ig-1".into(),
                status: "completed".into(),
                revised_prompt: Some("A tiny blue square".into()),
                result: "Zm9v".into(),
                saved_path: Some(test_path_buf("/tmp/ig-1.png").abs()),
            }),
            completed_at_ms: 0,
        };

        let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
        assert_eq!(legacy_events.len(), 1);
        match &legacy_events[0] {
            EventMsg::ImageGenerationEnd(event) => {
                assert_eq!(event.call_id, "ig-1");
                assert_eq!(event.status, "completed");
                assert_eq!(event.revised_prompt.as_deref(), Some("A tiny blue square"));
                assert_eq!(event.result, "Zm9v");
                assert_eq!(
                    event.saved_path.as_ref().map(AbsolutePathBuf::as_path),
                    Some(test_path_buf("/tmp/ig-1.png").as_path())
                );
            }
            _ => panic!("expected ImageGenerationEnd event"),
        }
    }

    #[test]
    fn item_completed_event_from_file_change_emits_patch_end_event() {
        let event = ItemCompletedEvent {
            thread_id: ThreadId::new(),
            turn_id: "turn-1".into(),
            completed_at_ms: 0,
            item: TurnItem::FileChange(FileChangeItem {
                id: "patch-1".into(),
                changes: [(
                    PathBuf::from("new.txt"),
                    FileChange::Add {
                        content: "hello".into(),
                    },
                )]
                .into_iter()
                .collect(),
                status: Some(PatchApplyStatus::Completed),
                auto_approved: None,
                stdout: Some("Done!".into()),
                stderr: Some(String::new()),
            }),
        };

        let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
        assert_eq!(legacy_events.len(), 1);
        match &legacy_events[0] {
            EventMsg::PatchApplyEnd(event) => {
                assert_eq!(event.call_id, "patch-1");
                assert_eq!(event.turn_id, "turn-1");
                assert_eq!(event.stdout, "Done!");
                assert!(event.success);
                assert_eq!(event.status, PatchApplyStatus::Completed);
                assert!(event.changes.contains_key(&PathBuf::from("new.txt")));
            }
            _ => panic!("expected PatchApplyEnd event"),
        }
    }

    #[test]
    fn item_completed_event_from_mcp_tool_call_emits_end_event() {
        let event = ItemCompletedEvent {
            thread_id: ThreadId::new(),
            turn_id: "turn-1".into(),
            completed_at_ms: 0,
            item: TurnItem::McpToolCall(McpToolCallItem {
                id: "mcp-1".into(),
                server: "server".into(),
                tool: "tool".into(),
                arguments: json!({"arg": "value"}),
                mcp_app_resource_uri: Some("app://connector".into()),
                status: McpToolCallStatus::Completed,
                result: Some(CallToolResult {
                    content: vec![json!({"type": "text", "text": "ok"})],
                    structured_content: None,
                    is_error: Some(false),
                    meta: None,
                }),
                error: None,
                duration: Some(Duration::from_millis(42)),
            }),
        };

        let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
        assert_eq!(legacy_events.len(), 1);
        match &legacy_events[0] {
            EventMsg::McpToolCallEnd(event) => {
                assert_eq!(event.call_id, "mcp-1");
                assert_eq!(event.invocation.server, "server");
                assert_eq!(event.invocation.tool, "tool");
                assert_eq!(
                    event.mcp_app_resource_uri.as_deref(),
                    Some("app://connector")
                );
                assert_eq!(event.duration, Duration::from_millis(42));
                assert!(event.is_success());
            }
            _ => panic!("expected McpToolCallEnd event"),
        }
    }

    #[test]
    fn item_started_event_requires_started_at_ms() {
        let mut value = serde_json::to_value(ItemStartedEvent {
            thread_id: ThreadId::new(),
            turn_id: "turn-1".into(),
            item: TurnItem::UserMessage(UserMessageItem::new(&[])),
            started_at_ms: 123,
        })
        .unwrap();
        value.as_object_mut().unwrap().remove("started_at_ms");

        assert!(serde_json::from_value::<ItemStartedEvent>(value).is_err());
    }

    #[test]
    fn item_completed_event_defaults_missing_completed_at_ms() {
        let mut value = serde_json::to_value(ItemCompletedEvent {
            thread_id: ThreadId::new(),
            turn_id: "turn-1".into(),
            item: TurnItem::UserMessage(UserMessageItem::new(&[])),
            completed_at_ms: 123,
        })
        .unwrap();
        value.as_object_mut().unwrap().remove("completed_at_ms");

        let event = serde_json::from_value::<ItemCompletedEvent>(value).unwrap();
        assert_eq!(event.completed_at_ms, 0);
    }
    #[test]
    fn rollback_failed_error_does_not_affect_turn_status() {
        let event = ErrorEvent {
            message: "rollback failed".into(),
            codex_error_info: Some(CodexErrorInfo::ThreadRollbackFailed),
        };
        assert!(!event.affects_turn_status());
    }

    #[test]
    fn active_turn_not_steerable_error_does_not_affect_turn_status() {
        let event = ErrorEvent {
            message: "cannot steer a review turn".into(),
            codex_error_info: Some(CodexErrorInfo::ActiveTurnNotSteerable {
                turn_kind: NonSteerableTurnKind::Review,
            }),
        };
        assert!(!event.affects_turn_status());
    }

    #[test]
    fn generic_error_affects_turn_status() {
        let event = ErrorEvent {
            message: "generic".into(),
            codex_error_info: Some(CodexErrorInfo::Other),
        };
        assert!(event.affects_turn_status());
    }

    #[test]
    fn conversation_op_serializes_as_unnested_variants() {
        let audio = Op::RealtimeConversationAudio(ConversationAudioParams {
            frame: RealtimeAudioFrame {
                data: "AQID".to_string(),
                sample_rate: 24_000,
                num_channels: 1,
                samples_per_channel: Some(480),
                item_id: None,
            },
        });
        let start = Op::RealtimeConversationStart(ConversationStartParams {
            output_modality: RealtimeOutputModality::Audio,
            prompt: Some(Some("be helpful".to_string())),
            realtime_session_id: Some("conv_1".to_string()),
            transport: None,
            voice: None,
        });
        let webrtc_start = Op::RealtimeConversationStart(ConversationStartParams {
            output_modality: RealtimeOutputModality::Audio,
            prompt: Some(Some("be helpful".to_string())),
            realtime_session_id: Some("conv_1".to_string()),
            transport: Some(ConversationStartTransport::Webrtc {
                sdp: "v=offer\r\n".to_string(),
            }),
            voice: Some(RealtimeVoice::Cove),
        });
        let text = Op::RealtimeConversationText(ConversationTextParams {
            text: "hello".to_string(),
        });
        let close = Op::RealtimeConversationClose;
        let default_prompt_start = Op::RealtimeConversationStart(ConversationStartParams {
            output_modality: RealtimeOutputModality::Audio,
            prompt: None,
            realtime_session_id: None,
            transport: None,
            voice: None,
        });
        let null_prompt_start = Op::RealtimeConversationStart(ConversationStartParams {
            output_modality: RealtimeOutputModality::Audio,
            prompt: Some(None),
            realtime_session_id: None,
            transport: None,
            voice: None,
        });
        let list_voices = Op::RealtimeConversationListVoices;

        assert_eq!(
            serde_json::to_value(&start).unwrap(),
            json!({
                "type": "realtime_conversation_start",
                "output_modality": "audio",
                "prompt": "be helpful",
                "realtime_session_id": "conv_1"
            })
        );
        assert_eq!(
            serde_json::to_value(&default_prompt_start).unwrap(),
            json!({
                "type": "realtime_conversation_start",
                "output_modality": "audio"
            })
        );
        assert_eq!(
            serde_json::to_value(&null_prompt_start).unwrap(),
            json!({
                "type": "realtime_conversation_start",
                "output_modality": "audio",
                "prompt": null
            })
        );
        assert_eq!(
            serde_json::from_value::<Op>(json!({
                "type": "realtime_conversation_start",
                "output_modality": "audio"
            }))
            .unwrap(),
            default_prompt_start
        );
        assert_eq!(
            serde_json::from_value::<Op>(json!({
                "type": "realtime_conversation_start",
                "output_modality": "audio",
                "prompt": null
            }))
            .unwrap(),
            null_prompt_start
        );
        assert_eq!(
            serde_json::to_value(&audio).unwrap(),
            json!({
                "type": "realtime_conversation_audio",
                "frame": {
                    "data": "AQID",
                    "sample_rate": 24000,
                    "num_channels": 1,
                    "samples_per_channel": 480
                }
            })
        );
        assert_eq!(
            serde_json::from_value::<Op>(serde_json::to_value(&text).unwrap()).unwrap(),
            text
        );
        assert_eq!(
            serde_json::to_value(&close).unwrap(),
            json!({
                "type": "realtime_conversation_close"
            })
        );
        assert_eq!(
            serde_json::from_value::<Op>(serde_json::to_value(&close).unwrap()).unwrap(),
            close
        );
        assert_eq!(
            serde_json::to_value(&list_voices).unwrap(),
            json!({
                "type": "realtime_conversation_list_voices"
            })
        );
        assert_eq!(
            serde_json::from_value::<Op>(serde_json::to_value(&list_voices).unwrap()).unwrap(),
            list_voices
        );
        assert_eq!(
            serde_json::to_value(&webrtc_start).unwrap(),
            json!({
                "type": "realtime_conversation_start",
                "output_modality": "audio",
                "prompt": "be helpful",
                "realtime_session_id": "conv_1",
                "transport": {
                    "type": "webrtc",
                    "sdp": "v=offer\r\n"
                },
                "voice": "cove"
            })
        );
    }

    #[test]
    fn realtime_conversation_started_event_uses_realtime_session_id() {
        let event = RealtimeConversationStartedEvent {
            realtime_session_id: Some("conv_1".to_string()),
            version: RealtimeConversationVersion::V2,
        };

        assert_eq!(
            serde_json::to_value(&event).unwrap(),
            json!({
                "realtime_session_id": "conv_1",
                "version": "v2"
            })
        );
    }

    #[test]
    fn realtime_voice_list_is_stable() {
        assert_eq!(
            RealtimeVoicesList::builtin(),
            RealtimeVoicesList {
                v1: vec![
                    RealtimeVoice::Juniper,
                    RealtimeVoice::Maple,
                    RealtimeVoice::Spruce,
                    RealtimeVoice::Ember,
                    RealtimeVoice::Vale,
                    RealtimeVoice::Breeze,
                    RealtimeVoice::Arbor,
                    RealtimeVoice::Sol,
                    RealtimeVoice::Cove,
                ],
                v2: vec![
                    RealtimeVoice::Alloy,
                    RealtimeVoice::Ash,
                    RealtimeVoice::Ballad,
                    RealtimeVoice::Coral,
                    RealtimeVoice::Echo,
                    RealtimeVoice::Sage,
                    RealtimeVoice::Shimmer,
                    RealtimeVoice::Verse,
                    RealtimeVoice::Marin,
                    RealtimeVoice::Cedar,
                ],
                default_v1: RealtimeVoice::Cove,
                default_v2: RealtimeVoice::Marin,
            }
        );
    }

    #[test]
    fn user_input_serialization_omits_final_output_json_schema_when_none() -> Result<()> {
        let op = Op::UserInput {
            environments: None,
            items: Vec::new(),
            final_output_json_schema: None,
            responsesapi_client_metadata: None,
        };

        let json_op = serde_json::to_value(op)?;
        assert_eq!(json_op, json!({ "type": "user_input", "items": [] }));

        Ok(())
    }

    #[test]
    fn user_input_deserializes_without_final_output_json_schema_field() -> Result<()> {
        let op: Op = serde_json::from_value(json!({ "type": "user_input", "items": [] }))?;

        assert_eq!(
            op,
            Op::UserInput {
                environments: None,
                items: Vec::new(),
                final_output_json_schema: None,
                responsesapi_client_metadata: None,
            }
        );

        Ok(())
    }

    #[test]
    fn user_input_serialization_includes_final_output_json_schema_when_some() -> Result<()> {
        let schema = json!({
            "type": "object",
            "properties": {
                "answer": { "type": "string" }
            },
            "required": ["answer"],
            "additionalProperties": false
        });
        let op = Op::UserInput {
            environments: None,
            items: Vec::new(),
            final_output_json_schema: Some(schema.clone()),
            responsesapi_client_metadata: None,
        };

        let json_op = serde_json::to_value(op)?;
        assert_eq!(
            json_op,
            json!({
                "type": "user_input",
                "items": [],
                "final_output_json_schema": schema,
            })
        );

        Ok(())
    }

    #[test]
    fn user_input_with_responsesapi_client_metadata_round_trips() -> Result<()> {
        let op = Op::UserInput {
            environments: None,
            items: Vec::new(),
            final_output_json_schema: None,
            responsesapi_client_metadata: Some(HashMap::from([(
                "fiber_run_id".to_string(),
                "fiber-123".to_string(),
            )])),
        };

        let json_op = serde_json::to_value(&op)?;
        assert_eq!(
            json_op,
            json!({
                "type": "user_input",
                "items": [],
                "responsesapi_client_metadata": {
                    "fiber_run_id": "fiber-123",
                }
            })
        );
        assert_eq!(serde_json::from_value::<Op>(json_op)?, op);

        Ok(())
    }

    #[test]
    fn user_input_with_turn_context_deserializes_without_selected_snippet_handoff() -> Result<()> {
        let op: Op = serde_json::from_value(json!({
            "type": "user_input_with_turn_context",
            "items": []
        }))?;

        let Op::UserInputWithTurnContext {
            context_recall_selected_snippets,
            ..
        } = op
        else {
            panic!("expected user_input_with_turn_context");
        };
        assert_eq!(context_recall_selected_snippets, None);

        Ok(())
    }

    #[test]
    fn user_input_with_turn_context_serializes_selected_snippet_handoff() -> Result<()> {
        let selected_snippets = test_selected_snippet_envelope();
        let op = Op::UserInputWithTurnContext {
            environments: None,
            items: Vec::new(),
            final_output_json_schema: None,
            responsesapi_client_metadata: None,
            context_recall_selected_snippets: Some(selected_snippets.clone()),
            cwd: None,
            workspace_roots: None,
            profile_workspace_roots: None,
            approval_policy: None,
            approvals_reviewer: None,
            sandbox_policy: None,
            permission_profile: None,
            active_permission_profile: None,
            windows_sandbox_level: None,
            model: None,
            effort: None,
            summary: None,
            service_tier: None,
            collaboration_mode: None,
            personality: None,
        };

        let json_op = serde_json::to_value(&op)?;
        assert_eq!(
            json_op["context_recall_selected_snippets"]["selected_snippet_count"],
            1
        );
        assert_eq!(
            json_op["context_recall_selected_snippets"]["snippets"][0]["text"],
            "[redacted-query] bounded memory"
        );
        assert!(
            json_op["context_recall_selected_snippets"]["snippets"][0]
                .get("source_id")
                .is_none()
        );
        assert_eq!(serde_json::from_value::<Op>(json_op)?, op);
        assert!(selected_snippets.has_shadow_integrity());

        Ok(())
    }

    #[test]
    fn user_input_text_serializes_empty_text_elements() -> Result<()> {
        let input = UserInput::Text {
            text: "hello".to_string(),
            text_elements: Vec::new(),
        };

        let json_input = serde_json::to_value(input)?;
        assert_eq!(
            json_input,
            json!({
                "type": "text",
                "text": "hello",
                "text_elements": [],
            })
        );

        Ok(())
    }

    #[test]
    fn user_message_event_serializes_empty_metadata_vectors() -> Result<()> {
        let event = UserMessageEvent {
            message: "hello".to_string(),
            images: None,
            local_images: Vec::new(),
            text_elements: Vec::new(),
            ..Default::default()
        };

        let json_event = serde_json::to_value(event)?;
        assert_eq!(
            json_event,
            json!({
                "message": "hello",
                "local_images": [],
                "text_elements": [],
            })
        );

        Ok(())
    }

    #[test]
    fn user_message_event_deserializes_without_image_detail_fields() -> Result<()> {
        let event: UserMessageEvent = serde_json::from_value(json!({
            "message": "hello",
            "images": ["https://example.com/image.png"],
            "local_images": ["/tmp/local.png"],
            "text_elements": [],
        }))?;

        assert_eq!(event.message, "hello");
        assert_eq!(
            event.images,
            Some(vec!["https://example.com/image.png".to_string()])
        );
        assert_eq!(event.image_details, Vec::<Option<ImageDetail>>::new());
        assert_eq!(event.local_images, vec![PathBuf::from("/tmp/local.png")]);
        assert_eq!(event.local_image_details, Vec::<Option<ImageDetail>>::new());
        assert_eq!(event.text_elements, Vec::new());

        Ok(())
    }

    #[test]
    fn user_message_item_legacy_event_preserves_image_details() {
        let local_path = PathBuf::from("/tmp/local.png");
        let item = UserMessageItem::new(&[
            crate::user_input::UserInput::Image {
                image_url: "https://example.com/first.png".to_string(),
                detail: Some(ImageDetail::Original),
            },
            crate::user_input::UserInput::Image {
                image_url: "https://example.com/second.png".to_string(),
                detail: None,
            },
            crate::user_input::UserInput::LocalImage {
                path: local_path.clone(),
                detail: Some(ImageDetail::Original),
            },
        ]);

        let EventMsg::UserMessage(event) = item.as_legacy_event() else {
            panic!("expected user message event");
        };

        assert_eq!(
            event.images,
            Some(vec![
                "https://example.com/first.png".to_string(),
                "https://example.com/second.png".to_string(),
            ])
        );
        assert_eq!(event.image_details, vec![Some(ImageDetail::Original)]);
        assert_eq!(event.local_images, vec![local_path]);
        assert_eq!(event.local_image_details, vec![Some(ImageDetail::Original)]);
    }

    #[test]
    fn turn_aborted_event_deserializes_without_turn_id() -> Result<()> {
        let event: EventMsg = serde_json::from_value(json!({
            "type": "turn_aborted",
            "reason": "interrupted",
        }))?;

        match event {
            EventMsg::TurnAborted(TurnAbortedEvent {
                turn_id, reason, ..
            }) => {
                assert_eq!(turn_id, None);
                assert_eq!(reason, TurnAbortReason::Interrupted);
            }
            _ => panic!("expected turn_aborted event"),
        }

        Ok(())
    }

    #[test]
    fn turn_context_item_deserializes_without_network() -> Result<()> {
        let item: TurnContextItem = serde_json::from_value(json!({
            "cwd": test_path_buf("/tmp"),
            "approval_policy": "never",
            "sandbox_policy": { "type": "danger-full-access" },
            "model": "gpt-5",
            "summary": "auto",
        }))?;

        assert_eq!(item.trace_id, None);
        assert_eq!(item.network, None);
        assert_eq!(item.file_system_sandbox_policy, None);
        assert_eq!(item.context_manifest, None);
        Ok(())
    }

    #[test]
    fn turn_context_item_serializes_network_when_present() -> Result<()> {
        let item = TurnContextItem {
            turn_id: None,
            trace_id: None,
            cwd: test_path_buf("/tmp"),
            current_date: None,
            timezone: None,
            approval_policy: AskForApproval::Never,
            sandbox_policy: SandboxPolicy::DangerFullAccess,
            permission_profile: None,
            network: Some(TurnContextNetworkItem {
                allowed_domains: vec!["api.example.com".to_string()],
                denied_domains: vec!["blocked.example.com".to_string()],
            }),
            file_system_sandbox_policy: Some(FileSystemSandboxPolicy::restricted(vec![
                FileSystemSandboxEntry {
                    path: FileSystemPath::GlobPattern {
                        pattern: "/tmp/private/**/*.txt".to_string(),
                    },
                    access: FileSystemAccessMode::None,
                },
            ])),
            model: "gpt-5".to_string(),
            personality: None,
            collaboration_mode: None,
            realtime_active: None,
            effort: None,
            summary: ReasoningSummaryConfig::Auto,
            user_instructions: None,
            developer_instructions: None,
            final_output_json_schema: None,
            truncation_policy: None,
            context_manifest: None,
        };

        let value = serde_json::to_value(item)?;
        assert_eq!(
            value["network"],
            json!({
                "allowed_domains": ["api.example.com"],
                "denied_domains": ["blocked.example.com"],
            })
        );
        assert_eq!(
            value["file_system_sandbox_policy"],
            json!({
                "kind": "restricted",
                "entries": [{
                    "path": {
                        "type": "glob_pattern",
                        "pattern": "/tmp/private/**/*.txt"
                    },
                    "access": "none"
                }]
            })
        );
        Ok(())
    }

    #[test]
    fn turn_context_manifest_entry_tier_is_backward_compatible() -> Result<()> {
        let legacy_entry: TurnContextManifestEntry = serde_json::from_value(json!({
            "role": "developer",
            "source": "initial_context:permissions:0",
            "replay_key": "initial_context:permissions:0:0123456789abcdef",
            "text_hash": "0123456789abcdef",
            "estimated_tokens": 3,
        }))?;
        assert_eq!(legacy_entry.tier, TurnContextTier::Unknown);
        assert!(serde_json::to_value(&legacy_entry)?.get("tier").is_none());

        let mut legacy_manifest = TurnContextManifestItem {
            version: TURN_CONTEXT_MANIFEST_VERSION,
            estimated_tokens: 3,
            ledger_hash: None,
            budget_tokens: None,
            omitted_entries: 0,
            omitted_sources: Vec::new(),
            truncated: false,
            decision_ledger_hash: None,
            decision_ledger: Vec::new(),
            recall_selection: None,
            recall_selected_snippets: None,
            memory_taxonomy: Vec::new(),
            memory_formation_receipts: Vec::new(),
            memory_temporal_facts: Vec::new(),
            compression_candidates: Vec::new(),
            adaptive_budget_allocations: Vec::new(),
            compression_stages: Vec::new(),
            entries: vec![legacy_entry],
        }
        .with_refreshed_ledger_hash();
        assert!(legacy_manifest.has_replay_integrity());
        let legacy_hash = legacy_manifest
            .ledger_hash
            .clone()
            .expect("legacy hash should be materialized");

        legacy_manifest.entries[0].tier = TurnContextTier::System;
        legacy_manifest.refresh_ledger_hash();
        assert!(legacy_manifest.has_replay_integrity());
        assert_ne!(
            legacy_manifest.ledger_hash.as_deref(),
            Some(legacy_hash.as_str())
        );
        assert_eq!(
            serde_json::to_value(&legacy_manifest.entries[0])?["tier"],
            "system"
        );
        assert!(
            serde_json::to_value(&legacy_manifest)?
                .get("compression_candidates")
                .is_none()
        );
        assert!(
            serde_json::to_value(&legacy_manifest)?
                .get("compression_stages")
                .is_none()
        );

        Ok(())
    }

    #[test]
    fn turn_context_manifest_hashes_are_replay_hashes_not_trust_digests() {
        let replay_hash = stable_turn_context_manifest_replay_hash("text:payload-light\n");
        assert_eq!(replay_hash.len(), 16);
        assert!(is_stable_manifest_replay_hash(&replay_hash));
        assert_eq!(
            stable_turn_context_manifest_text_hash("text:payload-light\n"),
            replay_hash
        );

        let sha256_shaped_digest =
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        assert_eq!(sha256_shaped_digest.len(), 64);
        assert!(!is_stable_manifest_replay_hash(sha256_shaped_digest));
    }

    #[test]
    fn turn_context_decision_entry_constructors_preserve_legacy_wire_strings() {
        let entries = vec![
            TurnContextDecisionEntry::included(
                "turn_context:developer:permissions:0",
                "always_include_safety_policy",
                Some("aaaaaaaaaaaaaaaa".to_string()),
            ),
            TurnContextDecisionEntry::policy(
                "turn_context:assembly_policy",
                "source_aware_omission",
                "budget_exceeded",
                None,
            ),
            TurnContextDecisionEntry::candidate_omit(
                "turn_context:developer:available_plugins:0:2",
                "available_plugins",
                20,
                11,
                None,
            ),
            TurnContextDecisionEntry::candidate_truncate(
                "turn_context:developer:selected_context_recall:0",
                "selected_context_recall",
                4,
                13,
                None,
            ),
            TurnContextDecisionEntry::omitted(
                "turn_context:developer:apps:0:3",
                "apps",
                30,
                9,
                None,
            ),
            TurnContextDecisionEntry::truncated(
                "turn_context:developer:selected_context_recall:0",
                "selected_context_recall",
                24,
                3,
                None,
            ),
        ];
        let expected = [
            "included:always_include_safety_policy",
            "policy:source_aware_omission:budget_exceeded",
            "candidate_omit:available_plugins:priority:20:tokens:11",
            "candidate_truncate:selected_context_recall:remaining_over_budget:4:tokens:13",
            "omitted:apps:priority:30:tokens:9",
            "truncated:selected_context_recall:original_tokens:24:tokens:3",
        ];

        for (entry, expected_decision) in entries.iter().zip(expected) {
            assert_eq!(entry.decision, expected_decision);
            let kind = entry.kind();
            assert_eq!(
                kind.schema_version(),
                Some(TURN_CONTEXT_DECISION_SCHEMA_VERSION)
            );
            assert_eq!(kind.to_legacy_decision_string(), expected_decision);
        }
        assert_eq!(entries[0].reason_hash.as_deref(), Some("aaaaaaaaaaaaaaaa"));
        assert!(entries[3].kind().is_candidate_truncation());

        let summary = summarize_turn_context_decision_ledger(&entries);
        assert_eq!(summary.schema_version, TURN_CONTEXT_DECISION_SCHEMA_VERSION);
        assert_eq!(summary.known_count(), 6);
        assert_eq!(summary.included_count, 1);
        assert_eq!(summary.policy_count, 1);
        assert_eq!(summary.candidate_omit_count, 1);
        assert_eq!(summary.candidate_truncate_count, 1);
        assert_eq!(summary.omitted_count, 1);
        assert_eq!(summary.truncated_count, 1);
        assert_eq!(summary.unknown_count, 0);

        let unknown = TurnContextDecisionKind::Unknown {
            raw: "legacy:custom".to_string(),
        };
        assert_eq!(unknown.schema_version(), None);
        assert_eq!(unknown.to_legacy_decision_string(), "legacy:custom");

        let unknown_entry =
            TurnContextDecisionEntry::from_kind("turn_context:legacy", unknown, None);
        let mixed_summary =
            summarize_turn_context_decision_ledger(&[entries[0].clone(), unknown_entry.clone()]);
        assert_eq!(
            mixed_summary.schema_version,
            TURN_CONTEXT_DECISION_SCHEMA_VERSION
        );
        assert_eq!(mixed_summary.known_count(), 1);
        assert_eq!(mixed_summary.unknown_count, 1);

        let unknown_only_summary = summarize_turn_context_decision_ledger(&[unknown_entry]);
        assert_eq!(unknown_only_summary.schema_version, 0);
        assert_eq!(unknown_only_summary.known_count(), 0);
        assert_eq!(unknown_only_summary.unknown_count, 1);
    }

    #[test]
    fn turn_context_manifest_compression_candidates_are_payload_light_and_hashed() -> Result<()> {
        let mut manifest = TurnContextManifestItem {
            version: TURN_CONTEXT_MANIFEST_VERSION,
            estimated_tokens: 12,
            ledger_hash: None,
            budget_tokens: Some(8),
            omitted_entries: 0,
            omitted_sources: Vec::new(),
            truncated: false,
            decision_ledger_hash: None,
            decision_ledger: Vec::new(),
            recall_selection: None,
            recall_selected_snippets: None,
            memory_taxonomy: Vec::new(),
            memory_formation_receipts: Vec::new(),
            memory_temporal_facts: Vec::new(),
            compression_candidates: vec![
                TurnContextCompressionCandidate {
                    kind: TurnContextCompressionStageKind::Summary,
                    tier: TurnContextTier::RetrievedSnippets,
                    source_id: "selected_context_recall".into(),
                    input_tokens: 40,
                    estimated_output_tokens: 12,
                    affected_entries: 1,
                    not_executed_reason:
                        TurnContextCompressionCandidateReason::BudgetPressureDryRun,
                },
                TurnContextCompressionCandidate {
                    kind: TurnContextCompressionStageKind::Prune,
                    tier: TurnContextTier::Tool,
                    source_id: "extension_developer_capabilities".into(),
                    input_tokens: 12,
                    estimated_output_tokens: 6,
                    affected_entries: 1,
                    not_executed_reason:
                        TurnContextCompressionCandidateReason::BudgetPressureDryRun,
                },
            ],
            adaptive_budget_allocations: Vec::new(),
            compression_stages: Vec::new(),
            entries: vec![TurnContextManifestEntry {
                role: "developer".into(),
                tier: TurnContextTier::RetrievedSnippets,
                source: "turn_context:developer:selected_context_recall:0".into(),
                replay_key: "turn_context:developer:selected_context_recall:0:0123456789abcdef"
                    .into(),
                text_hash: "0123456789abcdef".into(),
                estimated_tokens: 12,
            }],
        }
        .with_refreshed_ledger_hash();

        let value = serde_json::to_value(&manifest)?;

        assert_eq!(
            manifest.compression_candidates[0].schema_version(),
            Some(TURN_CONTEXT_COMPRESSION_CANDIDATE_SCHEMA_VERSION)
        );
        assert_eq!(value["compression_candidates"][0]["kind"], "summary");
        assert_eq!(
            value["compression_candidates"][0]["tier"],
            "retrieved_snippets"
        );
        assert_eq!(
            value["compression_candidates"][0]["source_id"],
            "selected_context_recall"
        );
        assert_eq!(value["compression_candidates"][0]["input_tokens"], 40);
        assert_eq!(
            value["compression_candidates"][0]["estimated_output_tokens"],
            12
        );
        assert_eq!(
            value["compression_candidates"][0]["not_executed_reason"],
            "budget_pressure_dry_run"
        );
        assert!(value["compression_candidates"][0].get("source").is_none());
        assert!(value["compression_candidates"][0].get("text").is_none());
        assert!(value["compression_candidates"][0].get("query").is_none());
        assert!(manifest.compression_candidates_have_integrity());
        assert!(manifest.has_replay_integrity());

        let original_ledger_hash = manifest
            .ledger_hash
            .clone()
            .expect("ledger hash should be materialized");
        manifest.compression_candidates[0].estimated_output_tokens = 11;
        assert!(!manifest.ledger_hash_matches_manifest());
        manifest.refresh_ledger_hash();
        assert_ne!(
            manifest.ledger_hash.as_deref(),
            Some(original_ledger_hash.as_str())
        );
        assert!(manifest.has_replay_integrity());

        manifest.compression_candidates[0].estimated_output_tokens = 41;
        manifest.refresh_ledger_hash();
        assert!(!manifest.compression_candidates_have_integrity());
        assert!(!manifest.has_replay_integrity());

        manifest.compression_candidates[0].estimated_output_tokens = 11;
        manifest.compression_candidates[1].source_id = "turn_context:developer:raw:0".into();
        manifest.refresh_ledger_hash();
        assert!(!manifest.compression_candidates_have_integrity());
        assert!(!manifest.has_replay_integrity());

        manifest.compression_candidates[1].source_id = "extension_developer_capabilities".into();
        manifest.compression_candidates[1].not_executed_reason =
            TurnContextCompressionCandidateReason::Unknown;
        manifest.refresh_ledger_hash();
        assert!(!manifest.compression_candidates_have_integrity());
        assert!(!manifest.has_replay_integrity());

        Ok(())
    }

    #[test]
    fn turn_context_manifest_adaptive_budget_allocations_are_payload_light_and_hashed() -> Result<()>
    {
        let mut manifest = TurnContextManifestItem {
            version: TURN_CONTEXT_MANIFEST_VERSION,
            estimated_tokens: 52,
            ledger_hash: None,
            budget_tokens: Some(24),
            omitted_entries: 0,
            omitted_sources: Vec::new(),
            truncated: false,
            decision_ledger_hash: None,
            decision_ledger: Vec::new(),
            recall_selection: None,
            recall_selected_snippets: None,
            memory_taxonomy: Vec::new(),
            memory_formation_receipts: Vec::new(),
            memory_temporal_facts: Vec::new(),
            compression_candidates: Vec::new(),
            adaptive_budget_allocations: vec![
                TurnContextAdaptiveBudgetAllocation {
                    tier: TurnContextTier::RetrievedSnippets,
                    source_id: "selected_context_recall".into(),
                    budget_class: "bounded_recall".into(),
                    input_tokens: 40,
                    reserve_tokens: 16,
                    proposed_budget_tokens: 16,
                    overflow_tokens: 24,
                    omit_priority: Some(50),
                    compression_kind: Some(TurnContextCompressionStageKind::Summary),
                    estimated_compressed_tokens: Some(16),
                    current_heuristic_action: TurnContextBudgetAllocationAction::Drop,
                    proposed_action: TurnContextBudgetAllocationAction::Compress,
                    would_drop: false,
                    would_compress: true,
                },
                TurnContextAdaptiveBudgetAllocation {
                    tier: TurnContextTier::Tool,
                    source_id: "available_plugins".into(),
                    budget_class: "tool_inventory".into(),
                    input_tokens: 12,
                    reserve_tokens: 9,
                    proposed_budget_tokens: 8,
                    overflow_tokens: 4,
                    omit_priority: Some(20),
                    compression_kind: Some(TurnContextCompressionStageKind::Defragment),
                    estimated_compressed_tokens: Some(9),
                    current_heuristic_action: TurnContextBudgetAllocationAction::Drop,
                    proposed_action: TurnContextBudgetAllocationAction::Compress,
                    would_drop: false,
                    would_compress: true,
                },
            ],
            compression_stages: Vec::new(),
            entries: vec![TurnContextManifestEntry {
                role: "developer".into(),
                tier: TurnContextTier::RetrievedSnippets,
                source: "turn_context:developer:selected_context_recall:0".into(),
                replay_key: "turn_context:developer:selected_context_recall:0:0123456789abcdef"
                    .into(),
                text_hash: "0123456789abcdef".into(),
                estimated_tokens: 40,
            }],
        }
        .with_refreshed_ledger_hash();

        let value = serde_json::to_value(&manifest)?;

        assert_eq!(
            manifest.adaptive_budget_allocations[0].schema_version(),
            Some(TURN_CONTEXT_ADAPTIVE_BUDGET_ALLOCATION_SCHEMA_VERSION)
        );
        assert_eq!(
            value["adaptive_budget_allocations"][0]["source_id"],
            "selected_context_recall"
        );
        assert_eq!(
            value["adaptive_budget_allocations"][0]["budget_class"],
            "bounded_recall"
        );
        assert_eq!(
            value["adaptive_budget_allocations"][0]["compression_kind"],
            "summary"
        );
        assert_eq!(
            value["adaptive_budget_allocations"][0]["proposed_action"],
            "compress"
        );
        assert_eq!(
            value["adaptive_budget_allocations"][0]["would_compress"],
            true
        );
        assert!(
            value["adaptive_budget_allocations"][0]
                .get("source")
                .is_none()
        );
        assert!(
            value["adaptive_budget_allocations"][0]
                .get("text")
                .is_none()
        );
        assert!(
            value["adaptive_budget_allocations"][0]
                .get("query")
                .is_none()
        );
        assert!(manifest.adaptive_budget_allocations_have_integrity());
        assert!(manifest.has_replay_integrity());

        let original_ledger_hash = manifest
            .ledger_hash
            .clone()
            .expect("ledger hash should be materialized");
        manifest.adaptive_budget_allocations[0].proposed_budget_tokens = 15;
        assert!(!manifest.ledger_hash_matches_manifest());
        manifest.adaptive_budget_allocations[0].overflow_tokens = 25;
        manifest.refresh_ledger_hash();
        assert_ne!(
            manifest.ledger_hash.as_deref(),
            Some(original_ledger_hash.as_str())
        );
        assert!(manifest.has_replay_integrity());

        manifest.adaptive_budget_allocations[0].overflow_tokens = 24;
        manifest.refresh_ledger_hash();
        assert!(!manifest.adaptive_budget_allocations_have_integrity());
        assert!(!manifest.has_replay_integrity());

        manifest.adaptive_budget_allocations[0].overflow_tokens = 25;
        manifest.adaptive_budget_allocations[1].source_id = "turn_context:developer:raw:0".into();
        manifest.refresh_ledger_hash();
        assert!(!manifest.adaptive_budget_allocations_have_integrity());
        assert!(!manifest.has_replay_integrity());

        Ok(())
    }

    #[test]
    fn turn_context_manifest_memory_taxonomy_is_payload_light_and_hashed() -> Result<()> {
        let mut manifest = TurnContextManifestItem {
            version: TURN_CONTEXT_MANIFEST_VERSION,
            estimated_tokens: 34,
            ledger_hash: None,
            budget_tokens: Some(24),
            omitted_entries: 0,
            omitted_sources: Vec::new(),
            truncated: false,
            decision_ledger_hash: None,
            decision_ledger: Vec::new(),
            recall_selection: None,
            recall_selected_snippets: None,
            memory_taxonomy: vec![
                TurnContextMemoryTaxonomyBucket {
                    class: TurnContextMemoryTaxonomyClass::Semantic,
                    source_count: 1,
                    returned_count: 2,
                    available_count: 3,
                    omitted_count: 1,
                    provenance_span_count: 0,
                },
                TurnContextMemoryTaxonomyBucket {
                    class: TurnContextMemoryTaxonomyClass::Episodic,
                    source_count: 1,
                    returned_count: 1,
                    available_count: 1,
                    omitted_count: 0,
                    provenance_span_count: 0,
                },
                TurnContextMemoryTaxonomyBucket {
                    class: TurnContextMemoryTaxonomyClass::Control,
                    source_count: 1,
                    returned_count: 0,
                    available_count: 2,
                    omitted_count: 2,
                    provenance_span_count: 0,
                },
                TurnContextMemoryTaxonomyBucket {
                    class: TurnContextMemoryTaxonomyClass::Transcript,
                    source_count: 2,
                    returned_count: 3,
                    available_count: 5,
                    omitted_count: 2,
                    provenance_span_count: 2,
                },
            ],
            memory_formation_receipts: Vec::new(),
            memory_temporal_facts: Vec::new(),
            compression_candidates: Vec::new(),
            adaptive_budget_allocations: Vec::new(),
            compression_stages: Vec::new(),
            entries: vec![TurnContextManifestEntry {
                role: "developer".into(),
                tier: TurnContextTier::RetrievedSnippets,
                source: "turn_context:developer:selected_context_recall:0".into(),
                replay_key: "turn_context:developer:selected_context_recall:0:0123456789abcdef"
                    .into(),
                text_hash: "0123456789abcdef".into(),
                estimated_tokens: 34,
            }],
        }
        .with_refreshed_ledger_hash();

        let value = serde_json::to_value(&manifest)?;

        assert_eq!(
            manifest.memory_taxonomy[0].schema_version(),
            Some(TURN_CONTEXT_MEMORY_TAXONOMY_SCHEMA_VERSION)
        );
        assert_eq!(value["memory_taxonomy"][0]["class"], "semantic");
        assert_eq!(value["memory_taxonomy"][0]["source_count"], 1);
        assert_eq!(value["memory_taxonomy"][0]["returned_count"], 2);
        assert_eq!(value["memory_taxonomy"][0]["available_count"], 3);
        assert_eq!(value["memory_taxonomy"][0]["omitted_count"], 1);
        assert_eq!(value["memory_taxonomy"][3]["class"], "transcript");
        assert_eq!(value["memory_taxonomy"][3]["provenance_span_count"], 2);
        assert!(value["memory_taxonomy"][0].get("source_id").is_none());
        assert!(value["memory_taxonomy"][0].get("memory_id").is_none());
        assert!(value["memory_taxonomy"][0].get("text").is_none());
        assert!(value["memory_taxonomy"][0].get("query").is_none());
        assert!(manifest.memory_taxonomy_has_integrity());
        assert!(manifest.has_replay_integrity());

        let original_ledger_hash = manifest
            .ledger_hash
            .clone()
            .expect("ledger hash should be materialized");
        manifest.memory_taxonomy[0].returned_count = 1;
        assert!(!manifest.ledger_hash_matches_manifest());
        manifest.memory_taxonomy[0].omitted_count = 2;
        manifest.refresh_ledger_hash();
        assert_ne!(
            manifest.ledger_hash.as_deref(),
            Some(original_ledger_hash.as_str())
        );
        assert!(manifest.has_replay_integrity());

        manifest.memory_taxonomy[0].omitted_count = 1;
        manifest.refresh_ledger_hash();
        assert!(!manifest.memory_taxonomy_has_integrity());
        assert!(!manifest.has_replay_integrity());

        manifest.memory_taxonomy[0].omitted_count = 2;
        manifest.memory_taxonomy[1].class = TurnContextMemoryTaxonomyClass::Unknown;
        manifest.refresh_ledger_hash();
        assert!(!manifest.memory_taxonomy_has_integrity());
        assert!(!manifest.has_replay_integrity());

        Ok(())
    }

    #[test]
    fn turn_context_manifest_memory_formation_receipts_are_payload_light_and_hashed() -> Result<()>
    {
        let mut manifest = TurnContextManifestItem {
            version: TURN_CONTEXT_MANIFEST_VERSION,
            estimated_tokens: 21,
            ledger_hash: None,
            budget_tokens: Some(30),
            omitted_entries: 0,
            omitted_sources: Vec::new(),
            truncated: false,
            decision_ledger_hash: None,
            decision_ledger: Vec::new(),
            recall_selection: None,
            recall_selected_snippets: None,
            memory_taxonomy: Vec::new(),
            memory_formation_receipts: vec![
                TurnContextMemoryFormationReceipt {
                    candidate_type: TurnContextMemoryFormationCandidateType::Fact,
                    transcript_span_count: 2,
                    provenance_span_count: 2,
                    confidence_basis_points: 6400,
                    idempotency_key_hash: "0123456789abcdef".into(),
                    privacy_class: "user_private".into(),
                    queued_for_background: true,
                    production_write: false,
                },
                TurnContextMemoryFormationReceipt {
                    candidate_type: TurnContextMemoryFormationCandidateType::Summary,
                    transcript_span_count: 2,
                    provenance_span_count: 1,
                    confidence_basis_points: 7000,
                    idempotency_key_hash: "fedcba9876543210".into(),
                    privacy_class: "user_private".into(),
                    queued_for_background: true,
                    production_write: false,
                },
            ],
            memory_temporal_facts: Vec::new(),
            compression_candidates: Vec::new(),
            adaptive_budget_allocations: Vec::new(),
            compression_stages: Vec::new(),
            entries: vec![TurnContextManifestEntry {
                role: "developer".into(),
                tier: TurnContextTier::RetrievedSnippets,
                source: "turn_context:developer:selected_context_recall:0".into(),
                replay_key: "turn_context:developer:selected_context_recall:0:0123456789abcdef"
                    .into(),
                text_hash: "0123456789abcdef".into(),
                estimated_tokens: 21,
            }],
        }
        .with_refreshed_ledger_hash();

        let value = serde_json::to_value(&manifest)?;

        assert_eq!(
            manifest.memory_formation_receipts[0].schema_version(),
            Some(TURN_CONTEXT_MEMORY_FORMATION_RECEIPT_SCHEMA_VERSION)
        );
        assert_eq!(
            value["memory_formation_receipts"][0]["candidate_type"],
            "fact"
        );
        assert_eq!(
            value["memory_formation_receipts"][0]["transcript_span_count"],
            2
        );
        assert_eq!(
            value["memory_formation_receipts"][0]["provenance_span_count"],
            2
        );
        assert_eq!(
            value["memory_formation_receipts"][0]["confidence_basis_points"],
            6400
        );
        assert_eq!(
            value["memory_formation_receipts"][0]["idempotency_key_hash"],
            "0123456789abcdef"
        );
        assert_eq!(
            value["memory_formation_receipts"][0]["privacy_class"],
            "user_private"
        );
        assert_eq!(
            value["memory_formation_receipts"][0]["queued_for_background"],
            true
        );
        assert!(
            value["memory_formation_receipts"][0]
                .get("production_write")
                .is_none()
        );
        assert!(
            value["memory_formation_receipts"][0]
                .get("transcript_text")
                .is_none()
        );
        assert!(
            value["memory_formation_receipts"][0]
                .get("memory_id")
                .is_none()
        );
        assert!(
            value["memory_formation_receipts"][0]
                .get("source_id")
                .is_none()
        );
        assert!(value["memory_formation_receipts"][0].get("query").is_none());
        assert!(manifest.memory_formation_receipts_have_integrity());
        assert!(manifest.has_replay_integrity());

        let original_ledger_hash = manifest
            .ledger_hash
            .clone()
            .expect("ledger hash should be materialized");
        manifest.memory_formation_receipts[0].confidence_basis_points = 6100;
        assert!(!manifest.ledger_hash_matches_manifest());
        manifest.refresh_ledger_hash();
        assert_ne!(
            manifest.ledger_hash.as_deref(),
            Some(original_ledger_hash.as_str())
        );
        assert!(manifest.has_replay_integrity());

        manifest.memory_formation_receipts[0].production_write = true;
        manifest.refresh_ledger_hash();
        assert!(!manifest.memory_formation_receipts_have_integrity());
        assert!(!manifest.has_replay_integrity());

        manifest.memory_formation_receipts[0].production_write = false;
        manifest.memory_formation_receipts[1].candidate_type =
            TurnContextMemoryFormationCandidateType::Unknown;
        manifest.refresh_ledger_hash();
        assert!(!manifest.memory_formation_receipts_have_integrity());
        assert!(!manifest.has_replay_integrity());

        manifest.memory_formation_receipts[1].candidate_type =
            TurnContextMemoryFormationCandidateType::Summary;
        manifest.memory_formation_receipts[1].idempotency_key_hash = "raw-key".into();
        manifest.refresh_ledger_hash();
        assert!(!manifest.memory_formation_receipts_have_integrity());
        assert!(!manifest.has_replay_integrity());

        Ok(())
    }

    #[test]
    fn turn_context_manifest_memory_temporal_facts_are_payload_light_and_hashed() -> Result<()> {
        let mut manifest = TurnContextManifestItem {
            version: TURN_CONTEXT_MANIFEST_VERSION,
            estimated_tokens: 21,
            ledger_hash: None,
            budget_tokens: Some(24),
            omitted_entries: 0,
            omitted_sources: Vec::new(),
            truncated: false,
            decision_ledger_hash: None,
            decision_ledger: Vec::new(),
            recall_selection: None,
            recall_selected_snippets: None,
            memory_taxonomy: Vec::new(),
            memory_formation_receipts: Vec::new(),
            memory_temporal_facts: vec![
                TurnContextMemoryTemporalFact {
                    fact_type: TurnContextMemoryTemporalFactType::Attribute,
                    entity_hash: "0123456789abcdef".into(),
                    provenance_span_count: 2,
                    valid_from_sequence: 8,
                    invalid_at_sequence: None,
                    confidence_basis_points: 6200,
                    supersedes_fact_hash: None,
                    privacy_class: "user_private".into(),
                    dry_run_only: true,
                    production_write: false,
                },
                TurnContextMemoryTemporalFact {
                    fact_type: TurnContextMemoryTemporalFactType::Summary,
                    entity_hash: "fedcba9876543210".into(),
                    provenance_span_count: 1,
                    valid_from_sequence: 9,
                    invalid_at_sequence: Some(12),
                    confidence_basis_points: 7000,
                    supersedes_fact_hash: Some("aaaaaaaaaaaaaaaa".into()),
                    privacy_class: "user_private".into(),
                    dry_run_only: true,
                    production_write: false,
                },
            ],
            compression_candidates: Vec::new(),
            adaptive_budget_allocations: Vec::new(),
            compression_stages: Vec::new(),
            entries: vec![TurnContextManifestEntry {
                role: "developer".into(),
                tier: TurnContextTier::RetrievedSnippets,
                source: "turn_context:developer:selected_context_recall:0".into(),
                replay_key: "turn_context:developer:selected_context_recall:0:0123456789abcdef"
                    .into(),
                text_hash: "0123456789abcdef".into(),
                estimated_tokens: 21,
            }],
        }
        .with_refreshed_ledger_hash();

        let value = serde_json::to_value(&manifest)?;

        assert_eq!(
            manifest.memory_temporal_facts[0].schema_version(),
            Some(TURN_CONTEXT_MEMORY_TEMPORAL_FACT_SCHEMA_VERSION)
        );
        assert_eq!(value["memory_temporal_facts"][0]["fact_type"], "attribute");
        assert_eq!(
            value["memory_temporal_facts"][0]["entity_hash"],
            "0123456789abcdef"
        );
        assert_eq!(
            value["memory_temporal_facts"][0]["provenance_span_count"],
            2
        );
        assert_eq!(value["memory_temporal_facts"][0]["valid_from_sequence"], 8);
        assert_eq!(
            value["memory_temporal_facts"][0]["confidence_basis_points"],
            6200
        );
        assert_eq!(
            value["memory_temporal_facts"][1]["supersedes_fact_hash"],
            "aaaaaaaaaaaaaaaa"
        );
        assert!(value["memory_temporal_facts"][0].get("fact_text").is_none());
        assert!(
            value["memory_temporal_facts"][0]
                .get("transcript_text")
                .is_none()
        );
        assert!(
            value["memory_temporal_facts"][0]
                .get("memory_text")
                .is_none()
        );
        assert!(value["memory_temporal_facts"][0].get("source_id").is_none());
        assert!(value["memory_temporal_facts"][0].get("memory_id").is_none());
        assert!(value["memory_temporal_facts"][0].get("query").is_none());
        assert!(manifest.memory_temporal_facts_have_integrity());
        assert!(manifest.has_replay_integrity());

        let original_ledger_hash = manifest
            .ledger_hash
            .clone()
            .expect("ledger hash should be materialized");
        manifest.memory_temporal_facts[0].confidence_basis_points = 6100;
        assert!(!manifest.ledger_hash_matches_manifest());
        manifest.refresh_ledger_hash();
        assert_ne!(
            manifest.ledger_hash.as_deref(),
            Some(original_ledger_hash.as_str())
        );
        assert!(manifest.has_replay_integrity());

        manifest.memory_temporal_facts[0].production_write = true;
        manifest.refresh_ledger_hash();
        assert!(!manifest.memory_temporal_facts_have_integrity());
        assert!(!manifest.has_replay_integrity());

        manifest.memory_temporal_facts[0].production_write = false;
        manifest.memory_temporal_facts[1].fact_type = TurnContextMemoryTemporalFactType::Unknown;
        manifest.refresh_ledger_hash();
        assert!(!manifest.memory_temporal_facts_have_integrity());
        assert!(!manifest.has_replay_integrity());

        manifest.memory_temporal_facts[1].fact_type = TurnContextMemoryTemporalFactType::Summary;
        manifest.memory_temporal_facts[1].supersedes_fact_hash = Some("raw-fact-id".into());
        manifest.refresh_ledger_hash();
        assert!(!manifest.memory_temporal_facts_have_integrity());
        assert!(!manifest.has_replay_integrity());

        Ok(())
    }

    #[test]
    fn turn_context_manifest_compression_stages_are_payload_light_and_hashed() -> Result<()> {
        assert_eq!(
            TurnContextCompressionStageKind::Summary.schema_version(),
            Some(TURN_CONTEXT_COMPRESSION_STAGE_SCHEMA_VERSION)
        );
        assert_eq!(
            TurnContextCompressionStageKind::Unknown.schema_version(),
            None
        );

        let mut manifest = TurnContextManifestItem {
            version: TURN_CONTEXT_MANIFEST_VERSION,
            estimated_tokens: 12,
            ledger_hash: None,
            budget_tokens: Some(16),
            omitted_entries: 0,
            omitted_sources: Vec::new(),
            truncated: false,
            decision_ledger_hash: None,
            decision_ledger: Vec::new(),
            recall_selection: None,
            recall_selected_snippets: None,
            memory_taxonomy: Vec::new(),
            memory_formation_receipts: Vec::new(),
            memory_temporal_facts: Vec::new(),
            compression_candidates: Vec::new(),
            adaptive_budget_allocations: Vec::new(),
            compression_stages: vec![
                TurnContextCompressionStage {
                    kind: TurnContextCompressionStageKind::Summary,
                    input_tokens: 40,
                    output_tokens: 12,
                    affected_entries: 2,
                    loss_check_status: Some(
                        TurnContextCompressionLossCheckStatus::MarkerBoundaryOnly,
                    ),
                    rollback_source_text_hash: Some("aaaaaaaaaaaaaaaa".into()),
                    protected_tier_invariant: Some(
                        TurnContextCompressionProtectedTierInvariant::Preserved,
                    ),
                },
                TurnContextCompressionStage {
                    kind: TurnContextCompressionStageKind::Defragment,
                    input_tokens: 12,
                    output_tokens: 10,
                    affected_entries: 1,
                    loss_check_status: Some(
                        TurnContextCompressionLossCheckStatus::MarkerBoundaryOnly,
                    ),
                    rollback_source_text_hash: Some("bbbbbbbbbbbbbbbb".into()),
                    protected_tier_invariant: Some(
                        TurnContextCompressionProtectedTierInvariant::Preserved,
                    ),
                },
            ],
            entries: vec![TurnContextManifestEntry {
                role: "developer".into(),
                tier: TurnContextTier::Summary,
                source: "turn_context:developer:summary:0".into(),
                replay_key: "turn_context:developer:summary:0:0123456789abcdef".into(),
                text_hash: "0123456789abcdef".into(),
                estimated_tokens: 12,
            }],
        }
        .with_refreshed_ledger_hash();

        let value = serde_json::to_value(&manifest)?;

        assert_eq!(value["compression_stages"][0]["kind"], "summary");
        assert_eq!(value["compression_stages"][0]["input_tokens"], 40);
        assert_eq!(value["compression_stages"][0]["output_tokens"], 12);
        assert_eq!(value["compression_stages"][0]["affected_entries"], 2);
        assert_eq!(
            value["compression_stages"][0]["loss_check_status"],
            "marker_boundary_only"
        );
        assert_eq!(
            value["compression_stages"][0]["rollback_source_text_hash"],
            "aaaaaaaaaaaaaaaa"
        );
        assert_eq!(
            value["compression_stages"][0]["protected_tier_invariant"],
            "preserved"
        );
        assert_eq!(value["compression_stages"][1]["kind"], "defragment");
        assert!(value["compression_stages"][0].get("source").is_none());
        assert!(value["compression_stages"][0].get("text").is_none());
        assert!(value["compression_stages"][0].get("query").is_none());
        assert!(manifest.compression_stages_have_integrity());
        assert!(manifest.has_replay_integrity());

        let original_ledger_hash = manifest
            .ledger_hash
            .clone()
            .expect("ledger hash should be materialized");
        manifest.compression_stages[0].loss_check_status =
            Some(TurnContextCompressionLossCheckStatus::SemanticLossCheckPassed);
        assert!(!manifest.ledger_hash_matches_manifest());
        manifest.refresh_ledger_hash();
        assert_ne!(
            manifest.ledger_hash.as_deref(),
            Some(original_ledger_hash.as_str())
        );
        assert!(manifest.has_replay_integrity());

        manifest.compression_stages[0].output_tokens = 11;
        assert!(!manifest.ledger_hash_matches_manifest());
        manifest.refresh_ledger_hash();
        assert!(manifest.has_replay_integrity());

        manifest.compression_stages[0].output_tokens = 41;
        manifest.refresh_ledger_hash();
        assert!(!manifest.compression_stages_have_integrity());
        assert!(!manifest.has_replay_integrity());

        manifest.compression_stages[0].output_tokens = 11;
        manifest.compression_stages[1].rollback_source_text_hash = Some("not-a-hash".into());
        manifest.refresh_ledger_hash();
        assert!(!manifest.compression_stages_have_integrity());
        assert!(!manifest.has_replay_integrity());

        manifest.compression_stages[1].rollback_source_text_hash = Some("bbbbbbbbbbbbbbbb".into());
        manifest.compression_stages[1].kind = TurnContextCompressionStageKind::Unknown;
        manifest.refresh_ledger_hash();
        assert!(!manifest.compression_stages_have_integrity());
        assert!(!manifest.has_replay_integrity());

        Ok(())
    }

    #[test]
    fn turn_context_manifest_recall_selection_serializes_payload_light_rollup() -> Result<()> {
        let mut manifest = TurnContextManifestItem {
            version: TURN_CONTEXT_MANIFEST_VERSION,
            estimated_tokens: 3,
            ledger_hash: None,
            budget_tokens: Some(4),
            omitted_entries: 0,
            omitted_sources: Vec::new(),
            truncated: false,
            decision_ledger_hash: None,
            decision_ledger: Vec::new(),
            recall_selection: Some(TurnContextRecallSelectionSummary {
                returned_source_count: 4,
                selected_source_count: 3,
                ranked_source_count: 3,
                returned_unselected_source_count: 1,
                source_diversity_met: true,
                source_diversity_target: 3,
                max_per_source: 2,
                ranked_item_count: 3,
                omitted_by_budget_count: 1,
                memory_control_omitted_count: 2,
                low_trust_ranked_item_count: 1,
                low_recency_ranked_item_count: 2,
            }),
            recall_selected_snippets: Some(TurnContextRecallSelectedSnippetEnvelope {
                version: TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION,
                max_snippets: 4,
                max_snippet_chars: 120,
                selected_snippet_count: 1,
                omitted_snippet_count: 2,
                redacted_snippet_count: 1,
                truncated_snippet_count: 0,
                snippets: vec![TurnContextRecallSelectedSnippet {
                    snippet_hash: "fedcba9876543210".into(),
                    text: "[redacted-query] bounded memory".into(),
                    estimated_tokens: 8,
                    redacted: true,
                    truncated: false,
                }],
                safety: TurnContextRecallSelectedSnippetSafety {
                    ready_for_shadow_handoff: true,
                    bounded: true,
                    origin_identifiers_exposed: false,
                    raw_ranked_payload_exposed: false,
                    rank_explanation_exposed: false,
                    control_marker_exposed: false,
                    query_payload_exposed: false,
                    per_origin_list_exposed: false,
                },
            }),
            memory_taxonomy: Vec::new(),
            memory_formation_receipts: Vec::new(),
            memory_temporal_facts: Vec::new(),
            compression_candidates: Vec::new(),
            adaptive_budget_allocations: Vec::new(),
            compression_stages: Vec::new(),
            entries: vec![TurnContextManifestEntry {
                role: "developer".into(),
                tier: TurnContextTier::System,
                source: "initial_context:permissions:0".into(),
                replay_key: "initial_context:permissions:0:0123456789abcdef".into(),
                text_hash: "0123456789abcdef".into(),
                estimated_tokens: 3,
            }],
        }
        .with_refreshed_ledger_hash();

        let value = serde_json::to_value(&manifest)?;

        assert_eq!(value["recall_selection"]["returned_source_count"], 4);
        assert_eq!(value["recall_selection"]["selected_source_count"], 3);
        assert_eq!(value["recall_selection"]["ranked_source_count"], 3);
        assert_eq!(
            value["recall_selection"]["returned_unselected_source_count"],
            1
        );
        assert_eq!(value["recall_selection"]["source_diversity_met"], true);
        assert_eq!(value["recall_selection"]["source_diversity_target"], 3);
        assert_eq!(value["recall_selection"]["max_per_source"], 2);
        assert_eq!(value["recall_selection"]["ranked_item_count"], 3);
        assert_eq!(value["recall_selection"]["omitted_by_budget_count"], 1);
        assert_eq!(value["recall_selection"]["memory_control_omitted_count"], 2);
        assert_eq!(value["recall_selection"]["low_trust_ranked_item_count"], 1);
        assert_eq!(
            value["recall_selection"]["low_recency_ranked_item_count"],
            2
        );
        assert!(value["recall_selection"].get("source_id").is_none());
        assert!(value["recall_selection"].get("summary").is_none());
        assert_eq!(
            value["recall_selected_snippets"]["selected_snippet_count"],
            1
        );
        assert_eq!(
            value["recall_selected_snippets"]["snippets"][0]["text"],
            "[redacted-query] bounded memory"
        );
        assert!(value["recall_selected_snippets"].get("source_id").is_none());
        assert!(
            value["recall_selected_snippets"]["snippets"][0]
                .get("source_memory_ids")
                .is_none()
        );
        assert!(
            manifest
                .recall_selection
                .as_ref()
                .expect("recall selection")
                .returned_unselected_source_count_matches()
        );
        assert!(manifest.recall_selection_has_integrity());
        assert!(manifest.recall_selected_snippets_have_integrity());
        assert!(manifest.has_replay_integrity());

        let original_ledger_hash = manifest
            .ledger_hash
            .clone()
            .expect("ledger hash should be materialized");
        manifest
            .recall_selection
            .as_mut()
            .expect("recall selection")
            .omitted_by_budget_count = 0;
        assert!(!manifest.ledger_hash_matches_manifest());
        manifest.refresh_ledger_hash();
        assert_ne!(
            manifest.ledger_hash.as_deref(),
            Some(original_ledger_hash.as_str())
        );
        assert!(manifest.has_replay_integrity());

        manifest
            .recall_selection
            .as_mut()
            .expect("recall selection")
            .ranked_item_count = 2;
        manifest.refresh_ledger_hash();
        assert!(!manifest.recall_selection_has_integrity());
        assert!(!manifest.has_replay_integrity());
        manifest
            .recall_selection
            .as_mut()
            .expect("recall selection")
            .ranked_item_count = 3;
        manifest.refresh_ledger_hash();
        assert!(manifest.recall_selection_has_integrity());
        assert!(manifest.has_replay_integrity());

        {
            let recall_selection = manifest
                .recall_selection
                .as_mut()
                .expect("recall selection");
            recall_selection.ranked_source_count = 0;
            recall_selection.low_trust_ranked_item_count = 0;
            recall_selection.low_recency_ranked_item_count = 0;
        }
        manifest.refresh_ledger_hash();
        assert!(!manifest.recall_selection_has_integrity());
        assert!(!manifest.has_replay_integrity());
        manifest
            .recall_selection
            .as_mut()
            .expect("recall selection")
            .ranked_item_count = 0;
        manifest.refresh_ledger_hash();
        assert!(manifest.recall_selection_has_integrity());
        assert!(manifest.has_replay_integrity());

        {
            let recall_selection = manifest
                .recall_selection
                .as_mut()
                .expect("recall selection");
            recall_selection.ranked_source_count = 3;
            recall_selection.ranked_item_count = 3;
            recall_selection.low_trust_ranked_item_count = 1;
            recall_selection.low_recency_ranked_item_count = 2;
            recall_selection.source_diversity_met = false;
        }
        manifest.refresh_ledger_hash();
        assert!(!manifest.recall_selection_has_integrity());
        assert!(!manifest.has_replay_integrity());
        {
            let recall_selection = manifest
                .recall_selection
                .as_mut()
                .expect("recall selection");
            recall_selection.source_diversity_target = 0;
        }
        manifest.refresh_ledger_hash();
        assert!(manifest.recall_selection_has_integrity());
        assert!(manifest.has_replay_integrity());

        manifest
            .recall_selected_snippets
            .as_mut()
            .expect("recall selected snippets")
            .selected_snippet_count = 2;
        manifest.refresh_ledger_hash();
        assert!(!manifest.recall_selected_snippets_have_integrity());
        assert!(!manifest.has_replay_integrity());
        manifest
            .recall_selected_snippets
            .as_mut()
            .expect("recall selected snippets")
            .selected_snippet_count = 1;
        manifest.refresh_ledger_hash();
        assert!(manifest.recall_selected_snippets_have_integrity());
        assert!(manifest.has_replay_integrity());

        manifest
            .recall_selected_snippets
            .as_mut()
            .expect("recall selected snippets")
            .safety
            .query_payload_exposed = true;
        manifest.refresh_ledger_hash();
        assert!(!manifest.recall_selected_snippets_have_integrity());
        assert!(!manifest.has_replay_integrity());
        manifest
            .recall_selected_snippets
            .as_mut()
            .expect("recall selected snippets")
            .safety
            .query_payload_exposed = false;
        manifest.refresh_ledger_hash();
        assert!(manifest.recall_selected_snippets_have_integrity());
        assert!(manifest.has_replay_integrity());

        Ok(())
    }

    #[test]
    fn turn_context_manifest_selected_snippets_serializes_shadow_envelope() -> Result<()> {
        let mut manifest = TurnContextManifestItem {
            version: TURN_CONTEXT_MANIFEST_VERSION,
            estimated_tokens: 3,
            ledger_hash: None,
            budget_tokens: Some(4),
            omitted_entries: 0,
            omitted_sources: Vec::new(),
            truncated: false,
            decision_ledger_hash: None,
            decision_ledger: Vec::new(),
            recall_selection: None,
            recall_selected_snippets: Some(TurnContextRecallSelectedSnippetEnvelope {
                version: TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION,
                max_snippets: 4,
                max_snippet_chars: 120,
                selected_snippet_count: 1,
                omitted_snippet_count: 2,
                redacted_snippet_count: 1,
                truncated_snippet_count: 0,
                snippets: vec![TurnContextRecallSelectedSnippet {
                    snippet_hash: "fedcba9876543210".into(),
                    text: "[redacted-query] bounded memory".into(),
                    estimated_tokens: 8,
                    redacted: true,
                    truncated: false,
                }],
                safety: TurnContextRecallSelectedSnippetSafety {
                    ready_for_shadow_handoff: true,
                    bounded: true,
                    origin_identifiers_exposed: false,
                    raw_ranked_payload_exposed: false,
                    rank_explanation_exposed: false,
                    control_marker_exposed: false,
                    query_payload_exposed: false,
                    per_origin_list_exposed: false,
                },
            }),
            memory_taxonomy: Vec::new(),
            memory_formation_receipts: Vec::new(),
            memory_temporal_facts: Vec::new(),
            compression_candidates: Vec::new(),
            adaptive_budget_allocations: Vec::new(),
            compression_stages: Vec::new(),
            entries: vec![TurnContextManifestEntry {
                role: "developer".into(),
                tier: TurnContextTier::System,
                source: "initial_context:permissions:0".into(),
                replay_key: "initial_context:permissions:0:0123456789abcdef".into(),
                text_hash: "0123456789abcdef".into(),
                estimated_tokens: 3,
            }],
        }
        .with_refreshed_ledger_hash();

        let value = serde_json::to_value(&manifest)?;

        assert_eq!(
            value["recall_selected_snippets"]["selected_snippet_count"],
            1
        );
        assert_eq!(
            value["recall_selected_snippets"]["snippets"][0]["text"],
            "[redacted-query] bounded memory"
        );
        assert!(
            value["recall_selected_snippets"]["snippets"][0]
                .get("source_id")
                .is_none()
        );
        assert!(manifest.recall_selected_snippets_have_integrity());
        assert!(manifest.has_replay_integrity());

        let original_ledger_hash = manifest
            .ledger_hash
            .clone()
            .expect("ledger hash should be materialized");
        manifest
            .recall_selected_snippets
            .as_mut()
            .expect("recall selected snippets")
            .snippets[0]
            .text = "[redacted-query] changed bounded memory".into();
        assert!(!manifest.ledger_hash_matches_manifest());
        manifest.refresh_ledger_hash();
        assert_ne!(
            manifest.ledger_hash.as_deref(),
            Some(original_ledger_hash.as_str())
        );
        assert!(manifest.has_replay_integrity());

        manifest
            .recall_selected_snippets
            .as_mut()
            .expect("recall selected snippets")
            .snippets[0]
            .text = "[hepta-memory:tombstone] leaked control marker".into();
        manifest.refresh_ledger_hash();
        assert!(!manifest.recall_selected_snippets_have_integrity());
        assert!(!manifest.has_replay_integrity());

        Ok(())
    }

    /// Serialize Event to verify that its JSON representation has the expected
    /// amount of nesting.
    #[test]
    fn serialize_event() -> Result<()> {
        let session_id = SessionId::from_string("67e55044-10b1-426f-9247-bb680e5fe0c7")?;
        let thread_id = ThreadId::from_string("67e55044-10b1-426f-9247-bb680e5fe0c8")?;
        let rollout_file = NamedTempFile::new()?;
        let permission_profile = PermissionProfile::read_only();
        let event = Event {
            id: "1234".to_string(),
            msg: EventMsg::SessionConfigured(SessionConfiguredEvent {
                session_id,
                thread_id,
                forked_from_id: None,
                thread_source: None,
                thread_name: None,
                model: "codex-mini-latest".to_string(),
                model_provider_id: "openai".to_string(),
                service_tier: None,
                approval_policy: AskForApproval::Never,
                approvals_reviewer: ApprovalsReviewer::User,
                permission_profile: permission_profile.clone(),
                active_permission_profile: None,
                cwd: test_path_buf("/home/user/project").abs(),
                reasoning_effort: Some(ReasoningEffortConfig::default()),
                initial_messages: None,
                network_proxy: None,
                rollout_path: Some(rollout_file.path().to_path_buf()),
            }),
        };

        let expected = json!({
            "id": "1234",
            "msg": {
                "type": "session_configured",
                "session_id": "67e55044-10b1-426f-9247-bb680e5fe0c7",
                "thread_id": "67e55044-10b1-426f-9247-bb680e5fe0c8",
                "model": "codex-mini-latest",
                "model_provider_id": "openai",
                "approval_policy": "never",
                "approvals_reviewer": "user",
                "permission_profile": permission_profile,
                "cwd": test_path_buf("/home/user/project"),
                "reasoning_effort": "medium",
                "rollout_path": format!("{}", rollout_file.path().display()),
            }
        });
        assert_eq!(expected, serde_json::to_value(&event)?);
        Ok(())
    }

    #[test]
    fn deserialize_legacy_session_configured_event_uses_sandbox_policy() -> Result<()> {
        let cwd = test_path_buf("/home/user/project");
        let value = json!({
            "session_id": "67e55044-10b1-426f-9247-bb680e5fe0c8",
            "model": "codex-mini-latest",
            "model_provider_id": "openai",
            "approval_policy": "never",
            "approvals_reviewer": "user",
            "sandbox_policy": {
                "type": "read-only"
            },
            "cwd": cwd,
        });

        let event: SessionConfiguredEvent = serde_json::from_value(value)?;
        assert_eq!(event.permission_profile, PermissionProfile::read_only());
        Ok(())
    }

    #[test]
    fn vec_u8_as_base64_serialization_and_deserialization() -> Result<()> {
        let event = ExecCommandOutputDeltaEvent {
            call_id: "call21".to_string(),
            stream: ExecOutputStream::Stdout,
            chunk: vec![1, 2, 3, 4, 5],
        };
        let serialized = serde_json::to_string(&event)?;
        assert_eq!(
            r#"{"call_id":"call21","stream":"stdout","chunk":"AQIDBAU="}"#,
            serialized,
        );

        let deserialized: ExecCommandOutputDeltaEvent = serde_json::from_str(&serialized)?;
        assert_eq!(deserialized, event);
        Ok(())
    }

    #[test]
    fn serialize_mcp_startup_update_event() -> Result<()> {
        let event = Event {
            id: "init".to_string(),
            msg: EventMsg::McpStartupUpdate(McpStartupUpdateEvent {
                server: "srv".to_string(),
                status: McpStartupStatus::Failed {
                    error: "boom".to_string(),
                },
            }),
        };

        let value = serde_json::to_value(&event)?;
        assert_eq!(value["msg"]["type"], "mcp_startup_update");
        assert_eq!(value["msg"]["server"], "srv");
        assert_eq!(value["msg"]["status"]["state"], "failed");
        assert_eq!(value["msg"]["status"]["error"], "boom");
        Ok(())
    }

    #[test]
    fn serialize_mcp_startup_complete_event() -> Result<()> {
        let event = Event {
            id: "init".to_string(),
            msg: EventMsg::McpStartupComplete(McpStartupCompleteEvent {
                ready: vec!["a".to_string()],
                failed: vec![McpStartupFailure {
                    server: "b".to_string(),
                    error: "bad".to_string(),
                }],
                cancelled: vec!["c".to_string()],
            }),
        };

        let value = serde_json::to_value(&event)?;
        assert_eq!(value["msg"]["type"], "mcp_startup_complete");
        assert_eq!(value["msg"]["ready"][0], "a");
        assert_eq!(value["msg"]["failed"][0]["server"], "b");
        assert_eq!(value["msg"]["failed"][0]["error"], "bad");
        assert_eq!(value["msg"]["cancelled"][0], "c");
        Ok(())
    }

    #[test]
    fn token_usage_info_new_or_append_updates_context_window_when_provided() {
        let initial = Some(TokenUsageInfo {
            total_token_usage: TokenUsage::default(),
            last_token_usage: TokenUsage::default(),
            model_context_window: Some(258_400),
        });
        let last = Some(TokenUsage {
            input_tokens: 10,
            cached_input_tokens: 0,
            output_tokens: 0,
            reasoning_output_tokens: 0,
            total_tokens: 10,
        });

        let info = TokenUsageInfo::new_or_append(&initial, &last, Some(128_000))
            .expect("new_or_append should return info");

        assert_eq!(info.model_context_window, Some(128_000));
    }

    #[test]
    fn token_usage_info_new_or_append_preserves_context_window_when_not_provided() {
        let initial = Some(TokenUsageInfo {
            total_token_usage: TokenUsage::default(),
            last_token_usage: TokenUsage::default(),
            model_context_window: Some(258_400),
        });
        let last = Some(TokenUsage {
            input_tokens: 10,
            cached_input_tokens: 0,
            output_tokens: 0,
            reasoning_output_tokens: 0,
            total_tokens: 10,
        });

        let info =
            TokenUsageInfo::new_or_append(&initial, &last, /*model_context_window*/ None)
                .expect("new_or_append should return info");

        assert_eq!(info.model_context_window, Some(258_400));
    }

    fn test_selected_snippet_envelope() -> TurnContextRecallSelectedSnippetEnvelope {
        TurnContextRecallSelectedSnippetEnvelope {
            version: TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION,
            max_snippets: 4,
            max_snippet_chars: 120,
            selected_snippet_count: 1,
            omitted_snippet_count: 2,
            redacted_snippet_count: 1,
            truncated_snippet_count: 0,
            snippets: vec![TurnContextRecallSelectedSnippet {
                snippet_hash: "fedcba9876543210".into(),
                text: "[redacted-query] bounded memory".into(),
                estimated_tokens: 8,
                redacted: true,
                truncated: false,
            }],
            safety: TurnContextRecallSelectedSnippetSafety {
                ready_for_shadow_handoff: true,
                bounded: true,
                origin_identifiers_exposed: false,
                raw_ranked_payload_exposed: false,
                rank_explanation_exposed: false,
                control_marker_exposed: false,
                query_payload_exposed: false,
                per_origin_list_exposed: false,
            },
        }
    }
}

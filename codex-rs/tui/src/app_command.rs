use std::path::PathBuf;

use codex_app_server_protocol::AskForApproval;
use codex_app_server_protocol::CommandExecutionApprovalDecision;
use codex_app_server_protocol::FileChangeApprovalDecision;
use codex_app_server_protocol::McpServerElicitationAction;
use codex_app_server_protocol::RequestId as AppServerRequestId;
use codex_app_server_protocol::ReviewTarget;
use codex_app_server_protocol::ThreadRealtimeAudioChunk;
use codex_app_server_protocol::ThreadRealtimeStartTransport;
use codex_app_server_protocol::ToolRequestUserInputResponse;
use codex_app_server_protocol::UserInput;
use codex_config::types::ApprovalsReviewer;
use codex_protocol::approvals::GuardianAssessmentEvent;
use codex_protocol::config_types::CollaborationMode;
use codex_protocol::config_types::Personality;
use codex_protocol::config_types::ReasoningSummary as ReasoningSummaryConfig;
use codex_protocol::config_types::WindowsSandboxLevel;
use codex_protocol::models::ActivePermissionProfile;
use codex_protocol::openai_models::ReasoningEffort as ReasoningEffortConfig;
use codex_protocol::protocol::TurnContextRecallSelectedSnippetEnvelope as CoreTurnContextRecallSelectedSnippetEnvelope;
use codex_protocol::request_permissions::RequestPermissionsResponse;
use serde::Serialize;
use serde_json::Value;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) enum AppCommand {
    Interrupt,
    CleanBackgroundTerminals,
    RealtimeConversationStart {
        transport: Option<ThreadRealtimeStartTransport>,
        voice: Option<Value>,
    },
    RealtimeConversationAudio(ThreadRealtimeAudioChunk),
    RealtimeConversationClose,
    RunUserShellCommand {
        command: String,
    },
    UserTurn {
        items: Vec<UserInput>,
        #[serde(skip_serializing)]
        context_recall_selected_snippets: Option<CoreTurnContextRecallSelectedSnippetEnvelope>,
        cwd: PathBuf,
        approval_policy: AskForApproval,
        approvals_reviewer: Option<ApprovalsReviewer>,
        active_permission_profile: Option<ActivePermissionProfile>,
        model: String,
        effort: Option<ReasoningEffortConfig>,
        summary: Option<ReasoningSummaryConfig>,
        service_tier: Option<Option<String>>,
        final_output_json_schema: Option<Value>,
        collaboration_mode: Option<CollaborationMode>,
        personality: Option<Personality>,
    },
    OverrideTurnContext {
        cwd: Option<PathBuf>,
        approval_policy: Option<AskForApproval>,
        approvals_reviewer: Option<ApprovalsReviewer>,
        active_permission_profile: Option<ActivePermissionProfile>,
        windows_sandbox_level: Option<WindowsSandboxLevel>,
        model: Option<String>,
        effort: Option<Option<ReasoningEffortConfig>>,
        summary: Option<ReasoningSummaryConfig>,
        service_tier: Option<Option<String>>,
        collaboration_mode: Option<CollaborationMode>,
        personality: Option<Personality>,
    },
    ExecApproval {
        id: String,
        turn_id: Option<String>,
        decision: CommandExecutionApprovalDecision,
    },
    PatchApproval {
        id: String,
        decision: FileChangeApprovalDecision,
    },
    ResolveElicitation {
        server_name: String,
        request_id: AppServerRequestId,
        decision: McpServerElicitationAction,
        content: Option<Value>,
        meta: Option<Value>,
    },
    UserInputAnswer {
        id: String,
        response: ToolRequestUserInputResponse,
    },
    RequestPermissionsResponse {
        id: String,
        response: RequestPermissionsResponse,
    },
    ReloadUserConfig,
    ListSkills {
        cwds: Vec<PathBuf>,
        force_reload: bool,
    },
    Compact,
    SetThreadName {
        name: String,
    },
    Shutdown,
    ThreadRollback {
        num_turns: u32,
    },
    Review {
        target: ReviewTarget,
    },
    ApproveGuardianDeniedAction {
        event: GuardianAssessmentEvent,
    },
}

impl AppCommand {
    pub(crate) fn interrupt() -> Self {
        Self::Interrupt
    }

    pub(crate) fn clean_background_terminals() -> Self {
        Self::CleanBackgroundTerminals
    }

    pub(crate) fn realtime_conversation_start(
        transport: Option<ThreadRealtimeStartTransport>,
        voice: Option<Value>,
    ) -> Self {
        Self::RealtimeConversationStart { transport, voice }
    }

    #[cfg_attr(target_os = "linux", allow(dead_code))]
    pub(crate) fn realtime_conversation_audio(frame: ThreadRealtimeAudioChunk) -> Self {
        Self::RealtimeConversationAudio(frame)
    }

    pub(crate) fn realtime_conversation_close() -> Self {
        Self::RealtimeConversationClose
    }

    pub(crate) fn run_user_shell_command(command: String) -> Self {
        Self::RunUserShellCommand { command }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn user_turn(
        items: Vec<UserInput>,
        cwd: PathBuf,
        approval_policy: AskForApproval,
        active_permission_profile: Option<ActivePermissionProfile>,
        model: String,
        effort: Option<ReasoningEffortConfig>,
        summary: Option<ReasoningSummaryConfig>,
        service_tier: Option<Option<String>>,
        final_output_json_schema: Option<Value>,
        collaboration_mode: Option<CollaborationMode>,
        personality: Option<Personality>,
    ) -> Self {
        Self::user_turn_with_context_recall_selected_snippets(
            items,
            /*context_recall_selected_snippets*/ None,
            cwd,
            approval_policy,
            active_permission_profile,
            model,
            effort,
            summary,
            service_tier,
            final_output_json_schema,
            collaboration_mode,
            personality,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn user_turn_with_context_recall_selected_snippets(
        items: Vec<UserInput>,
        context_recall_selected_snippets: Option<CoreTurnContextRecallSelectedSnippetEnvelope>,
        cwd: PathBuf,
        approval_policy: AskForApproval,
        active_permission_profile: Option<ActivePermissionProfile>,
        model: String,
        effort: Option<ReasoningEffortConfig>,
        summary: Option<ReasoningSummaryConfig>,
        service_tier: Option<Option<String>>,
        final_output_json_schema: Option<Value>,
        collaboration_mode: Option<CollaborationMode>,
        personality: Option<Personality>,
    ) -> Self {
        Self::UserTurn {
            items,
            context_recall_selected_snippets,
            cwd,
            approval_policy,
            approvals_reviewer: None,
            active_permission_profile,
            model,
            effort,
            summary,
            service_tier,
            final_output_json_schema,
            collaboration_mode,
            personality,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn override_turn_context(
        cwd: Option<PathBuf>,
        approval_policy: Option<AskForApproval>,
        approvals_reviewer: Option<ApprovalsReviewer>,
        active_permission_profile: Option<ActivePermissionProfile>,
        windows_sandbox_level: Option<WindowsSandboxLevel>,
        model: Option<String>,
        effort: Option<Option<ReasoningEffortConfig>>,
        summary: Option<ReasoningSummaryConfig>,
        service_tier: Option<Option<String>>,
        collaboration_mode: Option<CollaborationMode>,
        personality: Option<Personality>,
    ) -> Self {
        Self::OverrideTurnContext {
            cwd,
            approval_policy,
            approvals_reviewer,
            active_permission_profile,
            windows_sandbox_level,
            model,
            effort,
            summary,
            service_tier,
            collaboration_mode,
            personality,
        }
    }

    pub(crate) fn exec_approval(
        id: String,
        turn_id: Option<String>,
        decision: CommandExecutionApprovalDecision,
    ) -> Self {
        Self::ExecApproval {
            id,
            turn_id,
            decision,
        }
    }

    pub(crate) fn patch_approval(id: String, decision: FileChangeApprovalDecision) -> Self {
        Self::PatchApproval { id, decision }
    }

    pub(crate) fn resolve_elicitation(
        server_name: String,
        request_id: AppServerRequestId,
        decision: McpServerElicitationAction,
        content: Option<Value>,
        meta: Option<Value>,
    ) -> Self {
        Self::ResolveElicitation {
            server_name,
            request_id,
            decision,
            content,
            meta,
        }
    }

    pub(crate) fn user_input_answer(id: String, response: ToolRequestUserInputResponse) -> Self {
        Self::UserInputAnswer { id, response }
    }

    pub(crate) fn request_permissions_response(
        id: String,
        response: RequestPermissionsResponse,
    ) -> Self {
        Self::RequestPermissionsResponse { id, response }
    }

    pub(crate) fn reload_user_config() -> Self {
        Self::ReloadUserConfig
    }

    pub(crate) fn list_skills(cwds: Vec<PathBuf>, force_reload: bool) -> Self {
        Self::ListSkills { cwds, force_reload }
    }

    pub(crate) fn compact() -> Self {
        Self::Compact
    }

    pub(crate) fn set_thread_name(name: String) -> Self {
        Self::SetThreadName { name }
    }

    #[allow(dead_code)]
    pub(crate) fn shutdown() -> Self {
        Self::Shutdown
    }

    pub(crate) fn thread_rollback(num_turns: u32) -> Self {
        Self::ThreadRollback { num_turns }
    }

    pub(crate) fn review(target: ReviewTarget) -> Self {
        Self::Review { target }
    }

    pub(crate) fn approve_guardian_denied_action(event: GuardianAssessmentEvent) -> Self {
        Self::ApproveGuardianDeniedAction { event }
    }

    pub(crate) fn is_review(&self) -> bool {
        matches!(self, Self::Review { .. })
    }
}

impl From<&AppCommand> for AppCommand {
    fn from(value: &AppCommand) -> Self {
        value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_protocol::protocol::TurnContextRecallSelectedSnippet;
    use codex_protocol::protocol::TurnContextRecallSelectedSnippetSafety;

    #[test]
    fn user_turn_selected_snippets_are_not_serialized_into_tui_session_log_shape() {
        let op = AppCommand::user_turn_with_context_recall_selected_snippets(
            vec![UserInput::Text {
                text: "hello".to_string(),
                text_elements: Vec::new(),
            }],
            Some(test_core_selected_snippet_envelope()),
            PathBuf::from("/tmp"),
            AskForApproval::Never,
            None,
            "gpt-5".to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let serialized = serde_json::to_string(&op).expect("app command should serialize");

        assert!(!serialized.contains("context_recall_selected_snippets"));
        assert!(!serialized.contains("contextRecallSelectedSnippets"));
        assert!(!serialized.contains("bounded memory"));
        assert!(!serialized.contains("snippet_hash"));
        assert!(!serialized.contains("snippetHash"));
    }

    fn test_core_selected_snippet_envelope() -> CoreTurnContextRecallSelectedSnippetEnvelope {
        CoreTurnContextRecallSelectedSnippetEnvelope {
            version:
                codex_protocol::protocol::TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION,
            max_snippets: 4,
            max_snippet_chars: 120,
            selected_snippet_count: 1,
            omitted_snippet_count: 0,
            redacted_snippet_count: 0,
            truncated_snippet_count: 0,
            snippets: vec![TurnContextRecallSelectedSnippet {
                snippet_hash: "fedcba9876543210".to_string(),
                text: "bounded memory".to_string(),
                estimated_tokens: 4,
                redacted: false,
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

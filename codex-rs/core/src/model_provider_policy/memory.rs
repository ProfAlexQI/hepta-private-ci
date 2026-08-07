use std::sync::Arc;

use codex_extension_api::ModelProviderRequestKind;
use codex_protocol::error::Result as CodexResult;
use codex_protocol::protocol::Op;
use codex_protocol::protocol::W3cTraceContext;

use crate::CodexThread;
use crate::UserMessageAdmission;
use crate::model_provider_policy::ModelProviderPolicyContext;
use crate::session::session::Session;
use crate::session::turn_context::TurnContext;
use crate::user_message_admission::AdmittedUserMessage;
use crate::user_message_admission::PendingUserMessageAdmissionState;

/// Opaque capability for provider-governed detached Memory requests.
///
/// The handle retains the exact session and admitted parent turn. Callers
/// cannot substitute extension stores, admitted turn identity, or request kind.
pub struct MemoryModelProviderPolicyHandle {
    session: Arc<Session>,
    parent_turn: Arc<TurnContext>,
}

impl MemoryModelProviderPolicyHandle {
    fn new(session: Arc<Session>, parent_turn: Arc<TurnContext>) -> Self {
        Self {
            session,
            parent_turn,
        }
    }

    pub(crate) fn context(&self) -> ModelProviderPolicyContext<'_> {
        ModelProviderPolicyContext {
            registry: self.session.services.extensions.as_ref(),
            session_store: &self.session.services.session_extension_data,
            thread_store: &self.session.services.thread_extension_data,
            turn_store: self.parent_turn.extension_data.as_ref(),
            thread_id: self.session.thread_id().to_string(),
            turn_id: self.parent_turn.sub_id.clone(),
            request_kind: ModelProviderRequestKind::Memory,
            ephemeral_input_cwd: None,
        }
    }

    pub(crate) fn thread_id(&self) -> codex_protocol::ThreadId {
        self.session.thread_id()
    }

    pub(crate) fn session_id(&self) -> codex_protocol::SessionId {
        self.session.session_id()
    }
}

impl CodexThread {
    /// Submits user input and atomically retains the exact admitted turn as a
    /// provider-policy capability for detached Memory work.
    pub async fn submit_user_input_and_capture_memory_policy(
        &self,
        op: Op,
        trace: Option<W3cTraceContext>,
        client_user_message_id: Option<String>,
    ) -> CodexResult<(UserMessageAdmission, MemoryModelProviderPolicyHandle)> {
        let admitted = self
            .submit_user_input_and_wait_for_admission_inner(
                op,
                trace,
                client_user_message_id,
                PendingUserMessageAdmissionState::Immediate,
            )
            .await
            .map_err(codex_protocol::error::CodexErr::from)?;
        let (admission, parent_turn) = AdmittedUserMessage::into_parts(admitted);
        let policy = MemoryModelProviderPolicyHandle::new(Arc::clone(&self.session), parent_turn);
        Ok((admission, policy))
    }
}

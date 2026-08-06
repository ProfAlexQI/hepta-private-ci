use codex_protocol::error::Result as CodexResult;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::oneshot;

use crate::session::turn_context::TurnContext;

/// The turn that accepted a submitted user message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserMessageAdmission {
    /// Core installed a new turn for the submitted message.
    Started { turn_id: String },
    /// Core accepted the submitted message into an already-running turn.
    Steered { turn_id: String },
}

impl UserMessageAdmission {
    pub fn turn_id(&self) -> &str {
        match self {
            Self::Started { turn_id } | Self::Steered { turn_id } => turn_id,
        }
    }
}

/// Core-owned admission result that retains the exact turn context that
/// accepted the message. Public callers receive only [`UserMessageAdmission`];
/// narrow internal capabilities can retain this context without racing the
/// active-turn lifecycle.
pub(crate) struct AdmittedUserMessage {
    pub(crate) admission: UserMessageAdmission,
    pub(crate) turn_context: Arc<TurnContext>,
}

#[derive(Default)]
pub(crate) struct PendingUserMessageAdmissions {
    pending: Mutex<HashMap<String, oneshot::Sender<CodexResult<AdmittedUserMessage>>>>,
}

impl PendingUserMessageAdmissions {
    pub(crate) fn register(
        &self,
        submission_id: String,
    ) -> (
        PendingUserMessageAdmissionGuard<'_>,
        oneshot::Receiver<CodexResult<AdmittedUserMessage>>,
    ) {
        let (sender, receiver) = oneshot::channel();
        self.pending
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(submission_id.clone(), sender);
        (
            PendingUserMessageAdmissionGuard {
                admissions: self,
                submission_id,
            },
            receiver,
        )
    }

    pub(crate) fn complete(
        &self,
        submission_id: &str,
        admission: CodexResult<AdmittedUserMessage>,
    ) {
        let admission_sender = self
            .pending
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(submission_id);
        if let Some(sender) = admission_sender {
            let _ = sender.send(admission);
        }
    }

    fn remove(&self, submission_id: &str) {
        self.pending
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(submission_id);
    }
}

pub(crate) struct PendingUserMessageAdmissionGuard<'a> {
    admissions: &'a PendingUserMessageAdmissions,
    submission_id: String,
}

impl Drop for PendingUserMessageAdmissionGuard<'_> {
    fn drop(&mut self) {
        self.admissions.remove(&self.submission_id);
    }
}

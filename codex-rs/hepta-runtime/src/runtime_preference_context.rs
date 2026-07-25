use hepta_contracts::RevisionStamp;

use crate::HeptaError;
use crate::RuntimeKernel;

#[derive(Debug, Default)]
pub(crate) struct AttachedPreferenceContextState {
    entries: Vec<(String, RevisionStamp)>,
}

pub(crate) fn reset_attached_preference_context(
    runtime: &RuntimeKernel,
    session_id: Option<&str>,
) -> Result<(), HeptaError> {
    let mut state = runtime
        .attached_preference_context_state
        .lock()
        .map_err(|_| HeptaError("attached preference context mutex poisoned".into()))?;
    match session_id {
        Some(session_id) => state.remove_session(session_id),
        None => state.clear(),
    }
    Ok(())
}

impl AttachedPreferenceContextState {
    pub(crate) fn get(&self, session_id: &str) -> Option<RevisionStamp> {
        self.entries
            .iter()
            .find(|(session, _)| session == session_id)
            .map(|(_, stamp)| stamp.clone())
    }

    pub(crate) fn attach(&mut self, session_id: &str, stamp: RevisionStamp) {
        if let Some((_, current)) = self
            .entries
            .iter_mut()
            .find(|(session, _)| session == session_id)
        {
            *current = stamp;
        } else {
            self.entries.push((session_id.to_string(), stamp));
        }
    }

    pub(crate) fn remove_session(&mut self, session_id: &str) {
        self.entries.retain(|(session, _)| session != session_id);
    }

    pub(crate) fn clear(&mut self) {
        self.entries.clear();
    }
}

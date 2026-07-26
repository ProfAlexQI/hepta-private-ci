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

    pub(crate) fn attach(
        &mut self,
        session_id: &str,
        stamp: RevisionStamp,
    ) -> Result<(), HeptaError> {
        if let Some((_, current)) = self
            .entries
            .iter_mut()
            .find(|(session, _)| session == session_id)
        {
            if stamp.revision() < current.revision() {
                return Err(HeptaError(format!(
                    "authenticated preference context revision rollback from {} to {}",
                    current.revision(),
                    stamp.revision()
                )));
            }
            if stamp.revision() == current.revision() {
                if stamp.content_hash() != current.content_hash() {
                    return Err(HeptaError(format!(
                        "authenticated preference context diverged at revision {}",
                        current.revision()
                    )));
                }
                return Ok(());
            }
            *current = stamp;
        } else {
            self.entries.push((session_id.to_string(), stamp));
        }
        Ok(())
    }

    pub(crate) fn remove_session(&mut self, session_id: &str) {
        self.entries.retain(|(session, _)| session != session_id);
    }

    pub(crate) fn clear(&mut self) {
        self.entries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hepta_contracts::ContentHash;
    use hepta_contracts::Revision;

    fn stamp(revision: u64, content_hash: &str) -> RevisionStamp {
        RevisionStamp::new(Revision::new(revision), ContentHash::new(content_hash))
    }

    #[test]
    fn attached_preference_context_is_monotonic_and_idempotent() {
        let mut state = AttachedPreferenceContextState::default();
        state
            .attach("session", stamp(2, "sha256:revision-two"))
            .expect("attach revision two");
        state
            .attach("session", stamp(2, "sha256:revision-two"))
            .expect("idempotent revision two");

        assert!(
            state
                .attach("session", stamp(1, "sha256:revision-one"))
                .is_err()
        );
        assert!(
            state
                .attach("session", stamp(2, "sha256:revision-two-diverged"))
                .is_err()
        );
        assert_eq!(state.get("session"), Some(stamp(2, "sha256:revision-two")));

        state
            .attach("session", stamp(3, "sha256:revision-three"))
            .expect("attach revision three");
        assert_eq!(
            state.get("session"),
            Some(stamp(3, "sha256:revision-three"))
        );
    }
}

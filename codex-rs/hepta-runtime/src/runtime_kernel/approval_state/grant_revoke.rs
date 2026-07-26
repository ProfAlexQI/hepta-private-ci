use super::*;

impl ApprovalState {
    pub(crate) fn revoke_candidate_grant(&mut self, session_id: &str, binding_hash: &str) {
        if let Some(exact) = self
            .exact_sessions
            .iter_mut()
            .find(|session| session.session_id == session_id)
        {
            exact
                .grants
                .retain(|grant| grant.binding_hash().as_str() != binding_hash);
        }
    }
}

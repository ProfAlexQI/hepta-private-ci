use super::*;

impl RuntimeKernel {
    pub async fn approve_candidate_and_run_demo_turn_in_session(
        &self,
        session_id: &str,
        binding_hash: &str,
        input: &str,
    ) -> Result<VerticalSliceResult, HeptaError> {
        let session_id = session_id.trim();
        let binding_hash = binding_hash.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        if binding_hash.is_empty() {
            return Err(HeptaError(
                "candidate binding hash must not be empty".into(),
            ));
        }
        let _turn_reservation = self.begin_session_turn_reservation(session_id)?;
        if let Err(error) = self.approve_candidate_in_session(session_id, binding_hash) {
            let cleanup = self
                .approval_state
                .lock()
                .map_err(|_| HeptaError("approval state mutex poisoned".into()))
                .map(|mut approvals| {
                    approvals.revoke_candidate_grant(session_id, binding_hash);
                });
            return match cleanup {
                Ok(()) => Err(error),
                Err(cleanup_error) => Err(HeptaError(format!(
                    "{}; exact candidate cleanup also failed: {}",
                    error.0, cleanup_error.0
                ))),
            };
        }
        let result = TurnCoordinator {
            kernel: self,
            session_id: SessionId(session_id.to_owned()),
            input,
            model_timeout_ms: None,
            selected_snippets: None,
        }
        .run()
        .await;
        self.approval_state
            .lock()
            .map_err(|_| HeptaError("approval state mutex poisoned".into()))?
            .revoke_candidate_grant(session_id, binding_hash);
        result
    }
}

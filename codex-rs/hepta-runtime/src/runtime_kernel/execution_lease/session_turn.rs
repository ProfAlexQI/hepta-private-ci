use super::*;

/// Non-cloneable admission held across one session turn, including approval.
#[derive(Debug)]
pub(crate) struct SessionTurnReservation {
    registry: Arc<Mutex<ExecutionLeaseRegistry>>,
    session_id: String,
    active: bool,
}

impl RuntimeKernel {
    pub(crate) fn begin_session_turn_reservation(
        &self,
        session_id: &str,
    ) -> Result<SessionTurnReservation, HeptaError> {
        let mut registry = self.lock_execution_lease_registry()?;
        ensure_context_available(&registry, session_id)?;
        if registry.active_turn_sessions.contains(session_id)
            || registry.in_flight_sessions.contains(session_id)
        {
            return Err(session_error(
                ExecutionLeaseError::SessionTurnActive,
                session_id,
            ));
        }
        registry.active_turn_sessions.insert(session_id.to_owned());
        drop(registry);
        Ok(SessionTurnReservation {
            registry: Arc::clone(&self.execution_lease_registry),
            session_id: session_id.to_owned(),
            active: true,
        })
    }
}

impl Drop for SessionTurnReservation {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let mut registry = match self.registry.lock() {
            Ok(registry) => registry,
            Err(poisoned) => poisoned.into_inner(),
        };
        registry.active_turn_sessions.remove(&self.session_id);
        self.active = false;
    }
}

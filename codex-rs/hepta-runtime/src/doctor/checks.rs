use hepta_core::{MemoryRecord, ModelRef, SessionRecord};

use super::{DoctorCheck, state_checks};
use crate::{EventRecord, RuntimeKernel, RuntimeSnapshot, SessionApprovalState, TurnRecord};

impl RuntimeKernel {
    pub(super) fn session_integrity_checks(
        &self,
        active_model: &ModelRef,
        available_models: &[ModelRef],
        raw_sessions: &[SessionRecord],
        raw_memories: &[MemoryRecord],
        history: &[TurnRecord],
        events: &[EventRecord],
        approval_sessions: &[SessionApprovalState],
        snapshot: &RuntimeSnapshot,
    ) -> Vec<DoctorCheck> {
        state_checks::collect_session_integrity_checks(
            self,
            active_model,
            available_models,
            raw_sessions,
            raw_memories,
            history,
            events,
            approval_sessions,
            snapshot,
        )
    }
}

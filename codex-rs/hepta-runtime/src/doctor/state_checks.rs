use hepta_core::{MemoryRecord, ModelRef, SessionRecord};

use super::{DoctorCheck, event_log_integrity, runtime_state_integrity};
use crate::{EventRecord, RuntimeKernel, RuntimeSnapshot, SessionApprovalState, TurnRecord};

pub(super) fn collect_session_integrity_checks(
    runtime: &RuntimeKernel,
    active_model: &ModelRef,
    available_models: &[ModelRef],
    raw_sessions: &[SessionRecord],
    raw_memories: &[MemoryRecord],
    history: &[TurnRecord],
    events: &[EventRecord],
    approval_sessions: &[SessionApprovalState],
    snapshot: &RuntimeSnapshot,
) -> Vec<DoctorCheck> {
    let runtime_state_checks = runtime_state_integrity::collect_runtime_state_integrity_checks(
        runtime,
        active_model,
        available_models,
        raw_sessions,
        raw_memories,
        history,
        approval_sessions,
        snapshot,
    );

    let mut checks = runtime_state_checks.checks;
    checks.extend(event_log_integrity::collect_event_log_integrity_checks(
        events,
        &runtime_state_checks.known_session_ids,
    ));
    checks
}

use std::collections::HashSet;

use hepta_core::MemoryRecord;
use hepta_core::ModelRef;
use hepta_core::SessionRecord;

use super::DoctorCheck;
use super::runtime_state_check_bundle;
use super::runtime_state_check_bundle::RuntimeStateCheckInputs;
use super::runtime_state_findings;
use crate::RuntimeKernel;
use crate::RuntimeSnapshot;
use crate::SessionApprovalState;
use crate::TurnRecord;

pub(super) struct RuntimeStateIntegrityChecks {
    pub(super) known_session_ids: HashSet<String>,
    pub(super) checks: Vec<DoctorCheck>,
}

pub(super) fn collect_runtime_state_integrity_checks(
    runtime: &RuntimeKernel,
    active_model: &ModelRef,
    available_models: &[ModelRef],
    raw_sessions: &[SessionRecord],
    raw_memories: &[MemoryRecord],
    history: &[TurnRecord],
    approval_sessions: &[SessionApprovalState],
    snapshot: &RuntimeSnapshot,
) -> RuntimeStateIntegrityChecks {
    let mut findings = runtime_state_findings::collect_runtime_state_findings(
        runtime,
        active_model,
        available_models,
        raw_sessions,
        raw_memories,
        history,
        approval_sessions,
        snapshot,
    );
    let known_session_ids = std::mem::take(&mut findings.known_session_ids);
    let checks = runtime_state_check_bundle::build_runtime_state_checks(
        &findings,
        RuntimeStateCheckInputs {
            active_model,
            active_session_id: &snapshot.active_session_id,
            raw_session_count: raw_sessions.len(),
            raw_memory_count: raw_memories.len(),
            history_count: history.len(),
            session_model_binding_count: snapshot.session_models.len(),
        },
    );

    RuntimeStateIntegrityChecks {
        known_session_ids,
        checks,
    }
}

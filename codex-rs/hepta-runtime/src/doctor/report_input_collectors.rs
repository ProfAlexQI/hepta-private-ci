use hepta_core::{HeptaError, MemoryRecord, SessionRecord};

use super::DoctorProviderProbe;
use crate::{
    EventRecord, ModelSelection, RuntimeKernel, RuntimeSnapshot, SessionApprovalState, TurnRecord,
};

pub(super) struct CollectedDoctorRuntimeInputs {
    pub active_session_id: String,
    pub model_selection: ModelSelection,
    pub registered_providers: usize,
    pub registered_tools: usize,
    pub session_count: usize,
    pub total_topic_sessions: usize,
    pub total_topic_graph_edges: usize,
    pub raw_sessions: Vec<SessionRecord>,
    pub raw_memories: Vec<MemoryRecord>,
    pub history: Vec<TurnRecord>,
    pub events: Vec<EventRecord>,
    pub active_session_pending_approvals: usize,
    pub approval_sessions: Vec<SessionApprovalState>,
    pub snapshot: RuntimeSnapshot,
}

pub(super) fn collect_runtime_state_inputs(
    runtime: &RuntimeKernel,
) -> Result<CollectedDoctorRuntimeInputs, HeptaError> {
    let active_session_id = runtime.active_session_snapshot()?.session_id;
    let model_selection = runtime.model_selection()?;
    let registered_providers = runtime.provider_names().len();
    let registered_tools = runtime.tool_names().len();
    let session_count = runtime.sessions()?.len();
    let activity_overview = runtime.session_activity_overview(0, 0)?;
    let raw_sessions = runtime
        .memory
        .list_sessions()
        .map_err(|err| HeptaError(err.0))?;
    let raw_memories = runtime
        .memory
        .list_memories()
        .map_err(|err| HeptaError(err.0))?;
    let history = runtime.history(None, usize::MAX)?;
    let events = runtime.query_events(usize::MAX, None, None)?;
    let active_session_pending_approvals = runtime.approval_snapshot()?.pending.len();
    let snapshot = runtime.runtime_snapshot()?;
    let approval_sessions = runtime
        .approval_state
        .lock()
        .map_err(|_| HeptaError("approval state mutex poisoned".into()))?
        .all_sessions();

    Ok(CollectedDoctorRuntimeInputs {
        active_session_id,
        model_selection,
        registered_providers,
        registered_tools,
        session_count,
        total_topic_sessions: activity_overview.total_topic_sessions,
        total_topic_graph_edges: activity_overview.total_topic_graph_edges,
        raw_sessions,
        raw_memories,
        history,
        events,
        active_session_pending_approvals,
        approval_sessions,
        snapshot,
    })
}

pub(super) async fn collect_provider_probes(runtime: &RuntimeKernel) -> Vec<DoctorProviderProbe> {
    runtime.probe_providers().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn collects_runtime_state_inputs_from_doctor_runtime_reads() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello report input collectors")
            .await
            .expect("plain turn should succeed");
        runtime
            .route_topics(
                "session-main",
                Some("hello report input collectors"),
                4,
                4,
                4,
                1,
            )
            .expect("topic route should succeed");

        let inputs = collect_runtime_state_inputs(&runtime)
            .expect("runtime-state doctor inputs should collect successfully");

        assert_eq!(inputs.active_session_id, "session-main");
        assert_eq!(inputs.session_count, 1);
        assert_eq!(inputs.total_topic_sessions, 1);
        assert!(!inputs.raw_sessions.is_empty());
        assert!(!inputs.history.is_empty());
        assert!(!inputs.events.is_empty());
        assert!(inputs.registered_providers > 0);
        assert!(inputs.registered_tools > 0);
    }

    #[tokio::test]
    async fn collects_provider_probes_separately_from_runtime_state_reads() {
        let runtime = RuntimeKernel::new();

        let probes = collect_provider_probes(&runtime).await;

        assert!(!probes.is_empty());
        assert!(probes.iter().any(|probe| probe.provider_name == "demo"));
    }
}

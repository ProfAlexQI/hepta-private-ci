use super::{DoctorCheck, DoctorStatus, integrity};
use crate::{
    RuntimeIntelligenceEvalOverview, RuntimeKernel, query::RuntimeNeuronLifecycleOverview,
};

const DOCTOR_REPLAY_CASE_LIMIT: usize = 12;
const DOCTOR_REPLAY_RECENT_WINDOW_LIMIT: usize = 12;
const DOCTOR_REPLAY_TRANSCRIPT_LIMIT: usize = 12;
const DOCTOR_REPLAY_MEMORY_LIMIT: usize = 12;
const DOCTOR_REPLAY_TOPIC_LIMIT: usize = 4;
const DOCTOR_REPLAY_NEURON_LIMIT: usize = 4;
const DOCTOR_REPLAY_SKILL_LIMIT: usize = 4;

pub(super) fn active_session_replay_eval_check(
    runtime: &RuntimeKernel,
    active_session_id: &str,
) -> DoctorCheck {
    match runtime.intelligence_eval_overview(
        active_session_id,
        DOCTOR_REPLAY_CASE_LIMIT,
        DOCTOR_REPLAY_RECENT_WINDOW_LIMIT,
        DOCTOR_REPLAY_TRANSCRIPT_LIMIT,
        DOCTOR_REPLAY_MEMORY_LIMIT,
        DOCTOR_REPLAY_TOPIC_LIMIT,
        DOCTOR_REPLAY_NEURON_LIMIT,
        DOCTOR_REPLAY_SKILL_LIMIT,
    ) {
        Ok(overview) => integrity::doctor_check(
            integrity::ACTIVE_SESSION_INTELLIGENCE_REPLAY_EVAL,
            replay_eval_status(&overview),
            replay_eval_detail(&overview),
        ),
        Err(err) => integrity::doctor_check(
            integrity::ACTIVE_SESSION_INTELLIGENCE_REPLAY_EVAL,
            DoctorStatus::Fail,
            format!("replay eval failed for {active_session_id}: {}", err.0),
        ),
    }
}

pub(super) fn active_session_neuron_lifecycle_check(
    runtime: &RuntimeKernel,
    active_session_id: &str,
) -> DoctorCheck {
    match runtime.neuron_lifecycle_overview(active_session_id) {
        Ok(overview) => integrity::doctor_check(
            integrity::ACTIVE_SESSION_NEURON_LIFECYCLE,
            neuron_lifecycle_status(&overview),
            neuron_lifecycle_detail(&overview),
        ),
        Err(err) => integrity::doctor_check(
            integrity::ACTIVE_SESSION_NEURON_LIFECYCLE,
            DoctorStatus::Fail,
            format!(
                "neuron lifecycle check failed for {active_session_id}: {}",
                err.0
            ),
        ),
    }
}

fn replay_eval_status(overview: &RuntimeIntelligenceEvalOverview) -> DoctorStatus {
    if overview.failed_case_count == 0 {
        DoctorStatus::Ok
    } else {
        DoctorStatus::Fail
    }
}

fn replay_eval_detail(overview: &RuntimeIntelligenceEvalOverview) -> String {
    let summary = format!(
        "{}/{} cases passed; router={}; learned_cases={}; learned_signals={}; semantic={}/{} score={}; recall_items={}; transcript_spans={}; active_neurons={}; routed_topics={}; neuron_activations={}; suggested_skills={}; registered_skills={}; prepared_skills={}; gated_skills={}; workflow_priors={}; registered_workflows={}; prepared_workflows={}; gated_workflows={}; feedback_records={}; feedback_net={:+.2}; calibrated_skills={}; calibrated_workflows={}",
        overview.passed_case_count,
        overview.evaluated_case_count,
        overview.semantic_router_id,
        overview.learned_router_case_count,
        overview.total_learned_router_signals,
        overview.total_semantic_expectations_passed,
        overview.total_semantic_expectations,
        overview.semantic_score,
        overview.total_recall_ranked_items,
        overview.total_transcript_evidence_spans,
        overview.total_active_neurons,
        overview.total_routed_topics,
        overview.total_neuron_activations,
        overview.total_suggested_skills,
        overview.registered_skill_decision_count,
        overview.prepared_skill_decision_count,
        overview.gated_skill_decision_count,
        overview.total_workflow_priors,
        overview.registered_workflow_prior_count,
        overview.prepared_workflow_prior_count,
        overview.gated_workflow_prior_count,
        overview.feedback_record_count,
        overview.feedback_net_weight_delta,
        overview.calibrated_skill_target_count,
        overview.calibrated_workflow_target_count,
    );

    if overview.failed_case_count == 0 {
        return summary;
    }

    let failed_cases = overview
        .cases
        .iter()
        .filter(|case| !case.passed)
        .map(|case| format!("{} [{}]", case.case_id, case.warnings.join("; ")))
        .collect::<Vec<_>>()
        .join("; ");

    format!("{summary}; failed_cases={failed_cases}")
}

fn neuron_lifecycle_status(overview: &RuntimeNeuronLifecycleOverview) -> DoctorStatus {
    if overview.healthy {
        DoctorStatus::Ok
    } else {
        DoctorStatus::Fail
    }
}

fn neuron_lifecycle_detail(overview: &RuntimeNeuronLifecycleOverview) -> String {
    let summary = format!(
        "stored_neurons={}; active_topics={}; provenance={}/{}; evidence_digest={}/{}; lineage={}; merged={}; split={}; superseded={}; aging={}; cross_session_stable={}; cross_session_unstable={}; lineage_edges={}; avg_confidence={:.2}; avg_freshness={:.2}; stale={}; low_confidence={}; low_freshness={}",
        overview.stored_neurons,
        overview.active_topic_sessions,
        overview.neurons_with_transcript_provenance,
        overview.stored_neurons,
        overview.neurons_with_evidence_digest,
        overview.stored_neurons,
        overview.lineage_neurons,
        overview.merged_neurons,
        overview.split_neurons,
        overview.superseded_neurons,
        overview.aging_neurons,
        overview.cross_session_stable_neurons,
        overview.cross_session_unstable_neurons,
        overview.merge_split_lineage_edges,
        overview.average_confidence,
        overview.average_freshness,
        overview.stale_neurons,
        overview.low_confidence_neurons,
        overview.low_freshness_neurons,
    );

    if overview.findings.is_empty() {
        summary
    } else {
        format!("{summary}; findings={}", overview.findings.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn replay_eval_check_stays_ok_for_fresh_sessions_without_cases() {
        let runtime = RuntimeKernel::new();

        let check = active_session_replay_eval_check(&runtime, "session-main");

        assert_eq!(check.name, "active session intelligence replay eval");
        assert_eq!(check.status, DoctorStatus::Ok);
        assert!(check.detail.contains("0/0 cases passed"));
    }

    #[tokio::test]
    async fn replay_eval_check_passes_when_recent_turns_exercise_intelligence_loop() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello doctor intelligence eval")
            .await
            .expect("plain turn should succeed");

        let check = active_session_replay_eval_check(&runtime, "session-main");

        assert_eq!(check.status, DoctorStatus::Ok);
        assert!(check.detail.contains("1/1 cases passed"));
        assert!(check.detail.contains("semantic="));
        assert!(check.detail.contains("score="));
        assert!(check.detail.contains("routed_topics="));
        assert!(check.detail.contains("active_neurons="));
        assert!(check.detail.contains("neuron_activations="));
        assert!(check.detail.contains("workflow_priors="));
        assert!(check.detail.contains("registered_workflows="));
        assert!(check.detail.contains("prepared_workflows="));
        assert!(check.detail.contains("gated_workflows="));
        assert!(check.detail.contains("feedback_records="));
        assert!(check.detail.contains("calibrated_skills="));
        assert!(check.detail.contains("calibrated_workflows="));
    }

    #[tokio::test]
    async fn neuron_lifecycle_check_passes_after_intelligence_loop_materializes_neurons() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello doctor intelligence eval")
            .await
            .expect("plain turn should succeed");
        runtime
            .intelligence_eval_overview("session-main", 1, 12, 12, 12, 4, 4, 4)
            .expect("eval should materialize neuron lifecycle state");

        let check = active_session_neuron_lifecycle_check(&runtime, "session-main");

        assert_eq!(check.name, "active session neuron lifecycle");
        assert_eq!(check.status, DoctorStatus::Ok);
        assert!(check.detail.contains("stored_neurons="));
        assert!(check.detail.contains("provenance="));
        assert!(check.detail.contains("evidence_digest="));
    }
}

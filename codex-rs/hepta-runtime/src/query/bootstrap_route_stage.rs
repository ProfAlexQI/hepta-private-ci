use std::collections::BTreeSet;

use hepta_core::TopicActivationScore;
use hepta_core::TopicId;
use hepta_core::TopicSession;
use hepta_core::TopicSessionStatus;
use hepta_core::TopicShiftEvent;
use hepta_intelligence::BootstrapTopicRouteOutcomeDraftInput;
use hepta_intelligence::build_bootstrap_topic_route_outcome_draft;

use super::BootstrapTopicCandidateRoute;
use super::BootstrapTopicRouteOutcome;
use super::BootstrapTopicRoutePlan;
use super::SessionSnapshot;
use super::apply_topic_route_shell_patch;
use super::bootstrap_planner;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct BootstrapTopicRouteReadStage {
    pub(super) routes: Vec<BootstrapTopicCandidateRoute>,
    pub(super) session_indices: Vec<usize>,
    pub(super) previously_active_topic_ids: Vec<TopicId>,
    pub(super) selected_existing_indices: BTreeSet<usize>,
    pub(super) merged_source_indices: BTreeSet<usize>,
    pub(super) merge_marker: Option<&'static str>,
    pub(super) split_marker: Option<&'static str>,
    pub(super) has_evidence: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct BootstrapTopicRouteApplyStage {
    pub(super) routes: Vec<BootstrapTopicCandidateRoute>,
    pub(super) session_indices: Vec<usize>,
    pub(super) selected_existing_indices: BTreeSet<usize>,
    pub(super) merged_source_indices: BTreeSet<usize>,
    pub(super) outcome: BootstrapTopicRouteOutcome,
}

pub(super) fn prepare_bootstrap_topic_route_read_stage(
    projected_sessions: &[TopicSession],
    session_id: &str,
    query_text: Option<&str>,
    session: &SessionSnapshot,
    recent_entry_count: usize,
    transcript_matched_count: usize,
    durable_memory_hit_count: usize,
    summary_hit_count: usize,
    topic_score: f32,
    topic_limit: usize,
    learned_route_planning_signals: Vec<hepta_intelligence::LearnedSemanticRouterSignal>,
    semantic_router_id: Option<&str>,
) -> BootstrapTopicRouteReadStage {
    let effective_limit = topic_limit.max(1);
    let has_evidence = recent_entry_count > 0
        || transcript_matched_count > 0
        || durable_memory_hit_count > 0
        || summary_hit_count > 0;
    let session_indices = projected_sessions
        .iter()
        .enumerate()
        .filter(|(_, topic_session)| {
            topic_session
                .linked_surface_session_ids
                .iter()
                .any(|linked| linked.0 == session_id)
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let previously_active_topic_ids = session_indices
        .iter()
        .copied()
        .filter(|index| {
            matches!(
                projected_sessions[*index].status,
                TopicSessionStatus::Active
            )
        })
        .map(|index| projected_sessions[index].topic_id.clone())
        .collect::<Vec<_>>();
    let BootstrapTopicRoutePlan {
        routes,
        selected_existing_indices,
        merged_source_indices,
        merge_marker,
        split_marker,
    } = bootstrap_planner::plan_bootstrap_topic_routes(
        projected_sessions,
        &session_indices,
        session_id,
        query_text,
        session,
        effective_limit,
        topic_score,
        learned_route_planning_signals,
        semantic_router_id,
    );

    BootstrapTopicRouteReadStage {
        routes,
        session_indices,
        previously_active_topic_ids,
        selected_existing_indices,
        merged_source_indices,
        merge_marker,
        split_marker,
        has_evidence,
    }
}

pub(super) fn build_bootstrap_topic_route_apply_stage(
    read_stage: BootstrapTopicRouteReadStage,
    session_id: &str,
    fallback_topic_label: String,
    recent_entry_count: usize,
    transcript_matched_count: usize,
    durable_memory_hit_count: usize,
    summary_hit_count: usize,
) -> BootstrapTopicRouteApplyStage {
    let BootstrapTopicRouteReadStage {
        routes,
        session_indices,
        previously_active_topic_ids,
        selected_existing_indices,
        merged_source_indices,
        merge_marker,
        split_marker,
        has_evidence,
    } = read_stage;

    let active_topic_session_ids = routes
        .iter()
        .map(|route| route.topic_session_id.clone())
        .collect::<Vec<_>>();
    let created_topic_session_ids = routes
        .iter()
        .filter(|route| route.existing_index.is_none())
        .map(|route| route.topic_session_id.clone())
        .collect::<Vec<_>>();
    let revived_topic_session_ids = routes
        .iter()
        .filter(|route| route.existing_index.is_some() && !route.was_active)
        .map(|route| route.topic_session_id.clone())
        .collect::<Vec<_>>();
    let activation_scores = routes
        .iter()
        .map(|route| TopicActivationScore {
            topic_id: route.topic_id.clone(),
            topic_label: route.topic_label.clone(),
            score: route.topic_score,
            matched_terms: route.matched_terms.clone(),
            reason: Some(route.reason.clone()),
        })
        .collect::<Vec<_>>();
    let outcome = build_bootstrap_topic_route_outcome(
        session_id,
        &routes,
        &session_indices,
        &previously_active_topic_ids,
        &merged_source_indices,
        merge_marker,
        split_marker,
        activation_scores,
        active_topic_session_ids,
        created_topic_session_ids,
        revived_topic_session_ids,
        fallback_topic_label,
        has_evidence,
        recent_entry_count,
        transcript_matched_count,
        durable_memory_hit_count,
        summary_hit_count,
    );

    BootstrapTopicRouteApplyStage {
        routes,
        session_indices,
        selected_existing_indices,
        merged_source_indices,
        outcome,
    }
}

fn build_bootstrap_topic_route_outcome(
    session_id: &str,
    routes: &[BootstrapTopicCandidateRoute],
    session_indices: &[usize],
    previously_active_topic_ids: &[TopicId],
    merged_source_indices: &BTreeSet<usize>,
    merge_marker: Option<&'static str>,
    split_marker: Option<&'static str>,
    activation_scores: Vec<TopicActivationScore>,
    active_topic_session_ids: Vec<String>,
    created_topic_session_ids: Vec<String>,
    revived_topic_session_ids: Vec<String>,
    fallback_topic_label: String,
    has_evidence: bool,
    recent_entry_count: usize,
    transcript_matched_count: usize,
    durable_memory_hit_count: usize,
    summary_hit_count: usize,
) -> BootstrapTopicRouteOutcome {
    let route_outcome_draft =
        build_bootstrap_topic_route_outcome_draft(BootstrapTopicRouteOutcomeDraftInput {
            session_id,
            routes,
            session_indices,
            previously_active_topic_ids,
            merged_source_indices,
            merge_marker,
            split_marker,
            activation_scores: &activation_scores,
            active_topic_session_ids: &active_topic_session_ids,
            created_topic_session_ids: &created_topic_session_ids,
            revived_topic_session_ids: &revived_topic_session_ids,
            fallback_topic_label: &fallback_topic_label,
            has_evidence,
            recent_entry_count,
            transcript_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
        });

    let mut outcome = BootstrapTopicRouteOutcome {
        primary_topic_id: None,
        active_topic_session_ids,
        created_topic_session_ids: route_outcome_draft.output_created_topic_session_ids,
        revived_topic_session_ids,
        activation_scores,
        shift_event: TopicShiftEvent {
            kind: route_outcome_draft.shift_kind,
            from_topic_id: route_outcome_draft.shift_from_topic_id,
            to_topic_id: None,
            reason: None,
        },
        explanation: String::new(),
    };
    apply_topic_route_shell_patch(&mut outcome, &route_outcome_draft.route_shell_patch);
    outcome
}

use std::collections::BTreeMap;

use hepta_core::HeptaNeuron;
use hepta_core::IntuitionFeedbackOutcome;
use hepta_core::IntuitionFeedbackRecord;
use hepta_core::NeuronId;
use hepta_core::SessionId;
use hepta_core::TopicId;
use hepta_core::TopicLabel;
use hepta_core::TopicSession;
use hepta_core::TopicSessionStatus;

use super::*;

#[test]
fn outcome_weights_remain_closed_and_directional() {
    let cases = [
        (IntuitionFeedbackOutcome::Accepted, 0.12),
        (IntuitionFeedbackOutcome::ExecutedSuccess, 0.18),
        (IntuitionFeedbackOutcome::Corrected, 0.04),
        (IntuitionFeedbackOutcome::Ignored, -0.03),
        (IntuitionFeedbackOutcome::Rejected, -0.18),
        (IntuitionFeedbackOutcome::ExecutedFailed, -0.22),
        (IntuitionFeedbackOutcome::UserOverride, -0.10),
        (IntuitionFeedbackOutcome::ToolFailed, -0.08),
        (IntuitionFeedbackOutcome::UnsafeBlocked, -0.16),
    ];
    for (outcome, expected) in cases {
        assert_eq!(intuition_feedback_weight_delta(outcome), expected);
    }
}

#[test]
fn confidence_uses_only_matching_hydrated_feedback() {
    let matching = feedback(IntuitionFeedbackOutcome::ExecutedSuccess, 0.35);
    let mut unrelated = feedback(IntuitionFeedbackOutcome::Rejected, -0.35);
    unrelated.source_topic_ids = vec![TopicId("other-topic".into())];

    assert_eq!(
        estimate_intuition_feedback_confidence(
            &[matching, unrelated],
            &[TopicId("topic-a".into())],
            &[],
            None,
            None,
        ),
        0.85
    );
}

#[test]
fn topic_reducer_updates_only_the_session_feasible_set() {
    let mut record = feedback(IntuitionFeedbackOutcome::Accepted, 0.12);
    record.user_intent = "alpha beta gamma delta epsilon zeta eta theta iota".into();
    record.source_topic_ids = vec![TopicId("topic-a".into())];

    let mut sessions = vec![
        topic_session("target", "topic-a", "alpha", TopicSessionStatus::Dormant),
        topic_session("active", "topic-b", "alpha", TopicSessionStatus::Active),
        topic_session("dormant", "topic-b", "alpha", TopicSessionStatus::Dormant),
        topic_session(
            "other-session",
            "topic-a",
            "other",
            TopicSessionStatus::Active,
        ),
    ];
    let updated = apply_intuition_feedback_to_topic_sessions("alpha", &record, &mut sessions);

    assert_eq!(updated, 2);
    for session in &sessions[..2] {
        assert_eq!(
            session
                .entities
                .get(INTUITION_FEEDBACK_LEARNER_COUNT_KEY)
                .map(String::as_str),
            Some("1")
        );
        assert_eq!(
            session
                .entities
                .get(INTUITION_FEEDBACK_LEARNER_NET_DELTA_KEY)
                .map(String::as_str),
            Some("0.1200")
        );
        assert_eq!(
            session
                .entities
                .get(INTUITION_FEEDBACK_LEARNER_LAST_OUTCOME_KEY)
                .map(String::as_str),
            Some("accepted")
        );
        assert_eq!(
            session
                .entities
                .keys()
                .filter(|key| key.starts_with(BOOTSTRAP_SEMANTIC_HINT_PREFIX))
                .count(),
            MAX_BOOTSTRAP_SEMANTIC_HINTS
        );
    }
    assert!(sessions[2].entities.is_empty());
    assert!(sessions[3].entities.is_empty());
}

#[test]
fn neuron_reducer_returns_only_targeted_persistence_replacements() {
    let mut positive = feedback(IntuitionFeedbackOutcome::Accepted, 0.12);
    positive.skill_id = Some("skill-a".into());
    positive.source_neuron_ids = vec![NeuronId("neuron-a".into())];
    positive.reason = Some("accepted".into());

    let replacements = reduce_intuition_feedback_neurons(
        "alpha",
        &positive,
        vec![neuron("neuron-a", "topic-a"), neuron("neuron-b", "topic-b")],
    );
    assert_eq!(replacements.len(), 1);
    let replacement = &replacements[0];
    assert_eq!(replacement.neuron_revision, 2);
    assert_eq!(
        replacement.stable_preferences,
        vec!["feedback-confirmed:skill-a"]
    );
    assert_eq!(
        replacement.source_evidence_digest.as_deref(),
        Some("feedback:alpha:decision-1")
    );
    assert_eq!(
        replacement
            .entity_state
            .get(INTUITION_FEEDBACK_LEARNER_COUNT_KEY)
            .map(String::as_str),
        Some("1")
    );

    let mut negative = feedback(IntuitionFeedbackOutcome::Rejected, -0.18);
    negative.source_topic_ids = vec![TopicId("topic-a".into())];
    negative.reason = Some("wrong tool".into());
    let second = reduce_intuition_feedback_neurons("alpha", &negative, replacements);
    assert_eq!(second[0].neuron_revision, 3);
    assert_eq!(second[0].open_loops, vec!["feedback-review:wrong tool"]);
    assert_eq!(
        second[0].source_evidence_digest.as_deref(),
        Some("feedback:alpha:decision-1")
    );
}

fn feedback(outcome: IntuitionFeedbackOutcome, weight_delta: f32) -> IntuitionFeedbackRecord {
    IntuitionFeedbackRecord {
        decision_id: Some("decision-1".into()),
        surface_session_id: SessionId("alpha".into()),
        user_intent: "alpha memory review".into(),
        outcome,
        skill_id: None,
        workflow_id: None,
        source_topic_ids: vec![TopicId("topic-a".into())],
        source_neuron_ids: Vec::new(),
        weight_delta,
        observed_outcome: None,
        latency_ms: None,
        cost: None,
        user_correction: None,
        confidence_before: None,
        confidence_after: None,
        reason: None,
        created_at_unix_ms: 1,
    }
}

fn topic_session(
    id: &str,
    topic_id: &str,
    linked_session: &str,
    status: TopicSessionStatus,
) -> TopicSession {
    TopicSession {
        topic_session_id: id.into(),
        topic_id: TopicId(topic_id.into()),
        topic_label: TopicLabel(topic_id.into()),
        topic_embedding: None,
        linked_surface_session_ids: vec![SessionId(linked_session.into())],
        linked_transcript_spans: Vec::new(),
        open_loops: Vec::new(),
        entities: BTreeMap::new(),
        graph_edges: Vec::new(),
        durable_memory_refs: Vec::new(),
        status,
        created_at_unix_ms: 1,
        last_active_unix_ms: 1,
    }
}

fn neuron(neuron_id: &str, topic_id: &str) -> HeptaNeuron {
    HeptaNeuron {
        neuron_id: NeuronId(neuron_id.into()),
        topic_id: TopicId(topic_id.into()),
        topic_label: TopicLabel(topic_id.into()),
        topic_embedding_centroid: None,
        linked_session_ids: vec![SessionId("alpha".into())],
        linked_topic_session_ids: Vec::new(),
        important_transcript_spans: Vec::new(),
        promoted_memory_refs: Vec::new(),
        entity_state: BTreeMap::new(),
        stable_preferences: Vec::new(),
        open_loops: Vec::new(),
        skill_priors: Vec::new(),
        workflow_priors: Vec::new(),
        links: Vec::new(),
        neuron_revision: 1,
        compression_policy_version: "bootstrap-v1".into(),
        source_evidence_digest: None,
        last_refresh_reason: None,
        staleness_score: 0.2,
        merged_from: Vec::new(),
        split_from: Vec::new(),
        supersedes: Vec::new(),
        confidence: 0.5,
        freshness: 0.5,
        last_revalidated_unix_ms: 1,
    }
}

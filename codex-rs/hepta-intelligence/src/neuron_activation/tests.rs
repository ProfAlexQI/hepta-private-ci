use std::collections::BTreeMap;

use hepta_core::LinkPolarity;
use hepta_core::NeuronLink;
use hepta_core::SessionId;
use hepta_core::TopicGraphEdge;
use hepta_core::TopicGraphEdgeKind;
use hepta_core::TopicId;
use hepta_core::TopicLabel;
use hepta_core::TopicSessionStatus;
use hepta_core::TranscriptRange;

use super::*;

#[test]
fn direct_activation_matches_the_complete_cognitive_object() {
    let topic_sessions = vec![topic_session("session-a", "topic-a", "Topic A")];
    let neurons = vec![neuron("neuron-a", "topic-a", "Topic A", 1)];
    let active_topic_session_ids = vec!["session-a".to_string()];

    let actual = compute_neuron_activations(NeuronActivationInput {
        query_text: Some("topic a"),
        topic_sessions: &topic_sessions,
        neurons: &neurons,
        active_topic_session_ids: &active_topic_session_ids,
        activation_scores: &[],
        evidence_counts: NeuronActivationEvidenceCounts {
            recent_entry_count: 2,
            transcript_matched_count: 2,
            durable_memory_hit_count: 1,
            summary_hit_count: 1,
        },
        limit: 1,
    });

    assert_eq!(
        actual,
        vec![NeuronActivation {
            neuron_id: NeuronId("neuron-a".into()),
            topic_id: TopicId("topic-a".into()),
            direct_score: 1.0,
            propagated_score: 0.0,
            inhibition_score: 0.0,
            final_score: 1.0,
            source_topic_session_ids: vec!["session-a".into()],
            source_neuron_ids: Vec::new(),
            source_transcript_spans: vec![transcript_span("surface-a", 1)],
            source_link_kinds: Vec::new(),
            source_link_reasons: Vec::new(),
            reason: Some(
                "bootstrap direct activation via routed topic session 'session-a' for compressed neuron 'Topic A' with 0 open loops, 0 durable refs, and 0 prior(s); no additional propagated activation fired yet".into(),
            ),
        }]
    );
}

#[test]
fn propagation_matches_typed_graph_and_compressed_link_objects() {
    let mut session_a = topic_session("session-a", "topic-a", "Topic A");
    session_a.graph_edges.push(TopicGraphEdge {
        target_topic_session_id: "session-b".into(),
        kind: TopicGraphEdgeKind::CoActivation,
        relation: Some("co_activation".into()),
        weight: 0.70,
        evidence_count: 1,
        last_confirmed_unix_ms: Some(10),
    });
    let topic_sessions = vec![session_a, topic_session("session-b", "topic-b", "Topic B")];

    let neuron_a = neuron("persisted-neuron-alpha", "topic-a", "Topic A", 1);
    let mut neuron_b = neuron("persisted-neuron-beta", "topic-b", "Topic B", 2);
    neuron_b.links.push(NeuronLink {
        target_neuron_id: NeuronId("persisted-neuron-alpha".into()),
        kind: NeuronLinkKind::CausalDependency,
        relation: Some("supports".into()),
        polarity: LinkPolarity::Excitatory,
        directional: true,
        strength: 0.40,
        activation_decay: 1.0,
        evidence_count: 1,
        last_confirmed_unix_ms: Some(10),
    });
    let neurons = vec![neuron_a, neuron_b];
    let active_topic_session_ids = vec!["session-a".to_string(), "session-b".to_string()];
    let activation_scores = vec![
        activation_score("topic-a", "Topic A", 0.8),
        activation_score("topic-b", "Topic B", 0.6),
    ];
    let span_a = transcript_span("surface-a", 1);
    let span_b = transcript_span("surface-b", 2);

    let actual = compute_neuron_activations(NeuronActivationInput {
        query_text: Some("topic a and topic b"),
        topic_sessions: &topic_sessions,
        neurons: &neurons,
        active_topic_session_ids: &active_topic_session_ids,
        activation_scores: &activation_scores,
        evidence_counts: NeuronActivationEvidenceCounts::default(),
        limit: 2,
    });

    let propagated_into_a = 0.6 * 0.40 * 0.20;
    let propagated_into_b = 0.8 * 0.46 * 0.20;
    assert_eq!(
        actual,
        vec![
            NeuronActivation {
                neuron_id: NeuronId("persisted-neuron-alpha".into()),
                topic_id: TopicId("topic-a".into()),
                direct_score: 0.8,
                propagated_score: propagated_into_a,
                inhibition_score: 0.0,
                final_score: 0.8 + propagated_into_a,
                source_topic_session_ids: vec!["session-a".into(), "session-b".into()],
                source_neuron_ids: vec![NeuronId("persisted-neuron-beta".into())],
                source_transcript_spans: vec![span_b.clone(), span_a.clone()],
                source_link_kinds: vec![NeuronLinkKind::CausalDependency],
                source_link_reasons: vec![
                    "compressed neuron link 'supports' into 'Topic A' strength 0.40 via session-b"
                        .into(),
                ],
                reason: Some(
                    "bootstrap direct activation via routed topic session 'session-a' for compressed neuron 'Topic A' with 0 open loops, 0 durable refs, and 0 prior(s), plus propagated activation 0.05 from 1 linked neuron(s)".into(),
                ),
            },
            NeuronActivation {
                neuron_id: NeuronId("persisted-neuron-beta".into()),
                topic_id: TopicId("topic-b".into()),
                direct_score: 0.6,
                propagated_score: propagated_into_b,
                inhibition_score: 0.0,
                final_score: 0.6 + propagated_into_b,
                source_topic_session_ids: vec!["session-b".into(), "session-a".into()],
                source_neuron_ids: vec![NeuronId("persisted-neuron-alpha".into())],
                source_transcript_spans: vec![span_b, span_a],
                source_link_kinds: vec![NeuronLinkKind::WorkflowAdjacency],
                source_link_reasons: vec![
                    "bootstrap stored co-activation edge into 'Topic B' strength 0.70 via session-a"
                        .into(),
                ],
                reason: Some(
                    "bootstrap direct activation via routed topic session 'session-b' for compressed neuron 'Topic B' with 0 open loops, 0 durable refs, and 0 prior(s), plus propagated activation 0.07 from 1 linked neuron(s)".into(),
                ),
            },
        ]
    );
}

#[test]
fn contrast_inhibition_matches_the_complete_cognitive_objects() {
    let topic_sessions = vec![
        topic_session("session-a", "topic-a", "Topic A"),
        topic_session("session-b", "topic-b", "Topic B"),
    ];
    let mut neuron_a = neuron("neuron-a", "topic-a", "Topic A", 1);
    neuron_a.links.push(NeuronLink {
        target_neuron_id: NeuronId("neuron-b".into()),
        kind: NeuronLinkKind::Conflict,
        relation: Some("excludes".into()),
        polarity: LinkPolarity::Inhibitory,
        directional: true,
        strength: 0.40,
        activation_decay: 0.9,
        evidence_count: 1,
        last_confirmed_unix_ms: Some(10),
    });
    let neurons = vec![neuron_a, neuron("neuron-b", "topic-b", "Topic B", 2)];
    let active_topic_session_ids = vec!["session-a".to_string(), "session-b".to_string()];
    let activation_scores = vec![
        activation_score("topic-a", "Topic A", 0.8),
        activation_score("topic-b", "Topic B", 0.6),
    ];
    let span_a = transcript_span("surface-a", 1);
    let span_b = transcript_span("surface-b", 2);

    let actual = compute_neuron_activations(NeuronActivationInput {
        query_text: Some("topic a but not topic b"),
        topic_sessions: &topic_sessions,
        neurons: &neurons,
        active_topic_session_ids: &active_topic_session_ids,
        activation_scores: &activation_scores,
        evidence_counts: NeuronActivationEvidenceCounts::default(),
        limit: 2,
    });

    let inhibition_score = 0.8 * 0.40 * 0.26;
    assert_eq!(
        actual,
        vec![
            NeuronActivation {
                neuron_id: NeuronId("neuron-a".into()),
                topic_id: TopicId("topic-a".into()),
                direct_score: 0.8,
                propagated_score: 0.0,
                inhibition_score: 0.0,
                final_score: 0.8,
                source_topic_session_ids: vec!["session-a".into()],
                source_neuron_ids: Vec::new(),
                source_transcript_spans: vec![span_a.clone()],
                source_link_kinds: Vec::new(),
                source_link_reasons: Vec::new(),
                reason: Some(
                    "bootstrap direct activation via routed topic session 'session-a' for compressed neuron 'Topic A' with 0 open loops, 0 durable refs, and 0 prior(s); no additional propagated activation fired yet".into(),
                ),
            },
            NeuronActivation {
                neuron_id: NeuronId("neuron-b".into()),
                topic_id: TopicId("topic-b".into()),
                direct_score: 0.6,
                propagated_score: 0.0,
                inhibition_score,
                final_score: 0.6 - inhibition_score,
                source_topic_session_ids: vec!["session-b".into(), "session-a".into()],
                source_neuron_ids: vec![NeuronId("neuron-a".into())],
                source_transcript_spans: vec![span_b, span_a],
                source_link_kinds: vec![NeuronLinkKind::Inhibition],
                source_link_reasons: vec![
                    "bootstrap contrast ' but not ' followed compressed neuron inhibition 'excludes' into 'Topic B' strength 0.40 via session-a".into(),
                ],
                reason: Some(
                    "bootstrap direct activation via routed topic session 'session-b' for compressed neuron 'Topic B' with 0 open loops, 0 durable refs, and 0 prior(s), then inhibitory suppression 0.08 from 1 linked neuron(s)".into(),
                ),
            },
        ]
    );
}

#[test]
fn zero_limit_returns_the_complete_empty_result() {
    let topic_sessions = vec![topic_session("session-a", "topic-a", "Topic A")];
    let neurons = vec![neuron("neuron-a", "topic-a", "Topic A", 1)];
    let active_topic_session_ids = vec!["session-a".to_string()];

    assert_eq!(
        compute_neuron_activations(NeuronActivationInput {
            query_text: Some("topic a"),
            topic_sessions: &topic_sessions,
            neurons: &neurons,
            active_topic_session_ids: &active_topic_session_ids,
            activation_scores: &[],
            evidence_counts: NeuronActivationEvidenceCounts {
                recent_entry_count: 2,
                transcript_matched_count: 2,
                durable_memory_hit_count: 1,
                summary_hit_count: 1,
            },
            limit: 0,
        }),
        Vec::<NeuronActivation>::new()
    );
}

fn topic_session(id: &str, topic_id: &str, label: &str) -> TopicSession {
    TopicSession {
        topic_session_id: id.into(),
        topic_id: TopicId(topic_id.into()),
        topic_label: TopicLabel(label.into()),
        topic_embedding: None,
        linked_surface_session_ids: Vec::new(),
        linked_transcript_spans: Vec::new(),
        open_loops: Vec::new(),
        entities: BTreeMap::new(),
        graph_edges: Vec::new(),
        durable_memory_refs: Vec::new(),
        status: TopicSessionStatus::Active,
        created_at_unix_ms: 1,
        last_active_unix_ms: 1,
    }
}

fn neuron(neuron_id: &str, topic_id: &str, label: &str, transcript_sequence: u64) -> HeptaNeuron {
    HeptaNeuron {
        neuron_id: NeuronId(neuron_id.into()),
        topic_id: TopicId(topic_id.into()),
        topic_label: TopicLabel(label.into()),
        topic_embedding_centroid: None,
        linked_session_ids: Vec::new(),
        linked_topic_session_ids: Vec::new(),
        important_transcript_spans: vec![transcript_span(
            &format!(
                "surface-{label}",
                label = topic_id.trim_start_matches("topic-")
            ),
            transcript_sequence,
        )],
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
        staleness_score: 0.0,
        merged_from: Vec::new(),
        split_from: Vec::new(),
        supersedes: Vec::new(),
        confidence: 1.0,
        freshness: 1.0,
        last_revalidated_unix_ms: 1,
    }
}

fn activation_score(topic_id: &str, label: &str, score: f32) -> TopicActivationScore {
    TopicActivationScore {
        topic_id: TopicId(topic_id.into()),
        topic_label: TopicLabel(label.into()),
        score,
        matched_terms: Vec::new(),
        reason: None,
    }
}

fn transcript_span(session_id: &str, sequence: u64) -> TranscriptSpanRef {
    TranscriptSpanRef {
        session_id: SessionId(session_id.into()),
        range: TranscriptRange {
            start_sequence: sequence,
            end_sequence: sequence,
        },
        reason: Some("query_match".into()),
    }
}

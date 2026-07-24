//! Pure compatibility reducer for legacy intuition feedback.
//!
//! Runtime owns hydration and persistence. Intelligence owns the deterministic
//! scoring and state-transition rules. This compatibility learner is not the
//! Architecture V2 preference authority: it accepts legacy runtime feedback
//! records and must not be connected to `PreferenceTransition` CAS state.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use hepta_core::HeptaNeuron;
use hepta_core::IntuitionFeedbackOutcome;
use hepta_core::IntuitionFeedbackRecord;
use hepta_core::NeuronId;
use hepta_core::TopicId;
use hepta_core::TopicSession;
use hepta_core::TopicSessionStatus;

use crate::SEMANTIC_ROUTER_LAST_SIGNAL_KEY;
use crate::SEMANTIC_ROUTER_LEARNED_KEY;
use crate::SEMANTIC_ROUTER_NET_DELTA_KEY;
use crate::compute_intuition_feedback_delta;
use crate::format_intuition_feedback_outcome;
use crate::learned_semantic_terms_for_feedback;

/// Durable entity counter used by the compatibility learner.
pub const INTUITION_FEEDBACK_LEARNER_COUNT_KEY: &str = "feedback.learning.count";
/// Durable accumulated weight used by the compatibility learner.
pub const INTUITION_FEEDBACK_LEARNER_NET_DELTA_KEY: &str = "feedback.learning.net_weight_delta";
/// Durable last-outcome projection used by the compatibility learner.
pub const INTUITION_FEEDBACK_LEARNER_LAST_OUTCOME_KEY: &str = "feedback.learning.last_outcome";

const BOOTSTRAP_SEMANTIC_HINT_PREFIX: &str = "bootstrap.semantic_hint:";
const MAX_BOOTSTRAP_SEMANTIC_HINTS: usize = 8;

/// Returns the compatibility weight for one legacy intuition outcome.
pub fn intuition_feedback_weight_delta(outcome: IntuitionFeedbackOutcome) -> f32 {
    match outcome {
        IntuitionFeedbackOutcome::Accepted => 0.12,
        IntuitionFeedbackOutcome::ExecutedSuccess => 0.18,
        IntuitionFeedbackOutcome::Corrected => 0.04,
        IntuitionFeedbackOutcome::Ignored => -0.03,
        IntuitionFeedbackOutcome::Rejected => -0.18,
        IntuitionFeedbackOutcome::ExecutedFailed => -0.22,
        IntuitionFeedbackOutcome::UserOverride => -0.10,
        IntuitionFeedbackOutcome::ToolFailed => -0.08,
        IntuitionFeedbackOutcome::UnsafeBlocked => -0.16,
    }
}

/// Estimates the next legacy intuition confidence from hydrated feedback.
pub fn estimate_intuition_feedback_confidence(
    records: &[IntuitionFeedbackRecord],
    source_topic_ids: &[TopicId],
    source_neuron_ids: &[NeuronId],
    skill_id: Option<&str>,
    workflow_id: Option<&str>,
) -> f32 {
    let delta = compute_intuition_feedback_delta(
        records,
        source_topic_ids.first(),
        source_neuron_ids.first(),
        skill_id,
        workflow_id,
    );
    (0.50 + delta).clamp(0.0, 1.0)
}

/// Applies one legacy feedback record to hydrated topic-session state.
///
/// The caller retains ownership of locking and persistence. The returned count
/// is the number of in-memory topic sessions changed by this reduction.
pub fn apply_intuition_feedback_to_topic_sessions(
    session_id: &str,
    record: &IntuitionFeedbackRecord,
    topic_sessions: &mut [TopicSession],
) -> usize {
    let learned_terms = learned_semantic_terms_for_feedback(record);
    let source_topic_ids = record
        .source_topic_ids
        .iter()
        .map(|topic_id| topic_id.0.as_str())
        .collect::<BTreeSet<_>>();
    let mut update_count = 0;

    for topic_session in topic_sessions.iter_mut().filter(|topic_session| {
        topic_session
            .linked_surface_session_ids
            .iter()
            .any(|linked| linked.0 == session_id)
            && (source_topic_ids.is_empty()
                || source_topic_ids.contains(topic_session.topic_id.0.as_str())
                || matches!(topic_session.status, TopicSessionStatus::Active))
    }) {
        merge_semantic_hints(&mut topic_session.entities, &learned_terms);
        increment_entity_usize(
            &mut topic_session.entities,
            INTUITION_FEEDBACK_LEARNER_COUNT_KEY,
            1,
        );
        accumulate_entity_f32(
            &mut topic_session.entities,
            INTUITION_FEEDBACK_LEARNER_NET_DELTA_KEY,
            record.weight_delta,
        );
        accumulate_entity_f32(
            &mut topic_session.entities,
            SEMANTIC_ROUTER_NET_DELTA_KEY,
            record.weight_delta,
        );
        topic_session.entities.insert(
            INTUITION_FEEDBACK_LEARNER_LAST_OUTCOME_KEY.into(),
            format_intuition_feedback_outcome(record.outcome).into(),
        );
        topic_session
            .entities
            .insert(SEMANTIC_ROUTER_LEARNED_KEY.into(), "true".into());
        if let Some(term) = learned_terms.first() {
            topic_session
                .entities
                .insert(SEMANTIC_ROUTER_LAST_SIGNAL_KEY.into(), term.clone());
        }
        update_count += 1;
    }

    update_count
}

/// Reduces hydrated neurons and returns only the neurons requiring persistence.
///
/// Runtime is responsible for hydrating the session-scoped neuron snapshot and
/// atomically persisting the returned replacements.
pub fn reduce_intuition_feedback_neurons(
    session_id: &str,
    record: &IntuitionFeedbackRecord,
    neurons: impl IntoIterator<Item = HeptaNeuron>,
) -> Vec<HeptaNeuron> {
    let learned_terms = learned_semantic_terms_for_feedback(record);
    let source_topic_ids = record
        .source_topic_ids
        .iter()
        .map(|topic_id| topic_id.0.as_str())
        .collect::<BTreeSet<_>>();
    let source_neuron_ids = record
        .source_neuron_ids
        .iter()
        .map(|neuron_id| neuron_id.0.as_str())
        .collect::<BTreeSet<_>>();
    let mut updated = Vec::new();

    for mut neuron in neurons {
        let target_neuron = source_neuron_ids.contains(neuron.neuron_id.0.as_str());
        let target_topic = source_topic_ids.contains(neuron.topic_id.0.as_str());
        if !target_neuron && !target_topic {
            continue;
        }

        let confidence_delta = record.weight_delta * 0.35;
        let freshness_delta = record.weight_delta * 0.25;
        neuron.confidence = (neuron.confidence + confidence_delta).clamp(0.0, 1.0);
        neuron.freshness = (neuron.freshness + freshness_delta).clamp(0.0, 1.0);
        neuron.staleness_score = (neuron.staleness_score - freshness_delta).clamp(0.0, 1.0);
        neuron.neuron_revision = neuron.neuron_revision.saturating_add(1);
        neuron.last_refresh_reason = Some(format!(
            "feedback-learning:{}:{:+.2}",
            format_intuition_feedback_outcome(record.outcome),
            record.weight_delta,
        ));
        neuron.source_evidence_digest.get_or_insert_with(|| {
            format!(
                "feedback:{}:{}",
                session_id,
                record.decision_id.as_deref().unwrap_or("untracked")
            )
        });
        increment_entity_usize(
            &mut neuron.entity_state,
            INTUITION_FEEDBACK_LEARNER_COUNT_KEY,
            1,
        );
        accumulate_entity_f32(
            &mut neuron.entity_state,
            INTUITION_FEEDBACK_LEARNER_NET_DELTA_KEY,
            record.weight_delta,
        );
        neuron
            .entity_state
            .insert(SEMANTIC_ROUTER_LEARNED_KEY.into(), "true".into());
        if !learned_terms.is_empty() {
            neuron.entity_state.insert(
                SEMANTIC_ROUTER_LAST_SIGNAL_KEY.into(),
                learned_terms
                    .iter()
                    .take(4)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if record.weight_delta > 0.0 {
            let preference = format!(
                "feedback-confirmed:{}",
                record
                    .skill_id
                    .as_deref()
                    .or(record.workflow_id.as_deref())
                    .unwrap_or("intuition")
            );
            if !neuron.stable_preferences.contains(&preference) {
                neuron.stable_preferences.push(preference);
            }
        } else if let Some(reason) = record.reason.as_deref() {
            let open_loop = format!("feedback-review:{reason}");
            if !neuron.open_loops.contains(&open_loop) {
                neuron.open_loops.push(open_loop);
            }
        }

        updated.push(neuron);
    }

    updated
}

fn merge_semantic_hints(entities: &mut BTreeMap<String, String>, semantic_hints: &[String]) {
    for hint in semantic_hints {
        entities.insert(semantic_hint_key(hint), hint.clone());
    }

    let semantic_hint_keys = entities
        .keys()
        .filter(|key| key.starts_with(BOOTSTRAP_SEMANTIC_HINT_PREFIX))
        .cloned()
        .collect::<Vec<_>>();
    for key in semantic_hint_keys
        .into_iter()
        .skip(MAX_BOOTSTRAP_SEMANTIC_HINTS)
    {
        entities.remove(&key);
    }
}

fn semantic_hint_key(term: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;
    for character in term.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash && !slug.is_empty() {
            slug.push('-');
            last_was_dash = true;
        }
    }
    let slug = slug.trim_matches('-');
    format!(
        "{BOOTSTRAP_SEMANTIC_HINT_PREFIX}{}",
        if slug.is_empty() { "topic" } else { slug }
    )
}

fn increment_entity_usize(entities: &mut BTreeMap<String, String>, key: &str, amount: usize) {
    let next = entities
        .get(key)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0)
        .saturating_add(amount);
    entities.insert(key.to_string(), next.to_string());
}

fn accumulate_entity_f32(entities: &mut BTreeMap<String, String>, key: &str, amount: f32) {
    let next = entities
        .get(key)
        .and_then(|value| value.parse::<f32>().ok())
        .unwrap_or(0.0)
        + amount;
    entities.insert(key.to_string(), format!("{next:.4}"));
}

#[cfg(test)]
mod tests;

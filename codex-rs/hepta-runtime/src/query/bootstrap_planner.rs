use std::collections::BTreeMap;
use std::collections::BTreeSet;

use hepta_core::TopicLabel;
use hepta_core::TopicSession;
use hepta_core::TopicSessionStatus;

use super::BOOTSTRAP_SEMANTIC_HINT_PREFIX;
use super::BootstrapTopicCandidateRoute;
use super::BootstrapTopicGraphRouteCandidate;
use super::BootstrapTopicRoutePlan;
use super::MAX_BOOTSTRAP_SEMANTIC_HINTS;
use super::SessionSnapshot;
use super::allocate_bootstrap_topic_id;
use super::allocate_bootstrap_topic_session_id;
use super::bootstrap_topic_graph_edge;
use super::bootstrap_topic_graph_edge_relation;
use super::detect_bootstrap_merge_marker;
use super::detect_bootstrap_split_marker;
use super::slugify_identifier;
use super::topic_label_for_session;

pub(super) fn plan_bootstrap_topic_routes(
    existing_sessions: &[TopicSession],
    session_indices: &[usize],
    session_id: &str,
    query_text: Option<&str>,
    session: &SessionSnapshot,
    effective_limit: usize,
    topic_score: f32,
    learned_route_planning_signals: Vec<hepta_intelligence::LearnedSemanticRouterSignal>,
    semantic_router_id: Option<&str>,
) -> BootstrapTopicRoutePlan {
    let candidate_labels = bootstrap_candidate_topic_labels(query_text, session, effective_limit);
    let merge_marker = detect_bootstrap_merge_marker(query_text);
    let split_marker = detect_bootstrap_split_marker(query_text);
    let implicit_routes = (candidate_labels.len() == 1)
        .then(|| {
            infer_bootstrap_implicit_topic_routes(
                existing_sessions,
                session_indices,
                query_text,
                effective_limit,
                topic_score,
            )
        })
        .unwrap_or_default();

    let mut materializer = RuntimeBootstrapTopicRouteMaterializer {
        existing_sessions,
        session_indices,
        session_id,
        topic_score,
        effective_limit,
    };
    let router_registry = hepta_intelligence::SemanticRouterRegistry::new();
    let router = semantic_router_id
        .map(|router_id| router_registry.select(Some(router_id)))
        .unwrap_or_else(|| {
            router_registry.select_for_learned_signal_count(learned_route_planning_signals.len())
        });
    let router_input = hepta_intelligence::BootstrapSemanticRouterInput {
        implicit_routes,
        candidate_labels,
        merge_marker,
        split_marker,
        limit: effective_limit,
        learned_signals: learned_route_planning_signals,
    };
    let planner_outcome = router.route(router_input, &mut materializer);

    BootstrapTopicRoutePlan {
        routes: planner_outcome.routes,
        selected_existing_indices: planner_outcome.selected_existing_indices,
        merged_source_indices: planner_outcome.merged_source_indices,
        merge_marker,
        split_marker,
    }
}

struct RuntimeBootstrapTopicRouteMaterializer<'a> {
    existing_sessions: &'a [TopicSession],
    session_indices: &'a [usize],
    session_id: &'a str,
    topic_score: f32,
    effective_limit: usize,
}

impl hepta_intelligence::BootstrapTopicRouteMaterializer
    for RuntimeBootstrapTopicRouteMaterializer<'_>
{
    fn build_candidate_route(
        &mut self,
        selected_existing_indices: &BTreeSet<usize>,
        candidate_label: &str,
        has_prior_routes: bool,
    ) -> BootstrapTopicCandidateRoute {
        build_bootstrap_topic_candidate_route(
            self.existing_sessions,
            self.session_indices,
            selected_existing_indices,
            self.session_id,
            candidate_label,
            self.topic_score,
            has_prior_routes,
        )
    }

    fn build_merged_route(
        &mut self,
        routes: &[BootstrapTopicCandidateRoute],
        marker: &'static str,
    ) -> BootstrapTopicCandidateRoute {
        build_bootstrap_merged_topic_route(
            self.existing_sessions,
            self.session_id,
            routes,
            self.topic_score,
            marker,
        )
    }

    fn infer_graph_routes(
        &mut self,
        selected_existing_indices: &BTreeSet<usize>,
        routes: &[BootstrapTopicCandidateRoute],
    ) -> Vec<BootstrapTopicCandidateRoute> {
        bootstrap_topic_graph_routing::infer_bootstrap_topic_graph_routes(
            self.existing_sessions,
            self.session_indices,
            selected_existing_indices,
            routes,
            self.effective_limit,
        )
    }
}

pub(super) fn extract_semantic_terms(value: &str, limit: usize) -> Vec<String> {
    hepta_intelligence::extract_semantic_terms(value, limit)
}

fn bootstrap_candidate_topic_labels(
    query_text: Option<&str>,
    session: &SessionSnapshot,
    limit: usize,
) -> Vec<String> {
    hepta_intelligence::bootstrap_candidate_topic_labels(
        query_text,
        &topic_label_for_session(session),
        limit,
    )
}

fn infer_bootstrap_implicit_topic_routes(
    existing_sessions: &[TopicSession],
    session_indices: &[usize],
    query_text: Option<&str>,
    limit: usize,
    topic_score: f32,
) -> Vec<BootstrapTopicCandidateRoute> {
    let query = match query_text.map(str::trim).filter(|query| !query.is_empty()) {
        Some(query) => query,
        None => return Vec::new(),
    };
    if limit <= 1 {
        return Vec::new();
    }

    let query_terms = extract_semantic_terms(query, 12);
    if query_terms.len() < 4 {
        return Vec::new();
    }

    let matches = hepta_intelligence::select_bootstrap_implicit_topic_match_candidates(
        session_indices.iter().copied().map(|index| {
            let topic_session = &existing_sessions[index];
            let features =
                bootstrap_candidate_matching::compute_topic_match_features(query, topic_session);
            hepta_intelligence::BootstrapImplicitTopicMatchCandidate {
                index,
                score: features.score,
                matched_terms: features.matched_terms,
                was_active: matches!(topic_session.status, TopicSessionStatus::Active),
                last_active_unix_ms: topic_session.last_active_unix_ms,
            }
        }),
        limit,
        0.52,
        2,
    );

    matches
            .into_iter()
            .map(|match_candidate| {
                let index = match_candidate.index;
                let score = match_candidate.score;
                let overlap_terms = match_candidate.matched_terms;
                let topic_session = &existing_sessions[index];
                let was_active = matches!(topic_session.status, TopicSessionStatus::Active);
                let reason = if was_active {
                    format!(
                        "bootstrap router implicitly kept '{}' foregrounded from full-query semantic coverage {:.2}",
                        topic_session.topic_label.0, score,
                    )
                } else {
                    format!(
                        "bootstrap router implicitly revived '{}' from full-query semantic coverage {:.2}",
                        topic_session.topic_label.0, score,
                    )
                };

                BootstrapTopicCandidateRoute {
                    topic_id: topic_session.topic_id.clone(),
                    topic_label: topic_session.topic_label.clone(),
                    topic_session_id: topic_session.topic_session_id.clone(),
                    matched_terms: overlap_terms.into_iter().take(3).collect(),
                    semantic_hints:
                        bootstrap_candidate_matching::extract_bootstrap_semantic_hints_from_overlap(
                            query,
                            topic_session,
                            MAX_BOOTSTRAP_SEMANTIC_HINTS,
                        ),
                    topic_score: topic_score.max(score),
                    reason,
                    existing_index: Some(index),
                    was_active,
                    graph_routed: false,
                }
            })
            .collect()
}

fn build_bootstrap_merged_topic_route(
    existing_sessions: &[TopicSession],
    session_id: &str,
    source_routes: &[BootstrapTopicCandidateRoute],
    topic_score: f32,
    marker: &'static str,
) -> BootstrapTopicCandidateRoute {
    let mut labels = Vec::new();
    let mut seen = BTreeSet::new();
    let mut matched_terms = Vec::new();

    for route in source_routes {
        if seen.insert(route.topic_label.0.clone()) {
            labels.push(route.topic_label.0.clone());
        }
        for term in &route.matched_terms {
            if matched_terms.iter().all(|existing| existing != term) {
                matched_terms.push(term.clone());
                if matched_terms.len() >= 3 {
                    break;
                }
            }
        }
    }

    let merged_label = labels.join(" + ");
    let merged_slug = slugify_identifier(&merged_label);
    if let Some((index, topic_session)) = existing_sessions
        .iter()
        .enumerate()
        .find(|(_, topic_session)| slugify_identifier(&topic_session.topic_label.0) == merged_slug)
    {
        return BootstrapTopicCandidateRoute {
            topic_id: topic_session.topic_id.clone(),
            topic_label: topic_session.topic_label.clone(),
            topic_session_id: topic_session.topic_session_id.clone(),
            matched_terms,
            semantic_hints: Vec::new(),
            topic_score,
            reason: format!(
                "bootstrap router merged '{}' from {} source topic sessions via explicit merge signal '{}'",
                topic_session.topic_label.0,
                source_routes.len(),
                marker.trim(),
            ),
            existing_index: Some(index),
            was_active: matches!(topic_session.status, TopicSessionStatus::Active),
            graph_routed: false,
        };
    }

    let topic_id = allocate_bootstrap_topic_id(session_id, &merged_slug, true, existing_sessions);
    let topic_session_id =
        allocate_bootstrap_topic_session_id(session_id, &merged_slug, true, existing_sessions);

    BootstrapTopicCandidateRoute {
        topic_id,
        topic_label: TopicLabel(merged_label.clone()),
        topic_session_id,
        matched_terms,
        semantic_hints: Vec::new(),
        topic_score,
        reason: format!(
            "bootstrap router merged '{}' from {} source topic sessions via explicit merge signal '{}'",
            merged_label,
            source_routes.len(),
            marker.trim(),
        ),
        existing_index: None,
        was_active: false,
        graph_routed: false,
    }
}

fn build_bootstrap_topic_candidate_route(
    existing_sessions: &[TopicSession],
    session_indices: &[usize],
    selected_existing_indices: &BTreeSet<usize>,
    session_id: &str,
    candidate_label: &str,
    topic_score: f32,
    has_prior_routes: bool,
) -> BootstrapTopicCandidateRoute {
    let match_candidates = session_indices
        .iter()
        .copied()
        .filter(|index| !selected_existing_indices.contains(index))
        .filter_map(|index| {
            let features = bootstrap_candidate_matching::compute_topic_match_features(
                candidate_label,
                &existing_sessions[index],
            );
            (features.score > 0.0)
                .then_some(hepta_intelligence::BootstrapTopicMatchCandidate { index, features })
        });

    if let Some(match_candidate) =
        hepta_intelligence::select_bootstrap_topic_match_candidate(match_candidates, 0.55)
    {
        let selected_index = match_candidate.index;
        let features = match_candidate.features;
        let selected = &existing_sessions[selected_index];
        let was_active = matches!(selected.status, TopicSessionStatus::Active);
        let reason = if was_active {
            format!(
                "bootstrap router kept '{}' foregrounded with semantic term-overlap {:.2}",
                selected.topic_label.0, features.score,
            )
        } else {
            format!(
                "bootstrap router revived '{}' with semantic term-overlap {:.2}",
                selected.topic_label.0, features.score,
            )
        };

        return BootstrapTopicCandidateRoute {
            topic_id: selected.topic_id.clone(),
            topic_label: selected.topic_label.clone(),
            topic_session_id: selected.topic_session_id.clone(),
            matched_terms: features.matched_terms,
            semantic_hints:
                bootstrap_candidate_matching::extract_bootstrap_semantic_hints_for_match(
                    candidate_label,
                    selected,
                    MAX_BOOTSTRAP_SEMANTIC_HINTS,
                ),
            topic_score: topic_score.max(features.score),
            reason,
            existing_index: Some(selected_index),
            was_active,
            graph_routed: false,
        };
    }

    let candidate_slug = slugify_identifier(candidate_label);
    let has_existing_sessions = !session_indices.is_empty() || has_prior_routes;
    let topic_id = allocate_bootstrap_topic_id(
        session_id,
        &candidate_slug,
        has_existing_sessions,
        existing_sessions,
    );
    let topic_session_id = allocate_bootstrap_topic_session_id(
        session_id,
        &candidate_slug,
        has_existing_sessions,
        existing_sessions,
    );

    BootstrapTopicCandidateRoute {
        topic_id,
        topic_label: TopicLabel(candidate_label.to_string()),
        topic_session_id,
        matched_terms: extract_semantic_terms(candidate_label, 3),
        semantic_hints: Vec::new(),
        topic_score,
        reason: format!(
            "bootstrap router created '{}' because no matching topic session was found for session '{}'",
            candidate_label, session_id,
        ),
        existing_index: None,
        was_active: false,
        graph_routed: false,
    }
}

mod bootstrap_topic_graph_routing {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    use hepta_core::TopicSession;

    use super::BootstrapTopicCandidateRoute;
    use super::BootstrapTopicGraphRouteCandidate;
    use super::bootstrap_topic_graph_edge;
    use super::bootstrap_topic_graph_edge_relation;

    pub(super) fn infer_bootstrap_topic_graph_routes(
        existing_sessions: &[TopicSession],
        session_indices: &[usize],
        selected_existing_indices: &BTreeSet<usize>,
        routes: &[BootstrapTopicCandidateRoute],
        limit: usize,
    ) -> Vec<BootstrapTopicCandidateRoute> {
        if routes.is_empty() || routes.len() >= limit {
            return Vec::new();
        }

        let candidates = collect_bootstrap_topic_graph_route_candidates(
            existing_sessions,
            session_indices,
            selected_existing_indices,
            routes,
        );

        let ranked_target_indices = hepta_intelligence::rank_bootstrap_graph_route_candidates(
            candidates.iter().map(|candidate| {
                hepta_intelligence::BootstrapGraphRouteRankCandidate {
                    target_index: candidate.target_index,
                    strength: candidate.strength,
                    last_active_unix_ms: existing_sessions[candidate.target_index]
                        .last_active_unix_ms,
                }
            }),
            limit.saturating_sub(routes.len()),
        );
        let candidates_by_target_index = candidates
            .into_iter()
            .map(|candidate| (candidate.target_index, candidate))
            .collect::<BTreeMap<_, _>>();

        ranked_target_indices
            .into_iter()
            .filter_map(|target_index| candidates_by_target_index.get(&target_index).cloned())
            .map(|candidate| {
                let topic_session = &existing_sessions[candidate.target_index];
                BootstrapTopicCandidateRoute::from_graph_link(
                    topic_session,
                    candidate.target_index,
                    candidate.source_score,
                    hepta_intelligence::BootstrapTopicGraphLink {
                        strength: candidate.strength,
                        matched_terms: candidate.matched_terms,
                        reason: candidate.reason,
                    },
                )
            })
            .collect()
    }

    fn collect_bootstrap_topic_graph_route_candidates(
        existing_sessions: &[TopicSession],
        session_indices: &[usize],
        selected_existing_indices: &BTreeSet<usize>,
        routes: &[BootstrapTopicCandidateRoute],
    ) -> Vec<BootstrapTopicGraphRouteCandidate> {
        session_indices
            .iter()
            .copied()
            .filter(|index| !selected_existing_indices.contains(index))
            .filter_map(|target_index| {
                infer_bootstrap_topic_graph_route_candidate(existing_sessions, routes, target_index)
            })
            .collect()
    }

    fn infer_bootstrap_topic_graph_route_candidate(
        existing_sessions: &[TopicSession],
        routes: &[BootstrapTopicCandidateRoute],
        target_index: usize,
    ) -> Option<BootstrapTopicGraphRouteCandidate> {
        let target = &existing_sessions[target_index];
        let best = routes
            .iter()
            .filter_map(|route| {
                infer_bootstrap_topic_graph_link_for_target(existing_sessions, route, target)
            })
            .max_by(|left, right| left.1.total_cmp(&right.1));

        best.map(|(source_score, strength, matched_terms, reason)| {
            BootstrapTopicGraphRouteCandidate {
                target_index,
                source_score,
                strength,
                matched_terms,
                reason,
            }
        })
    }

    fn infer_bootstrap_topic_graph_link_for_target(
        existing_sessions: &[TopicSession],
        route: &BootstrapTopicCandidateRoute,
        target: &TopicSession,
    ) -> Option<(f32, f32, Vec<String>, String)> {
        let persisted = route.existing_index.and_then(|source_index| {
            infer_bootstrap_persisted_topic_graph_link(&existing_sessions[source_index], target)
        });
        let heuristic = infer_bootstrap_topic_graph_link(route, target);

        persisted
            .into_iter()
            .chain(heuristic)
            .map(|(strength, matched_terms, reason)| {
                (route.topic_score, strength, matched_terms, reason)
            })
            .max_by(|left, right| left.1.total_cmp(&right.1))
    }

    fn infer_bootstrap_persisted_topic_graph_link(
        source_topic_session: &TopicSession,
        target: &TopicSession,
    ) -> Option<(f32, Vec<String>, String)> {
        let edge = bootstrap_topic_graph_edge(source_topic_session, &target.topic_session_id)?;
        let link = hepta_intelligence::infer_bootstrap_persisted_topic_graph_link(
            &source_topic_session.topic_label.0,
            &target.topic_label.0,
            edge.kind,
            bootstrap_topic_graph_edge_relation(&edge),
            edge.weight,
        );

        Some((link.strength, link.matched_terms, link.reason))
    }

    fn infer_bootstrap_topic_graph_link(
        source_route: &BootstrapTopicCandidateRoute,
        target: &TopicSession,
    ) -> Option<(f32, Vec<String>, String)> {
        let link = hepta_intelligence::infer_bootstrap_heuristic_topic_graph_link(
            &source_route.topic_label.0,
            source_route.was_active,
            &source_route.reason,
            &target.topic_label.0,
            target.status,
        )?;

        Some((link.strength, link.matched_terms, link.reason))
    }
}

mod bootstrap_candidate_matching {
    use super::TopicSession;
    use super::slugify_identifier;

    pub(super) fn extract_bootstrap_semantic_hints_for_match(
        candidate_label: &str,
        topic_session: &TopicSession,
        limit: usize,
    ) -> Vec<String> {
        hepta_intelligence::extract_bootstrap_semantic_hints_for_match(
            candidate_label,
            topic_session,
            limit,
        )
    }

    pub(super) fn extract_bootstrap_semantic_hints_from_overlap(
        candidate_label: &str,
        topic_session: &TopicSession,
        limit: usize,
    ) -> Vec<String> {
        hepta_intelligence::extract_bootstrap_semantic_hints_from_overlap(
            candidate_label,
            topic_session,
            limit,
        )
    }

    pub(super) fn compute_topic_match_features(
        candidate_label: &str,
        topic_session: &TopicSession,
    ) -> hepta_intelligence::BootstrapTopicMatchFeatures {
        let candidate_slug = slugify_identifier(candidate_label);
        let topic_label_slug = slugify_identifier(&topic_session.topic_label.0);
        let features = hepta_intelligence::compute_bootstrap_topic_match_features(
            candidate_label,
            &candidate_slug,
            topic_session,
            &topic_label_slug,
        );

        features
    }
}

fn bootstrap_semantic_hint_key(term: &str) -> String {
    format!(
        "{}{}",
        BOOTSTRAP_SEMANTIC_HINT_PREFIX,
        slugify_identifier(term)
    )
}

pub(super) fn merge_bootstrap_topic_session_semantic_hints(
    entities: &mut BTreeMap<String, String>,
    semantic_hints: &[String],
) {
    for hint in semantic_hints {
        entities.insert(bootstrap_semantic_hint_key(hint), hint.clone());
    }

    let semantic_hint_keys = entities
        .keys()
        .filter(|key| key.starts_with(BOOTSTRAP_SEMANTIC_HINT_PREFIX))
        .cloned()
        .collect::<Vec<_>>();

    if semantic_hint_keys.len() <= MAX_BOOTSTRAP_SEMANTIC_HINTS {
        return;
    }

    for key in semantic_hint_keys
        .into_iter()
        .skip(MAX_BOOTSTRAP_SEMANTIC_HINTS)
    {
        entities.remove(&key);
    }
}

use std::collections::BTreeMap;

use hepta_core::{
    ContextRecallAvailability, HeptaError, IntuitionActionMode, MemoryRecord, MessageRole,
    NeuronActivation, RiskTier, SkillActivationDecision, TopicActivationScore, TopicSession,
    TopicShiftEvent, TranscriptEntry, TranscriptSpan, WorkflowPrior,
};
use hepta_intelligence::recall_evidence_summary;

use crate::events::{format_event_record, summarize_line};
use crate::query::{
    RuntimeContextRecallSlice, RuntimeIntelligenceEvalCase, RuntimeIntelligenceEvalOverview,
    RuntimeIntelligencePhase2Overview, RuntimeIntuitionCalibrationFeedback,
    RuntimeIntuitionCalibrationOverview, RuntimeIntuitionCalibrationTarget,
    RuntimeIntuitionOverview, RuntimeNeuronActivationOverview, RuntimeNeuronLifecycleOverview,
    RuntimeProvenanceOverview, RuntimeSessionActivityOverview, RuntimeSessionActivitySlice,
    RuntimeTopicRoutingOverview, RuntimeTopicSessionOverview, RuntimeTranscriptQueryOverview,
    RuntimeTranscriptQuerySessionTally,
};
use crate::{DoctorCheck, DoctorProviderProbe, DoctorStatus, RuntimeKernel, TurnRecord};

impl RuntimeKernel {
    pub async fn doctor_summary(&self) -> Result<Vec<String>, HeptaError> {
        let report = self.doctor_report().await?;
        let mut lines = vec![
            format!(
                "Hepta doctor: {}",
                doctor_status_label(report.overall_status)
            ),
            "- runtime kernel: ok".to_string(),
            format!(
                "- active model: {}/{}",
                report.active_model.provider, report.active_model.model
            ),
            format!("- registered providers: {}", report.registered_providers),
            format!("- registered tools: {}", report.registered_tools),
            format!("- active session: {}", report.active_session_id),
            format!("- sessions: {}", report.sessions),
            format!("- raw session records: {}", report.raw_session_records),
            format!("- memories: {}", report.memories),
            format!("- history entries: {}", report.history_entries),
            format!("- topic sessions: {}", report.total_topic_sessions),
            format!("- topic graph edges: {}", report.total_topic_graph_edges),
            format!(
                "- active topic sessions with transcript provenance: {}/{}",
                report.active_topic_sessions_with_transcript_provenance,
                report.active_topic_sessions
            ),
            format!(
                "- active topic sessions missing transcript provenance: {}",
                report.active_topic_sessions_missing_transcript_provenance
            ),
            format!(
                "- active session recall transcript evidence spans: {}",
                report.active_session_recall_transcript_evidence_spans
            ),
            format!(
                "- active session recall omitted items: {}",
                report.active_session_recall_omitted_items
            ),
            format!(
                "- active session intuition transcript evidence spans: {}",
                report.active_session_intuition_transcript_evidence_spans
            ),
            format!(
                "- active session intuition foreground topic sessions: {}",
                report.active_session_intuition_foreground_topic_sessions
            ),
            format!(
                "- pending approvals (active session): {}",
                report.active_session_pending_approvals
            ),
            format!(
                "- approval-scoped sessions: {}",
                report.approval_scoped_sessions
            ),
            "Provider probes:".to_string(),
        ];
        lines.extend(
            report
                .provider_probes
                .into_iter()
                .map(|probe| format!("  - {}", format_provider_probe(&probe))),
        );
        lines.push("Session integrity:".to_string());
        lines.extend(
            report
                .integrity_checks
                .into_iter()
                .map(|check| format!("  - {}", format_doctor_check(&check))),
        );
        Ok(lines)
    }

    pub fn activity_summary(
        &self,
        session_id: Option<&str>,
        history_limit: usize,
        event_limit: usize,
    ) -> Result<Vec<String>, HeptaError> {
        let activity = self.activity_slice(session_id, history_limit, event_limit)?;
        let scope = session_id
            .map(|session_id| format!("session {}", session_id))
            .unwrap_or_else(|| "all sessions".to_string());
        let mut lines = vec![
            format!("Runtime activity: {}", scope),
            format!("- recent history entries: {}", activity.history.len()),
            format!("- recent events: {}", activity.events.len()),
            "Recent history:".to_string(),
        ];

        if activity.history.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(activity.history.iter().map(format_turn_record));
        }

        lines.push("Recent events:".to_string());
        if activity.events.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(activity.events.iter().map(format_event_record));
        }

        Ok(lines)
    }

    pub fn session_activity_summary(
        &self,
        history_limit: usize,
        event_limit: usize,
    ) -> Result<Vec<String>, HeptaError> {
        let RuntimeSessionActivityOverview {
            sessions,
            active_sessions,
            archived_sessions,
            sessions_with_history,
            sessions_with_events,
            sessions_with_topic_state,
            total_topic_sessions,
            total_topic_graph_edges,
        } = self.session_activity_overview(history_limit, event_limit)?;
        let mut lines = vec![
            "Runtime session activity:".to_string(),
            format!("- sessions: {}", sessions.len()),
            format!("- active sessions: {}", active_sessions),
            format!("- archived sessions: {}", archived_sessions),
            format!("- sessions with recent history: {}", sessions_with_history),
            format!("- sessions with recent events: {}", sessions_with_events),
            format!("- sessions with topic state: {}", sessions_with_topic_state),
            format!("- total topic sessions: {}", total_topic_sessions),
            format!("- total topic graph edges: {}", total_topic_graph_edges),
            format!("- per-session history limit: {}", history_limit),
            format!("- per-session event limit: {}", event_limit),
        ];

        if sessions.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(sessions.iter().map(format_session_activity_slice));
        }

        Ok(lines)
    }

    pub fn event_digest_summary(&self, limit: usize) -> Result<Vec<String>, HeptaError> {
        let digest = self.event_digest(limit)?;
        let mut lines = vec![
            "Runtime event digest:".to_string(),
            format!("- recent events: {}", digest.recent_event_count()),
            format!("- event kinds: {}", digest.kind_count()),
            format!("- session scopes: {}", digest.session_scope_count()),
            format!("- limit: {}", format_limit(limit)),
        ];

        lines.extend(digest.summary_sections());
        Ok(lines)
    }

    pub fn transcript_query_summary(
        &self,
        session_id: Option<&str>,
        query: &str,
        limit: usize,
    ) -> Result<Vec<String>, HeptaError> {
        let RuntimeTranscriptQueryOverview {
            report,
            returned_entries,
            matched_sessions,
            sessions,
        } = self.transcript_query_overview(session_id, query, limit)?;
        let scope = session_id
            .map(|session_id| format!("session {}", session_id))
            .unwrap_or_else(|| "all sessions".to_string());
        let query_label = query
            .trim()
            .is_empty()
            .then(|| "none".to_string())
            .unwrap_or_else(|| format!("\"{}\"", summarize_line(query, 48)));
        let mut lines = vec![
            format!("Runtime transcript query: {}", scope),
            format!("- query: {}", query_label),
            format!("- matched spans: {}", report.matched_count),
            format!("- returned hits: {}", report.returned_count),
            format!("- matched sessions: {}", matched_sessions),
            format!("- returned transcript entries: {}", returned_entries),
            format!(
                "- truncated: {}",
                if report.truncated { "yes" } else { "no" }
            ),
            format!("- limit: {}", format_limit(limit)),
            "By session:".to_string(),
        ];

        if sessions.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(sessions.iter().map(format_transcript_query_session_tally));
        }

        lines.extend(["Hits:".to_string()]);

        if report.hits.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(report.hits.iter().map(format_transcript_span));
        }

        Ok(lines)
    }

    pub fn context_recall_summary(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        allow_cross_session: bool,
    ) -> Result<Vec<String>, HeptaError> {
        let RuntimeContextRecallSlice {
            bundle,
            total_recent_entry_count,
            transcript_matched_count,
            memory_matched_count,
            ..
        } = self.context_recall_slice(
            session_id,
            query_text,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            allow_cross_session,
        )?;
        let evidence = recall_evidence_summary(
            &bundle,
            ContextRecallAvailability {
                total_recent_entry_count,
                total_transcript_match_count: transcript_matched_count,
                total_memory_match_count: memory_matched_count,
            },
        );
        let query_label = bundle
            .request
            .query_text
            .as_deref()
            .map(str::trim)
            .filter(|query| !query.is_empty())
            .map(|query| format!("\"{}\"", summarize_line(query, 48)))
            .unwrap_or_else(|| "none".to_string());
        let mut lines = vec![
            format!("Runtime context recall: session {}", session_id),
            format!("- query: {}", query_label),
            format!("- evidence readiness: {}", evidence.readiness_label()),
            format!("- recent entries: {}", evidence.recent_entry_count),
            format!("- transcript matches: {}", transcript_matched_count),
            format!(
                "- transcript hits returned: {}",
                evidence.transcript_hit_count
            ),
            format!(
                "- durable memory hits: {}",
                evidence.durable_memory_hit_count
            ),
            format!("- summary hits: {}", evidence.summary_hit_count),
            format!(
                "- active topic sessions: {}",
                evidence.active_topic_session_count
            ),
            format!("- active neurons: {}", evidence.active_neuron_count),
            format!(
                "- transcript evidence spans: {}",
                evidence.transcript_evidence_span_count
            ),
            format!("- omitted items: {}", evidence.omitted_item_count),
            format!(
                "- cross-session memory: {}",
                if evidence.cross_session_allowed {
                    "allowed"
                } else {
                    "disabled"
                }
            ),
            format!(
                "- truncated: {}",
                if evidence.truncated { "yes" } else { "no" }
            ),
            "Recent window:".to_string(),
        ];

        if !evidence.findings.is_empty() {
            lines.push("Recall evidence findings:".to_string());
            lines.extend(
                evidence
                    .findings
                    .iter()
                    .map(|finding| format!("  - {}", finding)),
            );
        }

        if bundle.recent_entries.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(bundle.recent_entries.iter().map(format_transcript_entry));
        }

        lines.push("Transcript hits:".to_string());
        if bundle.transcript_hits.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(bundle.transcript_hits.iter().map(format_transcript_span));
        }

        lines.push("Durable memory hits:".to_string());
        if bundle.durable_memory_hits.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(bundle.durable_memory_hits.iter().map(format_memory_record));
        }

        lines.push("Session summary hits:".to_string());
        if bundle.summary_hits.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(bundle.summary_hits.iter().map(format_memory_record));
        }

        lines.push("Active topic sessions:".to_string());
        if bundle.active_topic_sessions.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(
                bundle
                    .active_topic_sessions
                    .iter()
                    .map(|topic_session| format_active_topic_session(topic_session)),
            );
        }

        Ok(lines)
    }

    pub fn intuition_summary(
        &self,
        session_id: &str,
        user_intent: &str,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        topic_limit: usize,
        neuron_limit: usize,
        skill_limit: usize,
    ) -> Result<Vec<String>, HeptaError> {
        let RuntimeIntuitionOverview {
            recent_entry_count,
            transcript_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
            active_topic_session_count,
            routed_topic_count,
            returned_neuron_activation_count,
            bundle,
            ..
        } = self.intuition_overview(
            session_id,
            user_intent,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            topic_limit,
            neuron_limit,
            skill_limit,
        )?;
        let mut lines = vec![
            format!("Runtime intuition: session {}", session_id),
            format!("- user intent: \"{}\"", summarize_line(user_intent, 56)),
            format!("- recent entries: {}", recent_entry_count),
            format!("- transcript matches: {}", transcript_matched_count),
            format!("- durable memory hits: {}", durable_memory_hit_count),
            format!("- summary hits: {}", summary_hit_count),
            format!(
                "- transcript evidence spans: {}",
                bundle.source_transcript_spans.len()
            ),
            format!(
                "- foreground topic sessions: {}",
                active_topic_session_count
            ),
            format!("- routed topics: {}", routed_topic_count),
            format!(
                "- returned neuron activations: {}",
                returned_neuron_activation_count
            ),
            format!("- suggested skills: {}", bundle.skill_decisions.len()),
            format!("- workflow priors: {}", bundle.workflow_priors.len()),
            format!(
                "- truncated: {}",
                if bundle.truncated { "yes" } else { "no" }
            ),
            "Topic activation scores:".to_string(),
        ];

        if bundle.topic_activation_scores.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(
                bundle
                    .topic_activation_scores
                    .iter()
                    .map(format_topic_activation_score),
            );
        }

        lines.push("Neuron activations:".to_string());
        if bundle.neuron_activations.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(
                bundle
                    .neuron_activations
                    .iter()
                    .map(format_neuron_activation),
            );
        }

        lines.push("Skill decisions:".to_string());
        if bundle.skill_decisions.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(
                bundle
                    .skill_decisions
                    .iter()
                    .map(format_skill_activation_decision),
            );
        }

        lines.push("Workflow priors:".to_string());
        if bundle.workflow_priors.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(bundle.workflow_priors.iter().map(format_workflow_prior));
        }

        if let Some(explanation) = &bundle.explanation {
            lines.push(format!(
                "Explanation: \"{}\"",
                summarize_line(explanation, 96)
            ));
        }

        Ok(lines)
    }

    pub fn provenance_summary(&self, session_id: &str) -> Result<Vec<String>, HeptaError> {
        let RuntimeProvenanceOverview {
            session_id,
            last_user_intent_summary,
            total_topic_sessions,
            active_topic_sessions,
            active_topic_sessions_with_transcript_provenance,
            active_topic_sessions_missing_transcript_provenance,
            recall_transcript_evidence_spans,
            recall_omitted_items,
            intuition_transcript_evidence_spans,
            intuition_foreground_topic_sessions,
        } = self.provenance_overview(session_id)?;

        Ok(vec![
            format!("Runtime provenance: session {}", session_id),
            format!(
                "- last user intent summary: {}",
                last_user_intent_summary
                    .as_deref()
                    .map(|text| format!("\"{}\"", summarize_line(text, 56)))
                    .unwrap_or_else(|| "none".to_string())
            ),
            format!("- total topic sessions: {}", total_topic_sessions),
            format!(
                "- active topic sessions with transcript provenance: {}/{}",
                active_topic_sessions_with_transcript_provenance, active_topic_sessions
            ),
            format!(
                "- active topic sessions missing transcript provenance: {}",
                active_topic_sessions_missing_transcript_provenance
            ),
            format!(
                "- recall transcript evidence spans: {}",
                recall_transcript_evidence_spans
            ),
            format!("- recall omitted items: {}", recall_omitted_items),
            format!(
                "- intuition transcript evidence spans: {}",
                intuition_transcript_evidence_spans
            ),
            format!(
                "- intuition foreground topic sessions: {}",
                intuition_foreground_topic_sessions
            ),
        ])
    }

    pub async fn intelligence_phase2_summary(
        &self,
        session_id: &str,
    ) -> Result<Vec<String>, HeptaError> {
        let RuntimeIntelligencePhase2Overview {
            session_id,
            phase,
            status,
            overall_percent,
            all_phase2_gates_ready,
            blended_recall_ready,
            provenance_memory_ready,
            semantic_router_generalized,
            neuron_compression_ready,
            recall_ranked_items,
            recall_source_count,
            recall_transcript_evidence_spans,
            durable_memory_hits,
            active_neurons,
            provenance_active_topic_sessions,
            provenance_topic_sessions_with_transcript,
            supported_semantic_router_count,
            learned_router_signal_count,
            compressed_neuron_count,
            neurons_with_evidence_digest,
            gates,
            findings,
        } = self.intelligence_phase2_gate(session_id).await?;

        let mut lines = vec![
            format!("Hepta intelligence phase2: {}", status),
            format!("- session: {}", session_id),
            format!("- phase: {}", phase),
            format!("- overall percent: {}", overall_percent),
            format!("- all gates ready: {}", all_phase2_gates_ready),
            format!("- blended recall ready: {}", blended_recall_ready),
            format!("- provenance memory ready: {}", provenance_memory_ready),
            format!(
                "- semantic router generalized: {}",
                semantic_router_generalized
            ),
            format!("- neuron compression ready: {}", neuron_compression_ready),
            format!("- recall ranked items: {}", recall_ranked_items),
            format!("- recall source count: {}", recall_source_count),
            format!(
                "- recall transcript evidence spans: {}",
                recall_transcript_evidence_spans
            ),
            format!("- durable memory hits: {}", durable_memory_hits),
            format!("- active neurons: {}", active_neurons),
            format!(
                "- provenance topic sessions with transcript: {}/{}",
                provenance_topic_sessions_with_transcript, provenance_active_topic_sessions
            ),
            format!(
                "- semantic routers/signals: {}/{}",
                supported_semantic_router_count, learned_router_signal_count
            ),
            format!(
                "- compressed neurons/evidence digests: {}/{}",
                compressed_neuron_count, neurons_with_evidence_digest
            ),
            "Gates:".to_string(),
        ];

        lines.extend(gates.into_iter().map(|gate| {
            format!(
                "  - {} ready={} evidence={}",
                gate.id, gate.ready, gate.evidence
            )
        }));
        lines.push("Findings:".to_string());
        if findings.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(
                findings
                    .into_iter()
                    .map(|finding| format!("  - {}", finding)),
            );
        }

        Ok(lines)
    }

    pub fn knowledge_graph_dry_run_summary(&self) -> Result<Vec<String>, HeptaError> {
        let report = self.knowledge_graph_dry_run_overview();
        let mut lines = vec![
            format!("Hepta KG dry-run: {}", report.status),
            format!("- contract: {}", report.contract),
            format!("- sample run: {}", report.sample_run),
            format!("- memory units: {}", report.memory_unit_count),
            format!("- write candidates: {}", report.candidate_count),
            format!("- live write enabled: {}", report.live_write_enabled_count),
            format!(
                "- external side effects enabled: {}",
                report.external_side_effect_enabled_count
            ),
            format!(
                "- all candidates have provenance: {}",
                report.checks.all_candidates_have_provenance
            ),
            format!(
                "- all candidates have graph payload: {}",
                report.checks.all_candidates_have_graph_payload
            ),
            format!(
                "- all plans are dry-run: {}",
                report.checks.all_plans_are_dry_run
            ),
            format!(
                "- no live write enabled: {}",
                report.checks.no_live_write_enabled
            ),
            format!(
                "- no external side effects: {}",
                report.checks.no_external_side_effects
            ),
            format!("- next phase: {}", report.next_phase),
            "Candidates:".to_string(),
        ];

        if report.candidates.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(report.candidates.iter().take(4).map(|candidate| {
                format!(
                    "  - {} episode={} entities={} relations={}",
                    candidate.id,
                    candidate.episode.id,
                    candidate.entities.len(),
                    candidate.relations.len()
                )
            }));
        }

        Ok(lines)
    }

    pub fn knowledge_graph_adapter_dry_run_summary(&self) -> Result<Vec<String>, HeptaError> {
        let report = self.knowledge_graph_adapter_dry_run_overview();
        let mut lines = vec![
            format!("Hepta KG adapter dry-run: {}", report.status),
            format!("- contract: {}", report.contract),
            format!("- sample run: {}", report.sample_run),
            format!("- write candidates: {}", report.candidate_count),
            format!("- supported adapters: {}", report.adapter_count),
            format!("- adapter projections: {}", report.projection_count),
            format!(
                "- network calls enabled: {}",
                report.network_call_enabled_count
            ),
            format!(
                "- external writes enabled: {}",
                report.external_write_enabled_count
            ),
            format!("- live writes enabled: {}", report.live_write_enabled_count),
            format!(
                "- all supported adapters projected: {}",
                report.checks.all_supported_adapters_projected
            ),
            format!(
                "- all projections have records: {}",
                report.checks.all_projections_have_records
            ),
            format!(
                "- no network calls enabled: {}",
                report.checks.no_network_calls_enabled
            ),
            format!(
                "- no external writes enabled: {}",
                report.checks.no_external_writes_enabled
            ),
            format!(
                "- no live writes enabled: {}",
                report.checks.no_live_writes_enabled
            ),
            format!("- next phase: {}", report.next_phase),
            "Adapter projections:".to_string(),
        ];

        if report.projections.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(report.projections.iter().take(6).map(|projection| {
                format!(
                    "  - adapter={} candidate={} family={} records={} network={} write={}",
                    projection.adapter_id,
                    projection.candidate_id,
                    projection.projection_family,
                    projection.projected_total_records,
                    projection.network_call_allowed,
                    projection.external_write_allowed
                )
            }));
        }

        Ok(lines)
    }

    pub fn knowledge_graph_adapter_staging_gate_summary(&self) -> Result<Vec<String>, HeptaError> {
        let report = self.knowledge_graph_adapter_staging_gate_overview();
        let mut lines = vec![
            format!("Hepta KG adapter staging gate: {}", report.status),
            format!("- contract: {}", report.contract),
            format!("- sample run: {}", report.sample_run),
            format!("- write candidates: {}", report.candidate_count),
            format!("- supported adapters: {}", report.adapter_count),
            format!("- staging plans: {}", report.staging_plan_count),
            format!("- staging ready: {}", report.staging_ready_count),
            format!(
                "- network calls enabled: {}",
                report.network_call_enabled_count
            ),
            format!(
                "- external writes enabled: {}",
                report.external_write_enabled_count
            ),
            format!("- live writes enabled: {}", report.live_write_enabled_count),
            format!(
                "- all supported adapters gated: {}",
                report.checks.all_supported_adapters_gated
            ),
            format!(
                "- closed by default: {}",
                report.checks.all_staging_plans_closed_by_default
            ),
            format!(
                "- operator review required: {}",
                report.checks.operator_review_required
            ),
            format!(
                "- rollback plan required: {}",
                report.checks.rollback_plan_required
            ),
            format!(
                "- post-write validation required: {}",
                report.checks.post_write_validation_required
            ),
            format!(
                "- no network calls enabled: {}",
                report.checks.no_network_calls_enabled
            ),
            format!(
                "- no external writes enabled: {}",
                report.checks.no_external_writes_enabled
            ),
            format!(
                "- no live writes enabled: {}",
                report.checks.no_live_writes_enabled
            ),
            format!("- next phase: {}", report.next_phase),
            "Adapter staging gates:".to_string(),
        ];

        if report.plans.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(report.plans.iter().take(6).map(|plan| {
                format!(
                    "  - adapter={} candidate={} gate={} staging_ready={} network={} write={} live={}",
                    plan.adapter_id,
                    plan.source_candidate_id,
                    plan.feature_gate_name,
                    plan.staging_ready,
                    plan.network_call_allowed,
                    plan.external_write_allowed,
                    plan.live_write_allowed
                )
            }));
        }

        Ok(lines)
    }

    pub fn knowledge_graph_adapter_client_summary(&self) -> Result<Vec<String>, HeptaError> {
        let report = self.knowledge_graph_adapter_client_overview();
        let mut lines = vec![
            format!("Hepta KG adapter clients: {}", report.status),
            format!("- contract: {}", report.contract),
            format!("- sample run: {}", report.sample_run),
            format!("- write candidates: {}", report.candidate_count),
            format!("- supported adapters: {}", report.adapter_count),
            format!("- client audits: {}", report.client_audit_count),
            format!("- denied clients: {}", report.denied_client_count),
            format!(
                "- network calls attempted: {}",
                report.network_call_attempted_count
            ),
            format!(
                "- external writes attempted: {}",
                report.external_write_attempted_count
            ),
            format!(
                "- live writes attempted: {}",
                report.live_write_attempted_count
            ),
            format!("- persisted records: {}", report.persisted_record_count),
            format!(
                "- all supported clients present: {}",
                report.checks.all_supported_clients_present
            ),
            format!(
                "- denied by default: {}",
                report.checks.all_client_calls_denied_by_default
            ),
            format!(
                "- no network calls attempted: {}",
                report.checks.no_network_calls_attempted
            ),
            format!(
                "- no external writes attempted: {}",
                report.checks.no_external_writes_attempted
            ),
            format!(
                "- no live writes attempted: {}",
                report.checks.no_live_writes_attempted
            ),
            format!(
                "- no records persisted: {}",
                report.checks.no_records_persisted
            ),
            format!("- next phase: {}", report.next_phase),
            "Adapter client audits:".to_string(),
        ];

        if report.audits.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(report.audits.iter().take(6).map(|audit| {
                format!(
                    "  - adapter={} candidate={} client={} network_attempted={} write_attempted={} live_attempted={} persisted={}",
                    audit.adapter_id,
                    audit.candidate_id,
                    audit.client_name,
                    audit.network_call_attempted,
                    audit.external_write_attempted,
                    audit.live_write_attempted,
                    audit.persisted_records
                )
            }));
        }

        Ok(lines)
    }

    pub fn knowledge_graph_adapter_config_env_summary(&self) -> Result<Vec<String>, HeptaError> {
        let report = self.knowledge_graph_adapter_config_env_overview();
        let mut lines = vec![
            format!("Hepta KG adapter config env: {}", report.status),
            format!("- contract: {}", report.contract),
            format!("- sample run: {}", report.sample_run),
            format!("- supported adapters: {}", report.adapter_count),
            format!("- config reads: {}", report.config_read_count),
            format!("- feature enabled: {}", report.feature_enabled_count),
            format!(
                "- endpoints configured: {}",
                report.endpoint_configured_count
            ),
            format!(
                "- credentials configured: {}",
                report.credentials_configured_count
            ),
            format!(
                "- network allowlisted: {}",
                report.network_allowlisted_count
            ),
            format!(
                "- external write allowlisted: {}",
                report.external_write_allowlisted_count
            ),
            format!("- operator approved: {}", report.operator_approved_count),
            format!(
                "- dry-run samples passed: {}",
                report.dry_run_sample_passed_count
            ),
            format!(
                "- rollback plans ready: {}",
                report.rollback_plan_ready_count
            ),
            format!(
                "- post-write validations ready: {}",
                report.post_write_validation_ready_count
            ),
            format!("- fully configured: {}", report.fully_configured_count),
            format!(
                "- live writes requested: {}",
                report.live_write_requested_count
            ),
            format!(
                "- credential values captured: {}",
                report.credential_value_captured_count
            ),
            format!(
                "- network calls attempted: {}",
                report.network_call_attempted_count
            ),
            format!(
                "- external writes attempted: {}",
                report.external_write_attempted_count
            ),
            format!(
                "- live writes attempted: {}",
                report.live_write_attempted_count
            ),
            format!(
                "- all supported adapters read: {}",
                report.checks.all_supported_adapters_read
            ),
            format!(
                "- env keys present in report: {}",
                report.checks.all_env_keys_present_in_report
            ),
            format!(
                "- configs closed by default: {}",
                report.checks.all_configs_closed_by_default
            ),
            format!(
                "- no credential values captured: {}",
                report.checks.no_credential_values_captured
            ),
            format!(
                "- no network calls attempted: {}",
                report.checks.no_network_calls_attempted
            ),
            format!(
                "- no external writes attempted: {}",
                report.checks.no_external_writes_attempted
            ),
            format!(
                "- no live writes attempted: {}",
                report.checks.no_live_writes_attempted
            ),
            format!("- next phase: {}", report.next_phase),
            "Adapter config env reads:".to_string(),
        ];

        if report.reads.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(report.reads.iter().take(6).map(|read| {
                format!(
                    "  - adapter={} gate_key={} endpoint_key={} credential_ref_key={} feature={} endpoint={} credential_ref={} network={} write={} live_requested={}",
                    read.adapter_id,
                    read.keys.feature_gate,
                    read.keys.endpoint,
                    read.keys.credential_ref,
                    read.staging_config.feature_enabled,
                    read.staging_config.endpoint_configured,
                    read.staging_config.credentials_configured,
                    read.staging_config.network_allowlisted,
                    read.staging_config.external_write_allowlisted,
                    read.staging_config.live_write_requested
                )
            }));
        }

        Ok(lines)
    }

    pub fn knowledge_graph_recall_plan_summary(&self) -> Result<Vec<String>, HeptaError> {
        let report = self.knowledge_graph_recall_plan_overview();
        let mut lines = vec![
            format!("Hepta KG recall plan: {}", report.status),
            format!("- contract: {}", report.contract),
            format!("- sample run: {}", report.sample_run),
            format!("- recall queries: {}", report.query_count),
            format!("- write candidates: {}", report.candidate_count),
            format!("- entity matches: {}", report.entity_match_count),
            format!(
                "- relation neighborhoods: {}",
                report.relation_neighborhood_count
            ),
            format!("- timeline slices: {}", report.timeline_slice_count),
            format!("- evidence paths: {}", report.evidence_path_count),
            format!(
                "- external reads enabled: {}",
                report.external_read_enabled_count
            ),
            format!(
                "- network calls enabled: {}",
                report.network_call_enabled_count
            ),
            format!("- live writes enabled: {}", report.live_write_enabled_count),
            format!(
                "- all plans are read-only: {}",
                report.checks.all_plans_are_read_only
            ),
            format!(
                "- no external reads enabled: {}",
                report.checks.no_external_reads_enabled
            ),
            format!(
                "- no network calls enabled: {}",
                report.checks.no_network_calls_enabled
            ),
            format!(
                "- no live writes enabled: {}",
                report.checks.no_live_writes_enabled
            ),
            format!("- next phase: {}", report.next_phase),
            "Recall plans:".to_string(),
        ];

        if report.plans.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(report.plans.iter().take(4).map(|plan| {
                format!(
                    "  - query={} entities={} relations={} timeline={} evidence={} read_only={} external_read={} network={} live={}",
                    plan.query_id,
                    plan.entity_match_count,
                    plan.relation_neighborhood_count,
                    plan.timeline_slice_count,
                    plan.evidence_path_count,
                    plan.read_only,
                    plan.external_read_allowed,
                    plan.network_call_allowed,
                    plan.live_write_allowed
                )
            }));
        }

        lines.push("Entity matches:".to_string());
        let entity_lines = report
            .plans
            .iter()
            .flat_map(|plan| plan.entity_matches.iter())
            .take(6)
            .map(|entity_match| {
                format!(
                    "  - entity={} label={} confidence={} evidence_spans={}",
                    entity_match.entity.id,
                    entity_match.matched_label,
                    entity_match.confidence.basis_points,
                    entity_match.evidence_span_count
                )
            })
            .collect::<Vec<_>>();
        if entity_lines.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(entity_lines);
        }

        Ok(lines)
    }

    pub fn knowledge_graph_context_recall_bridge_summary(&self) -> Result<Vec<String>, HeptaError> {
        let report = self.knowledge_graph_context_recall_bridge_overview();
        let mut lines = vec![
            format!("Hepta KG context recall bridge: {}", report.status),
            format!("- contract: {}", report.contract),
            format!("- kg recall contract: {}", report.kg_recall_contract),
            format!("- sample run: {}", report.sample_run),
            format!("- recall queries: {}", report.query_count),
            format!("- kg recall plans: {}", report.kg_plan_count),
            format!("- kg evidence paths: {}", report.kg_evidence_path_count),
            format!("- context recall items: {}", report.context_item_count),
            format!("- transcript spans: {}", report.transcript_span_count),
            format!(
                "- external reads enabled: {}",
                report.external_read_enabled_count
            ),
            format!(
                "- network calls enabled: {}",
                report.network_call_enabled_count
            ),
            format!("- live writes enabled: {}", report.live_write_enabled_count),
            format!("- model invoked: {}", report.model_invoked),
            format!(
                "- context injection performed: {}",
                report.context_injection_performed
            ),
            format!("- recall plan ready: {}", report.checks.recall_plan_ready),
            format!(
                "- all items have KG source: {}",
                report.checks.all_items_have_kg_source
            ),
            format!(
                "- all items have scores: {}",
                report.checks.all_items_have_scores
            ),
            format!(
                "- transcript provenance preserved: {}",
                report.checks.transcript_provenance_preserved
            ),
            format!(
                "- no external reads enabled: {}",
                report.checks.no_external_reads_enabled
            ),
            format!(
                "- no network calls enabled: {}",
                report.checks.no_network_calls_enabled
            ),
            format!(
                "- no live writes enabled: {}",
                report.checks.no_live_writes_enabled
            ),
            format!("- no model invoked: {}", report.checks.no_model_invoked),
            format!(
                "- no context injection performed: {}",
                report.checks.no_context_injection_performed
            ),
            format!("- next phase: {}", report.next_phase),
            "KG context recall items:".to_string(),
        ];

        if report.items.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(report.items.iter().take(6).map(|item| {
                format!(
                    "  - source_id={} score={:.3} spans={} summary={}",
                    item.source_id,
                    item.score.final_score,
                    item.source_transcript_spans.len(),
                    item.summary
                )
            }));
        }

        Ok(lines)
    }

    pub fn knowledge_graph_recall_evaluation_summary(&self) -> Result<Vec<String>, HeptaError> {
        let report = self.knowledge_graph_recall_evaluation_overview();
        let mut lines = vec![
            format!("Hepta KG recall evaluation: {}", report.status),
            format!("- contract: {}", report.contract),
            format!("- kg recall contract: {}", report.kg_recall_contract),
            format!(
                "- kg context bridge contract: {}",
                report.kg_context_bridge_contract
            ),
            format!("- sample run: {}", report.sample_run),
            format!("- recall queries: {}", report.query_count),
            format!("- context recall items: {}", report.context_item_count),
            format!("- evaluation cases: {}", report.evaluation_case_count),
            format!("- passed cases: {}", report.passed_case_count),
            format!("- failed cases: {}", report.failed_case_count),
            format!(
                "- entity evidence cases: {}",
                report.entity_evidence_case_count
            ),
            format!("- relation path cases: {}", report.relation_path_case_count),
            format!(
                "- timeline slice cases: {}",
                report.timeline_slice_case_count
            ),
            format!(
                "- transcript provenance cases: {}",
                report.transcript_provenance_case_count
            ),
            format!(
                "- duplicate context source ids: {}",
                report.duplicate_context_source_id_count
            ),
            format!(
                "- duplicate source memory ids: {}",
                report.duplicate_source_memory_id_count
            ),
            format!(
                "- score order violations: {}",
                report.score_order_violation_count
            ),
            format!("- coverage bp: {}", report.coverage_basis_points),
            format!(
                "- precision proxy bp: {}",
                report.precision_proxy_basis_points
            ),
            format!(
                "- score stability bp: {}",
                report.score_stability_basis_points
            ),
            format!(
                "- external reads enabled: {}",
                report.external_read_enabled_count
            ),
            format!(
                "- network calls enabled: {}",
                report.network_call_enabled_count
            ),
            format!("- live writes enabled: {}", report.live_write_enabled_count),
            format!("- model invoked: {}", report.model_invoked),
            format!(
                "- context injection performed: {}",
                report.context_injection_performed
            ),
            format!("- bridge ready: {}", report.checks.bridge_ready),
            format!("- all cases passed: {}", report.checks.all_cases_passed),
            format!(
                "- source memory ids unique: {}",
                report.checks.source_memory_ids_unique
            ),
            format!(
                "- scores stably ordered: {}",
                report.checks.scores_stably_ordered
            ),
            format!(
                "- no external reads enabled: {}",
                report.checks.no_external_reads_enabled
            ),
            format!(
                "- no network calls enabled: {}",
                report.checks.no_network_calls_enabled
            ),
            format!(
                "- no live writes enabled: {}",
                report.checks.no_live_writes_enabled
            ),
            format!("- no model invoked: {}", report.checks.no_model_invoked),
            format!(
                "- no context injection performed: {}",
                report.checks.no_context_injection_performed
            ),
            format!("- next phase: {}", report.next_phase),
            "Evaluation cases:".to_string(),
        ];

        if report.cases.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(report.cases.iter().take(6).map(|case| {
                format!(
                    "  - query={} candidate={} score={} entity={} relation={} timeline={} transcript={} passed={}",
                    case.query_id,
                    case.candidate_id,
                    case.final_score_basis_points,
                    case.entity_evidence_count,
                    case.relation_path_count,
                    case.timeline_slice_count,
                    case.transcript_span_count,
                    case.passed
                )
            }));
        }

        Ok(lines)
    }

    pub fn knowledge_graph_context_injection_readiness_summary(
        &self,
    ) -> Result<Vec<String>, HeptaError> {
        let report = self.knowledge_graph_context_injection_readiness_overview();
        let mut lines = vec![
            format!("Hepta KG context injection readiness: {}", report.status),
            format!("- contract: {}", report.contract),
            format!(
                "- kg recall evaluation contract: {}",
                report.kg_recall_evaluation_contract
            ),
            format!(
                "- kg context bridge contract: {}",
                report.kg_context_bridge_contract
            ),
            format!("- sample run: {}", report.sample_run),
            format!("- evaluation cases: {}", report.evaluation_case_count),
            format!("- passed cases: {}", report.passed_case_count),
            format!("- failed cases: {}", report.failed_case_count),
            format!("- coverage bp: {}", report.coverage_basis_points),
            format!(
                "- precision proxy bp: {}",
                report.precision_proxy_basis_points
            ),
            format!(
                "- score stability bp: {}",
                report.score_stability_basis_points
            ),
            format!(
                "- quality threshold bp: {}",
                report.quality_threshold_basis_points
            ),
            format!("- quality gate ready: {}", report.quality_gate_ready),
            format!("- operator approved: {}", report.operator_approved),
            format!("- shadow rank enabled: {}", report.shadow_rank_enabled),
            format!("- rollback plan ready: {}", report.rollback_plan_ready),
            format!("- kill switch ready: {}", report.kill_switch_ready),
            format!(
                "- context injection allowed: {}",
                report.context_injection_allowed
            ),
            format!(
                "- context injection performed: {}",
                report.context_injection_performed
            ),
            format!(
                "- prompt preview rendered: {}",
                report.prompt_preview_rendered
            ),
            format!("- model invoked: {}", report.model_invoked),
            format!(
                "- external reads enabled: {}",
                report.external_read_enabled_count
            ),
            format!(
                "- network calls enabled: {}",
                report.network_call_enabled_count
            ),
            format!("- live writes enabled: {}", report.live_write_enabled_count),
            format!(
                "- recall evaluation ready: {}",
                report.checks.recall_evaluation_ready
            ),
            format!(
                "- quality threshold met: {}",
                report.checks.quality_threshold_met
            ),
            format!(
                "- activation blocked without operator approval: {}",
                report.checks.activation_blocked_without_operator_approval
            ),
            format!(
                "- prompt preview not rendered: {}",
                report.checks.prompt_preview_not_rendered
            ),
            format!("- no model invoked: {}", report.checks.no_model_invoked),
            format!(
                "- no context injection performed: {}",
                report.checks.no_context_injection_performed
            ),
            format!("- next phase: {}", report.next_phase),
            "Readiness blockers:".to_string(),
        ];

        if report.blockers.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(
                report
                    .blockers
                    .iter()
                    .map(|blocker| format!("  - {:?}", blocker)),
            );
        }

        Ok(lines)
    }

    pub fn intelligence_eval_summary(
        &self,
        session_id: &str,
        case_limit: usize,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        topic_limit: usize,
        neuron_limit: usize,
        skill_limit: usize,
    ) -> Result<Vec<String>, HeptaError> {
        self.intelligence_eval_summary_with_router(
            session_id,
            case_limit,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            topic_limit,
            neuron_limit,
            skill_limit,
            None,
        )
    }

    pub fn intelligence_eval_summary_with_router(
        &self,
        session_id: &str,
        case_limit: usize,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        topic_limit: usize,
        neuron_limit: usize,
        skill_limit: usize,
        semantic_router_id: Option<&str>,
    ) -> Result<Vec<String>, HeptaError> {
        let RuntimeIntelligenceEvalOverview {
            session_id,
            evaluated_case_count,
            passed_case_count,
            failed_case_count,
            semantic_router_id,
            learned_router_case_count,
            total_learned_router_signals,
            total_learned_positive_signals,
            total_learned_negative_signals,
            contrast_focus_case_counts,
            contrast_focus_passed_counts,
            contrast_focus_signal_counts,
            contrast_focus_positive_signal_counts,
            contrast_focus_negative_signal_counts,
            total_recall_ranked_items,
            total_transcript_evidence_spans,
            total_active_neurons,
            total_routed_topics,
            total_neuron_activations,
            total_suggested_skills,
            registered_skill_decision_count,
            prepared_skill_decision_count,
            gated_skill_decision_count,
            total_workflow_priors,
            registered_workflow_prior_count,
            prepared_workflow_prior_count,
            gated_workflow_prior_count,
            feedback_record_count,
            feedback_net_weight_delta,
            calibrated_skill_target_count,
            calibrated_workflow_target_count,
            total_semantic_expectations,
            total_semantic_expectations_passed,
            semantic_score,
            cases,
        } = self.intelligence_eval_overview_with_router(
            session_id,
            case_limit,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            topic_limit,
            neuron_limit,
            skill_limit,
            semantic_router_id,
        )?;

        let mut lines = vec![
            format!("Runtime intelligence eval: session {}", session_id),
            format!("- evaluated cases: {}", evaluated_case_count),
            format!("- passed cases: {}", passed_case_count),
            format!("- failed cases: {}", failed_case_count),
            format!("- semantic router: {}", semantic_router_id),
            format!("- learned-router cases: {}", learned_router_case_count),
            format!("- learned-router signals: {}", total_learned_router_signals),
            format!(
                "- learned-router positive/negative signals: {}/{}",
                total_learned_positive_signals, total_learned_negative_signals
            ),
            format!(
                "- contrast focus cases: {}",
                format_string_usize_map(&contrast_focus_case_counts)
            ),
            format!(
                "- contrast focus passed: {}",
                format_string_usize_map(&contrast_focus_passed_counts)
            ),
            format!(
                "- contrast focus signals: {}",
                format_string_usize_map(&contrast_focus_signal_counts)
            ),
            format!(
                "- contrast focus positive signals: {}",
                format_string_usize_map(&contrast_focus_positive_signal_counts)
            ),
            format!(
                "- contrast focus negative signals: {}",
                format_string_usize_map(&contrast_focus_negative_signal_counts)
            ),
            format!("- total recall ranked items: {}", total_recall_ranked_items),
            format!(
                "- total transcript evidence spans: {}",
                total_transcript_evidence_spans
            ),
            format!("- total active neurons: {}", total_active_neurons),
            format!("- total routed topics: {}", total_routed_topics),
            format!("- total neuron activations: {}", total_neuron_activations),
            format!("- total suggested skills: {}", total_suggested_skills),
            format!(
                "- registered skill decisions: {}",
                registered_skill_decision_count
            ),
            format!(
                "- prepared skill decisions: {}",
                prepared_skill_decision_count
            ),
            format!("- gated skill decisions: {}", gated_skill_decision_count),
            format!("- total workflow priors: {}", total_workflow_priors),
            format!(
                "- registered workflow priors: {}",
                registered_workflow_prior_count
            ),
            format!(
                "- prepared workflow priors: {}",
                prepared_workflow_prior_count
            ),
            format!("- gated workflow priors: {}", gated_workflow_prior_count),
            format!("- feedback records: {}", feedback_record_count),
            format!(
                "- feedback net weight delta: {:+.2}",
                feedback_net_weight_delta
            ),
            format!(
                "- calibrated skill targets: {}",
                calibrated_skill_target_count
            ),
            format!(
                "- calibrated workflow targets: {}",
                calibrated_workflow_target_count
            ),
            format!(
                "- semantic expectations: {}/{}",
                total_semantic_expectations_passed, total_semantic_expectations
            ),
            format!("- semantic score: {}", semantic_score),
            "Cases:".to_string(),
        ];

        if cases.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(cases.iter().map(format_intelligence_eval_case));
        }

        Ok(lines)
    }

    pub fn neuron_lifecycle_summary(&self, session_id: &str) -> Result<Vec<String>, HeptaError> {
        let RuntimeNeuronLifecycleOverview {
            session_id,
            total_topic_sessions,
            active_topic_sessions,
            stored_neurons,
            neurons_with_transcript_provenance,
            neurons_with_memory_provenance,
            neurons_with_evidence_digest,
            v2_compressed_neurons,
            neurons_with_skill_priors,
            neurons_with_workflow_priors,
            neurons_with_typed_links,
            intuition_ready_neurons,
            lineage_neurons,
            merged_neurons,
            split_neurons,
            superseded_neurons,
            aging_neurons,
            cross_session_stable_neurons,
            cross_session_unstable_neurons,
            merge_split_lineage_edges,
            average_confidence,
            average_freshness,
            stale_neurons,
            low_confidence_neurons,
            low_freshness_neurons,
            compression_policy_versions,
            neuron_upgrade_ready,
            active_topics_without_neurons,
            findings,
            healthy,
        } = self.neuron_lifecycle_overview(session_id)?;

        let mut lines = vec![
            format!("Neuron lifecycle: session {}", session_id),
            format!("- healthy: {}", healthy),
            format!("- total topic sessions: {}", total_topic_sessions),
            format!("- active topic sessions: {}", active_topic_sessions),
            format!("- stored neurons: {}", stored_neurons),
            format!(
                "- neurons with transcript provenance: {}",
                neurons_with_transcript_provenance
            ),
            format!(
                "- neurons with memory provenance: {}",
                neurons_with_memory_provenance
            ),
            format!(
                "- neurons with evidence digest: {}",
                neurons_with_evidence_digest
            ),
            format!("- v2 compressed neurons: {}", v2_compressed_neurons),
            format!("- neurons with skill priors: {}", neurons_with_skill_priors),
            format!(
                "- neurons with workflow priors: {}",
                neurons_with_workflow_priors
            ),
            format!("- neurons with typed links: {}", neurons_with_typed_links),
            format!("- intuition-ready neurons: {}", intuition_ready_neurons),
            format!("- neuron upgrade ready: {}", neuron_upgrade_ready),
            format!("- lineage neurons: {}", lineage_neurons),
            format!("- merged neurons: {}", merged_neurons),
            format!("- split neurons: {}", split_neurons),
            format!("- superseded neurons: {}", superseded_neurons),
            format!("- aging neurons: {}", aging_neurons),
            format!(
                "- cross-session stable neurons: {}",
                cross_session_stable_neurons
            ),
            format!(
                "- cross-session unstable neurons: {}",
                cross_session_unstable_neurons
            ),
            format!("- merge/split lineage edges: {}", merge_split_lineage_edges),
            format!("- average confidence: {:.2}", average_confidence),
            format!("- average freshness: {:.2}", average_freshness),
            format!("- stale neurons: {}", stale_neurons),
            format!("- low confidence neurons: {}", low_confidence_neurons),
            format!("- low freshness neurons: {}", low_freshness_neurons),
        ];
        if compression_policy_versions.is_empty() {
            lines.push("- compression policy versions: none".to_string());
        } else {
            let versions = compression_policy_versions
                .into_iter()
                .map(|(version, count)| format!("{version}={count}"))
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!("- compression policy versions: {}", versions));
        }

        if active_topics_without_neurons.is_empty() {
            lines.push("- active topics without neurons: none".to_string());
        } else {
            lines.push(format!(
                "- active topics without neurons: {}",
                active_topics_without_neurons.join(", ")
            ));
        }

        lines.push("Findings:".to_string());
        if findings.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(
                findings
                    .into_iter()
                    .map(|finding| format!("  - {}", finding)),
            );
        }

        Ok(lines)
    }

    pub fn intuition_calibration_summary(
        &self,
        session_id: &str,
    ) -> Result<Vec<String>, HeptaError> {
        let RuntimeIntuitionCalibrationOverview {
            session_id,
            feedback_record_count,
            learner_applied_update_count,
            learned_topic_hint_count,
            learned_neuron_update_count,
            closed_loop_ready,
            positive_feedback_count,
            negative_feedback_count,
            neutral_feedback_count,
            net_weight_delta,
            average_weight_delta,
            confidence_shift_count,
            average_confidence_shift,
            outcome_counts,
            skill_targets,
            workflow_targets,
            learning_findings,
            recent_feedback,
        } = self.intuition_calibration_overview(session_id)?;

        let mut lines = vec![
            format!("Intuition calibration: session {}", session_id),
            format!("- feedback records: {}", feedback_record_count),
            format!("- closed-loop ready: {}", closed_loop_ready),
            format!(
                "- learner applied updates: {}",
                learner_applied_update_count
            ),
            format!("- learned topic hints: {}", learned_topic_hint_count),
            format!("- learned neuron updates: {}", learned_neuron_update_count),
            format!("- positive feedback: {}", positive_feedback_count),
            format!("- negative feedback: {}", negative_feedback_count),
            format!("- neutral feedback: {}", neutral_feedback_count),
            format!("- net weight delta: {:+.2}", net_weight_delta),
            format!("- average weight delta: {:+.2}", average_weight_delta),
            format!("- confidence shifts: {}", confidence_shift_count),
            format!(
                "- average confidence shift: {:+.2}",
                average_confidence_shift
            ),
            format!("- calibrated skill targets: {}", skill_targets.len()),
            format!("- calibrated workflow targets: {}", workflow_targets.len()),
            "Outcome counts:".to_string(),
        ];

        if outcome_counts.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(
                outcome_counts
                    .iter()
                    .map(|(outcome, count)| format!("  - {}={}", outcome, count)),
            );
        }

        lines.push("Skill targets:".to_string());
        if skill_targets.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(skill_targets.iter().take(8).map(format_calibration_target));
        }

        lines.push("Workflow targets:".to_string());
        if workflow_targets.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(
                workflow_targets
                    .iter()
                    .take(8)
                    .map(format_calibration_target),
            );
        }

        lines.push("Learning findings:".to_string());
        if learning_findings.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(
                learning_findings
                    .into_iter()
                    .map(|finding| format!("  - {}", finding)),
            );
        }

        lines.push("Recent feedback:".to_string());
        if recent_feedback.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(recent_feedback.iter().map(format_calibration_feedback));
        }

        Ok(lines)
    }

    pub fn neuron_activation_summary(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        neuron_limit: usize,
    ) -> Result<Vec<String>, HeptaError> {
        let RuntimeNeuronActivationOverview {
            activations,
            recent_entry_count,
            transcript_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
            active_topic_session_count,
            routed_topic_count,
            ..
        } = self.neuron_activation_overview(
            session_id,
            query_text,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            neuron_limit,
        )?;
        let query_label = query_text
            .map(str::trim)
            .filter(|query| !query.is_empty())
            .map(|query| format!("\"{}\"", summarize_line(query, 48)))
            .unwrap_or_else(|| "none".to_string());
        let mut lines = vec![
            format!("Runtime neuron activation: session {}", session_id),
            format!("- query: {}", query_label),
            format!("- recent entries: {}", recent_entry_count),
            format!("- transcript matches: {}", transcript_matched_count),
            format!("- durable memory hits: {}", durable_memory_hit_count),
            format!("- summary hits: {}", summary_hit_count),
            format!("- active topic sessions: {}", active_topic_session_count),
            format!("- routed topics: {}", routed_topic_count),
            format!("- returned activations: {}", activations.len()),
            format!("- limit: {}", format_limit(neuron_limit)),
            "Activations:".to_string(),
        ];

        if activations.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(activations.iter().map(format_neuron_activation));
        }

        Ok(lines)
    }

    pub fn topic_routing_summary(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        topic_limit: usize,
    ) -> Result<Vec<String>, HeptaError> {
        let RuntimeTopicRoutingOverview {
            decision,
            recent_entry_count,
            transcript_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
            ..
        } = self.topic_routing_overview(
            session_id,
            query_text,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            topic_limit,
        )?;
        let query_label = query_text
            .map(str::trim)
            .filter(|query| !query.is_empty())
            .map(|query| format!("\"{}\"", summarize_line(query, 48)))
            .unwrap_or_else(|| "none".to_string());
        let mut lines = vec![
            format!("Runtime topic routing: session {}", session_id),
            format!("- query: {}", query_label),
            format!("- recent entries: {}", recent_entry_count),
            format!("- transcript matches: {}", transcript_matched_count),
            format!("- durable memory hits: {}", durable_memory_hit_count),
            format!("- summary hits: {}", summary_hit_count),
            format!(
                "- transcript evidence spans: {}",
                decision.source_transcript_spans.len()
            ),
            format!(
                "- active topic sessions: {}",
                decision.active_topic_session_ids.len()
            ),
            format!(
                "- created topic sessions: {}",
                decision.created_topic_session_ids.len()
            ),
            format!(
                "- revived topic sessions: {}",
                decision.revived_topic_session_ids.len()
            ),
            format!(
                "- multi-topic: {}",
                if decision.is_multi_topic() {
                    "yes"
                } else {
                    "no"
                }
            ),
            format!(
                "- primary topic: {}",
                decision
                    .primary_topic_id
                    .as_ref()
                    .map(|topic_id| topic_id.0.as_str())
                    .unwrap_or("none")
            ),
            format!("- limit: {}", format_limit(topic_limit)),
            "Activation scores:".to_string(),
        ];

        if decision.activation_scores.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(
                decision
                    .activation_scores
                    .iter()
                    .map(format_topic_activation_score),
            );
        }

        lines.push("Shift event:".to_string());
        if let Some(shift_event) = &decision.shift_event {
            lines.push(format_topic_shift_event(shift_event));
        } else {
            lines.push("  - none".to_string());
        }

        if let Some(explanation) = &decision.explanation {
            lines.push(format!(
                "Explanation: \"{}\"",
                summarize_line(explanation, 96)
            ));
        }

        Ok(lines)
    }

    pub fn topic_session_summary(&self, session_id: &str) -> Result<Vec<String>, HeptaError> {
        let RuntimeTopicSessionOverview { topic_sessions, .. } =
            self.topic_session_overview(session_id)?;
        let mut lines = vec![
            format!("Runtime topic sessions: session {}", session_id),
            format!("- topic sessions: {}", topic_sessions.len()),
            "Sessions:".to_string(),
        ];

        if topic_sessions.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(topic_sessions.iter().map(format_topic_session));
        }

        Ok(lines)
    }
}

fn format_turn_record(turn: &TurnRecord) -> String {
    let mut parts = vec![
        format!("{}", turn.session_id),
        format!("user=\"{}\"", summarize_line(&turn.input, 48)),
        format!("assistant=\"{}\"", summarize_line(&turn.final_text, 48)),
    ];

    if let Some(tool_name) = &turn.invoked_tool {
        parts.push(format!("tool={}", tool_name));
    }

    if let Some(blocked_reason) = &turn.blocked_reason {
        parts.push(format!(
            "blocked=\"{}\"",
            summarize_line(blocked_reason, 32)
        ));
    }

    format!("  - {}", parts.join(", "))
}

fn format_session_activity_slice(activity: &RuntimeSessionActivitySlice) -> String {
    let mut parts = vec![activity.session.session_id.clone()];

    if activity.session.is_active {
        parts.push("active".to_string());
    }

    if activity.session.archived_at_unix_ms.is_some() {
        parts.push("archived".to_string());
    }

    parts.push(format!(
        "title=\"{}\"",
        summarize_line(&activity.session.title, 32)
    ));
    parts.push(format!(
        "model={}/{}",
        activity.session.model.provider, activity.session.model.model
    ));
    parts.push(format!("history={}", activity.history.len()));
    parts.push(format!("events={}", activity.events.len()));
    parts.push(format!(
        "topic_sessions={}",
        activity.session.topic_session_count
    ));
    parts.push(format!(
        "topic_graph_edges={}",
        activity.session.topic_graph_edge_count
    ));

    if let Some(intent) = &activity.session.last_user_intent_summary {
        parts.push(format!("intent=\"{}\"", summarize_line(intent, 40)));
    }

    if let Some(turn) = activity.history.first() {
        parts.push(format!(
            "latest_user=\"{}\"",
            summarize_line(&turn.input, 32)
        ));
    }

    if let Some(event) = activity.events.last() {
        parts.push(format!("latest_event={:?}", event.event.kind));
    }

    format!("  - {}", parts.join(", "))
}

fn format_transcript_entry(entry: &TranscriptEntry) -> String {
    let mut parts = vec![
        format!("#{}", entry.sequence),
        format!("kind={:?}", entry.kind),
        format!("content=\"{}\"", summarize_line(&entry.content, 56)),
    ];

    if let Some(role) = &entry.role {
        parts.push(format!("role={}", message_role_label(role)));
    }

    if let Some(tool_name) = &entry.tool_name {
        parts.push(format!("tool={}", tool_name));
    }

    format!("  - {}", parts.join(", "))
}

fn format_transcript_span(span: &TranscriptSpan) -> String {
    let mut parts = vec![
        span.session_id.0.clone(),
        format!(
            "range={}..{}",
            span.range.start_sequence, span.range.end_sequence
        ),
        format!("entries={}", span.entry_count),
    ];

    if let Some(excerpt) = &span.excerpt {
        parts.push(format!("excerpt=\"{}\"", summarize_line(excerpt, 56)));
    }

    format!("  - {}", parts.join(", "))
}

fn format_transcript_query_session_tally(tally: &RuntimeTranscriptQuerySessionTally) -> String {
    format!(
        "  - {}: hits={}, entries={}",
        tally.session_id, tally.hit_count, tally.entry_count
    )
}

fn format_memory_record(record: &MemoryRecord) -> String {
    format!(
        "  - {}, {:?}, \"{}\"",
        record.id,
        record.scope,
        summarize_line(&record.content, 72)
    )
}

fn format_active_topic_session(topic_session: &TopicSession) -> String {
    format!(
        "  - {} label=\"{}\" transcripts={} durable_refs={} open_loops={}",
        topic_session.topic_session_id,
        summarize_line(&topic_session.topic_label.0, 60),
        topic_session.linked_transcript_spans.len(),
        topic_session.durable_memory_refs.len(),
        topic_session.open_loops.len(),
    )
}

fn format_neuron_activation(activation: &NeuronActivation) -> String {
    let reason = activation
        .reason
        .as_deref()
        .map(|reason| format!(" reason=\"{}\"", summarize_line(reason, 72)))
        .unwrap_or_default();
    format!(
        "  - neuron={} topic={} direct={:.2} propagated={:.2} inhibited={:.2} final={:.2} topic_sessions={} transcript_spans={} links={}{}",
        activation.neuron_id.0,
        activation.topic_id.0,
        activation.direct_score,
        activation.propagated_score,
        activation.inhibition_score,
        activation.final_score,
        activation.source_topic_session_ids.len(),
        activation.source_transcript_spans.len(),
        activation.source_link_kinds.len(),
        reason,
    )
}

fn format_skill_activation_decision(decision: &SkillActivationDecision) -> String {
    let workflow = decision.workflow_id.as_deref().unwrap_or("none");
    let registry = if decision.exists_in_registry {
        "registered"
    } else {
        "missing"
    };
    let risk = decision
        .risk_tier
        .map(format_intuition_risk_tier)
        .unwrap_or("unknown");
    let action = format_intuition_action_mode(decision.action_mode);
    let reason = decision
        .reason
        .as_deref()
        .map(|reason| format!(" reason=\"{}\"", summarize_line(reason, 72)))
        .unwrap_or_default();
    format!(
        "  - skill={} workflow={} score={:.2} registry={} risk={} confirm={} action={} topics={} neurons={}{}",
        decision.skill_id,
        workflow,
        decision.score,
        registry,
        risk,
        decision.requires_confirmation,
        action,
        decision.source_topic_ids.len(),
        decision.source_neuron_ids.len(),
        reason,
    )
}

fn format_intuition_risk_tier(risk_tier: RiskTier) -> &'static str {
    match risk_tier {
        RiskTier::Low => "low",
        RiskTier::Medium => "medium",
        RiskTier::High => "high",
    }
}

fn format_intuition_action_mode(mode: IntuitionActionMode) -> &'static str {
    match mode {
        IntuitionActionMode::SuggestOnly => "suggest_only",
        IntuitionActionMode::Prepare => "prepare",
        IntuitionActionMode::ExecuteAllowed => "execute_allowed",
    }
}

fn format_intelligence_eval_case(case: &RuntimeIntelligenceEvalCase) -> String {
    let status = if case.passed { "pass" } else { "warn" };
    let contrast = case
        .contrast_focus
        .as_deref()
        .map(|focus| {
            format!(
                " contrast_focus={} expected_signal={}",
                focus,
                case.contrast_expected_signal_direction
                    .as_deref()
                    .unwrap_or("any")
            )
        })
        .unwrap_or_default();
    let warnings = if case.warnings.is_empty() {
        String::new()
    } else {
        format!(" warnings={}", case.warnings.join("|"))
    };
    format!(
        "  - case={} status={} query=\"{}\" router={}{} learned_signals={} learned_pos_neg={}/{} recall_items={} spans={} topics={} active_neurons={} activation_neurons={} skills={} registered_skills={} prepared_skills={} gated_skills={} workflows={} registered_workflows={} prepared_workflows={} gated_workflows={} semantic={}/{} score={}{}",
        case.case_id,
        status,
        summarize_line(&case.query_text, 48),
        case.router_id,
        contrast,
        case.learned_router_signal_count,
        case.learned_positive_signal_count,
        case.learned_negative_signal_count,
        case.recall_ranked_items,
        case.recall_transcript_evidence_spans,
        case.routed_topic_count,
        case.active_neuron_count,
        case.neuron_activation_count,
        case.suggested_skill_count,
        case.registered_skill_decision_count,
        case.prepared_skill_decision_count,
        case.gated_skill_decision_count,
        case.workflow_prior_count,
        case.registered_workflow_prior_count,
        case.prepared_workflow_prior_count,
        case.gated_workflow_prior_count,
        case.semantic_expectation_passed_count,
        case.semantic_expectation_count,
        case.semantic_score,
        warnings,
    )
}

fn format_string_usize_map(values: &BTreeMap<String, usize>) -> String {
    if values.is_empty() {
        return "none".into();
    }
    values
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_calibration_target(target: &RuntimeIntuitionCalibrationTarget) -> String {
    let reason = target
        .latest_reason
        .as_deref()
        .map(|reason| format!(" reason=\"{}\"", summarize_line(reason, 72)))
        .unwrap_or_default();
    let last = target
        .last_feedback_unix_ms
        .map(|value| value.to_string())
        .unwrap_or_else(|| "none".to_string());
    format!(
        "  - {}={} feedback={} positive={} negative={} neutral={} net={:+.2} avg={:+.2} confidence_shifts={} avg_confidence={:+.2} topics={} neurons={} last={}{}",
        target.target_kind,
        target.target_id,
        target.feedback_count,
        target.positive_feedback_count,
        target.negative_feedback_count,
        target.neutral_feedback_count,
        target.net_weight_delta,
        target.average_weight_delta,
        target.confidence_shift_count,
        target.average_confidence_shift,
        target.source_topic_ids.len(),
        target.source_neuron_ids.len(),
        last,
        reason,
    )
}

fn format_calibration_feedback(feedback: &RuntimeIntuitionCalibrationFeedback) -> String {
    let decision = feedback.decision_id.as_deref().unwrap_or("none");
    let skill = feedback.skill_id.as_deref().unwrap_or("none");
    let workflow = feedback.workflow_id.as_deref().unwrap_or("none");
    let reason = feedback
        .reason
        .as_deref()
        .map(|reason| format!(" reason=\"{}\"", summarize_line(reason, 72)))
        .unwrap_or_default();
    format!(
        "  - ts={} outcome={} decision={} skill={} workflow={} weight={:+.2} intent=\"{}\"{}",
        feedback.created_at_unix_ms,
        feedback.outcome,
        decision,
        skill,
        workflow,
        feedback.weight_delta,
        summarize_line(&feedback.user_intent, 56),
        reason,
    )
}

fn format_workflow_prior(prior: &WorkflowPrior) -> String {
    let reason = prior
        .reason
        .as_deref()
        .map(|reason| format!(" reason=\"{}\"", summarize_line(reason, 72)))
        .unwrap_or_default();
    let missing = prior
        .missing_capability
        .as_deref()
        .map(|capability| format!(" missing_capability={}", capability))
        .unwrap_or_default();
    format!(
        "  - workflow={} score={:.2} registered={} action={} requires_confirmation={}{}{}",
        prior.workflow_id,
        prior.score,
        prior.exists_in_registry,
        format_intuition_action_mode(prior.action_mode),
        prior.requires_confirmation,
        missing,
        reason,
    )
}

fn format_topic_activation_score(score: &TopicActivationScore) -> String {
    let matched_terms = if score.matched_terms.is_empty() {
        "none".to_string()
    } else {
        score.matched_terms.join("|")
    };
    let reason = score
        .reason
        .as_deref()
        .map(|reason| format!(" reason=\"{}\"", summarize_line(reason, 72)))
        .unwrap_or_default();
    format!(
        "  - topic={} label=\"{}\" score={:.2} matched_terms={}{}",
        score.topic_id.0,
        summarize_line(&score.topic_label.0, 48),
        score.score,
        matched_terms,
        reason,
    )
}

fn format_topic_shift_event(event: &TopicShiftEvent) -> String {
    format!(
        "  - kind={:?} from={} to={} reason=\"{}\"",
        event.kind,
        event
            .from_topic_id
            .as_ref()
            .map(|topic_id| topic_id.0.as_str())
            .unwrap_or("none"),
        event
            .to_topic_id
            .as_ref()
            .map(|topic_id| topic_id.0.as_str())
            .unwrap_or("none"),
        summarize_line(event.reason.as_deref().unwrap_or("none"), 72)
    )
}

fn format_topic_session(topic_session: &TopicSession) -> String {
    format!(
        "  - id={} topic={} label=\"{}\" status={:?} linked_surface_sessions={} open_loops={} durable_refs={} graph_links={}",
        topic_session.topic_session_id,
        topic_session.topic_id.0,
        summarize_line(&topic_session.topic_label.0, 48),
        topic_session.status,
        topic_session.linked_surface_session_ids.len(),
        topic_session.open_loops.len(),
        topic_session.durable_memory_refs.len(),
        topic_session.graph_edges.len(),
    )
}

fn message_role_label(role: &MessageRole) -> &'static str {
    match role {
        MessageRole::System => "system",
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::Tool => "tool",
    }
}

fn format_limit(limit: usize) -> String {
    if limit == 0 {
        "all available".to_string()
    } else {
        limit.to_string()
    }
}

fn doctor_status_label(status: DoctorStatus) -> &'static str {
    match status {
        DoctorStatus::Ok => "ok",
        DoctorStatus::Warn => "warn",
        DoctorStatus::Fail => "fail",
    }
}

fn format_provider_probe(probe: &DoctorProviderProbe) -> String {
    match &probe.model {
        Some(model) => format!(
            "{}: {} via {}/{} ({})",
            probe.provider_name,
            doctor_status_label(probe.status),
            model.provider,
            model.model,
            probe.detail
        ),
        None => format!(
            "{}: {} ({})",
            probe.provider_name,
            doctor_status_label(probe.status),
            probe.detail
        ),
    }
}

fn format_doctor_check(check: &DoctorCheck) -> String {
    format!(
        "{}: {} ({})",
        check.name,
        doctor_status_label(check.status),
        check.detail
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn activity_summary_respects_session_filter_and_limits() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("alpha first")
            .await
            .expect("alpha turn should succeed");
        runtime
            .run_demo_turn("alpha second")
            .await
            .expect("second alpha turn should succeed");
        runtime
            .run_demo_turn_in_session("beta", "beta only")
            .await
            .expect("beta turn should succeed");

        let summary = runtime
            .activity_summary(Some("alpha"), 1, 2)
            .expect("activity summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Runtime activity: session alpha"));
        assert!(rendered.contains("- recent history entries: 1"));
        assert!(rendered.contains("- recent events: 2"));
        assert!(rendered.contains("alpha second"));
        assert!(!rendered.contains("beta only"));
    }

    #[tokio::test]
    async fn activity_summary_includes_recent_history_and_event_lines() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("capture event line")
            .await
            .expect("turn should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");

        let summary = runtime
            .activity_summary(Some("alpha"), 2, 4)
            .expect("activity summary should succeed");
        let rendered = summary.join("\n");

        assert!(summary.iter().any(|line| line == "Recent history:"));
        assert!(summary.iter().any(|line| line == "Recent events:"));
        assert!(rendered.contains("capture event line"));
        assert!(rendered.contains("SessionRenamed"));
        assert!(rendered.contains("Alpha workspace"));
    }

    #[tokio::test]
    async fn session_activity_summary_covers_multiple_sessions_and_status_flags() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("alpha planning")
            .await
            .expect("alpha turn should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");
        runtime
            .run_demo_turn_in_session("beta", "beta follow-up")
            .await
            .expect("beta turn should succeed");
        runtime
            .route_topics("alpha", Some("alpha planning"), 4, 4, 4, 1)
            .expect("alpha route should succeed");
        runtime
            .archive_session(Some("beta"))
            .expect("archive should succeed");

        let summary = runtime
            .session_activity_summary(1, 2)
            .expect("session activity summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Runtime session activity:"));
        assert!(rendered.contains("- sessions: 2"));
        assert!(rendered.contains("- active sessions: 1"));
        assert!(rendered.contains("- archived sessions: 1"));
        assert!(rendered.contains("- sessions with recent history: 2"));
        assert!(rendered.contains("- sessions with recent events: 2"));
        assert!(rendered.contains("- sessions with topic state: 1"));
        assert!(rendered.contains("- total topic sessions: 1"));
        assert!(rendered.contains("- total topic graph edges: 0"));
        assert!(rendered.contains("alpha, active"));
        assert!(rendered.contains("title=\"Alpha workspace\""));
        assert!(rendered.contains("topic_sessions=1, topic_graph_edges=0"));
        assert!(rendered.contains("beta, archived"));
        assert!(rendered.contains("latest_user=\"beta follow-up\""));
        assert!(rendered.contains("latest_event=SessionArchived"));
    }

    #[tokio::test]
    async fn session_activity_summary_applies_per_session_limits() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("alpha first")
            .await
            .expect("first alpha turn should succeed");
        runtime
            .run_demo_turn("alpha second")
            .await
            .expect("second alpha turn should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");

        let summary = runtime
            .session_activity_summary(1, 1)
            .expect("session activity summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("alpha, active"));
        assert!(rendered.contains("history=1, events=1"));
        assert!(rendered.contains("latest_user=\"alpha second\""));
        assert!(rendered.contains("latest_event=SessionRenamed"));
    }

    #[tokio::test]
    async fn event_digest_summary_groups_recent_events_by_kind_and_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("alpha planning")
            .await
            .expect("alpha turn should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");
        runtime
            .run_demo_turn_in_session("beta", "beta follow-up")
            .await
            .expect("beta turn should succeed");

        let summary = runtime
            .event_digest_summary(0)
            .expect("event digest summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Runtime event digest:"));
        assert!(rendered.contains("- limit: all available"));
        assert!(summary.iter().any(|line| line == "By kind:"));
        assert!(summary.iter().any(|line| line == "By session:"));
        assert!(summary.iter().any(|line| line == "Recent events:"));
        assert!(rendered.contains("SessionRenamed"));
        assert!(rendered.contains("bootstrap"));
        assert!(rendered.contains("alpha:"));
        assert!(rendered.contains("beta:"));
    }

    #[tokio::test]
    async fn event_digest_summary_respects_recent_event_limit() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("alpha planning")
            .await
            .expect("alpha turn should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");

        let summary = runtime
            .event_digest_summary(1)
            .expect("event digest summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("- recent events: 1"));
        assert!(rendered.contains("- event kinds: 1"));
        assert!(rendered.contains("- session scopes: 1"));
        assert!(rendered.contains("latest=SessionRenamed"));
        assert!(rendered.contains("Alpha workspace"));
        assert!(!rendered.contains("bootstrap"));
    }

    #[tokio::test]
    async fn transcript_query_summary_renders_hits_and_metadata() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("alpha transcript needle")
            .await
            .expect("alpha turn should succeed");
        runtime
            .run_demo_turn_in_session("beta", "beta transcript needle")
            .await
            .expect("beta turn should succeed");

        let summary = runtime
            .transcript_query_summary(Some("alpha"), "alpha transcript needle", 2)
            .expect("transcript query summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Runtime transcript query: session alpha"));
        assert!(rendered.contains("- query: \"alpha transcript needle\""));
        assert!(!rendered.contains("- matched spans: 0"));
        assert!(!rendered.contains("- returned hits: 0"));
        assert!(rendered.contains("- matched sessions: 1"));
        assert!(rendered.contains("- returned transcript entries: 2"));
        assert!(rendered.contains("- truncated: no"));
        assert!(summary.iter().any(|line| line == "By session:"));
        assert!(summary.iter().any(|line| line == "Hits:"));
        assert!(rendered.contains("alpha: hits=2, entries=2"));
        assert!(rendered.contains("alpha transcript needle"));
        assert!(!rendered.contains("beta transcript needle"));
    }

    #[tokio::test]
    async fn transcript_query_summary_handles_empty_results() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("alpha transcript needle")
            .await
            .expect("alpha turn should succeed");

        let summary = runtime
            .transcript_query_summary(None, "missing transcript needle", 3)
            .expect("transcript query summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Runtime transcript query: all sessions"));
        assert!(rendered.contains("- query: \"missing transcript needle\""));
        assert!(rendered.contains("- matched spans: 0"));
        assert!(rendered.contains("- returned hits: 0"));
        assert!(rendered.contains("- matched sessions: 0"));
        assert!(rendered.contains("- returned transcript entries: 0"));
        assert!(rendered.contains("- truncated: no"));
        assert!(rendered.contains("- limit: 3"));
        assert!(summary.iter().any(|line| line == "By session:"));
        assert!(summary.iter().any(|line| line == "Hits:"));
        assert!(summary.iter().filter(|line| *line == "  - none").count() >= 2);
    }

    #[tokio::test]
    async fn transcript_query_summary_groups_hits_by_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("shared transcript needle")
            .await
            .expect("alpha turn should succeed");
        runtime
            .run_demo_turn_in_session("beta", "shared transcript needle")
            .await
            .expect("beta turn should succeed");

        let summary = runtime
            .transcript_query_summary(None, "shared transcript needle", 10)
            .expect("transcript query summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Runtime transcript query: all sessions"));
        assert!(rendered.contains("- matched sessions: 2"));
        assert!(summary.iter().any(|line| line == "By session:"));
        assert!(rendered.contains("alpha: hits=2, entries=2"));
        assert!(rendered.contains("beta: hits=2, entries=2"));
    }

    #[tokio::test]
    async fn context_recall_summary_renders_recent_entries_and_hits() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let summary = runtime
            .context_recall_summary("alpha", Some("hello adaptive memory"), 4, 4, 4, true)
            .expect("context recall summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Runtime context recall: session alpha"));
        assert!(rendered.contains("- query: \"hello adaptive memory\""));
        assert!(rendered.contains("- recent entries: 2"));
        assert!(rendered.contains("- transcript matches: 2"));
        assert!(rendered.contains("- transcript hits returned: 2"));
        assert!(rendered.contains("- durable memory hits: 1"));
        assert!(rendered.contains("- transcript evidence spans: "));
        assert!(rendered.contains("- omitted items: 0"));
        assert!(rendered.contains("- cross-session memory: allowed"));
        assert!(summary.iter().any(|line| line == "Recent window:"));
        assert!(summary.iter().any(|line| line == "Transcript hits:"));
        assert!(summary.iter().any(|line| line == "Durable memory hits:"));
        assert!(rendered.contains("role=user"));
        assert!(rendered.contains("role=assistant"));
        assert!(rendered.contains("excerpt=\"hello adaptive memory\""));
    }

    #[tokio::test]
    async fn context_recall_summary_handles_missing_query_hits() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let summary = runtime
            .context_recall_summary("alpha", Some("missing recall string"), 2, 2, 2, false)
            .expect("context recall summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("- query: \"missing recall string\""));
        assert!(rendered.contains("- transcript matches: 0"));
        assert!(rendered.contains("- transcript hits returned: 0"));
        assert!(rendered.contains("- durable memory hits: 0"));
        assert!(rendered.contains("- summary hits: 0"));
        assert!(rendered.contains("- transcript evidence spans: 1"));
        assert!(rendered.contains("- omitted items: 0"));
        assert!(rendered.contains("- cross-session memory: disabled"));
        assert!(rendered.contains("hello adaptive memory"));
        assert!(summary.iter().any(|line| line == "Session summary hits:"));
        assert!(summary.iter().filter(|line| *line == "  - none").count() >= 3);
    }

    #[tokio::test]
    async fn context_recall_summary_includes_active_topic_session_state() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("topic route should succeed");

        let summary = runtime
            .context_recall_summary("alpha", Some("hello adaptive memory"), 4, 4, 4, true)
            .expect("context recall summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("- active topic sessions: 1"));
        assert!(summary.iter().any(|line| line == "Active topic sessions:"));
        assert!(rendered.contains("topic-session-bootstrap:alpha"));
        assert!(rendered.contains("label=\"hello adaptive memory\""));
    }

    #[tokio::test]
    async fn intuition_summary_renders_provenance_aware_top_level_output() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let summary = runtime
            .intuition_summary("alpha", "hello adaptive memory", 4, 4, 4, 2, 2, 2)
            .expect("intuition summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Runtime intuition: session alpha"));
        assert!(rendered.contains("- user intent: \"hello adaptive memory\""));
        assert!(rendered.contains("- recent entries: 2"));
        assert!(rendered.contains("- transcript matches: 2"));
        assert!(rendered.contains("- durable memory hits: 1"));
        assert!(rendered.contains("- transcript evidence spans: "));
        assert!(rendered.contains("- foreground topic sessions: 1"));
        assert!(rendered.contains("- routed topics: 1"));
        assert!(rendered.contains("- returned neuron activations: 1"));
        assert!(rendered.contains("- suggested skills: 1"));
        assert!(rendered.contains("- workflow priors: 1"));
        assert!(
            summary
                .iter()
                .any(|line| line == "Topic activation scores:")
        );
        assert!(summary.iter().any(|line| line == "Neuron activations:"));
        assert!(summary.iter().any(|line| line == "Skill decisions:"));
        assert!(summary.iter().any(|line| line == "Workflow priors:"));
        assert!(rendered.contains("skill=skill-bootstrap:topic-alpha:followup"));
        assert!(rendered.contains("workflow=workflow:memory-review"));
        assert!(rendered.contains("registered=true action=prepare"));
        assert!(rendered.contains("bootstrap intuition synthesized"));
    }

    #[tokio::test]
    async fn provenance_summary_renders_compact_provenance_health_lines() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("topic route should succeed");

        let summary = runtime
            .provenance_summary("alpha")
            .expect("provenance summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Runtime provenance: session alpha"));
        assert!(rendered.contains("- last user intent summary: \"hello adaptive memory\""));
        assert!(rendered.contains("- total topic sessions: 1"));
        assert!(rendered.contains("- active topic sessions with transcript provenance: 1/1"));
        assert!(rendered.contains("- active topic sessions missing transcript provenance: 0"));
        assert!(rendered.contains("- recall transcript evidence spans: "));
        assert!(rendered.contains("- recall omitted items: 0"));
        assert!(rendered.contains("- intuition transcript evidence spans: "));
        assert!(rendered.contains("- intuition foreground topic sessions: 1"));
    }

    #[tokio::test]
    async fn intelligence_eval_summary_renders_replay_quality_rollup() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");

        let summary = runtime
            .intelligence_eval_summary("alpha", 2, 6, 6, 6, 2, 2, 2)
            .expect("eval summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Runtime intelligence eval: session alpha"));
        assert!(rendered.contains("- evaluated cases: 2"));
        assert!(rendered.contains("- passed cases: 2"));
        assert!(rendered.contains("- failed cases: 0"));
        assert!(rendered.contains("- total recall ranked items: "));
        assert!(rendered.contains("- total transcript evidence spans: "));
        assert!(rendered.contains("- total active neurons: "));
        assert!(rendered.contains("- total routed topics: "));
        assert!(rendered.contains("- total neuron activations: "));
        assert!(rendered.contains("- total suggested skills: "));
        assert!(rendered.contains("- total workflow priors: "));
        assert!(rendered.contains("- registered workflow priors: "));
        assert!(rendered.contains("- prepared workflow priors: "));
        assert!(rendered.contains("- gated workflow priors: "));
        assert!(rendered.contains("- feedback records: 0"));
        assert!(rendered.contains("- feedback net weight delta: +0.00"));
        assert!(rendered.contains("- calibrated skill targets: 0"));
        assert!(rendered.contains("- calibrated workflow targets: 0"));
        assert!(rendered.contains("- prepared skill decisions: "));
        assert!(rendered.contains("- gated skill decisions: "));
        assert!(rendered.contains("- semantic expectations: "));
        assert!(rendered.contains("- semantic score: 100"));
        assert!(summary.iter().any(|line| line == "Cases:"));
        assert!(rendered.contains("status=pass"));
        assert!(rendered.contains("active_neurons="));
        assert!(rendered.contains("activation_neurons="));
        assert!(rendered.contains("prepared_skills="));
        assert!(rendered.contains("gated_skills="));
        assert!(rendered.contains("registered_workflows="));
        assert!(rendered.contains("prepared_workflows="));
        assert!(rendered.contains("gated_workflows="));
        assert!(rendered.contains("score=100"));
        assert!(rendered.contains("hello adaptive memory"));
        assert!(rendered.contains("rust worker pipeline"));
    }

    #[tokio::test]
    async fn knowledge_graph_dry_run_summary_renders_no_write_report() {
        let runtime = RuntimeKernel::new();

        let summary = runtime
            .knowledge_graph_dry_run_summary()
            .expect("kg dry-run summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Hepta KG dry-run: ready"));
        assert!(rendered.contains("- contract: hepta-intelligence-memory-kg-write-candidate-v0"));
        assert!(rendered.contains("- write candidates: "));
        assert!(rendered.contains("- live write enabled: 0"));
        assert!(rendered.contains("- external side effects enabled: 0"));
        assert!(rendered.contains("- all plans are dry-run: true"));
        assert!(rendered.contains("- no live write enabled: true"));
        assert!(rendered.contains("- no external side effects: true"));
        assert!(rendered.contains("Candidates:"));
    }

    #[tokio::test]
    async fn knowledge_graph_adapter_dry_run_summary_renders_no_external_write_report() {
        let runtime = RuntimeKernel::new();

        let summary = runtime
            .knowledge_graph_adapter_dry_run_summary()
            .expect("kg adapter dry-run summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Hepta KG adapter dry-run: ready"));
        assert!(rendered.contains("- contract: hepta-kg-external-adapter-dry-run-v0"));
        assert!(rendered.contains("- supported adapters: 3"));
        assert!(rendered.contains("- network calls enabled: 0"));
        assert!(rendered.contains("- external writes enabled: 0"));
        assert!(rendered.contains("- live writes enabled: 0"));
        assert!(rendered.contains("- no network calls enabled: true"));
        assert!(rendered.contains("- no external writes enabled: true"));
        assert!(rendered.contains("- no live writes enabled: true"));
        assert!(rendered.contains("adapter=graphiti"));
        assert!(rendered.contains("adapter=neo4j"));
        assert!(rendered.contains("adapter=cocoindex"));
    }

    #[tokio::test]
    async fn knowledge_graph_adapter_staging_gate_summary_renders_closed_gate_report() {
        let runtime = RuntimeKernel::new();

        let summary = runtime
            .knowledge_graph_adapter_staging_gate_summary()
            .expect("kg adapter staging gate summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Hepta KG adapter staging gate: ready"));
        assert!(rendered.contains("- contract: hepta-kg-external-adapter-staging-gate-v0"));
        assert!(rendered.contains("- supported adapters: 3"));
        assert!(rendered.contains("- staging ready: 0"));
        assert!(rendered.contains("- network calls enabled: 0"));
        assert!(rendered.contains("- external writes enabled: 0"));
        assert!(rendered.contains("- live writes enabled: 0"));
        assert!(rendered.contains("- closed by default: true"));
        assert!(rendered.contains("- operator review required: true"));
        assert!(rendered.contains("- rollback plan required: true"));
        assert!(rendered.contains("- post-write validation required: true"));
        assert!(rendered.contains("gate=HEPTA_KG_GRAPHITI_STAGING"));
        assert!(rendered.contains("gate=HEPTA_KG_NEO4J_STAGING"));
        assert!(rendered.contains("gate=HEPTA_KG_COCOINDEX_STAGING"));
    }

    #[tokio::test]
    async fn knowledge_graph_adapter_client_summary_renders_disabled_client_report() {
        let runtime = RuntimeKernel::new();

        let summary = runtime
            .knowledge_graph_adapter_client_summary()
            .expect("kg adapter client summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Hepta KG adapter clients: ready"));
        assert!(rendered.contains("- contract: hepta-kg-external-adapter-client-v0"));
        assert!(rendered.contains("- supported adapters: 3"));
        assert!(rendered.contains("- client audits: "));
        assert!(rendered.contains("- denied clients: "));
        assert!(rendered.contains("- network calls attempted: 0"));
        assert!(rendered.contains("- external writes attempted: 0"));
        assert!(rendered.contains("- live writes attempted: 0"));
        assert!(rendered.contains("- persisted records: 0"));
        assert!(rendered.contains("- denied by default: true"));
        assert!(rendered.contains("disabled-graphiti-adapter-client"));
        assert!(rendered.contains("disabled-neo4j-adapter-client"));
        assert!(rendered.contains("disabled-cocoindex-adapter-client"));
    }

    #[tokio::test]
    async fn knowledge_graph_adapter_config_env_summary_renders_default_closed_report() {
        let runtime = RuntimeKernel::new();

        let summary = runtime
            .knowledge_graph_adapter_config_env_summary()
            .expect("kg adapter config env summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Hepta KG adapter config env: ready"));
        assert!(rendered.contains("- contract: hepta-kg-external-adapter-config-env-v0"));
        assert!(rendered.contains("- supported adapters: 3"));
        assert!(rendered.contains("- config reads: 3"));
        assert!(rendered.contains("- feature enabled: 0"));
        assert!(rendered.contains("- endpoints configured: 0"));
        assert!(rendered.contains("- credentials configured: 0"));
        assert!(rendered.contains("- network allowlisted: 0"));
        assert!(rendered.contains("- external write allowlisted: 0"));
        assert!(rendered.contains("- live writes requested: 0"));
        assert!(rendered.contains("- credential values captured: 0"));
        assert!(rendered.contains("- network calls attempted: 0"));
        assert!(rendered.contains("- external writes attempted: 0"));
        assert!(rendered.contains("- live writes attempted: 0"));
        assert!(rendered.contains("- configs closed by default: true"));
        assert!(rendered.contains("- no credential values captured: true"));
        assert!(rendered.contains("gate_key=HEPTA_KG_GRAPHITI_STAGING"));
        assert!(rendered.contains("credential_ref_key=HEPTA_KG_NEO4J_CREDENTIAL_REF"));
        assert!(rendered.contains("endpoint_key=HEPTA_KG_COCOINDEX_ENDPOINT"));
    }

    #[tokio::test]
    async fn knowledge_graph_recall_plan_summary_renders_read_only_report() {
        let runtime = RuntimeKernel::new();

        let summary = runtime
            .knowledge_graph_recall_plan_summary()
            .expect("kg recall plan summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Hepta KG recall plan: ready"));
        assert!(rendered.contains("- contract: hepta-kg-read-recall-v0"));
        assert!(rendered.contains("- recall queries: 2"));
        assert!(rendered.contains("- entity matches: "));
        assert!(rendered.contains("- relation neighborhoods: "));
        assert!(rendered.contains("- timeline slices: "));
        assert!(rendered.contains("- evidence paths: "));
        assert!(rendered.contains("- external reads enabled: 0"));
        assert!(rendered.contains("- network calls enabled: 0"));
        assert!(rendered.contains("- live writes enabled: 0"));
        assert!(rendered.contains("- all plans are read-only: true"));
        assert!(rendered.contains("- no external reads enabled: true"));
        assert!(rendered.contains("- no network calls enabled: true"));
        assert!(rendered.contains("- no live writes enabled: true"));
        assert!(rendered.contains("Recall plans:"));
        assert!(rendered.contains("Entity matches:"));
    }

    #[tokio::test]
    async fn knowledge_graph_context_recall_bridge_summary_renders_no_injection_report() {
        let runtime = RuntimeKernel::new();

        let summary = runtime
            .knowledge_graph_context_recall_bridge_summary()
            .expect("kg context recall bridge summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Hepta KG context recall bridge: ready"));
        assert!(
            rendered.contains("- contract: hepta-intelligence-memory-kg-context-recall-bridge-v0")
        );
        assert!(rendered.contains("- kg recall contract: hepta-kg-read-recall-v0"));
        assert!(rendered.contains("- context recall items: "));
        assert!(rendered.contains("- external reads enabled: 0"));
        assert!(rendered.contains("- network calls enabled: 0"));
        assert!(rendered.contains("- live writes enabled: 0"));
        assert!(rendered.contains("- model invoked: false"));
        assert!(rendered.contains("- context injection performed: false"));
        assert!(rendered.contains("- all items have KG source: true"));
        assert!(rendered.contains("- transcript provenance preserved: true"));
        assert!(rendered.contains("- no context injection performed: true"));
        assert!(rendered.contains("KG context recall items:"));
    }

    #[tokio::test]
    async fn knowledge_graph_recall_evaluation_summary_renders_quality_gate_report() {
        let runtime = RuntimeKernel::new();

        let summary = runtime
            .knowledge_graph_recall_evaluation_summary()
            .expect("kg recall evaluation summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Hepta KG recall evaluation: ready"));
        assert!(rendered.contains("- contract: hepta-intelligence-memory-kg-recall-evaluation-v0"));
        assert!(rendered.contains("- kg recall contract: hepta-kg-read-recall-v0"));
        assert!(rendered.contains(
            "- kg context bridge contract: hepta-intelligence-memory-kg-context-recall-bridge-v0"
        ));
        assert!(rendered.contains("- failed cases: 0"));
        assert!(rendered.contains("- duplicate source memory ids: 0"));
        assert!(rendered.contains("- score order violations: 0"));
        assert!(rendered.contains("- coverage bp: 10000"));
        assert!(rendered.contains("- precision proxy bp: 10000"));
        assert!(rendered.contains("- score stability bp: 10000"));
        assert!(rendered.contains("- external reads enabled: 0"));
        assert!(rendered.contains("- network calls enabled: 0"));
        assert!(rendered.contains("- live writes enabled: 0"));
        assert!(rendered.contains("- model invoked: false"));
        assert!(rendered.contains("- context injection performed: false"));
        assert!(rendered.contains("- source memory ids unique: true"));
        assert!(rendered.contains("- scores stably ordered: true"));
        assert!(rendered.contains("Evaluation cases:"));
    }

    #[tokio::test]
    async fn knowledge_graph_context_injection_readiness_summary_renders_blocking_gate() {
        let runtime = RuntimeKernel::new();

        let summary = runtime
            .knowledge_graph_context_injection_readiness_summary()
            .expect("kg context injection readiness summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Hepta KG context injection readiness: blocked"));
        assert!(
            rendered.contains(
                "- contract: hepta-intelligence-memory-kg-context-injection-readiness-v0"
            )
        );
        assert!(rendered.contains(
            "- kg recall evaluation contract: hepta-intelligence-memory-kg-recall-evaluation-v0"
        ));
        assert!(rendered.contains("- failed cases: 0"));
        assert!(rendered.contains("- coverage bp: 10000"));
        assert!(rendered.contains("- quality threshold bp: 9000"));
        assert!(rendered.contains("- quality gate ready: true"));
        assert!(rendered.contains("- operator approved: false"));
        assert!(rendered.contains("- shadow rank enabled: false"));
        assert!(rendered.contains("- rollback plan ready: false"));
        assert!(rendered.contains("- kill switch ready: false"));
        assert!(rendered.contains("- context injection allowed: false"));
        assert!(rendered.contains("- context injection performed: false"));
        assert!(rendered.contains("- prompt preview rendered: false"));
        assert!(rendered.contains("- model invoked: false"));
        assert!(rendered.contains("- external reads enabled: 0"));
        assert!(rendered.contains("- network calls enabled: 0"));
        assert!(rendered.contains("- live writes enabled: 0"));
        assert!(rendered.contains("- recall evaluation ready: true"));
        assert!(rendered.contains("- activation blocked without operator approval: true"));
        assert!(rendered.contains("- no context injection performed: true"));
        assert!(rendered.contains("Readiness blockers:"));
        assert!(rendered.contains("MissingOperatorApproval"));
        assert!(rendered.contains("ShadowRankNotEnabled"));
        assert!(rendered.contains("InjectionDisabledByDefault"));
    }

    #[tokio::test]
    async fn intuition_calibration_summary_renders_feedback_rollup() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("topic route should succeed");
        let overview = runtime
            .intuition_overview("alpha", "hello adaptive memory", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed");
        let skill = overview.bundle.skill_decisions[0].clone();
        let workflow = overview.bundle.workflow_priors[0].clone();
        runtime
            .record_intuition_feedback(
                "alpha",
                "hello adaptive memory",
                hepta_core::IntuitionFeedbackOutcome::Accepted,
                Some(skill.skill_id.as_str()),
                Some(workflow.workflow_id.as_str()),
                skill.source_topic_ids,
                skill.source_neuron_ids,
                Some("accepted follow-up"),
            )
            .expect("feedback should record");

        let summary = runtime
            .intuition_calibration_summary("alpha")
            .expect("calibration summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Intuition calibration: session alpha"));
        assert!(rendered.contains("- feedback records: 1"));
        assert!(rendered.contains("- positive feedback: 1"));
        assert!(rendered.contains("- net weight delta: +0.12"));
        assert!(rendered.contains("Outcome counts:"));
        assert!(rendered.contains("  - accepted=1"));
        assert!(rendered.contains("Skill targets:"));
        assert!(rendered.contains("Workflow targets:"));
        assert!(rendered.contains("Recent feedback:"));
        assert!(rendered.contains("skill=skill-bootstrap:topic-alpha:followup"));
        assert!(rendered.contains("workflow=workflow:memory-review"));
        assert!(rendered.contains("outcome=accepted"));
    }

    #[tokio::test]
    async fn neuron_lifecycle_summary_renders_health_findings() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("topic route should succeed");
        runtime
            .compress_active_topics_to_neurons("alpha", 2)
            .expect("compression should succeed");

        let summary = runtime
            .neuron_lifecycle_summary("alpha")
            .expect("lifecycle summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Neuron lifecycle: session alpha"));
        assert!(rendered.contains("- healthy: true"));
        assert!(rendered.contains("- stored neurons: 1"));
        assert!(rendered.contains("- neurons with transcript provenance: 1"));
        assert!(rendered.contains("- neurons with evidence digest: 1"));
        assert!(rendered.contains("- active topics without neurons: none"));
        assert!(summary.iter().any(|line| line == "Findings:"));
        assert!(rendered.contains("  - none"));
    }

    #[tokio::test]
    async fn neuron_activation_summary_renders_direct_activation_metadata() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let summary = runtime
            .neuron_activation_summary("alpha", Some("hello adaptive memory"), 4, 4, 4, 2)
            .expect("neuron activation summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Runtime neuron activation: session alpha"));
        assert!(rendered.contains("- query: \"hello adaptive memory\""));
        assert!(rendered.contains("- recent entries: 2"));
        assert!(rendered.contains("- transcript matches: 2"));
        assert!(rendered.contains("- durable memory hits: 1"));
        assert!(rendered.contains("- active topic sessions: 1"));
        assert!(rendered.contains("- routed topics: 1"));
        assert!(rendered.contains("- returned activations: 1"));
        assert!(summary.iter().any(|line| line == "Activations:"));
        assert!(rendered.contains("neuron=neuron-alpha"));
        assert!(rendered.contains("topic=topic-alpha"));
        assert!(rendered.contains("direct=0.90"));
        assert!(rendered.contains("propagated=0.00"));
        assert!(rendered.contains("inhibited=0.00"));
        assert!(rendered.contains("final=0.90"));
        assert!(rendered.contains("transcript_spans="));
        assert!(rendered.contains("via routed topic session"));
    }

    #[tokio::test]
    async fn topic_routing_summary_renders_bootstrap_topic_decision() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let summary = runtime
            .topic_routing_summary("alpha", Some("hello adaptive memory"), 4, 4, 4, 2)
            .expect("topic routing summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Runtime topic routing: session alpha"));
        assert!(rendered.contains("- query: \"hello adaptive memory\""));
        assert!(rendered.contains("- recent entries: 2"));
        assert!(rendered.contains("- transcript matches: 2"));
        assert!(rendered.contains("- durable memory hits: 1"));
        assert!(rendered.contains("- transcript evidence spans: "));
        assert!(rendered.contains("- active topic sessions: 1"));
        assert!(rendered.contains("- created topic sessions: 0"));
        assert!(rendered.contains("- multi-topic: no"));
        assert!(rendered.contains("- primary topic: topic-alpha"));
        assert!(summary.iter().any(|line| line == "Activation scores:"));
        assert!(summary.iter().any(|line| line == "Shift event:"));
        assert!(rendered.contains("topic=topic-alpha"));
        assert!(rendered.contains("label=\"hello adaptive memory\""));
        assert!(rendered.contains("matched_terms=hello|adaptive|memory"));
        assert!(rendered.contains("kind=Stayed"));
        assert!(rendered.contains("bootstrap topic routing anchored session"));
    }

    #[tokio::test]
    async fn topic_session_summary_renders_bootstrap_topic_session_state() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("route topics should succeed");

        let summary = runtime
            .topic_session_summary("alpha")
            .expect("topic session summary should succeed");
        let rendered = summary.join("\n");

        assert!(rendered.contains("Runtime topic sessions: session alpha"));
        assert!(rendered.contains("- topic sessions: 1"));
        assert!(summary.iter().any(|line| line == "Sessions:"));
        assert!(rendered.contains("id=topic-session-bootstrap:alpha"));
        assert!(rendered.contains("topic=topic-alpha"));
        assert!(rendered.contains("label=\"hello adaptive memory\""));
        assert!(rendered.contains("status=Active"));
        assert!(rendered.contains("linked_surface_sessions=1"));
    }

    #[tokio::test]
    async fn topic_routing_and_session_summaries_surface_shift_and_revive_state() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");

        let shifted = runtime
            .topic_routing_summary("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("shift summary should succeed")
            .join("\n");
        assert!(shifted.contains("- created topic sessions: 1"));
        assert!(shifted.contains("- revived topic sessions: 0"));
        assert!(shifted.contains("kind=Shifted"));
        assert!(shifted.contains("topic-alpha-rust-worker-pipeline"));

        let revived = runtime
            .topic_routing_summary("alpha", Some("hello memory"), 6, 6, 6, 1)
            .expect("revive summary should succeed")
            .join("\n");
        assert!(revived.contains("- created topic sessions: 0"));
        assert!(revived.contains("- revived topic sessions: 1"));
        assert!(revived.contains("kind=Revived"));
        assert!(revived.contains("topic-alpha"));

        let session_summary = runtime
            .topic_session_summary("alpha")
            .expect("topic session summary should succeed")
            .join("\n");
        assert!(session_summary.contains("- topic sessions: 2"));
        assert!(session_summary.contains("id=topic-session-bootstrap:alpha topic=topic-alpha label=\"hello adaptive memory\" status=Active"));
        assert!(session_summary.contains("id=topic-session-bootstrap:alpha:rust-worker-pipeline topic=topic-alpha-rust-worker-pipeline label=\"rust worker pipeline\" status=Dormant"));
    }

    #[tokio::test]
    async fn topic_routing_and_neuron_summaries_surface_multi_topic_coactivation() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let routing_summary = runtime
            .topic_routing_summary(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("routing summary should succeed")
            .join("\n");
        assert!(routing_summary.contains("- active topic sessions: 2"));
        assert!(routing_summary.contains("- created topic sessions: 0"));
        assert!(routing_summary.contains("- revived topic sessions: 1"));
        assert!(routing_summary.contains("- multi-topic: yes"));
        assert!(routing_summary.contains("kind=CoActivated"));
        assert!(routing_summary.contains("topic=topic-alpha"));
        assert!(routing_summary.contains("topic=topic-alpha-rust-worker-pipeline"));

        let neuron_summary = runtime
            .neuron_activation_summary(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                3,
            )
            .expect("neuron summary should succeed")
            .join("\n");
        assert!(neuron_summary.contains("- active topic sessions: 2"));
        assert!(neuron_summary.contains("- routed topics: 2"));
        assert!(neuron_summary.contains("- returned activations: 2"));
        assert!(neuron_summary.contains("neuron=neuron-alpha"));
        assert!(neuron_summary.contains("neuron=neuron-alpha-rust-worker-pipeline"));
        assert!(neuron_summary.contains("links=1"));
        assert!(neuron_summary.contains("inhibited=0.00"));
        assert!(!neuron_summary.contains("propagated=0.00"));

        let session_summary = runtime
            .topic_session_summary("alpha")
            .expect("topic session summary should succeed")
            .join("\n");
        assert!(session_summary.contains("status=Active"));
        assert!(session_summary.contains("id=topic-session-bootstrap:alpha topic=topic-alpha label=\"hello adaptive memory\" status=Active"));
        assert!(session_summary.contains("id=topic-session-bootstrap:alpha:rust-worker-pipeline topic=topic-alpha-rust-worker-pipeline label=\"rust worker pipeline\" status=Active"));
    }

    #[tokio::test]
    async fn topic_routing_summary_surfaces_implicit_multi_topic_detection_without_delimiters() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let summary = runtime
            .topic_routing_summary(
                "alpha",
                Some("continue hello adaptive memory rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("routing summary should succeed")
            .join("\n");

        assert!(summary.contains("- active topic sessions: 2"));
        assert!(summary.contains("- created topic sessions: 0"));
        assert!(summary.contains("- revived topic sessions: 1"));
        assert!(summary.contains("- multi-topic: yes"));
        assert!(summary.contains("kind=CoActivated"));
        assert!(summary.contains("implicitly kept") || summary.contains("implicitly revived"));
    }

    #[tokio::test]
    async fn topic_routing_summary_surfaces_semantic_mixed_turn_detection() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let summary = runtime
            .topic_routing_summary(
                "alpha",
                Some("continue adaptive recall while checking executor flow"),
                8,
                8,
                8,
                2,
            )
            .expect("routing summary should succeed")
            .join("\n");

        assert!(summary.contains("- active topic sessions: 2"));
        assert!(summary.contains("- created topic sessions: 0"));
        assert!(summary.contains("- revived topic sessions: 1"));
        assert!(summary.contains("- multi-topic: yes"));
        assert!(summary.contains("kind=CoActivated"));
        assert!(summary.contains("semantic"));
        assert!(summary.contains("hello adaptive memory"));
        assert!(summary.contains("rust worker pipeline"));
    }

    #[tokio::test]
    async fn topic_routing_and_session_summaries_surface_merge_and_split_state() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let merged = runtime
            .topic_routing_summary(
                "alpha",
                Some("merge hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("merge summary should succeed")
            .join("\n");
        assert!(merged.contains("- active topic sessions: 1"));
        assert!(merged.contains("- created topic sessions: 1"));
        assert!(merged.contains("kind=Merged"));
        assert!(merged.contains("topic-alpha-hello-adaptive-memory-rust-worker-pipeline"));

        let split = runtime
            .topic_routing_summary(
                "alpha",
                Some("split hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("split summary should succeed")
            .join("\n");
        assert!(split.contains("- active topic sessions: 2"));
        assert!(split.contains("- created topic sessions: 0"));
        assert!(split.contains("- revived topic sessions: 2"));
        assert!(split.contains("kind=Split"));

        let session_summary = runtime
            .topic_session_summary("alpha")
            .expect("topic session summary should succeed")
            .join("\n");
        assert!(session_summary.contains("id=topic-session-bootstrap:alpha topic=topic-alpha label=\"hello adaptive memory\" status=Active"));
        assert!(session_summary.contains("id=topic-session-bootstrap:alpha:rust-worker-pipeline topic=topic-alpha-rust-worker-pipeline label=\"rust worker pipeline\" status=Active"));
        assert!(session_summary.contains("id=topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline topic=topic-alpha-hello-adaptive-memory-rust-worker-pipeline label=\"hello adaptive memory + rust worker pipeline\" status=Dormant"));
    }

    #[tokio::test]
    async fn topic_routing_summary_surfaces_graph_expansion_from_component_to_composite_topic() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("merge hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("merge route should succeed");

        let summary = runtime
            .topic_routing_summary("alpha", Some("hello adaptive memory"), 8, 8, 8, 2)
            .expect("routing summary should succeed")
            .join("\n");

        assert!(summary.contains("- active topic sessions: 2"));
        assert!(summary.contains("- created topic sessions: 0"));
        assert!(summary.contains("- revived topic sessions: 1"));
        assert!(summary.contains("kind=CoActivated"));
        assert!(summary.contains("bootstrap topic graph expanded"));
        assert!(summary.contains("hello adaptive memory + rust worker pipeline"));
    }

    #[tokio::test]
    async fn topic_routing_and_session_summaries_surface_stored_graph_edges() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("mixed route should succeed");

        let routing_summary = runtime
            .topic_routing_summary("alpha", Some("hello adaptive memory"), 8, 8, 8, 2)
            .expect("routing summary should succeed")
            .join("\n");
        assert!(routing_summary.contains("kind=CoActivated"));
        assert!(
            routing_summary.contains("topic graph expansion")
                || routing_summary.contains("bootstrap topic graph expanded")
        );

        let session_summary = runtime
            .topic_session_summary("alpha")
            .expect("topic session summary should succeed")
            .join("\n");
        assert!(session_summary.contains("graph_links=1"));
    }

    #[tokio::test]
    async fn neuron_activation_summary_surfaces_inhibitory_suppression_for_contrast_query() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let summary = runtime
            .neuron_activation_summary(
                "alpha",
                Some("hello adaptive memory but not rust worker pipeline"),
                8,
                8,
                8,
                3,
            )
            .expect("neuron summary should succeed")
            .join("\n");

        assert!(summary.contains("neuron=neuron-alpha"));
        assert!(summary.contains("neuron=neuron-alpha-rust-worker-pipeline"));
        assert!(summary.contains("inhibited=0.00"));
        assert!(
            summary
                .lines()
                .any(|line| line.contains("inhibited=") && !line.contains("inhibited=0.00"))
        );
        assert!(summary.contains("links=1"));
    }
}

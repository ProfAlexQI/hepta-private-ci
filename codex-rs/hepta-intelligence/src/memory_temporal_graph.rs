use hepta_core::MemorySourceKind;
use hepta_core::MemorySourceSpan;
use hepta_core::MemoryTemporalValidity;
use hepta_core::TemporalFactEdge;
use hepta_core::TranscriptRange;
use serde::Deserialize;
use serde::Serialize;

pub const MEMORY_TEMPORAL_GRAPH_V1_CONTRACT: &str = "hepta-intelligence-temporal-graph-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemporalFactState {
    Current,
    Past,
    Superseded,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemporalGraphFact {
    pub edge: TemporalFactEdge,
    pub state: TemporalFactState,
    pub supersedes: Option<String>,
    pub superseded_by: Option<String>,
    pub inhibition_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalGraphChecks {
    pub current_fact_present: bool,
    pub past_fact_present: bool,
    pub superseded_fact_present: bool,
    pub supersession_link_present: bool,
    pub invalid_current_fact_excluded: bool,
    pub provenance_complete: bool,
    pub conflict_or_inhibition_visible: bool,
    pub no_external_side_effects: bool,
}

impl TemporalGraphChecks {
    pub fn ready(&self) -> bool {
        self.current_fact_present
            && self.past_fact_present
            && self.superseded_fact_present
            && self.supersession_link_present
            && self.invalid_current_fact_excluded
            && self.provenance_complete
            && self.conflict_or_inhibition_visible
            && self.no_external_side_effects
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemporalGraphReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p3_ready: bool,
    pub native_rewrite: bool,
    pub sample_run: bool,
    pub now_unix_ms: u64,
    pub fact_count: usize,
    pub current_fact_ids: Vec<String>,
    pub past_fact_ids: Vec<String>,
    pub superseded_fact_ids: Vec<String>,
    pub graph: Vec<TemporalGraphFact>,
    pub checks: TemporalGraphChecks,
    pub next_phase: &'static str,
}

pub fn memory_temporal_graph_sample_report(sample_run: bool) -> TemporalGraphReport {
    let now = 1_800_000_020_000;
    let graph = sample_temporal_graph(now);
    let current_fact_ids = graph
        .iter()
        .filter(|fact| fact.state == TemporalFactState::Current)
        .map(|fact| fact.edge.id.clone())
        .collect::<Vec<_>>();
    let past_fact_ids = graph
        .iter()
        .filter(|fact| fact.state == TemporalFactState::Past)
        .map(|fact| fact.edge.id.clone())
        .collect::<Vec<_>>();
    let superseded_fact_ids = graph
        .iter()
        .filter(|fact| fact.state == TemporalFactState::Superseded)
        .map(|fact| fact.edge.id.clone())
        .collect::<Vec<_>>();
    let checks = TemporalGraphChecks {
        current_fact_present: !current_fact_ids.is_empty(),
        past_fact_present: !past_fact_ids.is_empty(),
        superseded_fact_present: !superseded_fact_ids.is_empty(),
        supersession_link_present: graph
            .iter()
            .any(|fact| fact.supersedes.is_some() || fact.superseded_by.is_some()),
        invalid_current_fact_excluded: graph
            .iter()
            .filter(|fact| fact.state == TemporalFactState::Current)
            .all(|fact| fact.edge.validity.currently_valid_at(now)),
        provenance_complete: graph.iter().all(|fact| {
            !fact.edge.source_spans.is_empty()
                && fact
                    .edge
                    .source_spans
                    .iter()
                    .all(MemorySourceSpan::is_traceable)
        }),
        conflict_or_inhibition_visible: graph.iter().any(|fact| {
            fact.inhibition_reason
                .as_deref()
                .is_some_and(|value| !value.is_empty())
        }),
        no_external_side_effects: true,
    };
    let p3_ready = checks.ready();

    TemporalGraphReport {
        product: "Hepta",
        command: "memory-temporal-graph",
        contract: MEMORY_TEMPORAL_GRAPH_V1_CONTRACT,
        status: if p3_ready { "ready" } else { "attention" },
        p3_ready,
        native_rewrite: true,
        sample_run,
        now_unix_ms: now,
        fact_count: graph.len(),
        current_fact_ids,
        past_fact_ids,
        superseded_fact_ids,
        graph,
        checks,
        next_phase: "P4 short-term offload with refs/*.md and Mermaid node_id canvas",
    }
}

fn sample_temporal_graph(now: u64) -> Vec<TemporalGraphFact> {
    let old_span = sample_span("span-old-memory-direction", 1, 2);
    let current_span = sample_span("span-current-memory-direction", 10, 11);
    let past_span = sample_span("span-past-p0-completed", 20, 21);
    let old_edge = TemporalFactEdge {
        id: "fact-memory-flat-summary".into(),
        source_unit_id: "unit-old-preference".into(),
        subject_entity_id: "entity:hepta".into(),
        predicate: "memory_strategy".into(),
        object_entity_id: "entity:flat_summary".into(),
        validity: MemoryTemporalValidity {
            valid_from_unix_ms: Some(now - 20_000),
            valid_until_unix_ms: Some(now - 1),
            observed_at_unix_ms: Some(now - 20_000),
            last_revalidated_unix_ms: Some(now - 10_000),
        },
        confidence_ppm: 420_000,
        source_spans: vec![old_span],
    };
    let current_edge = TemporalFactEdge {
        id: "fact-memory-transcript-backed-kernel".into(),
        source_unit_id: "unit-current-preference".into(),
        subject_entity_id: "entity:hepta".into(),
        predicate: "memory_strategy".into(),
        object_entity_id: "entity:transcript_backed_kernel".into(),
        validity: MemoryTemporalValidity {
            valid_from_unix_ms: Some(now),
            valid_until_unix_ms: None,
            observed_at_unix_ms: Some(now),
            last_revalidated_unix_ms: Some(now),
        },
        confidence_ppm: 940_000,
        source_spans: vec![current_span],
    };
    let past_edge = TemporalFactEdge {
        id: "fact-p0-memory-kernel-landed".into(),
        source_unit_id: "unit-p0-completed".into(),
        subject_entity_id: "entity:memory_kernel_p0".into(),
        predicate: "completed_before".into(),
        object_entity_id: "entity:memory_atom_p1".into(),
        validity: MemoryTemporalValidity {
            valid_from_unix_ms: Some(now - 5_000),
            valid_until_unix_ms: Some(now - 2_000),
            observed_at_unix_ms: Some(now - 5_000),
            last_revalidated_unix_ms: Some(now),
        },
        confidence_ppm: 900_000,
        source_spans: vec![past_span],
    };

    vec![
        TemporalGraphFact {
            edge: old_edge,
            state: TemporalFactState::Superseded,
            supersedes: None,
            superseded_by: Some("fact-memory-transcript-backed-kernel".into()),
            inhibition_reason: Some(
                "old flat-summary strategy must not activate current recall".into(),
            ),
        },
        TemporalGraphFact {
            edge: current_edge,
            state: TemporalFactState::Current,
            supersedes: Some("fact-memory-flat-summary".into()),
            superseded_by: None,
            inhibition_reason: None,
        },
        TemporalGraphFact {
            edge: past_edge,
            state: TemporalFactState::Past,
            supersedes: None,
            superseded_by: None,
            inhibition_reason: Some(
                "historical completion fact is available for audit, not current strategy".into(),
            ),
        },
    ]
}

fn sample_span(id: &str, start: u64, end: u64) -> MemorySourceSpan {
    MemorySourceSpan {
        source_kind: MemorySourceKind::Transcript,
        source_id: id.into(),
        session_id: Some(hepta_core::SessionId("session-temporal-graph".into())),
        transcript_range: Some(TranscriptRange {
            start_sequence: start,
            end_sequence: end,
        }),
        transcript_entry_ids: vec![format!("session-temporal-graph:{start}")],
        transcript_span_ref: None,
        evidence_digest: format!("sha256:{id}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temporal_graph_sample_gate_is_ready() {
        let report = memory_temporal_graph_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p3_ready);
        assert!(report.checks.ready());
        assert!(!report.current_fact_ids.is_empty());
        assert!(!report.superseded_fact_ids.is_empty());
    }

    #[test]
    fn temporal_graph_excludes_expired_fact_from_current_state() {
        let report = memory_temporal_graph_sample_report(true);

        assert!(
            !report
                .current_fact_ids
                .iter()
                .any(|id| id == "fact-memory-flat-summary")
        );
        assert!(
            report
                .superseded_fact_ids
                .iter()
                .any(|id| id == "fact-memory-flat-summary")
        );
    }

    #[test]
    fn temporal_graph_preserves_source_provenance() {
        let report = memory_temporal_graph_sample_report(true);

        assert!(report.graph.iter().all(|fact| {
            fact.edge
                .source_spans
                .iter()
                .all(MemorySourceSpan::is_traceable)
        }));
    }
}

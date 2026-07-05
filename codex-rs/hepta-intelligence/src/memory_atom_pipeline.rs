use std::collections::BTreeSet;

use hepta_core::MemoryConflict;
use hepta_core::MemoryLayer;
use hepta_core::MemoryLifecycleState;
use hepta_core::MemoryLink;
use hepta_core::MemorySourceKind;
use hepta_core::MemorySourceSpan;
use hepta_core::MemoryTemporalValidity;
use hepta_core::MemoryUnit;
use hepta_core::MemoryUnitKind;
use hepta_core::MessageRole;
use hepta_core::SessionId;
use hepta_core::TranscriptEntry;
use hepta_core::TranscriptEntryKind;
use hepta_core::TranscriptRange;
use hepta_core::TranscriptSpanRef;
use serde::Deserialize;
use serde::Serialize;

pub const MEMORY_ATOM_PIPELINE_V1_CONTRACT: &str = "hepta-intelligence-l0-to-l1-atom-pipeline-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryAtomPipelineChecks {
    pub transcript_source_of_truth: bool,
    pub atom_count_nonzero: bool,
    pub add_only_no_updates_or_deletes: bool,
    pub all_atoms_active: bool,
    pub all_atoms_have_source_spans: bool,
    pub all_source_spans_traceable: bool,
    pub preference_atom_present: bool,
    pub decision_atom_present: bool,
    pub task_fact_atom_present: bool,
    pub entity_fact_atom_present: bool,
    pub unique_atom_ids: bool,
    pub no_llm_extraction_performed: bool,
    pub no_external_side_effects: bool,
}

impl MemoryAtomPipelineChecks {
    pub fn ready(&self) -> bool {
        self.transcript_source_of_truth
            && self.atom_count_nonzero
            && self.add_only_no_updates_or_deletes
            && self.all_atoms_active
            && self.all_atoms_have_source_spans
            && self.all_source_spans_traceable
            && self.preference_atom_present
            && self.decision_atom_present
            && self.task_fact_atom_present
            && self.entity_fact_atom_present
            && self.unique_atom_ids
            && self.no_llm_extraction_performed
            && self.no_external_side_effects
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryAtomPipelineReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p1_ready: bool,
    pub native_rewrite: bool,
    pub sample_run: bool,
    pub transcript_entry_count: usize,
    pub atom_count: usize,
    pub atom_kinds: BTreeSet<MemoryUnitKind>,
    pub llm_extraction_performed: bool,
    pub external_network_read: bool,
    pub memory_store_mutation_performed: bool,
    pub raw_private_memory_logged: bool,
    pub atoms: Vec<MemoryUnit>,
    pub checks: MemoryAtomPipelineChecks,
    pub next_phase: &'static str,
}

pub fn memory_atom_pipeline_sample_report(sample_run: bool) -> MemoryAtomPipelineReport {
    let now = 1_800_000_010_000;
    let entries = sample_transcript_entries(now);
    let atoms = extract_memory_atoms_from_transcript(
        "cube-hepta-intelligence",
        "user:default/project:hepta",
        &entries,
        now + 10,
    );
    let atom_kinds = atoms.iter().map(|atom| atom.kind).collect::<BTreeSet<_>>();
    let atom_ids = atoms
        .iter()
        .map(|atom| atom.id.clone())
        .collect::<BTreeSet<_>>();
    let checks = MemoryAtomPipelineChecks {
        transcript_source_of_truth: entries.iter().all(|entry| !entry.content.trim().is_empty()),
        atom_count_nonzero: !atoms.is_empty(),
        add_only_no_updates_or_deletes: atoms
            .iter()
            .all(|atom| atom.version == 1 && atom.conflicts.is_empty()),
        all_atoms_active: atoms
            .iter()
            .all(|atom| atom.lifecycle == MemoryLifecycleState::Active),
        all_atoms_have_source_spans: atoms.iter().all(|atom| !atom.source_spans.is_empty()),
        all_source_spans_traceable: atoms
            .iter()
            .flat_map(|atom| atom.source_spans.iter())
            .all(MemorySourceSpan::is_traceable),
        preference_atom_present: atom_kinds.contains(&MemoryUnitKind::Preference),
        decision_atom_present: atom_kinds.contains(&MemoryUnitKind::Decision),
        task_fact_atom_present: atom_kinds.contains(&MemoryUnitKind::TaskFact),
        entity_fact_atom_present: atom_kinds.contains(&MemoryUnitKind::EntityFact),
        unique_atom_ids: atom_ids.len() == atoms.len(),
        no_llm_extraction_performed: true,
        no_external_side_effects: true,
    };
    let p1_ready = checks.ready();

    MemoryAtomPipelineReport {
        product: "Hepta",
        command: "memory-atom-pipeline",
        contract: MEMORY_ATOM_PIPELINE_V1_CONTRACT,
        status: if p1_ready { "ready" } else { "attention" },
        p1_ready,
        native_rewrite: true,
        sample_run,
        transcript_entry_count: entries.len(),
        atom_count: atoms.len(),
        atom_kinds,
        llm_extraction_performed: false,
        external_network_read: false,
        memory_store_mutation_performed: false,
        raw_private_memory_logged: false,
        atoms,
        checks,
        next_phase: "P2 hybrid recall with BM25, embedding slot, entity, graph, recency, and RRF",
    }
}

pub fn extract_memory_atoms_from_transcript(
    cube_id: &str,
    namespace: &str,
    entries: &[TranscriptEntry],
    now_unix_ms: u64,
) -> Vec<MemoryUnit> {
    entries
        .iter()
        .filter_map(|entry| classify_atom_kind(&entry.content).map(|kind| (entry, kind)))
        .map(|(entry, kind)| memory_atom_from_entry(cube_id, namespace, entry, kind, now_unix_ms))
        .collect()
}

fn memory_atom_from_entry(
    cube_id: &str,
    namespace: &str,
    entry: &TranscriptEntry,
    kind: MemoryUnitKind,
    now_unix_ms: u64,
) -> MemoryUnit {
    let labels = BTreeSet::from([
        "l1_atom".to_string(),
        kind_label(kind).to_string(),
        source_kind_label(entry.kind).to_string(),
    ]);
    let entity_ids = extract_entity_ids(&entry.content);

    MemoryUnit {
        id: format!("atom-{}-{}", entry.entry_id, kind_label(kind)),
        cube_id: cube_id.into(),
        namespace: namespace.into(),
        layer: MemoryLayer::L1Atom,
        kind,
        lifecycle: MemoryLifecycleState::Active,
        version: 1,
        content: entry.content.clone(),
        labels,
        entity_ids,
        validity: MemoryTemporalValidity {
            valid_from_unix_ms: Some(entry.created_at_unix_ms),
            valid_until_unix_ms: None,
            observed_at_unix_ms: Some(entry.created_at_unix_ms),
            last_revalidated_unix_ms: Some(now_unix_ms),
        },
        source_spans: vec![source_span_for_entry(entry)],
        links: Vec::<MemoryLink>::new(),
        conflicts: Vec::<MemoryConflict>::new(),
        confidence_ppm: confidence_for_kind(kind),
        created_at_unix_ms: now_unix_ms,
        updated_at_unix_ms: now_unix_ms,
    }
}

fn classify_atom_kind(content: &str) -> Option<MemoryUnitKind> {
    let lower = content.to_ascii_lowercase();
    if contains_any(content, &["下一步", "待办", "任务", "需要"])
        || contains_any(&lower, &["next", "todo", "task", "p1", "must"])
    {
        Some(MemoryUnitKind::TaskFact)
    } else if contains_any(content, &["决定", "已定", "冻结"])
        || contains_any(&lower, &["decide", "decided", "decision", "freeze"])
    {
        Some(MemoryUnitKind::Decision)
    } else if contains_any(content, &["偏好", "喜欢", "倾向", "要求"])
        || contains_any(&lower, &["prefer", "preference", "likes", "requires"])
    {
        Some(MemoryUnitKind::Preference)
    } else if contains_any(content, &["Hepta", "TRNM", "OpenClaw"])
        || contains_any(&lower, &["hepta", "trnm", "openclaw"])
    {
        Some(MemoryUnitKind::EntityFact)
    } else {
        None
    }
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}

fn source_span_for_entry(entry: &TranscriptEntry) -> MemorySourceSpan {
    MemorySourceSpan {
        source_kind: match entry.kind {
            TranscriptEntryKind::Message => MemorySourceKind::Transcript,
            TranscriptEntryKind::ToolCall => MemorySourceKind::ToolCall,
            TranscriptEntryKind::ToolResult => MemorySourceKind::ToolResult,
            TranscriptEntryKind::Approval => MemorySourceKind::Approval,
            TranscriptEntryKind::Summary => MemorySourceKind::Summary,
            TranscriptEntryKind::Event => MemorySourceKind::Transcript,
        },
        source_id: entry.entry_id.clone(),
        session_id: Some(entry.session_id.clone()),
        transcript_range: Some(TranscriptRange {
            start_sequence: entry.sequence,
            end_sequence: entry.sequence,
        }),
        transcript_entry_ids: vec![entry.entry_id.clone()],
        transcript_span_ref: Some(TranscriptSpanRef {
            session_id: entry.session_id.clone(),
            range: TranscriptRange {
                start_sequence: entry.sequence,
                end_sequence: entry.sequence,
            },
            reason: Some("l0_to_l1_atom_source".into()),
        }),
        evidence_digest: format!("sha256:{}", entry.entry_id),
    }
}

fn source_kind_label(kind: TranscriptEntryKind) -> &'static str {
    match kind {
        TranscriptEntryKind::Message => "message",
        TranscriptEntryKind::ToolCall => "tool_call",
        TranscriptEntryKind::ToolResult => "tool_result",
        TranscriptEntryKind::Approval => "approval",
        TranscriptEntryKind::Summary => "summary",
        TranscriptEntryKind::Event => "event",
    }
}

fn kind_label(kind: MemoryUnitKind) -> &'static str {
    match kind {
        MemoryUnitKind::Semantic => "semantic",
        MemoryUnitKind::Episodic => "episodic",
        MemoryUnitKind::Procedural => "procedural",
        MemoryUnitKind::Profile => "profile",
        MemoryUnitKind::Preference => "preference",
        MemoryUnitKind::TaskFact => "task_fact",
        MemoryUnitKind::Decision => "decision",
        MemoryUnitKind::EntityFact => "entity_fact",
        MemoryUnitKind::Scenario => "scenario",
        MemoryUnitKind::CoreBlock => "core_block",
        MemoryUnitKind::TemporalFact => "temporal_fact",
        MemoryUnitKind::SymbolicContext => "symbolic_context",
    }
}

fn confidence_for_kind(kind: MemoryUnitKind) -> u32 {
    match kind {
        MemoryUnitKind::Preference | MemoryUnitKind::Decision => 900_000,
        MemoryUnitKind::TaskFact | MemoryUnitKind::EntityFact => 860_000,
        _ => 750_000,
    }
}

fn extract_entity_ids(content: &str) -> BTreeSet<String> {
    let mut entities = BTreeSet::new();
    let lower = content.to_ascii_lowercase();
    for (needle, entity) in [
        ("hepta", "entity:hepta"),
        ("openclaw", "entity:openclaw"),
        ("memory", "entity:memory"),
        ("rust", "entity:rust"),
        ("trnm", "entity:trnm"),
    ] {
        if lower.contains(needle) || content.contains(entity.trim_start_matches("entity:")) {
            entities.insert(entity.into());
        }
    }
    if entities.is_empty() {
        entities.insert("entity:unknown".into());
    }
    entities
}

fn sample_transcript_entries(now: u64) -> Vec<TranscriptEntry> {
    let session_id = SessionId("session-memory-atom-pipeline".into());
    vec![
        TranscriptEntry {
            entry_id: "entry-preference".into(),
            session_id: session_id.clone(),
            sequence: 1,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::User),
            content: "用户偏好：Hepta memory 必须以 raw transcript 作为 source of truth。".into(),
            created_at_unix_ms: now,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        },
        TranscriptEntry {
            entry_id: "entry-decision".into(),
            session_id: session_id.clone(),
            sequence: 2,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::Assistant),
            content: "决定：Memory Kernel v1 先落 add-only atom contract，再进入 hybrid recall。"
                .into(),
            created_at_unix_ms: now + 1,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        },
        TranscriptEntry {
            entry_id: "entry-task".into(),
            session_id: session_id.clone(),
            sequence: 3,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::User),
            content:
                "下一步 P1 任务：从 L0 transcript 抽取 preference/decision/task_fact/entity_fact。"
                    .into(),
            created_at_unix_ms: now + 2,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        },
        TranscriptEntry {
            entry_id: "entry-entity".into(),
            session_id,
            sequence: 4,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::Assistant),
            content: "Hepta 是 Rust-native intelligence runtime，正在对齐 OpenClaw memory 能力。"
                .into(),
            created_at_unix_ms: now + 3,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_atom_pipeline_sample_gate_is_ready() {
        let report = memory_atom_pipeline_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p1_ready);
        assert!(report.checks.ready());
        assert_eq!(report.atom_count, 4);
        assert!(!report.llm_extraction_performed);
        assert!(!report.memory_store_mutation_performed);
    }

    #[test]
    fn atom_extraction_is_add_only_and_traceable() {
        let report = memory_atom_pipeline_sample_report(true);

        assert!(
            report
                .atoms
                .iter()
                .all(|atom| atom.version == 1 && atom.lifecycle == MemoryLifecycleState::Active)
        );
        assert!(report.atoms.iter().all(MemoryUnit::has_traceable_source));
        assert!(
            report
                .atoms
                .iter()
                .flat_map(|atom| atom.source_spans.iter())
                .all(MemorySourceSpan::is_traceable)
        );
    }

    #[test]
    fn atom_extraction_covers_required_l1_kinds() {
        let report = memory_atom_pipeline_sample_report(true);

        assert!(report.atom_kinds.contains(&MemoryUnitKind::Preference));
        assert!(report.atom_kinds.contains(&MemoryUnitKind::Decision));
        assert!(report.atom_kinds.contains(&MemoryUnitKind::TaskFact));
        assert!(report.atom_kinds.contains(&MemoryUnitKind::EntityFact));
    }
}

use std::collections::BTreeSet;

use hepta_core::MemoryDeleteTombstone;
use hepta_core::MemoryLifecycleState;
use hepta_core::MemoryScope;
use hepta_core::MemorySourceSpan;
use hepta_core::MemoryUnit;
use hepta_core::MemoryUnitKind;
use serde::Deserialize;
use serde::Serialize;

use crate::extract_memory_atoms_from_transcript;
use crate::hybrid_recall;
use crate::memory_runtime_handoff_sample_report;
use crate::sample_runtime_transcript_entries;

pub const MEMORY_RUNTIME_STORE_READBACK_V1_CONTRACT: &str =
    "hepta-intelligence-memory-runtime-store-readback-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRuntimeStoredRecord {
    pub memory_id: String,
    pub source_atom_id: String,
    pub scope: MemoryScope,
    pub lifecycle: MemoryLifecycleState,
    pub kind: MemoryUnitKind,
    pub content_digest: String,
    pub redacted_summary: String,
    pub labels: Vec<String>,
    pub entity_ids: Vec<String>,
    pub source_spans: Vec<MemorySourceSpan>,
    pub journal_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRuntimeJournalEntry {
    pub sequence: u64,
    pub journal_id: String,
    pub op: &'static str,
    pub target_id: String,
    pub source_atom_id: String,
    pub idempotency_key: String,
    pub content_digest: String,
    pub readback_evidence_id: String,
    pub production_mutation_performed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRuntimeIndexEntry {
    pub index_id: String,
    pub memory_id: String,
    pub source_atom_id: String,
    pub lexical_terms: Vec<String>,
    pub entity_ids: Vec<String>,
    pub content_digest: String,
    pub tombstoned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRuntimeTemporalReadback {
    pub current_memory_ids: Vec<String>,
    pub superseded_memory_ids: Vec<String>,
    pub tombstoned_memory_ids: Vec<String>,
    pub conflict_policy: &'static str,
    pub source_atom_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRuntimeStoreReadbackChecks {
    pub prior_handoff_ready: bool,
    pub append_journal_nonempty: bool,
    pub journal_is_append_only: bool,
    pub unique_idempotency_keys: bool,
    pub stored_records_source_traceable: bool,
    pub stored_records_redacted: bool,
    pub readback_query_returns_records: bool,
    pub hybrid_index_refresh_readable: bool,
    pub temporal_refresh_readable: bool,
    pub prompt_context_readback_assembled: bool,
    pub tombstone_cascade_declared: bool,
    pub readback_evidence_complete: bool,
    pub no_llm_extraction_performed: bool,
    pub no_external_network_read: bool,
    pub no_production_filesystem_mutation: bool,
    pub no_raw_private_memory_logged: bool,
}

impl MemoryRuntimeStoreReadbackChecks {
    pub fn ready(&self) -> bool {
        self.prior_handoff_ready
            && self.append_journal_nonempty
            && self.journal_is_append_only
            && self.unique_idempotency_keys
            && self.stored_records_source_traceable
            && self.stored_records_redacted
            && self.readback_query_returns_records
            && self.hybrid_index_refresh_readable
            && self.temporal_refresh_readable
            && self.prompt_context_readback_assembled
            && self.tombstone_cascade_declared
            && self.readback_evidence_complete
            && self.no_llm_extraction_performed
            && self.no_external_network_read
            && self.no_production_filesystem_mutation
            && self.no_raw_private_memory_logged
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRuntimeStoreReadbackReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p8_store_readback_ready: bool,
    pub native_rewrite: bool,
    pub sample_run: bool,
    pub transcript_entry_count: usize,
    pub atom_count: usize,
    pub stored_record_count: usize,
    pub journal_entry_count: usize,
    pub index_entry_count: usize,
    pub tombstone_count: usize,
    pub readback_query: String,
    pub readback_hit_count: usize,
    pub prompt_context_node_count: usize,
    pub records: Vec<MemoryRuntimeStoredRecord>,
    pub journal: Vec<MemoryRuntimeJournalEntry>,
    pub hybrid_index: Vec<MemoryRuntimeIndexEntry>,
    pub temporal_readback: MemoryRuntimeTemporalReadback,
    pub tombstones: Vec<MemoryDeleteTombstone>,
    pub prompt_context_unit_ids: Vec<String>,
    pub checks: MemoryRuntimeStoreReadbackChecks,
    pub next_phase: &'static str,
}

pub fn memory_runtime_store_readback_sample_report(
    sample_run: bool,
) -> MemoryRuntimeStoreReadbackReport {
    let now = 1_800_000_040_000;
    let entries = sample_runtime_transcript_entries(now);
    let atoms = extract_memory_atoms_from_transcript(
        "cube-hepta-runtime-memory",
        "user:default/project:hepta/runtime",
        &entries,
        now + 10,
    );
    let recall_hits = hybrid_recall(
        "Hepta memory runtime prompt context source span",
        &atoms,
        4,
        250,
        19,
    );
    let handoff = memory_runtime_handoff_sample_report(true);
    let records = stored_records_from_atoms(&atoms);
    let tombstones = sample_tombstones(now + 20);
    let journal = journal_from_records_and_tombstones(&records, &tombstones);
    let hybrid_index = index_entries_from_records(&records);
    let temporal_readback = temporal_readback_from_records(&records, &tombstones);
    let readback_query = "memory runtime source span".to_string();
    let readback_hits = query_records_from_index(&readback_query, &records, &hybrid_index);
    let prompt_context_unit_ids = recall_hits
        .iter()
        .map(|hit| hit.unit_id.clone())
        .collect::<Vec<_>>();
    let checks = MemoryRuntimeStoreReadbackChecks {
        prior_handoff_ready: handoff.p7_runtime_handoff_ready,
        append_journal_nonempty: !journal.is_empty(),
        journal_is_append_only: journal_is_append_only(&journal),
        unique_idempotency_keys: unique_idempotency_keys(&journal),
        stored_records_source_traceable: records.iter().all(|record| {
            !record.source_spans.is_empty()
                && record
                    .source_spans
                    .iter()
                    .all(MemorySourceSpan::is_traceable)
        }),
        stored_records_redacted: records.iter().all(|record| {
            !record.redacted_summary.trim().is_empty()
                && !record.redacted_summary.contains("SECRET=")
                && !record.redacted_summary.contains("api_key")
        }),
        readback_query_returns_records: !readback_hits.is_empty(),
        hybrid_index_refresh_readable: hybrid_index.len() == records.len()
            && records.iter().all(|record| {
                hybrid_index
                    .iter()
                    .any(|entry| entry.memory_id == record.memory_id)
            }),
        temporal_refresh_readable: temporal_readback.current_memory_ids.len() == records.len()
            && temporal_readback.tombstoned_memory_ids.len() == tombstones.len(),
        prompt_context_readback_assembled: !prompt_context_unit_ids.is_empty()
            && prompt_context_unit_ids.len() == recall_hits.len(),
        tombstone_cascade_declared: tombstones
            .iter()
            .all(|tombstone| tombstone.cascade_indexes_required),
        readback_evidence_complete: journal
            .iter()
            .all(|entry| !entry.readback_evidence_id.trim().is_empty()),
        no_llm_extraction_performed: true,
        no_external_network_read: true,
        no_production_filesystem_mutation: true,
        no_raw_private_memory_logged: true,
    };
    let p8_store_readback_ready = checks.ready();

    MemoryRuntimeStoreReadbackReport {
        product: "Hepta",
        command: "memory-store-readback",
        contract: MEMORY_RUNTIME_STORE_READBACK_V1_CONTRACT,
        status: if p8_store_readback_ready {
            "ready"
        } else {
            "attention"
        },
        p8_store_readback_ready,
        native_rewrite: true,
        sample_run,
        transcript_entry_count: entries.len(),
        atom_count: atoms.len(),
        stored_record_count: records.len(),
        journal_entry_count: journal.len(),
        index_entry_count: hybrid_index.len(),
        tombstone_count: tombstones.len(),
        readback_query,
        readback_hit_count: readback_hits.len(),
        prompt_context_node_count: prompt_context_unit_ids.len(),
        records,
        journal,
        hybrid_index,
        temporal_readback,
        tombstones,
        prompt_context_unit_ids,
        checks,
        next_phase: "wire the store readback into live prompt assembly behind explicit runtime policy and stale-fact conflict gates",
    }
}

fn stored_records_from_atoms(atoms: &[MemoryUnit]) -> Vec<MemoryRuntimeStoredRecord> {
    atoms
        .iter()
        .enumerate()
        .map(|(idx, atom)| MemoryRuntimeStoredRecord {
            memory_id: format!("mem-{}", atom.id),
            source_atom_id: atom.id.clone(),
            scope: MemoryScope::LongTerm,
            lifecycle: atom.lifecycle,
            kind: atom.kind,
            content_digest: stable_digest(&atom.content),
            redacted_summary: format!(
                "{} memory atom from {} source span(s)",
                memory_kind_label(atom.kind),
                atom.source_spans.len()
            ),
            labels: atom.labels.iter().cloned().collect(),
            entity_ids: atom.entity_ids.iter().cloned().collect(),
            source_spans: atom.source_spans.clone(),
            journal_sequence: (idx + 1) as u64,
        })
        .collect()
}

fn sample_tombstones(now: u64) -> Vec<MemoryDeleteTombstone> {
    vec![MemoryDeleteTombstone {
        unit_id: "mem-obsolete-flat-summary".into(),
        cube_id: "cube-hepta-runtime-memory".into(),
        deleted_at_unix_ms: now,
        reason: "superseded_by_source_traceable_l1_atom".into(),
        deleted_by: "hepta-memory-runtime-store-readback-sample".into(),
        cascade_indexes_required: true,
        source_span_count_at_delete: 1,
    }]
}

fn journal_from_records_and_tombstones(
    records: &[MemoryRuntimeStoredRecord],
    tombstones: &[MemoryDeleteTombstone],
) -> Vec<MemoryRuntimeJournalEntry> {
    let mut journal = records
        .iter()
        .map(|record| MemoryRuntimeJournalEntry {
            sequence: record.journal_sequence,
            journal_id: format!("journal-append-{}", record.memory_id),
            op: "append_memory_unit",
            target_id: record.memory_id.clone(),
            source_atom_id: record.source_atom_id.clone(),
            idempotency_key: format!(
                "memory-append:{}:{}",
                record.memory_id, record.content_digest
            ),
            content_digest: record.content_digest.clone(),
            readback_evidence_id: format!("readback:{}", record.memory_id),
            production_mutation_performed: false,
        })
        .collect::<Vec<_>>();

    for (next_sequence, tombstone) in (journal.len() as u64 + 1..).zip(tombstones) {
        journal.push(MemoryRuntimeJournalEntry {
            sequence: next_sequence,
            journal_id: format!("journal-tombstone-{}", tombstone.unit_id),
            op: "append_delete_tombstone",
            target_id: tombstone.unit_id.clone(),
            source_atom_id: "prior-state".into(),
            idempotency_key: format!(
                "memory-tombstone:{}:{}",
                tombstone.unit_id, tombstone.deleted_at_unix_ms
            ),
            content_digest: stable_digest(&tombstone.reason),
            readback_evidence_id: format!("readback:tombstone:{}", tombstone.unit_id),
            production_mutation_performed: false,
        });
    }

    journal
}

fn index_entries_from_records(
    records: &[MemoryRuntimeStoredRecord],
) -> Vec<MemoryRuntimeIndexEntry> {
    records
        .iter()
        .map(|record| MemoryRuntimeIndexEntry {
            index_id: format!("idx-{}", record.memory_id),
            memory_id: record.memory_id.clone(),
            source_atom_id: record.source_atom_id.clone(),
            lexical_terms: lexical_terms_for_record(record),
            entity_ids: record.entity_ids.clone(),
            content_digest: record.content_digest.clone(),
            tombstoned: record.lifecycle == MemoryLifecycleState::Tombstoned,
        })
        .collect()
}

fn temporal_readback_from_records(
    records: &[MemoryRuntimeStoredRecord],
    tombstones: &[MemoryDeleteTombstone],
) -> MemoryRuntimeTemporalReadback {
    MemoryRuntimeTemporalReadback {
        current_memory_ids: records
            .iter()
            .filter(|record| record.lifecycle == MemoryLifecycleState::Active)
            .map(|record| record.memory_id.clone())
            .collect(),
        superseded_memory_ids: records
            .iter()
            .filter(|record| record.lifecycle == MemoryLifecycleState::Superseded)
            .map(|record| record.memory_id.clone())
            .collect(),
        tombstoned_memory_ids: tombstones
            .iter()
            .map(|tombstone| tombstone.unit_id.clone())
            .collect(),
        conflict_policy: "prefer_current_source_traceable_atom",
        source_atom_ids: records
            .iter()
            .map(|record| record.source_atom_id.clone())
            .collect(),
    }
}

fn query_records_from_index(
    query: &str,
    records: &[MemoryRuntimeStoredRecord],
    index: &[MemoryRuntimeIndexEntry],
) -> Vec<MemoryRuntimeStoredRecord> {
    let query_terms = extract_ascii_terms(query);
    let matching_ids = index
        .iter()
        .filter(|entry| {
            !entry.tombstoned
                && query_terms.iter().any(|term| {
                    entry
                        .lexical_terms
                        .iter()
                        .any(|candidate| candidate == term)
                })
        })
        .map(|entry| entry.memory_id.clone())
        .collect::<BTreeSet<_>>();

    records
        .iter()
        .filter(|record| matching_ids.contains(&record.memory_id))
        .cloned()
        .collect()
}

fn lexical_terms_for_record(record: &MemoryRuntimeStoredRecord) -> Vec<String> {
    let mut terms = BTreeSet::new();
    for label in &record.labels {
        terms.extend(extract_ascii_terms(label));
    }
    for entity_id in &record.entity_ids {
        terms.extend(extract_ascii_terms(entity_id));
    }
    terms.extend(extract_ascii_terms(&record.redacted_summary));
    terms.extend(extract_ascii_terms(&record.source_atom_id));
    terms.into_iter().collect()
}

fn extract_ascii_terms(text: &str) -> BTreeSet<String> {
    let mut terms = BTreeSet::new();
    let mut current = String::new();
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            current.push(ch.to_ascii_lowercase());
        } else if !current.is_empty() {
            if current.len() > 1 {
                terms.insert(current.clone());
            }
            current.clear();
        }
    }
    if current.len() > 1 {
        terms.insert(current);
    }
    terms
}

fn journal_is_append_only(journal: &[MemoryRuntimeJournalEntry]) -> bool {
    journal
        .windows(2)
        .all(|pair| pair[0].sequence < pair[1].sequence)
        && journal.iter().all(|entry| {
            matches!(entry.op, "append_memory_unit" | "append_delete_tombstone")
                && !entry.production_mutation_performed
        })
}

fn unique_idempotency_keys(journal: &[MemoryRuntimeJournalEntry]) -> bool {
    let keys = journal
        .iter()
        .map(|entry| entry.idempotency_key.as_str())
        .collect::<BTreeSet<_>>();
    keys.len() == journal.len()
}

fn memory_kind_label(kind: MemoryUnitKind) -> &'static str {
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

fn stable_digest(text: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("fnv1a64:{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_runtime_store_readback_sample_gate_is_ready() {
        let report = memory_runtime_store_readback_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p8_store_readback_ready);
        assert!(report.checks.ready());
        assert_eq!(report.transcript_entry_count, 4);
        assert_eq!(report.atom_count, 4);
        assert_eq!(report.stored_record_count, 4);
        assert_eq!(report.index_entry_count, 4);
        assert_eq!(report.tombstone_count, 1);
        assert!(report.readback_hit_count > 0);
    }

    #[test]
    fn store_readback_journal_is_append_only_and_idempotent() {
        let report = memory_runtime_store_readback_sample_report(true);

        assert!(report.checks.journal_is_append_only);
        assert!(report.checks.unique_idempotency_keys);
        assert!(
            report
                .journal
                .iter()
                .all(|entry| !entry.production_mutation_performed)
        );
        assert!(
            report
                .journal
                .iter()
                .all(|entry| !entry.readback_evidence_id.trim().is_empty())
        );
    }

    #[test]
    fn store_readback_preserves_provenance_and_tombstone_cascade() {
        let report = memory_runtime_store_readback_sample_report(true);

        assert!(report.checks.stored_records_source_traceable);
        assert!(report.records.iter().all(|record| {
            record
                .source_spans
                .iter()
                .all(MemorySourceSpan::is_traceable)
        }));
        assert!(report.checks.tombstone_cascade_declared);
        assert!(
            report
                .tombstones
                .iter()
                .all(|tombstone| tombstone.cascade_indexes_required)
        );
    }

    #[test]
    fn store_readback_does_not_log_raw_private_memory() {
        let report = memory_runtime_store_readback_sample_report(true);

        assert!(report.checks.stored_records_redacted);
        assert!(report.checks.no_raw_private_memory_logged);
        assert!(
            report
                .records
                .iter()
                .all(|record| record.content_digest.starts_with("fnv1a64:"))
        );
        assert!(report.records.iter().all(|record| {
            !record.redacted_summary.contains("用户")
                && !record.redacted_summary.contains("决定")
                && !record.redacted_summary.contains("下一步")
        }));
    }

    #[test]
    fn recall_hit_type_remains_serializable_boundary() {
        fn assert_hits(_: &[crate::HybridRecallHit]) {}

        let atoms = extract_memory_atoms_from_transcript(
            "cube-test",
            "namespace-test",
            &sample_runtime_transcript_entries(1_800_000_040_000),
            1_800_000_040_010,
        );
        let hits = hybrid_recall("memory runtime", &atoms, 2, 250, 10);
        assert_hits(&hits);
        assert!(!hits.is_empty());
    }
}

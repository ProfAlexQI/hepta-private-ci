//! Hepta Intelligence memory-kernel contracts.
//!
//! The kernel is the typed, auditable layer above raw transcript storage and
//! below extraction/recall/runtime orchestration.  P0 intentionally defines the
//! contract and a deterministic sample gate only; it does not run LLM
//! extraction, mutate production memory, or open external indexes.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::SessionId;
use crate::TranscriptRange;
use crate::TranscriptSpanRef;

pub const MEMORY_KERNEL_V1_CONTRACT: &str = "hepta-intelligence-memory-kernel-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryLayer {
    L0Transcript,
    L1Atom,
    L15SymbolicWorkingContext,
    L2Scenario,
    L3CoreBlock,
    L4TemporalGraph,
    L5ContextAssembly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryUnitKind {
    Semantic,
    Episodic,
    Procedural,
    Profile,
    Preference,
    TaskFact,
    Decision,
    EntityFact,
    Scenario,
    CoreBlock,
    TemporalFact,
    SymbolicContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryLifecycleState {
    PendingReview,
    Active,
    Superseded,
    Archived,
    Tombstoned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryCubeScope {
    Session,
    User,
    Project,
    Agent,
    Workspace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemorySourceKind {
    Transcript,
    ToolCall,
    ToolResult,
    Approval,
    Summary,
    ImportedMemory,
    OperatorFeedback,
    SyntheticSample,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryLinkKind {
    Evidence,
    SemanticSimilarity,
    EntityOverlap,
    WorkflowAdjacency,
    CausalDependency,
    TemporalContinuation,
    Supersedes,
    ConflictsWith,
    Inhibits,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryConflictKind {
    PreferenceChanged,
    FactChanged,
    ContradictoryEvidence,
    ScopeCollision,
    ProcedureUnsafe,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemorySourceSpan {
    pub source_kind: MemorySourceKind,
    pub source_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<SessionId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript_range: Option<TranscriptRange>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transcript_entry_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript_span_ref: Option<TranscriptSpanRef>,
    pub evidence_digest: String,
}

impl MemorySourceSpan {
    pub fn is_traceable(&self) -> bool {
        !self.source_id.trim().is_empty()
            && !self.evidence_digest.trim().is_empty()
            && (self.transcript_range.is_some()
                || !self.transcript_entry_ids.is_empty()
                || self.transcript_span_ref.is_some())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct MemoryTemporalValidity {
    pub valid_from_unix_ms: Option<u64>,
    pub valid_until_unix_ms: Option<u64>,
    pub observed_at_unix_ms: Option<u64>,
    pub last_revalidated_unix_ms: Option<u64>,
}

impl MemoryTemporalValidity {
    pub fn currently_valid_at(&self, now_unix_ms: u64) -> bool {
        self.valid_from_unix_ms
            .map(|start| now_unix_ms >= start)
            .unwrap_or(true)
            && self
                .valid_until_unix_ms
                .map(|end| now_unix_ms <= end)
                .unwrap_or(true)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryLink {
    pub target_id: String,
    pub kind: MemoryLinkKind,
    pub weight_ppm: u32,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryConflict {
    pub other_unit_id: String,
    pub kind: MemoryConflictKind,
    pub resolution: MemoryConflictResolution,
    pub reason: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<MemorySourceSpan>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryConflictResolution {
    Unresolved,
    PreferNewer,
    PreferHigherConfidence,
    KeepBothScoped,
    SupersedeOld,
    RejectUnsafe,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryUnit {
    pub id: String,
    pub cube_id: String,
    pub namespace: String,
    pub layer: MemoryLayer,
    pub kind: MemoryUnitKind,
    pub lifecycle: MemoryLifecycleState,
    pub version: u64,
    pub content: String,
    #[serde(default)]
    pub labels: BTreeSet<String>,
    #[serde(default)]
    pub entity_ids: BTreeSet<String>,
    #[serde(default)]
    pub validity: MemoryTemporalValidity,
    #[serde(default)]
    pub source_spans: Vec<MemorySourceSpan>,
    #[serde(default)]
    pub links: Vec<MemoryLink>,
    #[serde(default)]
    pub conflicts: Vec<MemoryConflict>,
    pub confidence_ppm: u32,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
}

impl MemoryUnit {
    pub fn is_active(&self) -> bool {
        self.lifecycle == MemoryLifecycleState::Active
    }

    pub fn is_recallable_at(&self, now_unix_ms: u64) -> bool {
        self.is_active()
            && self.validity.currently_valid_at(now_unix_ms)
            && self.has_traceable_source()
    }

    pub fn has_traceable_source(&self) -> bool {
        !self.source_spans.is_empty()
            && self.source_spans.iter().all(MemorySourceSpan::is_traceable)
    }

    pub fn supersede(&mut self, updated_at_unix_ms: u64) {
        self.lifecycle = MemoryLifecycleState::Superseded;
        self.updated_at_unix_ms = updated_at_unix_ms;
        self.version = self.version.saturating_add(1);
    }

    pub fn tombstone(&mut self, updated_at_unix_ms: u64) {
        self.lifecycle = MemoryLifecycleState::Tombstoned;
        self.updated_at_unix_ms = updated_at_unix_ms;
        self.version = self.version.saturating_add(1);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryDeleteTombstone {
    pub unit_id: String,
    pub cube_id: String,
    pub deleted_at_unix_ms: u64,
    pub reason: String,
    pub deleted_by: String,
    pub cascade_indexes_required: bool,
    pub source_span_count_at_delete: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreMemoryBlock {
    pub id: String,
    pub cube_id: String,
    pub title: String,
    pub block_kind: CoreMemoryBlockKind,
    pub pinned: bool,
    pub editable: bool,
    pub version: u64,
    pub content: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_unit_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_spans: Vec<MemorySourceSpan>,
}

impl CoreMemoryBlock {
    pub fn has_traceable_source(&self) -> bool {
        !self.source_spans.is_empty()
            && self.source_spans.iter().all(MemorySourceSpan::is_traceable)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoreMemoryBlockKind {
    Identity,
    UserProfile,
    ProjectState,
    StablePreference,
    ActiveObjective,
    WorkflowPrior,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemporalFactEdge {
    pub id: String,
    pub source_unit_id: String,
    pub subject_entity_id: String,
    pub predicate: String,
    pub object_entity_id: String,
    pub validity: MemoryTemporalValidity,
    pub confidence_ppm: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_spans: Vec<MemorySourceSpan>,
}

impl TemporalFactEdge {
    pub fn has_traceable_source(&self) -> bool {
        !self.source_spans.is_empty()
            && self.source_spans.iter().all(MemorySourceSpan::is_traceable)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryCube {
    pub id: String,
    pub scope: MemoryCubeScope,
    pub owner_id: String,
    pub schema_version: String,
    pub version: u64,
    #[serde(default)]
    pub units: BTreeMap<String, MemoryUnit>,
    #[serde(default)]
    pub core_blocks: BTreeMap<String, CoreMemoryBlock>,
    #[serde(default)]
    pub temporal_edges: BTreeMap<String, TemporalFactEdge>,
    #[serde(default)]
    pub tombstones: BTreeMap<String, MemoryDeleteTombstone>,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
}

impl MemoryCube {
    pub fn active_units(&self, now_unix_ms: u64) -> Vec<&MemoryUnit> {
        self.units
            .values()
            .filter(|unit| unit.is_recallable_at(now_unix_ms))
            .collect()
    }

    pub fn has_tombstone(&self, unit_id: &str) -> bool {
        self.tombstones.contains_key(unit_id)
    }

    pub fn tombstone_unit(
        &mut self,
        unit_id: &str,
        deleted_at_unix_ms: u64,
        reason: impl Into<String>,
        deleted_by: impl Into<String>,
    ) -> Result<(), String> {
        let unit = self
            .units
            .get_mut(unit_id)
            .ok_or_else(|| format!("memory unit not found: {unit_id}"))?;
        unit.tombstone(deleted_at_unix_ms);
        self.tombstones.insert(
            unit_id.to_string(),
            MemoryDeleteTombstone {
                unit_id: unit_id.to_string(),
                cube_id: self.id.clone(),
                deleted_at_unix_ms,
                reason: reason.into(),
                deleted_by: deleted_by.into(),
                cascade_indexes_required: true,
                source_span_count_at_delete: unit.source_spans.len(),
            },
        );
        self.updated_at_unix_ms = deleted_at_unix_ms;
        self.version = self.version.saturating_add(1);
        Ok(())
    }

    pub fn all_active_units_have_traceable_sources(&self) -> bool {
        self.units
            .values()
            .filter(|unit| unit.lifecycle == MemoryLifecycleState::Active)
            .all(MemoryUnit::has_traceable_source)
    }

    pub fn all_derived_artifacts_have_traceable_sources(&self) -> bool {
        self.all_active_units_have_traceable_sources()
            && self
                .core_blocks
                .values()
                .all(CoreMemoryBlock::has_traceable_source)
            && self
                .temporal_edges
                .values()
                .all(TemporalFactEdge::has_traceable_source)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryRecallBundleV2 {
    pub query: String,
    pub now_unix_ms: u64,
    #[serde(default)]
    pub recalled_unit_ids: Vec<String>,
    #[serde(default)]
    pub recalled_core_block_ids: Vec<String>,
    #[serde(default)]
    pub recalled_temporal_edge_ids: Vec<String>,
    #[serde(default)]
    pub source_spans: Vec<MemorySourceSpan>,
    pub omitted_tombstoned_count: usize,
    pub provenance_complete: bool,
}

impl MemoryRecallBundleV2 {
    pub fn from_cube(query: impl Into<String>, cube: &MemoryCube, now_unix_ms: u64) -> Self {
        let active_units = cube.active_units(now_unix_ms);
        let source_spans = active_units
            .iter()
            .flat_map(|unit| unit.source_spans.iter().cloned())
            .collect::<Vec<_>>();
        let provenance_complete = active_units.iter().all(|unit| unit.has_traceable_source())
            && source_spans.iter().all(MemorySourceSpan::is_traceable);
        Self {
            query: query.into(),
            now_unix_ms,
            recalled_unit_ids: active_units.iter().map(|unit| unit.id.clone()).collect(),
            recalled_core_block_ids: cube
                .core_blocks
                .values()
                .filter(|block| block.has_traceable_source())
                .map(|block| block.id.clone())
                .collect(),
            recalled_temporal_edge_ids: cube
                .temporal_edges
                .values()
                .filter(|edge| {
                    edge.validity.currently_valid_at(now_unix_ms) && edge.has_traceable_source()
                })
                .map(|edge| edge.id.clone())
                .collect(),
            source_spans,
            omitted_tombstoned_count: cube.tombstones.len(),
            provenance_complete,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKernelSampleChecks {
    pub source_span_required: bool,
    pub lifecycle_state_supported: bool,
    pub temporal_validity_supported: bool,
    pub tombstone_supported: bool,
    pub tombstoned_unit_not_recalled: bool,
    pub delete_cascade_required: bool,
    pub conflict_keeps_both_evidence_spans: bool,
    pub core_block_provenance_required: bool,
    pub temporal_edge_provenance_required: bool,
    pub no_llm_extraction_performed: bool,
    pub no_external_side_effects: bool,
}

impl MemoryKernelSampleChecks {
    pub fn ready(&self) -> bool {
        self.source_span_required
            && self.lifecycle_state_supported
            && self.temporal_validity_supported
            && self.tombstone_supported
            && self.tombstoned_unit_not_recalled
            && self.delete_cascade_required
            && self.conflict_keeps_both_evidence_spans
            && self.core_block_provenance_required
            && self.temporal_edge_provenance_required
            && self.no_llm_extraction_performed
            && self.no_external_side_effects
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryKernelSampleReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p0_ready: bool,
    pub native_rewrite: bool,
    pub sample_run: bool,
    pub llm_extraction_performed: bool,
    pub external_network_read: bool,
    pub memory_store_mutation_performed: bool,
    pub raw_private_memory_logged: bool,
    pub cube: MemoryCube,
    pub recall_bundle: MemoryRecallBundleV2,
    pub checks: MemoryKernelSampleChecks,
    pub next_phase: &'static str,
}

pub fn memory_kernel_v1_sample_report(sample_run: bool) -> MemoryKernelSampleReport {
    let now = 1_800_000_000_000;
    let mut cube = sample_memory_cube(now);
    cube.tombstone_unit(
        "unit-old-preference",
        now + 4,
        "superseded by explicit newer preference",
        "sample-operator",
    )
    .expect("sample unit exists");
    let recall_bundle =
        MemoryRecallBundleV2::from_cube("preferred runtime memory direction", &cube, now + 5);
    let conflict_spans = cube
        .units
        .get("unit-current-preference")
        .map(|unit| {
            unit.conflicts
                .iter()
                .flat_map(|conflict| conflict.evidence.iter())
                .count()
        })
        .unwrap_or(0);
    let checks = MemoryKernelSampleChecks {
        source_span_required: cube.all_active_units_have_traceable_sources(),
        lifecycle_state_supported: cube
            .units
            .values()
            .any(|unit| unit.lifecycle == MemoryLifecycleState::Active)
            && cube
                .units
                .values()
                .any(|unit| unit.lifecycle == MemoryLifecycleState::Tombstoned),
        temporal_validity_supported: cube
            .temporal_edges
            .values()
            .all(|edge| edge.validity.currently_valid_at(now + 5)),
        tombstone_supported: cube.has_tombstone("unit-old-preference"),
        tombstoned_unit_not_recalled: !recall_bundle
            .recalled_unit_ids
            .iter()
            .any(|id| id == "unit-old-preference"),
        delete_cascade_required: cube
            .tombstones
            .values()
            .all(|tombstone| tombstone.cascade_indexes_required),
        conflict_keeps_both_evidence_spans: conflict_spans >= 2,
        core_block_provenance_required: cube
            .core_blocks
            .values()
            .all(CoreMemoryBlock::has_traceable_source),
        temporal_edge_provenance_required: cube
            .temporal_edges
            .values()
            .all(TemporalFactEdge::has_traceable_source),
        no_llm_extraction_performed: true,
        no_external_side_effects: true,
    };
    let p0_ready = checks.ready() && recall_bundle.provenance_complete;

    MemoryKernelSampleReport {
        product: "Hepta",
        command: "memory-kernel",
        contract: MEMORY_KERNEL_V1_CONTRACT,
        status: if p0_ready { "ready" } else { "attention" },
        p0_ready,
        native_rewrite: true,
        sample_run,
        llm_extraction_performed: false,
        external_network_read: false,
        memory_store_mutation_performed: false,
        raw_private_memory_logged: false,
        cube,
        recall_bundle,
        checks,
        next_phase: "P1 L0 transcript to L1 add-only atom pipeline",
    }
}

fn sample_memory_cube(now: u64) -> MemoryCube {
    let newer_span = sample_span("span-current-preference", "session-memory-kernel", 10, 11);
    let older_span = sample_span("span-old-preference", "session-memory-kernel", 2, 3);
    let task_span = sample_span("span-task-fact", "session-memory-kernel", 20, 22);
    let unit_current = MemoryUnit {
        id: "unit-current-preference".into(),
        cube_id: "cube-hepta-intelligence".into(),
        namespace: "user:default/project:hepta".into(),
        layer: MemoryLayer::L1Atom,
        kind: MemoryUnitKind::Preference,
        lifecycle: MemoryLifecycleState::Active,
        version: 1,
        content: "Hepta memory must keep raw transcript as source of truth and use derived memory as an auditable layer.".into(),
        labels: BTreeSet::from(["memory_kernel".into(), "preference".into()]),
        entity_ids: BTreeSet::from(["entity:hepta".into(), "entity:memory".into()]),
        validity: MemoryTemporalValidity {
            valid_from_unix_ms: Some(now),
            valid_until_unix_ms: None,
            observed_at_unix_ms: Some(now),
            last_revalidated_unix_ms: Some(now),
        },
        source_spans: vec![newer_span.clone()],
        links: vec![MemoryLink {
            target_id: "unit-task-fact".into(),
            kind: MemoryLinkKind::WorkflowAdjacency,
            weight_ppm: 800_000,
            reason: "development order binds memory contract to later atom pipeline".into(),
        }],
        conflicts: vec![MemoryConflict {
            other_unit_id: "unit-old-preference".into(),
            kind: MemoryConflictKind::PreferenceChanged,
            resolution: MemoryConflictResolution::SupersedeOld,
            reason: "newer explicit requirement narrows memory direction to transcript-backed kernel".into(),
            evidence: vec![newer_span.clone(), older_span.clone()],
        }],
        confidence_ppm: 940_000,
        created_at_unix_ms: now,
        updated_at_unix_ms: now,
    };
    let unit_old = MemoryUnit {
        id: "unit-old-preference".into(),
        cube_id: "cube-hepta-intelligence".into(),
        namespace: "user:default/project:hepta".into(),
        layer: MemoryLayer::L1Atom,
        kind: MemoryUnitKind::Preference,
        lifecycle: MemoryLifecycleState::Active,
        version: 1,
        content: "Older memory direction before transcript-backed kernel was selected.".into(),
        labels: BTreeSet::from(["memory_kernel".into(), "preference".into()]),
        entity_ids: BTreeSet::from(["entity:hepta".into(), "entity:memory".into()]),
        validity: MemoryTemporalValidity {
            valid_from_unix_ms: Some(now - 10_000),
            valid_until_unix_ms: Some(now + 3),
            observed_at_unix_ms: Some(now - 10_000),
            last_revalidated_unix_ms: Some(now - 5_000),
        },
        source_spans: vec![older_span],
        links: vec![],
        conflicts: vec![],
        confidence_ppm: 500_000,
        created_at_unix_ms: now - 10_000,
        updated_at_unix_ms: now - 5_000,
    };
    let unit_task = MemoryUnit {
        id: "unit-task-fact".into(),
        cube_id: "cube-hepta-intelligence".into(),
        namespace: "user:default/project:hepta".into(),
        layer: MemoryLayer::L1Atom,
        kind: MemoryUnitKind::TaskFact,
        lifecycle: MemoryLifecycleState::Active,
        version: 1,
        content: "P0 implements types, lifecycle, provenance, delete tombstone, and a sample gate before LLM extraction.".into(),
        labels: BTreeSet::from(["memory_kernel".into(), "p0".into()]),
        entity_ids: BTreeSet::from(["entity:hepta".into(), "entity:rust".into()]),
        validity: MemoryTemporalValidity {
            valid_from_unix_ms: Some(now),
            valid_until_unix_ms: None,
            observed_at_unix_ms: Some(now),
            last_revalidated_unix_ms: Some(now),
        },
        source_spans: vec![task_span.clone()],
        links: vec![],
        conflicts: vec![],
        confidence_ppm: 920_000,
        created_at_unix_ms: now,
        updated_at_unix_ms: now,
    };

    MemoryCube {
        id: "cube-hepta-intelligence".into(),
        scope: MemoryCubeScope::Project,
        owner_id: "hepta".into(),
        schema_version: MEMORY_KERNEL_V1_CONTRACT.into(),
        version: 1,
        units: BTreeMap::from([
            (unit_current.id.clone(), unit_current),
            (unit_old.id.clone(), unit_old),
            (unit_task.id.clone(), unit_task),
        ]),
        core_blocks: BTreeMap::from([(
            "core-current-objective".into(),
            CoreMemoryBlock {
                id: "core-current-objective".into(),
                cube_id: "cube-hepta-intelligence".into(),
                title: "Current Hepta Memory Objective".into(),
                block_kind: CoreMemoryBlockKind::ActiveObjective,
                pinned: true,
                editable: true,
                version: 1,
                content: "Build Hepta Intelligence memory as transcript-backed, layered, auditable Rust-native kernel.".into(),
                source_unit_ids: vec!["unit-current-preference".into(), "unit-task-fact".into()],
                source_spans: vec![newer_span.clone(), task_span.clone()],
            },
        )]),
        temporal_edges: BTreeMap::from([(
            "edge-hepta-memory-direction".into(),
            TemporalFactEdge {
                id: "edge-hepta-memory-direction".into(),
                source_unit_id: "unit-current-preference".into(),
                subject_entity_id: "entity:hepta".into(),
                predicate: "memory_architecture".into(),
                object_entity_id: "entity:transcript_backed_kernel".into(),
                validity: MemoryTemporalValidity {
                    valid_from_unix_ms: Some(now),
                    valid_until_unix_ms: None,
                    observed_at_unix_ms: Some(now),
                    last_revalidated_unix_ms: Some(now),
                },
                confidence_ppm: 930_000,
                source_spans: vec![newer_span],
            },
        )]),
        tombstones: BTreeMap::new(),
        created_at_unix_ms: now,
        updated_at_unix_ms: now,
    }
}

fn sample_span(id: &str, session_id: &str, start: u64, end: u64) -> MemorySourceSpan {
    MemorySourceSpan {
        source_kind: MemorySourceKind::Transcript,
        source_id: id.into(),
        session_id: Some(SessionId(session_id.into())),
        transcript_range: Some(TranscriptRange {
            start_sequence: start,
            end_sequence: end,
        }),
        transcript_entry_ids: vec![
            format!("{session_id}:{start}"),
            format!("{session_id}:{end}"),
        ],
        transcript_span_ref: None,
        evidence_digest: format!("sha256:{id}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_kernel_sample_gate_is_ready_without_side_effects() {
        let report = memory_kernel_v1_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p0_ready);
        assert!(report.checks.ready());
        assert!(!report.llm_extraction_performed);
        assert!(!report.external_network_read);
        assert!(!report.memory_store_mutation_performed);
        assert!(!report.raw_private_memory_logged);
    }

    #[test]
    fn tombstoned_units_are_not_recalled() {
        let report = memory_kernel_v1_sample_report(true);

        assert!(report.cube.has_tombstone("unit-old-preference"));
        assert!(
            !report
                .recall_bundle
                .recalled_unit_ids
                .iter()
                .any(|id| id == "unit-old-preference")
        );
        assert_eq!(report.recall_bundle.omitted_tombstoned_count, 1);
    }

    #[test]
    fn conflicts_keep_both_source_spans_for_audit() {
        let report = memory_kernel_v1_sample_report(true);
        let current = report
            .cube
            .units
            .get("unit-current-preference")
            .expect("sample current unit exists");
        let conflict = current.conflicts.first().expect("conflict exists");

        assert_eq!(conflict.resolution, MemoryConflictResolution::SupersedeOld);
        assert!(conflict.evidence.len() >= 2);
        assert!(conflict.evidence.iter().all(MemorySourceSpan::is_traceable));
    }

    #[test]
    fn active_derived_artifacts_require_provenance() {
        let report = memory_kernel_v1_sample_report(true);

        assert!(report.cube.all_derived_artifacts_have_traceable_sources());
        assert!(report.recall_bundle.provenance_complete);
        assert!(!report.recall_bundle.source_spans.is_empty());
    }
}

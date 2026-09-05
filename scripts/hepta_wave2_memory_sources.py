#!/usr/bin/env python3
"""Source definitions for the bounded cognitive-memory closure wave."""

from __future__ import annotations

RUST_PACKAGES = {
    "hepta-cognitive-types": "codex-hepta-cognitive-types",
    "hepta-cognitive-store": "codex-hepta-cognitive-store",
    "hepta-cognitive-read": "codex-hepta-cognitive-read",
    "hepta-memory-retrieval": "codex-hepta-memory-retrieval",
    "hepta-memory-federation": "codex-hepta-memory-federation",
    "hepta-kg": "codex-hepta-kg",
    "hepta-compact-engine": "codex-hepta-compact-engine",
    "hepta-context-compiler": "codex-hepta-context-compiler",
}

SOURCE_ROOTS = {
    "cognitive.types": ("codex-rs/hepta-cognitive-types",),
    "cognitive.store": ("codex-rs/hepta-cognitive-store",),
    "cognitive.read": ("codex-rs/hepta-cognitive-read",),
    "memory.retrieval": ("codex-rs/hepta-memory-retrieval",),
    "memory.federation": ("codex-rs/hepta-memory-federation",),
    "knowledge.graph": ("codex-rs/hepta-kg",),
    "compact.engine": ("codex-rs/hepta-compact-engine",),
    "context.compiler": ("codex-rs/hepta-context-compiler",),
}


def manifest(
    package: str, lib: str, dependencies: tuple[tuple[str, str], ...] = ()
) -> str:
    dependency_lines = ['codex-hepta-types = { path = "../hepta-types" }']
    dependency_lines.extend(
        f'{name} = {{ path = "../{path}" }}' for name, path in dependencies
    )
    return f"""[package]
edition.workspace = true
license.workspace = true
name = "{package}"
version.workspace = true
publish = false

[lib]
name = "{lib}"
path = "src/lib.rs"
doctest = false

[lints]
workspace = true

[dependencies]
{chr(10).join(dependency_lines)}

[dev-dependencies]
pretty_assertions = {{ workspace = true }}
"""


def build(root: str) -> str:
    return f"""load("//:defs.bzl", "codex_rust_crate")

codex_rust_crate(
    name = "{root}",
    crate_name = "codex_{root.replace("-", "_")}",
)
"""


FILES = {
    "codex-rs/hepta-cognitive-types/Cargo.toml": manifest(
        "codex-hepta-cognitive-types", "codex_hepta_cognitive_types"
    ),
    "codex-rs/hepta-cognitive-types/BUILD.bazel": build("hepta-cognitive-types"),
    "codex-rs/hepta-cognitive-types/src/lib.rs": r"""//! Stable cognitive records and snapshot contracts.
//!
//! This crate defines bounded values only. It owns no SQL, daemon, model,
//! network, effect, selection, promotion or release authority.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::error::Error as StdError;
use std::fmt;

use codex_hepta_types::{AuthorityPosture, Digest32, Generation, Revision, StableId};

const MAX_CITATIONS: usize = 64;
const MAX_RECORDS: usize = 16_384;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MemoryKind {
    Episode,
    Fact,
    Preference,
    Procedure,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RecordState {
    Live,
    Tombstone,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Citation {
    pub source_id: StableId,
    pub source_digest: Digest32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryRecord {
    pub record_id: StableId,
    pub revision: Revision,
    pub kind: MemoryKind,
    pub content_digest: Digest32,
    pub predecessor_digest: Option<Digest32>,
    pub citations: Vec<Citation>,
    pub state: RecordState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CognitiveSnapshot {
    pub generation: Generation,
    pub records: Vec<MemoryRecord>,
    pub snapshot_digest: Digest32,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptyDigest(&'static str),
    CitationLimitExceeded,
    DuplicateCitation(String),
    MissingPredecessor,
    UnexpectedPredecessor,
    RecordLimitExceeded,
    DuplicateRecord(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

impl MemoryRecord {
    pub fn validate(&self) -> Result<(), Error> {
        if self.content_digest.is_zero() {
            return Err(Error::EmptyDigest("content"));
        }
        if self.citations.len() > MAX_CITATIONS {
            return Err(Error::CitationLimitExceeded);
        }
        if self.revision.get() == 1 && self.predecessor_digest.is_some() {
            return Err(Error::UnexpectedPredecessor);
        }
        if self.revision.get() > 1 && self.predecessor_digest.is_none() {
            return Err(Error::MissingPredecessor);
        }
        if self.predecessor_digest.is_some_and(Digest32::is_zero) {
            return Err(Error::EmptyDigest("predecessor"));
        }
        let mut seen = BTreeSet::new();
        for citation in &self.citations {
            if citation.source_digest.is_zero() {
                return Err(Error::EmptyDigest("citation"));
            }
            let key = (citation.source_id.clone(), citation.source_digest);
            if !seen.insert(key) {
                return Err(Error::DuplicateCitation(citation.source_id.to_string()));
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn record_digest(&self) -> Digest32 {
        let mut citations = self.citations.clone();
        citations.sort();
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"hepta.cognitive.record.v1");
        push_id(&mut bytes, &self.record_id);
        bytes.extend_from_slice(&self.revision.get().to_be_bytes());
        bytes.push(kind_code(self.kind));
        bytes.push(state_code(self.state));
        bytes.extend_from_slice(self.content_digest.as_array());
        match self.predecessor_digest {
            Some(value) => {
                bytes.push(1);
                bytes.extend_from_slice(value.as_array());
            }
            None => bytes.push(0),
        }
        for citation in citations {
            push_id(&mut bytes, &citation.source_id);
            bytes.extend_from_slice(citation.source_digest.as_array());
        }
        Digest32::of_bytes(&bytes)
    }
}

pub fn build_snapshot(
    generation: Generation,
    mut records: Vec<MemoryRecord>,
) -> Result<CognitiveSnapshot, Error> {
    if records.len() > MAX_RECORDS {
        return Err(Error::RecordLimitExceeded);
    }
    records.sort_by(|left, right| {
        left.record_id
            .cmp(&right.record_id)
            .then_with(|| left.revision.cmp(&right.revision))
    });
    let mut identities = BTreeSet::new();
    for record in &records {
        record.validate()?;
        if !identities.insert((record.record_id.clone(), record.revision)) {
            return Err(Error::DuplicateRecord(record.record_id.to_string()));
        }
    }
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.cognitive.snapshot.v1");
    bytes.extend_from_slice(&generation.get().to_be_bytes());
    for record in &records {
        bytes.extend_from_slice(record.record_digest().as_array());
    }
    Ok(CognitiveSnapshot {
        generation,
        records,
        snapshot_digest: Digest32::of_bytes(&bytes),
        authority: AuthorityPosture::DENY_ALL,
    })
}

fn kind_code(value: MemoryKind) -> u8 {
    match value {
        MemoryKind::Episode => 0,
        MemoryKind::Fact => 1,
        MemoryKind::Preference => 2,
        MemoryKind::Procedure => 3,
    }
}

fn state_code(value: RecordState) -> u8 {
    match value {
        RecordState::Live => 0,
        RecordState::Tombstone => 1,
    }
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    bytes.extend_from_slice(&u32::try_from(raw.len()).unwrap_or(u32::MAX).to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
""",
    "codex-rs/hepta-cognitive-types/src/lib_tests.rs": r"""use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn digest(value: &[u8]) -> Digest32 {
    Digest32::of_bytes(value)
}

fn revision(value: u64) -> Revision {
    let Ok(value) = Revision::new(value) else {
        panic!("test revision must be valid");
    };
    value
}

fn generation(value: u64) -> Generation {
    let Ok(value) = Generation::new(value) else {
        panic!("test generation must be valid");
    };
    value
}

fn record(name: &str) -> MemoryRecord {
    MemoryRecord {
        record_id: id(name),
        revision: revision(1),
        kind: MemoryKind::Fact,
        content_digest: digest(name.as_bytes()),
        predecessor_digest: None,
        citations: vec![Citation {
            source_id: id("source:1"),
            source_digest: digest(b"source"),
        }],
        state: RecordState::Live,
    }
}

#[test]
fn snapshot_is_canonical_and_authority_free() {
    let left = build_snapshot(generation(1), vec![record("record:b"), record("record:a")]);
    let right = build_snapshot(generation(1), vec![record("record:a"), record("record:b")]);
    let (Ok(left), Ok(right)) = (left, right) else {
        panic!("canonical snapshots must build");
    };
    assert_eq!(left, right);
    assert!(!left.authority.grants_any());
}

#[test]
fn later_revision_requires_predecessor() {
    let mut value = record("record:1");
    value.revision = revision(2);
    assert_eq!(value.validate(), Err(Error::MissingPredecessor));
}

#[test]
fn duplicate_citation_is_rejected() {
    let mut value = record("record:1");
    value.citations.push(value.citations[0].clone());
    assert_eq!(
        value.validate(),
        Err(Error::DuplicateCitation("source:1".to_string()))
    );
}

#[test]
fn duplicate_record_revision_is_rejected() {
    let value = record("record:1");
    assert_eq!(
        build_snapshot(generation(1), vec![value.clone(), value]),
        Err(Error::DuplicateRecord("record:1".to_string()))
    );
}
""",
    "codex-rs/hepta-cognitive-store/Cargo.toml": manifest(
        "codex-hepta-cognitive-store",
        "codex_hepta_cognitive_store",
        (("codex-hepta-cognitive-types", "hepta-cognitive-types"),),
    ),
    "codex-rs/hepta-cognitive-store/BUILD.bazel": build("hepta-cognitive-store"),
    "codex-rs/hepta-cognitive-store/src/lib.rs": r"""//! Append-only cognitive ledger with correction and tombstone lineage.
//!
//! The store is the only writer of its in-memory qualification ledger. It does
//! not perform federation, model calls, learning-policy writes or effects.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt;

use codex_hepta_cognitive_types::{MemoryRecord, RecordState};
use codex_hepta_types::{AuthorityPosture, Digest32, LogicalSequence, StableId};

const MAX_RECORDS: usize = 16_384;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppendDisposition {
    Inserted,
    Unchanged,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoreReceipt {
    pub record_id: StableId,
    pub sequence: LogicalSequence,
    pub record_digest: Digest32,
    pub disposition: AppendDisposition,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    ZeroCapacity,
    CapacityExceeded,
    InvalidRecord(String),
    StalePredecessor,
    RevisionNotAdvanced,
    ResurrectionDenied,
    IdentityConflict(String),
    SequenceOverflow,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CognitiveStore {
    records: BTreeMap<StableId, MemoryRecord>,
    sequence: LogicalSequence,
    maximum_records: usize,
}

impl CognitiveStore {
    pub fn new(maximum_records: usize) -> Result<Self, Error> {
        if maximum_records == 0 {
            return Err(Error::ZeroCapacity);
        }
        let Ok(sequence) = LogicalSequence::new(1) else {
            return Err(Error::SequenceOverflow);
        };
        Ok(Self {
            records: BTreeMap::new(),
            sequence,
            maximum_records: maximum_records.min(MAX_RECORDS),
        })
    }

    pub fn append(
        &mut self,
        record: MemoryRecord,
        expected_predecessor: Option<Digest32>,
    ) -> Result<StoreReceipt, Error> {
        record
            .validate()
            .map_err(|error| Error::InvalidRecord(error.to_string()))?;
        let digest = record.record_digest();

        if let Some(existing) = self.records.get(&record.record_id) {
            let existing_digest = existing.record_digest();
            if existing_digest == digest {
                return Ok(self.receipt(
                    record.record_id,
                    digest,
                    AppendDisposition::Unchanged,
                ));
            }
            if existing.state == RecordState::Tombstone {
                return Err(Error::ResurrectionDenied);
            }
            if expected_predecessor != Some(existing_digest)
                || record.predecessor_digest != Some(existing_digest)
            {
                return Err(Error::StalePredecessor);
            }
            if record.revision.get() != existing.revision.get().saturating_add(1) {
                return Err(Error::RevisionNotAdvanced);
            }
        } else {
            if self.records.len() >= self.maximum_records {
                return Err(Error::CapacityExceeded);
            }
            if expected_predecessor.is_some()
                || record.revision.get() != 1
                || record.predecessor_digest.is_some()
            {
                return Err(Error::StalePredecessor);
            }
        }

        self.records.insert(record.record_id.clone(), record.clone());
        self.sequence = self.sequence.next().map_err(|_| Error::SequenceOverflow)?;
        Ok(self.receipt(record.record_id, digest, AppendDisposition::Inserted))
    }

    #[must_use]
    pub fn get(&self, record_id: &StableId) -> Option<&MemoryRecord> {
        self.records.get(record_id)
    }

    #[must_use]
    pub fn snapshot_records(&self) -> Vec<MemoryRecord> {
        self.records.values().cloned().collect()
    }

    fn receipt(
        &self,
        record_id: StableId,
        record_digest: Digest32,
        disposition: AppendDisposition,
    ) -> StoreReceipt {
        StoreReceipt {
            record_id,
            sequence: self.sequence,
            record_digest,
            disposition,
            authority: AuthorityPosture::DENY_ALL,
        }
    }
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
""",
    "codex-rs/hepta-cognitive-store/src/lib_tests.rs": r"""use super::*;
use codex_hepta_cognitive_types::{MemoryKind, RecordState};
use codex_hepta_types::Revision;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn revision(value: u64) -> Revision {
    let Ok(value) = Revision::new(value) else {
        panic!("test revision must be valid");
    };
    value
}

fn digest(value: &[u8]) -> Digest32 {
    Digest32::of_bytes(value)
}

fn record(revision_value: u64, predecessor: Option<Digest32>, state: RecordState) -> MemoryRecord {
    MemoryRecord {
        record_id: id("memory:1"),
        revision: revision(revision_value),
        kind: MemoryKind::Fact,
        content_digest: digest(format!("content:{revision_value}").as_bytes()),
        predecessor_digest: predecessor,
        citations: Vec::new(),
        state,
    }
}

fn store() -> CognitiveStore {
    let Ok(value) = CognitiveStore::new(8) else {
        panic!("test store must initialize");
    };
    value
}

#[test]
fn append_and_correction_are_predecessor_fenced() {
    let mut value = store();
    let first = record(1, None, RecordState::Live);
    let first_digest = first.record_digest();
    assert!(value.append(first, None).is_ok());
    let second = record(2, Some(first_digest), RecordState::Live);
    let Ok(receipt) = value.append(second, Some(first_digest)) else {
        panic!("fenced correction must succeed");
    };
    assert!(!receipt.authority.grants_any());
}

#[test]
fn stale_correction_is_rejected() {
    let mut value = store();
    let first = record(1, None, RecordState::Live);
    let first_digest = first.record_digest();
    assert!(value.append(first, None).is_ok());
    let second = record(2, Some(first_digest), RecordState::Live);
    assert_eq!(
        value.append(second, Some(digest(b"stale"))),
        Err(Error::StalePredecessor)
    );
}

#[test]
fn tombstone_prevents_resurrection() {
    let mut value = store();
    let first = record(1, None, RecordState::Live);
    let first_digest = first.record_digest();
    assert!(value.append(first, None).is_ok());
    let tombstone = record(2, Some(first_digest), RecordState::Tombstone);
    let tombstone_digest = tombstone.record_digest();
    assert!(value.append(tombstone, Some(first_digest)).is_ok());
    let resurrected = record(3, Some(tombstone_digest), RecordState::Live);
    assert_eq!(
        value.append(resurrected, Some(tombstone_digest)),
        Err(Error::ResurrectionDenied)
    );
}

#[test]
fn identical_append_is_idempotent() {
    let mut value = store();
    let first = record(1, None, RecordState::Live);
    assert_eq!(
        value.append(first.clone(), None).map(|receipt| receipt.disposition),
        Ok(AppendDisposition::Inserted)
    );
    assert_eq!(
        value.append(first, None).map(|receipt| receipt.disposition),
        Ok(AppendDisposition::Unchanged)
    );
}
""",
    "codex-rs/hepta-cognitive-read/Cargo.toml": manifest(
        "codex-hepta-cognitive-read",
        "codex_hepta_cognitive_read",
        (("codex-hepta-cognitive-types", "hepta-cognitive-types"),),
    ),
    "codex-rs/hepta-cognitive-read/BUILD.bazel": build("hepta-cognitive-read"),
    "codex-rs/hepta-cognitive-read/src/lib.rs": r"""//! Snapshot-bound, redaction-aware cognitive read port.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::error::Error as StdError;
use std::fmt;

use codex_hepta_cognitive_types::{CognitiveSnapshot, MemoryKind, MemoryRecord, RecordState};
use codex_hepta_types::{AuthorityPosture, Digest32};

const MAX_RESULTS: usize = 1_024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadRequest {
    pub snapshot_digest: Digest32,
    pub allowed_kinds: Vec<MemoryKind>,
    pub maximum_results: usize,
    pub include_tombstones: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadReceipt {
    pub snapshot_digest: Digest32,
    pub records: Vec<MemoryRecord>,
    pub omitted_count: usize,
    pub receipt_digest: Digest32,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    SnapshotMismatch,
    InvalidMaximumResults,
    DuplicateKind,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

pub fn read(snapshot: &CognitiveSnapshot, request: ReadRequest) -> Result<ReadReceipt, Error> {
    if request.snapshot_digest != snapshot.snapshot_digest {
        return Err(Error::SnapshotMismatch);
    }
    if request.maximum_results == 0 || request.maximum_results > MAX_RESULTS {
        return Err(Error::InvalidMaximumResults);
    }
    let mut allowed = BTreeSet::new();
    for kind in request.allowed_kinds {
        if !allowed.insert(kind) {
            return Err(Error::DuplicateKind);
        }
    }

    let mut eligible = snapshot
        .records
        .iter()
        .filter(|record| {
            (allowed.is_empty() || allowed.contains(&record.kind))
                && (request.include_tombstones || record.state == RecordState::Live)
        })
        .cloned()
        .collect::<Vec<_>>();
    eligible.sort_by(|left, right| {
        left.record_id
            .cmp(&right.record_id)
            .then_with(|| left.revision.cmp(&right.revision))
    });
    let omitted_count = eligible.len().saturating_sub(request.maximum_results);
    eligible.truncate(request.maximum_results);

    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.cognitive.read.v1");
    bytes.extend_from_slice(snapshot.snapshot_digest.as_array());
    for record in &eligible {
        bytes.extend_from_slice(record.record_digest().as_array());
    }
    bytes.extend_from_slice(
        &u64::try_from(omitted_count)
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );

    Ok(ReadReceipt {
        snapshot_digest: snapshot.snapshot_digest,
        records: eligible,
        omitted_count,
        receipt_digest: Digest32::of_bytes(&bytes),
        authority: AuthorityPosture::DENY_ALL,
    })
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
""",
    "codex-rs/hepta-cognitive-read/src/lib_tests.rs": r"""use super::*;
use codex_hepta_cognitive_types::{build_snapshot, MemoryRecord};
use codex_hepta_types::{Generation, Revision, StableId};

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn revision(value: u64) -> Revision {
    let Ok(value) = Revision::new(value) else {
        panic!("test revision must be valid");
    };
    value
}

fn generation(value: u64) -> Generation {
    let Ok(value) = Generation::new(value) else {
        panic!("test generation must be valid");
    };
    value
}

fn record(name: &str, kind: MemoryKind, state: RecordState) -> MemoryRecord {
    MemoryRecord {
        record_id: id(name),
        revision: revision(1),
        kind,
        content_digest: Digest32::of_bytes(name.as_bytes()),
        predecessor_digest: None,
        citations: Vec::new(),
        state,
    }
}

fn snapshot() -> CognitiveSnapshot {
    let result = build_snapshot(
        generation(1),
        vec![
            record("memory:a", MemoryKind::Fact, RecordState::Live),
            record("memory:b", MemoryKind::Episode, RecordState::Live),
            record("memory:c", MemoryKind::Fact, RecordState::Tombstone),
        ],
    );
    let Ok(value) = result else {
        panic!("test snapshot must build");
    };
    value
}

#[test]
fn read_is_snapshot_bound_and_authority_free() {
    let snapshot = snapshot();
    let request = ReadRequest {
        snapshot_digest: snapshot.snapshot_digest,
        allowed_kinds: vec![MemoryKind::Fact],
        maximum_results: 8,
        include_tombstones: false,
    };
    let Ok(receipt) = read(&snapshot, request) else {
        panic!("bounded read must succeed");
    };
    assert_eq!(receipt.records.len(), 1);
    assert_eq!(receipt.records[0].record_id, id("memory:a"));
    assert!(!receipt.authority.grants_any());
}

#[test]
fn stale_snapshot_is_rejected() {
    let snapshot = snapshot();
    let request = ReadRequest {
        snapshot_digest: Digest32::of_bytes(b"stale"),
        allowed_kinds: Vec::new(),
        maximum_results: 8,
        include_tombstones: false,
    };
    assert_eq!(read(&snapshot, request), Err(Error::SnapshotMismatch));
}

#[test]
fn bounded_result_count_is_explicit() {
    let snapshot = snapshot();
    let request = ReadRequest {
        snapshot_digest: snapshot.snapshot_digest,
        allowed_kinds: Vec::new(),
        maximum_results: 1,
        include_tombstones: true,
    };
    let Ok(receipt) = read(&snapshot, request) else {
        panic!("bounded read must succeed");
    };
    assert_eq!(receipt.records.len(), 1);
    assert_eq!(receipt.omitted_count, 2);
}
""",
    "codex-rs/hepta-memory-retrieval/Cargo.toml": manifest(
        "codex-hepta-memory-retrieval",
        "codex_hepta_memory_retrieval",
        (("codex-hepta-cognitive-types", "hepta-cognitive-types"),),
    ),
    "codex-rs/hepta-memory-retrieval/BUILD.bazel": build("hepta-memory-retrieval"),
    "codex-rs/hepta-memory-retrieval/src/lib.rs": r"""//! Explainable, snapshot-revalidated local memory retrieval.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::error::Error as StdError;
use std::fmt;

use codex_hepta_cognitive_types::MemoryRecord;
use codex_hepta_types::{AuthorityPosture, Digest32, FixedQ32, StableId};

const MAX_CANDIDATES: usize = 16_384;
const MAX_RESULTS: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetrievalCandidate {
    pub record: MemoryRecord,
    pub snapshot_digest: Digest32,
    pub lexical_score: FixedQ32,
    pub graph_score: FixedQ32,
    pub freshness_score: FixedQ32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetrievalRequest {
    pub query_id: StableId,
    pub query_digest: Digest32,
    pub snapshot_digest: Digest32,
    pub maximum_results: usize,
    pub candidates: Vec<RetrievalCandidate>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetrievalResult {
    pub record_id: StableId,
    pub record_digest: Digest32,
    pub total_score: FixedQ32,
    pub lexical_score: FixedQ32,
    pub graph_score: FixedQ32,
    pub freshness_score: FixedQ32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetrievalReceipt {
    pub query_id: StableId,
    pub snapshot_digest: Digest32,
    pub results: Vec<RetrievalResult>,
    pub omitted_count: usize,
    pub receipt_digest: Digest32,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptyDigest(&'static str),
    InvalidMaximumResults,
    CandidateLimitExceeded,
    SnapshotMismatch(String),
    DuplicateRecord(String),
    InvalidRecord(String),
    Arithmetic,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

pub fn retrieve(request: RetrievalRequest) -> Result<RetrievalReceipt, Error> {
    if request.query_digest.is_zero() {
        return Err(Error::EmptyDigest("query"));
    }
    if request.snapshot_digest.is_zero() {
        return Err(Error::EmptyDigest("snapshot"));
    }
    if request.maximum_results == 0 || request.maximum_results > MAX_RESULTS {
        return Err(Error::InvalidMaximumResults);
    }
    if request.candidates.len() > MAX_CANDIDATES {
        return Err(Error::CandidateLimitExceeded);
    }

    let mut seen = BTreeSet::new();
    let mut results = Vec::with_capacity(request.candidates.len());
    for candidate in request.candidates {
        if candidate.snapshot_digest != request.snapshot_digest {
            return Err(Error::SnapshotMismatch(
                candidate.record.record_id.to_string(),
            ));
        }
        candidate
            .record
            .validate()
            .map_err(|error| Error::InvalidRecord(error.to_string()))?;
        if !seen.insert(candidate.record.record_id.clone()) {
            return Err(Error::DuplicateRecord(
                candidate.record.record_id.to_string(),
            ));
        }
        let score = candidate
            .lexical_score
            .checked_add(candidate.graph_score)
            .and_then(|value| value.checked_add(candidate.freshness_score))
            .map_err(|_| Error::Arithmetic)?;
        results.push(RetrievalResult {
            record_id: candidate.record.record_id,
            record_digest: candidate.record.record_digest(),
            total_score: score,
            lexical_score: candidate.lexical_score,
            graph_score: candidate.graph_score,
            freshness_score: candidate.freshness_score,
        });
    }
    results.sort_by(|left, right| {
        right
            .total_score
            .cmp(&left.total_score)
            .then_with(|| left.record_id.cmp(&right.record_id))
    });
    let omitted_count = results.len().saturating_sub(request.maximum_results);
    results.truncate(request.maximum_results);

    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.memory.retrieval.v1");
    push_id(&mut bytes, &request.query_id);
    bytes.extend_from_slice(request.query_digest.as_array());
    bytes.extend_from_slice(request.snapshot_digest.as_array());
    for result in &results {
        push_id(&mut bytes, &result.record_id);
        bytes.extend_from_slice(result.record_digest.as_array());
        bytes.extend_from_slice(&result.total_score.raw().to_be_bytes());
        bytes.extend_from_slice(&result.lexical_score.raw().to_be_bytes());
        bytes.extend_from_slice(&result.graph_score.raw().to_be_bytes());
        bytes.extend_from_slice(&result.freshness_score.raw().to_be_bytes());
    }

    Ok(RetrievalReceipt {
        query_id: request.query_id,
        snapshot_digest: request.snapshot_digest,
        results,
        omitted_count,
        receipt_digest: Digest32::of_bytes(&bytes),
        authority: AuthorityPosture::DENY_ALL,
    })
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    bytes.extend_from_slice(&u32::try_from(raw.len()).unwrap_or(u32::MAX).to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
""",
    "codex-rs/hepta-memory-retrieval/src/lib_tests.rs": r"""use super::*;
use codex_hepta_cognitive_types::{MemoryKind, RecordState};
use codex_hepta_types::Revision;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn revision() -> Revision {
    let Ok(value) = Revision::new(1) else {
        panic!("test revision must be valid");
    };
    value
}

fn candidate(name: &str, snapshot: Digest32, score: i64) -> RetrievalCandidate {
    RetrievalCandidate {
        record: MemoryRecord {
            record_id: id(name),
            revision: revision(),
            kind: MemoryKind::Fact,
            content_digest: Digest32::of_bytes(name.as_bytes()),
            predecessor_digest: None,
            citations: Vec::new(),
            state: RecordState::Live,
        },
        snapshot_digest: snapshot,
        lexical_score: FixedQ32::from_raw(score),
        graph_score: FixedQ32::ZERO,
        freshness_score: FixedQ32::ZERO,
    }
}

#[test]
fn ranking_is_deterministic_and_explainable() {
    let snapshot = Digest32::of_bytes(b"snapshot");
    let request = RetrievalRequest {
        query_id: id("query:1"),
        query_digest: Digest32::of_bytes(b"query"),
        snapshot_digest: snapshot,
        maximum_results: 2,
        candidates: vec![
            candidate("memory:b", snapshot, 10),
            candidate("memory:a", snapshot, 10),
        ],
    };
    let Ok(receipt) = retrieve(request) else {
        panic!("retrieval must succeed");
    };
    assert_eq!(receipt.results[0].record_id, id("memory:a"));
    assert_eq!(receipt.results[0].lexical_score, FixedQ32::from_raw(10));
    assert!(!receipt.authority.grants_any());
}

#[test]
fn stale_candidate_snapshot_is_rejected() {
    let snapshot = Digest32::of_bytes(b"snapshot");
    let request = RetrievalRequest {
        query_id: id("query:1"),
        query_digest: Digest32::of_bytes(b"query"),
        snapshot_digest: snapshot,
        maximum_results: 1,
        candidates: vec![candidate(
            "memory:a",
            Digest32::of_bytes(b"stale"),
            10,
        )],
    };
    assert_eq!(
        retrieve(request),
        Err(Error::SnapshotMismatch("memory:a".to_string()))
    );
}

#[test]
fn result_count_is_bounded() {
    let snapshot = Digest32::of_bytes(b"snapshot");
    let request = RetrievalRequest {
        query_id: id("query:1"),
        query_digest: Digest32::of_bytes(b"query"),
        snapshot_digest: snapshot,
        maximum_results: 1,
        candidates: vec![
            candidate("memory:a", snapshot, 20),
            candidate("memory:b", snapshot, 10),
        ],
    };
    let Ok(receipt) = retrieve(request) else {
        panic!("retrieval must succeed");
    };
    assert_eq!(receipt.results.len(), 1);
    assert_eq!(receipt.omitted_count, 1);
}
""",
    "codex-rs/hepta-memory-federation/Cargo.toml": manifest(
        "codex-hepta-memory-federation", "codex_hepta_memory_federation"
    ),
    "codex-rs/hepta-memory-federation/BUILD.bazel": build("hepta-memory-federation"),
    "codex-rs/hepta-memory-federation/src/lib.rs": r"""//! Scoped, fail-closed remote cognitive read verification.

#![forbid(unsafe_code)]

use std::error::Error as StdError;
use std::fmt;

use codex_hepta_types::{AuthorityPosture, Digest32, StableId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederatedReadRequest {
    pub request_id: StableId,
    pub peer_id: StableId,
    pub scope_digest: Digest32,
    pub source_snapshot_digest: Digest32,
    pub request_digest: Digest32,
    pub deadline_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederatedReadLease {
    pub lease_id: StableId,
    pub request_id: StableId,
    pub peer_id: StableId,
    pub scope_digest: Digest32,
    pub source_snapshot_digest: Digest32,
    pub request_digest: Digest32,
    pub expires_at_ms: u64,
    pub revoked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteObservation {
    pub response_digest: Digest32,
    pub terminal_observed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FederatedStatus {
    Succeeded,
    Indeterminate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederatedReadReceipt {
    pub request_id: StableId,
    pub lease_id: StableId,
    pub status: FederatedStatus,
    pub response_digest: Option<Digest32>,
    pub receipt_digest: Digest32,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptyDigest(&'static str),
    DeadlineExpired,
    LeaseExpired,
    LeaseRevoked,
    IdentityMismatch(&'static str),
    DigestMismatch(&'static str),
    MissingTerminalResponse,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

pub fn observe(
    now_ms: u64,
    request: FederatedReadRequest,
    lease: FederatedReadLease,
    observation: Option<RemoteObservation>,
) -> Result<FederatedReadReceipt, Error> {
    for (name, digest) in [
        ("scope", request.scope_digest),
        ("snapshot", request.source_snapshot_digest),
        ("request", request.request_digest),
    ] {
        if digest.is_zero() {
            return Err(Error::EmptyDigest(name));
        }
    }
    if now_ms >= request.deadline_ms {
        return Err(Error::DeadlineExpired);
    }
    if lease.revoked {
        return Err(Error::LeaseRevoked);
    }
    if now_ms >= lease.expires_at_ms {
        return Err(Error::LeaseExpired);
    }
    if lease.request_id != request.request_id {
        return Err(Error::IdentityMismatch("request"));
    }
    if lease.peer_id != request.peer_id {
        return Err(Error::IdentityMismatch("peer"));
    }
    for (name, left, right) in [
        ("scope", lease.scope_digest, request.scope_digest),
        (
            "snapshot",
            lease.source_snapshot_digest,
            request.source_snapshot_digest,
        ),
        ("request", lease.request_digest, request.request_digest),
    ] {
        if left != right {
            return Err(Error::DigestMismatch(name));
        }
    }

    let (status, response_digest) = match observation {
        None => (FederatedStatus::Indeterminate, None),
        Some(value) if !value.terminal_observed => (FederatedStatus::Indeterminate, None),
        Some(value) => {
            if value.response_digest.is_zero() {
                return Err(Error::MissingTerminalResponse);
            }
            (FederatedStatus::Succeeded, Some(value.response_digest))
        }
    };
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.memory.federation.receipt.v1");
    push_id(&mut bytes, &request.request_id);
    push_id(&mut bytes, &lease.lease_id);
    bytes.push(match status {
        FederatedStatus::Succeeded => 0,
        FederatedStatus::Indeterminate => 1,
    });
    if let Some(digest) = response_digest {
        bytes.extend_from_slice(digest.as_array());
    }

    Ok(FederatedReadReceipt {
        request_id: request.request_id,
        lease_id: lease.lease_id,
        status,
        response_digest,
        receipt_digest: Digest32::of_bytes(&bytes),
        authority: AuthorityPosture::DENY_ALL,
    })
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    bytes.extend_from_slice(&u32::try_from(raw.len()).unwrap_or(u32::MAX).to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
""",
    "codex-rs/hepta-memory-federation/src/lib_tests.rs": r"""use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn digest(value: &[u8]) -> Digest32 {
    Digest32::of_bytes(value)
}

fn fixture() -> (FederatedReadRequest, FederatedReadLease) {
    let request = FederatedReadRequest {
        request_id: id("request:1"),
        peer_id: id("peer:1"),
        scope_digest: digest(b"scope"),
        source_snapshot_digest: digest(b"snapshot"),
        request_digest: digest(b"request"),
        deadline_ms: 2_000,
    };
    let lease = FederatedReadLease {
        lease_id: id("lease:1"),
        request_id: request.request_id.clone(),
        peer_id: request.peer_id.clone(),
        scope_digest: request.scope_digest,
        source_snapshot_digest: request.source_snapshot_digest,
        request_digest: request.request_digest,
        expires_at_ms: 1_500,
        revoked: false,
    };
    (request, lease)
}

#[test]
fn missing_terminal_observation_is_indeterminate() {
    let (request, lease) = fixture();
    let Ok(receipt) = observe(1_000, request, lease, None) else {
        panic!("bounded unknown outcome must be representable");
    };
    assert_eq!(receipt.status, FederatedStatus::Indeterminate);
    assert_eq!(receipt.response_digest, None);
    assert!(!receipt.authority.grants_any());
}

#[test]
fn revoked_lease_is_rejected() {
    let (request, mut lease) = fixture();
    lease.revoked = true;
    assert_eq!(observe(1_000, request, lease, None), Err(Error::LeaseRevoked));
}

#[test]
fn snapshot_drift_is_rejected() {
    let (request, mut lease) = fixture();
    lease.source_snapshot_digest = digest(b"drift");
    assert_eq!(
        observe(1_000, request, lease, None),
        Err(Error::DigestMismatch("snapshot"))
    );
}

#[test]
fn terminal_response_is_bound() {
    let (request, lease) = fixture();
    let observation = RemoteObservation {
        response_digest: digest(b"response"),
        terminal_observed: true,
    };
    let Ok(receipt) = observe(1_000, request, lease, Some(observation)) else {
        panic!("terminal observation must succeed");
    };
    assert_eq!(receipt.status, FederatedStatus::Succeeded);
    assert_eq!(receipt.response_digest, Some(digest(b"response")));
}
""",
    "codex-rs/hepta-kg/Cargo.toml": manifest(
        "codex-hepta-kg",
        "codex_hepta_kg",
        (("codex-hepta-cognitive-types", "hepta-cognitive-types"),),
    ),
    "codex-rs/hepta-kg/BUILD.bazel": build("hepta-kg"),
    "codex-rs/hepta-kg/src/lib.rs": r"""//! Rebuildable knowledge-graph projection.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::error::Error as StdError;
use std::fmt;

use codex_hepta_types::{AuthorityPosture, Digest32, Generation, StableId};

const MAX_EDGES: usize = 65_536;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct KnowledgeEdge {
    pub source_id: StableId,
    pub relation_id: StableId,
    pub target_id: StableId,
    pub source_fact_digest: Digest32,
    pub live: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KnowledgeProjection {
    pub generation: Generation,
    pub source_snapshot_digest: Digest32,
    pub edges: Vec<KnowledgeEdge>,
    pub projection_digest: Digest32,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptySourceSnapshot,
    EdgeLimitExceeded,
    EmptyFactDigest(String),
    DuplicateEdge(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

pub fn rebuild(
    generation: Generation,
    source_snapshot_digest: Digest32,
    mut edges: Vec<KnowledgeEdge>,
) -> Result<KnowledgeProjection, Error> {
    if source_snapshot_digest.is_zero() {
        return Err(Error::EmptySourceSnapshot);
    }
    if edges.len() > MAX_EDGES {
        return Err(Error::EdgeLimitExceeded);
    }
    edges.sort();
    let mut seen = BTreeSet::new();
    for edge in &edges {
        if edge.source_fact_digest.is_zero() {
            return Err(Error::EmptyFactDigest(edge.source_id.to_string()));
        }
        let identity = (
            edge.source_id.clone(),
            edge.relation_id.clone(),
            edge.target_id.clone(),
        );
        if !seen.insert(identity) {
            return Err(Error::DuplicateEdge(edge.source_id.to_string()));
        }
    }
    edges.retain(|edge| edge.live);

    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.knowledge.projection.v1");
    bytes.extend_from_slice(&generation.get().to_be_bytes());
    bytes.extend_from_slice(source_snapshot_digest.as_array());
    for edge in &edges {
        push_id(&mut bytes, &edge.source_id);
        push_id(&mut bytes, &edge.relation_id);
        push_id(&mut bytes, &edge.target_id);
        bytes.extend_from_slice(edge.source_fact_digest.as_array());
    }
    Ok(KnowledgeProjection {
        generation,
        source_snapshot_digest,
        edges,
        projection_digest: Digest32::of_bytes(&bytes),
        authority: AuthorityPosture::DENY_ALL,
    })
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    bytes.extend_from_slice(&u32::try_from(raw.len()).unwrap_or(u32::MAX).to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
""",
    "codex-rs/hepta-kg/src/lib_tests.rs": r"""use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn generation() -> Generation {
    let Ok(value) = Generation::new(1) else {
        panic!("test generation must be valid");
    };
    value
}

fn edge(source: &str, live: bool) -> KnowledgeEdge {
    KnowledgeEdge {
        source_id: id(source),
        relation_id: id("rel:knows"),
        target_id: id("target:1"),
        source_fact_digest: Digest32::of_bytes(source.as_bytes()),
        live,
    }
}

#[test]
fn rebuild_is_canonical_and_authority_free() {
    let snapshot = Digest32::of_bytes(b"snapshot");
    let left = rebuild(generation(), snapshot, vec![edge("source:b", true), edge("source:a", true)]);
    let right = rebuild(generation(), snapshot, vec![edge("source:a", true), edge("source:b", true)]);
    let (Ok(left), Ok(right)) = (left, right) else {
        panic!("projections must build");
    };
    assert_eq!(left, right);
    assert!(!left.authority.grants_any());
}

#[test]
fn deleted_edges_do_not_enter_projection() {
    let snapshot = Digest32::of_bytes(b"snapshot");
    let Ok(value) = rebuild(generation(), snapshot, vec![edge("source:a", false)]) else {
        panic!("projection must build");
    };
    assert!(value.edges.is_empty());
}

#[test]
fn duplicate_edges_are_rejected() {
    let snapshot = Digest32::of_bytes(b"snapshot");
    assert_eq!(
        rebuild(
            generation(),
            snapshot,
            vec![edge("source:a", true), edge("source:a", true)]
        ),
        Err(Error::DuplicateEdge("source:a".to_string()))
    );
}
""",
    "codex-rs/hepta-compact-engine/Cargo.toml": manifest(
        "codex-hepta-compact-engine",
        "codex_hepta_compact_engine",
        (("codex-hepta-cognitive-types", "hepta-cognitive-types"),),
    ),
    "codex-rs/hepta-compact-engine/BUILD.bazel": build("hepta-compact-engine"),
    "codex-rs/hepta-compact-engine/src/lib.rs": r"""//! Bounded cognitive compaction checkpoint builder.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt;

use codex_hepta_cognitive_types::MemoryRecord;
use codex_hepta_types::{AuthorityPosture, Digest32, Generation, StableId};

const MAX_INPUT_RECORDS: usize = 65_536;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompactCheckpoint {
    pub generation: Generation,
    pub source_snapshot_digest: Digest32,
    pub records: Vec<MemoryRecord>,
    pub checkpoint_digest: Digest32,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptySourceSnapshot,
    InputLimitExceeded,
    InvalidRecord(String),
    DuplicateRevision(String),
    BrokenLineage(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

pub fn compact(
    generation: Generation,
    source_snapshot_digest: Digest32,
    mut records: Vec<MemoryRecord>,
) -> Result<CompactCheckpoint, Error> {
    if source_snapshot_digest.is_zero() {
        return Err(Error::EmptySourceSnapshot);
    }
    if records.len() > MAX_INPUT_RECORDS {
        return Err(Error::InputLimitExceeded);
    }
    records.sort_by(|left, right| {
        left.record_id
            .cmp(&right.record_id)
            .then_with(|| left.revision.cmp(&right.revision))
    });

    let mut latest = BTreeMap::<StableId, MemoryRecord>::new();
    for record in records {
        record
            .validate()
            .map_err(|error| Error::InvalidRecord(error.to_string()))?;
        if let Some(previous) = latest.get(&record.record_id) {
            if record.revision == previous.revision {
                return Err(Error::DuplicateRevision(record.record_id.to_string()));
            }
            if record.revision.get() != previous.revision.get().saturating_add(1)
                || record.predecessor_digest != Some(previous.record_digest())
            {
                return Err(Error::BrokenLineage(record.record_id.to_string()));
            }
        } else if record.revision.get() != 1 {
            return Err(Error::BrokenLineage(record.record_id.to_string()));
        }
        latest.insert(record.record_id.clone(), record);
    }
    let compacted = latest.into_values().collect::<Vec<_>>();
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.compact.checkpoint.v1");
    bytes.extend_from_slice(&generation.get().to_be_bytes());
    bytes.extend_from_slice(source_snapshot_digest.as_array());
    for record in &compacted {
        bytes.extend_from_slice(record.record_digest().as_array());
    }
    Ok(CompactCheckpoint {
        generation,
        source_snapshot_digest,
        records: compacted,
        checkpoint_digest: Digest32::of_bytes(&bytes),
        authority: AuthorityPosture::DENY_ALL,
    })
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
""",
    "codex-rs/hepta-compact-engine/src/lib_tests.rs": r"""use super::*;
use codex_hepta_cognitive_types::{MemoryKind, RecordState};
use codex_hepta_types::Revision;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn revision(value: u64) -> Revision {
    let Ok(value) = Revision::new(value) else {
        panic!("test revision must be valid");
    };
    value
}

fn generation() -> Generation {
    let Ok(value) = Generation::new(1) else {
        panic!("test generation must be valid");
    };
    value
}

fn record(revision_value: u64, predecessor: Option<Digest32>, state: RecordState) -> MemoryRecord {
    MemoryRecord {
        record_id: id("memory:1"),
        revision: revision(revision_value),
        kind: MemoryKind::Fact,
        content_digest: Digest32::of_bytes(format!("content:{revision_value}").as_bytes()),
        predecessor_digest: predecessor,
        citations: Vec::new(),
        state,
    }
}

#[test]
fn latest_revision_and_tombstone_are_preserved() {
    let first = record(1, None, RecordState::Live);
    let second = record(2, Some(first.record_digest()), RecordState::Tombstone);
    let Ok(checkpoint) = compact(
        generation(),
        Digest32::of_bytes(b"snapshot"),
        vec![second.clone(), first],
    ) else {
        panic!("compaction must succeed");
    };
    assert_eq!(checkpoint.records, vec![second]);
    assert!(!checkpoint.authority.grants_any());
}

#[test]
fn broken_lineage_is_rejected() {
    let first = record(1, None, RecordState::Live);
    let second = record(
        2,
        Some(Digest32::of_bytes(b"wrong")),
        RecordState::Live,
    );
    assert_eq!(
        compact(
            generation(),
            Digest32::of_bytes(b"snapshot"),
            vec![first, second]
        ),
        Err(Error::BrokenLineage("memory:1".to_string()))
    );
}

#[test]
fn missing_initial_revision_is_rejected() {
    let second = record(
        2,
        Some(Digest32::of_bytes(b"prior")),
        RecordState::Live,
    );
    assert_eq!(
        compact(
            generation(),
            Digest32::of_bytes(b"snapshot"),
            vec![second]
        ),
        Err(Error::BrokenLineage("memory:1".to_string()))
    );
}
""",
    "codex-rs/hepta-context-compiler/Cargo.toml": manifest(
        "codex-hepta-context-compiler", "codex_hepta_context_compiler"
    ),
    "codex-rs/hepta-context-compiler/BUILD.bazel": build("hepta-context-compiler"),
    "codex-rs/hepta-context-compiler/src/lib.rs": r"""//! Source-aware context compiler.
//!
//! Trusted instructions and untrusted evidence remain separate sections. The
//! compiler accepts digests, never raw secrets, and cannot call a model.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::error::Error as StdError;
use std::fmt;

use codex_hepta_types::{AuthorityPosture, Digest32, StableId};

const MAX_ITEMS: usize = 4_096;
const MAX_TOKENS: u64 = 1_000_000;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ContextRole {
    TrustedInstruction,
    UntrustedEvidence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextItem {
    pub item_id: StableId,
    pub role: ContextRole,
    pub content_digest: Digest32,
    pub source_digest: Digest32,
    pub token_count: u64,
    pub contains_secret: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilationRequest {
    pub compilation_id: StableId,
    pub run_snapshot_digest: Digest32,
    pub objective_digest: Digest32,
    pub token_budget: u64,
    pub items: Vec<ContextItem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextCompilationReceipt {
    pub compilation_id: StableId,
    pub trusted_instruction_ids: Vec<StableId>,
    pub untrusted_evidence_ids: Vec<StableId>,
    pub omitted_ids: Vec<StableId>,
    pub used_tokens: u64,
    pub context_digest: Digest32,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    ItemLimitExceeded,
    InvalidTokenBudget,
    EmptyDigest(&'static str),
    DuplicateItem(String),
    ZeroTokenItem(String),
    SecretRejected(String),
    Arithmetic,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

pub fn compile(mut request: CompilationRequest) -> Result<ContextCompilationReceipt, Error> {
    if request.items.len() > MAX_ITEMS {
        return Err(Error::ItemLimitExceeded);
    }
    if request.token_budget == 0 || request.token_budget > MAX_TOKENS {
        return Err(Error::InvalidTokenBudget);
    }
    if request.run_snapshot_digest.is_zero() {
        return Err(Error::EmptyDigest("run snapshot"));
    }
    if request.objective_digest.is_zero() {
        return Err(Error::EmptyDigest("objective"));
    }
    request.items.sort_by(|left, right| {
        left.role
            .cmp(&right.role)
            .then_with(|| left.item_id.cmp(&right.item_id))
    });
    let mut seen = BTreeSet::new();
    let mut trusted = Vec::new();
    let mut evidence = Vec::new();
    let mut omitted = Vec::new();
    let mut used_tokens = 0_u64;
    let mut included = Vec::new();

    for item in request.items {
        if !seen.insert(item.item_id.clone()) {
            return Err(Error::DuplicateItem(item.item_id.to_string()));
        }
        if item.content_digest.is_zero() || item.source_digest.is_zero() {
            return Err(Error::EmptyDigest("context item"));
        }
        if item.token_count == 0 {
            return Err(Error::ZeroTokenItem(item.item_id.to_string()));
        }
        if item.contains_secret {
            return Err(Error::SecretRejected(item.item_id.to_string()));
        }
        let Some(next_total) = used_tokens.checked_add(item.token_count) else {
            return Err(Error::Arithmetic);
        };
        if next_total > request.token_budget {
            omitted.push(item.item_id);
            continue;
        }
        used_tokens = next_total;
        match item.role {
            ContextRole::TrustedInstruction => trusted.push(item.item_id.clone()),
            ContextRole::UntrustedEvidence => evidence.push(item.item_id.clone()),
        }
        included.push(item);
    }

    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.context.compilation.v1");
    push_id(&mut bytes, &request.compilation_id);
    bytes.extend_from_slice(request.run_snapshot_digest.as_array());
    bytes.extend_from_slice(request.objective_digest.as_array());
    bytes.extend_from_slice(&request.token_budget.to_be_bytes());
    for item in included {
        bytes.push(match item.role {
            ContextRole::TrustedInstruction => 0,
            ContextRole::UntrustedEvidence => 1,
        });
        push_id(&mut bytes, &item.item_id);
        bytes.extend_from_slice(item.content_digest.as_array());
        bytes.extend_from_slice(item.source_digest.as_array());
        bytes.extend_from_slice(&item.token_count.to_be_bytes());
    }

    Ok(ContextCompilationReceipt {
        compilation_id: request.compilation_id,
        trusted_instruction_ids: trusted,
        untrusted_evidence_ids: evidence,
        omitted_ids: omitted,
        used_tokens,
        context_digest: Digest32::of_bytes(&bytes),
        authority: AuthorityPosture::DENY_ALL,
    })
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    bytes.extend_from_slice(&u32::try_from(raw.len()).unwrap_or(u32::MAX).to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
""",
    "codex-rs/hepta-context-compiler/src/lib_tests.rs": r"""use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn item(name: &str, role: ContextRole, tokens: u64) -> ContextItem {
    ContextItem {
        item_id: id(name),
        role,
        content_digest: Digest32::of_bytes(name.as_bytes()),
        source_digest: Digest32::of_bytes(b"source"),
        token_count: tokens,
        contains_secret: false,
    }
}

fn request(items: Vec<ContextItem>) -> CompilationRequest {
    CompilationRequest {
        compilation_id: id("compile:1"),
        run_snapshot_digest: Digest32::of_bytes(b"snapshot"),
        objective_digest: Digest32::of_bytes(b"objective"),
        token_budget: 10,
        items,
    }
}

#[test]
fn evidence_never_becomes_instruction() {
    let Ok(receipt) = compile(request(vec![
        item("evidence:1", ContextRole::UntrustedEvidence, 2),
        item("instruction:1", ContextRole::TrustedInstruction, 2),
    ])) else {
        panic!("compilation must succeed");
    };
    assert_eq!(
        receipt.trusted_instruction_ids,
        vec![id("instruction:1")]
    );
    assert_eq!(
        receipt.untrusted_evidence_ids,
        vec![id("evidence:1")]
    );
    assert!(!receipt.authority.grants_any());
}

#[test]
fn secret_bearing_item_is_rejected() {
    let mut value = item("secret:1", ContextRole::UntrustedEvidence, 1);
    value.contains_secret = true;
    assert_eq!(
        compile(request(vec![value])),
        Err(Error::SecretRejected("secret:1".to_string()))
    );
}

#[test]
fn token_budget_omission_is_explicit() {
    let Ok(receipt) = compile(request(vec![
        item("instruction:a", ContextRole::TrustedInstruction, 8),
        item("instruction:b", ContextRole::TrustedInstruction, 8),
    ])) else {
        panic!("compilation must succeed");
    };
    assert_eq!(receipt.used_tokens, 8);
    assert_eq!(receipt.omitted_ids, vec![id("instruction:b")]);
}

#[test]
fn duplicate_item_is_rejected() {
    let value = item("item:1", ContextRole::UntrustedEvidence, 1);
    assert_eq!(
        compile(request(vec![value.clone(), value])),
        Err(Error::DuplicateItem("item:1".to_string()))
    );
}
""",
}

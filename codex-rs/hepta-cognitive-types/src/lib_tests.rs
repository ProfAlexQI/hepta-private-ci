use super::*;

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

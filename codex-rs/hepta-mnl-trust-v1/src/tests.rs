use serde::Serialize;
use sha2::Digest;
use static_assertions::assert_not_impl_any;

use super::*;

const ANCHOR_RAW_COMMIT: &[u8] = b"tree 40ce734411c96a6ce00ba24e360fb4fdab9c1f88\nparent 59ca7c7caba34068db1c26a97e67a6e949be4711\nparent a72d58d95e7bb081728eea7b395e4a3a75a0aed5\nauthor Qian Qi <qianqi@MacBook-Pro.local> 1786972505 +0800\ncommitter Qian Qi <qianqi@MacBook-Pro.local> 1786972505 +0800\n\nMerge Linux MNL lint closure\n";
const CAPABILITY_LEDGER_CANONICAL_SHA256: &str =
    "b1b23775bd0088306d9e1913e34418fc8e7d6f2b8e2e7f48cbf8266d96f13dee";
const ANCESTRY_MANIFEST_CANONICAL_SHA256: &str =
    "6e9e18b5491bd4ca7ba251a558cbf7501a23b1bceded2926435ac1f90fc70b42";
const EXACT_CAPABILITY_WIRE_IDS: [&str; 8] = [
    "ExternalRoleSeparatedSignatures",
    "FinalToolingAncestry",
    "ExactSourceAndRoleBinaryProvenance",
    "DurableAtomicOneShotReplay",
    "PreRunWallClockSupervisor",
    "LiveReadOnlyCollectorAndClosedRunner",
    "IndependentBundleCopyReadbackAndAckSigner",
    "ImmutablePreRunAndPostRunPublication",
];

assert_not_impl_any!(StructuralAncestryInspectionV1: Clone, Serialize, serde::de::DeserializeOwned);
assert_not_impl_any!(RawGitCommitSidecarV1<'static>: Clone, Serialize, serde::de::DeserializeOwned);
assert_not_impl_any!(AbsentCapabilityLedgerInspectionV1: Clone, Serialize, serde::de::DeserializeOwned);

#[test]
fn exact_capability_ledger_is_complete_absent_and_non_authorizing() {
    let ledger = exact_phase_a_capability_ledger();
    assert_eq!(ledger.schema, CAPABILITY_LEDGER_SCHEMA);
    assert_eq!(ledger.anchor, exact_phase_a_anchor());
    assert_eq!(ledger.entries.len(), 8);
    assert!(ledger.entries.iter().all(|entry| {
        entry.state == CapabilityStateV1::Absent
            && entry.authority == AuthorityDispositionV1::InspectionOnlyNoAuthority
            && entry.evidence_origin == EvidenceOriginV1::FutureExternalFrozenOnly
    }));
    let unique = ledger
        .entries
        .iter()
        .map(|entry| entry.capability)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(unique.len(), ledger.entries.len());
    validate_phase_a_capability_ledger(&ledger).expect("exact Phase-A ledger");

    let canonical = exact_phase_a_capability_ledger_bytes().expect("canonical ledger");
    let canonical_text = std::str::from_utf8(&canonical).expect("canonical ledger UTF-8");
    assert!(
        EXACT_CAPABILITY_WIRE_IDS
            .iter()
            .all(|capability| canonical_text.contains(capability))
    );
    assert_eq!(sha256(&canonical), CAPABILITY_LEDGER_CANONICAL_SHA256);
    let inspected = inspect_canonical_phase_a_capability_ledger(&canonical)
        .expect("inspect exact canonical ledger");
    assert_eq!(inspected.entry_count(), 8);
    assert_eq!(inspected.canonical_sha256(), sha256(&canonical));
    assert!(!inspected.authorizes_live());
}

#[test]
fn capability_ledger_rejects_removal_reorder_and_noncanonical_authority_injection() {
    let mut removed = exact_phase_a_capability_ledger();
    removed.entries.pop();
    assert!(validate_phase_a_capability_ledger(&removed).is_err());

    let mut reordered = exact_phase_a_capability_ledger();
    reordered.entries.swap(0, 1);
    assert!(validate_phase_a_capability_ledger(&reordered).is_err());

    let canonical = exact_phase_a_capability_ledger_bytes().expect("canonical ledger");
    let mut padded = canonical.clone();
    padded.push(b'\n');
    assert!(inspect_canonical_phase_a_capability_ledger(&padded).is_err());

    let mut value: serde_json::Value =
        serde_json::from_slice(&canonical).expect("ledger JSON value");
    value["entries"][0]["authorizes_live"] = serde_json::json!(true);
    assert!(
        inspect_canonical_phase_a_capability_ledger(&serde_json::to_vec(&value).unwrap()).is_err()
    );

    let mut value: serde_json::Value =
        serde_json::from_slice(&canonical).expect("ledger JSON value");
    value["entries"][0]["state"] = serde_json::json!("present");
    assert!(
        inspect_canonical_phase_a_capability_ledger(&serde_json::to_vec(&value).unwrap()).is_err()
    );
}

#[test]
fn exact_anchor_raw_object_matches_all_compiled_identities() {
    assert_eq!(
        ANCHOR_RAW_COMMIT.len() as u64,
        PHASE_A_ANCHOR_COMMIT_RAW_BYTES
    );
    assert_eq!(sha256(ANCHOR_RAW_COMMIT), PHASE_A_ANCHOR_COMMIT_RAW_SHA256);
    let entry = commit_manifest_entry(ANCHOR_RAW_COMMIT);
    assert_eq!(entry.oid_sha1, PHASE_A_ANCHOR_HEAD);
    assert_eq!(entry.tree_sha1, PHASE_A_ANCHOR_TREE);
    assert_eq!(
        entry.parent_oids_sha1,
        [
            "59ca7c7caba34068db1c26a97e67a6e949be4711",
            "a72d58d95e7bb081728eea7b395e4a3a75a0aed5",
        ]
    );
}

#[test]
fn canonical_child_to_anchor_path_is_structural_only() {
    let fixture = valid_fixture();
    let inspected = inspect_fixture(&fixture).expect("structural ancestry");
    assert_eq!(inspected.anchor(), &exact_phase_a_anchor());
    assert_eq!(inspected.final_tooling(), &fixture.final_identity);
    assert_eq!(inspected.commit_count(), 2);
    assert_eq!(
        inspected.manifest_sha256(),
        ANCESTRY_MANIFEST_CANONICAL_SHA256
    );
    assert_eq!(
        inspected.raw_objects_sha256(),
        "b66995388e3461d387832c025402ec6f107299dcc45d0d47ff8b29121e52a1d5"
    );
    assert!(!inspected.authorizes_live());
}

#[test]
fn canonical_three_commit_path_accepts_merge_with_exact_ordered_parents() {
    let intermediate_raw = synthetic_commit(&"9".repeat(40), PHASE_A_ANCHOR_HEAD, &[]);
    let intermediate = commit_manifest_entry(&intermediate_raw);
    let child_raw = synthetic_commit(&"8".repeat(40), &intermediate.oid_sha1, &["7".repeat(40)]);
    let child = commit_manifest_entry(&child_raw);
    let final_identity = RepositoryIdentityV1 {
        head: child.oid_sha1.clone(),
        tree: child.tree_sha1.clone(),
    };
    let manifest = manifest(
        final_identity,
        vec![
            child,
            intermediate,
            commit_manifest_entry(ANCHOR_RAW_COMMIT),
        ],
    );
    let canonical = serde_json::to_vec(&manifest).expect("canonical manifest");
    let sidecars = [
        RawGitCommitSidecarV1::new(&manifest.commits[0].oid_sha1, &child_raw),
        RawGitCommitSidecarV1::new(&manifest.commits[1].oid_sha1, &intermediate_raw),
        RawGitCommitSidecarV1::new(&manifest.commits[2].oid_sha1, ANCHOR_RAW_COMMIT),
    ];
    inspect_canonical_ancestry_path(&canonical, &sidecars).expect("three-commit merge path");
}

#[test]
fn ancestry_rejects_unknown_noncanonical_wrong_anchor_and_final_identity() {
    let fixture = valid_fixture();
    let mut padded = fixture.canonical.clone();
    padded.push(b'\n');
    assert!(inspect_with_fixture_bytes(&fixture, &padded).is_err());

    let pretty = serde_json::to_vec_pretty(&fixture.manifest).expect("pretty manifest");
    assert!(inspect_with_fixture_bytes(&fixture, &pretty).is_err());

    let reordered = reordered_manifest_bytes(&fixture.manifest);
    assert!(inspect_with_fixture_bytes(&fixture, &reordered).is_err());

    let canonical_text = std::str::from_utf8(&fixture.canonical).expect("manifest UTF-8");
    let escaped = canonical_text.replacen("hepta", "\\u0068epta", 1);
    assert!(inspect_with_fixture_bytes(&fixture, escaped.as_bytes()).is_err());

    let duplicate_schema = format!(
        "{{\"schema\":\"{ANCESTRY_PATH_PROOF_SCHEMA}\",{}",
        &canonical_text[1..]
    );
    assert!(inspect_with_fixture_bytes(&fixture, duplicate_schema.as_bytes()).is_err());

    let mut value = serde_json::to_value(&fixture.manifest).expect("manifest value");
    value["authority"] = serde_json::json!(true);
    let unknown = serde_json::to_vec(&value).expect("unknown field manifest");
    assert!(inspect_with_fixture_bytes(&fixture, &unknown).is_err());

    let mut wrong_anchor = fixture.manifest.clone();
    wrong_anchor.anchor.head = "1".repeat(40);
    rejected_manifest(&fixture, &wrong_anchor);

    let mut wrong_final = fixture.manifest.clone();
    wrong_final.final_tooling.tree = "2".repeat(40);
    rejected_manifest(&fixture, &wrong_final);
}

#[test]
fn ancestry_rejects_raw_count_hash_oid_tree_and_ordered_parent_drift() {
    let fixture = valid_fixture();

    let mut wrong_count = fixture.manifest.clone();
    wrong_count.commits[0].raw_byte_count += 1;
    rejected_manifest(&fixture, &wrong_count);

    let mut wrong_raw_sha = fixture.manifest.clone();
    wrong_raw_sha.commits[0].raw_commit_sha256 = "4".repeat(64);
    rejected_manifest(&fixture, &wrong_raw_sha);

    let mut wrong_oid = fixture.manifest.clone();
    wrong_oid.commits[0].oid_sha1 = "3".repeat(40);
    let wrong_oid_sidecars = [
        RawGitCommitSidecarV1::new(&wrong_oid.commits[0].oid_sha1, &fixture.child_raw),
        RawGitCommitSidecarV1::new(PHASE_A_ANCHOR_HEAD, ANCHOR_RAW_COMMIT),
    ];
    assert!(
        rejected_manifest_message(&wrong_oid, &wrong_oid_sidecars)
            .contains("raw Git commit oid does not match")
    );

    let mut wrong_tree = fixture.manifest.clone();
    wrong_tree.commits[0].tree_sha1 = "5".repeat(40);
    rejected_manifest(&fixture, &wrong_tree);

    let mut reordered_anchor_parents = fixture.manifest.clone();
    reordered_anchor_parents.commits[1]
        .parent_oids_sha1
        .swap(0, 1);
    rejected_manifest(&fixture, &reordered_anchor_parents);

    let mut omitted_parent = fixture.manifest.clone();
    omitted_parent.commits[1].parent_oids_sha1.pop();
    rejected_manifest(&fixture, &omitted_parent);

    let mut added_parent = fixture.manifest.clone();
    added_parent.commits[0]
        .parent_oids_sha1
        .push("6".repeat(40));
    rejected_manifest(&fixture, &added_parent);

    let mut uppercase_oid = fixture.manifest.clone();
    uppercase_oid.commits[0].oid_sha1.make_ascii_uppercase();
    rejected_manifest(&fixture, &uppercase_oid);

    let mut zero_oid = fixture.manifest.clone();
    zero_oid.commits[0].oid_sha1 = "0".repeat(40);
    rejected_manifest(&fixture, &zero_oid);

    let mut too_many_parents = fixture.manifest.clone();
    too_many_parents.commits[0].parent_oids_sha1 =
        (0..=MAX_COMMIT_PARENTS).map(valid_index_oid).collect();
    assert!(
        rejected_manifest_message(&too_many_parents, &fixture.sidecars())
            .contains("parent count exceeds")
    );
}

#[test]
fn ancestry_rejects_second_parent_gap_duplicate_anchor_drift_and_sidecar_mismatch() {
    let fixture = valid_fixture();

    let second_parent_raw = synthetic_commit(
        &fixture.final_identity.tree,
        &"6".repeat(40),
        &[PHASE_A_ANCHOR_HEAD.to_string()],
    );
    let second_parent_child = commit_manifest_entry(&second_parent_raw);
    let second_parent_manifest = manifest(
        RepositoryIdentityV1 {
            head: second_parent_child.oid_sha1.clone(),
            tree: second_parent_child.tree_sha1.clone(),
        },
        vec![
            second_parent_child,
            commit_manifest_entry(ANCHOR_RAW_COMMIT),
        ],
    );
    let second_parent_canonical =
        serde_json::to_vec(&second_parent_manifest).expect("canonical manifest");
    let second_parent_sidecars = [
        RawGitCommitSidecarV1::new(
            &second_parent_manifest.commits[0].oid_sha1,
            &second_parent_raw,
        ),
        RawGitCommitSidecarV1::new(
            &second_parent_manifest.commits[1].oid_sha1,
            ANCHOR_RAW_COMMIT,
        ),
    ];
    assert!(
        inspection_error_message(inspect_canonical_ancestry_path(
            &second_parent_canonical,
            &second_parent_sidecars,
        ))
        .contains("first-parent path")
    );

    let mut duplicate = fixture.manifest.clone();
    duplicate.commits.insert(1, duplicate.commits[0].clone());
    let duplicate_sidecars = [
        RawGitCommitSidecarV1::new(&duplicate.commits[0].oid_sha1, &fixture.child_raw),
        RawGitCommitSidecarV1::new(&duplicate.commits[1].oid_sha1, &fixture.child_raw),
        RawGitCommitSidecarV1::new(PHASE_A_ANCHOR_HEAD, ANCHOR_RAW_COMMIT),
    ];
    assert!(
        rejected_manifest_message(&duplicate, &duplicate_sidecars).contains("repeats a commit oid")
    );

    let mut anchor_drift = fixture.manifest.clone();
    anchor_drift.commits[1].raw_commit_sha256 = "7".repeat(64);
    rejected_manifest(&fixture, &anchor_drift);

    let sidecars = fixture.sidecars();
    assert!(
        inspect_canonical_ancestry_path(&fixture.canonical, &sidecars[..1]).is_err(),
        "missing sidecar must fail"
    );
    let extra_raw = synthetic_commit(&"a".repeat(40), PHASE_A_ANCHOR_HEAD, &[]);
    let extra_oid = git_oid(&extra_raw);
    let extra = RawGitCommitSidecarV1::new(&extra_oid, &extra_raw);
    let mut extra_sidecars = fixture.sidecars();
    extra_sidecars.push(extra);
    assert!(inspect_canonical_ancestry_path(&fixture.canonical, &extra_sidecars).is_err());

    let swapped_sidecars = [
        RawGitCommitSidecarV1::new(PHASE_A_ANCHOR_HEAD, ANCHOR_RAW_COMMIT),
        RawGitCommitSidecarV1::new(&fixture.manifest.commits[0].oid_sha1, &fixture.child_raw),
    ];
    assert!(inspect_canonical_ancestry_path(&fixture.canonical, &swapped_sidecars).is_err());
}

#[test]
fn ancestry_rejects_anchor_as_final_missing_malformed_and_oversized_material() {
    let fixture = valid_fixture();

    let mut anchor_final = fixture.manifest.clone();
    anchor_final.final_tooling = exact_phase_a_anchor();
    anchor_final.commits.remove(0);
    rejected_manifest(&fixture, &anchor_final);

    let mut missing = fixture.manifest.clone();
    missing.commits.clear();
    rejected_manifest(&fixture, &missing);

    let mut too_many = fixture.manifest.clone();
    too_many.commits = vec![fixture.manifest.commits[0].clone(); MAX_ANCESTRY_COMMITS + 1];
    rejected_manifest(&fixture, &too_many);

    let oversized = vec![b'x'; MAX_ANCESTRY_MANIFEST_BYTES + 1];
    assert!(inspect_with_fixture_bytes(&fixture, &oversized).is_err());

    let malformed = b"tree not-an-oid\n\nmessage\n";
    let malformed_oid = git_oid(malformed);
    let mut malformed_manifest = fixture.manifest.clone();
    malformed_manifest.commits[0].oid_sha1 = malformed_oid.clone();
    malformed_manifest.commits[0].raw_byte_count = malformed.len() as u64;
    malformed_manifest.commits[0].raw_commit_sha256 = sha256(malformed);
    malformed_manifest.final_tooling.head = malformed_oid.clone();
    let malformed_sidecars = [
        RawGitCommitSidecarV1::new(&malformed_oid, malformed),
        RawGitCommitSidecarV1::new(PHASE_A_ANCHOR_HEAD, ANCHOR_RAW_COMMIT),
    ];
    assert!(
        rejected_manifest_message(&malformed_manifest, &malformed_sidecars)
            .contains("cannot be parsed")
    );

    let huge_raw = vec![b'x'; MAX_RAW_COMMIT_BYTES + 1];
    let huge_sidecars = [
        RawGitCommitSidecarV1::new(&fixture.manifest.commits[0].oid_sha1, &huge_raw),
        RawGitCommitSidecarV1::new(PHASE_A_ANCHOR_HEAD, ANCHOR_RAW_COMMIT),
    ];
    assert!(
        inspection_error_message(inspect_canonical_ancestry_path(
            &fixture.canonical,
            &huge_sidecars,
        ))
        .contains("raw commit byte length")
    );

    let empty_sidecars = [
        RawGitCommitSidecarV1::new(&fixture.manifest.commits[0].oid_sha1, b""),
        RawGitCommitSidecarV1::new(PHASE_A_ANCHOR_HEAD, ANCHOR_RAW_COMMIT),
    ];
    assert!(inspect_canonical_ancestry_path(&fixture.canonical, &empty_sidecars).is_err());
}

struct Fixture {
    canonical: Vec<u8>,
    child_raw: Vec<u8>,
    final_identity: RepositoryIdentityV1,
    manifest: GitAncestryPathManifestV1,
}

impl Fixture {
    fn sidecars(&self) -> Vec<RawGitCommitSidecarV1<'_>> {
        vec![
            RawGitCommitSidecarV1::new(&self.manifest.commits[0].oid_sha1, &self.child_raw),
            RawGitCommitSidecarV1::new(&self.manifest.commits[1].oid_sha1, ANCHOR_RAW_COMMIT),
        ]
    }
}

fn valid_fixture() -> Fixture {
    let final_tree = "8".repeat(40);
    let child_raw = synthetic_commit(&final_tree, PHASE_A_ANCHOR_HEAD, &[]);
    let child = commit_manifest_entry(&child_raw);
    let final_identity = RepositoryIdentityV1 {
        head: child.oid_sha1.clone(),
        tree: final_tree,
    };
    let manifest = manifest(
        final_identity.clone(),
        vec![child, commit_manifest_entry(ANCHOR_RAW_COMMIT)],
    );
    let canonical = serde_json::to_vec(&manifest).expect("canonical manifest");
    Fixture {
        canonical,
        child_raw,
        final_identity,
        manifest,
    }
}

fn manifest(
    final_tooling: RepositoryIdentityV1,
    commits: Vec<GitCommitManifestEntryV1>,
) -> GitAncestryPathManifestV1 {
    GitAncestryPathManifestV1 {
        anchor: exact_phase_a_anchor(),
        commits,
        final_tooling,
        path_policy: GitAncestryPathPolicyV1::FinalToAnchorFirstParent,
        schema: ANCESTRY_PATH_PROOF_SCHEMA.to_string(),
    }
}

fn synthetic_commit(tree: &str, first_parent: &str, remaining_parents: &[String]) -> Vec<u8> {
    let mut raw = format!("tree {tree}\nparent {first_parent}\n");
    for parent in remaining_parents {
        raw.push_str(&format!("parent {parent}\n"));
    }
    raw.push_str(
        "author Test <test@example.invalid> 1 +0000\ncommitter Test <test@example.invalid> 1 +0000\n\nSynthetic descendant\n",
    );
    raw.into_bytes()
}

fn commit_manifest_entry(raw: &[u8]) -> GitCommitManifestEntryV1 {
    let parsed = gix_object::CommitRef::from_bytes(raw).expect("valid test Git commit");
    GitCommitManifestEntryV1 {
        oid_sha1: git_oid(raw),
        parent_oids_sha1: parsed.parents().map(|parent| parent.to_string()).collect(),
        raw_byte_count: raw.len() as u64,
        raw_commit_sha256: sha256(raw),
        tree_sha1: parsed.tree().to_string(),
    }
}

fn git_oid(raw: &[u8]) -> String {
    gix_object::compute_hash(gix_hash::Kind::Sha1, gix_object::Kind::Commit, raw)
        .expect("hash valid test Git commit")
        .to_string()
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

fn inspect_fixture(fixture: &Fixture) -> Result<StructuralAncestryInspectionV1, MnlTrustError> {
    inspect_canonical_ancestry_path(&fixture.canonical, &fixture.sidecars())
}

fn inspect_with_fixture_bytes(
    fixture: &Fixture,
    canonical: &[u8],
) -> Result<StructuralAncestryInspectionV1, MnlTrustError> {
    inspect_canonical_ancestry_path(canonical, &fixture.sidecars())
}

fn rejected_manifest(fixture: &Fixture, manifest: &GitAncestryPathManifestV1) {
    let canonical = serde_json::to_vec(manifest).expect("serialize rejected manifest");
    assert!(inspect_canonical_ancestry_path(&canonical, &fixture.sidecars()).is_err());
}

fn rejected_manifest_message(
    manifest: &GitAncestryPathManifestV1,
    sidecars: &[RawGitCommitSidecarV1<'_>],
) -> String {
    let canonical = serde_json::to_vec(manifest).expect("serialize rejected manifest");
    inspection_error_message(inspect_canonical_ancestry_path(&canonical, sidecars))
}

fn inspection_error_message(
    result: Result<StructuralAncestryInspectionV1, MnlTrustError>,
) -> String {
    match result {
        Ok(_) => panic!("invalid ancestry material was accepted"),
        Err(error) => error.to_string(),
    }
}

fn reordered_manifest_bytes(manifest: &GitAncestryPathManifestV1) -> Vec<u8> {
    format!(
        "{{\"schema\":{},\"path_policy\":{},\"final_tooling\":{},\"commits\":{},\"anchor\":{}}}",
        serde_json::to_string(&manifest.schema).expect("schema JSON"),
        serde_json::to_string(&manifest.path_policy).expect("path policy JSON"),
        serde_json::to_string(&manifest.final_tooling).expect("final tooling JSON"),
        serde_json::to_string(&manifest.commits).expect("commits JSON"),
        serde_json::to_string(&manifest.anchor).expect("anchor JSON"),
    )
    .into_bytes()
}

fn valid_index_oid(index: usize) -> String {
    format!("{:040x}", index + 1)
}

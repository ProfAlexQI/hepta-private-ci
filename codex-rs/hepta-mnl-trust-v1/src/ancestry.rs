use std::collections::HashSet;

use gix_hash::Kind as HashKind;
use gix_object::Kind as ObjectKind;
use sha2::Digest;

use crate::GitAncestryPathManifestV1;
use crate::GitAncestryPathPolicyV1;
use crate::MnlTrustError;
use crate::RawGitCommitSidecarV1;
use crate::RepositoryIdentityV1;
use crate::StructuralAncestryInspectionV1;
use crate::invalid;
use crate::model::StructuralInspectionSealV1;

pub const ANCESTRY_PATH_PROOF_SCHEMA: &str = "hepta_mnl_external_git_ancestry_path_manifest_v1";
pub const PHASE_A_ANCHOR_HEAD: &str = "ef1cf8c1d34f34dfc161e8cbeb5da67481f44763";
pub const PHASE_A_ANCHOR_TREE: &str = "40ce734411c96a6ce00ba24e360fb4fdab9c1f88";
pub const PHASE_A_ANCHOR_COMMIT_RAW_SHA256: &str =
    "b2624fde619d21a8a8986d7b3152d0f89fda9edfbf86694378f85d41872368a3";
pub const PHASE_A_ANCHOR_COMMIT_RAW_BYTES: u64 = 293;
pub const MAX_ANCESTRY_COMMITS: usize = 256;
pub const MAX_ANCESTRY_MANIFEST_BYTES: usize = 1024 * 1024;
pub const MAX_COMMIT_PARENTS: usize = 64;
pub const MAX_RAW_COMMIT_BYTES: usize = 64 * 1024;
pub const MAX_TOTAL_RAW_COMMIT_BYTES: usize = 16 * 1024 * 1024;

const RAW_OBJECT_SET_DIGEST_DOMAIN: &[u8] = b"hepta_mnl_git_commit_sidecars_v1\0";

pub fn exact_phase_a_anchor() -> RepositoryIdentityV1 {
    RepositoryIdentityV1 {
        head: PHASE_A_ANCHOR_HEAD.to_string(),
        tree: PHASE_A_ANCHOR_TREE.to_string(),
    }
}

pub fn inspect_canonical_ancestry_path(
    canonical_manifest: &[u8],
    raw_commits: &[RawGitCommitSidecarV1<'_>],
) -> Result<StructuralAncestryInspectionV1, MnlTrustError> {
    if canonical_manifest.is_empty() || canonical_manifest.len() > MAX_ANCESTRY_MANIFEST_BYTES {
        return Err(invalid(
            "ancestry manifest byte length is outside its bound",
        ));
    }
    let manifest: GitAncestryPathManifestV1 = serde_json::from_slice(canonical_manifest)
        .map_err(|error| MnlTrustError::Serialization(error.to_string()))?;
    let reencoded = serde_json::to_vec(&manifest)
        .map_err(|error| MnlTrustError::Serialization(error.to_string()))?;
    if reencoded != canonical_manifest {
        return Err(invalid("ancestry manifest is not exact canonical JSON"));
    }
    inspect_manifest(manifest, canonical_manifest, raw_commits)
}

fn inspect_manifest(
    manifest: GitAncestryPathManifestV1,
    canonical_manifest: &[u8],
    raw_commits: &[RawGitCommitSidecarV1<'_>],
) -> Result<StructuralAncestryInspectionV1, MnlTrustError> {
    if manifest.schema != ANCESTRY_PATH_PROOF_SCHEMA {
        return Err(invalid("ancestry manifest schema is not exact"));
    }
    if manifest.path_policy != GitAncestryPathPolicyV1::FinalToAnchorFirstParent {
        return Err(invalid("ancestry manifest path policy is not exact"));
    }
    if manifest.anchor != exact_phase_a_anchor() {
        return Err(invalid(
            "ancestry manifest anchor is not the integrated Phase-A root",
        ));
    }
    validate_repository_identity(&manifest.final_tooling, "final tooling")?;
    if manifest.final_tooling == manifest.anchor {
        return Err(invalid(
            "final tooling must be a strict descendant of Phase A",
        ));
    }
    if !(2..=MAX_ANCESTRY_COMMITS).contains(&manifest.commits.len()) {
        return Err(invalid("ancestry path commit count is outside its bound"));
    }
    if raw_commits.len() != manifest.commits.len() {
        return Err(invalid(
            "raw commit sidecars are not one-to-one with the manifest",
        ));
    }

    let mut seen_oids = HashSet::with_capacity(manifest.commits.len());
    let mut seen_raw_sha256 = HashSet::with_capacity(manifest.commits.len());
    let mut total_raw_bytes = 0usize;

    for (entry, sidecar) in manifest.commits.iter().zip(raw_commits) {
        validate_oid(&entry.oid_sha1, "commit oid")?;
        validate_oid(&entry.tree_sha1, "commit tree")?;
        validate_sha256(&entry.raw_commit_sha256, "raw commit sha256")?;
        if entry.parent_oids_sha1.len() > MAX_COMMIT_PARENTS {
            return Err(invalid("commit parent count exceeds its bound"));
        }
        for parent in &entry.parent_oids_sha1 {
            validate_oid(parent, "commit parent")?;
        }
        if entry.parent_oids_sha1.iter().collect::<HashSet<_>>().len()
            != entry.parent_oids_sha1.len()
        {
            return Err(invalid("commit manifest entry repeats a parent"));
        }
        if !seen_oids.insert(entry.oid_sha1.as_str()) {
            return Err(invalid("ancestry path repeats a commit oid"));
        }
        if !seen_raw_sha256.insert(entry.raw_commit_sha256.as_str()) {
            return Err(invalid("ancestry path repeats raw commit bytes"));
        }
        if sidecar.oid_sha1 != entry.oid_sha1 {
            return Err(invalid("raw commit sidecar order or oid label differs"));
        }
        if sidecar.raw_commit.is_empty() || sidecar.raw_commit.len() > MAX_RAW_COMMIT_BYTES {
            return Err(invalid("raw commit byte length is outside its bound"));
        }
        let raw_byte_count = u64::try_from(sidecar.raw_commit.len())
            .map_err(|_| invalid("raw commit byte length is not representable"))?;
        if entry.raw_byte_count != raw_byte_count {
            return Err(invalid("raw commit byte count does not match its sidecar"));
        }
        total_raw_bytes = total_raw_bytes
            .checked_add(sidecar.raw_commit.len())
            .ok_or_else(|| invalid("total raw commit byte count overflowed"))?;
        if total_raw_bytes > MAX_TOTAL_RAW_COMMIT_BYTES {
            return Err(invalid("total raw commit byte count exceeds its bound"));
        }
        if sha256_hex(sidecar.raw_commit) != entry.raw_commit_sha256 {
            return Err(invalid("raw commit SHA-256 does not match its sidecar"));
        }

        let parsed = gix_object::CommitRef::from_bytes(sidecar.raw_commit)
            .map_err(|error| invalid(format!("raw Git commit cannot be parsed: {error}")))?;
        let computed_oid =
            gix_object::compute_hash(HashKind::Sha1, ObjectKind::Commit, sidecar.raw_commit)
                .map_err(|error| invalid(format!("raw Git commit cannot be hashed: {error}")))?
                .to_string();
        if computed_oid != entry.oid_sha1 {
            return Err(invalid("raw Git commit oid does not match its manifest"));
        }
        let parsed_tree = parsed.tree().to_string();
        if parsed_tree != entry.tree_sha1 {
            return Err(invalid("raw Git commit tree does not match its manifest"));
        }
        let parsed_parents = parsed
            .parents()
            .map(|parent| parent.to_string())
            .collect::<Vec<_>>();
        if parsed_parents != entry.parent_oids_sha1 {
            return Err(invalid(
                "raw Git commit ordered parents do not match its manifest",
            ));
        }
    }

    let first = manifest
        .commits
        .first()
        .ok_or_else(|| invalid("ancestry path unexpectedly became empty"))?;
    if first.oid_sha1 != manifest.final_tooling.head
        || first.tree_sha1 != manifest.final_tooling.tree
    {
        return Err(invalid(
            "ancestry path does not begin at the declared final tooling",
        ));
    }
    let last = manifest
        .commits
        .last()
        .ok_or_else(|| invalid("ancestry path unexpectedly became empty"))?;
    if last.oid_sha1 != PHASE_A_ANCHOR_HEAD
        || last.tree_sha1 != PHASE_A_ANCHOR_TREE
        || last.raw_commit_sha256 != PHASE_A_ANCHOR_COMMIT_RAW_SHA256
        || last.raw_byte_count != PHASE_A_ANCHOR_COMMIT_RAW_BYTES
    {
        return Err(invalid(
            "ancestry path does not end at the exact Phase-A commit object",
        ));
    }

    for pair in manifest.commits.windows(2) {
        if pair[0].parent_oids_sha1.first() != Some(&pair[1].oid_sha1) {
            return Err(invalid(
                "ancestry path is not an exact final-to-anchor first-parent path",
            ));
        }
    }

    Ok(StructuralAncestryInspectionV1 {
        anchor: manifest.anchor,
        commit_count: manifest.commits.len(),
        final_tooling: manifest.final_tooling,
        manifest_sha256: sha256_hex(canonical_manifest),
        raw_objects_sha256: raw_object_set_sha256(raw_commits),
        _seal: StructuralInspectionSealV1,
    })
}

fn validate_repository_identity(
    identity: &RepositoryIdentityV1,
    label: &str,
) -> Result<(), MnlTrustError> {
    validate_oid(&identity.head, &format!("{label} head"))?;
    validate_oid(&identity.tree, &format!("{label} tree"))
}

fn validate_oid(value: &str, label: &str) -> Result<(), MnlTrustError> {
    if value.len() != 40
        || value.bytes().all(|byte| byte == b'0')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(format!(
            "{label} is not a canonical nonzero SHA-1 oid"
        )));
    }
    Ok(())
}

fn validate_sha256(value: &str, label: &str) -> Result<(), MnlTrustError> {
    if value.len() != 64
        || value.bytes().all(|byte| byte == b'0')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(format!("{label} is not canonical SHA-256")));
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

fn raw_object_set_sha256(raw_commits: &[RawGitCommitSidecarV1<'_>]) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(RAW_OBJECT_SET_DIGEST_DOMAIN);
    hasher.update((raw_commits.len() as u64).to_be_bytes());
    for sidecar in raw_commits {
        hasher.update((sidecar.oid_sha1.len() as u64).to_be_bytes());
        hasher.update(sidecar.oid_sha1.as_bytes());
        hasher.update((sidecar.raw_commit.len() as u64).to_be_bytes());
        hasher.update(sidecar.raw_commit);
    }
    format!("{:x}", hasher.finalize())
}

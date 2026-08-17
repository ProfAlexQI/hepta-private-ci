use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryIdentityV1 {
    pub head: String,
    pub tree: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum CompletionCapabilityV1 {
    #[serde(rename = "ExternalRoleSeparatedSignatures")]
    ExternalRoleSeparatedSignatures,
    #[serde(rename = "FinalToolingAncestry")]
    FinalToolingAncestry,
    #[serde(rename = "ExactSourceAndRoleBinaryProvenance")]
    ExactSourceAndRoleBinaryProvenance,
    #[serde(rename = "DurableAtomicOneShotReplay")]
    DurableAtomicOneShotReplay,
    #[serde(rename = "PreRunWallClockSupervisor")]
    PreRunWallClockSupervisor,
    #[serde(rename = "LiveReadOnlyCollectorAndClosedRunner")]
    LiveReadOnlyCollectorAndClosedRunner,
    #[serde(rename = "IndependentBundleCopyReadbackAndAckSigner")]
    IndependentBundleCopyReadbackAndAckSigner,
    #[serde(rename = "ImmutablePreRunAndPostRunPublication")]
    ImmutablePreRunAndPostRunPublication,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityStateV1 {
    Absent,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityDispositionV1 {
    InspectionOnlyNoAuthority,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceOriginV1 {
    FutureExternalFrozenOnly,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionDispositionV1 {
    Blocked,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityEntryV1 {
    pub authority: AuthorityDispositionV1,
    pub capability: CompletionCapabilityV1,
    pub evidence_origin: EvidenceOriginV1,
    pub state: CapabilityStateV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletionCapabilityLedgerV1 {
    pub anchor: RepositoryIdentityV1,
    pub authority: AuthorityDispositionV1,
    pub disposition: CompletionDispositionV1,
    pub entries: Vec<CapabilityEntryV1>,
    pub schema: String,
}

/// Opaque inspection of the exact blocked Phase-A capability ledger.
///
/// This token cannot be serialized, cloned, or converted into a platform
/// production context.
#[derive(Debug)]
pub struct AbsentCapabilityLedgerInspectionV1 {
    pub(crate) canonical_sha256: String,
    pub(crate) entry_count: usize,
    pub(crate) _seal: AbsentLedgerInspectionSealV1,
}

#[derive(Debug)]
pub(crate) struct AbsentLedgerInspectionSealV1;

impl AbsentCapabilityLedgerInspectionV1 {
    pub fn canonical_sha256(&self) -> &str {
        &self.canonical_sha256
    }

    pub fn entry_count(&self) -> usize {
        self.entry_count
    }

    pub const fn authorizes_live(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GitAncestryPathPolicyV1 {
    FinalToAnchorFirstParent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GitCommitManifestEntryV1 {
    pub oid_sha1: String,
    pub parent_oids_sha1: Vec<String>,
    pub raw_byte_count: u64,
    pub raw_commit_sha256: String,
    pub tree_sha1: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GitAncestryPathManifestV1 {
    pub anchor: RepositoryIdentityV1,
    pub commits: Vec<GitCommitManifestEntryV1>,
    pub final_tooling: RepositoryIdentityV1,
    pub path_policy: GitAncestryPathPolicyV1,
    pub schema: String,
}

/// Borrowed raw Git commit body bytes supplied out of tree beside the canonical
/// manifest.
///
/// `raw_commit` is exactly the output of `git cat-file commit <oid>`: it does
/// not include the loose-object `commit <length>\0` header, zlib framing, or
/// compression. The label is checked against both the manifest and the Git
/// object id recomputed from these body bytes.
#[derive(Debug)]
pub struct RawGitCommitSidecarV1<'a> {
    pub(crate) oid_sha1: &'a str,
    pub(crate) raw_commit: &'a [u8],
}

impl<'a> RawGitCommitSidecarV1<'a> {
    pub const fn new(oid_sha1: &'a str, raw_commit: &'a [u8]) -> Self {
        Self {
            oid_sha1,
            raw_commit,
        }
    }
}

/// Opaque result of structural Git-object inspection.
///
/// This type is intentionally neither serializable nor cloneable. It proves
/// only that the supplied raw objects form a path to the compiled Phase-A
/// anchor. It does not prove that the final identity was selected by a trusted
/// signer and it cannot authorize execution.
#[derive(Debug)]
pub struct StructuralAncestryInspectionV1 {
    pub(crate) anchor: RepositoryIdentityV1,
    pub(crate) commit_count: usize,
    pub(crate) final_tooling: RepositoryIdentityV1,
    pub(crate) manifest_sha256: String,
    pub(crate) raw_objects_sha256: String,
    pub(crate) _seal: StructuralInspectionSealV1,
}

#[derive(Debug)]
pub(crate) struct StructuralInspectionSealV1;

impl StructuralAncestryInspectionV1 {
    pub fn anchor(&self) -> &RepositoryIdentityV1 {
        &self.anchor
    }

    pub fn commit_count(&self) -> usize {
        self.commit_count
    }

    pub fn final_tooling(&self) -> &RepositoryIdentityV1 {
        &self.final_tooling
    }

    pub fn manifest_sha256(&self) -> &str {
        &self.manifest_sha256
    }

    pub fn raw_objects_sha256(&self) -> &str {
        &self.raw_objects_sha256
    }

    pub const fn authorizes_live(&self) -> bool {
        false
    }
}

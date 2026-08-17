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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum DetachedSignatureRoleV1 {
    #[serde(rename = "final_artifact_freeze")]
    FinalArtifactFreeze,
    #[serde(rename = "pre_run_profile")]
    PreRunProfile,
    #[serde(rename = "freeze_manifest")]
    FreezeManifest,
    #[serde(rename = "supervisor_seal")]
    SupervisorSeal,
    #[serde(rename = "independent_copy_ack")]
    IndependentCopyAck,
    #[serde(rename = "terminal_manifest")]
    TerminalManifest,
    #[serde(rename = "post_run_result_envelope")]
    PostRunResultEnvelope,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DetachedSignatureManifestV1 {
    pub schema: String,
    pub algorithm: String,
    pub role: DetachedSignatureRoleV1,
    pub trust_root_id: String,
    pub trust_root_revision: u64,
    pub trust_policy_sha256: String,
    pub signer_key_id: String,
    pub profile_id: String,
    pub payload_schema: String,
    pub payload_byte_count: u64,
    pub payload_sha256: String,
    pub signature_sha256: String,
}

/// Borrowed raw 64-byte detached Ed25519 signature supplied beside its
/// canonical manifest. This input is not a proof token.
#[derive(Debug)]
pub struct RawDetachedEd25519SignatureV1<'a> {
    pub(crate) raw_signature: &'a [u8],
}

impl<'a> RawDetachedEd25519SignatureV1<'a> {
    pub const fn new(raw_signature: &'a [u8]) -> Self {
        Self { raw_signature }
    }
}

/// Opaque observation that one compiled role key verified a detached
/// signature over exact payload bytes.
///
/// This observation does not validate payload semantics, freshness,
/// publication, copy-domain independence, or any live authority.
#[derive(Debug)]
pub struct VerifiedDetachedSignatureInspectionV1 {
    pub(crate) manifest_sha256: String,
    pub(crate) payload_bytes: Vec<u8>,
    pub(crate) payload_byte_count: u64,
    pub(crate) payload_schema: String,
    pub(crate) payload_sha256: String,
    pub(crate) profile_id: String,
    pub(crate) role: DetachedSignatureRoleV1,
    pub(crate) signature_sha256: String,
    pub(crate) signed_frame_sha256: String,
    pub(crate) signer_key_id: String,
    pub(crate) trust_policy_sha256: String,
    pub(crate) trust_root_id: String,
    pub(crate) trust_root_revision: u64,
    pub(crate) _seal: DetachedSignatureInspectionSealV1,
}

#[derive(Debug)]
pub(crate) struct DetachedSignatureInspectionSealV1;

impl VerifiedDetachedSignatureInspectionV1 {
    pub fn manifest_sha256(&self) -> &str {
        &self.manifest_sha256
    }

    pub fn payload_byte_count(&self) -> u64 {
        self.payload_byte_count
    }

    /// Returns the exact bytes whose detached signature was inspected.
    ///
    /// Keeping these bytes inside the opaque inspection is what allows later
    /// structural layers to re-parse signed semantics instead of accepting a
    /// caller-authored digest twin. The bytes remain non-authorizing.
    pub fn exact_payload_bytes(&self) -> &[u8] {
        &self.payload_bytes
    }

    pub fn payload_schema(&self) -> &str {
        &self.payload_schema
    }

    pub fn payload_sha256(&self) -> &str {
        &self.payload_sha256
    }

    pub fn profile_id(&self) -> &str {
        &self.profile_id
    }

    pub fn role(&self) -> DetachedSignatureRoleV1 {
        self.role
    }

    pub fn signature_sha256(&self) -> &str {
        &self.signature_sha256
    }

    pub fn signed_frame_sha256(&self) -> &str {
        &self.signed_frame_sha256
    }

    pub fn signer_key_id(&self) -> &str {
        &self.signer_key_id
    }

    pub fn trust_policy_sha256(&self) -> &str {
        &self.trust_policy_sha256
    }

    pub fn trust_root_id(&self) -> &str {
        &self.trust_root_id
    }

    pub fn trust_root_revision(&self) -> u64 {
        self.trust_root_revision
    }

    pub const fn authorizes_live(&self) -> bool {
        false
    }
}

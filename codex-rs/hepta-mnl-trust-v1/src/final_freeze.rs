use std::cmp::Ordering;
use std::collections::HashSet;

use serde::Deserialize;
use serde::Serialize;

use crate::DetachedSignatureRoleV1;
use crate::MatchedPreparedPreRunReplayClaimInspectionV1;
use crate::MnlTrustError;
use crate::ReplayPlatformScopeV1;
use crate::RepositoryIdentityV1;
use crate::StructuralAncestryInspectionV1;
use crate::VerifiedDetachedSignatureInspectionV1;
use crate::exact_phase_a_anchor;
use crate::invalid;

pub const FINAL_ARTIFACT_FREEZE_PAYLOAD_SCHEMA: &str = "hepta_mnl_final_artifact_freeze_payload_v1";

const MAX_FINAL_ARTIFACT_FREEZE_PAYLOAD_BYTES: usize = 4 * 1024 * 1024;
const MAX_PLATFORM_ARTIFACTS: usize = 256;
const MAX_NAMED_MATERIALS: usize = 256;
const MAX_IDENTIFIER_BYTES: usize = 128;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenArtifactBytesV1 {
    pub byte_count: u64,
    pub mode: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalSourceFreezeV1 {
    pub archive: FrozenArtifactBytesV1,
    pub archive_recipe_sha256: String,
    pub source_tree_manifest_sha256: String,
    pub toolchain_manifest_sha256: String,
}

/// Plan-derived portion of the canonical source freeze.
///
/// Archive-recipe and toolchain declarations deliberately stay in the signed
/// `CanonicalSourceFreezeV1`; a closed plan must not be treated as authority
/// for those provenance claims.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedCanonicalSourceFreezeV1 {
    pub archive: FrozenArtifactBytesV1,
    pub source_tree_manifest_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlatformArtifactFreezeV1 {
    pub artifact: FrozenArtifactBytesV1,
    pub build_recipe_sha256: String,
    pub platform_scope: ReplayPlatformScopeV1,
    pub role_id: String,
    pub role_source_manifest_sha256: String,
    pub toolchain_manifest_sha256: String,
}

/// Plan-derived identity of one frozen role artifact. Signed source, recipe,
/// and toolchain declarations are retained only in `PlatformArtifactFreezeV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedPlatformArtifactBytesV1 {
    pub artifact: FrozenArtifactBytesV1,
    pub platform_scope: ReplayPlatformScopeV1,
    pub role_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NamedMaterialFreezeV1 {
    pub name: String,
    pub platform_scope: ReplayPlatformScopeV1,
    pub sha256: String,
}

/// Exact semantic payload authenticated by the final-artifact-freeze role.
///
/// This wire value contains declarations only. In particular, its source,
/// recipe, toolchain, artifact, and named-material digests do not prove that
/// any corresponding bytes were observed or produced from the final tree.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FinalArtifactFreezePayloadV1 {
    pub ancestry_commit_count: u64,
    pub ancestry_manifest_sha256: String,
    pub ancestry_raw_objects_sha256: String,
    pub canonical_source: CanonicalSourceFreezeV1,
    pub final_tooling: RepositoryIdentityV1,
    pub named_materials: Vec<NamedMaterialFreezeV1>,
    pub phase_a_anchor: RepositoryIdentityV1,
    pub platform_artifacts: Vec<PlatformArtifactFreezeV1>,
    pub profile_id: String,
    pub schema: String,
}

/// Opaque consuming join of N1 structural ancestry and one N2 final-freeze
/// signature inspection.
///
/// The token is intentionally neither cloneable nor serializable. It retains
/// both inputs, but only records semantic equality; it grants no authority and
/// observes no source, build, toolchain, or artifact bytes.
#[derive(Debug)]
pub struct InspectedFinalArtifactFreezeV1 {
    ancestry: StructuralAncestryInspectionV1,
    payload: FinalArtifactFreezePayloadV1,
    signature: VerifiedDetachedSignatureInspectionV1,
}

/// Exact platform projection supplied by a closed-plan implementation.
///
/// Artifact and material vectors must already be filtered to
/// `platform_scope`, strictly sorted, and unique. This is an equality
/// expectation, not evidence or authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedPlatformArtifactFreezeV1 {
    pub canonical_source: ExpectedCanonicalSourceFreezeV1,
    pub final_artifact_freeze_payload_sha256: String,
    pub final_artifact_freeze_profile_id: String,
    pub final_tooling: RepositoryIdentityV1,
    pub named_materials: Vec<NamedMaterialFreezeV1>,
    pub platform_artifacts: Vec<ExpectedPlatformArtifactBytesV1>,
    pub platform_scope: ReplayPlatformScopeV1,
}

/// Opaque consuming equality join of typed final-freeze semantics, a prepared
/// claim already matched to a closed plan, and that plan's exact platform
/// freeze projection.
#[derive(Debug)]
pub struct MatchedFinalFreezePlanClaimInspectionV1 {
    final_freeze: InspectedFinalArtifactFreezeV1,
    matched_claim: MatchedPreparedPreRunReplayClaimInspectionV1,
    platform_scope: ReplayPlatformScopeV1,
}

impl InspectedFinalArtifactFreezeV1 {
    pub fn ancestry(&self) -> &StructuralAncestryInspectionV1 {
        &self.ancestry
    }

    pub fn canonical_source(&self) -> &CanonicalSourceFreezeV1 {
        &self.payload.canonical_source
    }

    pub fn final_tooling(&self) -> &RepositoryIdentityV1 {
        &self.payload.final_tooling
    }

    pub fn named_materials(&self) -> &[NamedMaterialFreezeV1] {
        &self.payload.named_materials
    }

    pub fn payload_sha256(&self) -> &str {
        self.signature.payload_sha256()
    }

    pub fn platform_artifacts(&self) -> &[PlatformArtifactFreezeV1] {
        &self.payload.platform_artifacts
    }

    pub fn profile_id(&self) -> &str {
        &self.payload.profile_id
    }

    pub const fn authorizes_live(&self) -> bool {
        false
    }

    pub const fn actual_artifact_bytes_observed(&self) -> bool {
        false
    }

    pub const fn source_provenance_observed(&self) -> bool {
        false
    }

    pub const fn toolchain_provenance_observed(&self) -> bool {
        false
    }

    pub(crate) fn signature_inspection(&self) -> &VerifiedDetachedSignatureInspectionV1 {
        &self.signature
    }
}

impl MatchedFinalFreezePlanClaimInspectionV1 {
    pub fn final_freeze(&self) -> &InspectedFinalArtifactFreezeV1 {
        &self.final_freeze
    }

    pub fn matched_claim(&self) -> &MatchedPreparedPreRunReplayClaimInspectionV1 {
        &self.matched_claim
    }

    pub fn platform_scope(&self) -> ReplayPlatformScopeV1 {
        self.platform_scope
    }

    pub const fn typed_final_freeze_bound(&self) -> bool {
        true
    }

    pub const fn authorizes_live(&self) -> bool {
        false
    }

    pub const fn actual_artifact_bytes_observed(&self) -> bool {
        false
    }

    pub const fn source_provenance_observed(&self) -> bool {
        false
    }

    pub const fn toolchain_provenance_observed(&self) -> bool {
        false
    }

    pub const fn durable_claim_observed(&self) -> bool {
        false
    }

    pub const fn wall_clock_verified(&self) -> bool {
        false
    }

    pub const fn launch_grant_available(&self) -> bool {
        false
    }

    pub const fn launch_performed(&self) -> bool {
        false
    }

    pub const fn receipt_emitted(&self) -> bool {
        false
    }
}

pub fn inspect_final_artifact_freeze_semantics(
    ancestry: StructuralAncestryInspectionV1,
    signature: VerifiedDetachedSignatureInspectionV1,
) -> Result<InspectedFinalArtifactFreezeV1, MnlTrustError> {
    if signature.role() != DetachedSignatureRoleV1::FinalArtifactFreeze
        || signature.authorizes_live()
        || ancestry.authorizes_live()
    {
        return Err(invalid(
            "final artifact freeze uses the wrong signature role or an authorizing input",
        ));
    }
    let exact_payload = signature.exact_payload_bytes();
    if exact_payload.is_empty() || exact_payload.len() > MAX_FINAL_ARTIFACT_FREEZE_PAYLOAD_BYTES {
        return Err(invalid(
            "final artifact-freeze payload byte length is outside its bound",
        ));
    }
    let payload: FinalArtifactFreezePayloadV1 = serde_json::from_slice(exact_payload)
        .map_err(|error| MnlTrustError::Serialization(error.to_string()))?;
    let reencoded = serde_json::to_vec(&payload)
        .map_err(|error| MnlTrustError::Serialization(error.to_string()))?;
    if reencoded != exact_payload {
        return Err(invalid(
            "final artifact-freeze payload is not exact canonical JSON",
        ));
    }
    validate_payload(&payload)?;
    let ancestry_commit_count = u64::try_from(ancestry.commit_count())
        .map_err(|_| invalid("ancestry commit count is not representable"))?;
    if payload.profile_id != signature.profile_id()
        || payload.phase_a_anchor != exact_phase_a_anchor()
        || payload.phase_a_anchor != *ancestry.anchor()
        || payload.final_tooling != *ancestry.final_tooling()
        || payload.ancestry_manifest_sha256 != ancestry.manifest_sha256()
        || payload.ancestry_raw_objects_sha256 != ancestry.raw_objects_sha256()
        || payload.ancestry_commit_count != ancestry_commit_count
    {
        return Err(invalid(
            "final artifact-freeze semantics differ from signature or structural ancestry",
        ));
    }
    Ok(InspectedFinalArtifactFreezeV1 {
        ancestry,
        payload,
        signature,
    })
}

pub fn match_final_freeze_to_prepared_claim(
    final_freeze: InspectedFinalArtifactFreezeV1,
    matched_claim: MatchedPreparedPreRunReplayClaimInspectionV1,
    expected: &ExpectedPlatformArtifactFreezeV1,
) -> Result<MatchedFinalFreezePlanClaimInspectionV1, MnlTrustError> {
    validate_expected_platform_freeze(expected)?;
    let prepared_claim = matched_claim.prepared_claim();
    if !matched_claim.matches_final_artifact_freeze_signature(final_freeze.signature_inspection())
        || prepared_claim.platform_scope() != expected.platform_scope
        || final_freeze.payload_sha256() != expected.final_artifact_freeze_payload_sha256
        || final_freeze.profile_id() != expected.final_artifact_freeze_profile_id
        || final_freeze.final_tooling() != &expected.final_tooling
        || final_freeze.canonical_source().archive != expected.canonical_source.archive
        || final_freeze.canonical_source().source_tree_manifest_sha256
            != expected.canonical_source.source_tree_manifest_sha256
    {
        return Err(invalid(
            "typed final artifact freeze differs from prepared claim or platform expectation",
        ));
    }

    let platform_artifacts = final_freeze
        .platform_artifacts()
        .iter()
        .filter(|record| record.platform_scope == expected.platform_scope)
        .collect::<Vec<_>>();
    if platform_artifacts.len() != expected.platform_artifacts.len()
        || !platform_artifacts
            .iter()
            .zip(&expected.platform_artifacts)
            .all(|(actual, expected)| {
                actual.artifact == expected.artifact
                    && actual.platform_scope == expected.platform_scope
                    && actual.role_id == expected.role_id
            })
    {
        return Err(invalid(
            "typed final artifact freeze differs from exact platform artifact expectation",
        ));
    }

    let named_materials = final_freeze
        .named_materials()
        .iter()
        .filter(|record| record.platform_scope == expected.platform_scope)
        .collect::<Vec<_>>();
    if named_materials.len() != expected.named_materials.len()
        || !named_materials
            .iter()
            .zip(&expected.named_materials)
            .all(|(actual, expected)| *actual == expected)
    {
        return Err(invalid(
            "typed final artifact freeze differs from exact named-material expectation",
        ));
    }

    Ok(MatchedFinalFreezePlanClaimInspectionV1 {
        final_freeze,
        matched_claim,
        platform_scope: expected.platform_scope,
    })
}

fn validate_payload(payload: &FinalArtifactFreezePayloadV1) -> Result<(), MnlTrustError> {
    if payload.schema != FINAL_ARTIFACT_FREEZE_PAYLOAD_SCHEMA {
        return Err(invalid("final artifact-freeze payload schema is not exact"));
    }
    validate_identifier(&payload.profile_id, "final artifact-freeze profile id")?;
    validate_repository_identity(&payload.phase_a_anchor, "Phase-A anchor")?;
    validate_repository_identity(&payload.final_tooling, "final tooling")?;
    if payload.ancestry_commit_count == 0 {
        return Err(invalid(
            "final artifact-freeze ancestry commit count is zero",
        ));
    }
    validate_sha256(
        &payload.ancestry_manifest_sha256,
        "ancestry manifest digest",
    )?;
    validate_sha256(
        &payload.ancestry_raw_objects_sha256,
        "ancestry raw-object-set digest",
    )?;
    validate_canonical_source(&payload.canonical_source)?;
    validate_platform_artifacts(&payload.platform_artifacts, false)?;
    validate_named_materials(&payload.named_materials, false)
}

fn validate_expected_platform_freeze(
    expected: &ExpectedPlatformArtifactFreezeV1,
) -> Result<(), MnlTrustError> {
    validate_repository_identity(&expected.final_tooling, "expected final tooling")?;
    validate_identifier(
        &expected.final_artifact_freeze_profile_id,
        "expected final artifact-freeze profile id",
    )?;
    validate_sha256(
        &expected.final_artifact_freeze_payload_sha256,
        "expected final artifact-freeze payload digest",
    )?;
    validate_expected_canonical_source(&expected.canonical_source)?;
    validate_expected_platform_artifacts(&expected.platform_artifacts)?;
    validate_named_materials(&expected.named_materials, true)?;
    if expected
        .platform_artifacts
        .iter()
        .any(|record| record.platform_scope != expected.platform_scope)
        || expected
            .named_materials
            .iter()
            .any(|record| record.platform_scope != expected.platform_scope)
    {
        return Err(invalid(
            "expected platform freeze contains a record from another scope",
        ));
    }
    Ok(())
}

fn validate_canonical_source(source: &CanonicalSourceFreezeV1) -> Result<(), MnlTrustError> {
    validate_frozen_artifact_bytes(&source.archive, "canonical source archive")?;
    validate_sha256(&source.archive_recipe_sha256, "source archive recipe")?;
    validate_sha256(&source.source_tree_manifest_sha256, "source tree manifest")?;
    validate_sha256(
        &source.toolchain_manifest_sha256,
        "source toolchain manifest",
    )
}

fn validate_expected_canonical_source(
    source: &ExpectedCanonicalSourceFreezeV1,
) -> Result<(), MnlTrustError> {
    validate_frozen_artifact_bytes(&source.archive, "expected canonical source archive")?;
    validate_sha256(
        &source.source_tree_manifest_sha256,
        "expected source tree manifest",
    )
}

fn validate_expected_platform_artifacts(
    artifacts: &[ExpectedPlatformArtifactBytesV1],
) -> Result<(), MnlTrustError> {
    if artifacts.len() > MAX_PLATFORM_ARTIFACTS {
        return Err(invalid(
            "expected platform artifact record count exceeds its bound",
        ));
    }
    let mut artifact_digests = HashSet::with_capacity(artifacts.len());
    for artifact in artifacts {
        validate_identifier(&artifact.role_id, "expected platform artifact role id")?;
        validate_frozen_artifact_bytes(&artifact.artifact, "expected platform artifact")?;
        if !artifact_digests.insert((
            platform_scope_rank(artifact.platform_scope),
            artifact.artifact.sha256.as_str(),
        )) {
            return Err(invalid(
                "expected platform artifact roles alias the same frozen bytes",
            ));
        }
    }
    if artifacts.windows(2).any(|pair| {
        compare_scoped_name(
            pair[0].platform_scope,
            &pair[0].role_id,
            pair[1].platform_scope,
            &pair[1].role_id,
        ) != Ordering::Less
    }) {
        return Err(invalid(
            "expected platform artifact records are not strictly sorted and unique",
        ));
    }
    Ok(())
}

fn validate_platform_artifacts(
    artifacts: &[PlatformArtifactFreezeV1],
    allow_empty: bool,
) -> Result<(), MnlTrustError> {
    if (!allow_empty && artifacts.is_empty()) || artifacts.len() > MAX_PLATFORM_ARTIFACTS {
        return Err(invalid(
            "platform artifact record count is outside its bound",
        ));
    }
    let mut artifact_digests = HashSet::with_capacity(artifacts.len());
    for artifact in artifacts {
        validate_identifier(&artifact.role_id, "platform artifact role id")?;
        validate_frozen_artifact_bytes(&artifact.artifact, "platform artifact")?;
        validate_sha256(&artifact.build_recipe_sha256, "artifact build recipe")?;
        validate_sha256(
            &artifact.role_source_manifest_sha256,
            "artifact role-source manifest",
        )?;
        validate_sha256(
            &artifact.toolchain_manifest_sha256,
            "artifact toolchain manifest",
        )?;
        if !artifact_digests.insert((
            platform_scope_rank(artifact.platform_scope),
            artifact.artifact.sha256.as_str(),
        )) {
            return Err(invalid(
                "platform artifact roles alias the same frozen bytes",
            ));
        }
    }
    if artifacts.windows(2).any(|pair| {
        compare_scoped_name(
            pair[0].platform_scope,
            &pair[0].role_id,
            pair[1].platform_scope,
            &pair[1].role_id,
        ) != Ordering::Less
    }) {
        return Err(invalid(
            "platform artifact records are not strictly sorted and unique",
        ));
    }
    Ok(())
}

fn validate_named_materials(
    materials: &[NamedMaterialFreezeV1],
    allow_empty: bool,
) -> Result<(), MnlTrustError> {
    if (!allow_empty && materials.is_empty()) || materials.len() > MAX_NAMED_MATERIALS {
        return Err(invalid("named material record count is outside its bound"));
    }
    let mut material_digests = HashSet::with_capacity(materials.len());
    for material in materials {
        validate_identifier(&material.name, "named material id")?;
        validate_sha256(&material.sha256, "named material digest")?;
        if !material_digests.insert((
            platform_scope_rank(material.platform_scope),
            material.sha256.as_str(),
        )) {
            return Err(invalid("named material roles alias the same frozen digest"));
        }
    }
    if materials.windows(2).any(|pair| {
        compare_scoped_name(
            pair[0].platform_scope,
            &pair[0].name,
            pair[1].platform_scope,
            &pair[1].name,
        ) != Ordering::Less
    }) {
        return Err(invalid(
            "named material records are not strictly sorted and unique",
        ));
    }
    Ok(())
}

fn compare_scoped_name(
    left_scope: ReplayPlatformScopeV1,
    left_name: &str,
    right_scope: ReplayPlatformScopeV1,
    right_name: &str,
) -> Ordering {
    platform_scope_rank(left_scope)
        .cmp(&platform_scope_rank(right_scope))
        .then_with(|| left_name.cmp(right_name))
}

const fn platform_scope_rank(scope: ReplayPlatformScopeV1) -> u8 {
    match scope {
        ReplayPlatformScopeV1::MacOs => 0,
        ReplayPlatformScopeV1::LinuxPhase1 => 1,
        ReplayPlatformScopeV1::Nix => 2,
    }
}

fn validate_frozen_artifact_bytes(
    artifact: &FrozenArtifactBytesV1,
    label: &str,
) -> Result<(), MnlTrustError> {
    if artifact.byte_count == 0 {
        return Err(invalid(format!("{label} byte count is zero")));
    }
    validate_mode(&artifact.mode, label)?;
    validate_sha256(&artifact.sha256, &format!("{label} digest"))
}

fn validate_repository_identity(
    identity: &RepositoryIdentityV1,
    label: &str,
) -> Result<(), MnlTrustError> {
    validate_sha1(&identity.head, &format!("{label} head"))?;
    validate_sha1(&identity.tree, &format!("{label} tree"))
}

fn validate_mode(value: &str, label: &str) -> Result<(), MnlTrustError> {
    let bytes = value.as_bytes();
    if bytes.len() != 4
        || bytes[0] != b'0'
        || !bytes[1..].iter().all(|byte| matches!(byte, b'0'..=b'7'))
    {
        return Err(invalid(format!("{label} mode is not canonical octal")));
    }
    Ok(())
}

fn validate_identifier(value: &str, label: &str) -> Result<(), MnlTrustError> {
    let mut bytes = value.bytes();
    let Some(first) = bytes.next() else {
        return Err(invalid(format!("{label} is empty")));
    };
    if value.len() > MAX_IDENTIFIER_BYTES
        || !(first.is_ascii_lowercase() || first.is_ascii_digit())
        || !bytes.all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'-' | b'_' | b'.' | b':')
        })
    {
        return Err(invalid(format!("{label} is not a canonical identifier")));
    }
    Ok(())
}

fn validate_sha1(value: &str, label: &str) -> Result<(), MnlTrustError> {
    validate_lower_hex(value, 40, label)
}

fn validate_sha256(value: &str, label: &str) -> Result<(), MnlTrustError> {
    validate_lower_hex(value, 64, label)
}

fn validate_lower_hex(value: &str, length: usize, label: &str) -> Result<(), MnlTrustError> {
    if value.len() != length
        || value.bytes().all(|byte| byte == b'0')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(format!("{label} is not canonical lowercase hex")));
    }
    Ok(())
}

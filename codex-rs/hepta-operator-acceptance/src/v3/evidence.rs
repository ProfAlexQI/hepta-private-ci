use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Component;
use std::path::Path;

use serde_json::Map;
use serde_json::Value;

use crate::AcceptanceError;
use crate::durable::MAX_SMALL_FILE_BYTES;
use crate::durable::canonical_json;
use crate::durable::secure_canonical_file_path;
use crate::durable::secure_read;
use crate::durable::sha256;
use crate::manifest_inventory::LegacyExtendedMetadataPolicy;
use crate::manifest_inventory::VerifiedManifest;
use crate::manifest_inventory::digest_shape;
use crate::manifest_inventory::load_legacy_manifest;
use crate::manifest_inventory::load_legacy_manifest_with_policy;
use crate::manifest_inventory::parse_manifest;
use crate::manifest_inventory::validate_relative_path;
use crate::model::AuthorityBoundary;
use crate::trust::verify_sshsig_bytes;

use super::model::AggregateBuildSpecV3;
use super::model::AggregateQualificationPacketV3;
use super::model::ArtifactBindingV3;
use super::model::CandidateBindingV3;
use super::model::CandidateBundleBindingV3;
use super::model::EvidenceProfileV3;
use super::model::ManifestLayerBindingV3;
use super::model::ManifestLayerIdV3;
use super::model::ModeManifestFormatV3;
use super::model::ObservedGateV3;
use super::model::ObservedPrerequisiteV3;
use super::model::PlatformGateBindingV3;
use super::model::PlatformPolicyV3;
use super::model::PrerequisiteReceiptBindingV3;
use super::model::QualificationAssessmentV3;
use super::model::QualificationDecisionV3;
use super::model::ReceiptEvidenceBindingV3;
use super::model::ReceiptProvenanceV3;
use super::profiles;

pub(super) const BUILD_SPEC_SCHEMA: &str = "hepta_vnext_aggregate_build_spec_v3";
pub(super) const PACKET_SCHEMA: &str = "hepta_vnext_aggregate_qualification_packet_v3";
pub(super) const ASSESSMENT_SCHEMA: &str = "hepta_operator_acceptance_qualification_assessment_v3";
pub(super) const CANDIDATE_HEAD: &str = "52ec4b3868fc5272e19ed516d00e11e44c549ea4";
pub(super) const CANDIDATE_TREE: &str = "247e9e7cfcb41dbfcc8c5b3b531b1e1407c0bd5d";
pub(super) const CANDIDATE_PARENT: &str = "32fb822ccc4eda7949b0fc4101f594604e31f282";
pub(super) const INTEGRATION_MERGE: &str = "8b60a902b537a1b01f7580327bcf08317f9a145a";
pub(super) const UPSTREAM_CUTOFF: &str = "74004b5397b24662a87a5264a6ae80664168c7f3";
const CANDIDATE_BUNDLE_RELATIVE_PATH: &str = "candidate-52ec4b3868.bundle";
const CANDIDATE_BUNDLE_SHA256: &str =
    "cd27e0b0a7bbbb14fd78183b1ffe5aa5ea9fb7d187a08ce381305f29f8d7feb3";
const CANDIDATE_BUNDLE_SIZE_BYTES: u64 = 176_335_964;
const PORTABLE_INPUTS_ID: &str = "portable-inputs";

pub(super) const GATES: [&str; 5] = [
    "macos-aarch64",
    "linux-x86_64",
    "nix-x86_64-linux",
    "windows-x86_64-native",
    "github-actions",
];

pub(super) const PREREQUISITES: [&str; 3] = [
    PORTABLE_INPUTS_ID,
    "canonical-path-trust",
    "upstream-cutoff-observation",
];

pub(super) struct ValidationPolicy<'a> {
    pub expected_candidate: &'a CandidateBindingV3,
    pub receipts_parent: &'a Path,
}

struct VerifiedReceipt {
    layers: BTreeMap<ManifestLayerIdV3, VerifiedManifest>,
    original: Option<crate::manifest_inventory::LegacyManifestSnapshot>,
    profile: EvidenceProfileV3,
}

impl VerifiedReceipt {
    fn layer(&self, id: ManifestLayerIdV3) -> Result<&VerifiedManifest, AcceptanceError> {
        self.layers
            .get(&id)
            .ok_or_else(|| invalid(format!("profile {:?} omits layer {id:?}", self.profile)))
    }

    fn reverify(&self) -> Result<(), AcceptanceError> {
        for manifest in self.layers.values() {
            manifest.reverify()?;
        }
        if let Some(original) = &self.original {
            original.reverify()?;
        }
        Ok(())
    }
}

pub(super) fn exact_candidate() -> CandidateBindingV3 {
    CandidateBindingV3 {
        bundle: CandidateBundleBindingV3 {
            prerequisite_id: PORTABLE_INPUTS_ID.to_string(),
            relative_path: CANDIDATE_BUNDLE_RELATIVE_PATH.to_string(),
            sha256: CANDIDATE_BUNDLE_SHA256.to_string(),
            size_bytes: CANDIDATE_BUNDLE_SIZE_BYTES,
        },
        head: CANDIDATE_HEAD.to_string(),
        integration_merge: INTEGRATION_MERGE.to_string(),
        parents: vec![CANDIDATE_PARENT.to_string()],
        tree: CANDIDATE_TREE.to_string(),
        upstream_cutoff: UPSTREAM_CUTOFF.to_string(),
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct LinuxOperatorTrustPolicyV7 {
    acceptance_profile_revision: u32,
    allowed_signers_sha256: String,
    authorization_scope: String,
    authorized_action: String,
    candidate_head: String,
    candidate_nix_process_pause_authority: bool,
    candidate_tree: String,
    challenge_maximum_lifetime_seconds: u64,
    challenge_schema: String,
    delete_authority: bool,
    driver_revision: u32,
    execution_authorization_schema: String,
    fresh_authorization_nonce_required: bool,
    fresh_challenge_required: bool,
    independent_workload_pause_restore_authority: bool,
    key_fingerprint: String,
    nix_container_volume_source_mutation_authority: bool,
    parent_trust_policy_sha256: String,
    principal: String,
    production_authority: bool,
    promotion_authority: bool,
    qualification_host: String,
    runner_pause_restore_authority: bool,
    schema: String,
    schema_version: u32,
    signature_algorithm: String,
    signature_namespace: String,
    single_use: bool,
    trust_policy_scope: String,
    trust_root_id: String,
    trust_root_revision: u32,
    unregister_authority: bool,
}

fn validate_linux_v5_trust_policy(
    policy_bytes: &[u8],
    allowed_signers_bytes: &[u8],
) -> Result<(), AcceptanceError> {
    let value = super::strict_json::parse(policy_bytes)?;
    let policy: LinuxOperatorTrustPolicyV7 = serde_json::from_value(value)
        .map_err(|error| invalid(format!("invalid Linux v5 trust policy: {error}")))?;
    if canonical_json(&policy)? != policy_bytes {
        return Err(invalid("Linux v5 trust policy is not canonical JSON"));
    }
    let expected_candidate = exact_candidate();
    if policy.schema != profiles::LINUX_TRUST_POLICY_SCHEMA
        || policy.schema_version != 1
        || policy.trust_root_id != profiles::LINUX_TRUST_ROOT_ID
        || policy.trust_root_revision != 2
        || policy.parent_trust_policy_sha256 != profiles::LINUX_PARENT_TRUST_POLICY_SHA256
        || policy.candidate_head != expected_candidate.head
        || policy.candidate_tree != expected_candidate.tree
        || policy.acceptance_profile_revision != 7
        || policy.driver_revision != 5
        || policy.qualification_host != "desktop-ts"
        || policy.principal != profiles::LINUX_OPERATOR_PRINCIPAL
        || policy.key_fingerprint != profiles::LINUX_OPERATOR_KEY_FINGERPRINT
        || policy.allowed_signers_sha256 != profiles::LINUX_OPERATOR_ALLOWED_SIGNERS_SHA256
        || policy.signature_algorithm != "sshsig-ed25519"
        || policy.signature_namespace != profiles::LINUX_OPERATOR_SIGNATURE_NAMESPACE
        || policy.authorized_action != profiles::LINUX_OPERATOR_ACTION
        || policy.authorization_scope != profiles::LINUX_OPERATOR_AUTHORIZATION_SCOPE
        || policy.challenge_schema != "hepta_vnext_linux_operator_challenge_v2"
        || policy.execution_authorization_schema != "hepta_vnext_linux_execution_authorization_v1"
        || policy.challenge_maximum_lifetime_seconds != 900
        || !policy.fresh_challenge_required
        || !policy.fresh_authorization_nonce_required
        || !policy.single_use
        || !policy.runner_pause_restore_authority
        || !policy.independent_workload_pause_restore_authority
        || policy.candidate_nix_process_pause_authority
        || policy.nix_container_volume_source_mutation_authority
        || policy.promotion_authority
        || policy.production_authority
        || policy.unregister_authority
        || policy.delete_authority
        || policy.trust_policy_scope
            != "candidate_52ec_linux_v5_runner_and_independent_workload_lifecycle_only"
    {
        return Err(invalid(
            "Linux v5 trust policy differs from the compiled revision-7 boundary",
        ));
    }
    if allowed_signers_bytes != profiles::LINUX_OPERATOR_ALLOWED_SIGNERS.as_bytes()
        || sha256(allowed_signers_bytes) != profiles::LINUX_OPERATOR_ALLOWED_SIGNERS_SHA256
    {
        return Err(invalid(
            "Linux v5 allowed-signers file differs from the compiled public signer",
        ));
    }
    Ok(())
}

#[cfg(test)]
pub(super) fn validate_linux_v5_trust_policy_for_test(
    policy_bytes: &[u8],
    allowed_signers_bytes: &[u8],
) -> Result<(), AcceptanceError> {
    validate_linux_v5_trust_policy(policy_bytes, allowed_signers_bytes)
}

pub(super) fn exact_platform_policy() -> PlatformPolicyV3 {
    PlatformPolicyV3 {
        blocked_external_satisfies_required_gate: false,
        native_windows_substitutes_for_github: false,
        require_all_required_gates_pass: true,
        required_gates: GATES.iter().map(|gate| (*gate).to_string()).collect(),
        zero_step_execution_satisfies_pass: false,
    }
}

pub(super) fn validate_spec(
    spec: &AggregateBuildSpecV3,
    policy: ValidationPolicy<'_>,
) -> Result<AggregateQualificationPacketV3, AcceptanceError> {
    if spec.schema != BUILD_SPEC_SCHEMA
        || spec.schema_version != 3
        || spec.profile_set != profiles::PROFILE_SET
        || spec.automatic_transition
        || spec.authority != AuthorityBoundary::all_closed()
        || spec.candidate != *policy.expected_candidate
        || spec.platform_policy != exact_platform_policy()
        || spec.platform_gates.len() != GATES.len()
        || spec.prerequisite_receipts.len() != PREREQUISITES.len()
    {
        return Err(invalid(
            "aggregate build spec differs from the exact 52ec V3 boundary",
        ));
    }

    let mut receipt_roots = Vec::new();
    let mut verified_receipts = Vec::new();
    let mut platform_receipts = Vec::new();
    for (input, expected_gate) in spec.platform_gates.iter().zip(GATES) {
        let expected_profile = profiles::gate_profile(expected_gate)
            .ok_or_else(|| invalid("compiled gate profile is absent"))?;
        if input.gate != expected_gate || !input.required || input.profile != expected_profile {
            return Err(invalid(
                "platform gates must use the compiled profile and canonical order",
            ));
        }
        if profiles::is_unpinned(expected_profile) {
            if input.receipt.is_some() {
                return Err(invalid(
                    "PROFILE_IDENTITY_UNPINNED: an unpinned profile cannot accept a receipt",
                ));
            }
            platform_receipts.push(PlatformGateBindingV3 {
                gate: input.gate.clone(),
                observed: ObservedGateV3 {
                    candidate_executed: false,
                    candidate_failure: false,
                    executed_steps: 0,
                    harness_failure: false,
                    pass: false,
                    production_changed: None,
                    qualification: false,
                    refs_changed: None,
                    status: "PROFILE_UNPINNED".to_string(),
                },
                profile: expected_profile,
                receipt: None,
                required: true,
            });
            continue;
        }
        let binding = input
            .receipt
            .as_ref()
            .ok_or_else(|| invalid("pinned platform profile requires a receipt"))?;
        let verified = validate_receipt(binding, expected_profile, policy.receipts_parent, None)?;
        let observed = observe_gate(expected_profile, &verified, policy.expected_candidate)?;
        admit_formal_platform_pass(&observed)?;
        verified.reverify()?;
        receipt_roots.push(binding.receipt_root.clone());
        if let Some(original) = &verified.original {
            receipt_roots.push(
                original
                    .root
                    .to_str()
                    .ok_or_else(|| invalid("original receipt root is not UTF-8"))?
                    .to_string(),
            );
        }
        platform_receipts.push(PlatformGateBindingV3 {
            gate: input.gate.clone(),
            observed,
            profile: expected_profile,
            receipt: Some(binding.clone()),
            required: true,
        });
        verified_receipts.push(verified);
    }

    let mut prerequisite_receipts = Vec::new();
    for (input, expected_id) in spec.prerequisite_receipts.iter().zip(PREREQUISITES) {
        let expected_profile = profiles::prerequisite_profile(expected_id)
            .ok_or_else(|| invalid("compiled prerequisite profile is absent"))?;
        if input.id != expected_id || !input.required || input.profile != expected_profile {
            return Err(invalid(
                "prerequisites must use the compiled profile and canonical order",
            ));
        }
        let required_file = (expected_id == PORTABLE_INPUTS_ID).then_some(&spec.candidate.bundle);
        let verified = validate_receipt(
            &input.receipt,
            expected_profile,
            policy.receipts_parent,
            required_file,
        )?;
        let observed =
            observe_prerequisite(expected_profile, &verified, policy.expected_candidate)?;
        admit_formal_prerequisite_pass(&observed)?;
        verified.reverify()?;
        receipt_roots.push(input.receipt.receipt_root.clone());
        if let Some(original) = &verified.original {
            receipt_roots.push(
                original
                    .root
                    .to_str()
                    .ok_or_else(|| invalid("original receipt root is not UTF-8"))?
                    .to_string(),
            );
        }
        prerequisite_receipts.push(PrerequisiteReceiptBindingV3 {
            id: input.id.clone(),
            observed,
            profile: expected_profile,
            receipt: input.receipt.clone(),
            required: true,
        });
        verified_receipts.push(verified);
    }
    validate_disjoint_receipt_roots(&receipt_roots)?;
    for receipt in &verified_receipts {
        receipt.reverify()?;
    }

    let decision = decision(&platform_receipts, &prerequisite_receipts);
    Ok(AggregateQualificationPacketV3 {
        automatic_transition: false,
        authority: AuthorityBoundary::all_closed(),
        candidate: policy.expected_candidate.clone(),
        decision,
        platform_policy: exact_platform_policy(),
        platform_receipts,
        prerequisite_receipts,
        profile_set: profiles::PROFILE_SET.to_string(),
        schema: PACKET_SCHEMA.to_string(),
        schema_version: 3,
    })
}

fn admit_formal_platform_pass(observed: &ObservedGateV3) -> Result<(), AcceptanceError> {
    if observed.status != "PASS"
        || !observed.pass
        || !observed.qualification
        || !observed.candidate_executed
        || observed.candidate_failure
        || observed.harness_failure
        || observed.executed_steps == 0
        || observed.production_changed == Some(true)
        || observed.refs_changed == Some(true)
    {
        return Err(invalid(
            "NON_PASS_RECEIPT_NOT_AGGREGATE_INPUT: diagnostic platform receipts are excluded",
        ));
    }
    Ok(())
}

fn admit_formal_prerequisite_pass(
    observed: &ObservedPrerequisiteV3,
) -> Result<(), AcceptanceError> {
    if observed.status != "PASS"
        || !observed.pass
        || observed.production_changed == Some(true)
        || observed.refs_changed == Some(true)
    {
        return Err(invalid(
            "NON_PASS_RECEIPT_NOT_AGGREGATE_INPUT: diagnostic prerequisite receipts are excluded",
        ));
    }
    Ok(())
}

#[cfg(test)]
pub(super) fn admit_formal_platform_pass_for_test(
    observed: &ObservedGateV3,
) -> Result<(), AcceptanceError> {
    admit_formal_platform_pass(observed)
}

pub(super) fn assess_packet(
    packet: &AggregateQualificationPacketV3,
    aggregate_manifest_sha256: &str,
) -> QualificationAssessmentV3 {
    QualificationAssessmentV3 {
        aggregate_manifest_sha256: aggregate_manifest_sha256.to_string(),
        blockers: packet.decision.blockers.clone(),
        candidate_head: packet.candidate.head.clone(),
        candidate_tree: packet.candidate.tree.clone(),
        complete_gate_count: packet.decision.complete_gate_count,
        pass_gate_count: packet.decision.pass_gate_count,
        prerequisite_pass_count: packet.decision.prerequisite_pass_count,
        ready_for_challenge: packet.decision.blockers.is_empty(),
        schema: ASSESSMENT_SCHEMA.to_string(),
    }
}

fn validate_receipt(
    binding: &ReceiptEvidenceBindingV3,
    profile: EvidenceProfileV3,
    receipts_parent: &Path,
    required_file: Option<&CandidateBundleBindingV3>,
) -> Result<VerifiedReceipt, AcceptanceError> {
    if binding.profile != profile {
        return Err(invalid("receipt profile differs from its compiled role"));
    }
    if profiles::is_unpinned(profile) {
        return Err(invalid(format!(
            "PROFILE_IDENTITY_UNPINNED: final {profile:?} receipt identity is not frozen"
        )));
    }
    validate_frozen_receipt_identity(binding, profile)?;
    let receipt_root = Path::new(&binding.receipt_root);
    if !receipt_root.starts_with(receipts_parent) || receipt_root == receipts_parent {
        return Err(invalid(
            "receipt root must be a strict child of the frozen receipts parent",
        ));
    }
    let expected_layers = profiles::expected_layers(profile);
    if binding.manifest_layers.len() != expected_layers.len() {
        return Err(invalid("receipt layer topology differs from its profile"));
    }

    let mut layers = BTreeMap::new();
    for (source, expected_id) in binding.manifest_layers.iter().zip(expected_layers) {
        validate_layer_binding(profile, source, *expected_id)?;
        let manifest = load_layer(receipt_root, source)?;
        verify_mode_manifest(
            source,
            &manifest,
            &compiled_visible_mode_paths(profile, *expected_id)?,
        )?;
        if layers.insert(*expected_id, manifest).is_some() {
            return Err(invalid("receipt contains a duplicate manifest layer"));
        }
    }
    let outer = layers
        .get(&ManifestLayerIdV3::Outer)
        .ok_or_else(|| invalid("receipt outer layer is absent"))?;
    reject_superseded(outer)?;
    if let Some(inner) = layers.get(&ManifestLayerIdV3::InnerReceipt) {
        let inner_binding = binding
            .manifest_layers
            .iter()
            .find(|layer| layer.layer_id == ManifestLayerIdV3::InnerReceipt)
            .ok_or_else(|| invalid("inner manifest binding is absent"))?;
        let nested_path = Path::new(&inner_binding.root_relative_path)
            .join(&inner_binding.manifest_relative_path)
            .to_str()
            .ok_or_else(|| invalid("nested manifest path is not UTF-8"))?
            .to_string();
        outer.require_hash(&nested_path, &inner_binding.manifest_sha256)?;
        inner.reverify()?;
    }
    if let Some(required) = required_file {
        let bundle = outer
            .entry(&required.relative_path)
            .ok_or_else(|| invalid("candidate bundle is absent from portable inputs"))?;
        if bundle.sha256 != required.sha256 || bundle.size_bytes != required.size_bytes {
            return Err(invalid(
                "candidate bundle differs from the exact 52ec materialization pin",
            ));
        }
    }
    validate_required_artifacts(binding, profile, &layers)?;
    let original = validate_receipt_provenance(binding, &layers, receipts_parent)?;
    reject_conflicting_terminal_evidence(profile, &layers, &binding.provenance)?;
    let verified = VerifiedReceipt {
        layers,
        original,
        profile,
    };
    verified.reverify()?;
    Ok(verified)
}

fn validate_frozen_receipt_identity(
    binding: &ReceiptEvidenceBindingV3,
    profile: EvidenceProfileV3,
) -> Result<(), AcceptanceError> {
    let identity = profiles::frozen_receipt_identity(profile).ok_or_else(|| {
        invalid(format!(
            "PROFILE_IDENTITY_UNPINNED: final {profile:?} receipt identity is not frozen"
        ))
    })?;
    if binding.receipt_root != identity.receipt_root {
        return Err(invalid(
            "receipt root differs from the compiled exact identity",
        ));
    }
    let outer = binding
        .manifest_layers
        .iter()
        .find(|layer| layer.layer_id == ManifestLayerIdV3::Outer)
        .ok_or_else(|| invalid("receipt outer layer is absent"))?;
    validate_frozen_layer(outer, identity.outer)?;
    match (
        identity.inner,
        binding
            .manifest_layers
            .iter()
            .find(|layer| layer.layer_id == ManifestLayerIdV3::InnerReceipt),
    ) {
        (Some(expected), Some(actual)) => validate_frozen_layer(actual, expected),
        (None, None) => Ok(()),
        _ => Err(invalid(
            "receipt inner identity differs from the compiled exact topology",
        )),
    }
}

fn validate_frozen_layer(
    layer: &ManifestLayerBindingV3,
    expected: profiles::FrozenReceiptLayerV3,
) -> Result<(), AcceptanceError> {
    if layer.manifest_entry_count != expected.entry_count
        || layer.manifest_sha256 != expected.manifest_sha256
        || layer.mode_manifest.sha256 != expected.mode_sha256
    {
        return Err(invalid(
            "receipt manifest or mode identity differs from the compiled exact pin",
        ));
    }
    Ok(())
}

#[cfg(test)]
pub(super) fn validate_receipt_for_test(
    binding: &ReceiptEvidenceBindingV3,
    receipts_parent: &Path,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    let verified = validate_receipt(binding, binding.profile, receipts_parent, None)?;
    match binding.profile {
        EvidenceProfileV3::CanonicalPathTrustV2
        | EvidenceProfileV3::PortableInputsV1
        | EvidenceProfileV3::UpstreamCutoffObservationV1 => {
            observe_prerequisite(binding.profile, &verified, candidate)?;
        }
        EvidenceProfileV3::MacExactV6
        | EvidenceProfileV3::LinuxExactV5
        | EvidenceProfileV3::NixExactV3
        | EvidenceProfileV3::WindowsNativeV6 => {
            observe_gate(binding.profile, &verified, candidate)?;
        }
        EvidenceProfileV3::GithubHostedExactV2 => {
            observe_github_hosted(&verified, candidate)?;
        }
    }
    verified.reverify()
}

fn validate_required_artifacts(
    binding: &ReceiptEvidenceBindingV3,
    profile: EvidenceProfileV3,
    layers: &BTreeMap<ManifestLayerIdV3, VerifiedManifest>,
) -> Result<(), AcceptanceError> {
    let expected = profiles::required_artifacts(profile);
    if binding.required_artifacts.len() != expected.len() {
        return Err(invalid(
            "receipt does not pin the complete compiled artifact roster",
        ));
    }
    for (artifact, compiled) in binding.required_artifacts.iter().zip(expected) {
        if artifact.layer_id != compiled.layer || artifact.relative_path != compiled.path {
            return Err(invalid(
                "receipt artifact binding differs from the compiled roster",
            ));
        }
        validate_artifact_binding(artifact, layers)?;
    }
    Ok(())
}

fn validate_artifact_binding(
    artifact: &ArtifactBindingV3,
    layers: &BTreeMap<ManifestLayerIdV3, VerifiedManifest>,
) -> Result<(), AcceptanceError> {
    validate_relative_path(&artifact.relative_path)?;
    if !digest_shape(&artifact.sha256) {
        return Err(invalid("required artifact digest is malformed"));
    }
    let entry = layers
        .get(&artifact.layer_id)
        .ok_or_else(|| invalid("required artifact layer is absent"))?
        .entry(&artifact.relative_path)
        .ok_or_else(|| invalid("required artifact is absent from its sealed layer"))?;
    if entry.sha256 != artifact.sha256 || entry.size_bytes != artifact.size_bytes {
        return Err(invalid(
            "required artifact differs from its externally pinned digest or size",
        ));
    }
    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct ReemissionAttestationV3 {
    authority_projection_byte_identical: bool,
    dealiased: bool,
    hardlink_topology_sha256: String,
    original_inventory_sha256: String,
    original_extended_metadata_inventory_sha256: String,
    original_metadata_inventory_sha256: String,
    original_manifest_entry_count: usize,
    original_manifest_relative_path: String,
    original_manifest_sha256: String,
    original_receipt_root: String,
    original_reverified_after: bool,
    original_reverified_before: bool,
    post_original_inventory_sha256: String,
    pre_original_inventory_sha256: String,
    projection_map_sha256: String,
    reemitter_sha256: String,
    schema: String,
}

#[cfg(test)]
pub(super) fn wrapper_attestation_bytes_for_test(
    original: &super::model::OriginalReceiptBindingV3,
    original_inventory_sha256: &str,
    hardlink_topology: &ArtifactBindingV3,
    original_extended_metadata_inventory: &ArtifactBindingV3,
    original_metadata_inventory: &ArtifactBindingV3,
    projection_map: &ArtifactBindingV3,
    reemitter: &ArtifactBindingV3,
) -> Result<Vec<u8>, AcceptanceError> {
    canonical_json(&ReemissionAttestationV3 {
        authority_projection_byte_identical: true,
        dealiased: true,
        hardlink_topology_sha256: hardlink_topology.sha256.clone(),
        original_inventory_sha256: original_inventory_sha256.to_string(),
        original_extended_metadata_inventory_sha256: original_extended_metadata_inventory
            .sha256
            .clone(),
        original_metadata_inventory_sha256: original_metadata_inventory.sha256.clone(),
        original_manifest_entry_count: original.manifest_entry_count,
        original_manifest_relative_path: original.manifest_relative_path.clone(),
        original_manifest_sha256: original.manifest_sha256.clone(),
        original_receipt_root: original.receipt_root.clone(),
        original_reverified_after: true,
        original_reverified_before: true,
        post_original_inventory_sha256: original_inventory_sha256.to_string(),
        pre_original_inventory_sha256: original_inventory_sha256.to_string(),
        projection_map_sha256: projection_map.sha256.clone(),
        reemitter_sha256: reemitter.sha256.clone(),
        schema: "hepta_vnext_provenance_reemission_v1".to_string(),
    })
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ProjectionRow {
    kind: String,
    original: String,
    sha256: String,
    size_bytes: u64,
    wrapper: String,
}

fn validate_receipt_provenance(
    binding: &ReceiptEvidenceBindingV3,
    layers: &BTreeMap<ManifestLayerIdV3, VerifiedManifest>,
    receipts_parent: &Path,
) -> Result<Option<crate::manifest_inventory::LegacyManifestSnapshot>, AcceptanceError> {
    validate_receipt_provenance_with_identity(
        binding,
        layers,
        receipts_parent,
        profiles::frozen_original_identity(binding.profile),
        &mut || Ok(()),
    )
}

fn validate_receipt_provenance_with_identity(
    binding: &ReceiptEvidenceBindingV3,
    layers: &BTreeMap<ManifestLayerIdV3, VerifiedManifest>,
    receipts_parent: &Path,
    frozen_original: Option<profiles::FrozenOriginalIdentityV3>,
    before_final_original_reverification: &mut dyn FnMut() -> Result<(), AcceptanceError>,
) -> Result<Option<crate::manifest_inventory::LegacyManifestSnapshot>, AcceptanceError> {
    let ReceiptProvenanceV3::ReemittedWrapper {
        attestation,
        hardlink_topology,
        original,
        original_extended_metadata_inventory,
        original_metadata_inventory,
        original_tree_relative_path,
        projection_map,
        reemitter,
    } = &binding.provenance
    else {
        if !profiles::direct_provenance_allowed(binding.profile) {
            return Err(invalid(
                "this compiled profile requires provenance-preserving re-emission",
            ));
        }
        if layers.values().any(|manifest| {
            manifest
                .entry_paths()
                .any(|path| path == "provenance" || path.starts_with("provenance/"))
                || manifest
                    .directory_paths()
                    .any(|path| path == "provenance" || path.starts_with("provenance/"))
        }) {
            return Err(invalid(
                "direct receipt cannot carry the wrapper provenance namespace",
            ));
        }
        return Ok(None);
    };
    let frozen_original = frozen_original.ok_or_else(|| {
        invalid(format!(
            "PROFILE_IDENTITY_UNPINNED: original {:?} receipt identity is not frozen",
            binding.profile
        ))
    })?;
    if original.manifest_entry_count != frozen_original.entry_count
        || original.manifest_relative_path != frozen_original.manifest_relative_path
        || original.manifest_sha256 != frozen_original.manifest_sha256
        || original.receipt_root != frozen_original.receipt_root
    {
        return Err(invalid(
            "wrapper original receipt differs from the compiled exact identity",
        ));
    }
    if !matches!(
        binding.profile,
        EvidenceProfileV3::MacExactV6
            | EvidenceProfileV3::NixExactV3
            | EvidenceProfileV3::PortableInputsV1
    ) {
        return Err(invalid(
            "this compiled profile does not permit provenance re-emission",
        ));
    }
    if original_tree_relative_path != "provenance/original-tree" {
        return Err(invalid(
            "wrapper original-tree path differs from its profile",
        ));
    }
    for (artifact, path) in [
        (attestation, "provenance/reemission-attestation.json"),
        (hardlink_topology, "provenance/hardlink-topology.tsv"),
        (
            original_extended_metadata_inventory,
            "provenance/original-extended-metadata.tsv",
        ),
        (
            original_metadata_inventory,
            "provenance/original-metadata.tsv",
        ),
        (projection_map, "provenance/projection-map.tsv"),
        (reemitter, "provenance/reemitter"),
    ] {
        if artifact.layer_id != ManifestLayerIdV3::Outer || artifact.relative_path != path {
            return Err(invalid(
                "wrapper provenance artifact differs from its compiled path",
            ));
        }
        validate_artifact_binding(artifact, layers)?;
    }
    validate_relative_path(&original.manifest_relative_path)?;
    if original.manifest_entry_count == 0 || !digest_shape(&original.manifest_sha256) {
        return Err(invalid("original receipt seal binding is malformed"));
    }
    let original_root = Path::new(&original.receipt_root);
    let wrapper_root = Path::new(&binding.receipt_root);
    if !original_root.starts_with(receipts_parent)
        || original_root == receipts_parent
        || original_root == wrapper_root
        || original_root.starts_with(wrapper_root)
        || wrapper_root.starts_with(original_root)
    {
        return Err(invalid(
            "wrapper original root must be a disjoint receipt-store child",
        ));
    }

    let extended_metadata_policy = match binding.profile {
        EvidenceProfileV3::MacExactV6 => LegacyExtendedMetadataPolicy::MacAttempt2,
        EvidenceProfileV3::PortableInputsV1 => LegacyExtendedMetadataPolicy::PortableInputs,
        _ => LegacyExtendedMetadataPolicy::None,
    };
    let before = load_legacy_manifest_with_policy(
        original_root,
        &original.manifest_relative_path,
        &original.manifest_sha256,
        original.manifest_entry_count,
        extended_metadata_policy,
    )?;
    verify_frozen_original_provenance(&before, frozen_original)?;
    let outer = layers
        .get(&ManifestLayerIdV3::Outer)
        .ok_or_else(|| invalid("wrapper outer layer is absent"))?;
    let topology_bytes = outer.bytes(&hardlink_topology.relative_path)?;
    if topology_bytes != before.hardlink_topology {
        return Err(invalid(
            "wrapper hardlink topology differs from the original receipt",
        ));
    }
    if outer.bytes(&original_metadata_inventory.relative_path)? != before.metadata_inventory {
        return Err(invalid(
            "wrapper original metadata inventory differs from the live original receipt",
        ));
    }
    if outer.bytes(&original_extended_metadata_inventory.relative_path)?
        != before.extended_metadata_inventory
    {
        return Err(invalid(
            "wrapper original extended metadata inventory differs from the live original receipt",
        ));
    }
    let rows = parse_projection_map(&outer.bytes(&projection_map.relative_path)?)?;
    verify_wrapper_projection(binding, layers, &before, &rows, original_tree_relative_path)?;

    let attestation_bytes = outer.bytes(&attestation.relative_path)?;
    let attestation_value = super::strict_json::parse(&attestation_bytes)?;
    let attested: ReemissionAttestationV3 = serde_json::from_value(attestation_value)
        .map_err(|error| invalid(format!("invalid wrapper attestation: {error}")))?;
    if canonical_json(&attested)? != attestation_bytes {
        return Err(invalid("wrapper attestation is not canonical JSON"));
    }
    let inventory_sha256 = sha256(&before.inventory);
    let expected_attestation = ReemissionAttestationV3 {
        authority_projection_byte_identical: true,
        dealiased: true,
        hardlink_topology_sha256: hardlink_topology.sha256.clone(),
        original_inventory_sha256: inventory_sha256.clone(),
        original_extended_metadata_inventory_sha256: original_extended_metadata_inventory
            .sha256
            .clone(),
        original_metadata_inventory_sha256: original_metadata_inventory.sha256.clone(),
        original_manifest_entry_count: original.manifest_entry_count,
        original_manifest_relative_path: original.manifest_relative_path.clone(),
        original_manifest_sha256: original.manifest_sha256.clone(),
        original_receipt_root: original.receipt_root.clone(),
        original_reverified_after: true,
        original_reverified_before: true,
        post_original_inventory_sha256: inventory_sha256.clone(),
        pre_original_inventory_sha256: inventory_sha256,
        projection_map_sha256: projection_map.sha256.clone(),
        reemitter_sha256: reemitter.sha256.clone(),
        schema: "hepta_vnext_provenance_reemission_v1".to_string(),
    };
    if canonical_json(&expected_attestation)? != attestation_bytes {
        return Err(invalid(
            "wrapper attestation differs from independently verified provenance",
        ));
    }
    before_final_original_reverification()?;
    let after = before.reverify()?;
    if after.inventory != before.inventory
        || after.metadata_inventory != before.metadata_inventory
        || after.extended_metadata_inventory != before.extended_metadata_inventory
        || after.hardlink_topology != before.hardlink_topology
    {
        return Err(invalid(
            "original receipt changed between wrapper provenance checks",
        ));
    }
    Ok(Some(after))
}

fn verify_frozen_original_provenance(
    original: &crate::manifest_inventory::LegacyManifestSnapshot,
    frozen: profiles::FrozenOriginalIdentityV3,
) -> Result<(), AcceptanceError> {
    for (name, bytes, identity) in [
        (
            "metadata inventory",
            original.metadata_inventory.as_slice(),
            frozen.metadata_inventory,
        ),
        (
            "hardlink topology",
            original.hardlink_topology.as_slice(),
            frozen.hardlink_topology,
        ),
        (
            "extended metadata inventory",
            original.extended_metadata_inventory.as_slice(),
            frozen.extended_metadata_inventory,
        ),
    ] {
        let rows = bytes.iter().filter(|byte| **byte == b'\n').count();
        if bytes.len() != identity.size_bytes
            || rows != identity.row_count
            || sha256(bytes) != identity.sha256
        {
            return Err(invalid(format!(
                "original {name} differs from its compiled exact identity"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
pub(super) fn verify_frozen_original_provenance_for_test(
    original: &crate::manifest_inventory::LegacyManifestSnapshot,
    frozen: profiles::FrozenOriginalIdentityV3,
) -> Result<(), AcceptanceError> {
    verify_frozen_original_provenance(original, frozen)
}

#[cfg(test)]
pub(super) fn validate_reemitted_wrapper_for_test(
    binding: &ReceiptEvidenceBindingV3,
    receipts_parent: &Path,
    frozen_original: profiles::FrozenOriginalIdentityV3,
    mut before_final_original_reverification: impl FnMut() -> Result<(), AcceptanceError>,
) -> Result<(), AcceptanceError> {
    let mut layers = BTreeMap::new();
    for (source, expected_id) in binding
        .manifest_layers
        .iter()
        .zip(profiles::expected_layers(binding.profile))
    {
        validate_layer_binding(binding.profile, source, *expected_id)?;
        let manifest = load_layer(Path::new(&binding.receipt_root), source)?;
        verify_mode_manifest(
            source,
            &manifest,
            &compiled_visible_mode_paths(binding.profile, *expected_id)?,
        )?;
        layers.insert(*expected_id, manifest);
    }
    validate_receipt_provenance_with_identity(
        binding,
        &layers,
        receipts_parent,
        Some(frozen_original),
        &mut before_final_original_reverification,
    )?;
    Ok(())
}

fn parse_projection_map(bytes: &[u8]) -> Result<BTreeSet<ProjectionRow>, AcceptanceError> {
    let text =
        std::str::from_utf8(bytes).map_err(|_| invalid("wrapper projection map is not UTF-8"))?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(invalid(
            "wrapper projection map must be nonempty and newline terminated",
        ));
    }
    let mut rows = BTreeSet::new();
    let mut previous: Option<ProjectionRow> = None;
    for line in text.lines() {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 5 || !matches!(fields[0], "archive" | "canonical") {
            return Err(invalid("wrapper projection row is malformed"));
        }
        let original = fields[1]
            .strip_prefix("./")
            .ok_or_else(|| invalid("wrapper projection source lacks ./"))?;
        let wrapper = fields[4]
            .strip_prefix("./")
            .ok_or_else(|| invalid("wrapper projection destination lacks ./"))?;
        validate_relative_path(original)?;
        validate_relative_path(wrapper)?;
        if !digest_shape(fields[2]) {
            return Err(invalid("wrapper projection digest is malformed"));
        }
        let size_bytes = fields[3]
            .parse::<u64>()
            .map_err(|_| invalid("wrapper projection size is malformed"))?;
        if size_bytes.to_string() != fields[3] {
            return Err(invalid("wrapper projection size is not canonical"));
        }
        let row = ProjectionRow {
            kind: fields[0].to_string(),
            original: original.to_string(),
            sha256: fields[2].to_string(),
            size_bytes,
            wrapper: wrapper.to_string(),
        };
        if previous.as_ref().is_some_and(|value| value >= &row) || !rows.insert(row.clone()) {
            return Err(invalid(
                "wrapper projection rows must be unique and strictly sorted",
            ));
        }
        previous = Some(row);
    }
    Ok(rows)
}

fn verify_wrapper_projection(
    binding: &ReceiptEvidenceBindingV3,
    layers: &BTreeMap<ManifestLayerIdV3, VerifiedManifest>,
    original: &crate::manifest_inventory::LegacyManifestSnapshot,
    rows: &BTreeSet<ProjectionRow>,
    original_tree: &str,
) -> Result<(), AcceptanceError> {
    let outer = layers
        .get(&ManifestLayerIdV3::Outer)
        .ok_or_else(|| invalid("wrapper outer layer is absent"))?;
    let expected_archive = original
        .entries
        .iter()
        .map(|(path, entry)| ProjectionRow {
            kind: "archive".to_string(),
            original: path.clone(),
            sha256: entry.sha256.clone(),
            size_bytes: entry.size_bytes,
            wrapper: format!("{original_tree}/{path}"),
        })
        .collect::<BTreeSet<_>>();
    let actual_archive = rows
        .iter()
        .filter(|row| row.kind == "archive")
        .cloned()
        .collect::<BTreeSet<_>>();
    if actual_archive != expected_archive {
        return Err(invalid(
            "wrapper archive projection does not cover the exact original receipt",
        ));
    }
    for row in &expected_archive {
        let entry = outer
            .entry(&row.wrapper)
            .ok_or_else(|| invalid("wrapper original-tree copy is absent"))?;
        if entry.sha256 != row.sha256 || entry.size_bytes != row.size_bytes {
            return Err(invalid(
                "wrapper original-tree copy differs from the original receipt",
            ));
        }
    }
    let mut expected_provenance_files = [
        "provenance/reemission-attestation.json",
        "provenance/hardlink-topology.tsv",
        "provenance/original-extended-metadata.tsv",
        "provenance/original-metadata.tsv",
        "provenance/projection-map.tsv",
        "provenance/reemitter",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    expected_provenance_files.extend(expected_archive.iter().map(|row| row.wrapper.clone()));
    let actual_provenance_files = outer
        .entry_paths()
        .filter(|path| path.starts_with("provenance/"))
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if actual_provenance_files != expected_provenance_files {
        return Err(invalid(
            "wrapper provenance file namespace differs from its exact closure",
        ));
    }
    let mut expected_provenance_directories = BTreeSet::from([
        "provenance".to_string(),
        "provenance/original-tree".to_string(),
    ]);
    for path in &expected_provenance_files {
        let mut parent = Path::new(path).parent();
        while let Some(directory) = parent {
            let directory = directory
                .to_str()
                .ok_or_else(|| invalid("wrapper provenance directory is not UTF-8"))?;
            if directory.is_empty() || !directory.starts_with("provenance") {
                break;
            }
            expected_provenance_directories.insert(directory.to_string());
            parent = Path::new(directory).parent();
        }
    }
    let actual_provenance_directories = outer
        .directory_paths()
        .filter(|path| *path == "provenance" || path.starts_with("provenance/"))
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if actual_provenance_directories != expected_provenance_directories {
        return Err(invalid(
            "wrapper provenance directory namespace differs from its exact closure",
        ));
    }

    let mut generated = BTreeSet::new();
    for layer in &binding.manifest_layers {
        let prefix = (layer.root_relative_path != ".")
            .then_some(format!("{}/", layer.root_relative_path))
            .unwrap_or_default();
        generated.insert(format!("{prefix}{}", layer.manifest_relative_path));
        generated.insert(format!("{prefix}{}", layer.mode_manifest.relative_path));
    }
    let canonical_files = outer
        .entry_paths()
        .map(str::to_string)
        .chain(std::iter::once(outer.manifest_relative_path().to_string()))
        .filter(|path| !path.starts_with("provenance/") && !generated.contains(path))
        .collect::<BTreeSet<_>>();
    let mut expected_canonical = BTreeSet::new();
    for path in canonical_files {
        let source = original.entries.get(&path).ok_or_else(|| {
            invalid(format!(
                "wrapper adds a non-provenance canonical artifact: {path}"
            ))
        })?;
        let target = outer
            .entry(&path)
            .ok_or_else(|| invalid("wrapper canonical artifact is absent"))?;
        if source != target {
            return Err(invalid(
                "wrapper canonical projection changes original artifact bytes",
            ));
        }
        expected_canonical.insert(ProjectionRow {
            kind: "canonical".to_string(),
            original: path.clone(),
            sha256: source.sha256.clone(),
            size_bytes: source.size_bytes,
            wrapper: path,
        });
    }
    let actual_canonical = rows
        .iter()
        .filter(|row| row.kind == "canonical")
        .cloned()
        .collect::<BTreeSet<_>>();
    if actual_canonical != expected_canonical {
        return Err(invalid(
            "wrapper canonical projection map is incomplete or noncanonical",
        ));
    }
    Ok(())
}

fn reject_conflicting_terminal_evidence(
    _profile: EvidenceProfileV3,
    layers: &BTreeMap<ManifestLayerIdV3, VerifiedManifest>,
    _provenance: &ReceiptProvenanceV3,
) -> Result<(), AcceptanceError> {
    for manifest in layers.values() {
        for path in manifest.entry_paths().chain(manifest.directory_paths()) {
            let name = Path::new(path)
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            if name == "prior-partial.txt"
                || name.contains("superseded")
                || name.starts_with("blocked")
                || name.starts_with("interrupted")
                || name.starts_with("failed")
                || name.starts_with("failure")
            {
                return Err(invalid(
                    "PASS evidence contains a conflicting terminal marker",
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
pub(super) fn reject_conflicting_markers_for_test(
    root: &Path,
    manifest_relative_path: &str,
    manifest_sha256: &str,
    manifest_entry_count: usize,
) -> Result<(), AcceptanceError> {
    let manifest = VerifiedManifest::load_named(
        root,
        manifest_relative_path,
        manifest_sha256,
        manifest_entry_count,
    )?;
    let layers = [(ManifestLayerIdV3::Outer, manifest)].into_iter().collect();
    reject_conflicting_terminal_evidence(
        EvidenceProfileV3::CanonicalPathTrustV2,
        &layers,
        &ReceiptProvenanceV3::Direct,
    )
}

fn validate_layer_binding(
    profile: EvidenceProfileV3,
    binding: &ManifestLayerBindingV3,
    expected_id: ManifestLayerIdV3,
) -> Result<(), AcceptanceError> {
    let compiled = profiles::layer_profile(profile, expected_id)
        .ok_or_else(|| invalid("compiled layer profile is absent"))?;
    if binding.layer_id != compiled.id
        || binding.root_relative_path != compiled.root
        || binding.manifest_relative_path != compiled.manifest_path
        || binding.mode_manifest.relative_path != compiled.mode_path
        || binding.mode_manifest.format != compiled.mode_format
        || binding.manifest_entry_count == 0
        || !digest_shape(&binding.manifest_sha256)
        || !digest_shape(&binding.mode_manifest.sha256)
    {
        return Err(invalid("manifest layer differs from its compiled profile"));
    }
    validate_relative_path(&binding.manifest_relative_path)?;
    validate_relative_path(&binding.mode_manifest.relative_path)?;
    if binding.root_relative_path != "." {
        validate_relative_path(&binding.root_relative_path)?;
    }
    Ok(())
}

fn load_layer(
    receipt_root: &Path,
    layer: &ManifestLayerBindingV3,
) -> Result<VerifiedManifest, AcceptanceError> {
    let root = if layer.root_relative_path == "." {
        receipt_root.to_path_buf()
    } else {
        receipt_root.join(&layer.root_relative_path)
    };
    VerifiedManifest::load_named(
        &root,
        &layer.manifest_relative_path,
        &layer.manifest_sha256,
        layer.manifest_entry_count,
    )
}

fn reject_superseded(manifest: &VerifiedManifest) -> Result<(), AcceptanceError> {
    if manifest.entry_paths().any(|path| {
        Path::new(path).file_name().and_then(|value| value.to_str()) == Some("SUPERSEDED.txt")
    }) || manifest.directory_paths().any(|path| {
        Path::new(path).file_name().and_then(|value| value.to_str()) == Some("SUPERSEDED.txt")
    }) {
        return Err(invalid("receipt contains a SUPERSEDED.txt marker"));
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ModeKind {
    Directory,
    File,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ModeRow {
    kind: ModeKind,
    mode: Option<u32>,
    size: Option<u64>,
}

pub(super) fn compiled_visible_mode_paths(
    profile: EvidenceProfileV3,
    current_layer: ManifestLayerIdV3,
) -> Result<BTreeSet<String>, AcceptanceError> {
    let mut paths = BTreeSet::new();
    for layer in profiles::expected_layers(profile) {
        let compiled = profiles::layer_profile(profile, *layer)
            .ok_or_else(|| invalid("compiled mode layer profile is absent"))?;
        let visible = if *layer == current_layer {
            Some(compiled.mode_path.to_string())
        } else if current_layer == ManifestLayerIdV3::Outer && compiled.root != "." {
            Some(
                Path::new(compiled.root)
                    .join(compiled.mode_path)
                    .to_str()
                    .ok_or_else(|| invalid("compiled nested mode path is not UTF-8"))?
                    .to_string(),
            )
        } else {
            None
        };
        if let Some(visible) = visible {
            paths.insert(visible);
        }
    }
    Ok(paths)
}

pub(super) fn verify_mode_manifest(
    binding: &ManifestLayerBindingV3,
    manifest: &VerifiedManifest,
    allowed_mode_paths: &BTreeSet<String>,
) -> Result<(), AcceptanceError> {
    if !allowed_mode_paths.contains(&binding.mode_manifest.relative_path)
        || allowed_mode_paths
            .iter()
            .any(|relative| manifest.entry(relative).is_none())
    {
        return Err(invalid(
            "compiled metadata inventory is absent from its visible sealed layer",
        ));
    }
    manifest.require_hash(
        &binding.mode_manifest.relative_path,
        &binding.mode_manifest.sha256,
    )?;
    let rows = parse_mode_rows(
        &manifest.bytes(&binding.mode_manifest.relative_path)?,
        binding.mode_manifest.format,
    )?;

    let expected_files = manifest
        .entry_paths()
        .map(str::to_string)
        .chain(std::iter::once(
            manifest.manifest_relative_path().to_string(),
        ))
        .collect::<BTreeSet<_>>();
    let actual_files = rows
        .iter()
        .filter(|(_, row)| row.kind == ModeKind::File)
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    if actual_files != expected_files {
        return Err(invalid(
            "mode manifest file rows do not exactly cover the sealed layer",
        ));
    }
    let expected_directories = manifest
        .directory_paths()
        .map(|path| if path.is_empty() { "." } else { path }.to_string())
        .collect::<BTreeSet<_>>();
    let actual_directories = rows
        .iter()
        .filter(|(_, row)| row.kind == ModeKind::Directory)
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    if actual_directories != expected_directories {
        return Err(invalid(
            "mode manifest directory rows do not exactly cover the sealed layer",
        ));
    }
    for directory in expected_directories
        .iter()
        .filter(|path| path.as_str() != ".")
    {
        let directory = Path::new(directory);
        let has_immediate_child = expected_files
            .iter()
            .map(Path::new)
            .chain(
                expected_directories
                    .iter()
                    .filter(|path| path.as_str() != ".")
                    .map(Path::new),
            )
            .any(|path| path != directory && path.parent() == Some(directory));
        if !has_immediate_child {
            return Err(invalid(
                "mode manifest contains an empty evidence directory",
            ));
        }
    }

    for (relative, row) in rows {
        let target = if relative == "." {
            manifest.root.clone()
        } else {
            manifest.root.join(&relative)
        };
        let metadata = std::fs::symlink_metadata(&target)?;
        if metadata.file_type().is_symlink()
            || (row.kind == ModeKind::File && !metadata.is_file())
            || (row.kind == ModeKind::Directory && !metadata.is_dir())
        {
            return Err(invalid("mode manifest type differs from the receipt"));
        }
        if row.size.is_some_and(|expected| metadata.len() != expected) {
            return Err(invalid("mode manifest size differs from the receipt"));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if let Some(expected) = row.mode {
                let actual = metadata.mode() & 0o7777;
                if actual & 0o7000 != 0 || actual != expected {
                    return Err(invalid(
                        "mode manifest mode or special bits differ from the receipt",
                    ));
                }
            }
        }
    }
    manifest.reverify()
}

pub(super) fn parse_mode_rows(
    bytes: &[u8],
    format: ModeManifestFormatV3,
) -> Result<BTreeMap<String, ModeRow>, AcceptanceError> {
    let text = std::str::from_utf8(bytes).map_err(|_| invalid("mode manifest is not UTF-8"))?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(invalid(
            "mode manifest must be nonempty and newline terminated",
        ));
    }
    let mut rows = BTreeMap::new();
    let mut previous: Option<String> = None;
    for line in text.lines() {
        if line.contains('\r') {
            return Err(invalid("mode manifest contains a carriage return"));
        }
        let (kind, mode, size, raw_path) = match format {
            ModeManifestFormatV3::TypedPosixModeSizePathTsvV2 => {
                let fields = line.split('\t').collect::<Vec<_>>();
                if fields.len() != 4 {
                    return Err(invalid("typed mode/size/path manifest row is malformed"));
                }
                let size = match fields[0] {
                    "Regular File" => Some(fields[2]),
                    "Directory" if fields[2] == "-" => None,
                    _ => {
                        return Err(invalid(
                            "typed mode/size/path manifest type or size is malformed",
                        ));
                    }
                };
                (fields[0], Some(fields[1]), size, fields[3])
            }
            ModeManifestFormatV3::WindowsNtfsTypeSizePathTsvV1 => {
                let fields = line.split('\t').collect::<Vec<_>>();
                if fields.len() != 3 {
                    return Err(invalid("Windows type/size/path inventory row is malformed"));
                }
                let size = match fields[0] {
                    "Regular File" => Some(fields[1]),
                    "Directory" if fields[1] == "-" => None,
                    _ => {
                        return Err(invalid(
                            "Windows type/size/path inventory type or size is malformed",
                        ));
                    }
                };
                (fields[0], None, size, fields[2])
            }
        };
        let relative = if raw_path == "." {
            "."
        } else {
            raw_path
                .strip_prefix("./")
                .ok_or_else(|| invalid("metadata inventory path lacks the exact ./ prefix"))?
        };
        if relative != "." {
            validate_relative_path(relative)?;
        }
        if previous.as_deref().is_some_and(|value| value >= relative) {
            return Err(invalid(
                "mode manifest paths must be unique and strictly sorted",
            ));
        }
        previous = Some(relative.to_string());
        let kind = match kind {
            "Regular File" => ModeKind::File,
            "Directory" => ModeKind::Directory,
            _ => return Err(invalid("typed metadata manifest contains an unknown type")),
        };
        let mode = mode
            .map(|value| {
                let parsed = u32::from_str_radix(value, 8)
                    .ok()
                    .filter(|value| *value <= 0o7777 && value & 0o7000 == 0)
                    .ok_or_else(|| {
                        invalid("mode manifest contains invalid or special mode bits")
                    })?;
                if format!("{parsed:o}") != value {
                    return Err(invalid("mode manifest mode is not canonical octal"));
                }
                Ok(parsed)
            })
            .transpose()?;
        let size = size
            .map(|value| {
                let parsed = value
                    .parse::<u64>()
                    .map_err(|_| invalid("mode manifest size is malformed"))?;
                if parsed.to_string() != value {
                    return Err(invalid("mode manifest size is not canonical decimal"));
                }
                Ok(parsed)
            })
            .transpose()?;
        if rows
            .insert(relative.to_string(), ModeRow { kind, mode, size })
            .is_some()
        {
            return Err(invalid("mode manifest contains a duplicate path"));
        }
    }
    Ok(rows)
}

fn observe_gate(
    profile: EvidenceProfileV3,
    receipt: &VerifiedReceipt,
    candidate: &CandidateBindingV3,
) -> Result<ObservedGateV3, AcceptanceError> {
    match profile {
        EvidenceProfileV3::MacExactV6 => observe_mac(receipt, candidate),
        EvidenceProfileV3::LinuxExactV5 => observe_linux(receipt, candidate),
        EvidenceProfileV3::NixExactV3 => observe_nix(receipt, candidate),
        EvidenceProfileV3::WindowsNativeV6 => observe_windows(receipt, candidate),
        EvidenceProfileV3::GithubHostedExactV2 => observe_github_hosted(receipt, candidate),
        _ => Err(invalid("profile is not a compiled platform gate")),
    }
}

fn observe_mac(
    receipt: &VerifiedReceipt,
    candidate: &CandidateBindingV3,
) -> Result<ObservedGateV3, AcceptanceError> {
    let artifact = kv_artifact(
        receipt,
        EvidenceProfileV3::MacExactV6,
        ManifestLayerIdV3::Outer,
    )?;
    require_exact_kv_fields(
        &artifact,
        &[
            "schema",
            "status",
            "candidate_commit",
            "candidate_tree",
            "candidate_parent",
            "integration_merge",
            "upstream_cutoff",
            "upstream_cutoff_exact",
            "worktree_clean",
            "exact_phase_count",
            "exact_phases_all_pass",
            "hepta_package_tests",
            "hepta_package_failures",
            "strict_clippy_hepta_packages",
            "strict_clippy_all_targets",
            "product_caller_builds",
            "targeted_filter_count",
            "targeted_filters_nonzero",
            "upstream_targeted_tests",
            "upstream_targeted_failures",
            "guardian_baseline_exclusion",
            "guardian_baseline_head",
            "guardian_excluded_test_candidate_failure",
            "bazel_command",
            "bazel_lock_check",
            "bazel_target_tests",
            "bazel_target_failures",
            "caller_ledger_bazel_test",
            "bazel_product_build",
            "bazel_action_cache_allowed",
            "mac_binary_sha256",
            "mac_binary_size_bytes",
            "mac_binary_built_from_exact_head",
            "mac_binary_inherited",
            "target_release_inherited",
            "immutable_release_self_test",
            "state_snapshot_full_state_self_test",
            "generation_pointer_self_test",
            "launchd_bridge_epoch_recutover_self_test",
            "launchd_bridge_self_test_schema",
            "launchd_bridge_generation_specific_health_routes",
            "launchd_bridge_legacy_health_route_verified",
            "launchd_bridge_vnext_health_route_verified",
            "launchd_bridge_health_failure_source_restored",
            "launchd_bridge_health_paths_recorded_in_transition_receipts",
            "launchd_bridge_health_paths_recorded_in_recovery_receipts",
            "fixture_full_state_snapshot_v2",
            "fixture_full_state_metadata_preserved",
            "fixture_full_state_hardlinks_preserved",
            "fixture_v1_downgrade_negative",
            "fixture_v1_downgrade_listener_started",
            "fixture_v1_downgrade_receipt_published",
            "fixture_isolated_canary",
            "fixture_isolated_canary_soak",
            "fixture_isolated_canary_authority_all_closed",
            "fixture_isolated_canary_production_service_changed",
            "snapshot_receipt_sha256",
            "target_manifest_sha256",
            "target_release_sha256s_sha256",
            "canary_receipt_sha256",
            "canary_soak_receipt_sha256",
            "v1_downgrade_verdict_sha256",
            "production_listener_pid_after_canary",
            "production_binary_path_after_canary",
            "production_binary_sha256_after_canary",
            "production_state_snapshot",
            "production_canary",
            "operator_acceptance",
            "candidate_operator_acceptance",
            "cross_platform_qualification",
            "promotion",
            "enforce",
            "outbound",
            "retirement",
            "automatic_transition",
            "default_branch_changed",
            "production_cutover",
            "qualification_scope",
            "top_level_evidence_mode_sealed",
            "fixture_and_release_payload_modes_preserved",
            "completed_at",
        ],
    )?;
    require_kv_schema(&artifact, "hepta_vnext_main_mac_validation_v6")?;
    require_kv(&artifact, "status", "pass")?;
    require_candidate_kv(&artifact, candidate, "candidate_commit", "upstream_cutoff")?;
    require_kv(&artifact, "integration_merge", &candidate.integration_merge)?;
    require_kv_bool(&artifact, "worktree_clean", true)?;
    require_kv_bool(&artifact, "upstream_cutoff_exact", true)?;
    require_kv_u64(&artifact, "exact_phase_count", 12)?;
    require_kv_bool(&artifact, "exact_phases_all_pass", true)?;
    require_kv_u64(&artifact, "hepta_package_tests", 180)?;
    require_kv_u64(&artifact, "hepta_package_failures", 0)?;
    require_kv(&artifact, "strict_clippy_hepta_packages", "pass")?;
    require_kv_bool(&artifact, "strict_clippy_all_targets", true)?;
    require_kv(&artifact, "product_caller_builds", "pass")?;
    require_kv_u64(&artifact, "targeted_filter_count", 28)?;
    require_kv_bool(&artifact, "targeted_filters_nonzero", true)?;
    require_kv_u64(&artifact, "upstream_targeted_tests", 549)?;
    require_kv_u64(&artifact, "upstream_targeted_failures", 0)?;
    require_kv(&artifact, "guardian_baseline_exclusion", "pass")?;
    require_kv(
        &artifact,
        "guardian_baseline_head",
        "09e9e9ff7fa6b6c1d129d0c7a858979823e13ae8",
    )?;
    require_kv_bool(&artifact, "guardian_excluded_test_candidate_failure", false)?;
    require_kv(
        &artifact,
        "bazel_command",
        "/Volumes/T5/hepta-vnext/bin/bazel",
    )?;
    require_kv(&artifact, "bazel_lock_check", "pass")?;
    require_kv_u64(&artifact, "bazel_target_tests", 6)?;
    require_kv_u64(&artifact, "bazel_target_failures", 0)?;
    require_kv(&artifact, "caller_ledger_bazel_test", "pass")?;
    require_kv(&artifact, "bazel_product_build", "pass")?;
    require_kv_bool(&artifact, "bazel_action_cache_allowed", true)?;
    require_kv_bool(&artifact, "mac_binary_built_from_exact_head", true)?;
    require_kv_bool(&artifact, "mac_binary_inherited", false)?;
    require_kv_bool(&artifact, "target_release_inherited", false)?;
    for field in [
        "immutable_release_self_test",
        "state_snapshot_full_state_self_test",
        "generation_pointer_self_test",
        "launchd_bridge_epoch_recutover_self_test",
        "fixture_full_state_snapshot_v2",
        "fixture_v1_downgrade_negative",
        "fixture_isolated_canary",
    ] {
        require_kv(&artifact, field, "pass")?;
    }
    require_kv(
        &artifact,
        "launchd_bridge_self_test_schema",
        "hepta_vnext_launchd_bridge_self_test_v2",
    )?;
    for field in [
        "launchd_bridge_generation_specific_health_routes",
        "launchd_bridge_legacy_health_route_verified",
        "launchd_bridge_vnext_health_route_verified",
        "launchd_bridge_health_failure_source_restored",
        "launchd_bridge_health_paths_recorded_in_transition_receipts",
        "launchd_bridge_health_paths_recorded_in_recovery_receipts",
        "fixture_full_state_metadata_preserved",
        "fixture_full_state_hardlinks_preserved",
        "fixture_isolated_canary_authority_all_closed",
        "top_level_evidence_mode_sealed",
        "fixture_and_release_payload_modes_preserved",
    ] {
        require_kv_bool(&artifact, field, true)?;
    }
    for field in [
        "fixture_v1_downgrade_listener_started",
        "fixture_v1_downgrade_receipt_published",
        "fixture_isolated_canary_production_service_changed",
    ] {
        require_kv_bool(&artifact, field, false)?;
    }
    require_kv(&artifact, "fixture_isolated_canary_soak", "3/3")?;
    require_kv(
        &artifact,
        "qualification_scope",
        "mac_exact_and_isolated_fixture_full_state_v2_only",
    )?;
    if !valid_utc_timestamp(
        artifact
            .get("completed_at")
            .ok_or_else(|| invalid("Mac completion time is absent"))?,
    ) {
        return Err(invalid("Mac completion time is not an exact UTC second"));
    }
    for field in [
        "mac_binary_sha256",
        "snapshot_receipt_sha256",
        "target_manifest_sha256",
        "target_release_sha256s_sha256",
        "canary_receipt_sha256",
        "canary_soak_receipt_sha256",
        "v1_downgrade_verdict_sha256",
        "production_binary_sha256_after_canary",
    ] {
        if !artifact.get(field).is_some_and(|value| digest_shape(value)) {
            return Err(invalid(format!("Mac digest field is malformed: {field}")));
        }
    }
    require_kv_bool(&artifact, "automatic_transition", false)?;
    require_kv_bool(&artifact, "default_branch_changed", false)?;
    require_kv_bool(&artifact, "production_cutover", false)?;
    require_kv_u64(&artifact, "production_listener_pid_after_canary", 818)?;
    require_kv(
        &artifact,
        "production_binary_path_after_canary",
        "/Users/qianqi/.local/opt/hepta/releases/1b3958e929b82a327abcd74c7293cdf5da806a5e-a6ccf13cc81f62a822beea7dc1b9aa9d61c9734728d123d4fde473969c5efaf7-7668f32e20e25fb2b79a1aff305548a0799834acb6d433f4c7f8241f94cc52ed/bin/hepta",
    )?;
    require_kv(
        &artifact,
        "production_binary_sha256_after_canary",
        "a6ccf13cc81f62a822beea7dc1b9aa9d61c9734728d123d4fde473969c5efaf7",
    )?;
    for field in [
        "production_state_snapshot",
        "production_canary",
        "operator_acceptance",
        "candidate_operator_acceptance",
        "cross_platform_qualification",
        "promotion",
        "enforce",
        "outbound",
        "retirement",
    ] {
        require_kv_bool(&artifact, field, false)?;
    }
    let layer = receipt.layer(ManifestLayerIdV3::Outer)?;
    let steps = step_tsv(
        layer,
        "PASS",
        &profiles::MAC_STEPS,
        StepPolicy::PrefixFirstFailure,
    )?;
    if steps.count() != 12 {
        return Err(invalid("Mac exact step roster is incomplete"));
    }
    verify_mac_suite_counts(layer)?;
    verify_mac_binary_and_canary(layer, &artifact, candidate)?;
    Ok(pass_gate(12, None, None))
}

fn observe_linux(
    receipt: &VerifiedReceipt,
    candidate: &CandidateBindingV3,
) -> Result<ObservedGateV3, AcceptanceError> {
    let inner = kv_artifact(
        receipt,
        EvidenceProfileV3::LinuxExactV5,
        ManifestLayerIdV3::InnerReceipt,
    )?;
    require_exact_kv_fields(
        &inner,
        &[
            "schema",
            "driver_revision",
            "linux_exact_rc",
            "qualification",
            "verdict",
            "candidate_pass",
            "candidate_fail",
            "harness_blocked",
            "harness_fail",
            "harness_preflight_pass",
            "candidate_execution_started",
            "candidate_execution_completed",
            "postflight_verified",
            "candidate_head",
            "candidate_tree",
            "candidate_parent",
            "upstream_cutoff",
            "cutover_bridge_sha256",
            "source_identity",
            "worktree_clean",
            "qualification_host",
            "qualification_observed_hostname",
            "qualification_machine_id_sha256",
            "sanitized_environment_binding_sha256",
            "sanitized_bootstrap_environment_sha256",
            "sanitized_build_environment_sha256",
            "remote_environment_sanitized",
            "build_affecting_environment_allowlist_exact",
            "bash_env_absent",
            "target_triple",
            "cargo_net_offline",
            "cargo_build_jobs",
            "cargo_incremental",
            "fresh_cargo_home",
            "fresh_cargo_target",
            "fresh_bazel_output_user_root",
            "inherited_build_results",
            "nix_same_head_terminal_pass_bound",
            "nix_pass_binding_sha256",
            "nix_attempt_inventory_sha256",
            "nix_attempt_full_inventory_sha256",
            "nix_attempt_full_inventory_entry_count",
            "nix_attempt_full_inventory_file_count",
            "nix_attempt_full_inventory_directory_count",
            "nix_attempt_full_inventory_root",
            "nix_attempt_full_inventory_pre_post_equal",
            "nix_outer_sha256sums_sha256",
            "host_tools_binding_sha256",
            "host_executables_binding_sha256",
            "driver_manifest_sha256",
            "driver_manifest_entry_count",
            "host_tools_preflight_verified",
            "host_tools_postflight_verified",
            "host_executables_preflight_verified",
            "host_executables_postflight_verified",
            "build_materialization_executables_digest_bound",
            "input_manifest_sha256",
            "tool_input_manifest_sha256",
            "rust_toolchain_archive_sha256",
            "rust_toolchain_archive_pin_sha256",
            "bazelisk_sha256",
            "bazel_sha256",
            "shellcheck_archive_sha256",
            "shellcheck_sha256",
            "shellcheck_static_runtime_closure",
            "host_qualification_lock_path",
            "host_qualification_lock_protocol",
            "host_qualification_lock_holder_pid",
            "host_qualification_lock_holder_startticks",
            "host_qualification_lock_coordinator_payload_sha256",
            "host_qualification_lock_driver_inherits_fd",
            "host_qualification_lock_evidence_sha256",
            "host_qualification_lock_exclusive_verified",
            "host_qualification_lock_held_through_inner_seal",
            "host_qualification_lock_released",
            "resource_watchdog_started",
            "resource_watchdog_completed",
            "resource_watchdog_event_observed",
            "resource_watchdog_interval_seconds",
            "resource_watchdog_started_at",
            "resource_watchdog_stopped_at",
            "candidate_window_started_at",
            "candidate_window_completed_at",
            "resource_watchdog_row_count",
            "resource_watchdog_first_observed_at",
            "resource_watchdog_last_observed_at",
            "resource_watchdog_log_sha256",
            "resource_watchdog_single_observation_writer",
            "resource_watchdog_host_lock_verified_each_sample",
            "resource_watchdog_boundary_request_count",
            "resource_watchdog_boundary_ack_count",
            "expected_suite_counts_sha256",
            "expected_suite_counts_inner_copy_sha256",
            "expected_suite_count_rows",
            "expected_suite_count_total",
            "data_deleted",
            "promotion_authority",
            "linux_nonce",
            "nix_nonce",
            "remote_attempt_root",
            "remote_input_root",
            "remote_driver_root",
            "remote_run_root",
            "completed_at",
        ],
    )?;
    require_kv_schema(&inner, "hepta_vnext_linux_exact_result_v3")?;
    require_kv_u64(&inner, "driver_revision", 5)?;
    require_kv_u64(&inner, "linux_exact_rc", 0)?;
    require_candidate_kv(&inner, candidate, "candidate_head", "upstream_cutoff")?;
    let outer = outer_kv(receipt, EvidenceProfileV3::LinuxExactV5)?;
    require_exact_kv_fields(
        &outer,
        &[
            "schema",
            "driver_revision",
            "status",
            "candidate_head",
            "candidate_tree",
            "candidate_parent",
            "upstream_cutoff",
            "nonce",
            "linux_nonce",
            "nix_nonce",
            "remote_driver_rc",
            "receipt_archive_sha256",
            "copy_path",
            "execution_authorization_sha256",
            "operator_authority_sha256",
            "runner_freeze_sha256",
            "runner_restore_sha256",
            "runner_restore_verified",
            "host_workload_freeze_sha256",
            "host_workload_restore_sha256",
            "host_workload_restore_verified",
            "candidate_nix_process_selected_for_workload_freeze",
            "nix_container_volume_source_mutated",
            "host_lock_profile_sha256",
            "inner_recursive_hashes",
            "inner_recursive_modes",
            "inner_manifest_coverage",
            "candidate_binding",
            "tool_binding",
            "driver_relative_root",
            "driver_recursive_hashes",
            "driver_type_mode_size_directory_inventory",
            "driver_manifest_sha256",
            "driver_manifest_entry_count",
            "nix_same_head_terminal_pass_binding",
            "nix_pass_binding_sha256",
            "nix_attempt_inventory_sha256",
            "nix_attempt_full_inventory_sha256",
            "nix_attempt_full_inventory_entry_count",
            "nix_attempt_full_inventory_file_count",
            "nix_attempt_full_inventory_directory_count",
            "nix_attempt_full_inventory_root",
            "nix_attempt_full_inventory_pre_post_equal",
            "nix_outer_sha256sums_sha256",
            "nix_inner_sha256sums_sha256",
            "host_tools_binding_sha256",
            "host_executables_binding_sha256",
            "sanitized_environment_binding_sha256",
            "sanitized_bootstrap_environment_sha256",
            "sanitized_build_environment_sha256",
            "remote_environment_sanitized",
            "build_affecting_environment_allowlist_exact",
            "bash_env_absent",
            "resource_watchdog_started",
            "resource_watchdog_completed",
            "resource_watchdog_event_observed",
            "resource_watchdog_interval_seconds",
            "resource_watchdog_started_at",
            "resource_watchdog_stopped_at",
            "candidate_window_started_at",
            "candidate_window_completed_at",
            "resource_watchdog_row_count",
            "resource_watchdog_first_observed_at",
            "resource_watchdog_last_observed_at",
            "resource_watchdog_log_sha256",
            "resource_watchdog_single_observation_writer",
            "resource_watchdog_host_lock_verified_each_sample",
            "resource_watchdog_boundary_request_count",
            "resource_watchdog_boundary_ack_count",
            "host_qualification_lock_path",
            "host_qualification_lock_protocol",
            "host_qualification_lock_device",
            "host_qualification_lock_inode",
            "host_qualification_lock_original_holder_pid",
            "host_qualification_lock_original_holder_startticks",
            "host_qualification_lock_coordinator_payload_sha256",
            "host_qualification_lock_driver_inherits_fd",
            "host_qualification_lock_held_through_verified_receipt_copy",
            "host_qualification_lock_original_holder_absent",
            "host_qualification_lock_same_identity_reacquired",
            "host_qualification_lock_release_observation",
            "expected_suite_counts_sha256",
            "expected_suite_counts_inner_copy_sha256",
            "expected_suite_count_rows",
            "expected_suite_count_total",
            "inner_type_mode_size_directory_inventory",
            "incoming_copy_preserved",
            "remote_roots_preserved",
            "data_deleted",
            "promotion_authority",
            "remote_attempt_root",
            "remote_input_root",
            "remote_driver_root",
            "remote_run_root",
            "verified_at",
        ],
    )?;
    require_kv_schema(&outer, "hepta_vnext_linux_local_verification_v3")?;
    require_kv_u64(&outer, "driver_revision", 5)?;
    require_candidate_kv(&outer, candidate, "candidate_head", "upstream_cutoff")?;
    require_kv_bool(&outer, "data_deleted", false)?;
    require_outer_recursive_verification(&outer, true)?;
    verify_linux_v5_bindings(receipt, &inner, &outer, candidate)?;
    verify_linux_v5_lifecycle(receipt, &inner, &outer, candidate)?;
    let inner_manifest = receipt.layer(ManifestLayerIdV3::InnerReceipt)?;
    verify_linux_suite_counts(
        receipt.layer(ManifestLayerIdV3::Outer)?,
        inner_manifest,
        &inner,
        &outer,
    )?;
    verify_linux_binary(inner_manifest, &inner, candidate)?;
    verify_linux_resource_and_exclusivity(inner_manifest)?;
    verify_linux_watchdog(
        receipt.layer(ManifestLayerIdV3::Outer)?,
        inner_manifest,
        &inner,
        &outer,
    )?;
    verify_linux_environment_and_bazel(inner_manifest, candidate)?;
    verify_linux_pass_artifacts(inner_manifest, &inner, candidate)?;
    observe_kv_execution(
        receipt,
        &inner,
        &outer,
        EvidenceProfileV3::LinuxExactV5,
        &profiles::LINUX_STEPS,
        StepPolicy::PrefixFirstFailure,
    )
}

fn verify_linux_pass_artifacts(
    manifest: &VerifiedManifest,
    result: &BTreeMap<String, String>,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    if manifest.entry("failure-status.txt").is_some() {
        return Err(invalid("Linux PASS receipt contains failure-status.txt"));
    }
    let started = parse_key_values(&manifest.bytes("RUN-STARTED.txt")?)?;
    require_exact_kv_fields(
        &started,
        &[
            "schema",
            "candidate_head",
            "nonce",
            "linux_nonce",
            "nix_nonce",
            "remote_attempt_root",
            "remote_input_root",
            "remote_driver_root",
            "remote_run_root",
            "started_at",
        ],
    )?;
    require_kv_schema(&started, "hepta_vnext_linux_exact_run_started_v1")?;
    let completed = parse_key_values(&manifest.bytes("RUN-COMPLETED.txt")?)?;
    require_exact_kv_fields(
        &completed,
        &[
            "schema",
            "driver_revision",
            "candidate_head",
            "linux_nonce",
            "nix_nonce",
            "remote_attempt_root",
            "remote_input_root",
            "remote_driver_root",
            "remote_run_root",
            "verdict",
            "exit_code",
            "completed_at",
        ],
    )?;
    require_kv_schema(&completed, "hepta_vnext_linux_exact_run_completed_v5")?;
    require_kv_u64(&completed, "driver_revision", 5)?;
    require_kv(&completed, "verdict", "PASS")?;
    require_kv_u64(&completed, "exit_code", 0)?;
    for values in [&started, &completed] {
        require_kv(values, "candidate_head", &candidate.head)?;
        for field in [
            "linux_nonce",
            "nix_nonce",
            "remote_attempt_root",
            "remote_input_root",
            "remote_driver_root",
            "remote_run_root",
        ] {
            require_kv(
                values,
                field,
                result
                    .get(field)
                    .ok_or_else(|| invalid("Linux result run binding is absent"))?,
            )?;
        }
    }
    require_kv(
        &started,
        "nonce",
        result
            .get("linux_nonce")
            .ok_or_else(|| invalid("Linux result nonce is absent"))?,
    )?;
    for (values, field) in [(&started, "started_at"), (&completed, "completed_at")] {
        if !values
            .get(field)
            .is_some_and(|value| valid_utc_timestamp(value))
        {
            return Err(invalid("Linux run marker time is not an exact UTC second"));
        }
    }
    for (preflight, postflight) in [
        (
            "preflight-git-tree.manifest",
            "postflight-git-tree.manifest",
        ),
        ("preflight-git-blobs.tsv", "postflight-git-blobs.tsv"),
    ] {
        if manifest.bytes(preflight)? != manifest.bytes(postflight)? {
            return Err(invalid("Linux preflight/postflight Git evidence changed"));
        }
    }
    for (path, expected_rows) in [
        ("input-verification.txt", 21_usize),
        ("tool-input-verification.txt", 4_usize),
        ("vendor-verification.txt", 72_857_usize),
    ] {
        let bytes = manifest.bytes_bounded(path, 32 * 1024 * 1024)?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|_| invalid("Linux verification output is not UTF-8"))?;
        if !text.ends_with('\n')
            || text.lines().count() != expected_rows
            || text.lines().any(|line| !line.ends_with(": OK"))
        {
            return Err(invalid(
                "Linux verification output does not contain the exact OK roster",
            ));
        }
    }
    verify_linux_toolchain_manifest(manifest)
}

fn verify_linux_toolchain_manifest(manifest: &VerifiedManifest) -> Result<(), AcceptanceError> {
    let environment = parse_key_values(&manifest.bytes("environment.txt")?)?;
    let bytes = manifest.bytes("toolchain-binaries.sha256")?;
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| invalid("Linux toolchain binary manifest is not UTF-8"))?;
    let expected = [
        ("rustc_sha256", "rustc"),
        ("cargo_sha256", "cargo"),
        ("clippy_driver_sha256", "clippy-driver"),
        ("rustfmt_sha256", "rustfmt"),
        ("bazelisk_sha256", "bazelisk-linux-amd64"),
        ("bazel_sha256", "bazel-9.0.0-linux-x86_64"),
        ("shellcheck_sha256", "shellcheck"),
    ];
    if !text.ends_with('\n') || text.lines().count() != expected.len() {
        return Err(invalid(
            "Linux toolchain binary manifest has the wrong shape",
        ));
    }
    for (line, (digest_field, basename)) in text.lines().zip(expected) {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let observed_basename = fields
            .get(1)
            .map(|path| path.trim_start_matches('*'))
            .and_then(|path| Path::new(path).file_name())
            .and_then(|value| value.to_str());
        if fields.len() != 2
            || observed_basename != Some(basename)
            || fields[0]
                != environment
                    .get(digest_field)
                    .ok_or_else(|| invalid("Linux toolchain digest field is absent"))?
        {
            return Err(invalid(
                "Linux toolchain binary manifest differs from exact environment pins",
            ));
        }
    }
    Ok(())
}

fn verify_linux_suite_counts(
    outer_manifest: &VerifiedManifest,
    manifest: &VerifiedManifest,
    inner: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
) -> Result<(), AcceptanceError> {
    let bytes = manifest.bytes("test-suite-counts.tsv")?;
    let expected_bytes = profiles::MAC_SUITE_COUNTS
        .iter()
        .map(|(name, count)| format!("{name}\t{count}\n"))
        .collect::<String>()
        .into_bytes();
    if outer_manifest.bytes("driver/expected-suite-counts.tsv")? != expected_bytes {
        return Err(invalid(
            "Linux driver suite count baseline differs from the compiled 52ec matrix",
        ));
    }
    if manifest.bytes("expected-suite-counts.tsv")? != expected_bytes {
        return Err(invalid(
            "Linux inner suite count baseline differs from the compiled 52ec matrix",
        ));
    }
    let expected_digest = sha256(&expected_bytes);
    if expected_digest != "86791565128659a83dc70c99de32a2de13e63f8f684c240d0f2191575e9ea81a" {
        return Err(invalid(
            "compiled Linux suite count baseline digest drifted",
        ));
    }
    for values in [inner, outer] {
        require_kv(values, "expected_suite_counts_sha256", &expected_digest)?;
        require_kv(
            values,
            "expected_suite_counts_inner_copy_sha256",
            &expected_digest,
        )?;
        require_kv_u64(values, "expected_suite_count_rows", 29)?;
        require_kv_u64(values, "expected_suite_count_total", 729)?;
    }
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| invalid("Linux test-suite-counts.tsv is not UTF-8"))?;
    if text.is_empty()
        || !text.ends_with('\n')
        || text.lines().count() != profiles::MAC_SUITE_COUNTS.len()
    {
        return Err(invalid("Linux suite count roster has the wrong shape"));
    }
    let mut targeted_total = 0_u64;
    for (index, (line, (expected_name, expected_count))) in
        text.lines().zip(profiles::MAC_SUITE_COUNTS).enumerate()
    {
        let fields = line.split('\t').collect::<Vec<_>>();
        let count = fields.get(1).and_then(|value| value.parse::<u64>().ok());
        let expected_log = format!("test-{expected_name}.log");
        if fields.len() != 3
            || fields[0] != expected_name
            || count != Some(expected_count)
            || expected_count.to_string() != fields[1]
            || fields[2] != expected_log
        {
            return Err(invalid(
                "Linux suite count row differs from its exact roster",
            ));
        }
        let log = manifest.bytes_bounded(&expected_log, 64 * 1024 * 1024)?;
        let log = std::str::from_utf8(&log).map_err(|_| invalid("Linux suite log is not UTF-8"))?;
        let mut total = 0_u64;
        for result in log
            .lines()
            .filter(|line| line.starts_with("test result: ok."))
        {
            let words = result.split_whitespace().collect::<Vec<_>>();
            for pair in words.windows(2) {
                if pair[1] == "passed;" {
                    total =
                        total
                            .checked_add(pair[0].parse::<u64>().map_err(|_| {
                                invalid("Linux suite log has a malformed passed count")
                            })?)
                            .ok_or_else(|| invalid("Linux suite passed count overflows"))?;
                }
            }
        }
        if total != count.unwrap_or_default() {
            return Err(invalid(
                "Linux suite count differs from independently parsed sealed log results",
            ));
        }
        if index > 0 {
            targeted_total = targeted_total
                .checked_add(total)
                .ok_or_else(|| invalid("Linux targeted suite total overflows"))?;
        }
    }
    if targeted_total != 549 {
        return Err(invalid("Linux targeted suite total differs from 549"));
    }
    let actual_counts = text
        .lines()
        .map(|line| {
            let fields = line.split('\t').collect::<Vec<_>>();
            format!("{}\t{}\n", fields[0], fields[1])
        })
        .collect::<String>()
        .into_bytes();
    if actual_counts != expected_bytes {
        return Err(invalid(
            "Linux suite name/count projection differs from its compiled baseline",
        ));
    }
    Ok(())
}

fn verify_linux_binary(
    manifest: &VerifiedManifest,
    result: &BTreeMap<String, String>,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    let binary = parse_key_values(&manifest.bytes("binary.txt")?)?;
    require_exact_kv_fields(
        &binary,
        &[
            "schema",
            "binary_sha256",
            "binary_size_bytes",
            "binary_built_from_head",
            "binary_built_from_tree",
            "binary_source",
            "fresh_bazel_output_root",
            "inherited_binary",
        ],
    )?;
    require_kv_schema(&binary, "hepta_vnext_linux_exact_binary_v1")?;
    require_kv(&binary, "binary_built_from_head", &candidate.head)?;
    require_kv(&binary, "binary_built_from_tree", &candidate.tree)?;
    require_kv_bool(&binary, "fresh_bazel_output_root", true)?;
    require_kv_bool(&binary, "inherited_binary", false)?;
    let hepta = manifest
        .entry("hepta")
        .ok_or_else(|| invalid("Linux sealed binary is absent"))?;
    let expected_source_prefix = format!(
        "{}/bazel-output-user-root/",
        result
            .get("remote_run_root")
            .ok_or_else(|| invalid("Linux remote run root is absent"))?
    );
    let source = binary
        .get("binary_source")
        .ok_or_else(|| invalid("Linux binary source path is absent"))?;
    if !source.starts_with(&expected_source_prefix)
        || !source.contains("/bazel-out/")
        || !source.contains("/bin/codex-rs/cli/hepta")
        || !source.ends_with("/codex-rs/cli/hepta")
        || Path::new(source)
            .components()
            .any(|component| component == std::path::Component::ParentDir)
    {
        return Err(invalid(
            "Linux binary source path differs from the fresh run root",
        ));
    }
    require_kv(&binary, "binary_sha256", &hepta.sha256)?;
    require_kv_u64(&binary, "binary_size_bytes", hepta.size_bytes)?;
    if hepta.size_bytes == 0 {
        return Err(invalid("Linux sealed binary is empty"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if std::fs::symlink_metadata(manifest.root.join("hepta"))?.mode() & 0o7777 != 0o500 {
            return Err(invalid("Linux sealed binary mode differs from 0500"));
        }
    }
    Ok(())
}

fn verify_linux_resource_and_exclusivity(
    manifest: &VerifiedManifest,
) -> Result<(), AcceptanceError> {
    let resource = parse_key_values(&manifest.bytes("resource-preflight.txt")?)?;
    require_exact_kv_fields(
        &resource,
        &[
            "schema",
            "qualification_host",
            "data_filesystem_source",
            "data_filesystem_mount",
            "run_root_filesystem_source",
            "run_root_filesystem_mount",
            "data_filesystem_matches_run_root",
            "minimum_free_bytes",
            "observed_free_bytes",
            "minimum_free_memory_bytes",
            "observed_available_memory_bytes",
            "resource_gate",
            "observed_at",
        ],
    )?;
    require_kv_schema(&resource, "hepta_vnext_linux_resource_preflight_v5")?;
    require_kv(&resource, "qualification_host", "desktop-ts")?;
    require_kv_bool(&resource, "data_filesystem_matches_run_root", true)?;
    require_kv(&resource, "resource_gate", "pass")?;
    let minimum_disk = parse_decimal_field(&resource, "minimum_free_bytes")?;
    let observed_disk = parse_decimal_field(&resource, "observed_free_bytes")?;
    let minimum_memory = parse_decimal_field(&resource, "minimum_free_memory_bytes")?;
    let observed_memory = parse_decimal_field(&resource, "observed_available_memory_bytes")?;
    if minimum_disk < 107_374_182_400
        || observed_disk < minimum_disk
        || minimum_memory < 3_221_225_472
        || observed_memory < minimum_memory
    {
        return Err(invalid("Linux resource evidence is below its exact floors"));
    }
    require_kv(
        &resource,
        "run_root_filesystem_source",
        resource
            .get("data_filesystem_source")
            .ok_or_else(|| invalid("Linux data filesystem source is absent"))?,
    )?;
    require_kv(
        &resource,
        "run_root_filesystem_mount",
        resource
            .get("data_filesystem_mount")
            .ok_or_else(|| invalid("Linux data filesystem mount is absent"))?,
    )?;
    if resource
        .get("data_filesystem_source")
        .is_none_or(|value| value.is_empty() || value.contains(['\t', '\n']))
        || resource
            .get("data_filesystem_mount")
            .is_none_or(|value| !value.starts_with('/') || value.contains(['\t', '\n']))
        || !resource
            .get("observed_at")
            .is_some_and(|value| valid_utc_timestamp(value))
    {
        return Err(invalid(
            "Linux resource filesystem or time evidence is malformed",
        ));
    }
    for path in [
        "host-exclusivity-preflight.txt",
        "host-exclusivity-postflight.txt",
    ] {
        let exclusivity = parse_key_values(&manifest.bytes(path)?)?;
        require_exact_kv_fields(
            &exclusivity,
            &[
                "schema",
                "qualification_host",
                "active_hepta_nix_process",
                "active_hepta_nix_container",
                "active_hepta_nix_volume_or_source_mount",
                "running_libvirt_domains",
                "active_github_actions_runner_listeners",
                "active_github_actions_runner_workers",
                "other_active_hepta_qualification_builds",
                "observed_at",
            ],
        )?;
        require_kv_schema(&exclusivity, "hepta_vnext_linux_host_exclusivity_v5")?;
        require_kv(&exclusivity, "qualification_host", "desktop-ts")?;
        for field in [
            "active_hepta_nix_process",
            "active_hepta_nix_container",
            "active_hepta_nix_volume_or_source_mount",
        ] {
            require_kv_bool(&exclusivity, field, false)?;
        }
        require_kv_u64(&exclusivity, "running_libvirt_domains", 0)?;
        for field in [
            "active_github_actions_runner_listeners",
            "active_github_actions_runner_workers",
            "other_active_hepta_qualification_builds",
        ] {
            require_kv_u64(&exclusivity, field, 0)?;
        }
        if !exclusivity
            .get("observed_at")
            .is_some_and(|value| valid_utc_timestamp(value))
        {
            return Err(invalid(
                "Linux host exclusivity time is not an exact UTC second",
            ));
        }
    }
    Ok(())
}

fn verify_linux_watchdog(
    outer_manifest: &VerifiedManifest,
    inner_manifest: &VerifiedManifest,
    inner: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
) -> Result<(), AcceptanceError> {
    if inner_manifest
        .entry("resource-watchdog-event.txt")
        .is_some()
    {
        return Err(invalid(
            "Linux PASS receipt contains a resource watchdog event",
        ));
    }
    if outer_manifest.bytes("driver/resource-watchdog-filter-v5.awk")?
        != inner_manifest.bytes("resource-watchdog-filter-v5.awk")?
    {
        return Err(invalid(
            "Linux resource watchdog filter differs between driver and receipt",
        ));
    }
    if outer_manifest.bytes("driver/verify-resource-watchdog-v5.py")?
        != inner_manifest.bytes("verify-resource-watchdog-v5.py")?
    {
        return Err(invalid(
            "Linux resource watchdog verifier differs between driver and receipt",
        ));
    }
    let observations = inner_manifest.bytes("resource-watchdog-observations.tsv")?;
    let digest = sha256(&observations);
    for values in [inner, outer] {
        require_kv_bool(values, "resource_watchdog_started", true)?;
        require_kv_bool(values, "resource_watchdog_completed", true)?;
        require_kv_bool(values, "resource_watchdog_event_observed", false)?;
        require_kv_u64(values, "resource_watchdog_interval_seconds", 15)?;
        require_kv(values, "resource_watchdog_log_sha256", &digest)?;
    }
    for field in [
        "resource_watchdog_started_at",
        "resource_watchdog_stopped_at",
        "candidate_window_started_at",
        "candidate_window_completed_at",
        "resource_watchdog_row_count",
        "resource_watchdog_first_observed_at",
        "resource_watchdog_last_observed_at",
    ] {
        require_kv(
            outer,
            field,
            inner
                .get(field)
                .ok_or_else(|| invalid("Linux resource watchdog result binding is absent"))?,
        )?;
    }
    for field in [
        "resource_watchdog_single_observation_writer",
        "resource_watchdog_host_lock_verified_each_sample",
    ] {
        require_kv_bool(inner, field, true)?;
        require_kv_bool(outer, field, true)?;
    }
    for field in [
        "resource_watchdog_boundary_request_count",
        "resource_watchdog_boundary_ack_count",
    ] {
        require_kv_u64(inner, field, 2)?;
        require_kv_u64(outer, field, 2)?;
    }
    verify_linux_watchdog_observations(&observations, inner)?;
    verify_linux_watchdog_boundaries(inner_manifest, &observations, inner)
}

fn verify_linux_watchdog_observations(
    observations: &[u8],
    result: &BTreeMap<String, String>,
) -> Result<(), AcceptanceError> {
    if observations.is_empty() || !observations.ends_with(b"\n") || observations.contains(&b'\r') {
        return Err(invalid(
            "Linux resource watchdog observations are empty or not canonical LF text",
        ));
    }
    let text = std::str::from_utf8(observations)
        .map_err(|_| invalid("Linux resource watchdog observations are not UTF-8"))?;
    let mut first = None;
    let mut previous = None;
    let mut previous_seconds = None;
    let mut last = None;
    let mut row_count = 0_u64;
    let mut boundary_start_count = 0_u64;
    let mut boundary_complete_count = 0_u64;
    for line in text.lines() {
        let columns = line.split('\t').collect::<Vec<_>>();
        if columns.len() != 7
            || columns[3] != "listeners=0"
            || columns[4] != "workers=0"
            || columns[5] != "other_hepta_builds=0"
            || columns[6] != "lock_held=true"
            || !valid_utc_timestamp(columns[0])
        {
            return Err(invalid(
                "Linux resource watchdog observation row differs from its exact seven-column v5 contract",
            ));
        }
        let sample = columns[1]
            .strip_prefix("sample=")
            .ok_or_else(|| invalid("Linux watchdog sample field is malformed"))?;
        let sequence = columns[2]
            .strip_prefix("request_sequence=")
            .and_then(|value| value.parse::<u64>().ok())
            .ok_or_else(|| invalid("Linux watchdog request sequence is malformed"))?;
        match (sample, sequence) {
            ("periodic", 0) => {}
            ("boundary_candidate_start", 1) => {
                boundary_start_count = boundary_start_count.saturating_add(1)
            }
            ("boundary_candidate_complete", 2) => {
                boundary_complete_count = boundary_complete_count.saturating_add(1)
            }
            _ => {
                return Err(invalid(
                    "Linux watchdog sample kind/request sequence differs from v5",
                ));
            }
        }
        let current_seconds = utc_timestamp_seconds(columns[0])
            .ok_or_else(|| invalid("Linux resource watchdog timestamp is malformed"))?;
        if previous.is_some_and(|value| columns[0] < value) {
            return Err(invalid(
                "Linux resource watchdog observation times are not monotonic",
            ));
        }
        if previous_seconds.is_some_and(|value| current_seconds - value > 20) {
            return Err(invalid(
                "Linux resource watchdog observation gap exceeds 20 seconds",
            ));
        }
        first.get_or_insert(columns[0]);
        previous = Some(columns[0]);
        previous_seconds = Some(current_seconds);
        last = Some(columns[0]);
        row_count = row_count
            .checked_add(1)
            .ok_or_else(|| invalid("Linux resource watchdog row count overflow"))?;
    }
    let first = first.ok_or_else(|| invalid("Linux resource watchdog has no observations"))?;
    let last = last.ok_or_else(|| invalid("Linux resource watchdog has no final observation"))?;
    require_kv_u64(result, "resource_watchdog_row_count", row_count)?;
    require_kv(result, "resource_watchdog_first_observed_at", first)?;
    require_kv(result, "resource_watchdog_last_observed_at", last)?;

    let timestamp = |field| {
        result
            .get(field)
            .filter(|value| valid_utc_timestamp(value))
            .map(String::as_str)
            .ok_or_else(|| {
                invalid(format!(
                    "Linux resource watchdog boundary is not an exact UTC second: {field}"
                ))
            })
    };
    let started = timestamp("resource_watchdog_started_at")?;
    let stopped = timestamp("resource_watchdog_stopped_at")?;
    let candidate_started = timestamp("candidate_window_started_at")?;
    let candidate_completed = timestamp("candidate_window_completed_at")?;
    if !(started <= first
        && first <= candidate_started
        && candidate_started <= candidate_completed
        && candidate_completed <= last
        && last <= stopped)
    {
        return Err(invalid(
            "Linux resource watchdog observations do not cover the complete candidate window",
        ));
    }
    if boundary_start_count != 1 || boundary_complete_count != 1 {
        return Err(invalid(
            "Linux watchdog lacks exactly one start and completion boundary sample",
        ));
    }
    Ok(())
}

fn verify_linux_watchdog_boundaries(
    manifest: &VerifiedManifest,
    observations: &[u8],
    result: &BTreeMap<String, String>,
) -> Result<(), AcceptanceError> {
    let prefix = "resource-watchdog-boundaries/";
    let expected_names = [
        "ack-00000001.txt",
        "ack-00000002.txt",
        "request-00000001.txt",
        "request-00000002.txt",
    ]
    .into_iter()
    .map(|name| format!("{prefix}{name}"))
    .collect::<BTreeSet<_>>();
    let actual_names = manifest
        .entry_paths()
        .filter(|path| path.starts_with(prefix))
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if actual_names != expected_names
        || !manifest
            .directory_paths()
            .any(|path| path == "resource-watchdog-boundaries")
    {
        return Err(invalid(
            "Linux watchdog boundary evidence namespace is not exact",
        ));
    }
    let text = std::str::from_utf8(observations)
        .map_err(|_| invalid("Linux watchdog observations are not UTF-8"))?;
    let nonce = result
        .get("linux_nonce")
        .ok_or_else(|| invalid("Linux watchdog nonce is absent"))?;
    for (sequence, purpose, observed_field) in [
        (1_u64, "candidate_start", "candidate_window_started_at"),
        (2_u64, "candidate_complete", "candidate_window_completed_at"),
    ] {
        let request =
            parse_key_values(&manifest.bytes(&format!("{prefix}request-{sequence:08}.txt"))?)?;
        require_exact_kv_fields(
            &request,
            &[
                "schema",
                "candidate_head",
                "candidate_tree",
                "linux_nonce",
                "sequence",
                "purpose",
                "requested_at",
            ],
        )?;
        require_kv_schema(&request, "hepta_vnext_linux_resource_watchdog_request_v5")?;
        require_kv(&request, "candidate_head", CANDIDATE_HEAD)?;
        require_kv(&request, "candidate_tree", CANDIDATE_TREE)?;
        require_kv(&request, "linux_nonce", nonce)?;
        require_kv_u64(&request, "sequence", sequence)?;
        require_kv(&request, "purpose", purpose)?;
        let requested_at = request
            .get("requested_at")
            .filter(|value| valid_utc_timestamp(value))
            .ok_or_else(|| invalid("Linux watchdog request time is malformed"))?;

        let ack = parse_key_values(&manifest.bytes(&format!("{prefix}ack-{sequence:08}.txt"))?)?;
        require_exact_kv_fields(
            &ack,
            &[
                "schema",
                "status",
                "candidate_head",
                "candidate_tree",
                "linux_nonce",
                "sequence",
                "purpose",
                "observed_at",
                "observation_row_sha256",
                "host_qualification_lock_held",
            ],
        )?;
        require_kv_schema(&ack, "hepta_vnext_linux_resource_watchdog_ack_v5")?;
        require_kv(&ack, "status", "CLEAR")?;
        require_kv(&ack, "candidate_head", CANDIDATE_HEAD)?;
        require_kv(&ack, "candidate_tree", CANDIDATE_TREE)?;
        require_kv(&ack, "linux_nonce", nonce)?;
        require_kv_u64(&ack, "sequence", sequence)?;
        require_kv(&ack, "purpose", purpose)?;
        require_kv_bool(&ack, "host_qualification_lock_held", true)?;
        let observed_at = ack
            .get("observed_at")
            .filter(|value| valid_utc_timestamp(value))
            .ok_or_else(|| invalid("Linux watchdog acknowledgment time is malformed"))?;
        require_kv(
            &ack,
            "observed_at",
            result
                .get(observed_field)
                .ok_or_else(|| invalid("Linux candidate boundary time is absent"))?,
        )?;
        if observed_at < requested_at {
            return Err(invalid(
                "Linux watchdog acknowledgment predates its request",
            ));
        }
        let sample = format!("sample=boundary_{purpose}");
        let sequence_field = format!("request_sequence={sequence}");
        let matching = text
            .lines()
            .filter(|line| {
                let fields = line.split('\t').collect::<Vec<_>>();
                fields.len() == 7
                    && fields[0] == observed_at
                    && fields[1] == sample
                    && fields[2] == sequence_field
            })
            .collect::<Vec<_>>();
        if matching.len() != 1
            || ack.get("observation_row_sha256")
                != Some(&sha256(format!("{}\n", matching[0]).as_bytes()))
        {
            return Err(invalid(
                "Linux watchdog boundary acknowledgment does not bind one exact row",
            ));
        }
    }

    let expected_verification = format!(
        "row_count={}\nfirst_observed_at={}\nlast_observed_at={}\nboundary_request_count=2\nboundary_ack_count=2\nsingle_observation_writer=true\nhost_lock_verified_each_sample=true\n",
        result
            .get("resource_watchdog_row_count")
            .ok_or_else(|| invalid("Linux watchdog row count is absent"))?,
        result
            .get("resource_watchdog_first_observed_at")
            .ok_or_else(|| invalid("Linux watchdog first time is absent"))?,
        result
            .get("resource_watchdog_last_observed_at")
            .ok_or_else(|| invalid("Linux watchdog last time is absent"))?,
    );
    if manifest.bytes("resource-watchdog-verification.txt")? != expected_verification.as_bytes() {
        return Err(invalid(
            "Linux watchdog verification output differs from independent replay",
        ));
    }
    Ok(())
}

#[cfg(test)]
pub(super) fn verify_linux_watchdog_observations_for_test(
    observations: &[u8],
    result: &BTreeMap<String, String>,
) -> Result<(), AcceptanceError> {
    verify_linux_watchdog_observations(observations, result)
}

fn parse_decimal_field(
    values: &BTreeMap<String, String>,
    field: &str,
) -> Result<u64, AcceptanceError> {
    let value = values
        .get(field)
        .ok_or_else(|| invalid(format!("required decimal field is absent: {field}")))?;
    let parsed = value
        .parse::<u64>()
        .map_err(|_| invalid(format!("required decimal field is malformed: {field}")))?;
    if parsed.to_string() != *value {
        return Err(invalid(format!(
            "required decimal field is not canonical: {field}"
        )));
    }
    Ok(parsed)
}

fn verify_linux_environment_and_bazel(
    manifest: &VerifiedManifest,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    let environment = parse_key_values(&manifest.bytes("environment.txt")?)?;
    require_exact_kv_fields(
        &environment,
        &[
            "schema",
            "candidate_head",
            "candidate_tree",
            "input_manifest_sha256",
            "bundle_sha256",
            "expected_tree_manifest_sha256",
            "expected_blob_manifest_sha256",
            "vendor_archive_sha256",
            "vendor_manifest_sha256",
            "cutover_bridge_sha256",
            "vendor_file_count",
            "tool_input_manifest_sha256",
            "rust_toolchain_archive_sha256",
            "rust_toolchain_archive_pin_sha256",
            "rustc_sha256",
            "cargo_sha256",
            "clippy_driver_sha256",
            "rustfmt_sha256",
            "bazelisk_sha256",
            "bazel_sha256",
            "shellcheck_archive_sha256",
            "shellcheck_sha256",
            "shellcheck_static_runtime_closure",
            "cargo_home",
            "cargo_target",
            "bazel_output_user_root",
            "seeded_build_results",
            "qualification_host",
            "sanitized_environment_binding_sha256",
            "sanitized_bootstrap_environment_sha256",
            "sanitized_build_environment_sha256",
            "remote_environment_sanitized",
            "build_affecting_environment_allowlist_exact",
            "bash_env_absent",
            "expected_suite_counts_sha256",
            "expected_suite_count_rows",
            "expected_suite_count_total",
        ],
    )?;
    require_kv_schema(&environment, "hepta_vnext_linux_exact_environment_v1")?;
    require_kv(&environment, "candidate_head", &candidate.head)?;
    require_kv(&environment, "candidate_tree", &candidate.tree)?;
    require_kv(&environment, "qualification_host", "desktop-ts")?;
    require_kv_bool(&environment, "seeded_build_results", false)?;
    require_kv_bool(&environment, "shellcheck_static_runtime_closure", true)?;
    for field in [
        "remote_environment_sanitized",
        "build_affecting_environment_allowlist_exact",
        "bash_env_absent",
    ] {
        require_kv_bool(&environment, field, true)?;
    }
    require_kv(
        &environment,
        "input_manifest_sha256",
        "65e9f7e70294c44b5f8e79881af6eebacdcb428452fd47bd8f5dcc54f4fd4bda",
    )?;
    require_kv(
        &environment,
        "tool_input_manifest_sha256",
        "a1f398db1d435348d7486e732d7915d39936db953e864943c7569350926cb592",
    )?;
    require_kv_u64(&environment, "vendor_file_count", 72_857)?;
    require_kv(
        &environment,
        "expected_suite_counts_sha256",
        "86791565128659a83dc70c99de32a2de13e63f8f684c240d0f2191575e9ea81a",
    )?;
    require_kv_u64(&environment, "expected_suite_count_rows", 29)?;
    require_kv_u64(&environment, "expected_suite_count_total", 729)?;
    for (field, path) in [
        (
            "sanitized_environment_binding_sha256",
            "sanitized-environment-v5.txt",
        ),
        (
            "sanitized_bootstrap_environment_sha256",
            "sanitized-bootstrap-environment.txt",
        ),
        (
            "sanitized_build_environment_sha256",
            "sanitized-build-environment.txt",
        ),
    ] {
        require_kv(&environment, field, &sha256(&manifest.bytes(path)?))?;
    }
    for field in [
        "input_manifest_sha256",
        "bundle_sha256",
        "expected_tree_manifest_sha256",
        "expected_blob_manifest_sha256",
        "vendor_archive_sha256",
        "vendor_manifest_sha256",
        "cutover_bridge_sha256",
        "tool_input_manifest_sha256",
        "rust_toolchain_archive_sha256",
        "rust_toolchain_archive_pin_sha256",
        "rustc_sha256",
        "cargo_sha256",
        "clippy_driver_sha256",
        "rustfmt_sha256",
        "bazelisk_sha256",
        "bazel_sha256",
        "shellcheck_archive_sha256",
        "shellcheck_sha256",
    ] {
        if !environment
            .get(field)
            .is_some_and(|value| digest_shape(value))
        {
            return Err(invalid(format!(
                "Linux environment digest is malformed: {field}"
            )));
        }
    }
    let bazel = parse_key_values(&manifest.bytes("bazel-result.txt")?)?;
    require_exact_kv_fields(
        &bazel,
        &[
            "schema",
            "candidate_head",
            "candidate_tree",
            "bazel_sha256",
            "bazel_version",
            "bazel_lock_check",
            "bazel_target_tests",
            "caller_ledger_bazel_test",
            "bazel_product_build",
            "fresh_output_root",
            "candidate_changed",
        ],
    )?;
    require_kv_schema(&bazel, "hepta_vnext_linux_bazel_result_v1")?;
    require_kv(&bazel, "candidate_head", &candidate.head)?;
    require_kv(&bazel, "candidate_tree", &candidate.tree)?;
    require_kv(
        &bazel,
        "bazel_sha256",
        "c44a93f25398c68f904fa1d19b61d321de6c0d2f09dca375d7bc0dc9b9428403",
    )?;
    require_kv(&bazel, "bazel_version", "9.0.0")?;
    for (field, expected) in [
        ("bazel_lock_check", "pass"),
        ("bazel_target_tests", "6/6"),
        ("caller_ledger_bazel_test", "pass"),
        ("bazel_product_build", "pass"),
    ] {
        require_kv(&bazel, field, expected)?;
    }
    require_kv_bool(&bazel, "fresh_output_root", true)?;
    require_kv_bool(&bazel, "candidate_changed", false)?;
    Ok(())
}

fn verify_mac_suite_counts(manifest: &VerifiedManifest) -> Result<(), AcceptanceError> {
    let bytes = manifest.bytes("test-suite-counts.tsv")?;
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| invalid("Mac test-suite-counts.tsv is not UTF-8"))?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(invalid(
            "Mac test-suite-counts.tsv must be nonempty and newline terminated",
        ));
    }
    let lines = text.lines().collect::<Vec<_>>();
    if lines.len() != profiles::MAC_SUITE_COUNTS.len() {
        return Err(invalid("Mac suite count roster has the wrong length"));
    }
    for (line, (expected_name, expected_count)) in lines.iter().zip(profiles::MAC_SUITE_COUNTS) {
        let fields = line.split('\t').collect::<Vec<_>>();
        let expected_log = format!("test-{expected_name}.log");
        let count = fields.get(1).and_then(|value| value.parse::<u64>().ok());
        if fields.len() != 3
            || fields[0] != expected_name
            || count != Some(expected_count)
            || count.is_some_and(|value| value.to_string() != fields[1])
            || fields[2] != expected_log
            || manifest.entry(&expected_log).is_none()
        {
            return Err(invalid(
                "Mac suite count or sealed log differs from the compiled roster",
            ));
        }
    }
    let targeted_total = profiles::MAC_SUITE_COUNTS
        .iter()
        .skip(1)
        .map(|(_, count)| count)
        .sum::<u64>();
    if targeted_total != 549 {
        return Err(invalid("compiled Mac targeted suite total is inconsistent"));
    }
    Ok(())
}

fn verify_mac_binary_and_canary(
    manifest: &VerifiedManifest,
    status: &BTreeMap<String, String>,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    let binary = parse_key_values(&manifest.bytes("binary.txt")?)?;
    require_exact_kv_fields(
        &binary,
        &[
            "binary_sha256",
            "binary_size_bytes",
            "binary_source",
            "binary_built_from_head",
            "binary_built_from_tree",
            "canonical_ssd_bazel_output",
            "inherited_manifest",
            "inherited_binary",
        ],
    )?;
    require_kv(
        &binary,
        "binary_sha256",
        status
            .get("mac_binary_sha256")
            .ok_or_else(|| invalid("Mac binary digest is absent"))?,
    )?;
    require_kv(
        &binary,
        "binary_size_bytes",
        status
            .get("mac_binary_size_bytes")
            .ok_or_else(|| invalid("Mac binary size is absent"))?,
    )?;
    require_kv(&binary, "binary_built_from_head", &candidate.head)?;
    require_kv(&binary, "binary_built_from_tree", &candidate.tree)?;
    require_kv_bool(&binary, "canonical_ssd_bazel_output", true)?;
    require_kv_bool(&binary, "inherited_manifest", false)?;
    require_kv_bool(&binary, "inherited_binary", false)?;
    let size = binary
        .get("binary_size_bytes")
        .and_then(|value| value.parse::<u64>().ok())
        .ok_or_else(|| invalid("Mac binary size is malformed"))?;
    if size.to_string() != binary["binary_size_bytes"] || size == 0 {
        return Err(invalid("Mac binary size is not canonical and positive"));
    }
    let executable = manifest
        .entry("hepta")
        .ok_or_else(|| invalid("Mac exact binary is absent from the sealed receipt"))?;
    if executable.sha256 != binary["binary_sha256"] || executable.size_bytes != size {
        return Err(invalid(
            "Mac exact binary bytes differ from their status binding",
        ));
    }

    let canary = parse_key_values(&manifest.bytes("canary-result.txt")?)?;
    require_exact_kv_fields(
        &canary,
        &[
            "schema",
            "canary_rc",
            "candidate_head",
            "candidate_tree",
            "binary_sha256",
            "snapshot_receipt_sha256",
            "target_manifest_sha256",
            "target_release_sha256s_sha256",
            "canary_receipt_sha256",
            "canary_soak_receipt_sha256",
            "v1_downgrade_verdict_sha256",
            "full_state_v2",
            "isolated_canary_soak",
            "authority_all_closed",
            "production_changed",
            "completed_at",
        ],
    )?;
    require_kv(
        &canary,
        "schema",
        "hepta_vnext_main_mac_full_state_v2_canary_v1",
    )?;
    require_kv_u64(&canary, "canary_rc", 0)?;
    require_kv(&canary, "candidate_head", &candidate.head)?;
    require_kv(&canary, "candidate_tree", &candidate.tree)?;
    for (canary_field, status_field) in [
        ("binary_sha256", "mac_binary_sha256"),
        ("snapshot_receipt_sha256", "snapshot_receipt_sha256"),
        ("target_manifest_sha256", "target_manifest_sha256"),
        (
            "target_release_sha256s_sha256",
            "target_release_sha256s_sha256",
        ),
        ("canary_receipt_sha256", "canary_receipt_sha256"),
        ("canary_soak_receipt_sha256", "canary_soak_receipt_sha256"),
        ("v1_downgrade_verdict_sha256", "v1_downgrade_verdict_sha256"),
    ] {
        require_kv(
            &canary,
            canary_field,
            status
                .get(status_field)
                .ok_or_else(|| invalid("Mac status cross-binding is absent"))?,
        )?;
    }
    require_kv_bool(&canary, "full_state_v2", true)?;
    require_kv(&canary, "isolated_canary_soak", "3/3")?;
    require_kv_bool(&canary, "authority_all_closed", true)?;
    require_kv_bool(&canary, "production_changed", false)?;
    if !valid_utc_timestamp(
        canary
            .get("completed_at")
            .ok_or_else(|| invalid("Mac canary completion time is absent"))?,
    ) {
        return Err(invalid(
            "Mac canary completion time is not an exact UTC second",
        ));
    }
    Ok(())
}

fn verify_linux_flat_contract_seal(
    manifest: &VerifiedManifest,
    prefix: &str,
) -> Result<String, AcceptanceError> {
    let manifest_path = format!("{prefix}/SHA256SUMS");
    let mode_path = format!("{prefix}/MODES.tsv");
    let manifest_bytes = manifest.bytes(&manifest_path)?;
    let parsed = parse_manifest(&manifest_bytes)?;
    let prefix_with_slash = format!("{prefix}/");
    let actual_files = manifest
        .entry_paths()
        .filter_map(|path| path.strip_prefix(&prefix_with_slash))
        .filter(|path| !path.contains('/') && *path != "SHA256SUMS")
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if parsed.keys().cloned().collect::<BTreeSet<_>>() != actual_files {
        return Err(invalid(format!(
            "Linux flat contract seal does not close its exact file namespace: {prefix}"
        )));
    }
    for (path, digest) in &parsed {
        if manifest
            .entry(&format!("{prefix}/{path}"))
            .map(|entry| entry.sha256.as_str())
            != Some(digest)
        {
            return Err(invalid(format!(
                "Linux flat contract payload differs from its seal: {prefix}/{path}"
            )));
        }
    }
    if manifest
        .directory_paths()
        .any(|path| path.starts_with(&prefix_with_slash))
    {
        return Err(invalid(format!(
            "Linux flat contract contains a child directory: {prefix}"
        )));
    }
    let inner_modes = parse_mode_rows(
        &manifest.bytes(&mode_path)?,
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    )?;
    let expected_mode_files = actual_files
        .iter()
        .cloned()
        .chain(std::iter::once("SHA256SUMS".to_string()))
        .collect::<BTreeSet<_>>();
    let observed_mode_files = inner_modes
        .iter()
        .filter(|(_, row)| row.kind == ModeKind::File)
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    let observed_mode_directories = inner_modes
        .iter()
        .filter(|(_, row)| row.kind == ModeKind::Directory)
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    if observed_mode_files != expected_mode_files
        || observed_mode_directories != BTreeSet::from([".".to_string()])
    {
        return Err(invalid(format!(
            "Linux flat contract mode inventory does not close its namespace: {prefix}"
        )));
    }
    let outer_modes = parse_mode_rows(
        &manifest.bytes("OUTER-MODES.tsv")?,
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    )?;
    for (path, row) in &inner_modes {
        let outer_path = if path == "." {
            prefix.to_string()
        } else {
            format!("{prefix}/{path}")
        };
        if outer_modes.get(&outer_path) != Some(row) {
            return Err(invalid(format!(
                "Linux flat contract metadata differs from outer inventory: {outer_path}"
            )));
        }
    }
    Ok(sha256(&manifest_bytes))
}

fn verify_linux_authorization_window(
    values: &BTreeMap<String, String>,
) -> Result<(), AcceptanceError> {
    let issued = values
        .get("issued_at")
        .and_then(|value| utc_timestamp_seconds(value))
        .ok_or_else(|| invalid("Linux authorization issue time is malformed"))?;
    let expires = values
        .get("expires_at")
        .and_then(|value| utc_timestamp_seconds(value))
        .ok_or_else(|| invalid("Linux authorization expiry time is malformed"))?;
    if expires <= issued || expires - issued > 900 {
        return Err(invalid(
            "Linux authorization lifetime is not within 1..=900 seconds",
        ));
    }
    Ok(())
}

fn verify_linux_v5_lifecycle(
    receipt: &VerifiedReceipt,
    inner: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    let manifest = receipt.layer(ManifestLayerIdV3::Outer)?;
    let inner_manifest = receipt.layer(ManifestLayerIdV3::InnerReceipt)?;
    let nonce = inner
        .get("linux_nonce")
        .ok_or_else(|| invalid("Linux lifecycle nonce is absent"))?;

    let contract_bytes = manifest.bytes("driver/acceptance-v7-linux-v5-contract.txt")?;
    let contract = parse_key_values(&contract_bytes)?;
    require_exact_kv_fields(
        &contract,
        &[
            "schema",
            "status",
            "contract_revision",
            "driver_revision",
            "acceptance_profile_revision",
            "implementation_compatible",
            "execution_authority_eligible",
            "operator_allowed_signers_sha256",
            "operator_identity",
            "operator_signature_algorithm",
            "operator_signature_namespace",
            "qualification_execution_path",
            "copy_path",
            "recovery_supported",
            "resume_supported",
            "copy_only_supported",
            "inner_result_schema",
            "outer_verification_schema",
            "host_observation_schema",
            "watchdog_columns",
            "watchdog_boundary_request_ack_required",
            "host_lock_lifecycle_required",
            "legacy_production_pre_post_required",
            "runner_freeze_required_before_launch",
            "runner_restore_required_before_outer_seal",
            "host_workload_freeze_required_before_launch",
            "host_workload_restore_required_before_outer_seal",
            "nix_pass_required_before_host_workload_freeze",
            "execution_authorization_required_before_launch_root",
            "execution_authorization_required_before_remote_contact",
            "promotion_authority",
        ],
    )?;
    for (field, expected) in [
        ("schema", "hepta_vnext_acceptance_v7_linux_v5_contract_v1"),
        ("status", "FROZEN"),
        ("operator_identity", profiles::LINUX_OPERATOR_PRINCIPAL),
        ("operator_signature_algorithm", "sshsig-ed25519"),
        (
            "operator_signature_namespace",
            profiles::LINUX_OPERATOR_SIGNATURE_NAMESPACE,
        ),
        ("qualification_execution_path", "direct_launcher"),
        ("copy_path", "direct_launcher"),
        ("inner_result_schema", "hepta_vnext_linux_exact_result_v3"),
        (
            "outer_verification_schema",
            "hepta_vnext_linux_local_verification_v3",
        ),
        (
            "host_observation_schema",
            "hepta_vnext_linux_host_tool_observation_v5",
        ),
        (
            "watchdog_columns",
            "timestamp,sample,request_sequence,listeners,workers,other_hepta_builds,lock_held",
        ),
    ] {
        require_kv(&contract, field, expected)?;
    }
    require_kv_u64(&contract, "contract_revision", 1)?;
    require_kv_u64(&contract, "driver_revision", 5)?;
    require_kv_u64(&contract, "acceptance_profile_revision", 7)?;
    require_kv(
        &contract,
        "operator_allowed_signers_sha256",
        profiles::LINUX_OPERATOR_ALLOWED_SIGNERS_SHA256,
    )?;
    for field in [
        "implementation_compatible",
        "execution_authority_eligible",
        "watchdog_boundary_request_ack_required",
        "host_lock_lifecycle_required",
        "legacy_production_pre_post_required",
        "runner_freeze_required_before_launch",
        "runner_restore_required_before_outer_seal",
        "host_workload_freeze_required_before_launch",
        "host_workload_restore_required_before_outer_seal",
        "nix_pass_required_before_host_workload_freeze",
        "execution_authorization_required_before_launch_root",
        "execution_authorization_required_before_remote_contact",
    ] {
        require_kv_bool(&contract, field, true)?;
    }
    for field in [
        "recovery_supported",
        "resume_supported",
        "copy_only_supported",
        "promotion_authority",
    ] {
        require_kv_bool(&contract, field, false)?;
    }

    let trust_root_sha = verify_linux_flat_contract_seal(manifest, "driver/operator-trust-policy")?;
    let trust_policy_bytes = manifest.bytes("driver/operator-trust-policy/trust-policy.json")?;
    let trust_allowed = manifest.bytes("driver/operator-trust-policy/allowed_signers")?;
    validate_linux_v5_trust_policy(&trust_policy_bytes, &trust_allowed)?;
    if !digest_shape(&trust_root_sha) {
        return Err(invalid("Linux trust-policy root seal digest is malformed"));
    }

    let authority_sha = verify_linux_flat_contract_seal(manifest, "driver/operator-authority")?;
    require_kv(outer, "operator_authority_sha256", &authority_sha)?;
    let authority_bytes = manifest.bytes("driver/operator-authority/AUTHORITY.txt")?;
    let authority = parse_key_values(&authority_bytes)?;
    require_exact_kv_fields(
        &authority,
        &[
            "schema",
            "status",
            "action",
            "driver_revision",
            "candidate_head",
            "candidate_tree",
            "linux_nonce",
            "qualification_host",
            "operator_identity",
            "authorization_nonce",
            "authorization_scope",
            "signature_algorithm",
            "signature_namespace",
            "allowed_signers_sha256",
            "acceptance_contract_sha256",
            "challenge_sha256",
            "detached_signature_sha256",
            "issued_at",
            "expires_at",
            "single_use",
            "promotion_authority",
            "production_authority",
            "unregister_authority",
            "delete_authority",
        ],
    )?;
    require_kv_schema(&authority, "hepta_vnext_linux_operator_authority_v1")?;
    require_kv(&authority, "status", "AUTHORIZED")?;
    require_kv(&authority, "action", profiles::LINUX_OPERATOR_ACTION)?;
    require_kv_u64(&authority, "driver_revision", 5)?;
    require_kv(&authority, "candidate_head", &candidate.head)?;
    require_kv(&authority, "candidate_tree", &candidate.tree)?;
    require_kv(&authority, "linux_nonce", nonce)?;
    require_kv(&authority, "qualification_host", "desktop-ts")?;
    require_kv(
        &authority,
        "operator_identity",
        profiles::LINUX_OPERATOR_PRINCIPAL,
    )?;
    require_kv(
        &authority,
        "authorization_scope",
        profiles::LINUX_OPERATOR_AUTHORIZATION_SCOPE,
    )?;
    require_kv(&authority, "signature_algorithm", "sshsig-ed25519")?;
    require_kv(
        &authority,
        "signature_namespace",
        profiles::LINUX_OPERATOR_SIGNATURE_NAMESPACE,
    )?;
    require_kv(
        &authority,
        "allowed_signers_sha256",
        profiles::LINUX_OPERATOR_ALLOWED_SIGNERS_SHA256,
    )?;
    require_kv(
        &authority,
        "acceptance_contract_sha256",
        &sha256(&contract_bytes),
    )?;
    require_kv_bool(&authority, "single_use", true)?;
    for field in [
        "promotion_authority",
        "production_authority",
        "unregister_authority",
        "delete_authority",
    ] {
        require_kv_bool(&authority, field, false)?;
    }
    if !authority.get("authorization_nonce").is_some_and(|value| {
        (8..=64).contains(&value.len())
            && value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    }) {
        return Err(invalid("Linux operator authorization nonce is malformed"));
    }
    verify_linux_authorization_window(&authority)?;
    let challenge_bytes = manifest.bytes("driver/operator-authority/CHALLENGE.txt")?;
    let challenge = parse_key_values(&challenge_bytes)?;
    require_exact_kv_fields(
        &challenge,
        &[
            "schema",
            "action",
            "driver_revision",
            "candidate_head",
            "candidate_tree",
            "linux_nonce",
            "qualification_host",
            "operator_identity",
            "authorization_nonce",
            "authorization_scope",
            "acceptance_contract_sha256",
            "allowed_signers_sha256",
            "signature_algorithm",
            "signature_namespace",
            "issued_at",
            "expires_at",
            "single_use",
            "promotion_authority",
            "production_authority",
            "unregister_authority",
            "delete_authority",
        ],
    )?;
    require_kv_schema(&challenge, "hepta_vnext_linux_operator_challenge_v2")?;
    for field in [
        "action",
        "driver_revision",
        "candidate_head",
        "candidate_tree",
        "linux_nonce",
        "qualification_host",
        "operator_identity",
        "authorization_nonce",
        "authorization_scope",
        "acceptance_contract_sha256",
        "allowed_signers_sha256",
        "signature_algorithm",
        "signature_namespace",
        "issued_at",
        "expires_at",
        "single_use",
        "promotion_authority",
        "production_authority",
        "unregister_authority",
        "delete_authority",
    ] {
        require_kv(
            &challenge,
            field,
            authority
                .get(field)
                .ok_or_else(|| invalid("Linux authority/challenge binding is absent"))?,
        )?;
    }
    require_kv(&authority, "challenge_sha256", &sha256(&challenge_bytes))?;
    let signature = manifest.bytes("driver/operator-authority/CHALLENGE.txt.sig")?;
    require_kv(&authority, "detached_signature_sha256", &sha256(&signature))?;
    if manifest.bytes("driver/operator-authority/ALLOWED-SIGNERS")? != trust_allowed {
        return Err(invalid(
            "Linux authority allowed-signers differs from the frozen trust policy",
        ));
    }
    verify_sshsig_bytes(
        &challenge_bytes,
        &signature,
        &trust_allowed,
        profiles::LINUX_OPERATOR_PRINCIPAL,
        profiles::LINUX_OPERATOR_SIGNATURE_NAMESPACE,
    )?;

    verify_linux_v5_freeze_restore_lifecycle(
        manifest,
        inner_manifest,
        inner,
        outer,
        candidate,
        nonce,
        &authority_sha,
        &contract_bytes,
        &trust_allowed,
    )
}

#[allow(clippy::too_many_arguments)]
fn verify_linux_v5_freeze_restore_lifecycle(
    manifest: &VerifiedManifest,
    inner_manifest: &VerifiedManifest,
    inner: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
    candidate: &CandidateBindingV3,
    nonce: &str,
    authority_sha: &str,
    contract_bytes: &[u8],
    trust_allowed: &[u8],
) -> Result<(), AcceptanceError> {
    let runner_freeze_sha = verify_linux_flat_contract_seal(manifest, "driver/runner-freeze")?;
    let workload_freeze_sha = verify_linux_flat_contract_seal(manifest, "driver/workload-freeze")?;
    let host_lock_sha = verify_linux_flat_contract_seal(manifest, "driver/host-lock-profile")?;
    let execution_sha = verify_linux_flat_contract_seal(manifest, "execution-authorization")?;
    let runner_restore_sha = verify_linux_flat_contract_seal(manifest, "runner-restore")?;
    let workload_restore_sha = verify_linux_flat_contract_seal(manifest, "workload-restore")?;
    for (field, digest) in [
        ("runner_freeze_sha256", &runner_freeze_sha),
        ("host_workload_freeze_sha256", &workload_freeze_sha),
        ("host_lock_profile_sha256", &host_lock_sha),
        ("execution_authorization_sha256", &execution_sha),
        ("runner_restore_sha256", &runner_restore_sha),
        ("host_workload_restore_sha256", &workload_restore_sha),
    ] {
        require_kv(outer, field, digest)?;
    }
    for field in ["runner_restore_verified", "host_workload_restore_verified"] {
        require_kv_bool(outer, field, true)?;
    }
    for field in [
        "candidate_nix_process_selected_for_workload_freeze",
        "nix_container_volume_source_mutated",
    ] {
        require_kv_bool(outer, field, false)?;
    }

    verify_linux_runner_freeze(
        manifest,
        candidate,
        nonce,
        authority_sha,
        &runner_freeze_sha,
    )?;
    verify_linux_workload_freeze(
        manifest,
        candidate,
        nonce,
        authority_sha,
        &workload_freeze_sha,
    )?;
    verify_linux_host_lock_profile(
        manifest,
        candidate,
        nonce,
        authority_sha,
        &runner_freeze_sha,
        &workload_freeze_sha,
        &host_lock_sha,
    )?;
    verify_linux_execution_authorization(
        manifest,
        candidate,
        nonce,
        authority_sha,
        contract_bytes,
        trust_allowed,
        &runner_freeze_sha,
        &workload_freeze_sha,
        &host_lock_sha,
        &execution_sha,
    )?;
    verify_linux_restores(
        manifest,
        candidate,
        nonce,
        authority_sha,
        &runner_freeze_sha,
        &workload_freeze_sha,
        &runner_restore_sha,
        &workload_restore_sha,
    )?;
    verify_linux_inner_host_lock(inner_manifest, inner, outer, nonce)?;
    verify_linux_legacy_production(manifest)?;
    Ok(())
}

fn parse_linux_contract_tsv(
    bytes: &[u8],
    columns: usize,
    label: &str,
) -> Result<Vec<Vec<String>>, AcceptanceError> {
    if bytes.is_empty() || !bytes.ends_with(b"\n") || bytes.contains(&b'\r') {
        return Err(invalid(format!(
            "Linux {label} TSV is empty or not canonical LF text"
        )));
    }
    let text = std::str::from_utf8(bytes)
        .map_err(|_| invalid(format!("Linux {label} TSV is not UTF-8")))?;
    let mut rows = Vec::new();
    for line in text.lines() {
        let fields = line.split('\t').map(str::to_string).collect::<Vec<_>>();
        if fields.len() != columns || fields.iter().any(String::is_empty) {
            return Err(invalid(format!(
                "Linux {label} TSV differs from its {columns}-column contract"
            )));
        }
        rows.push(fields);
    }
    if rows.is_empty() {
        return Err(invalid(format!("Linux {label} TSV has no rows")));
    }
    Ok(rows)
}

fn require_linux_lifecycle_identity(
    values: &BTreeMap<String, String>,
    candidate: &CandidateBindingV3,
    nonce: &str,
) -> Result<(), AcceptanceError> {
    require_kv_u64(values, "driver_revision", 5)?;
    require_kv(values, "candidate_head", &candidate.head)?;
    require_kv(values, "candidate_tree", &candidate.tree)?;
    require_kv(values, "linux_nonce", nonce)?;
    require_kv(values, "qualification_host", "desktop-ts")
}

fn valid_linux_safe_token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(byte, b'_' | b'.' | b':' | b'@' | b'/' | b'+' | b',' | b'-')
        })
}

fn valid_linux_runner_version(value: &str) -> bool {
    let parts = value.split('.').collect::<Vec<_>>();
    (2..=4).contains(&parts.len())
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
}

fn verify_linux_runner_freeze(
    manifest: &VerifiedManifest,
    candidate: &CandidateBindingV3,
    nonce: &str,
    authority_sha: &str,
    freeze_sha: &str,
) -> Result<(), AcceptanceError> {
    let values = parse_key_values(&manifest.bytes("driver/runner-freeze/RUNNER-FREEZE.txt")?)?;
    require_exact_kv_fields(
        &values,
        &[
            "schema",
            "status",
            "driver_revision",
            "candidate_head",
            "candidate_tree",
            "linux_nonce",
            "qualification_host",
            "operator_authority_sha256",
            "runner_inventory_sha256",
            "service_inventory_sha256",
            "restore_plan_sha256",
            "runner_count",
            "workers_before",
            "job_idle_verified",
            "listeners_after",
            "workers_after",
            "controls_paused",
            "systemd_unit_required",
            "runner_identity_exact",
            "control_kind_inventory_exact",
            "freeze_held_until_terminal_copy",
            "unregister_performed",
            "delete_performed",
            "legacy_production_touched",
            "frozen_at",
        ],
    )?;
    require_kv_schema(&values, "hepta_vnext_linux_runner_freeze_v1")?;
    require_kv(&values, "status", "FROZEN")?;
    require_linux_lifecycle_identity(&values, candidate, nonce)?;
    require_kv(&values, "operator_authority_sha256", authority_sha)?;
    let runner_bytes = manifest.bytes("driver/runner-freeze/RUNNER-INVENTORY.tsv")?;
    let service_bytes = manifest.bytes("driver/runner-freeze/SERVICE-INVENTORY.tsv")?;
    require_kv(&values, "runner_inventory_sha256", &sha256(&runner_bytes))?;
    require_kv(&values, "service_inventory_sha256", &sha256(&service_bytes))?;
    require_kv(
        &values,
        "restore_plan_sha256",
        &sha256(&manifest.bytes("driver/runner-freeze/RESTORE-PLAN.txt")?),
    )?;
    let runners = parse_linux_contract_tsv(&runner_bytes, 14, "runner inventory")?;
    let services = parse_linux_contract_tsv(&service_bytes, 10, "runner service inventory")?;
    require_kv_u64(&values, "runner_count", runners.len() as u64)?;
    let runner_names = runners.iter().map(|row| &row[0]).collect::<BTreeSet<_>>();
    let service_names = services.iter().map(|row| &row[0]).collect::<BTreeSet<_>>();
    if runner_names.len() != runners.len()
        || service_names.len() != services.len()
        || runners.len() != services.len()
        || runner_names != service_names
    {
        return Err(invalid(
            "Linux runner/service freeze inventories do not map one-to-one",
        ));
    }
    for row in &runners {
        if !valid_linux_safe_token(&row[0])
            || !valid_linux_safe_token(&row[1])
            || !valid_linux_runner_version(&row[2])
            || [&row[3], &row[10], &row[11], &row[12]]
                .iter()
                .any(|value| !digest_shape(value))
            || !row[4].starts_with("/data/")
            || !row[9].starts_with('/')
            || [&row[5], &row[6], &row[7], &row[8]]
                .iter()
                .any(|value| !value.bytes().all(|byte| byte.is_ascii_digit()))
            || row[13] != "true"
        {
            return Err(invalid("Linux frozen runner identity is malformed"));
        }
    }
    for row in &services {
        let expected_pause = match row[1].as_str() {
            "systemd_unit" => "systemd_runtime_mask_stop",
            "process_supervisor" => "supervisor_cooperative_stop",
            "launch_script" => "launch_script_cooperative_stop",
            _ => return Err(invalid("Linux runner control kind is unsupported")),
        };
        if row[4] != expected_pause
            || !row[2].starts_with('/')
            || [&row[3], &row[5], &row[6]]
                .iter()
                .any(|value| !digest_shape(value))
            || !matches!(row[7].as_str(), "enabled" | "disabled" | "not_applicable")
            || !matches!(row[8].as_str(), "active" | "inactive")
            || !matches!(row[9].as_str(), "direct_child" | "descendant" | "self_exec")
        {
            return Err(invalid("Linux runner control inventory is malformed"));
        }
    }
    let restore_plan = manifest.bytes("driver/runner-freeze/RESTORE-PLAN.txt")?;
    if restore_plan.is_empty() || restore_plan.contains(&b'\r') || !restore_plan.ends_with(b"\n") {
        return Err(invalid(
            "Linux runner restore plan is not canonical LF text",
        ));
    }
    for field in [
        "job_idle_verified",
        "controls_paused",
        "runner_identity_exact",
        "control_kind_inventory_exact",
        "freeze_held_until_terminal_copy",
    ] {
        require_kv_bool(&values, field, true)?;
    }
    for field in [
        "systemd_unit_required",
        "unregister_performed",
        "delete_performed",
        "legacy_production_touched",
    ] {
        require_kv_bool(&values, field, false)?;
    }
    for field in ["workers_before", "listeners_after", "workers_after"] {
        require_kv_u64(&values, field, 0)?;
    }
    if !values
        .get("frozen_at")
        .is_some_and(|value| valid_utc_timestamp(value))
        || !digest_shape(freeze_sha)
    {
        return Err(invalid("Linux runner freeze time or seal is malformed"));
    }
    Ok(())
}

fn verify_linux_workload_freeze(
    manifest: &VerifiedManifest,
    candidate: &CandidateBindingV3,
    nonce: &str,
    authority_sha: &str,
    freeze_sha: &str,
) -> Result<(), AcceptanceError> {
    let values = parse_key_values(&manifest.bytes("driver/workload-freeze/WORKLOAD-FREEZE.txt")?)?;
    require_exact_kv_fields(
        &values,
        &[
            "schema",
            "status",
            "driver_revision",
            "candidate_head",
            "candidate_tree",
            "linux_nonce",
            "nix_nonce",
            "qualification_host",
            "operator_authority_sha256",
            "nix_pass_binding_sha256",
            "workload_inventory_sha256",
            "restore_plan_sha256",
            "workload_count",
            "workload_class",
            "nix_pass_verified_before_freeze",
            "candidate_nix_process_selected",
            "nix_container_volume_source_mutated",
            "workloads_paused",
            "workload_identity_exact",
            "freeze_held_until_terminal_copy",
            "kill_minus_nine_performed",
            "delete_performed",
            "nix_pass_verified_at",
            "frozen_at",
        ],
    )?;
    require_kv_schema(&values, "hepta_vnext_linux_host_workload_freeze_v1")?;
    require_kv(&values, "status", "FROZEN")?;
    require_linux_lifecycle_identity(&values, candidate, nonce)?;
    require_kv(&values, "nix_nonce", "52ec08130755")?;
    require_kv(&values, "operator_authority_sha256", authority_sha)?;
    let nix_binding = manifest.bytes("driver/workload-freeze/NIX-PASS-BINDING.txt")?;
    let workload_bytes = manifest.bytes("driver/workload-freeze/WORKLOAD-INVENTORY.tsv")?;
    let restore_plan = manifest.bytes("driver/workload-freeze/WORKLOAD-RESTORE-PLAN.tsv")?;
    require_kv(&values, "nix_pass_binding_sha256", &sha256(&nix_binding))?;
    if nix_binding != manifest.bytes("driver/NIX-PASS-BINDING.txt")? {
        return Err(invalid(
            "Linux workload freeze Nix binding differs from the sealed driver binding",
        ));
    }
    require_kv(
        &values,
        "workload_inventory_sha256",
        &sha256(&workload_bytes),
    )?;
    require_kv(&values, "restore_plan_sha256", &sha256(&restore_plan))?;
    let workloads = parse_linux_contract_tsv(&workload_bytes, 16, "workload inventory")?;
    let plans = parse_linux_contract_tsv(&restore_plan, 4, "workload restore plan")?;
    require_kv_u64(&values, "workload_count", workloads.len() as u64)?;
    let workload_ids = workloads.iter().map(|row| &row[0]).collect::<BTreeSet<_>>();
    let plan_ids = plans.iter().map(|row| &row[0]).collect::<BTreeSet<_>>();
    if workload_ids.len() != workloads.len()
        || plan_ids.len() != plans.len()
        || workloads.len() != plans.len()
        || workload_ids != plan_ids
    {
        return Err(invalid(
            "Linux workload freeze/restore plan does not map one-to-one",
        ));
    }
    require_kv(&values, "workload_class", "independent_heavy_build")?;
    for row in &workloads {
        let lowered = row.join("\t").to_ascii_lowercase();
        if !valid_linux_safe_token(&row[0])
            || !valid_linux_safe_token(&row[2])
            || row[1] != "independent_heavy_build"
            || (lowered.contains("nix") && lowered.contains("hepta"))
            || !row[3].starts_with("/data/")
            || !row[8].starts_with('/')
            || [&row[4], &row[5], &row[6], &row[7]]
                .iter()
                .any(|value| !value.bytes().all(|byte| byte.is_ascii_digit()))
            || [&row[9], &row[10], &row[13], &row[14], &row[15]]
                .iter()
                .any(|value| !digest_shape(value))
            || !matches!(row[11].as_str(), "process_supervisor" | "launch_script")
            || !row[12].starts_with('/')
        {
            return Err(invalid(
                "Linux workload freeze selected a non-independent or candidate Nix workload",
            ));
        }
    }
    for row in &plans {
        if !matches!(row[1].as_str(), "process_supervisor" | "launch_script")
            || !digest_shape(&row[2])
            || !matches!(row[3].as_str(), "active" | "inactive")
        {
            return Err(invalid("Linux workload restore plan is malformed"));
        }
    }
    for field in [
        "nix_pass_verified_before_freeze",
        "workloads_paused",
        "workload_identity_exact",
        "freeze_held_until_terminal_copy",
    ] {
        require_kv_bool(&values, field, true)?;
    }
    for field in [
        "candidate_nix_process_selected",
        "nix_container_volume_source_mutated",
        "kill_minus_nine_performed",
        "delete_performed",
    ] {
        require_kv_bool(&values, field, false)?;
    }
    for field in ["nix_pass_verified_at", "frozen_at"] {
        if !values
            .get(field)
            .is_some_and(|value| valid_utc_timestamp(value))
        {
            return Err(invalid("Linux workload freeze time is malformed"));
        }
    }
    if values["frozen_at"] < values["nix_pass_verified_at"] || !digest_shape(freeze_sha) {
        return Err(invalid(
            "Linux workload freeze predates Nix PASS or has a malformed seal",
        ));
    }
    Ok(())
}

fn verify_linux_host_lock_profile(
    manifest: &VerifiedManifest,
    candidate: &CandidateBindingV3,
    nonce: &str,
    authority_sha: &str,
    runner_freeze_sha: &str,
    workload_freeze_sha: &str,
    host_lock_sha: &str,
) -> Result<(), AcceptanceError> {
    let bytes = manifest.bytes("driver/host-lock-profile/MODES.tsv")?;
    if bytes != manifest.bytes("driver/host-lock-profile/INVENTORY.tsv")? {
        return Err(invalid(
            "Linux host-lock compatibility inventory differs from MODES.tsv",
        ));
    }
    let values = parse_key_values(&manifest.bytes("driver/host-lock-profile/PROFILE.txt")?)?;
    require_exact_kv_fields(
        &values,
        &[
            "schema",
            "status",
            "driver_revision",
            "linux_nonce",
            "candidate_head",
            "candidate_tree",
            "nix_nonce",
            "nix_attempt_root",
            "nix_binding_sha256",
            "runner_freeze_sha256",
            "host_workload_freeze_sha256",
            "lock_path",
            "lock_device",
            "lock_inode",
            "lock_uid",
            "lock_gid",
            "lock_mode",
            "lock_nlink",
            "lock_created",
            "lock_replaced",
            "lock_unlinked",
            "lock_initially_unheld",
            "parent_chain_root_owned",
            "parent_chain_group_other_writable",
            "observed_at",
        ],
    )?;
    require_kv_schema(&values, "hepta_vnext_host_lock_profile_v5")?;
    require_kv(&values, "status", "PASS")?;
    require_kv_u64(&values, "driver_revision", 5)?;
    require_kv(&values, "linux_nonce", nonce)?;
    require_kv(&values, "candidate_head", &candidate.head)?;
    require_kv(&values, "candidate_tree", &candidate.tree)?;
    require_kv(&values, "nix_nonce", "52ec08130755")?;
    require_kv(
        &values,
        "nix_attempt_root",
        "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-nix-exact-v3-attempt-3-20260813T065739Z/attempt-52ec08130755",
    )?;
    let nix_copy = manifest.bytes("driver/host-lock-profile/NIX-BINDING.txt")?;
    require_kv(&values, "nix_binding_sha256", &sha256(&nix_copy))?;
    let nix_binding = parse_key_values(&nix_copy)?;
    require_exact_kv_fields(
        &nix_binding,
        &[
            "schema",
            "driver_revision",
            "candidate_head",
            "candidate_tree",
            "nix_nonce",
            "nix_attempt_root",
            "nix_outer_sha256sums_sha256",
            "nix_outer_modes_sha256",
            "nix_result_sha256",
            "nix_status",
            "nix_qualification",
        ],
    )?;
    require_kv_schema(&nix_binding, "hepta_vnext_host_lock_nix_binding_v1")?;
    require_kv_u64(&nix_binding, "driver_revision", 5)?;
    require_kv(&nix_binding, "candidate_head", &candidate.head)?;
    require_kv(&nix_binding, "candidate_tree", &candidate.tree)?;
    require_kv(&nix_binding, "nix_nonce", "52ec08130755")?;
    require_kv(
        &nix_binding,
        "nix_attempt_root",
        "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-nix-exact-v3-attempt-3-20260813T065739Z/attempt-52ec08130755",
    )?;
    let driver_nix_binding = parse_key_values(&manifest.bytes("driver/NIX-PASS-BINDING.txt")?)?;
    for field in [
        "candidate_head",
        "candidate_tree",
        "nix_nonce",
        "nix_attempt_root",
        "nix_outer_sha256sums_sha256",
        "nix_outer_modes_sha256",
        "nix_result_sha256",
        "nix_status",
        "nix_qualification",
    ] {
        require_kv(
            &nix_binding,
            field,
            driver_nix_binding
                .get(field)
                .ok_or_else(|| invalid("Linux driver Nix binding field is absent"))?,
        )?;
    }
    for field in [
        "nix_outer_sha256sums_sha256",
        "nix_outer_modes_sha256",
        "nix_result_sha256",
    ] {
        if !nix_binding
            .get(field)
            .is_some_and(|value| digest_shape(value))
        {
            return Err(invalid("Linux host-lock Nix digest is malformed"));
        }
    }
    require_kv(&nix_binding, "nix_status", "PASS")?;
    require_kv_bool(&nix_binding, "nix_qualification", true)?;

    let runner_binding =
        parse_key_values(&manifest.bytes("driver/host-lock-profile/RUNNER-FREEZE-BINDING.txt")?)?;
    require_exact_kv_fields(
        &runner_binding,
        &[
            "schema",
            "driver_revision",
            "linux_nonce",
            "operator_authority_sha256",
            "runner_freeze_sha256",
            "host_workload_freeze_sha256",
            "runner_freeze_status",
            "controls_paused",
            "restore_required",
        ],
    )?;
    require_kv_schema(
        &runner_binding,
        "hepta_vnext_host_lock_runner_freeze_binding_v1",
    )?;
    require_kv_u64(&runner_binding, "driver_revision", 5)?;
    require_kv(&runner_binding, "linux_nonce", nonce)?;
    require_kv(&runner_binding, "operator_authority_sha256", authority_sha)?;
    require_kv(&runner_binding, "runner_freeze_sha256", runner_freeze_sha)?;
    require_kv(
        &runner_binding,
        "host_workload_freeze_sha256",
        workload_freeze_sha,
    )?;
    require_kv(&runner_binding, "runner_freeze_status", "FROZEN")?;
    require_kv_bool(&runner_binding, "controls_paused", true)?;
    require_kv_bool(&runner_binding, "restore_required", true)?;

    let workload_binding =
        parse_key_values(&manifest.bytes("driver/host-lock-profile/WORKLOAD-FREEZE-BINDING.txt")?)?;
    require_exact_kv_fields(
        &workload_binding,
        &[
            "schema",
            "driver_revision",
            "linux_nonce",
            "operator_authority_sha256",
            "host_workload_freeze_sha256",
            "nix_pass_binding_sha256",
            "workload_class",
            "candidate_nix_process_selected",
            "nix_container_volume_source_mutated",
            "restore_required",
        ],
    )?;
    require_kv_schema(
        &workload_binding,
        "hepta_vnext_host_lock_workload_freeze_binding_v1",
    )?;
    require_kv_u64(&workload_binding, "driver_revision", 5)?;
    require_kv(&workload_binding, "linux_nonce", nonce)?;
    require_kv(
        &workload_binding,
        "operator_authority_sha256",
        authority_sha,
    )?;
    require_kv(
        &workload_binding,
        "host_workload_freeze_sha256",
        workload_freeze_sha,
    )?;
    require_kv(
        &workload_binding,
        "nix_pass_binding_sha256",
        &sha256(&manifest.bytes("driver/NIX-PASS-BINDING.txt")?),
    )?;
    require_kv(
        &workload_binding,
        "workload_class",
        "independent_heavy_build",
    )?;
    require_kv_bool(&workload_binding, "candidate_nix_process_selected", false)?;
    require_kv_bool(
        &workload_binding,
        "nix_container_volume_source_mutated",
        false,
    )?;
    require_kv_bool(&workload_binding, "restore_required", true)?;
    require_kv(&values, "runner_freeze_sha256", runner_freeze_sha)?;
    require_kv(&values, "host_workload_freeze_sha256", workload_freeze_sha)?;
    for field in ["lock_device", "lock_inode"] {
        if parse_decimal_field(&values, field)? == 0 {
            return Err(invalid("Linux host-lock device/inode is zero"));
        }
    }
    for (field, expected) in [
        (
            "lock_path",
            "/var/lib/hepta-vnext/locks/host-qualification.lock",
        ),
        ("lock_uid", "0"),
        ("lock_gid", "1000"),
        ("lock_mode", "0640"),
        ("lock_nlink", "1"),
    ] {
        require_kv(&values, field, expected)?;
    }
    for field in ["lock_initially_unheld", "parent_chain_root_owned"] {
        require_kv_bool(&values, field, true)?;
    }
    let _ = kv_bool(&values, "lock_created")?;
    for field in [
        "lock_replaced",
        "lock_unlinked",
        "parent_chain_group_other_writable",
    ] {
        require_kv_bool(&values, field, false)?;
    }
    if !values
        .get("observed_at")
        .is_some_and(|value| valid_utc_timestamp(value))
        || !digest_shape(host_lock_sha)
    {
        return Err(invalid("Linux host-lock profile time or seal is malformed"));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn verify_linux_execution_authorization(
    manifest: &VerifiedManifest,
    candidate: &CandidateBindingV3,
    nonce: &str,
    authority_sha: &str,
    contract_bytes: &[u8],
    trust_allowed: &[u8],
    runner_freeze_sha: &str,
    workload_freeze_sha: &str,
    host_lock_sha: &str,
    execution_sha: &str,
) -> Result<(), AcceptanceError> {
    let statement = manifest.bytes("execution-authorization/EXECUTION-AUTHORIZATION.txt")?;
    let values = parse_key_values(&statement)?;
    require_exact_kv_fields(
        &values,
        &[
            "schema",
            "status",
            "driver_revision",
            "candidate_head",
            "candidate_tree",
            "linux_nonce",
            "nix_nonce",
            "qualification_host",
            "authorization_scope",
            "operator_identity",
            "signature_algorithm",
            "signature_namespace",
            "allowed_signers_sha256",
            "single_use",
            "qualification_execution_path",
            "copy_path",
            "recovery_supported",
            "resume_supported",
            "copy_only_supported",
            "operator_authority_sha256",
            "acceptance_contract_sha256",
            "driver_manifest_sha256",
            "nix_pass_binding_sha256",
            "host_observation_sha256sums_sha256",
            "host_lock_profile_sha256",
            "runner_freeze_sha256",
            "host_workload_freeze_sha256",
            "remote_attempt_root",
            "remote_input_root",
            "remote_driver_root",
            "remote_run_root",
            "issued_at",
            "expires_at",
            "promotion_authority",
        ],
    )?;
    require_kv_schema(&values, "hepta_vnext_linux_execution_authorization_v1")?;
    require_kv(&values, "status", "AUTHORIZED")?;
    require_linux_lifecycle_identity(&values, candidate, nonce)?;
    require_kv(&values, "nix_nonce", "52ec08130755")?;
    require_kv(
        &values,
        "authorization_scope",
        "single_linux_exact_v5_direct_launch",
    )?;
    require_kv(
        &values,
        "operator_identity",
        profiles::LINUX_OPERATOR_PRINCIPAL,
    )?;
    require_kv(&values, "signature_algorithm", "sshsig-ed25519")?;
    require_kv(
        &values,
        "signature_namespace",
        profiles::LINUX_OPERATOR_SIGNATURE_NAMESPACE,
    )?;
    require_kv(
        &values,
        "allowed_signers_sha256",
        profiles::LINUX_OPERATOR_ALLOWED_SIGNERS_SHA256,
    )?;
    require_kv_bool(&values, "single_use", true)?;
    for field in ["qualification_execution_path", "copy_path"] {
        require_kv(&values, field, "direct_launcher")?;
    }
    for field in [
        "recovery_supported",
        "resume_supported",
        "copy_only_supported",
        "promotion_authority",
    ] {
        require_kv_bool(&values, field, false)?;
    }
    require_kv(&values, "operator_authority_sha256", authority_sha)?;
    require_kv(
        &values,
        "acceptance_contract_sha256",
        &sha256(contract_bytes),
    )?;
    require_kv(
        &values,
        "driver_manifest_sha256",
        &sha256(&manifest.bytes("driver/DRIVER-SHA256SUMS")?),
    )?;
    require_kv(
        &values,
        "nix_pass_binding_sha256",
        &sha256(&manifest.bytes("driver/NIX-PASS-BINDING.txt")?),
    )?;
    require_kv(
        &values,
        "host_observation_sha256sums_sha256",
        &sha256(&manifest.bytes("driver/host-observation/SHA256SUMS")?),
    )?;
    require_kv(&values, "host_lock_profile_sha256", host_lock_sha)?;
    require_kv(&values, "runner_freeze_sha256", runner_freeze_sha)?;
    require_kv(&values, "host_workload_freeze_sha256", workload_freeze_sha)?;
    let attempt = format!("/data/hepta-linux-exact-52ec-v5-{nonce}");
    for (field, expected) in [
        ("remote_attempt_root", attempt.clone()),
        ("remote_input_root", format!("{attempt}/input")),
        ("remote_driver_root", format!("{attempt}/input/drivers")),
        ("remote_run_root", format!("{attempt}/run")),
    ] {
        require_kv(&values, field, &expected)?;
    }
    verify_linux_authorization_window(&values)?;
    let signature = manifest.bytes("execution-authorization/EXECUTION-AUTHORIZATION.txt.sig")?;
    verify_sshsig_bytes(
        &statement,
        &signature,
        trust_allowed,
        profiles::LINUX_OPERATOR_PRINCIPAL,
        profiles::LINUX_OPERATOR_SIGNATURE_NAMESPACE,
    )?;
    if !digest_shape(execution_sha) {
        return Err(invalid(
            "Linux execution authorization seal digest is malformed",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn verify_linux_restores(
    manifest: &VerifiedManifest,
    candidate: &CandidateBindingV3,
    nonce: &str,
    authority_sha: &str,
    runner_freeze_sha: &str,
    workload_freeze_sha: &str,
    runner_restore_sha: &str,
    workload_restore_sha: &str,
) -> Result<(), AcceptanceError> {
    let runner = parse_key_values(&manifest.bytes("runner-restore/RUNNER-RESTORE.txt")?)?;
    require_exact_kv_fields(
        &runner,
        &[
            "schema",
            "status",
            "driver_revision",
            "candidate_head",
            "candidate_tree",
            "linux_nonce",
            "qualification_host",
            "operator_authority_sha256",
            "runner_freeze_sha256",
            "incoming_root",
            "copied_manifest_sha256",
            "pre_restore_inventory_sha256",
            "post_restore_inventory_sha256",
            "service_restore_sha256",
            "same_runner_identity",
            "prior_enabled_state_restored",
            "prior_active_state_restored",
            "runner_online",
            "listeners_restored",
            "workers_after",
            "controls_restored",
            "systemd_unit_required",
            "unregister_performed",
            "delete_performed",
            "legacy_production_touched",
            "restored_at",
        ],
    )?;
    require_kv_schema(&runner, "hepta_vnext_linux_runner_restore_v1")?;
    require_kv(&runner, "status", "RESTORED")?;
    require_linux_lifecycle_identity(&runner, candidate, nonce)?;
    require_kv(&runner, "operator_authority_sha256", authority_sha)?;
    require_kv(&runner, "runner_freeze_sha256", runner_freeze_sha)?;
    let receipt_parent = manifest
        .root
        .parent()
        .ok_or_else(|| invalid("Linux receipt root has no parent"))?;
    let incoming_root = receipt_parent.join(format!(".attempt-{nonce}.incoming"));
    require_kv(
        &runner,
        "incoming_root",
        incoming_root
            .to_str()
            .ok_or_else(|| invalid("Linux incoming root is not UTF-8"))?,
    )?;
    require_kv(
        &runner,
        "copied_manifest_sha256",
        &sha256(&manifest.bytes("COPIED.sha256")?),
    )?;
    let runner_pre = manifest.bytes("runner-restore/PRE-RESTORE-INVENTORY.tsv")?;
    let runner_post = manifest.bytes("runner-restore/POST-RESTORE-INVENTORY.tsv")?;
    if runner_pre != manifest.bytes("driver/runner-freeze/RUNNER-INVENTORY.tsv")? {
        return Err(invalid(
            "Linux runner pre-restore inventory differs from the frozen inventory",
        ));
    }
    let before = parse_linux_contract_tsv(&runner_pre, 14, "runner pre-restore inventory")?;
    let after = parse_linux_contract_tsv(&runner_post, 14, "runner post-restore inventory")?;
    if before.len() != after.len()
        || before.iter().map(|row| &row[0]).collect::<BTreeSet<_>>()
            != after.iter().map(|row| &row[0]).collect::<BTreeSet<_>>()
    {
        return Err(invalid("Linux runner identity set changed across restore"));
    }
    let after_by_name = after
        .iter()
        .map(|row| (row[0].as_str(), row))
        .collect::<BTreeMap<_, _>>();
    for prior in &before {
        let observed = after_by_name
            .get(prior[0].as_str())
            .ok_or_else(|| invalid("Linux restored runner identity is absent"))?;
        for index in [0, 1, 2, 3, 4, 5, 6, 9, 10, 11, 12] {
            if prior[index] != observed[index] {
                return Err(invalid(
                    "Linux stable runner identity changed across restore",
                ));
            }
        }
        if !observed[7].bytes().all(|byte| byte.is_ascii_digit())
            || !observed[8].bytes().all(|byte| byte.is_ascii_digit())
            || observed[13] != "true"
        {
            return Err(invalid(
                "Linux runner listener was not restored online with numeric identity",
            ));
        }
    }
    for field in [
        "same_runner_identity",
        "prior_enabled_state_restored",
        "prior_active_state_restored",
        "runner_online",
        "listeners_restored",
        "controls_restored",
    ] {
        require_kv_bool(&runner, field, true)?;
    }
    require_kv_u64(&runner, "workers_after", 0)?;
    for field in [
        "systemd_unit_required",
        "unregister_performed",
        "delete_performed",
        "legacy_production_touched",
    ] {
        require_kv_bool(&runner, field, false)?;
    }
    for (field, bytes) in [
        ("pre_restore_inventory_sha256", &runner_pre),
        ("post_restore_inventory_sha256", &runner_post),
    ] {
        require_kv(&runner, field, &sha256(bytes))?;
    }
    require_kv(
        &runner,
        "service_restore_sha256",
        &sha256(&manifest.bytes("runner-restore/SERVICE-RESTORE.tsv")?),
    )?;
    let frozen_services = parse_linux_contract_tsv(
        &manifest.bytes("driver/runner-freeze/SERVICE-INVENTORY.tsv")?,
        10,
        "runner frozen service inventory",
    )?;
    let restored_services = parse_linux_contract_tsv(
        &manifest.bytes("runner-restore/SERVICE-RESTORE.tsv")?,
        10,
        "runner service restore results",
    )?;
    let restored_by_name = restored_services
        .iter()
        .map(|row| (row[0].as_str(), row))
        .collect::<BTreeMap<_, _>>();
    if restored_by_name.len() != restored_services.len()
        || restored_by_name.len() != frozen_services.len()
    {
        return Err(invalid("Linux runner service set changed across restore"));
    }
    for prior in &frozen_services {
        let restored = restored_by_name
            .get(prior[0].as_str())
            .ok_or_else(|| invalid("Linux restored runner service is absent"))?;
        let expected_primitive = match prior[1].as_str() {
            "systemd_unit" => "systemd_unmask_restore",
            "process_supervisor" => "supervisor_restore",
            "launch_script" => "launch_script_restore",
            _ => return Err(invalid("Linux frozen runner control kind is unsupported")),
        };
        if restored[1..4] != prior[1..4]
            || restored[4] != expected_primitive
            || restored[5] != prior[6]
            || restored[6] != prior[7]
            || restored[7] != prior[8]
            || restored[8] != "0"
            || restored[9] != "true"
        {
            return Err(invalid(
                "Linux runner service restore identity or result differs",
            ));
        }
    }

    let workload = parse_key_values(&manifest.bytes("workload-restore/WORKLOAD-RESTORE.txt")?)?;
    require_exact_kv_fields(
        &workload,
        &[
            "schema",
            "status",
            "driver_revision",
            "candidate_head",
            "candidate_tree",
            "linux_nonce",
            "nix_nonce",
            "qualification_host",
            "operator_authority_sha256",
            "host_workload_freeze_sha256",
            "incoming_root",
            "copied_manifest_sha256",
            "pre_restore_workloads_sha256",
            "post_restore_workloads_sha256",
            "workload_restore_sha256",
            "same_workload_identity",
            "prior_active_state_restored",
            "nix_container_volume_source_mutated",
            "kill_minus_nine_performed",
            "delete_performed",
            "restored_at",
        ],
    )?;
    require_kv_schema(&workload, "hepta_vnext_linux_host_workload_restore_v1")?;
    require_kv(&workload, "status", "RESTORED")?;
    require_linux_lifecycle_identity(&workload, candidate, nonce)?;
    require_kv(&workload, "nix_nonce", "52ec08130755")?;
    require_kv(&workload, "operator_authority_sha256", authority_sha)?;
    require_kv(
        &workload,
        "host_workload_freeze_sha256",
        workload_freeze_sha,
    )?;
    require_kv(
        &workload,
        "incoming_root",
        incoming_root
            .to_str()
            .ok_or_else(|| invalid("Linux incoming root is not UTF-8"))?,
    )?;
    require_kv(
        &workload,
        "copied_manifest_sha256",
        &sha256(&manifest.bytes("COPIED.sha256")?),
    )?;
    let workload_pre = manifest.bytes("workload-restore/PRE-RESTORE-WORKLOADS.tsv")?;
    let workload_post = manifest.bytes("workload-restore/POST-RESTORE-WORKLOADS.tsv")?;
    if workload_pre != manifest.bytes("driver/workload-freeze/WORKLOAD-INVENTORY.tsv")? {
        return Err(invalid(
            "Linux workload pre-restore inventory differs from the frozen inventory",
        ));
    }
    let before = parse_linux_contract_tsv(&workload_pre, 16, "workload pre-restore inventory")?;
    let after = parse_linux_contract_tsv(&workload_post, 16, "workload post-restore inventory")?;
    if before.len() != after.len()
        || before.iter().map(|row| &row[0]).collect::<BTreeSet<_>>()
            != after.iter().map(|row| &row[0]).collect::<BTreeSet<_>>()
    {
        return Err(invalid(
            "Linux independent workload identity set changed across restore",
        ));
    }
    let after_by_id = after
        .iter()
        .map(|row| (row[0].as_str(), row))
        .collect::<BTreeMap<_, _>>();
    for prior in &before {
        let observed = after_by_id
            .get(prior[0].as_str())
            .ok_or_else(|| invalid("Linux restored workload identity is absent"))?;
        for index in [0, 1, 2, 3, 4, 5, 8, 9, 10, 11, 12, 13, 14, 15] {
            if prior[index] != observed[index] {
                return Err(invalid(
                    "Linux stable workload identity changed across restore",
                ));
            }
        }
        if !observed[6].bytes().all(|byte| byte.is_ascii_digit())
            || !observed[7].bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(invalid(
                "Linux workload was not restored with numeric process identity",
            ));
        }
    }
    for field in ["same_workload_identity", "prior_active_state_restored"] {
        require_kv_bool(&workload, field, true)?;
    }
    for field in [
        "nix_container_volume_source_mutated",
        "kill_minus_nine_performed",
        "delete_performed",
    ] {
        require_kv_bool(&workload, field, false)?;
    }
    for (field, bytes) in [
        ("pre_restore_workloads_sha256", &workload_pre),
        ("post_restore_workloads_sha256", &workload_post),
    ] {
        require_kv(&workload, field, &sha256(bytes))?;
    }
    require_kv(
        &workload,
        "workload_restore_sha256",
        &sha256(&manifest.bytes("workload-restore/WORKLOAD-RESTORE.tsv")?),
    )?;
    let workload_restores = parse_linux_contract_tsv(
        &manifest.bytes("workload-restore/WORKLOAD-RESTORE.tsv")?,
        6,
        "workload restore results",
    )?;
    let before_by_id = before
        .iter()
        .map(|row| (row[0].as_str(), row))
        .collect::<BTreeMap<_, _>>();
    if workload_restores.len() != before_by_id.len()
        || workload_restores
            .iter()
            .map(|row| row[0].as_str())
            .collect::<BTreeSet<_>>()
            != before_by_id.keys().copied().collect::<BTreeSet<_>>()
    {
        return Err(invalid(
            "Linux workload restore results do not map one-to-one",
        ));
    }
    for restored in &workload_restores {
        let prior = before_by_id
            .get(restored[0].as_str())
            .ok_or_else(|| invalid("Linux restored workload is absent"))?;
        if restored[1] != prior[11]
            || restored[2] != prior[13]
            || restored[3] != prior[15]
            || restored[4] != "0"
            || restored[5] != "true"
        {
            return Err(invalid(
                "Linux workload restore control identity or result differs",
            ));
        }
    }
    for (values, field) in [(&runner, "restored_at"), (&workload, "restored_at")] {
        if !values
            .get(field)
            .is_some_and(|value| valid_utc_timestamp(value))
        {
            return Err(invalid("Linux restore time is malformed"));
        }
    }
    if !digest_shape(runner_restore_sha) || !digest_shape(workload_restore_sha) {
        return Err(invalid("Linux restore seal digest is malformed"));
    }
    Ok(())
}

fn verify_linux_inner_host_lock(
    manifest: &VerifiedManifest,
    inner: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
    nonce: &str,
) -> Result<(), AcceptanceError> {
    let bytes = manifest.bytes("HOST-QUALIFICATION-LOCK.txt")?;
    let values = parse_key_values(&bytes)?;
    require_exact_kv_fields(
        &values,
        &[
            "schema",
            "protocol",
            "lock_path",
            "lock_device",
            "lock_inode",
            "holder_pid",
            "holder_startticks",
            "boot_id",
            "machine_id_sha256",
            "linux_nonce",
            "candidate_head",
            "candidate_tree",
            "coordinator_payload_sha256",
            "acquired_at",
            "driver_inherits_lock_fd",
            "lock_held_through_inner_seal_required",
            "release_evidence_scope",
            "lock_file_deleted",
        ],
    )?;
    require_kv_schema(&values, "hepta_vnext_host_qualification_lock_evidence_v2")?;
    for (field, expected) in [
        ("protocol", "detached_coordinator_v5"),
        (
            "lock_path",
            "/var/lib/hepta-vnext/locks/host-qualification.lock",
        ),
        ("linux_nonce", nonce),
        ("candidate_head", CANDIDATE_HEAD),
        ("candidate_tree", CANDIDATE_TREE),
        ("release_evidence_scope", "outer_receipt_only"),
    ] {
        require_kv(&values, field, expected)?;
    }
    for field in ["driver_inherits_lock_fd", "lock_file_deleted"] {
        require_kv_bool(&values, field, false)?;
    }
    require_kv_bool(&values, "lock_held_through_inner_seal_required", true)?;
    for field in [
        "lock_device",
        "lock_inode",
        "holder_pid",
        "holder_startticks",
    ] {
        if parse_decimal_field(&values, field)? == 0 {
            return Err(invalid("Linux host-lock numeric identity is zero"));
        }
    }
    for field in ["machine_id_sha256", "coordinator_payload_sha256"] {
        if !values.get(field).is_some_and(|value| digest_shape(value)) {
            return Err(invalid("Linux host-lock digest identity is malformed"));
        }
    }
    if !values
        .get("acquired_at")
        .is_some_and(|value| valid_utc_timestamp(value))
        || values.get("boot_id").is_none_or(String::is_empty)
    {
        return Err(invalid("Linux host-lock acquisition identity is malformed"));
    }
    require_kv(
        inner,
        "host_qualification_lock_evidence_sha256",
        &sha256(&bytes),
    )?;
    for field in [
        "host_qualification_lock_exclusive_verified",
        "host_qualification_lock_held_through_inner_seal",
    ] {
        require_kv_bool(inner, field, true)?;
    }
    require_kv_bool(inner, "host_qualification_lock_released", false)?;
    for (result_field, evidence_field) in [
        ("host_qualification_lock_holder_pid", "holder_pid"),
        (
            "host_qualification_lock_holder_startticks",
            "holder_startticks",
        ),
        (
            "host_qualification_lock_coordinator_payload_sha256",
            "coordinator_payload_sha256",
        ),
    ] {
        require_kv(
            inner,
            result_field,
            values
                .get(evidence_field)
                .ok_or_else(|| invalid("Linux host-lock inner binding is absent"))?,
        )?;
    }
    for (outer_field, evidence_field) in [
        ("host_qualification_lock_device", "lock_device"),
        ("host_qualification_lock_inode", "lock_inode"),
        ("host_qualification_lock_original_holder_pid", "holder_pid"),
        (
            "host_qualification_lock_original_holder_startticks",
            "holder_startticks",
        ),
        (
            "host_qualification_lock_coordinator_payload_sha256",
            "coordinator_payload_sha256",
        ),
    ] {
        require_kv(
            outer,
            outer_field,
            values
                .get(evidence_field)
                .ok_or_else(|| invalid("Linux host-lock outer binding is absent"))?,
        )?;
    }
    for field in [
        "host_qualification_lock_original_holder_absent",
        "host_qualification_lock_same_identity_reacquired",
        "host_qualification_lock_held_through_verified_receipt_copy",
    ] {
        require_kv_bool(outer, field, true)?;
    }
    require_kv_bool(outer, "host_qualification_lock_driver_inherits_fd", false)?;
    require_kv(outer, "host_qualification_lock_release_observation", "pass")?;
    Ok(())
}

fn verify_linux_legacy_production(manifest: &VerifiedManifest) -> Result<(), AcceptanceError> {
    let pre = parse_key_values(&manifest.bytes("legacy-production-preflight.txt")?)?;
    let post = parse_key_values(&manifest.bytes("legacy-production-postflight.txt")?)?;
    require_kv_schema(&pre, "hepta_vnext_legacy_production_observation_v1")?;
    require_kv_schema(&post, "hepta_vnext_legacy_production_observation_v1")?;
    require_kv(&pre, "status", "PASS")?;
    require_kv(&post, "status", "PASS")?;
    require_kv(&pre, "phase", "pre_remote_contact")?;
    require_kv(&post, "phase", "post_remote_completion")?;
    for values in [&pre, &post] {
        for (field, expected) in [
            ("launchd_label", "ai.hepta.gateway"),
            ("launchd_state", "running"),
            ("process_ppid", "1"),
            ("listener_address", "127.0.0.1:7373"),
            (
                "executable_sha256",
                "a6ccf13cc81f62a822beea7dc1b9aa9d61c9734728d123d4fde473969c5efaf7",
            ),
            (
                "plist_sha256",
                "720c669ca6a847e47cf8717b2d633d85feb322aa19d2369e8ea3e0c1d7a3c0b2",
            ),
            ("health_url", "http://127.0.0.1:7373/health"),
            ("health_status", "200"),
            ("health_content_type", "application/json; charset=utf-8"),
            (
                "health_payload",
                r#"{"product":"Hepta","runtime":"hepta","status":"ready"}"#,
            ),
            (
                "health_payload_sha256",
                "bcf56b8536d983aaea86b4b47bab40381389b2b86b9112ba0669ea7e8c69d619",
            ),
        ] {
            require_kv(values, field, expected)?;
        }
        if !values
            .get("observed_at")
            .is_some_and(|value| valid_utc_timestamp(value))
        {
            return Err(invalid(
                "Linux legacy production observation time is malformed",
            ));
        }
    }
    let mut normalized_pre = pre.clone();
    let mut normalized_post = post.clone();
    normalized_pre.remove("phase");
    normalized_pre.remove("observed_at");
    normalized_post.remove("phase");
    normalized_post.remove("observed_at");
    if normalized_pre != normalized_post {
        return Err(invalid(
            "Linux legacy production identity changed across qualification",
        ));
    }
    Ok(())
}

fn verify_linux_v5_bindings(
    receipt: &VerifiedReceipt,
    inner: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    let outer_manifest = receipt.layer(ManifestLayerIdV3::Outer)?;
    let inner_manifest = receipt.layer(ManifestLayerIdV3::InnerReceipt)?;
    for (values, field, expected) in [
        (inner, "qualification_host", "desktop-ts"),
        (inner, "target_triple", "x86_64-unknown-linux-gnu"),
        (inner, "cargo_build_jobs", "1"),
        (inner, "cargo_incremental", "0"),
        (inner, "nix_nonce", "52ec08130755"),
        (outer, "nix_nonce", "52ec08130755"),
        (outer, "driver_relative_root", "driver"),
    ] {
        require_kv(values, field, expected)?;
    }
    let linux_nonce = inner
        .get("linux_nonce")
        .ok_or_else(|| invalid("Linux v5 nonce is absent"))?;
    if linux_nonce == "52ec08130755"
        || linux_nonce.len() < 8
        || linux_nonce.len() > 16
        || !linux_nonce
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(invalid(
            "Linux v5 nonce is malformed or aliases the Nix nonce",
        ));
    }
    for field in ["nonce", "linux_nonce"] {
        require_kv(outer, field, linux_nonce)?;
    }
    if outer.get("copy_path").map(String::as_str) != Some("direct_launcher")
        || outer_manifest.entry("RECOVERY.txt").is_some()
        || outer_manifest.entry("recovery-poll.tsv").is_some()
    {
        return Err(invalid(
            "Linux v5 permits only the direct launcher and no recovery evidence",
        ));
    }
    let attempt_root = format!("/data/hepta-linux-exact-52ec-v5-{linux_nonce}");
    for (field, expected) in [
        ("remote_attempt_root", attempt_root.clone()),
        ("remote_input_root", format!("{attempt_root}/input")),
        (
            "remote_driver_root",
            format!("{attempt_root}/input/drivers"),
        ),
        ("remote_run_root", format!("{attempt_root}/run")),
    ] {
        require_kv(inner, field, &expected)?;
        require_kv(outer, field, &expected)?;
    }
    for field in [
        "cargo_net_offline",
        "fresh_cargo_home",
        "fresh_cargo_target",
        "fresh_bazel_output_user_root",
        "nix_same_head_terminal_pass_bound",
        "host_tools_preflight_verified",
        "host_tools_postflight_verified",
        "host_executables_preflight_verified",
        "host_executables_postflight_verified",
        "build_materialization_executables_digest_bound",
        "shellcheck_static_runtime_closure",
    ] {
        require_kv_bool(inner, field, true)?;
    }
    require_kv_bool(inner, "inherited_build_results", false)?;
    for field in ["incoming_copy_preserved", "remote_roots_preserved"] {
        require_kv_bool(outer, field, true)?;
    }
    for field in [
        "driver_recursive_hashes",
        "driver_type_mode_size_directory_inventory",
        "nix_same_head_terminal_pass_binding",
        "inner_type_mode_size_directory_inventory",
    ] {
        require_kv(outer, field, "pass")?;
    }
    require_kv_u64(outer, "remote_driver_rc", 0)?;
    for field in [
        "receipt_archive_sha256",
        "driver_manifest_sha256",
        "nix_pass_binding_sha256",
        "nix_attempt_inventory_sha256",
        "nix_attempt_full_inventory_sha256",
        "nix_outer_sha256sums_sha256",
        "nix_inner_sha256sums_sha256",
        "host_tools_binding_sha256",
        "host_executables_binding_sha256",
    ] {
        if !outer.get(field).is_some_and(|value| digest_shape(value)) {
            return Err(invalid(format!("Linux outer digest is malformed: {field}")));
        }
    }
    for field in [
        "cutover_bridge_sha256",
        "nix_pass_binding_sha256",
        "nix_attempt_inventory_sha256",
        "nix_outer_sha256sums_sha256",
        "host_tools_binding_sha256",
        "host_executables_binding_sha256",
        "driver_manifest_sha256",
        "input_manifest_sha256",
        "tool_input_manifest_sha256",
        "rust_toolchain_archive_sha256",
        "rust_toolchain_archive_pin_sha256",
        "bazelisk_sha256",
        "bazel_sha256",
        "shellcheck_archive_sha256",
        "shellcheck_sha256",
    ] {
        if !inner.get(field).is_some_and(|value| digest_shape(value)) {
            return Err(invalid(format!("Linux inner digest is malformed: {field}")));
        }
    }
    require_kv(
        inner,
        "input_manifest_sha256",
        "65e9f7e70294c44b5f8e79881af6eebacdcb428452fd47bd8f5dcc54f4fd4bda",
    )?;
    require_kv(
        inner,
        "tool_input_manifest_sha256",
        "a1f398db1d435348d7486e732d7915d39936db953e864943c7569350926cb592",
    )?;
    for (field, expected) in [
        (
            "rust_toolchain_archive_sha256",
            "d5c9b7c7aeb8c00f71e87bfab0fd6c9526a0a4f92176e74dded20afd4a2d587c",
        ),
        (
            "rust_toolchain_archive_pin_sha256",
            "d5c9b7c7aeb8c00f71e87bfab0fd6c9526a0a4f92176e74dded20afd4a2d587c",
        ),
        (
            "bazelisk_sha256",
            "22e7d3a188699982f661cf4687137ee52d1f24fec1ec893d91a6c4d791a75de8",
        ),
        (
            "bazel_sha256",
            "c44a93f25398c68f904fa1d19b61d321de6c0d2f09dca375d7bc0dc9b9428403",
        ),
        (
            "shellcheck_archive_sha256",
            "8c3be12b05d5c177a04c29e3c78ce89ac86f1595681cab149b65b97c4e227198",
        ),
        (
            "shellcheck_sha256",
            "4da528ddb3a4d1b7b24a59d4e16eb2f5fd960f4bd9a3708a15baddbdf1d5a55b",
        ),
    ] {
        require_kv(inner, field, expected)?;
    }
    if !inner
        .get("completed_at")
        .is_some_and(|value| valid_utc_timestamp(value))
        || !outer
            .get("verified_at")
            .is_some_and(|value| valid_utc_timestamp(value))
    {
        return Err(invalid("Linux completion time is not an exact UTC second"));
    }

    verify_linux_host_observation(outer_manifest, inner, candidate)?;
    verify_linux_runtime_environment(outer_manifest, inner_manifest, inner, outer, linux_nonce)?;
    for (outer_path, inner_path) in [
        ("driver/NIX-PASS-BINDING.txt", "NIX-PASS-BINDING.txt"),
        (
            "driver/NIX-ATTEMPT-INVENTORY.tsv",
            "NIX-ATTEMPT-INVENTORY.tsv",
        ),
        ("driver/HOST-TOOLS.tsv", "HOST-TOOLS.tsv"),
        ("driver/HOST-EXECUTABLES.tsv", "HOST-EXECUTABLES.tsv"),
    ] {
        if outer_manifest.bytes(outer_path)? != inner_manifest.bytes(inner_path)? {
            return Err(invalid(
                "Linux inner binding differs from the sealed v5 driver",
            ));
        }
    }
    for (canonical, preflight, postflight) in [
        (
            "HOST-TOOLS.tsv",
            "host-tools-preflight.tsv",
            "host-tools-postflight.tsv",
        ),
        (
            "HOST-EXECUTABLES.tsv",
            "host-executables-preflight.tsv",
            "host-executables-postflight.tsv",
        ),
    ] {
        let canonical = inner_manifest.bytes(canonical)?;
        if canonical != inner_manifest.bytes(preflight)?
            || canonical != inner_manifest.bytes(postflight)?
        {
            return Err(invalid(
                "Linux preflight/postflight executable closure changed",
            ));
        }
    }
    verify_linux_driver_seal(outer_manifest, inner, outer)?;
    verify_linux_nix_binding(outer_manifest, inner_manifest, inner, outer, candidate)?;
    verify_linux_tool_binding(outer_manifest, inner)?;
    Ok(())
}

fn verify_linux_host_observation(
    outer_manifest: &VerifiedManifest,
    inner: &BTreeMap<String, String>,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    let observation =
        parse_key_values(&outer_manifest.bytes("driver/host-observation/OBSERVATION.txt")?)?;
    require_exact_kv_fields(
        &observation,
        &[
            "schema",
            "qualification_host",
            "target_alias",
            "observed_hostname",
            "machine_id_sha256",
            "candidate_head",
            "candidate_tree",
            "host_qualification_lock_path",
            "host_qualification_lock_device",
            "host_qualification_lock_inode",
            "host_qualification_lock_uid",
            "host_qualification_lock_gid",
            "host_qualification_lock_mode",
            "host_qualification_lock_nlink",
            "host_qualification_lock_parent_chain_root_owned",
            "host_qualification_lock_parent_chain_group_other_writable",
            "host_qualification_lock_active_records",
            "host_qualification_lock_never_unlink_policy",
            "remote_state_created",
            "active_hepta_nix",
            "running_libvirt_domains",
            "active_github_actions_runner_listeners",
            "active_github_actions_runner_workers",
            "other_active_hepta_qualification_builds",
            "remote_environment_sanitized",
            "remote_env_executable",
            "remote_bash_executable",
            "remote_bash_startup_mode",
            "bash_env_absent",
            "build_affecting_environment_allowlist_exact",
            "observed_at",
            "nix_attempt_full_inventory_sha256",
            "nix_attempt_full_inventory_entry_count",
            "nix_attempt_full_inventory_file_count",
            "nix_attempt_full_inventory_directory_count",
            "nix_attempt_full_inventory_root",
            "nix_attempt_full_inventory_pre_post_equal",
            "sanitized_environment_binding_sha256",
            "acceptance_v7_contract_sha256",
            "operator_authority_sha256",
            "runner_freeze_sha256",
            "host_workload_freeze_sha256",
            "host_lock_profile_sha256",
        ],
    )?;
    require_kv_schema(&observation, "hepta_vnext_linux_host_tool_observation_v5")?;
    for (field, expected) in [
        ("qualification_host", "desktop-ts"),
        ("target_alias", "desktop-ts"),
        ("candidate_head", candidate.head.as_str()),
        ("candidate_tree", candidate.tree.as_str()),
        (
            "host_qualification_lock_path",
            "/var/lib/hepta-vnext/locks/host-qualification.lock",
        ),
        ("host_qualification_lock_uid", "0"),
        ("host_qualification_lock_gid", "1000"),
        ("host_qualification_lock_mode", "0640"),
        ("host_qualification_lock_nlink", "1"),
        ("remote_env_executable", "/usr/bin/env"),
        ("remote_bash_executable", "/usr/bin/bash"),
        ("remote_bash_startup_mode", "--noprofile,--norc"),
    ] {
        require_kv(&observation, field, expected)?;
    }
    for field in [
        "remote_environment_sanitized",
        "bash_env_absent",
        "build_affecting_environment_allowlist_exact",
        "nix_attempt_full_inventory_pre_post_equal",
        "host_qualification_lock_parent_chain_root_owned",
        "host_qualification_lock_never_unlink_policy",
    ] {
        require_kv_bool(&observation, field, true)?;
    }
    for field in [
        "remote_state_created",
        "active_hepta_nix",
        "host_qualification_lock_parent_chain_group_other_writable",
    ] {
        require_kv_bool(&observation, field, false)?;
    }
    for field in [
        "host_qualification_lock_device",
        "host_qualification_lock_inode",
    ] {
        if parse_decimal_field(&observation, field)? == 0 {
            return Err(invalid("Linux observed host-lock identity is zero"));
        }
    }
    require_kv_u64(&observation, "host_qualification_lock_active_records", 0)?;
    require_kv_u64(&observation, "running_libvirt_domains", 0)?;
    for field in [
        "active_github_actions_runner_listeners",
        "active_github_actions_runner_workers",
        "other_active_hepta_qualification_builds",
    ] {
        require_kv_u64(&observation, field, 0)?;
    }
    let hostname = observation
        .get("observed_hostname")
        .ok_or_else(|| invalid("Linux observed hostname is absent"))?;
    if hostname.is_empty()
        || hostname.len() > 253
        || !hostname
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        || !hostname.as_bytes()[0].is_ascii_alphanumeric()
    {
        return Err(invalid("Linux observed hostname is malformed"));
    }
    let machine_id = observation
        .get("machine_id_sha256")
        .ok_or_else(|| invalid("Linux machine-id digest is absent"))?;
    if !digest_shape(machine_id) {
        return Err(invalid("Linux machine-id digest is malformed"));
    }
    require_kv(inner, "qualification_observed_hostname", hostname)?;
    require_kv(inner, "qualification_machine_id_sha256", machine_id)?;
    if !observation
        .get("observed_at")
        .is_some_and(|value| valid_utc_timestamp(value))
    {
        return Err(invalid(
            "Linux host observation time is not an exact UTC second",
        ));
    }
    for field in [
        "nix_attempt_full_inventory_sha256",
        "nix_attempt_full_inventory_entry_count",
        "nix_attempt_full_inventory_file_count",
        "nix_attempt_full_inventory_directory_count",
        "nix_attempt_full_inventory_root",
        "nix_attempt_full_inventory_pre_post_equal",
    ] {
        require_kv(
            inner,
            field,
            observation
                .get(field)
                .ok_or_else(|| invalid("Linux host observation cross-binding is absent"))?,
        )?;
    }
    let host_lock_profile =
        parse_key_values(&outer_manifest.bytes("driver/host-lock-profile/PROFILE.txt")?)?;
    for (observation_field, profile_field) in [
        ("host_qualification_lock_device", "lock_device"),
        ("host_qualification_lock_inode", "lock_inode"),
    ] {
        require_kv(
            &observation,
            observation_field,
            host_lock_profile
                .get(profile_field)
                .ok_or_else(|| invalid("Linux host-lock profile binding is absent"))?,
        )?;
    }
    for (field, expected) in [
        (
            "acceptance_v7_contract_sha256",
            sha256(&outer_manifest.bytes("driver/acceptance-v7-linux-v5-contract.txt")?),
        ),
        (
            "operator_authority_sha256",
            sha256(&outer_manifest.bytes("driver/operator-authority/SHA256SUMS")?),
        ),
        (
            "runner_freeze_sha256",
            sha256(&outer_manifest.bytes("driver/runner-freeze/SHA256SUMS")?),
        ),
        (
            "host_workload_freeze_sha256",
            sha256(&outer_manifest.bytes("driver/workload-freeze/SHA256SUMS")?),
        ),
        (
            "host_lock_profile_sha256",
            sha256(&outer_manifest.bytes("driver/host-lock-profile/SHA256SUMS")?),
        ),
    ] {
        require_kv(&observation, field, &expected)?;
    }
    verify_linux_sanitized_environment(outer_manifest, &observation)
}

fn verify_linux_sanitized_environment(
    outer_manifest: &VerifiedManifest,
    observation: &BTreeMap<String, String>,
) -> Result<(), AcceptanceError> {
    let bytes = outer_manifest.bytes("driver/sanitized-environment-v5.txt")?;
    let environment = parse_key_values(&bytes)?;
    require_exact_kv_fields(
        &environment,
        &[
            "schema",
            "driver_revision",
            "environment_isolation",
            "remote_env_executable",
            "remote_bash_executable",
            "bash_startup_mode",
            "bootstrap_marker_name",
            "bootstrap_marker_value",
            "bootstrap_home",
            "bootstrap_lang",
            "bootstrap_lc_all",
            "bootstrap_path",
            "bootstrap_tz",
            "dynamic_exported_names",
            "normalized_bootstrap_names",
            "normalized_build_names",
            "bash_env_absent",
            "env_absent",
            "build_affecting_environment_allowlist_exact",
        ],
    )?;
    for (field, expected) in [
        ("schema", "hepta_vnext_linux_sanitized_environment_v5"),
        ("driver_revision", "5"),
        ("environment_isolation", "env-i"),
        ("remote_env_executable", "/usr/bin/env"),
        ("remote_bash_executable", "/usr/bin/bash"),
        ("bash_startup_mode", "--noprofile,--norc"),
        ("bootstrap_marker_name", "HEPTA_LINUX_SANITIZED_ENV_V5"),
        ("bootstrap_marker_value", "1"),
        ("bootstrap_home", "/nonexistent"),
        ("bootstrap_lang", "C"),
        ("bootstrap_lc_all", "C"),
        (
            "bootstrap_path",
            "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
        ),
        ("bootstrap_tz", "UTC"),
        ("dynamic_exported_names", "PWD,SHLVL,_"),
        (
            "normalized_bootstrap_names",
            "HEPTA_LINUX_SANITIZED_ENV_V5,HEPTA_HOST_LOCK_ACQUIRED_AT,HEPTA_HOST_LOCK_BOOT_ID,HEPTA_HOST_LOCK_CANDIDATE_HEAD,HEPTA_HOST_LOCK_CANDIDATE_TREE,HEPTA_HOST_LOCK_DEVICE,HEPTA_HOST_LOCK_FD,HEPTA_HOST_LOCK_HOLDER_PID,HEPTA_HOST_LOCK_HOLDER_STARTTICKS,HEPTA_HOST_LOCK_INODE,HEPTA_HOST_LOCK_MACHINE_ID_SHA256,HEPTA_HOST_LOCK_NONCE,HEPTA_HOST_LOCK_PATH,HEPTA_HOST_LOCK_PAYLOAD_SHA256,HEPTA_HOST_LOCK_PROTOCOL,HOME,LANG,LC_ALL,PATH,TZ",
        ),
        (
            "normalized_build_names",
            "BAZELISK_HOME,BAZEL_OUTPUT_USER_ROOT,BAZEL_REPOSITORY_CACHE,BAZEL_REPO_CONTENTS_CACHE,CARGO_BUILD_JOBS,CARGO_HOME,CARGO_INCREMENTAL,CARGO_NET_OFFLINE,CARGO_TARGET_DIR,CODEX_BAZEL_BIN,HEPTA_LINUX_SANITIZED_ENV_V5,HEPTA_HOST_LOCK_ACQUIRED_AT,HEPTA_HOST_LOCK_BOOT_ID,HEPTA_HOST_LOCK_CANDIDATE_HEAD,HEPTA_HOST_LOCK_CANDIDATE_TREE,HEPTA_HOST_LOCK_DEVICE,HEPTA_HOST_LOCK_FD,HEPTA_HOST_LOCK_HOLDER_PID,HEPTA_HOST_LOCK_HOLDER_STARTTICKS,HEPTA_HOST_LOCK_INODE,HEPTA_HOST_LOCK_MACHINE_ID_SHA256,HEPTA_HOST_LOCK_NONCE,HEPTA_HOST_LOCK_PATH,HEPTA_HOST_LOCK_PAYLOAD_SHA256,HEPTA_HOST_LOCK_PROTOCOL,HOME,LANG,LC_ALL,PATH,RUSTC,RUSTDOC,RUSTUP_HOME,RUSTUP_TOOLCHAIN,RUST_MIN_STACK,TEMP,TMP,TMPDIR,TZ,XDG_CACHE_HOME,XDG_CONFIG_HOME,XDG_DATA_HOME",
        ),
    ] {
        require_kv(&environment, field, expected)?;
    }
    for field in [
        "bash_env_absent",
        "env_absent",
        "build_affecting_environment_allowlist_exact",
    ] {
        require_kv_bool(&environment, field, true)?;
    }
    require_kv(
        observation,
        "sanitized_environment_binding_sha256",
        &sha256(&bytes),
    )?;
    Ok(())
}

fn verify_linux_runtime_environment(
    outer_manifest: &VerifiedManifest,
    inner_manifest: &VerifiedManifest,
    inner: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
    nonce: &str,
) -> Result<(), AcceptanceError> {
    let driver = outer_manifest.bytes("driver/sanitized-environment-v5.txt")?;
    let copied = inner_manifest.bytes("sanitized-environment-v5.txt")?;
    if driver != copied {
        return Err(invalid(
            "Linux copied sanitized environment contract differs from sealed driver",
        ));
    }
    require_kv(
        inner,
        "sanitized_environment_binding_sha256",
        &sha256(&driver),
    )?;
    require_kv(
        outer,
        "sanitized_environment_binding_sha256",
        &sha256(&driver),
    )?;
    for field in [
        "remote_environment_sanitized",
        "build_affecting_environment_allowlist_exact",
        "bash_env_absent",
    ] {
        require_kv_bool(inner, field, true)?;
    }
    let bootstrap = inner_manifest.bytes("sanitized-bootstrap-environment.txt")?;
    let expected_bootstrap_prefix = b"HEPTA_LINUX_SANITIZED_ENV_V5=1\n";
    if !bootstrap.starts_with(expected_bootstrap_prefix)
        || !bootstrap.ends_with(b"HOME=/nonexistent\nLANG=C\nLC_ALL=C\nPATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin\nTZ=UTC\n")
    {
        return Err(invalid(
            "Linux normalized bootstrap environment differs from its v5 allowlist envelope",
        ));
    }
    require_kv(
        inner,
        "sanitized_bootstrap_environment_sha256",
        &sha256(&bootstrap),
    )?;
    require_kv(
        outer,
        "sanitized_bootstrap_environment_sha256",
        &sha256(&bootstrap),
    )?;
    let build = inner_manifest.bytes("sanitized-build-environment.txt")?;
    require_kv(inner, "sanitized_build_environment_sha256", &sha256(&build))?;
    require_kv(outer, "sanitized_build_environment_sha256", &sha256(&build))?;
    let build = parse_normalized_environment(&build)?;
    let attempt = format!("/data/hepta-linux-exact-52ec-v5-{nonce}");
    let run = format!("{attempt}/run");
    let input = format!("{attempt}/input");
    let toolchain = format!("{input}/tools/1.95.0-x86_64-unknown-linux-gnu");
    let expected = [
        ("BAZELISK_HOME", format!("{run}/bazelisk-home-unused")),
        (
            "BAZEL_OUTPUT_USER_ROOT",
            format!("{run}/bazel-output-user-root"),
        ),
        ("BAZEL_REPOSITORY_CACHE", format!("{run}/bazel-repo-cache")),
        (
            "BAZEL_REPO_CONTENTS_CACHE",
            format!("{run}/bazel-repo-contents-cache"),
        ),
        ("CARGO_BUILD_JOBS", "1".to_string()),
        ("CARGO_HOME", format!("{run}/cargo-home")),
        ("CARGO_INCREMENTAL", "0".to_string()),
        ("CARGO_NET_OFFLINE", "true".to_string()),
        ("CARGO_TARGET_DIR", format!("{run}/cargo-target")),
        (
            "CODEX_BAZEL_BIN",
            format!("{input}/tools/bazel-9.0.0-linux-x86_64"),
        ),
        ("HEPTA_LINUX_SANITIZED_ENV_V5", "1".to_string()),
        ("HOME", format!("{run}/home")),
        ("LANG", "C".to_string()),
        ("LC_ALL", "C".to_string()),
        (
            "PATH",
            format!("{toolchain}/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"),
        ),
        ("RUSTC", format!("{toolchain}/bin/rustc")),
        ("RUSTDOC", format!("{toolchain}/bin/rustdoc")),
        ("RUSTUP_HOME", format!("{run}/rustup-home-unused")),
        (
            "RUSTUP_TOOLCHAIN",
            "1.95.0-x86_64-unknown-linux-gnu".to_string(),
        ),
        ("RUST_MIN_STACK", "8388608".to_string()),
        ("TEMP", format!("{run}/tmp")),
        ("TMP", format!("{run}/tmp")),
        ("TMPDIR", format!("{run}/tmp")),
        ("TZ", "UTC".to_string()),
        ("XDG_CACHE_HOME", format!("{run}/xdg-cache")),
        ("XDG_CONFIG_HOME", format!("{run}/xdg-config")),
        ("XDG_DATA_HOME", format!("{run}/xdg-data")),
    ]
    .into_iter()
    .map(|(name, value)| (name.to_string(), value))
    .collect::<BTreeMap<_, _>>();
    if expected
        .iter()
        .any(|(name, value)| build.get(name) != Some(value))
    {
        return Err(invalid(
            "Linux normalized build environment differs from its fixed v5 values",
        ));
    }
    let environment_contract = parse_key_values(&driver)?;
    let allowed_names = environment_contract
        .get("normalized_build_names")
        .ok_or_else(|| invalid("Linux v5 normalized build-name contract is absent"))?
        .split(',')
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if build.keys().cloned().collect::<BTreeSet<_>>() != allowed_names {
        return Err(invalid(
            "Linux normalized build environment name set differs from v5",
        ));
    }
    for (name, expected) in [
        ("HEPTA_HOST_LOCK_CANDIDATE_HEAD", CANDIDATE_HEAD),
        ("HEPTA_HOST_LOCK_CANDIDATE_TREE", CANDIDATE_TREE),
        ("HEPTA_HOST_LOCK_NONCE", nonce),
        (
            "HEPTA_HOST_LOCK_PATH",
            "/var/lib/hepta-vnext/locks/host-qualification.lock",
        ),
        ("HEPTA_HOST_LOCK_PROTOCOL", "detached_coordinator_v5"),
    ] {
        if build.get(name).map(String::as_str) != Some(expected) {
            return Err(invalid(format!(
                "Linux v5 host-lock environment binding differs: {name}"
            )));
        }
    }
    if !build
        .get("HEPTA_HOST_LOCK_ACQUIRED_AT")
        .is_some_and(|value| valid_utc_timestamp(value))
        || !build
            .get("HEPTA_HOST_LOCK_MACHINE_ID_SHA256")
            .is_some_and(|value| digest_shape(value))
        || !build
            .get("HEPTA_HOST_LOCK_PAYLOAD_SHA256")
            .is_some_and(|value| digest_shape(value))
    {
        return Err(invalid(
            "Linux v5 host-lock timestamp or digest environment binding is malformed",
        ));
    }
    for name in [
        "HEPTA_HOST_LOCK_DEVICE",
        "HEPTA_HOST_LOCK_FD",
        "HEPTA_HOST_LOCK_HOLDER_PID",
        "HEPTA_HOST_LOCK_HOLDER_STARTTICKS",
        "HEPTA_HOST_LOCK_INODE",
    ] {
        if build
            .get(name)
            .and_then(|value| value.parse::<u64>().ok())
            .is_none_or(|value| value == 0)
        {
            return Err(invalid(format!(
                "Linux v5 host-lock decimal environment binding is malformed: {name}"
            )));
        }
    }
    for field in [
        "remote_environment_sanitized",
        "build_affecting_environment_allowlist_exact",
        "bash_env_absent",
    ] {
        require_kv_bool(outer, field, true)?;
    }
    let environment = parse_key_values(&inner_manifest.bytes("environment.txt")?)?;
    for (field, expected) in [
        ("cargo_home", format!("{run}/cargo-home")),
        ("cargo_target", format!("{run}/cargo-target")),
        (
            "bazel_output_user_root",
            format!("{run}/bazel-output-user-root"),
        ),
    ] {
        require_kv(&environment, field, &expected)?;
    }
    Ok(())
}

fn parse_normalized_environment(bytes: &[u8]) -> Result<BTreeMap<String, String>, AcceptanceError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| invalid("Linux normalized environment is not UTF-8"))?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(invalid(
            "Linux normalized environment is empty or unterminated",
        ));
    }
    let mut result = BTreeMap::new();
    let mut previous = None;
    for line in text.lines() {
        let (name, value) = line
            .split_once('=')
            .ok_or_else(|| invalid("Linux normalized environment row lacks ="))?;
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
            || previous.is_some_and(|value: &str| value >= name)
            || result.insert(name.to_string(), value.to_string()).is_some()
        {
            return Err(invalid(
                "Linux normalized environment names are malformed, duplicate, or unordered",
            ));
        }
        previous = Some(name);
    }
    Ok(result)
}

fn verify_linux_driver_seal(
    manifest: &VerifiedManifest,
    inner: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
) -> Result<(), AcceptanceError> {
    let driver_manifest_path = "driver/DRIVER-SHA256SUMS";
    let bytes = manifest.bytes(driver_manifest_path)?;
    let parsed = parse_manifest(&bytes)?;
    let actual = manifest
        .entry_paths()
        .filter_map(|path| path.strip_prefix("driver/"))
        .filter(|path| *path != "DRIVER-SHA256SUMS")
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if parsed.keys().cloned().collect::<BTreeSet<_>>() != actual {
        return Err(invalid(
            "Linux DRIVER-SHA256SUMS does not cover the exact driver subtree",
        ));
    }
    for (path, digest) in &parsed {
        if manifest
            .entry(&format!("driver/{path}"))
            .map(|entry| entry.sha256.as_str())
            != Some(digest)
        {
            return Err(invalid("Linux driver payload differs from its nested seal"));
        }
    }
    for required in [
        "DRIVER-MODES.tsv",
        "NIX-PASS-BINDING.txt",
        "NIX-ATTEMPT-INVENTORY.tsv",
        "NIX-ATTEMPT-FULL-INVENTORY.tsv",
        "HOST-TOOLS.tsv",
        "HOST-EXECUTABLES.tsv",
        "tool-input-binding.txt",
        "sanitized-environment-v5.txt",
        "resource-watchdog-filter-v5.awk",
        "resource-watchdog-static-test-v5.sh",
        "verify-resource-watchdog-v5.py",
        "finalize-sealed-driver-v5.sh",
        "lib-v5.sh",
        ".capture-host-tools-remote-v5.sh",
        "capture-host-tools-v5.sh",
        "STATIC-TESTS.txt",
        "run-linux-exact-v5.sh",
        "launch-linux-exact-v5.sh",
        "verify-host-tools-v5.sh",
        "verify-and-seal-copied-receipt-v5.sh",
        "acceptance-v7-linux-v5-contract.txt",
        "EXECUTION-MODE.txt",
        "operator-trust-policy/trust-policy.json",
        "operator-trust-policy/allowed_signers",
        "operator-trust-policy/SHA256SUMS",
        "operator-trust-policy/MODES.tsv",
        "expected-step-names.txt",
        "expected-suite-names.txt",
        "expected-suite-counts.tsv",
        "host-tool-roster.txt",
        "host-observation/SHA256SUMS",
        "host-observation/INVENTORY.tsv",
        "host-observation/OBSERVATION.txt",
        "host-observation/HOST-TOOLS.tsv",
        "host-observation/HOST-EXECUTABLES.tsv",
        "host-observation/host-tool-roster.txt",
        "operator-authority/AUTHORITY.txt",
        "operator-authority/CHALLENGE.txt",
        "operator-authority/CHALLENGE.txt.sig",
        "operator-authority/ALLOWED-SIGNERS",
        "operator-authority/SHA256SUMS",
        "operator-authority/MODES.tsv",
        "runner-freeze/RUNNER-FREEZE.txt",
        "runner-freeze/RUNNER-INVENTORY.tsv",
        "runner-freeze/SERVICE-INVENTORY.tsv",
        "runner-freeze/RESTORE-PLAN.txt",
        "runner-freeze/SHA256SUMS",
        "runner-freeze/MODES.tsv",
        "workload-freeze/WORKLOAD-FREEZE.txt",
        "workload-freeze/WORKLOAD-INVENTORY.tsv",
        "workload-freeze/WORKLOAD-RESTORE-PLAN.tsv",
        "workload-freeze/NIX-PASS-BINDING.txt",
        "workload-freeze/SHA256SUMS",
        "workload-freeze/MODES.tsv",
        "host-lock-profile/PROFILE.txt",
        "host-lock-profile/NIX-BINDING.txt",
        "host-lock-profile/RUNNER-FREEZE-BINDING.txt",
        "host-lock-profile/WORKLOAD-FREEZE-BINDING.txt",
        "host-lock-profile/SHA256SUMS",
        "host-lock-profile/MODES.tsv",
        "host-lock-profile/INVENTORY.tsv",
    ] {
        if !parsed.contains_key(required) {
            return Err(invalid(format!(
                "Linux v5 driver seal omits required evidence: {required}"
            )));
        }
    }
    let manifest_digest = sha256(&bytes);
    require_kv(inner, "driver_manifest_sha256", &manifest_digest)?;
    require_kv(outer, "driver_manifest_sha256", &manifest_digest)?;
    require_kv_u64(inner, "driver_manifest_entry_count", parsed.len() as u64)?;
    require_kv_u64(outer, "driver_manifest_entry_count", parsed.len() as u64)?;

    let driver_modes = parse_mode_rows(
        &manifest.bytes("driver/DRIVER-MODES.tsv")?,
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    )?;
    let outer_modes = parse_mode_rows(
        &manifest.bytes("OUTER-MODES.tsv")?,
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    )?;
    let driver_files = parsed
        .keys()
        .cloned()
        .chain(std::iter::once("DRIVER-SHA256SUMS".to_string()))
        .collect::<BTreeSet<_>>();
    let actual_files = driver_modes
        .iter()
        .filter(|(_, row)| row.kind == ModeKind::File)
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    if actual_files != driver_files {
        return Err(invalid(
            "Linux DRIVER-MODES.tsv does not cover the exact driver files",
        ));
    }
    let expected_directories = outer_modes
        .iter()
        .filter(|(_, row)| row.kind == ModeKind::Directory)
        .filter_map(|(path, _)| {
            if path == "driver" {
                Some(".".to_string())
            } else {
                path.strip_prefix("driver/").map(str::to_string)
            }
        })
        .collect::<BTreeSet<_>>();
    let actual_directories = driver_modes
        .iter()
        .filter(|(_, row)| row.kind == ModeKind::Directory)
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    if actual_directories != expected_directories {
        return Err(invalid(
            "Linux DRIVER-MODES.tsv does not cover the exact driver directories",
        ));
    }
    for (path, row) in &driver_modes {
        let outer_path = if path == "." {
            "driver".to_string()
        } else {
            format!("driver/{path}")
        };
        if outer_modes.get(&outer_path) != Some(row) {
            return Err(invalid(
                "Linux driver metadata inventory differs from the outer inventory",
            ));
        }
    }
    Ok(())
}

fn verify_linux_nix_binding(
    outer_manifest: &VerifiedManifest,
    manifest: &VerifiedManifest,
    result: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    let bytes = manifest.bytes("NIX-PASS-BINDING.txt")?;
    let binding = parse_key_values(&bytes)?;
    require_exact_kv_fields(
        &binding,
        &[
            "schema",
            "driver_revision",
            "nix_nonce",
            "nix_attempt_root",
            "candidate_head",
            "candidate_tree",
            "candidate_parent",
            "upstream_cutoff",
            "nix_outer_sha256sums_sha256",
            "nix_outer_modes_sha256",
            "nix_result_sha256",
            "nix_inner_sha256sums_sha256",
            "nix_inner_modes_sha256",
            "nix_attempt_inventory_sha256",
            "nix_attempt_full_inventory_sha256",
            "nix_attempt_full_inventory_entry_count",
            "nix_attempt_full_inventory_file_count",
            "nix_attempt_full_inventory_directory_count",
            "nix_attempt_full_inventory_root",
            "nix_attempt_full_inventory_pre_post_equal",
            "nix_status",
            "nix_qualification",
            "nix_candidate_pass",
            "nix_candidate_fail",
            "nix_harness_fail",
            "nix_interrupted",
            "nix_recursive_hashes_verified",
            "nix_recursive_modes_verified",
            "host_tools_sha256",
            "host_executables_sha256",
            "host_observation_sha256sums_sha256",
            "sanitized_environment_binding_sha256",
            "tool_payload_manifest_sha256",
            "sealed_at",
        ],
    )?;
    require_kv_schema(&binding, "hepta_vnext_linux_nix_pass_binding_v5")?;
    require_kv_u64(&binding, "driver_revision", 5)?;
    require_kv(&binding, "nix_nonce", "52ec08130755")?;
    require_candidate_kv(&binding, candidate, "candidate_head", "upstream_cutoff")?;
    require_kv(&binding, "nix_status", "PASS")?;
    for field in [
        "nix_qualification",
        "nix_candidate_pass",
        "nix_recursive_hashes_verified",
        "nix_recursive_modes_verified",
    ] {
        require_kv_bool(&binding, field, true)?;
    }
    for field in ["nix_candidate_fail", "nix_harness_fail", "nix_interrupted"] {
        require_kv_bool(&binding, field, false)?;
    }
    for field in [
        "nix_outer_sha256sums_sha256",
        "nix_outer_modes_sha256",
        "nix_result_sha256",
        "nix_inner_sha256sums_sha256",
        "nix_inner_modes_sha256",
        "nix_attempt_inventory_sha256",
        "nix_attempt_full_inventory_sha256",
        "host_tools_sha256",
        "host_executables_sha256",
        "host_observation_sha256sums_sha256",
        "sanitized_environment_binding_sha256",
        "tool_payload_manifest_sha256",
    ] {
        if !binding.get(field).is_some_and(|value| digest_shape(value)) {
            return Err(invalid(format!(
                "Linux Nix binding digest is malformed: {field}"
            )));
        }
    }
    if !binding
        .get("sealed_at")
        .is_some_and(|value| valid_utc_timestamp(value))
    {
        return Err(invalid("Linux Nix binding seal time is malformed"));
    }
    let digest = sha256(&bytes);
    require_kv(result, "nix_pass_binding_sha256", &digest)?;
    require_kv(outer, "nix_pass_binding_sha256", &digest)?;
    require_kv(
        &binding,
        "sanitized_environment_binding_sha256",
        &sha256(&outer_manifest.bytes("driver/sanitized-environment-v5.txt")?),
    )?;
    let inventory_bytes = manifest.bytes("NIX-ATTEMPT-INVENTORY.tsv")?;
    let inventory_digest = sha256(&inventory_bytes);
    require_kv(&binding, "nix_attempt_inventory_sha256", &inventory_digest)?;
    require_kv(result, "nix_attempt_inventory_sha256", &inventory_digest)?;
    require_kv(outer, "nix_attempt_inventory_sha256", &inventory_digest)?;
    verify_linux_nix_full_inventory(outer_manifest, manifest, &binding, result, outer)?;
    for (binding_field, result_field) in [
        ("nix_outer_sha256sums_sha256", "nix_outer_sha256sums_sha256"),
        ("host_tools_sha256", "host_tools_binding_sha256"),
        ("host_executables_sha256", "host_executables_binding_sha256"),
        ("tool_payload_manifest_sha256", "tool_input_manifest_sha256"),
    ] {
        require_kv(
            result,
            result_field,
            binding
                .get(binding_field)
                .ok_or_else(|| invalid("Linux Nix cross-binding is absent"))?,
        )?;
    }
    for (binding_field, outer_field) in [
        ("nix_outer_sha256sums_sha256", "nix_outer_sha256sums_sha256"),
        ("nix_inner_sha256sums_sha256", "nix_inner_sha256sums_sha256"),
        ("host_tools_sha256", "host_tools_binding_sha256"),
        ("host_executables_sha256", "host_executables_binding_sha256"),
    ] {
        require_kv(
            outer,
            outer_field,
            binding
                .get(binding_field)
                .ok_or_else(|| invalid("Linux Nix outer cross-binding is absent"))?,
        )?;
    }
    verify_linux_nix_inventory(outer_manifest, manifest, &binding, &inventory_bytes)?;
    if sha256(&manifest.bytes("HOST-TOOLS.tsv")?) != binding["host_tools_sha256"]
        || sha256(&manifest.bytes("HOST-EXECUTABLES.tsv")?) != binding["host_executables_sha256"]
    {
        return Err(invalid(
            "Linux Nix binding differs from copied sealed evidence",
        ));
    }
    Ok(())
}

fn verify_linux_nix_full_inventory(
    outer_manifest: &VerifiedManifest,
    inner_manifest: &VerifiedManifest,
    binding: &BTreeMap<String, String>,
    result: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
) -> Result<(), AcceptanceError> {
    let frozen_nix = profiles::frozen_original_identity(EvidenceProfileV3::NixExactV3)
        .ok_or_else(|| invalid("PROFILE_IDENTITY_UNPINNED: frozen Nix attempt identity"))?;
    let nix_root = frozen_nix.receipt_root;
    let bytes = inner_manifest.bytes("NIX-ATTEMPT-FULL-INVENTORY.tsv")?;
    if outer_manifest.bytes("driver/NIX-ATTEMPT-FULL-INVENTORY.tsv")? != bytes {
        return Err(invalid(
            "Linux copied Nix full inventory differs between driver and receipt",
        ));
    }
    require_kv(binding, "nix_attempt_root", nix_root)?;
    require_kv(
        binding,
        "nix_outer_sha256sums_sha256",
        frozen_nix.manifest_sha256,
    )?;
    let live = load_legacy_manifest(
        Path::new(nix_root),
        frozen_nix.manifest_relative_path,
        frozen_nix.manifest_sha256,
        frozen_nix.entry_count,
    )?;
    if !live.hardlink_topology.is_empty() {
        return Err(invalid("frozen Nix attempt contains hardlinks"));
    }
    if bytes != live.metadata_inventory {
        return Err(invalid(
            "Linux Nix full inventory differs from the live frozen Nix attempt",
        ));
    }
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| invalid("Linux Nix full inventory is not UTF-8"))?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(invalid("Linux Nix full inventory is empty or unterminated"));
    }
    let mut previous = None;
    let mut files = 0_u64;
    let mut directories = 0_u64;
    for line in text.lines() {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 4
            || !matches!(fields[0], "Directory" | "Regular File")
            || !matches!(fields[1].len(), 3 | 4)
            || !fields[1].bytes().all(|byte| matches!(byte, b'0'..=b'7'))
            || (fields[0] == "Directory" && fields[2] != "-")
            || (fields[0] == "Regular File"
                && fields[2]
                    .parse::<u64>()
                    .ok()
                    .is_none_or(|size| size.to_string() != fields[2]))
        {
            return Err(invalid("Linux Nix full inventory row is malformed"));
        }
        let path = if fields[3] == "." {
            ""
        } else {
            fields[3]
                .strip_prefix("./")
                .ok_or_else(|| invalid("Linux Nix full inventory path lacks ./"))?
        };
        if !path.is_empty() {
            validate_relative_path(path)?;
        }
        if previous.is_some_and(|value: &str| value >= path) {
            return Err(invalid(
                "Linux Nix full inventory paths are not strictly ordinal",
            ));
        }
        previous = Some(path);
        files += u64::from(fields[0] == "Regular File");
        directories += u64::from(fields[0] == "Directory");
    }
    let digest = sha256(&bytes);
    let entries = files + directories;
    for values in [binding, result, outer] {
        require_kv(values, "nix_attempt_full_inventory_sha256", &digest)?;
        require_kv_u64(values, "nix_attempt_full_inventory_entry_count", entries)?;
        require_kv_u64(values, "nix_attempt_full_inventory_file_count", files)?;
        require_kv_u64(
            values,
            "nix_attempt_full_inventory_directory_count",
            directories,
        )?;
        require_kv(values, "nix_attempt_full_inventory_root", nix_root)?;
        require_kv_bool(values, "nix_attempt_full_inventory_pre_post_equal", true)?;
    }
    let after = live.reverify()?;
    if after.metadata_inventory != bytes || !after.hardlink_topology.is_empty() {
        return Err(invalid(
            "frozen Nix attempt changed across full inventory verification",
        ));
    }
    Ok(())
}

fn verify_linux_nix_inventory(
    outer_manifest: &VerifiedManifest,
    inner_manifest: &VerifiedManifest,
    binding: &BTreeMap<String, String>,
    bytes: &[u8],
) -> Result<(), AcceptanceError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| invalid("Linux Nix attempt inventory is not UTF-8"))?;
    if !text.ends_with('\n') || text.lines().count() != 8 {
        return Err(invalid(
            "Linux Nix attempt inventory must contain exactly eight LF rows",
        ));
    }
    let nix_root = binding
        .get("nix_attempt_root")
        .ok_or_else(|| invalid("Linux bound Nix attempt root is absent"))?;
    let expected_root = profiles::frozen_original_identity(EvidenceProfileV3::NixExactV3)
        .ok_or_else(|| invalid("PROFILE_IDENTITY_UNPINNED: frozen Nix attempt identity"))?
        .receipt_root;
    if nix_root != expected_root {
        return Err(invalid(
            "Linux Nix attempt root differs from the exact v3 attempt",
        ));
    }
    let expected = [
        (
            "host_executables".to_string(),
            "HOST-EXECUTABLES.tsv".to_string(),
        ),
        (
            "host_observation_sha256sums".to_string(),
            "host-observation/SHA256SUMS".to_string(),
        ),
        ("host_tools".to_string(), "HOST-TOOLS.tsv".to_string()),
        (
            "nix_inner_modes".to_string(),
            format!("{nix_root}/receipt/MODES.tsv"),
        ),
        (
            "nix_inner_sha256sums".to_string(),
            format!("{nix_root}/receipt/SHA256SUMS"),
        ),
        (
            "nix_outer_modes".to_string(),
            format!("{nix_root}/OUTER-MODES.tsv"),
        ),
        (
            "nix_outer_sha256sums".to_string(),
            format!("{nix_root}/OUTER-SHA256SUMS"),
        ),
        (
            "nix_result".to_string(),
            format!("{nix_root}/receipt/result.txt"),
        ),
    ];
    for (line, (expected_key, expected_path)) in text.lines().zip(expected) {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 3
            || fields[0] != expected_key
            || fields[1] != expected_path
            || !digest_shape(fields[2])
        {
            return Err(invalid(
                "Linux Nix attempt inventory row differs from its schema",
            ));
        }
        let actual = match fields[1] {
            "HOST-EXECUTABLES.tsv" => sha256(&inner_manifest.bytes("HOST-EXECUTABLES.tsv")?),
            "HOST-TOOLS.tsv" => sha256(&inner_manifest.bytes("HOST-TOOLS.tsv")?),
            "host-observation/SHA256SUMS" => {
                sha256(&outer_manifest.bytes("driver/host-observation/SHA256SUMS")?)
            }
            absolute => {
                let absolute = Path::new(absolute);
                secure_canonical_file_path(absolute, "bound Nix attempt artifact")?;
                sha256(&secure_read(absolute, MAX_SMALL_FILE_BYTES)?)
            }
        };
        if actual != fields[2] {
            return Err(invalid(
                "Linux Nix attempt inventory digest differs from the bound artifact",
            ));
        }
    }
    for (inventory_key, binding_field) in [
        ("nix_outer_sha256sums", "nix_outer_sha256sums_sha256"),
        ("nix_outer_modes", "nix_outer_modes_sha256"),
        ("nix_result", "nix_result_sha256"),
        ("nix_inner_sha256sums", "nix_inner_sha256sums_sha256"),
        ("nix_inner_modes", "nix_inner_modes_sha256"),
        ("host_tools", "host_tools_sha256"),
        ("host_executables", "host_executables_sha256"),
        (
            "host_observation_sha256sums",
            "host_observation_sha256sums_sha256",
        ),
    ] {
        let digest = text
            .lines()
            .find_map(|line| {
                let fields = line.split('\t').collect::<Vec<_>>();
                (fields.first() == Some(&inventory_key)).then_some(fields[2])
            })
            .ok_or_else(|| invalid("Linux Nix inventory key is absent"))?;
        require_kv(binding, binding_field, digest)?;
    }
    Ok(())
}

fn verify_linux_tool_binding(
    manifest: &VerifiedManifest,
    result: &BTreeMap<String, String>,
) -> Result<(), AcceptanceError> {
    let binding = parse_key_values(&manifest.bytes("driver/tool-input-binding.txt")?)?;
    require_exact_kv_fields(
        &binding,
        &[
            "schema",
            "driver_revision",
            "rust_release",
            "rust_host",
            "rust_archive_sha256",
            "rustc_sha256",
            "cargo_sha256",
            "clippy_driver_sha256",
            "rustfmt_sha256",
            "bazelisk_release",
            "bazelisk_linux_amd64_sha256",
            "bazel_release",
            "bazel_linux_x86_64_sha256",
            "shellcheck_release",
            "shellcheck_linux_x86_64_archive_sha256",
            "shellcheck_linux_x86_64_binary_sha256",
            "shellcheck_github_asset_id",
            "shellcheck_github_asset_size",
            "shellcheck_static_elf64_x86_64_no_interp_no_dynamic",
            "source",
            "inherited_build_results_allowed",
            "host_tool_snapshot_required",
            "host_tool_snapshot_preflight_and_postflight_match_required",
            "host_tool_install_or_mutation_allowed",
            "build_and_materialization_executable_digest_binding_required",
            "tool_payload_root",
            "tool_payload_manifest_sha256",
        ],
    )?;
    require_kv_schema(&binding, "hepta_vnext_linux_pinned_tool_inputs_v5")?;
    require_kv_u64(&binding, "driver_revision", 5)?;
    require_kv(&binding, "rust_release", "1.95.0")?;
    require_kv(&binding, "rust_host", "x86_64-unknown-linux-gnu")?;
    require_kv(&binding, "bazelisk_release", "1.28.1")?;
    require_kv(&binding, "bazel_release", "9.0.0")?;
    require_kv(&binding, "shellcheck_release", "0.11.0")?;
    require_kv_u64(&binding, "shellcheck_github_asset_id", 279056942)?;
    require_kv_u64(&binding, "shellcheck_github_asset_size", 2559196)?;
    require_kv(
        &binding,
        "source",
        "T5_reproducibly_materialized_official_Rust_payload_official_Bazel_release_objects_and_official_ShellCheck_GitHub_release_asset",
    )?;
    require_kv(
        &binding,
        "tool_payload_root",
        "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-linux-exact-v3-desktop-fallback-20260813T073309Z/tool-inputs",
    )?;
    for field in [
        "rustc_sha256",
        "cargo_sha256",
        "clippy_driver_sha256",
        "rustfmt_sha256",
    ] {
        if !binding.get(field).is_some_and(|value| digest_shape(value)) {
            return Err(invalid(format!(
                "Linux Rust tool digest is malformed: {field}"
            )));
        }
    }
    require_kv_bool(&binding, "inherited_build_results_allowed", false)?;
    for field in [
        "host_tool_snapshot_required",
        "host_tool_snapshot_preflight_and_postflight_match_required",
        "build_and_materialization_executable_digest_binding_required",
        "shellcheck_static_elf64_x86_64_no_interp_no_dynamic",
    ] {
        require_kv_bool(&binding, field, true)?;
    }
    require_kv_bool(&binding, "host_tool_install_or_mutation_allowed", false)?;
    for (binding_field, result_field) in [
        ("rust_archive_sha256", "rust_toolchain_archive_sha256"),
        ("bazelisk_linux_amd64_sha256", "bazelisk_sha256"),
        ("bazel_linux_x86_64_sha256", "bazel_sha256"),
        (
            "shellcheck_linux_x86_64_archive_sha256",
            "shellcheck_archive_sha256",
        ),
        ("shellcheck_linux_x86_64_binary_sha256", "shellcheck_sha256"),
        ("tool_payload_manifest_sha256", "tool_input_manifest_sha256"),
    ] {
        require_kv(
            result,
            result_field,
            binding
                .get(binding_field)
                .ok_or_else(|| invalid("Linux tool binding field is absent"))?,
        )?;
    }
    Ok(())
}

fn observe_nix(
    receipt: &VerifiedReceipt,
    candidate: &CandidateBindingV3,
) -> Result<ObservedGateV3, AcceptanceError> {
    let inner = kv_artifact(
        receipt,
        EvidenceProfileV3::NixExactV3,
        ManifestLayerIdV3::InnerReceipt,
    )?;
    require_exact_kv_fields(
        &inner,
        &[
            "schema",
            "status",
            "verdict",
            "qualification",
            "candidate_pass",
            "candidate_fail",
            "harness_fail",
            "interrupted",
            "failure_domain",
            "candidate_head",
            "candidate_tree",
            "candidate_parent",
            "upstream_cutoff",
            "input_manifest_sha256",
            "pinned_image",
            "container_exists",
            "container_exit_code",
            "container_oom_killed",
            "volume_exists",
            "raw_status",
            "candidate_execution_started",
            "candidate_execution_completed",
            "source_postflight_verified",
            "resource_monitor_verified",
            "pass_evidence_verified",
            "resource_binding_verified",
            "probe_verified",
            "container_cpuset",
            "container_nano_cpus",
            "finalizer_input_verification",
            "finalizer_driver_verification",
            "fresh_source",
            "fresh_named_nix_volume",
            "max_jobs",
            "cores",
            "single_cpuset",
            "remote_root",
            "remote_inputs_preserved",
            "remote_source_preserved",
            "remote_containers_preserved",
            "remote_volume_preserved",
            "production_changed",
            "refs_changed",
            "data_deleted",
            "promotion_authority",
            "classified_at",
        ],
    )?;
    require_kv_schema(&inner, "hepta_vnext_nix_exact_v3_result_v1")?;
    require_candidate_kv(&inner, candidate, "candidate_head", "upstream_cutoff")?;
    require_kv_bool(&inner, "production_changed", false)?;
    require_kv_bool(&inner, "refs_changed", false)?;
    let outer = outer_kv(receipt, EvidenceProfileV3::NixExactV3)?;
    require_exact_kv_fields(
        &outer,
        &[
            "schema",
            "status",
            "candidate_head",
            "candidate_tree",
            "candidate_parent",
            "upstream_cutoff",
            "nonce",
            "receipt_archive_sha256",
            "inner_recursive_hashes",
            "inner_recursive_modes",
            "inner_manifest_coverage",
            "candidate_binding",
            "remote_roots_preserved",
            "production_changed",
            "refs_changed",
            "promotion_authority",
            "verified_at",
        ],
    )?;
    require_kv_schema(&outer, "hepta_vnext_nix_exact_v3_local_verification_v1")?;
    require_candidate_kv(&outer, candidate, "candidate_head", "upstream_cutoff")?;
    require_kv_bool(&outer, "production_changed", false)?;
    require_kv_bool(&outer, "refs_changed", false)?;
    require_outer_recursive_verification(&outer, false)?;
    verify_nix_exact_bindings(&inner, &outer)?;
    observe_kv_execution(
        receipt,
        &inner,
        &outer,
        EvidenceProfileV3::NixExactV3,
        &profiles::NIX_STEPS,
        StepPolicy::PrefixFirstFailure,
    )
}

fn verify_nix_exact_bindings(
    inner: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
) -> Result<(), AcceptanceError> {
    require_kv(inner, "failure_domain", "none")?;
    require_kv(
        inner,
        "input_manifest_sha256",
        "65e9f7e70294c44b5f8e79881af6eebacdcb428452fd47bd8f5dcc54f4fd4bda",
    )?;
    require_kv(
        inner,
        "pinned_image",
        "nixos/nix@sha256:d78540374f6a886653cba47d5c3f61c5a41d42e2a8db2607b8d68cb226fd463e",
    )?;
    for field in [
        "container_exists",
        "volume_exists",
        "resource_monitor_verified",
        "pass_evidence_verified",
        "resource_binding_verified",
        "probe_verified",
        "finalizer_input_verification",
        "finalizer_driver_verification",
        "fresh_source",
        "fresh_named_nix_volume",
        "single_cpuset",
        "remote_inputs_preserved",
        "remote_source_preserved",
        "remote_containers_preserved",
        "remote_volume_preserved",
    ] {
        require_kv_bool(inner, field, true)?;
    }
    require_kv_bool(inner, "container_oom_killed", false)?;
    require_kv_u64(inner, "container_exit_code", 0)?;
    // `raw_status` is the remote qualification result emitted by the Nix
    // driver, not Docker's container lifecycle state.  Container termination
    // is independently bound by `container_exit_code=0` and
    // `container_oom_killed=false` above.
    require_kv(inner, "raw_status", "PASS")?;
    require_kv_u64(inner, "max_jobs", 1)?;
    require_kv_u64(inner, "cores", 1)?;
    let nonce = outer
        .get("nonce")
        .ok_or_else(|| invalid("Nix nonce is absent"))?;
    if nonce.len() < 8
        || nonce.len() > 16
        || !nonce
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(invalid("Nix nonce is malformed"));
    }
    require_kv(
        inner,
        "remote_root",
        &format!("/data/hepta-nix-exact-v3-52ec-{nonce}"),
    )?;
    for field in ["container_cpuset", "container_nano_cpus"] {
        let value = inner
            .get(field)
            .ok_or_else(|| invalid("Nix resource binding is absent"))?;
        if value.is_empty() || value == "missing" {
            return Err(invalid("Nix resource binding is not substantive"));
        }
    }
    if !inner
        .get("classified_at")
        .is_some_and(|value| valid_utc_timestamp(value))
        || !outer
            .get("verified_at")
            .is_some_and(|value| valid_utc_timestamp(value))
        || !outer
            .get("receipt_archive_sha256")
            .is_some_and(|value| digest_shape(value))
    {
        return Err(invalid("Nix terminal time or archive binding is malformed"));
    }
    Ok(())
}

#[cfg(test)]
pub(super) fn verify_nix_exact_bindings_for_test(
    inner: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
) -> Result<(), AcceptanceError> {
    verify_nix_exact_bindings(inner, outer)
}

fn github_prepared_profile(
    bytes: &[u8],
    candidate: &CandidateBindingV3,
) -> Result<Map<String, Value>, AcceptanceError> {
    let identity = profiles::frozen_github_prepared_profile_identity();
    if sha256(bytes) != identity.profile_sha256 {
        return Err(invalid(
            "GitHub hosted PROFILE.json differs from the compiled prepared profile",
        ));
    }
    let value = super::strict_json::parse(bytes)?;
    let object = value
        .as_object()
        .ok_or_else(|| invalid("GitHub hosted PROFILE.json must be an object"))?;
    require_exact_json_fields(
        object,
        &[
            "schema",
            "status",
            "profile_name",
            "profile_revision",
            "qualification_nonce",
            "repository",
            "repository_id",
            "repository_private",
            "default_branch",
            "remote_name",
            "candidate",
            "wrapper",
            "workflow",
            "trigger_ref",
            "trigger_branch",
            "authority_refs",
            "jobs",
            "action_pins",
            "candidate_anchors",
            "required_captures",
            "outer_inventory",
            "self_hosted_substitutes_for_hosted",
            "acceptance_revision_7_profile_compiled",
            "automatic_transition",
            "promotion_authority",
            "default_ref_authority",
            "production_authority",
            "outbound_authority",
            "enforce_authority",
            "retirement_authority",
            "prepared_at",
        ],
    )?;
    require_json_string(object, "schema", profiles::GITHUB_PROFILE_SCHEMA)?;
    require_json_string(object, "status", "PREPARED_NOT_EXECUTED")?;
    require_json_string(object, "profile_name", profiles::GITHUB_PROFILE_NAME)?;
    if json_u64(object, "profile_revision")? != 2
        || json_u64(object, "repository_id")? != profiles::GITHUB_REPOSITORY_ID
    {
        return Err(invalid("GitHub hosted profile numeric identity differs"));
    }
    require_json_string(
        object,
        "qualification_nonce",
        profiles::GITHUB_QUALIFICATION_NONCE,
    )?;
    require_json_string(object, "repository", profiles::GITHUB_REPOSITORY)?;
    require_json_bool(object, "repository_private", true)?;
    require_json_string(object, "default_branch", "main")?;
    require_json_string(object, "remote_name", "hepta-ci")?;
    require_json_string(object, "trigger_ref", profiles::GITHUB_TRIGGER_REF)?;
    require_json_string(object, "trigger_branch", profiles::GITHUB_TRIGGER_BRANCH)?;
    require_json_string(object, "outer_inventory", "typed_posix_mode_size_path_v2")?;
    for field in [
        "self_hosted_substitutes_for_hosted",
        "acceptance_revision_7_profile_compiled",
        "automatic_transition",
        "promotion_authority",
        "default_ref_authority",
        "production_authority",
        "outbound_authority",
        "enforce_authority",
        "retirement_authority",
    ] {
        require_json_bool(object, field, false)?;
    }

    let candidate_object = json_object_field(object, "candidate")?;
    require_exact_json_fields(
        candidate_object,
        &["head", "tree", "parent", "upstream_cutoff"],
    )?;
    require_json_string(candidate_object, "head", &candidate.head)?;
    require_json_string(candidate_object, "tree", &candidate.tree)?;
    require_json_string(candidate_object, "parent", &candidate.parents[0])?;
    require_json_string(
        candidate_object,
        "upstream_cutoff",
        &candidate.upstream_cutoff,
    )?;
    let wrapper = json_object_field(object, "wrapper")?;
    require_json_string(wrapper, "head", profiles::GITHUB_WRAPPER_HEAD)?;
    require_json_string(wrapper, "tree", profiles::GITHUB_WRAPPER_TREE)?;
    require_json_string(wrapper, "parent", &candidate.head)?;
    if json_u64(wrapper, "parent_count")? != 1 {
        return Err(invalid("GitHub wrapper parent count differs"));
    }
    let workflow = json_object_field(object, "workflow")?;
    require_json_string(workflow, "blob", profiles::GITHUB_WORKFLOW_BLOB)?;
    require_json_string(workflow, "sha256", profiles::GITHUB_WORKFLOW_SHA256)?;
    require_json_string(workflow, "event", "push")?;
    if json_u64(workflow, "run_attempt")? != 1 {
        return Err(invalid("GitHub workflow attempt profile differs"));
    }

    let jobs = object
        .get("jobs")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid("GitHub hosted jobs must be an array"))?;
    if jobs.len() != profiles::GITHUB_HOSTED_JOBS.len() {
        return Err(invalid("GitHub hosted job roster length differs"));
    }
    for (actual, expected) in jobs.iter().zip(profiles::GITHUB_HOSTED_JOBS) {
        let actual = actual
            .as_object()
            .ok_or_else(|| invalid("GitHub hosted job profile must be an object"))?;
        require_exact_json_fields(
            actual,
            &[
                "slug",
                "name",
                "requested_label",
                "runner_os",
                "kind",
                "artifact_inventory",
            ],
        )?;
        require_json_string(actual, "slug", expected.slug)?;
        require_json_string(actual, "name", expected.name)?;
        require_json_string(actual, "requested_label", expected.requested_label)?;
        require_json_string(actual, "runner_os", expected.runner_os)?;
        require_json_string(actual, "kind", expected.kind)?;
        require_json_string(
            actual,
            "artifact_inventory",
            if expected.artifact_inventory == "FILES.posix.tsv" {
                "posix"
            } else {
                "ntfs"
            },
        )?;
    }
    Ok(object.clone())
}

#[cfg(test)]
pub(super) fn validate_github_prepared_profile_for_test(
    bytes: &[u8],
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    github_prepared_profile(bytes, candidate).map(|_| ())
}

fn json_object_field<'a>(
    object: &'a Map<String, Value>,
    field: &str,
) -> Result<&'a Map<String, Value>, AcceptanceError> {
    object
        .get(field)
        .and_then(Value::as_object)
        .ok_or_else(|| invalid(format!("JSON object field is absent or malformed: {field}")))
}

fn positive_json_u64(object: &Map<String, Value>, field: &str) -> Result<u64, AcceptanceError> {
    let value = json_u64(object, field)?;
    if value == 0 {
        return Err(invalid(format!(
            "JSON numeric field is not positive: {field}"
        )));
    }
    Ok(value)
}

fn require_manifest_file(
    manifest: &VerifiedManifest,
    relative: &str,
    nonempty: bool,
) -> Result<(), AcceptanceError> {
    let entry = manifest
        .entry(relative)
        .ok_or_else(|| invalid(format!("GitHub hosted closure omits {relative}")))?;
    if nonempty && entry.size_bytes == 0 {
        return Err(invalid(format!(
            "GitHub hosted evidence is empty: {relative}"
        )));
    }
    Ok(())
}

fn parse_github_ref_observation(bytes: &[u8]) -> Result<BTreeMap<String, String>, AcceptanceError> {
    let text =
        std::str::from_utf8(bytes).map_err(|_| invalid("GitHub ref observation is not UTF-8"))?;
    if !text.is_empty() && (!text.ends_with('\n') || text.contains('\r')) {
        return Err(invalid("GitHub ref observation is not canonical LF text"));
    }
    let mut result = BTreeMap::new();
    let mut previous = None;
    for line in text.lines() {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 2
            || !digest_shape(fields[0])
            || !fields[1].starts_with("refs/")
            || fields[1].bytes().any(|byte| byte.is_ascii_whitespace())
            || previous.is_some_and(|value: &str| value >= fields[1])
            || result
                .insert(fields[1].to_string(), fields[0].to_string())
                .is_some()
        {
            return Err(invalid(
                "GitHub ref observation row is malformed or unordered",
            ));
        }
        previous = Some(fields[1]);
    }
    Ok(result)
}

fn github_payload_files(job: profiles::GithubHostedJobProfileV3) -> BTreeSet<String> {
    let mut files = [
        "binding.txt",
        "event.json",
        "workflow.yml",
        "job-api.json",
        "toolchain.txt",
        "tool-executables.tsv",
        "rustup-tool-paths.tsv",
        "rustup-tool-executables.tsv",
        "caller-ledger.log",
        "caller_ledger.started",
        "caller_ledger.rc",
        "caller_ledger.completed",
        "bazel-lock-check.log",
        "bazel_lock.started",
        "bazel_lock.rc",
        "bazel_lock.completed",
        "postflight.txt",
        "result.txt",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if job.runner_os == "Linux" {
        files.insert("linux-package-versions.tsv".to_string());
    } else {
        for file in [
            "windows-toolchain.txt",
            "windows-msvc-executables.tsv",
            "windows-where-cl.txt",
            "windows-where-link.txt",
        ] {
            files.insert(file.to_string());
        }
    }
    if job.kind == "product" {
        for (stem, log) in [
            ("locked_metadata", "locked-metadata.log"),
            ("hepta_tests", "hepta-tests.log"),
            ("product_callers", "product-callers.log"),
            ("strict_clippy", "strict-clippy.log"),
        ] {
            files.insert(log.to_string());
            for suffix in ["started", "rc", "completed"] {
                files.insert(format!("{stem}.{suffix}"));
            }
        }
    } else {
        for file in [
            "format.log",
            "format.started",
            "format.rc",
            "format.completed",
            "app-server-schemas.log",
            "app-server-schema-diff.log",
            "app-server-schemas.component-rc",
            "app_server_schemas.started",
            "app_server_schemas.rc",
            "app_server_schemas.completed",
        ] {
            files.insert(file.to_string());
        }
    }
    files
}

fn observe_github_hosted(
    receipt: &VerifiedReceipt,
    candidate: &CandidateBindingV3,
) -> Result<ObservedGateV3, AcceptanceError> {
    let manifest = receipt.layer(ManifestLayerIdV3::Outer)?;
    let profile_bytes = manifest.bytes("PROFILE.json")?;
    github_prepared_profile(&profile_bytes, candidate)?;
    if manifest.bytes("pre-observation/PROFILE.json")? != profile_bytes {
        return Err(invalid("GitHub pre-observation profile copy differs"));
    }
    let prepared = profiles::frozen_github_prepared_profile_identity();
    if sha256(&manifest.bytes("pre-observation/DRIVERS.sha256")?) != prepared.driver_manifest_sha256
        || sha256(&manifest.bytes("pre-observation/DRIVER-MODES.tsv")?)
            != prepared.driver_mode_sha256
    {
        return Err(invalid(
            "GitHub pre-observation driver identity differs from the compiled prepared root",
        ));
    }

    let result_value = super::strict_json::parse(&manifest.bytes("OUTER-RESULT.json")?)?;
    let result = result_value
        .as_object()
        .ok_or_else(|| invalid("GitHub outer result must be a JSON object"))?;
    require_exact_json_fields(
        result,
        &[
            "schema",
            "status",
            "qualification",
            "profile_revision",
            "profile_name",
            "qualification_nonce",
            "repository",
            "repository_id",
            "candidate_head",
            "candidate_tree",
            "candidate_parent",
            "upstream_cutoff",
            "wrapper_head",
            "wrapper_tree",
            "wrapper_parent",
            "workflow_blob",
            "workflow_sha256",
            "run_id",
            "run_attempt",
            "run_number",
            "run_status",
            "run_conclusion",
            "trigger_event",
            "trigger_ref",
            "trigger_ref_fresh_create_only",
            "required_job_count",
            "successful_job_count",
            "unique_numeric_job_count",
            "unique_numeric_hosted_runner_count",
            "artifact_count",
            "artifact_payloads_verified",
            "artifact_seals_verified",
            "candidate_execution_nonzero",
            "candidate_failure",
            "harness_failure",
            "self_hosted_substitutes_for_hosted",
            "observation_window_full_refs_exact",
            "authority_refs_changed",
            "production_observed",
            "production_changed_claim",
            "acceptance_revision_7_profile_compiled",
            "aggregate_promotion_authority",
            "automatic_transition",
            "artifacts",
        ],
    )?;
    require_json_string(result, "schema", profiles::GITHUB_OUTER_RESULT_SCHEMA)?;
    require_json_string(result, "status", "PASS")?;
    require_json_bool(result, "qualification", true)?;
    require_json_string(result, "profile_name", profiles::GITHUB_PROFILE_NAME)?;
    require_json_string(
        result,
        "qualification_nonce",
        profiles::GITHUB_QUALIFICATION_NONCE,
    )?;
    require_json_string(result, "repository", profiles::GITHUB_REPOSITORY)?;
    require_json_string(result, "candidate_head", &candidate.head)?;
    require_json_string(result, "candidate_tree", &candidate.tree)?;
    require_json_string(result, "candidate_parent", &candidate.parents[0])?;
    require_json_string(result, "upstream_cutoff", &candidate.upstream_cutoff)?;
    require_json_string(result, "wrapper_head", profiles::GITHUB_WRAPPER_HEAD)?;
    require_json_string(result, "wrapper_tree", profiles::GITHUB_WRAPPER_TREE)?;
    require_json_string(result, "wrapper_parent", &candidate.head)?;
    require_json_string(result, "workflow_blob", profiles::GITHUB_WORKFLOW_BLOB)?;
    require_json_string(result, "workflow_sha256", profiles::GITHUB_WORKFLOW_SHA256)?;
    require_json_string(result, "run_status", "completed")?;
    require_json_string(result, "run_conclusion", "success")?;
    require_json_string(result, "trigger_event", "push")?;
    require_json_string(result, "trigger_ref", profiles::GITHUB_TRIGGER_REF)?;
    require_json_string(result, "production_changed_claim", "not_asserted")?;
    if json_u64(result, "profile_revision")? != 2
        || json_u64(result, "repository_id")? != profiles::GITHUB_REPOSITORY_ID
        || json_u64(result, "run_attempt")? != 1
        || positive_json_u64(result, "run_id")? == 0
        || positive_json_u64(result, "run_number")? == 0
    {
        return Err(invalid("GitHub outer result numeric binding differs"));
    }
    for field in [
        "required_job_count",
        "successful_job_count",
        "unique_numeric_job_count",
        "unique_numeric_hosted_runner_count",
        "artifact_count",
    ] {
        if json_u64(result, field)? != 3 {
            return Err(invalid(format!(
                "GitHub outer result count differs: {field}"
            )));
        }
    }
    for field in [
        "trigger_ref_fresh_create_only",
        "artifact_payloads_verified",
        "artifact_seals_verified",
        "candidate_execution_nonzero",
        "observation_window_full_refs_exact",
    ] {
        require_json_bool(result, field, true)?;
    }
    for field in [
        "candidate_failure",
        "harness_failure",
        "self_hosted_substitutes_for_hosted",
        "authority_refs_changed",
        "production_observed",
        "acceptance_revision_7_profile_compiled",
        "aggregate_promotion_authority",
        "automatic_transition",
    ] {
        require_json_bool(result, field, false)?;
    }

    let run_id = positive_json_u64(result, "run_id")?;
    let artifacts = result
        .get("artifacts")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid("GitHub outer artifact result roster is absent"))?;
    if artifacts.len() != profiles::GITHUB_HOSTED_JOBS.len() {
        return Err(invalid(
            "GitHub outer artifact result roster length differs",
        ));
    }
    let mut expected_files = [
        "PROFILE.json",
        "capture-started-at.txt",
        "capture-completed-at.txt",
        "OUTER-RESULT.json",
        "OUTER-MODES.tsv",
        "pre-observation/PRE-OBSERVATION.json",
        "pre-observation/SHA256SUMS",
        "pre-observation/MODES.tsv",
        "pre-observation/PROFILE.json",
        "pre-observation/DRIVERS.sha256",
        "pre-observation/DRIVER-MODES.tsv",
        "pre-observation/repository.json",
        "pre-observation/actions-permissions.json",
        "pre-observation/refs.full.tsv",
        "pre-observation/wrapper.commit",
        "pre-observation/wrapper-tree.tsv",
        "pre-observation/workflow.yml",
        "pre-observation/wrapper-name-status.tsv",
        "pre-observation/observed-at.txt",
        "api/run.json",
        "api/jobs.json",
        "api/check-suite.json",
        "api/check-runs.json",
        "api/artifacts.json",
        "api/workflow.json",
        "api/repository.json",
        "api/actions-permissions.json",
        "api/wrapper-commit.json",
        "api/candidate-commit.json",
        "api/trigger-ref.json",
        "api/workflow.yml",
        "api/refs.full.post.tsv",
        "logs/run-logs.http",
        "logs/run-logs.zip",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let mut expected_directories = [
        "",
        "pre-observation",
        "api",
        "logs",
        "artifacts",
        "artifacts/downloads",
        "artifacts/extracted",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let mut job_ids = BTreeSet::new();
    let mut runner_ids = BTreeSet::new();
    let mut artifact_ids = BTreeSet::new();
    let mut observed_slugs = BTreeSet::new();
    for artifact in artifacts {
        let artifact = artifact
            .as_object()
            .ok_or_else(|| invalid("GitHub outer artifact result must be an object"))?;
        require_exact_json_fields(
            artifact,
            &[
                "slug",
                "artifact_id",
                "artifact_name",
                "artifact_zip_sha256",
                "artifact_size_bytes",
                "artifact_api_digest",
                "job_id",
                "runner_id",
                "runner_name",
                "runner_group_name",
                "requested_runner_label",
                "runner_image_os",
                "runner_image_version",
                "inner_inventory",
                "inner_inventory_sha256",
                "inner_payload_manifest_sha256",
                "inner_seal_sha256",
                "status",
                "qualification",
            ],
        )?;
        let slug = json_string(artifact, "slug")?;
        let job = profiles::GITHUB_HOSTED_JOBS
            .iter()
            .copied()
            .find(|job| job.slug == slug)
            .ok_or_else(|| invalid("GitHub outer artifact slug is not compiled"))?;
        if !observed_slugs.insert(slug.to_string()) {
            return Err(invalid("GitHub outer artifact slug is duplicated"));
        }
        let artifact_id = positive_json_u64(artifact, "artifact_id")?;
        let job_id = positive_json_u64(artifact, "job_id")?;
        let runner_id = positive_json_u64(artifact, "runner_id")?;
        if !artifact_ids.insert(artifact_id)
            || !job_ids.insert(job_id)
            || !runner_ids.insert(runner_id)
        {
            return Err(invalid(
                "GitHub numeric artifact/job/runner identity is reused",
            ));
        }
        let artifact_name = format!(
            "hepta-hosted-52ec-{}-{run_id}-attempt-1-{}",
            profiles::GITHUB_QUALIFICATION_NONCE,
            job.slug
        );
        require_json_string(artifact, "artifact_name", &artifact_name)?;
        require_json_string(artifact, "runner_group_name", "GitHub Actions")?;
        require_json_string(artifact, "requested_runner_label", job.requested_label)?;
        require_json_string(artifact, "inner_inventory", job.artifact_inventory)?;
        require_json_string(artifact, "status", "PASS")?;
        require_json_bool(artifact, "qualification", true)?;
        if positive_json_u64(artifact, "artifact_size_bytes")? == 0
            || json_string(artifact, "runner_name")?.is_empty()
            || json_string(artifact, "runner_image_os")?.is_empty()
            || json_string(artifact, "runner_image_version")?.is_empty()
        {
            return Err(invalid(
                "GitHub hosted runner or artifact evidence is empty",
            ));
        }
        for field in [
            "artifact_zip_sha256",
            "inner_inventory_sha256",
            "inner_payload_manifest_sha256",
            "inner_seal_sha256",
        ] {
            if !digest_shape(json_string(artifact, field)?) {
                return Err(invalid(format!(
                    "GitHub artifact digest is malformed: {field}"
                )));
            }
        }
        if let Some(digest) = artifact.get("artifact_api_digest") {
            if !digest.is_null()
                && digest
                    .as_str()
                    .and_then(|value| value.strip_prefix("sha256:"))
                    .is_none_or(|value| !digest_shape(value))
            {
                return Err(invalid("GitHub artifact API digest is malformed"));
            }
        }
        for path in [
            format!("api/job-{job_id}.json"),
            format!("api/annotations-{job_id}.json"),
            format!("logs/job-{job_id}.log"),
            format!("artifacts/downloads/{artifact_name}.http"),
            format!("artifacts/downloads/{artifact_name}.zip"),
        ] {
            require_manifest_file(manifest, &path, true)?;
            expected_files.insert(path);
        }
        let extracted = format!("artifacts/extracted/{}", job.slug);
        for directory in [
            extracted.clone(),
            format!("{extracted}/payload"),
            format!("{extracted}/seal"),
        ] {
            expected_directories.insert(directory);
        }
        for payload in github_payload_files(job) {
            expected_files.insert(format!("{extracted}/payload/{payload}"));
        }
        for seal in [job.artifact_inventory, "PAYLOAD.sha256", "SEAL.sha256"] {
            expected_files.insert(format!("{extracted}/seal/{seal}"));
        }
        verify_github_inner_result(
            manifest, &extracted, job, job_id, runner_id, run_id, candidate,
        )?;
    }
    if observed_slugs
        != profiles::GITHUB_HOSTED_JOBS
            .iter()
            .map(|job| job.slug.to_string())
            .collect()
    {
        return Err(invalid(
            "GitHub hosted artifact roster differs from the compiled roster",
        ));
    }
    if manifest
        .entry_paths()
        .map(str::to_string)
        .collect::<BTreeSet<_>>()
        != expected_files
        || manifest
            .directory_paths()
            .map(str::to_string)
            .collect::<BTreeSet<_>>()
            != expected_directories
    {
        return Err(invalid(
            "GitHub hosted outer file/directory closure differs from the compiled profile",
        ));
    }
    for path in &expected_files {
        require_manifest_file(manifest, path, true)?;
    }
    if sha256(&manifest.bytes("api/workflow.yml")?) != profiles::GITHUB_WORKFLOW_SHA256
        || sha256(&manifest.bytes("pre-observation/workflow.yml")?)
            != profiles::GITHUB_WORKFLOW_SHA256
    {
        return Err(invalid(
            "GitHub workflow bytes differ from the compiled profile",
        ));
    }
    verify_github_api_and_refs(manifest, result, run_id, &job_ids, &artifact_ids)?;
    Ok(ObservedGateV3 {
        candidate_executed: true,
        candidate_failure: false,
        executed_steps: 16,
        harness_failure: false,
        pass: true,
        production_changed: None,
        qualification: true,
        refs_changed: Some(false),
        status: "PASS".to_string(),
    })
}

fn verify_github_inner_result(
    manifest: &VerifiedManifest,
    extracted: &str,
    job: profiles::GithubHostedJobProfileV3,
    job_id: u64,
    runner_id: u64,
    run_id: u64,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    let payload = |name: &str| manifest.bytes(&format!("{extracted}/payload/{name}"));
    let result = parse_key_values(&payload("result.txt")?)?;
    require_kv_schema(&result, "hepta_vnext_github_hosted_job_result_v1")?;
    require_kv(&result, "status", "PASS")?;
    require_kv_bool(&result, "qualification", true)?;
    require_kv_bool(&result, "candidate_execution_nonzero", true)?;
    require_kv_bool(&result, "candidate_failure", false)?;
    require_kv_bool(&result, "harness_failure", false)?;
    require_kv(&result, "caller_ledger_scope", "manifest_schema_only")?;
    require_kv_u64(
        &result,
        "candidate_required_step_count",
        if job.kind == "product" { 6 } else { 4 },
    )?;
    let binding = parse_key_values(&payload("binding.txt")?)?;
    require_kv_schema(&binding, "hepta_vnext_github_hosted_job_binding_v1")?;
    for (field, expected) in [
        ("status", "WRAPPER_AND_RUNNER_BOUND"),
        ("qualification_nonce", profiles::GITHUB_QUALIFICATION_NONCE),
        ("repository", profiles::GITHUB_REPOSITORY),
        ("event_name", "push"),
        ("ref", profiles::GITHUB_TRIGGER_REF),
        ("ref_name", profiles::GITHUB_TRIGGER_BRANCH),
        ("wrapper_head", profiles::GITHUB_WRAPPER_HEAD),
        ("wrapper_tree", profiles::GITHUB_WRAPPER_TREE),
        ("wrapper_parent", candidate.head.as_str()),
        ("workflow_blob", profiles::GITHUB_WORKFLOW_BLOB),
        ("workflow_sha256", profiles::GITHUB_WORKFLOW_SHA256),
        ("runner_environment", "github-hosted"),
        ("runner_os", job.runner_os),
        ("runner_arch", "X64"),
        ("requested_runner_label", job.requested_label),
        ("candidate_head", candidate.head.as_str()),
        ("candidate_tree", candidate.tree.as_str()),
        ("candidate_parent", candidate.parents[0].as_str()),
        ("upstream_cutoff", candidate.upstream_cutoff.as_str()),
    ] {
        require_kv(&binding, field, expected)?;
    }
    require_kv_u64(&binding, "repository_id", profiles::GITHUB_REPOSITORY_ID)?;
    require_kv_u64(&binding, "run_id", run_id)?;
    require_kv_u64(&binding, "run_attempt", 1)?;
    require_kv(&binding, "job_name", job.name)?;
    require_kv(&binding, "job_slug", job.slug)?;
    if binding.get("runner_name").is_none_or(String::is_empty) {
        return Err(invalid("GitHub hosted inner runner name is absent"));
    }
    let job_api = super::strict_json::parse(&payload("job-api.json")?)?;
    let job_api = job_api
        .as_object()
        .ok_or_else(|| invalid("GitHub inner job API binding must be an object"))?;
    if positive_json_u64(job_api, "id")? != job_id
        || positive_json_u64(job_api, "runner_id")? != runner_id
        || json_u64(job_api, "run_attempt")? != 1
    {
        return Err(invalid("GitHub inner numeric job/runner binding differs"));
    }
    require_json_string(job_api, "name", job.name)?;
    require_json_string(job_api, "head_sha", profiles::GITHUB_WRAPPER_HEAD)?;
    require_json_string(job_api, "head_branch", profiles::GITHUB_TRIGGER_BRANCH)?;
    let postflight = parse_key_values(&payload("postflight.txt")?)?;
    require_kv_schema(
        &postflight,
        "hepta_vnext_github_hosted_candidate_postflight_v1",
    )?;
    require_kv(&postflight, "status", "PASS")?;
    require_kv(&postflight, "candidate_head", &candidate.head)?;
    require_kv(&postflight, "candidate_tree", &candidate.tree)?;
    require_kv(&postflight, "candidate_parent", &candidate.parents[0])?;
    require_kv_bool(&postflight, "candidate_worktree_clean_post", true)?;
    let event = super::strict_json::parse(&payload("event.json")?)?;
    let event = event
        .as_object()
        .ok_or_else(|| invalid("GitHub inner event must be an object"))?;
    require_json_string(event, "ref", profiles::GITHUB_TRIGGER_REF)?;
    require_json_string(event, "after", profiles::GITHUB_WRAPPER_HEAD)?;
    require_json_string(event, "before", "0000000000000000000000000000000000000000")?;
    require_json_bool(event, "created", true)?;
    require_json_bool(event, "deleted", false)?;
    require_json_bool(event, "forced", false)?;
    Ok(())
}

fn verify_github_api_and_refs(
    manifest: &VerifiedManifest,
    result: &Map<String, Value>,
    run_id: u64,
    job_ids: &BTreeSet<u64>,
    artifact_ids: &BTreeSet<u64>,
) -> Result<(), AcceptanceError> {
    let api = |name: &str| -> Result<Value, AcceptanceError> {
        super::strict_json::parse(&manifest.bytes(&format!("api/{name}"))?)
    };
    let run_value = api("run.json")?;
    let run = run_value
        .as_object()
        .ok_or_else(|| invalid("GitHub run API must be an object"))?;
    if positive_json_u64(run, "id")? != run_id
        || json_u64(run, "run_attempt")? != 1
        || positive_json_u64(run, "run_number")? != json_u64(result, "run_number")?
    {
        return Err(invalid("GitHub run API numeric binding differs"));
    }
    for (field, expected) in [
        ("status", "completed"),
        ("conclusion", "success"),
        ("event", "push"),
        ("head_sha", profiles::GITHUB_WRAPPER_HEAD),
        ("head_branch", profiles::GITHUB_TRIGGER_BRANCH),
    ] {
        require_json_string(run, field, expected)?;
    }
    let jobs_value = api("jobs.json")?;
    let jobs = jobs_value
        .as_object()
        .ok_or_else(|| invalid("GitHub jobs API must be an object"))?;
    if json_u64(jobs, "total_count")? != 3 {
        return Err(invalid("GitHub jobs API count differs"));
    }
    let observed_job_ids = jobs
        .get("jobs")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid("GitHub jobs API roster is absent"))?
        .iter()
        .map(|job| {
            let job = job
                .as_object()
                .ok_or_else(|| invalid("GitHub jobs API row must be an object"))?;
            require_json_string(job, "status", "completed")?;
            require_json_string(job, "conclusion", "success")?;
            require_json_string(job, "head_sha", profiles::GITHUB_WRAPPER_HEAD)?;
            require_json_string(job, "head_branch", profiles::GITHUB_TRIGGER_BRANCH)?;
            if json_u64(job, "run_attempt")? != 1 {
                return Err(invalid("GitHub job API attempt differs"));
            }
            positive_json_u64(job, "id")
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if &observed_job_ids != job_ids {
        return Err(invalid("GitHub jobs API IDs differ from the outer result"));
    }
    let artifacts_value = api("artifacts.json")?;
    let artifacts = artifacts_value
        .as_object()
        .ok_or_else(|| invalid("GitHub artifacts API must be an object"))?;
    if json_u64(artifacts, "total_count")? != 3 {
        return Err(invalid("GitHub artifact API count differs"));
    }
    let observed_artifact_ids = artifacts
        .get("artifacts")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid("GitHub artifact API roster is absent"))?
        .iter()
        .map(|artifact| {
            artifact
                .as_object()
                .ok_or_else(|| invalid("GitHub artifact API row must be an object"))
                .and_then(|artifact| positive_json_u64(artifact, "id"))
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if &observed_artifact_ids != artifact_ids {
        return Err(invalid(
            "GitHub artifact API IDs differ from the outer result",
        ));
    }
    let pre = parse_github_ref_observation(&manifest.bytes("pre-observation/refs.full.tsv")?)?;
    if pre.contains_key(profiles::GITHUB_TRIGGER_REF) {
        return Err(invalid("GitHub trigger ref existed before qualification"));
    }
    let mut expected_post = pre;
    expected_post.insert(
        profiles::GITHUB_TRIGGER_REF.to_string(),
        profiles::GITHUB_WRAPPER_HEAD.to_string(),
    );
    let post = parse_github_ref_observation(&manifest.bytes("api/refs.full.post.tsv")?)?;
    if post != expected_post {
        return Err(invalid(
            "GitHub full ref observation changed beyond exact trigger creation",
        ));
    }
    Ok(())
}

fn observe_windows(
    receipt: &VerifiedReceipt,
    candidate: &CandidateBindingV3,
) -> Result<ObservedGateV3, AcceptanceError> {
    let (layer, path, schema) =
        profiles::authoritative_artifact(EvidenceProfileV3::WindowsNativeV6)
            .ok_or_else(|| invalid("Windows authority profile is absent"))?;
    let value = super::strict_json::parse(&receipt.layer(layer)?.bytes(path)?)?;
    let object = value
        .as_object()
        .ok_or_else(|| invalid("Windows result must be a JSON object"))?;
    require_exact_json_fields(
        object,
        &[
            "schema",
            "driver_revision",
            "status",
            "verdict",
            "qualification",
            "candidate_pass",
            "candidate_fail",
            "harness_fail",
            "harness_preflight_pass",
            "candidate_execution_started",
            "candidate_execution_completed",
            "postflight_verified",
            "candidate_head",
            "candidate_tree",
            "candidate_parent",
            "upstream_cutoff",
            "source_identity",
            "worktree_clean",
            "windows_native",
            "windows_guest_hostname",
            "windows_vm_domain_guest_asserted",
            "windows_vm_domain_observation_source",
            "x230_vm_domain_evidence_required",
            "windows_toolchain",
            "rust_toolchain",
            "cargo_net_offline",
            "files_inventory_profile",
            "files_inventory_full_type_size_path_closure",
            "files_inventory_reparse_and_special_rejected",
            "files_inventory_hardlink_rejected_when_provider_reports_linktype",
            "seeded_build_results",
            "build_jobs",
            "fresh_attempt_root",
            "github_actions_runner_active",
            "github_actions_runner_active_state_verified",
            "github_actions_runner_service_configuration_present",
            "github_actions_runner_registration_markers_present_in_constrained_roots",
            "github_actions_runner_registration_state_scope",
            "github_actions_runner_registration_state_verified",
            "production_scope",
            "refs_changed",
            "production_changed",
            "mac_production_boundary_observed",
            "bazel",
            "product_binary",
            "ordered_step_count",
            "portable_hepta_tests_passed_count",
            "step_results",
            "failure_message",
            "run_root",
            "source_root",
            "vendor_root",
            "target_root",
            "completed_at_utc",
            "free_bytes_after",
        ],
    )?;
    require_json_string(object, "schema", schema)?;
    if json_u64(object, "driver_revision")? != 6 {
        return Err(invalid("Windows driver revision differs from v6"));
    }
    require_json_candidate(object, candidate)?;
    require_json_bool(object, "production_changed", false)?;
    require_json_bool(object, "refs_changed", false)?;
    let outer = outer_kv(receipt, EvidenceProfileV3::WindowsNativeV6)?;
    require_exact_kv_fields(
        &outer,
        &[
            "schema",
            "driver_revision",
            "status",
            "candidate_head",
            "candidate_tree",
            "candidate_parent",
            "upstream_cutoff",
            "guest_receipt_status",
            "guest_receipt_sha256sums",
            "guest_receipt_exact_file_set",
            "guest_files_inventory_profile",
            "guest_files_inventory_full_closure",
            "guest_driver_payload_manifest",
            "toolchain_exactness",
            "git_capture_self_test",
            "environment_sanitization",
            "native_capture_self_test",
            "guest_result_classification",
            "candidate_execution_marker_consistent",
            "x230_vm_domain_uuid_state_evidence",
            "x230_original_domain_state_autostart_evidence",
            "x230_recovery_interface_mac_ipv4_evidence",
            "incoming_lstat_type_scan",
            "copied_regular_file_set_exact_coverage",
            "outer_modes_profile",
            "mac_jq_fixture_sha256",
            "mac_process_path_fixed_system_roots",
            "mac_verifier_userland_full_closure_pinned",
            "mac_verifier_trusted_os_boundary",
            "outer_attempt_one_shot",
            "github_actions_runner_active_state_verified",
            "github_actions_runner_registration_state_verified",
            "refs_changed",
            "production_changed",
        ],
    )?;
    require_kv_schema(&outer, "hepta_vnext_windows_native_outer_verification_v3")?;
    require_kv_u64(&outer, "driver_revision", 6)?;
    require_kv(&outer, "status", "PASS")?;
    require_candidate_kv(&outer, candidate, "candidate_head", "upstream_cutoff")?;
    require_kv_bool(&outer, "production_changed", false)?;
    require_kv_bool(&outer, "refs_changed", false)?;
    for field in [
        "guest_receipt_sha256sums",
        "guest_receipt_exact_file_set",
        "guest_driver_payload_manifest",
        "guest_result_classification",
        "candidate_execution_marker_consistent",
        "x230_vm_domain_uuid_state_evidence",
        "x230_original_domain_state_autostart_evidence",
        "x230_recovery_interface_mac_ipv4_evidence",
        "incoming_lstat_type_scan",
        "copied_regular_file_set_exact_coverage",
    ] {
        require_kv(&outer, field, "PASS")?;
    }
    require_kv_bool(&outer, "outer_attempt_one_shot", true)?;
    require_kv(
        &outer,
        "guest_files_inventory_profile",
        "WindowsNtfsTypeSizePathTsvV1",
    )?;
    require_kv(&outer, "guest_files_inventory_full_closure", "PASS")?;
    require_kv(&outer, "outer_modes_profile", "TypedPosixModeSizePathTsvV2")?;
    for field in [
        "toolchain_exactness",
        "git_capture_self_test",
        "environment_sanitization",
        "native_capture_self_test",
    ] {
        require_kv_bool(&outer, field, true)?;
    }
    require_kv_bool(&outer, "mac_process_path_fixed_system_roots", true)?;
    require_kv_bool(&outer, "mac_verifier_userland_full_closure_pinned", false)?;
    require_kv_bool(&outer, "mac_verifier_trusted_os_boundary", true)?;
    if !outer
        .get("mac_jq_fixture_sha256")
        .is_some_and(|value| digest_shape(value))
    {
        return Err(invalid("Windows macOS jq fixture digest is malformed"));
    }

    let status = json_string(object, "status")?.to_string();
    if json_string(object, "verdict")? != status
        || outer.get("guest_receipt_status").map(String::as_str) != Some(status.as_str())
    {
        return Err(invalid(
            "Windows outer status does not relay the inner verdict",
        ));
    }
    let expected_steps = json_u64(object, "ordered_step_count")?;
    if expected_steps != 5 {
        return Err(invalid(
            "Windows ordered step count differs from its profile",
        ));
    }
    let steps = step_tsv(
        receipt.layer(ManifestLayerIdV3::InnerReceipt)?,
        &status,
        &profiles::WINDOWS_STEPS,
        StepPolicy::WindowsFullCandidateRun,
    )?;
    let candidate_execution_started = json_bool(object, "candidate_execution_started")?;
    if steps.count() > 0 && !candidate_execution_started {
        return Err(invalid(
            "Windows candidate execution marker contradicts the sealed step roster",
        ));
    }
    require_windows_step_results(object.get("step_results"), &steps, &status)?;
    validate_windows_execution_fields(object, &outer, &status)?;
    verify_windows_v6_bindings(receipt, object, &outer, candidate, &status)?;
    let observed = ObservedGateV3 {
        candidate_executed: candidate_execution_started,
        candidate_failure: json_bool(object, "candidate_fail")?,
        executed_steps: steps.count(),
        harness_failure: json_bool(object, "harness_fail")?,
        pass: json_bool(object, "candidate_pass")?,
        production_changed: Some(false),
        qualification: json_bool(object, "qualification")?,
        refs_changed: Some(false),
        status,
    };
    validate_gate_shape(&observed, expected_steps)?;
    Ok(observed)
}

fn observe_kv_execution(
    receipt: &VerifiedReceipt,
    inner: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
    profile: EvidenceProfileV3,
    expected_steps: &[&str],
    step_policy: StepPolicy,
) -> Result<ObservedGateV3, AcceptanceError> {
    let status = require_same_status(inner, outer)?;
    let steps = step_tsv(
        receipt.layer(ManifestLayerIdV3::InnerReceipt)?,
        &status,
        expected_steps,
        step_policy,
    )?;
    validate_kv_execution_fields(profile, inner, &status)?;
    let observed = ObservedGateV3 {
        candidate_executed: kv_bool(inner, "candidate_execution_started")?,
        candidate_failure: kv_bool(inner, "candidate_fail")?,
        executed_steps: steps.count(),
        harness_failure: kv_bool(inner, "harness_fail")?,
        pass: kv_bool(inner, "candidate_pass")?,
        production_changed: (profile != EvidenceProfileV3::LinuxExactV5).then_some(false),
        qualification: kv_bool(inner, "qualification")?,
        refs_changed: (profile != EvidenceProfileV3::LinuxExactV5).then_some(false),
        status,
    };
    validate_gate_shape(&observed, expected_steps.len() as u64)?;
    Ok(observed)
}

fn verify_windows_v6_bindings(
    receipt: &VerifiedReceipt,
    inner: &Map<String, Value>,
    _outer: &BTreeMap<String, String>,
    candidate: &CandidateBindingV3,
    status: &str,
) -> Result<(), AcceptanceError> {
    require_json_bool(inner, "windows_native", true)?;
    require_json_bool(inner, "windows_vm_domain_guest_asserted", false)?;
    require_json_string(
        inner,
        "windows_vm_domain_observation_source",
        "x230_relay_outer_verification_required",
    )?;
    require_json_bool(inner, "x230_vm_domain_evidence_required", true)?;
    require_json_string(inner, "windows_toolchain", "x86_64-pc-windows-msvc")?;
    require_json_string(inner, "rust_toolchain", "1.95.0")?;
    require_json_bool(inner, "cargo_net_offline", true)?;
    require_json_string(
        inner,
        "files_inventory_profile",
        "WindowsNtfsTypeSizePathTsvV1",
    )?;
    for field in [
        "files_inventory_full_type_size_path_closure",
        "files_inventory_reparse_and_special_rejected",
        "files_inventory_hardlink_rejected_when_provider_reports_linktype",
        "fresh_attempt_root",
    ] {
        require_json_bool(inner, field, true)?;
    }
    require_json_bool(inner, "seeded_build_results", false)?;
    if json_u64(inner, "build_jobs")? != 1 {
        return Err(invalid("Windows native build jobs must equal one"));
    }
    require_json_string(
        inner,
        "github_actions_runner_registration_state_scope",
        "windows_service_configuration_processes_and_constrained_runner_roots_v1",
    )?;
    require_json_string(inner, "production_scope", "guest_local_exact_gate_only")?;
    require_json_bool(inner, "mac_production_boundary_observed", false)?;
    require_json_string(inner, "bazel", "NOT_RUN_NATIVE_GATE_OUT_OF_SCOPE")?;
    require_json_string(inner, "product_binary", "not_produced")?;
    if inner
        .get("windows_guest_hostname")
        .and_then(Value::as_str)
        .is_none_or(str::is_empty)
    {
        return Err(invalid("Windows guest hostname is absent"));
    }
    let completed = json_string(inner, "completed_at_utc")?;
    if !valid_utc_timestamp(completed) || json_u64(inner, "free_bytes_after")? == 0 {
        return Err(invalid(
            "Windows completion time or free-byte observation is invalid",
        ));
    }
    if matches!(status, "PASS" | "FAIL_CANDIDATE") {
        require_json_bool(inner, "github_actions_runner_active", false)?;
        require_json_bool(
            inner,
            "github_actions_runner_service_configuration_present",
            false,
        )?;
        require_json_bool(
            inner,
            "github_actions_runner_registration_markers_present_in_constrained_roots",
            false,
        )?;
        require_json_string(inner, "portable_hepta_tests_passed_count", "180")?;
        if status == "PASS" && !json_string(inner, "failure_message")?.is_empty() {
            return Err(invalid("Windows PASS carries a failure message"));
        }
    } else {
        for field in [
            "github_actions_runner_active",
            "github_actions_runner_service_configuration_present",
            "github_actions_runner_registration_markers_present_in_constrained_roots",
        ] {
            if inner.get(field) != Some(&Value::Null) {
                return Err(invalid(
                    "Windows blocked gate claims an unverified negative runner state",
                ));
            }
        }
    }

    let inner_manifest = receipt.layer(ManifestLayerIdV3::InnerReceipt)?;
    let driver_manifest_bytes = inner_manifest.bytes("DRIVERS.sha256")?;
    let (driver_manifest_sha256, driver_payloads) = profiles::frozen_windows_driver_identity()
        .ok_or_else(|| invalid("PROFILE_IDENTITY_UNPINNED: Windows v6 final driver"))?;
    if sha256(&driver_manifest_bytes) != driver_manifest_sha256 {
        return Err(invalid(
            "Windows driver manifest is not the frozen v6 identity",
        ));
    }
    let drivers = parse_manifest(&driver_manifest_bytes)?;
    let expected_drivers = driver_payloads
        .into_iter()
        .map(|(path, digest)| (path.to_string(), digest.to_string()))
        .collect::<BTreeMap<_, _>>();
    if drivers != expected_drivers {
        return Err(invalid(
            "Windows DRIVERS.sha256 differs from the exact v6 payload closure",
        ));
    }
    for (path, digest) in drivers {
        if inner_manifest
            .entry(&path)
            .map(|entry| entry.sha256.as_str())
            != Some(digest.as_str())
        {
            return Err(invalid("Windows v6 driver payload differs from its seal"));
        }
    }

    let launcher = parse_key_values(
        &receipt
            .layer(ManifestLayerIdV3::Outer)?
            .bytes("launcher-result.txt")?,
    )?;
    require_exact_kv_fields(
        &launcher,
        &[
            "schema",
            "driver_revision",
            "status",
            "candidate_head",
            "candidate_tree",
            "candidate_parent",
            "upstream_cutoff",
            "nonce",
            "guest_exit_code",
            "guest_status",
            "candidate_execution_started",
            "candidate_execution_completed",
            "x230_recovery_domain",
            "x230_recovery_uuid",
            "x230_recovery_state",
            "x230_original_state",
            "x230_original_autostart",
            "x230_recovery_interface_mac",
            "x230_recovery_guest_ipv4",
            "x230_vm_preflight_evidence",
            "github_actions_runner_active_state_verified",
            "github_actions_runner_registration_state_verified",
            "refs_changed",
            "production_changed",
            "completed_at_utc",
        ],
    )?;
    require_kv_schema(&launcher, "hepta_vnext_windows_native_mac_attempt_v3")?;
    require_kv_u64(&launcher, "driver_revision", 6)?;
    require_kv(&launcher, "status", "RECEIPT_RETRIEVED_VERIFIED_AND_SEALED")?;
    require_candidate_kv(&launcher, candidate, "candidate_head", "upstream_cutoff")?;
    let nonce = launcher
        .get("nonce")
        .ok_or_else(|| invalid("Windows launcher nonce is absent"))?;
    if nonce.len() < 8
        || nonce.len() > 32
        || !nonce
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(invalid("Windows launcher nonce is malformed"));
    }
    validate_windows_nonce_paths(
        nonce,
        json_string(inner, "run_root")?,
        json_string(inner, "source_root")?,
        json_string(inner, "vendor_root")?,
        json_string(inner, "target_root")?,
    )?;
    require_kv(&launcher, "guest_status", status)?;
    let guest_exit_code = launcher
        .get("guest_exit_code")
        .and_then(|value| value.parse::<u64>().ok())
        .ok_or_else(|| invalid("Windows guest exit code is malformed"))?;
    if guest_exit_code.to_string() != launcher["guest_exit_code"]
        || (status == "PASS") != (guest_exit_code == 0)
    {
        return Err(invalid(
            "Windows guest exit code contradicts the inner status",
        ));
    }
    require_kv(
        &launcher,
        "candidate_execution_started",
        if json_bool(inner, "candidate_execution_started")? {
            "true"
        } else {
            "false"
        },
    )?;
    require_kv(
        &launcher,
        "candidate_execution_completed",
        if json_bool(inner, "candidate_execution_completed")? {
            "true"
        } else {
            "false"
        },
    )?;
    for (field, expected) in [
        ("x230_recovery_domain", "win11-dev-hepta-8a84-recovery"),
        ("x230_recovery_uuid", "204cb858-ec25-423a-b9c9-957cef99cf62"),
        ("x230_recovery_state", "running"),
        ("x230_original_state", "shut off"),
        ("x230_original_autostart", "disable"),
        ("x230_recovery_interface_mac", "52:54:00:a9:31:17"),
        ("x230_recovery_guest_ipv4", "192.168.122.218"),
        ("x230_vm_preflight_evidence", "PASS"),
    ] {
        require_kv(&launcher, field, expected)?;
    }
    require_kv_bool(&launcher, "refs_changed", false)?;
    require_kv_bool(&launcher, "production_changed", false)?;
    if !launcher
        .get("completed_at_utc")
        .is_some_and(|value| valid_utc_timestamp(value))
    {
        return Err(invalid("Windows launcher completion time is malformed"));
    }
    if status == "PASS" {
        verify_windows_v6_execution_artifacts(receipt, inner, nonce, candidate)?;
    }
    Ok(())
}

fn verify_windows_v6_execution_artifacts(
    receipt: &VerifiedReceipt,
    result: &Map<String, Value>,
    nonce: &str,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    let manifest = receipt.layer(ManifestLayerIdV3::InnerReceipt)?;
    verify_windows_flat_manifest_transcript(
        manifest,
        "INPUTS.sha256",
        "input-verification.tsv",
        Some("65e9f7e70294c44b5f8e79881af6eebacdcb428452fd47bd8f5dcc54f4fd4bda"),
    )?;
    verify_windows_flat_manifest_transcript(
        manifest,
        "DRIVERS.sha256",
        "driver-verification.tsv",
        profiles::frozen_windows_driver_identity().map(|identity| identity.0),
    )?;
    for marker in [
        "candidate-execution-started.txt",
        "candidate-execution-completed.txt",
    ] {
        let bytes = manifest.bytes(marker)?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|_| invalid("Windows candidate marker is not UTF-8"))?;
        if !text.ends_with('\n')
            || text.lines().count() != 1
            || !valid_utc_timestamp(text.trim_end())
        {
            return Err(invalid(
                "Windows candidate marker is not an exact UTC second",
            ));
        }
    }
    if manifest.bytes("test-suite-counts.tsv")?
        != b"portable_hepta_tests\t180\tportable_hepta_tests.log\n"
    {
        return Err(invalid(
            "Windows portable test count is not the exact 180-test result",
        ));
    }
    require_json_string(result, "portable_hepta_tests_passed_count", "180")?;
    verify_windows_source_identity(manifest, "preflight", candidate)?;
    verify_windows_source_identity(manifest, "postflight", candidate)?;
    verify_windows_resource_preflight(manifest, result)?;
    let exactness = verify_windows_toolchain_exactness(manifest)?;
    verify_windows_tool_inventory(manifest, &exactness, nonce)?;
    verify_windows_git_capture_self_test(manifest)?;
    verify_windows_environment_sanitization(manifest)?;
    verify_windows_native_capture_self_test(manifest, nonce)?;
    Ok(())
}

fn verify_windows_flat_manifest_transcript(
    manifest: &VerifiedManifest,
    manifest_path: &str,
    transcript_path: &str,
    expected_digest: Option<&str>,
) -> Result<(), AcceptanceError> {
    let bytes = manifest.bytes(manifest_path)?;
    if expected_digest.is_some_and(|expected| sha256(&bytes) != expected) {
        return Err(invalid(
            "Windows copied manifest differs from its frozen identity",
        ));
    }
    let parsed = parse_manifest(&bytes)?;
    let expected = parsed
        .iter()
        .map(|(path, digest)| format!("{digest}\t{path}\n"))
        .collect::<String>()
        .into_bytes();
    if manifest.bytes(transcript_path)? != expected {
        return Err(invalid(
            "Windows manifest-verification transcript differs from the sealed manifest",
        ));
    }
    Ok(())
}

fn verify_windows_source_identity(
    manifest: &VerifiedManifest,
    phase: &str,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    let identity = parse_key_values(&manifest.bytes(&format!("{phase}.txt"))?)?;
    require_exact_kv_fields(
        &identity,
        &[
            "schema",
            "phase",
            "candidate_head",
            "candidate_tree",
            "candidate_parent",
            "upstream_cutoff",
            "upstream_cutoff_is_ancestor",
            "worktree_clean",
            "tracked_entry_count",
            "symlink_entry_count",
            "verified_at_utc",
        ],
    )?;
    require_kv_schema(&identity, "hepta_vnext_windows_source_identity_v1")?;
    require_kv(&identity, "phase", phase)?;
    require_candidate_kv(&identity, candidate, "candidate_head", "upstream_cutoff")?;
    require_kv_bool(&identity, "upstream_cutoff_is_ancestor", true)?;
    require_kv_bool(&identity, "worktree_clean", true)?;
    require_kv_u64(&identity, "tracked_entry_count", 6330)?;
    require_kv_u64(&identity, "symlink_entry_count", 1)?;
    if !identity
        .get("verified_at_utc")
        .is_some_and(|value| valid_utc_timestamp(value))
    {
        return Err(invalid("Windows source identity time is malformed"));
    }
    const WINDOWS_BLOB_MANIFEST_MAX_BYTES: usize = 2 * 1024 * 1024;
    let expected_tree = manifest.bytes("expected-git-tree.manifest")?;
    let expected_blobs =
        manifest.bytes_bounded("expected-git-blobs.tsv", WINDOWS_BLOB_MANIFEST_MAX_BYTES)?;
    if manifest.bytes(&format!("{phase}-git-tree.manifest"))? != expected_tree
        || manifest.bytes_bounded(
            &format!("{phase}-windows-materialized-git-blobs.tsv"),
            WINDOWS_BLOB_MANIFEST_MAX_BYTES,
        )? != expected_blobs
    {
        return Err(invalid(
            "Windows pre/post source materialization differs from the sealed input",
        ));
    }
    verify_windows_blob_manifest(&expected_blobs)?;
    if std::str::from_utf8(&expected_tree)
        .ok()
        .is_none_or(|text| text.lines().count() != 6330)
    {
        return Err(invalid("Windows Git tree manifest has the wrong row count"));
    }
    Ok(())
}

fn verify_windows_blob_manifest(bytes: &[u8]) -> Result<(), AcceptanceError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| invalid("Windows materialized blob manifest is not UTF-8"))?;
    if !text.ends_with('\n') || text.lines().count() != 6330 {
        return Err(invalid(
            "Windows materialized blob manifest row count is wrong",
        ));
    }
    let mut previous: Option<&str> = None;
    let mut symlinks = 0_u64;
    for line in text.lines() {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 4
            || !matches!(fields[0], "100644" | "100755" | "120000")
            || !lower_hex_shape(fields[1], 40)
            || !digest_shape(fields[2])
        {
            return Err(invalid("Windows materialized blob row is malformed"));
        }
        validate_relative_path(fields[3])?;
        if previous.is_some_and(|value| value >= fields[3]) {
            return Err(invalid(
                "Windows materialized blob paths are not strictly sorted",
            ));
        }
        previous = Some(fields[3]);
        symlinks += u64::from(fields[0] == "120000");
    }
    if symlinks != 1 {
        return Err(invalid("Windows materialized blob symlink count differs"));
    }
    Ok(())
}

fn lower_hex_shape(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn verify_windows_resource_preflight(
    manifest: &VerifiedManifest,
    result: &Map<String, Value>,
) -> Result<(), AcceptanceError> {
    let bytes = manifest.bytes("resource-preflight.json")?;
    let value = super::strict_json::parse(&bytes)?;
    let object = value
        .as_object()
        .ok_or_else(|| invalid("Windows resource preflight is not an object"))?;
    require_exact_json_fields(
        object,
        &[
            "schema",
            "free_bytes",
            "minimum_free_bytes",
            "free_memory_bytes",
            "minimum_free_memory_bytes",
            "logical_processors",
            "github_actions_runner_service_count",
            "github_actions_runner_process_count",
            "github_actions_runner_active",
            "github_actions_runner_active_state_verified",
            "github_actions_runner_service_configuration_present",
            "github_actions_runner_registration_marker_count",
            "github_actions_runner_registration_markers_present_in_constrained_roots",
            "github_actions_runner_registration_state_scope",
            "github_actions_runner_registration_state_verified",
            "github_actions_runner_root_candidates",
            "github_actions_runner_existing_roots_checked",
            "windows_guest_hostname",
            "observed_at_utc",
        ],
    )?;
    require_json_string(
        object,
        "schema",
        "hepta_vnext_windows_native_resource_preflight_v2",
    )?;
    for field in [
        "github_actions_runner_service_count",
        "github_actions_runner_process_count",
        "github_actions_runner_registration_marker_count",
    ] {
        if json_u64(object, field)? != 0 {
            return Err(invalid("Windows runner resource count is nonzero"));
        }
    }
    for field in [
        "github_actions_runner_active_state_verified",
        "github_actions_runner_registration_state_verified",
    ] {
        require_json_bool(object, field, true)?;
        require_json_bool(result, field, true)?;
    }
    for field in [
        "github_actions_runner_active",
        "github_actions_runner_service_configuration_present",
        "github_actions_runner_registration_markers_present_in_constrained_roots",
    ] {
        require_json_bool(object, field, false)?;
        require_json_bool(result, field, false)?;
    }
    require_json_string(
        object,
        "github_actions_runner_registration_state_scope",
        "windows_service_configuration_processes_and_constrained_runner_roots_v1",
    )?;
    require_json_string(object, "windows_guest_hostname", "DESKTOP-SA9FTJ9")?;
    validate_windows_resource_floors(
        json_u64(object, "free_bytes")?,
        json_u64(object, "minimum_free_bytes")?,
        json_u64(object, "free_memory_bytes")?,
        json_u64(object, "minimum_free_memory_bytes")?,
    )?;
    if json_u64(object, "logical_processors")? == 0
        || !object
            .get("observed_at_utc")
            .and_then(Value::as_str)
            .is_some_and(valid_utc_timestamp)
    {
        return Err(invalid("Windows resource observation is not substantive"));
    }
    for field in [
        "github_actions_runner_root_candidates",
        "github_actions_runner_existing_roots_checked",
    ] {
        let values = object
            .get(field)
            .and_then(Value::as_array)
            .ok_or_else(|| invalid("Windows runner roots are not an array"))?;
        let mut unique = BTreeSet::new();
        for value in values {
            let value = value
                .as_str()
                .filter(|value| !value.is_empty())
                .ok_or_else(|| invalid("Windows runner root is not a string"))?;
            if !unique.insert(value) {
                return Err(invalid("Windows runner roots contain a duplicate"));
            }
        }
    }
    Ok(())
}

fn verify_windows_toolchain_exactness(
    manifest: &VerifiedManifest,
) -> Result<BTreeMap<String, String>, AcceptanceError> {
    let exactness = parse_key_values(&manifest.bytes("toolchain-exactness.txt")?)?;
    require_exact_kv_fields(
        &exactness,
        &[
            "schema",
            "status",
            "git_sha256",
            "bash_sha256",
            "tar_sha256",
            "zstd_sha256",
            "rustc_source_sha256",
            "cargo_source_sha256",
            "rustdoc_source_sha256",
            "clippy_driver_source_sha256",
            "cargo_clippy_source_sha256",
            "rustc_fresh_sha256",
            "cargo_fresh_sha256",
            "rustdoc_fresh_sha256",
            "clippy_driver_fresh_sha256",
            "cargo_clippy_fresh_sha256",
            "rust_toolchain_source_manifest_sha256",
            "rust_toolchain_fresh_manifest_sha256",
            "rust_toolchain_file_count",
            "cl_sha256",
            "link_sha256",
            "rc_sha256",
            "cmd_sha256",
            "powershell_sha256",
            "robocopy_sha256",
            "windows_root",
            "cmd_path",
            "powershell_path",
            "robocopy_path",
            "vctools_install_dir",
            "windows_sdk_version",
            "rc_path",
            "vsdevcmd_executed",
            "deterministic_msvc_sdk_environment_materialized",
            "git_authenticode",
            "bash_authenticode",
            "tar_authenticode",
            "zstd_authenticode",
            "rust_source_authenticode",
            "rust_fresh_authenticode",
            "cl_authenticode",
            "link_authenticode",
            "rc_authenticode",
            "robocopy_authenticode",
            "cmd_authenticode",
            "powershell_authenticode",
            "zstd_provenance",
            "rust_fixture_provenance",
            "source_fixture_pins_verified",
            "fresh_copy_pins_verified",
            "full_rust_fixture_source_copy_manifest_equal",
            "full_rust_fixture_external_manifest_pin",
            "msvc_sdk_full_closure_external_manifest_pin",
            "listed_git_bash_tar_zstd_rust_msvc_sdk_executables_pinned_before_first_use",
            "windows_os_cmd_powershell_robocopy_external_digest_pin",
            "windows_os_cmd_powershell_robocopy_measured_and_authenticode_valid",
            "full_windows_os_msvc_sdk_hermetic",
            "git_bash_companion_userland_full_closure_pinned",
        ],
    )?;
    require_kv_schema(
        &exactness,
        "hepta_vnext_windows_native_toolchain_exactness_v1",
    )?;
    require_kv(&exactness, "status", "PASS")?;
    for (field, expected) in [
        (
            "git_sha256",
            "c470d205517c7a53ceca321df16a6e4549fcd52b576ab4d09536d36f26fda5a9",
        ),
        (
            "bash_sha256",
            "bb67d7991534f97cc98d048d7f1d950fc9cb6f0da426d9233cf0c267588cd388",
        ),
        (
            "tar_sha256",
            "9b77d4c912f2edae8c241d0ece1094d2ac068b084269ceaf85d7c7b085d2ae86",
        ),
        (
            "zstd_sha256",
            "8076aae03feac7c66b319579e82172eed168deed2a3f25e5e2d3c60f55e84111",
        ),
        (
            "rustc_source_sha256",
            "e3ebbd547ea7b73c034d588ba569602b379f3b05ad1a3b5f8dcfab9d4478d74a",
        ),
        (
            "cargo_source_sha256",
            "dc19c8e6d66802d120bf0696b1924b748bd90f3ca16f21391e54a290ff12b7c5",
        ),
        (
            "rustdoc_source_sha256",
            "cafb47cd842bf391eea8a909ddebffa98f0a3b6387fd5997d289d52138ef041b",
        ),
        (
            "clippy_driver_source_sha256",
            "42de42b2fcc5ae9c1390e8e6272f3035cac24fb09fa32dbec67ec35bfb027d57",
        ),
        (
            "cargo_clippy_source_sha256",
            "9f292e31019763fe51ce5673509383acba4ea7ae89d18516e7ae0c4a07d6525a",
        ),
        (
            "cl_sha256",
            "88c8344236a27a6e727e0a8edc49aaa2690bdc7a9464b9d18cc7abe70a9f1c0d",
        ),
        (
            "link_sha256",
            "ca11e6c45debd34bf652dfe984c5360a531a005ed78bf72852330c9c2590cf0d",
        ),
        (
            "rc_sha256",
            "43da1503c262c30894c851589bf0155f8365d77e63a5f7bc13982320e3a6b42d",
        ),
    ] {
        require_kv(&exactness, field, expected)?;
    }
    for (source, fresh) in [
        ("rustc_source_sha256", "rustc_fresh_sha256"),
        ("cargo_source_sha256", "cargo_fresh_sha256"),
        ("rustdoc_source_sha256", "rustdoc_fresh_sha256"),
        ("clippy_driver_source_sha256", "clippy_driver_fresh_sha256"),
        ("cargo_clippy_source_sha256", "cargo_clippy_fresh_sha256"),
    ] {
        require_kv(
            &exactness,
            fresh,
            exactness
                .get(source)
                .ok_or_else(|| invalid("Windows source tool digest is absent"))?,
        )?;
    }
    verify_windows_rust_closure(manifest, &exactness)?;
    for (field, expected) in [
        ("windows_root", r"C:\Windows"),
        ("cmd_path", r"C:\Windows\System32\cmd.exe"),
        (
            "powershell_path",
            r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe",
        ),
        ("robocopy_path", r"C:\Windows\System32\robocopy.exe"),
        (
            "vctools_install_dir",
            r"C:\BuildTools\VC\Tools\MSVC\14.44.35207",
        ),
        ("windows_sdk_version", "10.0.26100.0"),
        (
            "rc_path",
            r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\rc.exe",
        ),
        (
            "zstd_provenance",
            "preinstalled_recovery_fixture_digest_pinned",
        ),
        (
            "rust_fixture_provenance",
            "preinstalled_rust_1.95_fixture_source_and_fresh_copy_digest_pinned",
        ),
    ] {
        require_kv(&exactness, field, expected)?;
    }
    for field in [
        "vsdevcmd_executed",
        "full_rust_fixture_external_manifest_pin",
        "msvc_sdk_full_closure_external_manifest_pin",
        "windows_os_cmd_powershell_robocopy_external_digest_pin",
        "full_windows_os_msvc_sdk_hermetic",
        "git_bash_companion_userland_full_closure_pinned",
    ] {
        require_kv_bool(&exactness, field, false)?;
    }
    for field in [
        "deterministic_msvc_sdk_environment_materialized",
        "source_fixture_pins_verified",
        "fresh_copy_pins_verified",
        "full_rust_fixture_source_copy_manifest_equal",
        "listed_git_bash_tar_zstd_rust_msvc_sdk_executables_pinned_before_first_use",
        "windows_os_cmd_powershell_robocopy_measured_and_authenticode_valid",
    ] {
        require_kv_bool(&exactness, field, true)?;
    }
    for (field, expected) in [
        ("git_authenticode", "Valid"),
        ("bash_authenticode", "Valid"),
        ("tar_authenticode", "Valid"),
        ("zstd_authenticode", "NotSigned"),
        ("rust_source_authenticode", "NotSigned"),
        ("rust_fresh_authenticode", "NotSigned"),
        ("cl_authenticode", "Valid"),
        ("link_authenticode", "Valid"),
        ("rc_authenticode", "Valid"),
        ("robocopy_authenticode", "Valid"),
        ("cmd_authenticode", "Valid"),
        ("powershell_authenticode", "Valid"),
    ] {
        require_kv(&exactness, field, expected)?;
    }
    Ok(exactness)
}

fn verify_windows_rust_closure(
    manifest: &VerifiedManifest,
    exactness: &BTreeMap<String, String>,
) -> Result<(), AcceptanceError> {
    let source = manifest.bytes("rust-toolchain-source-files.sha256")?;
    let fresh = manifest.bytes("rust-toolchain-fresh-files.sha256")?;
    if source != fresh {
        return Err(invalid(
            "Windows source/fresh Rust toolchain closures differ",
        ));
    }
    let entries = parse_manifest(&source)?;
    let count = exactness
        .get("rust_toolchain_file_count")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|count| *count > 0 && count.to_string() == exactness["rust_toolchain_file_count"])
        .ok_or_else(|| invalid("Windows Rust toolchain closure count is malformed"))?;
    if entries.len() != count {
        return Err(invalid("Windows Rust toolchain closure count differs"));
    }
    for field in [
        "rust_toolchain_source_manifest_sha256",
        "rust_toolchain_fresh_manifest_sha256",
    ] {
        require_kv(exactness, field, &sha256(&source))?;
    }
    Ok(())
}

fn verify_windows_tool_inventory(
    manifest: &VerifiedManifest,
    exactness: &BTreeMap<String, String>,
    nonce: &str,
) -> Result<(), AcceptanceError> {
    let value = super::strict_json::parse(&manifest.bytes("tool-inventory.json")?)?;
    let entries = value
        .as_array()
        .filter(|entries| entries.len() == 15)
        .ok_or_else(|| invalid("Windows tool inventory does not contain exactly 15 tools"))?;
    let mut by_path = BTreeMap::new();
    let mut digests = BTreeSet::new();
    for entry in entries {
        let object = entry
            .as_object()
            .ok_or_else(|| invalid("Windows tool inventory entry is not an object"))?;
        require_exact_json_fields(
            object,
            &[
                "path",
                "sha256",
                "size_bytes",
                "file_version",
                "authenticode_status",
                "signer",
            ],
        )?;
        let path = json_string(object, "path")?;
        let digest = json_string(object, "sha256")?;
        if path.as_bytes().get(1).is_none_or(|value| *value != b':')
            || !path.contains('\\')
            || !digest_shape(digest)
            || json_u64(object, "size_bytes")? == 0
            || !object
                .get("authenticode_status")
                .and_then(Value::as_str)
                .is_some_and(|value| matches!(value, "Valid" | "NotSigned"))
            || !matches!(
                object.get("file_version"),
                Some(Value::Null | Value::String(_))
            )
            || !matches!(object.get("signer"), Some(Value::Null | Value::String(_)))
        {
            return Err(invalid("Windows tool inventory entry is malformed"));
        }
        if by_path.insert(path, (digest, object)).is_some() || !digests.insert(digest) {
            return Err(invalid(
                "Windows tool inventory path or digest is duplicated",
            ));
        }
    }
    let fresh_bin =
        format!(r"C:\q\52ec-{nonce}\rustup\toolchains\1.95.0-x86_64-pc-windows-msvc\bin");
    for (field, path) in [
        (
            "git_sha256",
            r"C:\Program Files\Git\cmd\git.exe".to_string(),
        ),
        (
            "bash_sha256",
            r"C:\Program Files\Git\bin\bash.exe".to_string(),
        ),
        ("tar_sha256", r"C:\Windows\System32\tar.exe".to_string()),
        ("rustc_fresh_sha256", format!(r"{fresh_bin}\rustc.exe")),
        ("cargo_fresh_sha256", format!(r"{fresh_bin}\cargo.exe")),
        ("rustdoc_fresh_sha256", format!(r"{fresh_bin}\rustdoc.exe")),
        (
            "clippy_driver_fresh_sha256",
            format!(r"{fresh_bin}\clippy-driver.exe"),
        ),
        (
            "cargo_clippy_fresh_sha256",
            format!(r"{fresh_bin}\cargo-clippy.exe"),
        ),
        (
            "cl_sha256",
            r"C:\BuildTools\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64\cl.exe".to_string(),
        ),
        (
            "link_sha256",
            r"C:\BuildTools\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64\link.exe".to_string(),
        ),
        (
            "rc_sha256",
            r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\rc.exe".to_string(),
        ),
        ("cmd_sha256", r"C:\Windows\System32\cmd.exe".to_string()),
        (
            "powershell_sha256",
            r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe".to_string(),
        ),
        (
            "robocopy_sha256",
            r"C:\Windows\System32\robocopy.exe".to_string(),
        ),
    ] {
        let expected = exactness
            .get(field)
            .ok_or_else(|| invalid("Windows tool exactness digest is absent"))?;
        if by_path.get(path.as_str()).map(|value| value.0) != Some(expected.as_str()) {
            return Err(invalid(
                "Windows tool inventory path/digest binding differs",
            ));
        }
    }
    let zstd = exactness
        .get("zstd_sha256")
        .ok_or_else(|| invalid("Windows zstd digest is absent"))?;
    if by_path
        .iter()
        .filter(|(path, (digest, _))| {
            path.starts_with(r"C:\hepta-recovery\zstd-v1.5.7\")
                && path.ends_with(r"\zstd.exe")
                && *digest == zstd
        })
        .count()
        != 1
    {
        return Err(invalid("Windows zstd inventory binding differs"));
    }
    Ok(())
}

fn verify_windows_git_capture_self_test(
    manifest: &VerifiedManifest,
) -> Result<(), AcceptanceError> {
    let values = parse_key_values(&manifest.bytes("git-capture-self-test.txt")?)?;
    require_exact_kv_fields(
        &values,
        &[
            "schema",
            "status",
            "command_not_found_rejected",
            "fake_git_nonzero_empty_stdout_rejected",
            "nonrepo_git_status_rejected",
            "empty_stdout_requires_exit_zero",
            "exact_git_path_used",
        ],
    )?;
    require_kv_schema(&values, "hepta_vnext_windows_git_capture_self_test_v1")?;
    require_kv(&values, "status", "PASS")?;
    for field in [
        "command_not_found_rejected",
        "fake_git_nonzero_empty_stdout_rejected",
        "nonrepo_git_status_rejected",
        "empty_stdout_requires_exit_zero",
        "exact_git_path_used",
    ] {
        require_kv_bool(&values, field, true)?;
    }
    Ok(())
}

fn verify_windows_environment_sanitization(
    manifest: &VerifiedManifest,
) -> Result<(), AcceptanceError> {
    let values = parse_key_values(&manifest.bytes("environment-sanitization.txt")?)?;
    require_exact_kv_fields(
        &values,
        &[
            "schema",
            "status",
            "removed_variable_count",
            "removed_variable_names",
            "removed_variable_values_recorded",
            "rust_and_cargo_ambient_overrides_removed",
            "git_ambient_overrides_removed",
            "compiler_wrapper_and_flag_overrides_removed",
            "bash_startup_and_exported_function_overrides_removed",
            "proxy_variables_absent",
            "cargo_home",
            "rustup_home",
            "cargo_target_dir",
            "cargo_net_offline",
            "cargo_incremental",
            "cargo_build_jobs",
            "rustc_exact_path",
            "rustdoc_exact_path",
            "target_linker_exact_path",
            "git_config_system_disabled",
            "git_config_global_empty_nonce_file",
            "git_terminal_prompt",
            "git_allowed_protocol",
            "ambient_path_include_lib_libpath_removed",
            "vsdevcmd_executed",
            "deterministic_msvc_sdk_environment_materialized",
            "path_environment_deterministic",
            "full_windows_os_msvc_sdk_hermetic",
        ],
    )?;
    require_kv_schema(
        &values,
        "hepta_vnext_windows_native_environment_sanitization_v1",
    )?;
    require_kv(&values, "status", "PASS")?;
    let count = values
        .get("removed_variable_count")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|count| count.to_string() == values["removed_variable_count"])
        .ok_or_else(|| invalid("Windows removed-variable count is malformed"))?;
    let names = values
        .get("removed_variable_names")
        .ok_or_else(|| invalid("Windows removed-variable names are absent"))?;
    if (count == 0 && names != "none")
        || (count > 0
            && (names.split(',').count() != count
                || names.split(',').any(|name| !valid_environment_name(name))))
    {
        return Err(invalid(
            "Windows removed-variable names disagree with the count",
        ));
    }
    for (field, expected) in [
        ("cargo_home", "fresh_nonce_root"),
        ("rustup_home", "fresh_nonce_root"),
        ("cargo_target_dir", "fresh_nonce_root"),
        ("cargo_incremental", "0"),
        ("cargo_build_jobs", "1"),
        ("git_terminal_prompt", "0"),
        ("git_allowed_protocol", "file"),
    ] {
        require_kv(&values, field, expected)?;
    }
    for field in [
        "removed_variable_values_recorded",
        "vsdevcmd_executed",
        "full_windows_os_msvc_sdk_hermetic",
    ] {
        require_kv_bool(&values, field, false)?;
    }
    for field in [
        "rust_and_cargo_ambient_overrides_removed",
        "git_ambient_overrides_removed",
        "compiler_wrapper_and_flag_overrides_removed",
        "bash_startup_and_exported_function_overrides_removed",
        "proxy_variables_absent",
        "cargo_net_offline",
        "rustc_exact_path",
        "rustdoc_exact_path",
        "target_linker_exact_path",
        "git_config_system_disabled",
        "git_config_global_empty_nonce_file",
        "ambient_path_include_lib_libpath_removed",
        "deterministic_msvc_sdk_environment_materialized",
        "path_environment_deterministic",
    ] {
        require_kv_bool(&values, field, true)?;
    }
    Ok(())
}

fn valid_environment_name(value: &str) -> bool {
    let mut bytes = value.bytes();
    bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn verify_windows_native_capture_self_test(
    manifest: &VerifiedManifest,
    nonce: &str,
) -> Result<(), AcceptanceError> {
    let values = parse_key_values(&manifest.bytes("native-capture-self-test.txt")?)?;
    require_exact_kv_fields(
        &values,
        &[
            "schema",
            "status",
            "exit_zero_with_stderr",
            "stdout_and_stderr_separately_captured",
            "nonzero_exit_fail_closed",
            "failed_status_not_treated_as_clean",
            "real_invocation_errors_not_swallowed",
            "nonexistent_executable_rejected",
            "nonexistent_executable_capture_status",
            "nonexistent_executable_error_kind",
            "comspec_canonical_system32_cmd",
            "cmd_authenticode",
        ],
    )?;
    require_kv_schema(&values, "hepta_vnext_windows_native_capture_self_test_v1")?;
    require_kv(&values, "status", "PASS")?;
    require_kv(&values, "exit_zero_with_stderr", "PASS")?;
    require_kv(&values, "nonzero_exit_fail_closed", "PASS")?;
    require_kv(
        &values,
        "nonexistent_executable_capture_status",
        "INVOCATION_ERROR",
    )?;
    require_kv(
        &values,
        "nonexistent_executable_error_kind",
        "EXECUTABLE_MISSING_OR_NOT_REGULAR",
    )?;
    require_kv(&values, "cmd_authenticode", "Valid")?;
    for field in [
        "stdout_and_stderr_separately_captured",
        "failed_status_not_treated_as_clean",
        "real_invocation_errors_not_swallowed",
        "nonexistent_executable_rejected",
        "comspec_canonical_system32_cmd",
    ] {
        require_kv_bool(&values, field, true)?;
    }
    verify_windows_native_record(
        manifest,
        "native-capture-success-stderr.log",
        "COMPLETED",
        Some(0),
        None,
        Some("hepta-native-success-stderr"),
    )?;
    verify_windows_native_record(
        manifest,
        "native-capture-nonzero.log",
        "COMPLETED",
        Some(23),
        None,
        Some("hepta-native-nonzero"),
    )?;
    verify_windows_native_record(
        manifest,
        "native-capture-invocation-error.log",
        "INVOCATION_ERROR",
        None,
        Some("EXECUTABLE_MISSING_OR_NOT_REGULAR"),
        None,
    )?;
    let invocation = parse_key_values(&manifest.bytes("native-capture-invocation-error.log")?)?;
    require_kv(
        &invocation,
        "file_path",
        &format!(r"C:\q\52ec-{nonce}\nonexistent-native-command.exe"),
    )?;
    Ok(())
}

fn verify_windows_native_record(
    manifest: &VerifiedManifest,
    record_path: &str,
    status: &str,
    exit_code: Option<u64>,
    error_kind: Option<&str>,
    stderr_marker: Option<&str>,
) -> Result<(), AcceptanceError> {
    let values = parse_key_values(&manifest.bytes(record_path)?)?;
    let expected_fields = if exit_code.is_some() {
        &[
            "schema",
            "status",
            "file_path",
            "exit_code",
            "stdout_file",
            "stderr_file",
            "stdout_sha256",
            "stderr_sha256",
        ][..]
    } else {
        &[
            "schema",
            "status",
            "error_kind",
            "file_path",
            "stdout_file",
            "stderr_file",
            "stdout_sha256",
            "stderr_sha256",
            "message",
        ][..]
    };
    require_exact_kv_fields(&values, expected_fields)?;
    require_kv_schema(&values, "hepta_vnext_windows_native_invocation_v1")?;
    require_kv(&values, "status", status)?;
    if let Some(exit_code) = exit_code {
        require_kv_u64(&values, "exit_code", exit_code)?;
        require_kv(&values, "file_path", r"C:\Windows\System32\cmd.exe")?;
    }
    if let Some(error_kind) = error_kind {
        require_kv(&values, "error_kind", error_kind)?;
    }
    for stream in ["stdout", "stderr"] {
        let file_field = format!("{stream}_file");
        let digest_field = format!("{stream}_sha256");
        let expected_file = format!("{record_path}.{stream}.log");
        require_kv(&values, &file_field, &expected_file)?;
        let bytes = manifest.bytes(&expected_file)?;
        require_kv(&values, &digest_field, &sha256(&bytes))?;
    }
    if let Some(marker) = stderr_marker {
        let stderr = manifest.bytes(&format!("{record_path}.stderr.log"))?;
        if !std::str::from_utf8(&stderr)
            .ok()
            .is_some_and(|text| text.contains(marker))
        {
            return Err(invalid("Windows native capture stderr marker is absent"));
        }
    }
    Ok(())
}

fn require_outer_recursive_verification(
    outer: &BTreeMap<String, String>,
    require_tool_binding: bool,
) -> Result<(), AcceptanceError> {
    for field in [
        "inner_recursive_hashes",
        "inner_recursive_modes",
        "inner_manifest_coverage",
        "candidate_binding",
    ] {
        require_kv(outer, field, "pass")?;
    }
    if require_tool_binding {
        require_kv(outer, "tool_binding", "pass")?;
    }
    require_kv_bool(outer, "remote_roots_preserved", true)?;
    require_kv_bool(outer, "promotion_authority", false)
}

pub(super) fn validate_kv_execution_fields(
    profile: EvidenceProfileV3,
    inner: &BTreeMap<String, String>,
    status: &str,
) -> Result<(), AcceptanceError> {
    match profile {
        EvidenceProfileV3::LinuxExactV5 => {
            let harness_blocked = kv_bool(inner, "harness_blocked")?;
            if harness_blocked != (status == "BLOCKED_HARNESS") {
                return Err(invalid(
                    "Linux harness_blocked differs from the exact status",
                ));
            }
            require_kv_bool(inner, "data_deleted", false)?;
            require_kv_bool(inner, "promotion_authority", false)?;
            match status {
                "PASS" => {
                    require_kv_bool(inner, "harness_preflight_pass", true)?;
                    require_kv_bool(inner, "candidate_execution_started", true)?;
                    require_kv_bool(inner, "candidate_execution_completed", true)?;
                    require_kv_bool(inner, "postflight_verified", true)?;
                    require_kv(inner, "source_identity", "match")?;
                    require_kv_bool(inner, "worktree_clean", true)?;
                }
                "FAIL_CANDIDATE" => {
                    require_kv_bool(inner, "harness_preflight_pass", true)?;
                    require_kv_bool(inner, "candidate_execution_started", true)?;
                    require_kv_bool(inner, "candidate_execution_completed", false)?;
                    require_kv_bool(inner, "postflight_verified", false)?;
                    require_kv(inner, "source_identity", "match")?;
                    require_kv_bool(inner, "worktree_clean", true)?;
                }
                "BLOCKED_HARNESS" => {}
                _ => return Err(invalid("Linux status differs from its fixed profile")),
            }
        }
        EvidenceProfileV3::NixExactV3 => {
            require_kv_bool(inner, "interrupted", false)?;
            require_kv_bool(inner, "data_deleted", false)?;
            require_kv_bool(inner, "promotion_authority", false)?;
            match status {
                "PASS" => {
                    require_kv_bool(inner, "candidate_execution_started", true)?;
                    require_kv_bool(inner, "candidate_execution_completed", true)?;
                    require_kv_bool(inner, "source_postflight_verified", true)?;
                    require_kv_bool(inner, "resource_monitor_verified", true)?;
                    require_kv_bool(inner, "pass_evidence_verified", true)?;
                    require_kv_bool(inner, "resource_binding_verified", true)?;
                    require_kv_bool(inner, "probe_verified", true)?;
                }
                "FAIL_CANDIDATE" => {
                    require_kv_bool(inner, "candidate_execution_started", true)?;
                    require_kv_bool(inner, "candidate_execution_completed", false)?;
                    require_kv_bool(inner, "source_postflight_verified", true)?;
                    require_kv_bool(inner, "resource_monitor_verified", true)?;
                    require_kv_bool(inner, "resource_binding_verified", true)?;
                    require_kv_bool(inner, "probe_verified", true)?;
                }
                "BLOCKED_HARNESS" => {}
                _ => return Err(invalid("Nix status differs from its fixed profile")),
            }
        }
        _ => return Err(invalid("key/value execution profile is not compiled")),
    }
    Ok(())
}

pub(super) fn validate_windows_execution_fields(
    inner: &Map<String, Value>,
    outer: &BTreeMap<String, String>,
    status: &str,
) -> Result<(), AcceptanceError> {
    if matches!(status, "PASS" | "FAIL_CANDIDATE") {
        require_json_bool(inner, "harness_preflight_pass", true)?;
        require_json_bool(inner, "candidate_execution_started", true)?;
        require_json_bool(inner, "candidate_execution_completed", true)?;
        require_json_bool(inner, "postflight_verified", true)?;
        require_json_string(inner, "source_identity", "match")?;
        require_json_bool(inner, "worktree_clean", true)?;
        require_json_bool(inner, "github_actions_runner_active_state_verified", true)?;
        require_json_bool(
            inner,
            "github_actions_runner_registration_state_verified",
            true,
        )?;
        require_kv_bool(outer, "github_actions_runner_active_state_verified", true)?;
        require_kv_bool(
            outer,
            "github_actions_runner_registration_state_verified",
            true,
        )?;
    }
    Ok(())
}

fn validate_gate_shape(
    observed: &ObservedGateV3,
    expected_steps: u64,
) -> Result<(), AcceptanceError> {
    match observed.status.as_str() {
        "PASS"
            if observed.pass
                && observed.qualification
                && observed.candidate_executed
                && !observed.candidate_failure
                && !observed.harness_failure
                && observed.executed_steps == expected_steps => {}
        "BLOCKED_HARNESS"
            if !observed.pass
                && !observed.qualification
                && !observed.candidate_failure
                && observed.harness_failure
                && observed.executed_steps <= expected_steps => {}
        "FAIL_CANDIDATE"
            if !observed.pass
                && !observed.qualification
                && observed.candidate_executed
                && observed.candidate_failure
                && !observed.harness_failure
                && observed.executed_steps > 0
                && observed.executed_steps <= expected_steps => {}
        _ => return Err(invalid("platform gate has an invalid exact status shape")),
    }
    Ok(())
}

fn pass_gate(
    executed_steps: u64,
    production_changed: Option<bool>,
    refs_changed: Option<bool>,
) -> ObservedGateV3 {
    ObservedGateV3 {
        candidate_executed: true,
        candidate_failure: false,
        executed_steps,
        harness_failure: false,
        pass: true,
        production_changed,
        qualification: true,
        refs_changed,
        status: "PASS".to_string(),
    }
}

fn observe_prerequisite(
    profile: EvidenceProfileV3,
    receipt: &VerifiedReceipt,
    candidate: &CandidateBindingV3,
) -> Result<ObservedPrerequisiteV3, AcceptanceError> {
    let observed = match profile {
        EvidenceProfileV3::PortableInputsV1 => {
            let complete = kv_artifact(receipt, profile, ManifestLayerIdV3::Outer)?;
            require_exact_kv_fields(
                &complete,
                &[
                    "schema",
                    "status",
                    "candidate_head",
                    "candidate_tree",
                    "candidate_parent",
                    "candidate_ref",
                    "tracked_entry_count",
                    "vendor_file_count",
                    "vendor_archive_sha256",
                    "bundle_sha256",
                    "generated_at_utc",
                    "source_worktree_clean",
                    "refs_changed",
                    "production_changed",
                    "candidate_fail",
                ],
            )?;
            require_kv_schema(&complete, "hepta_vnext_portable_generation_v1")?;
            require_kv(&complete, "status", "pass")?;
            require_kv(&complete, "candidate_head", &candidate.head)?;
            require_kv(&complete, "candidate_tree", &candidate.tree)?;
            require_kv(&complete, "candidate_parent", &candidate.parents[0])?;
            require_kv_bool(&complete, "source_worktree_clean", true)?;
            require_kv_bool(&complete, "candidate_fail", false)?;
            require_kv_bool(&complete, "refs_changed", false)?;
            require_kv_bool(&complete, "production_changed", false)?;
            require_kv(
                &complete,
                "candidate_ref",
                "refs/heads/hepta/vnext-main-integration-20260811",
            )?;
            require_kv_u64(&complete, "tracked_entry_count", 6330)?;
            require_kv_u64(&complete, "vendor_file_count", 72857)?;
            require_kv(&complete, "bundle_sha256", &candidate.bundle.sha256)?;
            if !complete
                .get("vendor_archive_sha256")
                .is_some_and(|value| digest_shape(value))
                || !complete
                    .get("generated_at_utc")
                    .is_some_and(|value| valid_utc_timestamp(value))
            {
                return Err(invalid("portable generation digest or time is malformed"));
            }
            let binding = parse_key_values(
                &receipt
                    .layer(ManifestLayerIdV3::Outer)?
                    .bytes("candidate-binding.txt")?,
            )?;
            require_exact_kv_fields(
                &binding,
                &[
                    "schema",
                    "candidate_head",
                    "candidate_tree",
                    "candidate_parent",
                    "integration_merge",
                    "upstream_cutoff",
                    "advertised_ref",
                    "source_branch",
                    "tracked_entry_count",
                    "symlink_entry_count",
                    "windows_materialized_symlink_path",
                    "windows_materialized_symlink_blob",
                    "windows_materialized_symlink_sha256",
                    "cargo_lock_sha256",
                    "cutover_bridge_sha256",
                    "source_worktree",
                    "source_worktree_clean",
                    "prepared_at_utc",
                    "refs_changed",
                    "production_changed",
                ],
            )?;
            require_kv_schema(&binding, "hepta_vnext_portable_exact_inputs_v1")?;
            require_candidate_kv(&binding, candidate, "candidate_head", "upstream_cutoff")?;
            require_kv(&binding, "integration_merge", &candidate.integration_merge)?;
            require_kv_bool(&binding, "source_worktree_clean", true)?;
            require_kv_bool(&binding, "refs_changed", false)?;
            require_kv_bool(&binding, "production_changed", false)?;
            require_kv(
                &binding,
                "advertised_ref",
                "refs/heads/hepta/vnext-main-integration-20260811",
            )?;
            require_kv(
                &binding,
                "source_branch",
                "hepta/vnext-main-integration-20260811",
            )?;
            require_kv_u64(&binding, "tracked_entry_count", 6330)?;
            require_kv_u64(&binding, "symlink_entry_count", 1)?;
            require_kv(
                &binding,
                "windows_materialized_symlink_path",
                "codex-rs/vendor/bubblewrap/LICENSE",
            )?;
            require_kv(
                &binding,
                "source_worktree",
                "/Volumes/T5/hepta-vnext/worktrees/main-integration",
            )?;
            for field in [
                "windows_materialized_symlink_sha256",
                "cargo_lock_sha256",
                "cutover_bridge_sha256",
            ] {
                if !binding.get(field).is_some_and(|value| digest_shape(value)) {
                    return Err(invalid(format!(
                        "portable binding digest is malformed: {field}"
                    )));
                }
            }
            if !binding
                .get("prepared_at_utc")
                .is_some_and(|value| valid_utc_timestamp(value))
            {
                return Err(invalid("portable preparation time is malformed"));
            }
            pass_prerequisite(Some(false), Some(false))
        }
        EvidenceProfileV3::CanonicalPathTrustV2 => {
            let status = kv_artifact(receipt, profile, ManifestLayerIdV3::Outer)?;
            require_exact_kv_fields(
                &status,
                &[
                    "schema",
                    "status",
                    "receipt_attempt",
                    "candidate_head",
                    "candidate_tree",
                    "candidate_parent",
                    "canonical_worktree",
                    "canonical_branch",
                    "canonical_local_ref_exact",
                    "worktree_clean",
                    "t5_uuid_exact",
                    "t5_owners_enabled",
                    "openclaw_agent_workspaces_aligned",
                    "codex_configs_checked",
                    "codex_lane_scoped_trust_aligned",
                    "explicit_old_workspace_trust_entries",
                    "agent_instructions_point_to_t5",
                    "agent_identities_point_to_t5",
                    "old_workspace_paths_frozen",
                    "remote_candidate_ref",
                    "remote_candidate_switch_deferred",
                    "local_main_head",
                    "remote_main_head",
                    "remote_default_branch",
                    "candidate_archive_branch_exact",
                    "candidate_archive_tag_exact",
                    "hosted_qualification_ref_exact",
                    "default_main_changed",
                    "production_changed",
                ],
            )?;
            require_kv_schema(&status, "hepta_vnext_canonical_path_trust_v2")?;
            require_kv(&status, "status", "pass")?;
            require_kv_u64(&status, "receipt_attempt", 4)?;
            require_kv(&status, "candidate_head", &candidate.head)?;
            require_kv(&status, "candidate_tree", &candidate.tree)?;
            require_kv(&status, "candidate_parent", &candidate.parents[0])?;
            require_kv_bool(&status, "worktree_clean", true)?;
            require_kv(
                &status,
                "canonical_worktree",
                "/Volumes/T5/hepta-vnext/worktrees/main-integration",
            )?;
            require_kv(
                &status,
                "canonical_branch",
                "hepta/vnext-main-integration-20260811",
            )?;
            for field in [
                "canonical_local_ref_exact",
                "t5_uuid_exact",
                "t5_owners_enabled",
                "openclaw_agent_workspaces_aligned",
                "codex_lane_scoped_trust_aligned",
                "agent_instructions_point_to_t5",
                "agent_identities_point_to_t5",
                "old_workspace_paths_frozen",
                "remote_candidate_switch_deferred",
                "candidate_archive_branch_exact",
                "candidate_archive_tag_exact",
                "hosted_qualification_ref_exact",
            ] {
                require_kv_bool(&status, field, true)?;
            }
            require_kv_bool(&status, "explicit_old_workspace_trust_entries", false)?;
            require_kv_u64(&status, "codex_configs_checked", 6)?;
            require_kv(
                &status,
                "remote_candidate_ref",
                "09e9e9ff7fa6b6c1d129d0c7a858979823e13ae8",
            )?;
            require_kv(
                &status,
                "local_main_head",
                "fe848052ceed06ed431e20893f15516fd349ffe5",
            )?;
            require_kv(
                &status,
                "remote_main_head",
                "1577a50e37c6332ab267dea9d838dab8b8c07536",
            )?;
            require_kv(&status, "remote_default_branch", "main")?;
            require_kv_bool(&status, "default_main_changed", false)?;
            require_kv_bool(&status, "production_changed", false)?;
            verify_path_trust_artifacts(receipt.layer(ManifestLayerIdV3::Outer)?, candidate)?;
            pass_prerequisite(Some(false), None)
        }
        EvidenceProfileV3::UpstreamCutoffObservationV1 => {
            let status = kv_artifact(receipt, profile, ManifestLayerIdV3::Outer)?;
            require_exact_kv_fields(
                &status,
                &[
                    "schema",
                    "observed_utc",
                    "candidate_head",
                    "candidate_tree",
                    "frozen_upstream_cutoff",
                    "observed_upstream_main",
                    "post_cutoff_commit_count",
                    "post_cutoff_behind_count",
                    "new_since_previous_review",
                    "new_commit_scope",
                    "new_commit_reachable_from_narrow_hepta_serve_ui_release",
                    "source",
                    "observation_method",
                    "candidate_changed",
                    "qualification_invalidated",
                    "policy",
                ],
            )?;
            require_kv_schema(&status, "hepta_vnext_upstream_cutoff_observation_v1")?;
            require_kv(&status, "candidate_head", &candidate.head)?;
            require_kv(&status, "candidate_tree", &candidate.tree)?;
            require_kv(
                &status,
                "frozen_upstream_cutoff",
                &candidate.upstream_cutoff,
            )?;
            require_kv_bool(&status, "candidate_changed", false)?;
            require_kv_bool(&status, "qualification_invalidated", false)?;
            require_kv(&status, "observed_utc", "2026-08-13T11:21:18Z")?;
            require_kv(
                &status,
                "observed_upstream_main",
                "c30a3e49c9231361abeaa88d4a57bb7c3e248a50",
            )?;
            require_kv_u64(&status, "post_cutoff_commit_count", 33)?;
            require_kv_u64(&status, "post_cutoff_behind_count", 0)?;
            require_kv(
                &status,
                "new_since_previous_review",
                "c30a3e49c9231361abeaa88d4a57bb7c3e248a50",
            )?;
            require_kv(
                &status,
                "new_commit_scope",
                "exec_server_sandboxed_file_streaming",
            )?;
            require_kv_bool(
                &status,
                "new_commit_reachable_from_narrow_hepta_serve_ui_release",
                false,
            )?;
            require_kv(&status, "source", "https://github.com/openai/codex.git")?;
            require_kv(
                &status,
                "observation_method",
                "git_ls_remote_refs_heads_main_github_compare_api_and_commit_diff_review",
            )?;
            require_kv(
                &status,
                "policy",
                "freeze_52ec_for_narrow_hepta_serve_ui_and_absorb_post_cutoff_upstream_in_first_post_cutover_cycle",
            )?;
            pass_prerequisite(None, None)
        }
        _ => return Err(invalid("profile is not a compiled prerequisite")),
    };
    Ok(observed)
}

fn pass_prerequisite(
    production_changed: Option<bool>,
    refs_changed: Option<bool>,
) -> ObservedPrerequisiteV3 {
    ObservedPrerequisiteV3 {
        pass: true,
        production_changed,
        refs_changed,
        status: "PASS".to_string(),
    }
}

fn verify_path_trust_artifacts(
    manifest: &VerifiedManifest,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    let require_text = |path: &str, needles: &[&str]| -> Result<(), AcceptanceError> {
        let bytes = manifest.bytes(path)?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|_| invalid(format!("PathTrust artifact is not UTF-8: {path}")))?;
        if needles.iter().any(|needle| !text.contains(needle)) {
            return Err(invalid(format!(
                "PathTrust artifact omits an exact substantive binding: {path}"
            )));
        }
        Ok(())
    };
    require_text(
        "t5-volume-info.txt",
        &[
            "Mount Point:               /Volumes/T5",
            "File System Personality:   APFS",
            "Type (Bundle):             apfs",
            "Owners:                    Enabled",
            "Volume UUID:               FB804D1B-24CB-4D6E-AEA7-A9E180807758",
            "This disk is an APFS Volume.",
        ],
    )?;
    require_text(
        "canonical-worktree-status.txt",
        &[
            &format!("# branch.oid {}", candidate.head),
            "# branch.head hepta/vnext-main-integration-20260811",
            "# branch.upstream hepta-ci/integration/vnext-main-20260811",
            "# branch.ab +19 -0",
        ],
    )?;
    require_text(
        "local-canonical-ref.txt",
        &[&format!(
            "{} refs/heads/hepta/vnext-main-integration-20260811",
            candidate.head
        )],
    )?;
    require_text(
        "remote-candidate-preservation-refs.txt",
        &[
            &format!(
                "{}\trefs/heads/archive/vnext-candidate-52ec4b3868-20260813",
                candidate.head
            ),
            &format!(
                "{}\trefs/heads/integration/vnext-main-full-ci-52ec-20260813",
                candidate.head
            ),
            &format!(
                "{}\trefs/tags/hepta-vnext-candidate-52ec4b3868-20260813",
                candidate.head
            ),
        ],
    )?;
    require_text(
        "remote-head.txt",
        &[
            "ref: refs/heads/main\tHEAD",
            "1577a50e37c6332ab267dea9d838dab8b8c07536\tHEAD",
        ],
    )?;
    require_text(
        "remote-refs.txt",
        &[
            "09e9e9ff7fa6b6c1d129d0c7a858979823e13ae8\trefs/heads/integration/vnext-main-20260811",
            "1577a50e37c6332ab267dea9d838dab8b8c07536\trefs/heads/main",
        ],
    )?;
    require_text(
        "codex-trust-bindings.txt",
        &[
            "main_integration_binding_count=1",
            "ui_main_binding_count=1",
            "explicit_old_workspace_binding_count=0",
            "/Users/qianqi/.openclaw/agents/main/agent/codex-home/config.toml",
            "/Users/qianqi/.openclaw/agents/hepta/agent/codex-home/config.toml",
            "/Users/qianqi/.openclaw/agents/hepta-backend/agent/codex-home/config.toml",
            "/Users/qianqi/.openclaw/agents/trnm/agent/codex-home/config.toml",
            "/Users/qianqi/.openclaw/agents/hepta-ui/agent/codex-home/config.toml",
        ],
    )?;
    require_text(
        "agent-path-instructions.txt",
        &[
            "/Volumes/T5/hepta-vnext/worktrees/main-integration",
            "/Volumes/T5/hepta-vnext/worktrees/ui-main",
            "/Users/qianqi/.openclaw/workspace/Hepta",
            "frozen migration/rollback inputs",
        ],
    )
}

fn kv_artifact(
    receipt: &VerifiedReceipt,
    profile: EvidenceProfileV3,
    layer: ManifestLayerIdV3,
) -> Result<BTreeMap<String, String>, AcceptanceError> {
    let (expected_layer, path, _) = profiles::authoritative_artifact(profile)
        .ok_or_else(|| invalid("authoritative artifact profile is absent"))?;
    if expected_layer != layer {
        return Err(invalid(
            "authoritative artifact layer differs from its profile",
        ));
    }
    parse_key_values(&receipt.layer(layer)?.bytes(path)?)
}

fn outer_kv(
    receipt: &VerifiedReceipt,
    profile: EvidenceProfileV3,
) -> Result<BTreeMap<String, String>, AcceptanceError> {
    let (path, _) = profiles::outer_verification_artifact(profile)
        .ok_or_else(|| invalid("outer verification profile is absent"))?;
    parse_key_values(&receipt.layer(ManifestLayerIdV3::Outer)?.bytes(path)?)
}

fn require_same_status(
    inner: &BTreeMap<String, String>,
    outer: &BTreeMap<String, String>,
) -> Result<String, AcceptanceError> {
    let status = kv_status(inner)?;
    if outer.get("status").map(String::as_str) != Some(status.as_str()) {
        return Err(invalid("outer verification does not relay inner status"));
    }
    Ok(status)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum StepPolicy {
    PrefixFirstFailure,
    WindowsFullCandidateRun,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ParsedSteps {
    ordered_return_codes: Vec<u32>,
    return_codes: BTreeMap<String, u32>,
}

impl ParsedSteps {
    fn count(&self) -> u64 {
        self.return_codes.len() as u64
    }
}

fn step_tsv(
    manifest: &VerifiedManifest,
    status: &str,
    expected: &[&str],
    policy: StepPolicy,
) -> Result<ParsedSteps, AcceptanceError> {
    let Some(_) = manifest.entry("steps.tsv") else {
        return if status == "BLOCKED_HARNESS" {
            Ok(ParsedSteps {
                ordered_return_codes: Vec::new(),
                return_codes: BTreeMap::new(),
            })
        } else {
            Err(invalid("executed gate lacks steps.tsv"))
        };
    };
    let bytes = manifest.bytes("steps.tsv")?;
    let parsed = parse_step_tsv(&bytes, status, expected, policy)?;
    if policy == StepPolicy::WindowsFullCandidateRun {
        for name in parsed.return_codes.keys() {
            let log_path = format!("{name}.log");
            if manifest.entry(&log_path).is_none() {
                return Err(invalid(
                    "Windows steps.tsv refers to an unsealed or absent step log",
                ));
            }
        }
    }
    Ok(parsed)
}

pub(super) fn parse_step_tsv(
    bytes: &[u8],
    status: &str,
    expected: &[&str],
    policy: StepPolicy,
) -> Result<ParsedSteps, AcceptanceError> {
    let text = std::str::from_utf8(bytes).map_err(|_| invalid("steps.tsv is not UTF-8"))?;
    if text.is_empty() && status == "BLOCKED_HARNESS" {
        return Ok(ParsedSteps {
            ordered_return_codes: Vec::new(),
            return_codes: BTreeMap::new(),
        });
    }
    if text.is_empty() || !text.ends_with('\n') {
        return Err(invalid("steps.tsv must be nonempty and newline terminated"));
    }
    let mut names = BTreeSet::new();
    let mut ordered_return_codes = Vec::new();
    let mut return_codes = BTreeMap::new();
    let mut domains = Vec::new();
    let mut previous_finished: Option<String> = None;
    let expected_field_count = match policy {
        StepPolicy::PrefixFirstFailure => 4,
        StepPolicy::WindowsFullCandidateRun => 6,
    };
    for (index, line) in text.lines().enumerate() {
        let fields = line.split('\t').collect::<Vec<_>>();
        let return_code = fields.get(1).and_then(|value| value.parse::<u32>().ok());
        if fields.len() != expected_field_count
            || fields[0].is_empty()
            || !names.insert(fields[0])
            || return_code.is_none()
            || return_code.is_some_and(|value| {
                value.to_string() != fields.get(1).copied().unwrap_or_default()
            })
            || expected.get(index).copied() != Some(fields[0])
        {
            return Err(invalid("steps.tsv differs from the fixed ordered roster"));
        }
        match policy {
            StepPolicy::PrefixFirstFailure => {
                if !valid_time_range(fields[2], fields[3])
                    || previous_finished
                        .as_deref()
                        .is_some_and(|finished| fields[2] < finished)
                {
                    return Err(invalid("steps.tsv contains an invalid UTC timestamp"));
                }
                previous_finished = Some(fields[3].to_string());
            }
            StepPolicy::WindowsFullCandidateRun => {
                if !matches!(fields[2], "candidate" | "harness")
                    || !valid_time_range(fields[3], fields[4])
                    || previous_finished
                        .as_deref()
                        .is_some_and(|finished| fields[3] < finished)
                    || fields[5] != format!("{}.log", fields[0])
                {
                    return Err(invalid(
                        "Windows steps.tsv contains invalid domain, time, or log evidence",
                    ));
                }
                previous_finished = Some(fields[4].to_string());
                domains.push(fields[2]);
            }
        }
        let return_code =
            return_code.ok_or_else(|| invalid("steps.tsv contains an invalid return code"))?;
        ordered_return_codes.push(return_code);
        return_codes.insert(fields[0].to_string(), return_code);
    }
    let expected_count = expected.len() as u64;
    let count = return_codes.len() as u64;
    let nonzero_indices = ordered_return_codes
        .iter()
        .enumerate()
        .filter_map(|(index, value)| (*value != 0).then_some(index))
        .collect::<Vec<_>>();
    let last_is_nonzero =
        nonzero_indices.last().copied() == count.checked_sub(1).map(|v| v as usize);
    let valid = match (policy, status) {
        (StepPolicy::PrefixFirstFailure, "PASS") => {
            count == expected_count && nonzero_indices.is_empty()
        }
        (StepPolicy::PrefixFirstFailure, "FAIL_CANDIDATE") => {
            count > 0 && count <= expected_count && nonzero_indices.len() == 1 && last_is_nonzero
        }
        (StepPolicy::PrefixFirstFailure, "BLOCKED_HARNESS") => {
            count <= expected_count
                && nonzero_indices.len() <= 1
                && (nonzero_indices.is_empty() || last_is_nonzero)
        }
        (StepPolicy::WindowsFullCandidateRun, "PASS") => {
            count == expected_count
                && nonzero_indices.is_empty()
                && domains.iter().all(|domain| *domain == "candidate")
        }
        (StepPolicy::WindowsFullCandidateRun, "FAIL_CANDIDATE") => {
            count == expected_count
                && !nonzero_indices.is_empty()
                && domains.iter().all(|domain| *domain == "candidate")
        }
        (StepPolicy::WindowsFullCandidateRun, "BLOCKED_HARNESS") => {
            count <= expected_count
                && ordered_return_codes
                    .iter()
                    .zip(&domains)
                    .all(|(return_code, domain)| *domain == "harness" || *return_code == 0)
                && domains.iter().enumerate().all(|(index, domain)| {
                    *domain == "candidate"
                        || (index + 1 == domains.len()
                            && *domain == "harness"
                            && ordered_return_codes[index] != 0)
                })
        }
        _ => false,
    };
    if !valid {
        return Err(invalid("steps.tsv contradicts the fixed status shape"));
    }
    Ok(ParsedSteps {
        ordered_return_codes,
        return_codes,
    })
}

fn valid_utc_timestamp(value: &str) -> bool {
    if value.len() != 20
        || value.as_bytes()[4] != b'-'
        || value.as_bytes()[7] != b'-'
        || value.as_bytes()[10] != b'T'
        || value.as_bytes()[13] != b':'
        || value.as_bytes()[16] != b':'
        || value.as_bytes()[19] != b'Z'
        || !value.bytes().enumerate().all(|(index, byte)| {
            matches!(index, 4 | 7 | 10 | 13 | 16 | 19) || byte.is_ascii_digit()
        })
    {
        return false;
    }
    let parse = |start, end| value[start..end].parse::<u32>().ok();
    let Some((year, month, day, hour, minute, second)) = parse(0, 4)
        .zip(parse(5, 7))
        .zip(parse(8, 10))
        .zip(parse(11, 13))
        .zip(parse(14, 16))
        .zip(parse(17, 19))
        .map(|(((((year, month), day), hour), minute), second)| {
            (year, month, day, hour, minute, second)
        })
    else {
        return false;
    };
    let leap_year =
        year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap_year => 29,
        2 => 28,
        _ => return false,
    };
    year != 0 && (1..=days_in_month).contains(&day) && hour < 24 && minute < 60 && second < 60
}

fn utc_timestamp_seconds(value: &str) -> Option<i64> {
    if !valid_utc_timestamp(value) {
        return None;
    }
    let year = value[0..4].parse::<i64>().ok()?;
    let month = value[5..7].parse::<i64>().ok()?;
    let day = value[8..10].parse::<i64>().ok()?;
    let hour = value[11..13].parse::<i64>().ok()?;
    let minute = value[14..16].parse::<i64>().ok()?;
    let second = value[17..19].parse::<i64>().ok()?;
    let adjusted_year = year - i64::from(month <= 2);
    let era = adjusted_year.div_euclid(400);
    let year_of_era = adjusted_year - era * 400;
    let adjusted_month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * adjusted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era * 146_097 + day_of_era;
    Some(days * 86_400 + hour * 3_600 + minute * 60 + second)
}

#[cfg(test)]
pub(super) fn valid_utc_timestamp_for_test(value: &str) -> bool {
    valid_utc_timestamp(value)
}

fn validate_windows_resource_floors(
    free_bytes: u64,
    minimum_free_bytes: u64,
    free_memory_bytes: u64,
    minimum_free_memory_bytes: u64,
) -> Result<(), AcceptanceError> {
    if minimum_free_bytes != 25_769_803_776
        || minimum_free_memory_bytes != 1_610_612_736
        || free_bytes < minimum_free_bytes
        || free_memory_bytes < minimum_free_memory_bytes
    {
        return Err(invalid("Windows resource floor was not satisfied"));
    }
    Ok(())
}

#[cfg(test)]
pub(super) fn validate_windows_resource_floors_for_test(
    free_bytes: u64,
    minimum_free_bytes: u64,
    free_memory_bytes: u64,
    minimum_free_memory_bytes: u64,
) -> Result<(), AcceptanceError> {
    validate_windows_resource_floors(
        free_bytes,
        minimum_free_bytes,
        free_memory_bytes,
        minimum_free_memory_bytes,
    )
}

fn validate_windows_nonce_paths(
    nonce: &str,
    run_root: &str,
    source_root: &str,
    vendor_root: &str,
    target_root: &str,
) -> Result<(), AcceptanceError> {
    let expected_run = format!(r"C:\q\52ec-{nonce}");
    for (actual, expected) in [
        (run_root, expected_run.clone()),
        (source_root, format!(r"{expected_run}\s")),
        (vendor_root, format!(r"{expected_run}\vendor")),
        (target_root, format!(r"{expected_run}\target")),
    ] {
        if actual != expected {
            return Err(invalid(
                "Windows native path differs from the launcher nonce contract",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
pub(super) fn validate_windows_nonce_paths_for_test(
    nonce: &str,
    run_root: &str,
    source_root: &str,
    vendor_root: &str,
    target_root: &str,
) -> Result<(), AcceptanceError> {
    validate_windows_nonce_paths(nonce, run_root, source_root, vendor_root, target_root)
}

fn valid_time_range(started: &str, finished: &str) -> bool {
    valid_utc_timestamp(started) && valid_utc_timestamp(finished) && started <= finished
}

pub(super) fn require_windows_step_results(
    value: Option<&Value>,
    steps: &ParsedSteps,
    status: &str,
) -> Result<(), AcceptanceError> {
    let values = value
        .and_then(Value::as_object)
        .ok_or_else(|| invalid("Windows step_results must be an object"))?;
    if status == "BLOCKED_HARNESS" {
        if !values.is_empty() {
            return Err(invalid(
                "Windows blocked result must not claim completed step results",
            ));
        }
        return Ok(());
    }
    if values.len() != steps.return_codes.len() {
        return Err(invalid(
            "Windows step_results differs from sealed ordered steps",
        ));
    }
    for (name, return_code) in &steps.return_codes {
        if values.get(name).and_then(Value::as_u64) != Some(u64::from(*return_code)) {
            return Err(invalid(
                "Windows step_results return code differs from steps.tsv",
            ));
        }
    }
    Ok(())
}

fn require_candidate_kv(
    values: &BTreeMap<String, String>,
    candidate: &CandidateBindingV3,
    head_key: &str,
    cutoff_key: &str,
) -> Result<(), AcceptanceError> {
    require_kv(values, head_key, &candidate.head)?;
    require_kv(values, "candidate_tree", &candidate.tree)?;
    require_kv(values, "candidate_parent", &candidate.parents[0])?;
    require_kv(values, cutoff_key, &candidate.upstream_cutoff)
}

#[cfg(test)]
const RESERVED_CORE_FIELDS: [&str; 43] = [
    "schema",
    "status",
    "verdict",
    "qualification",
    "pass",
    "ready",
    "ready_for_challenge",
    "candidate_pass",
    "candidate_fail",
    "candidate_failure",
    "harness_fail",
    "harness_blocked",
    "harness_failure",
    "interrupted",
    "candidate_execution_started",
    "candidate_execution_completed",
    "candidate_executed",
    "ordered_step_count",
    "executed_steps",
    "step_results",
    "harness_preflight_pass",
    "postflight_verified",
    "source_postflight_verified",
    "source_identity",
    "worktree_clean",
    "source_worktree_clean",
    "production_changed",
    "production_cutover",
    "production_state_snapshot",
    "production_canary",
    "refs_changed",
    "default_branch_changed",
    "default_main_changed",
    "automatic_transition",
    "candidate_head",
    "candidate_commit",
    "candidate_tree",
    "candidate_parent",
    "integration_merge",
    "upstream_cutoff",
    "frozen_upstream_cutoff",
    "candidate_changed",
    "qualification_invalidated",
];

#[cfg(test)]
const RESERVED_AUTHORITY_FIELDS: [&str; 10] = [
    "operator_acceptance",
    "candidate_operator_acceptance",
    "cross_platform_qualification",
    "promotion",
    "promotion_authority",
    "enforce",
    "outbound",
    "retirement",
    "data_deleted",
    "default_branch_switch",
];

#[cfg(test)]
pub(super) fn reject_reserved_kv_fields(
    values: &BTreeMap<String, String>,
    allowed: &[&str],
) -> Result<(), AcceptanceError> {
    reject_reserved_field_names(values.keys().map(String::as_str), allowed)
}

#[cfg(test)]
fn reject_reserved_field_names<'a>(
    names: impl Iterator<Item = &'a str>,
    allowed: &[&str],
) -> Result<(), AcceptanceError> {
    for name in names {
        if (RESERVED_CORE_FIELDS.contains(&name) || RESERVED_AUTHORITY_FIELDS.contains(&name))
            && !allowed.contains(&name)
        {
            return Err(invalid(format!(
                "evidence profile contains an uncompiled reserved field: {name}"
            )));
        }
    }
    Ok(())
}

fn require_json_candidate(
    values: &Map<String, Value>,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    require_json_string(values, "candidate_head", &candidate.head)?;
    require_json_string(values, "candidate_tree", &candidate.tree)?;
    require_json_string(values, "candidate_parent", &candidate.parents[0])?;
    require_json_string(values, "upstream_cutoff", &candidate.upstream_cutoff)
}

fn require_kv_schema(
    values: &BTreeMap<String, String>,
    schema: &str,
) -> Result<(), AcceptanceError> {
    require_kv(values, "schema", schema)
}

fn require_exact_kv_fields(
    values: &BTreeMap<String, String>,
    expected: &[&str],
) -> Result<(), AcceptanceError> {
    let actual = values.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(invalid(
            "key/value artifact differs from its exact field set",
        ));
    }
    Ok(())
}

fn require_exact_json_fields(
    values: &Map<String, Value>,
    expected: &[&str],
) -> Result<(), AcceptanceError> {
    let actual = values.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(invalid("JSON artifact differs from its exact field set"));
    }
    Ok(())
}

fn require_kv(
    values: &BTreeMap<String, String>,
    key: &str,
    expected: &str,
) -> Result<(), AcceptanceError> {
    if values.get(key).map(String::as_str) != Some(expected) {
        return Err(invalid(format!("fixed key/value field differs: {key}")));
    }
    Ok(())
}

fn require_kv_bool(
    values: &BTreeMap<String, String>,
    key: &str,
    expected: bool,
) -> Result<(), AcceptanceError> {
    if kv_bool(values, key)? != expected {
        return Err(invalid(format!("fixed boolean field differs: {key}")));
    }
    Ok(())
}

fn kv_bool(values: &BTreeMap<String, String>, key: &str) -> Result<bool, AcceptanceError> {
    match values.get(key).map(String::as_str) {
        Some("true") => Ok(true),
        Some("false") => Ok(false),
        _ => Err(invalid(format!("field is not an exact boolean: {key}"))),
    }
}

fn require_kv_u64(
    values: &BTreeMap<String, String>,
    key: &str,
    expected: u64,
) -> Result<(), AcceptanceError> {
    let raw = values
        .get(key)
        .ok_or_else(|| invalid(format!("field is absent: {key}")))?;
    let value = raw
        .parse::<u64>()
        .map_err(|_| invalid(format!("field is not an exact u64: {key}")))?;
    if value != expected || value.to_string() != *raw {
        return Err(invalid(format!("fixed u64 field differs: {key}")));
    }
    Ok(())
}

fn kv_status(values: &BTreeMap<String, String>) -> Result<String, AcceptanceError> {
    let raw = values
        .get("status")
        .or_else(|| values.get("verdict"))
        .ok_or_else(|| invalid("status is absent"))?;
    if values.get("verdict").is_some_and(|value| value != raw)
        || !matches!(raw.as_str(), "PASS" | "BLOCKED_HARNESS" | "FAIL_CANDIDATE")
    {
        return Err(invalid("status is not an exact supported literal"));
    }
    Ok(raw.clone())
}

fn require_json_string(
    values: &Map<String, Value>,
    key: &str,
    expected: &str,
) -> Result<(), AcceptanceError> {
    if json_string(values, key)? != expected {
        return Err(invalid(format!("fixed JSON string differs: {key}")));
    }
    Ok(())
}

fn json_string<'a>(values: &'a Map<String, Value>, key: &str) -> Result<&'a str, AcceptanceError> {
    values
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| invalid(format!("JSON field is not a string: {key}")))
}

fn require_json_bool(
    values: &Map<String, Value>,
    key: &str,
    expected: bool,
) -> Result<(), AcceptanceError> {
    if json_bool(values, key)? != expected {
        return Err(invalid(format!("fixed JSON boolean differs: {key}")));
    }
    Ok(())
}

fn json_bool(values: &Map<String, Value>, key: &str) -> Result<bool, AcceptanceError> {
    values
        .get(key)
        .and_then(Value::as_bool)
        .ok_or_else(|| invalid(format!("JSON field is not a boolean: {key}")))
}

fn json_u64(values: &Map<String, Value>, key: &str) -> Result<u64, AcceptanceError> {
    values
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| invalid(format!("JSON field is not an unsigned integer: {key}")))
}

fn decision(
    gates: &[PlatformGateBindingV3],
    prerequisites: &[PrerequisiteReceiptBindingV3],
) -> QualificationDecisionV3 {
    let mut blockers = Vec::new();
    for gate in gates {
        if !gate.observed.pass {
            blockers.push(format!("gate:{}:{}", gate.gate, gate.observed.status));
        }
    }
    for prerequisite in prerequisites {
        if !prerequisite.observed.pass {
            blockers.push(format!(
                "prerequisite:{}:{}",
                prerequisite.id, prerequisite.observed.status
            ));
        }
    }
    QualificationDecisionV3 {
        blockers: blockers.clone(),
        complete_gate_count: gates.len(),
        pass_gate_count: gates.iter().filter(|gate| gate.observed.pass).count(),
        prerequisite_pass_count: prerequisites
            .iter()
            .filter(|receipt| receipt.observed.pass)
            .count(),
        verdict: if blockers.is_empty() {
            "PASS"
        } else {
            "BLOCKED"
        }
        .to_string(),
    }
}

fn validate_disjoint_receipt_roots(roots: &[String]) -> Result<(), AcceptanceError> {
    for (index, left) in roots.iter().enumerate() {
        let left = Path::new(left);
        for right in roots.iter().skip(index + 1).map(Path::new) {
            if left == right || left.starts_with(right) || right.starts_with(left) {
                return Err(invalid("receipt roots must be pairwise disjoint"));
            }
        }
    }
    Ok(())
}

pub(super) fn parse_key_values(bytes: &[u8]) -> Result<BTreeMap<String, String>, AcceptanceError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| invalid("key/value evidence artifact is not UTF-8"))?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(invalid(
            "key/value evidence artifact must be nonempty and newline terminated",
        ));
    }
    let mut values = BTreeMap::new();
    for line in text.lines() {
        if line.contains('\r') {
            return Err(invalid(
                "key/value evidence artifact contains a carriage return",
            ));
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| invalid("key/value evidence artifact contains a malformed line"))?;
        validate_identifier(key, "key/value field")?;
        if value.is_empty() || values.insert(key.to_string(), value.to_string()).is_some() {
            return Err(invalid(
                "key/value evidence artifact contains an empty or duplicate field",
            ));
        }
    }
    Ok(values)
}

fn validate_identifier(value: &str, label: &str) -> Result<(), AcceptanceError> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(invalid(format!("{label} is malformed")));
    }
    Ok(())
}

pub(super) fn validate_output_relative_name(value: &str) -> Result<(), AcceptanceError> {
    let path = Path::new(value);
    if value.is_empty()
        || value.starts_with('.')
        || path.parent() != Some(Path::new(""))
        || path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(invalid("output name is not a safe published relative path"));
    }
    Ok(())
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Component;
use std::path::Path;

use serde_json::Value;

use crate::AcceptanceError;
use crate::durable::canonical_json;
use crate::manifest_inventory::VerifiedManifest;
use crate::manifest_inventory::digest_shape;
use crate::manifest_inventory::validate_relative_path;
use crate::model::AuthorityBoundary;

use super::model::AggregateBuildSpecV3;
use super::model::AggregateQualificationPacketV3;
use super::model::CandidateBindingV3;
use super::model::CandidateBundleBindingV3;
use super::model::EvidenceArtifactBindingV3;
use super::model::EvidenceArtifactFormatV3;
use super::model::ManifestLayerBindingV3;
use super::model::ModeManifestBindingV3;
use super::model::ModeManifestFormatV3;
use super::model::ObservedGateV3;
use super::model::ObservedPrerequisiteV3;
use super::model::PlatformGateBindingV3;
use super::model::PlatformPolicyV3;
use super::model::PrerequisiteReceiptBindingV3;
use super::model::QualificationAssessmentV3;
use super::model::QualificationDecisionV3;
use super::model::ReceiptEvidenceBindingV3;
use super::model::SemanticClaimBindingV3;
use super::model::SemanticClaimV3;

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
    claims: BTreeMap<SemanticClaimV3, Value>,
}

enum ParsedArtifact {
    Json(Value),
    KeyValues(BTreeMap<String, String>),
}

impl ParsedArtifact {
    fn select(&self, selector: &str) -> Result<Value, AcceptanceError> {
        match self {
            Self::Json(value) => value
                .pointer(selector)
                .cloned()
                .ok_or_else(|| invalid(format!("JSON selector is absent: {selector}"))),
            Self::KeyValues(values) => values
                .get(selector)
                .cloned()
                .map(Value::String)
                .ok_or_else(|| invalid(format!("key/value selector is absent: {selector}"))),
        }
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
    let mut platform_receipts = Vec::new();
    for (input, expected_gate) in spec.platform_gates.iter().zip(GATES) {
        if input.gate != expected_gate || !input.required {
            return Err(invalid(
                "platform gates must be required and canonically ordered",
            ));
        }
        let verified = validate_receipt(&input.receipt, policy.receipts_parent, None)?;
        let observed = observe_gate(expected_gate, &verified.claims, policy.expected_candidate)?;
        receipt_roots.push(input.receipt.receipt_root.clone());
        platform_receipts.push(PlatformGateBindingV3 {
            gate: input.gate.clone(),
            observed,
            receipt: input.receipt.clone(),
            required: true,
        });
    }

    let mut prerequisite_receipts = Vec::new();
    for (input, expected_id) in spec.prerequisite_receipts.iter().zip(PREREQUISITES) {
        if input.id != expected_id || !input.required {
            return Err(invalid(
                "prerequisites must be required and canonically ordered",
            ));
        }
        let required_file = (expected_id == PORTABLE_INPUTS_ID).then_some(&spec.candidate.bundle);
        let verified = validate_receipt(&input.receipt, policy.receipts_parent, required_file)?;
        let observed =
            observe_prerequisite(expected_id, &verified.claims, policy.expected_candidate)?;
        receipt_roots.push(input.receipt.receipt_root.clone());
        prerequisite_receipts.push(PrerequisiteReceiptBindingV3 {
            id: input.id.clone(),
            observed,
            receipt: input.receipt.clone(),
            required: true,
        });
    }
    validate_disjoint_receipt_roots(&receipt_roots)?;

    let decision = decision(&platform_receipts, &prerequisite_receipts);
    Ok(AggregateQualificationPacketV3 {
        automatic_transition: false,
        authority: AuthorityBoundary::all_closed(),
        candidate: policy.expected_candidate.clone(),
        decision,
        platform_policy: exact_platform_policy(),
        platform_receipts,
        prerequisite_receipts,
        schema: PACKET_SCHEMA.to_string(),
        schema_version: 3,
    })
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
    receipts_parent: &Path,
    required_file: Option<&CandidateBundleBindingV3>,
) -> Result<VerifiedReceipt, AcceptanceError> {
    if binding.expected_schema.is_empty()
        || binding.manifest_layers.is_empty()
        || binding.artifacts.is_empty()
        || binding.semantic_claims.is_empty()
    {
        return Err(invalid("receipt binding is incomplete"));
    }
    let receipt_root = Path::new(&binding.receipt_root);
    if !receipt_root.starts_with(receipts_parent) || receipt_root == receipts_parent {
        return Err(invalid(
            "receipt root must be a strict child of the frozen receipts parent",
        ));
    }

    validate_layer_order(&binding.manifest_layers)?;
    let outer_binding = &binding.manifest_layers[0];
    let outer_manifest = load_layer(receipt_root, outer_binding)?;
    for path in outer_manifest.entry_paths() {
        if Path::new(path).file_name().and_then(|value| value.to_str()) == Some("SUPERSEDED.txt") {
            return Err(invalid("receipt contains a SUPERSEDED.txt marker"));
        }
    }

    for layer in binding.manifest_layers.iter().skip(1) {
        let nested_manifest = load_layer(receipt_root, layer)?;
        let nested_manifest_from_outer = Path::new(&layer.root_relative_path)
            .join(&layer.manifest_relative_path)
            .to_str()
            .ok_or_else(|| invalid("nested manifest path is not UTF-8"))?
            .to_string();
        outer_manifest.require_hash(&nested_manifest_from_outer, &layer.manifest_sha256)?;
        if nested_manifest.entry_count() != layer.manifest_entry_count {
            return Err(invalid("nested manifest entry count changed"));
        }
    }
    if outer_binding.manifest_relative_path == "ATTEMPT.sha256"
        && !binding.manifest_layers.iter().skip(1).any(|layer| {
            layer.root_relative_path == "receipt" && layer.manifest_relative_path == "SHA256SUMS"
        })
    {
        return Err(invalid(
            "ATTEMPT.sha256 receipts require the nested receipt/SHA256SUMS layer",
        ));
    }

    if let Some(required) = required_file {
        let bundle = outer_manifest
            .entry(&required.relative_path)
            .ok_or_else(|| invalid("candidate bundle is absent from portable inputs"))?;
        if bundle.sha256 != required.sha256 || bundle.size_bytes != required.size_bytes {
            return Err(invalid(
                "candidate bundle differs from the exact 52ec materialization pin",
            ));
        }
    }

    validate_mode_manifest(binding, &outer_manifest)?;

    let parsed = parse_artifacts(binding, &outer_manifest)?;
    let claims = bind_semantic_claims(binding, &parsed)?;
    require_claim_string(&claims, SemanticClaimV3::Schema, &binding.expected_schema)?;
    Ok(VerifiedReceipt { claims })
}

fn validate_mode_manifest(
    binding: &ReceiptEvidenceBindingV3,
    outer: &VerifiedManifest,
) -> Result<(), AcceptanceError> {
    let manifest_mode_paths = outer
        .entry_paths()
        .filter(|path| {
            Path::new(path).file_name().and_then(|name| name.to_str()) == Some("MODES.tsv")
        })
        .collect::<Vec<_>>();
    let Some(mode_binding) = &binding.mode_manifest else {
        if manifest_mode_paths.is_empty() {
            return Ok(());
        }
        return Err(invalid(
            "receipt seals a MODES.tsv artifact but omits its semantic binding",
        ));
    };
    if !digest_shape(&mode_binding.sha256) {
        return Err(invalid("mode manifest digest is malformed"));
    }
    validate_relative_path(&mode_binding.relative_path)?;
    outer.require_hash(&mode_binding.relative_path, &mode_binding.sha256)?;
    let bytes = outer.bytes(&mode_binding.relative_path)?;
    verify_mode_lines(&outer.root, mode_binding, &bytes)
}

fn verify_mode_lines(
    receipt_root: &Path,
    binding: &ModeManifestBindingV3,
    bytes: &[u8],
) -> Result<(), AcceptanceError> {
    let text = std::str::from_utf8(bytes).map_err(|_| invalid("mode manifest is not UTF-8"))?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(invalid(
            "mode manifest must be nonempty and newline terminated",
        ));
    }
    let mut previous: Option<String> = None;
    for line in text.lines() {
        let (kind, mode, size, raw_path) = match binding.format {
            ModeManifestFormatV3::PosixModePathTsvV1 => {
                let fields = line.split('\t').collect::<Vec<_>>();
                if fields.len() != 2 {
                    return Err(invalid("mode/path manifest row is malformed"));
                }
                (None, fields[0], None, fields[1])
            }
            ModeManifestFormatV3::PosixModeSizePathTsvV1 => {
                let fields = line.split('\t').collect::<Vec<_>>();
                if fields.len() != 3 {
                    return Err(invalid("mode/size/path manifest row is malformed"));
                }
                (None, fields[0], Some(fields[1]), fields[2])
            }
            ModeManifestFormatV3::TypedLiteralBackslashTModePathV1 => {
                let fields = line.split("\\t").collect::<Vec<_>>();
                if fields.len() != 3 {
                    return Err(invalid("typed mode/path manifest row is malformed"));
                }
                (Some(fields[0]), fields[1], None, fields[2])
            }
        };
        let relative = raw_path.strip_prefix("./").unwrap_or(raw_path);
        validate_relative_path(relative)?;
        if previous.as_deref().is_some_and(|value| value >= relative) {
            return Err(invalid(
                "mode manifest paths must be unique and strictly sorted",
            ));
        }
        previous = Some(relative.to_string());
        let expected_mode = u32::from_str_radix(mode, 8)
            .ok()
            .filter(|mode| *mode <= 0o777)
            .ok_or_else(|| invalid("mode manifest contains an invalid POSIX mode"))?;
        let metadata = std::fs::symlink_metadata(receipt_root.join(relative))?;
        if metadata.file_type().is_symlink() {
            return Err(invalid("mode manifest path is a symlink"));
        }
        if let Some(kind) = kind {
            let correct_kind = match kind {
                "Regular File" => metadata.is_file(),
                "Directory" => metadata.is_dir(),
                _ => false,
            };
            if !correct_kind {
                return Err(invalid("mode manifest type differs from the receipt"));
            }
        }
        if let Some(size) = size {
            let expected_size = size
                .parse::<u64>()
                .map_err(|_| invalid("mode manifest size is malformed"))?;
            if metadata.len() != expected_size {
                return Err(invalid("mode manifest size differs from the receipt"));
            }
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if metadata.mode() & 0o777 != expected_mode {
                return Err(invalid("mode manifest mode differs from the receipt"));
            }
        }
    }
    Ok(())
}

fn validate_layer_order(layers: &[ManifestLayerBindingV3]) -> Result<(), AcceptanceError> {
    let mut previous: Option<&str> = None;
    for (index, layer) in layers.iter().enumerate() {
        if layer.manifest_entry_count == 0
            || !digest_shape(&layer.manifest_sha256)
            || layer.manifest_relative_path.is_empty()
        {
            return Err(invalid("manifest layer has an invalid source pin"));
        }
        validate_relative_path(&layer.manifest_relative_path)?;
        if index == 0 {
            if layer.root_relative_path != "." {
                return Err(invalid("first manifest layer must bind the receipt root"));
            }
        } else {
            validate_relative_path(&layer.root_relative_path)?;
            if previous.is_some_and(|value| value >= layer.root_relative_path.as_str()) {
                return Err(invalid(
                    "nested manifest roots must be unique and strictly sorted",
                ));
            }
        }
        previous = Some(&layer.root_relative_path);
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

fn parse_artifacts(
    binding: &ReceiptEvidenceBindingV3,
    outer: &VerifiedManifest,
) -> Result<BTreeMap<String, ParsedArtifact>, AcceptanceError> {
    let mut parsed = BTreeMap::new();
    let mut previous_id: Option<&str> = None;
    for artifact in &binding.artifacts {
        validate_identifier(&artifact.id, "artifact id")?;
        validate_relative_path(&artifact.relative_path)?;
        if !digest_shape(&artifact.sha256) {
            return Err(invalid("artifact digest is malformed"));
        }
        if previous_id.is_some_and(|value| value >= artifact.id.as_str()) {
            return Err(invalid("artifact ids must be unique and strictly sorted"));
        }
        previous_id = Some(&artifact.id);
        outer.require_hash(&artifact.relative_path, &artifact.sha256)?;
        let bytes = outer.bytes(&artifact.relative_path)?;
        let value = parse_artifact(artifact, &bytes)?;
        validate_assertions(artifact, &value)?;
        parsed.insert(artifact.id.clone(), value);
    }
    Ok(parsed)
}

fn parse_artifact(
    artifact: &EvidenceArtifactBindingV3,
    bytes: &[u8],
) -> Result<ParsedArtifact, AcceptanceError> {
    match artifact.format {
        EvidenceArtifactFormatV3::CanonicalJsonV1 => {
            let value: Value = serde_json::from_slice(bytes)
                .map_err(|error| invalid(format!("invalid canonical JSON artifact: {error}")))?;
            if canonical_json(&value)? != bytes {
                return Err(invalid("JSON artifact is not canonical"));
            }
            Ok(ParsedArtifact::Json(value))
        }
        EvidenceArtifactFormatV3::JsonV1 => {
            let value: Value = serde_json::from_slice(bytes)
                .map_err(|error| invalid(format!("invalid JSON artifact: {error}")))?;
            Ok(ParsedArtifact::Json(value))
        }
        EvidenceArtifactFormatV3::KeyValueLinesV1 => {
            Ok(ParsedArtifact::KeyValues(parse_key_values(bytes)?))
        }
    }
}

fn validate_assertions(
    artifact: &EvidenceArtifactBindingV3,
    parsed: &ParsedArtifact,
) -> Result<(), AcceptanceError> {
    if artifact.assertions.is_empty() {
        return Err(invalid("evidence artifact has no assertions"));
    }
    let mut previous: Option<&str> = None;
    for assertion in &artifact.assertions {
        if assertion.selector.is_empty()
            || previous.is_some_and(|value| value >= assertion.selector.as_str())
        {
            return Err(invalid(
                "artifact selectors must be nonempty, unique, and strictly sorted",
            ));
        }
        previous = Some(&assertion.selector);
        if parsed.select(&assertion.selector)? != assertion.expected {
            return Err(invalid(format!(
                "artifact assertion differs: {}:{}",
                artifact.id, assertion.selector
            )));
        }
    }
    Ok(())
}

fn bind_semantic_claims(
    binding: &ReceiptEvidenceBindingV3,
    artifacts: &BTreeMap<String, ParsedArtifact>,
) -> Result<BTreeMap<SemanticClaimV3, Value>, AcceptanceError> {
    let artifact_bindings = binding
        .artifacts
        .iter()
        .map(|artifact| (artifact.id.as_str(), artifact))
        .collect::<BTreeMap<_, _>>();
    let mut claims = BTreeMap::new();
    let mut previous: Option<SemanticClaimV3> = None;
    for binding in &binding.semantic_claims {
        if previous.is_some_and(|value| value >= binding.claim) {
            return Err(invalid(
                "semantic claims must be unique and strictly sorted",
            ));
        }
        previous = Some(binding.claim);
        validate_semantic_binding(binding, &artifact_bindings)?;
        let value = artifacts
            .get(&binding.artifact_id)
            .ok_or_else(|| invalid("semantic claim references an unknown artifact"))?
            .select(&binding.selector)?;
        claims.insert(binding.claim, value);
    }
    Ok(claims)
}

fn validate_semantic_binding(
    binding: &SemanticClaimBindingV3,
    artifacts: &BTreeMap<&str, &EvidenceArtifactBindingV3>,
) -> Result<(), AcceptanceError> {
    let artifact = artifacts
        .get(binding.artifact_id.as_str())
        .ok_or_else(|| invalid("semantic claim references an unknown artifact binding"))?;
    if !artifact
        .assertions
        .iter()
        .any(|assertion| assertion.selector == binding.selector)
    {
        return Err(invalid(
            "semantic claim must reference an explicitly asserted selector",
        ));
    }
    Ok(())
}

fn observe_gate(
    gate: &str,
    claims: &BTreeMap<SemanticClaimV3, Value>,
    candidate: &CandidateBindingV3,
) -> Result<ObservedGateV3, AcceptanceError> {
    require_claim_set(claims, &required_gate_claims())?;
    validate_candidate_claims(claims, true, candidate)?;
    let observed = ObservedGateV3 {
        candidate_executed: require_claim_bool(claims, SemanticClaimV3::CandidateExecuted)?,
        candidate_failure: require_claim_bool(claims, SemanticClaimV3::CandidateFailure)?,
        executed_steps: require_claim_u64(claims, SemanticClaimV3::ExecutedSteps)?,
        harness_failure: require_claim_bool(claims, SemanticClaimV3::HarnessFailure)?,
        pass: require_claim_bool(claims, SemanticClaimV3::Pass)?,
        production_changed: require_claim_bool(claims, SemanticClaimV3::ProductionChanged)?,
        qualification: require_claim_bool(claims, SemanticClaimV3::Qualification)?,
        refs_changed: require_claim_bool(claims, SemanticClaimV3::RefsChanged)?,
        status: require_claim_status(claims)?,
    };
    if observed.refs_changed || observed.production_changed {
        return Err(invalid(format!(
            "qualification receipt changed refs or production: {gate}"
        )));
    }
    validate_gate_shape(gate, &observed)?;
    Ok(observed)
}

fn observe_prerequisite(
    id: &str,
    claims: &BTreeMap<SemanticClaimV3, Value>,
    candidate: &CandidateBindingV3,
) -> Result<ObservedPrerequisiteV3, AcceptanceError> {
    let required = required_prerequisite_claims(id)?;
    require_claim_set(claims, &required)?;
    validate_candidate_claims(claims, id != "canonical-path-trust", candidate)?;
    let observed = ObservedPrerequisiteV3 {
        pass: require_claim_bool(claims, SemanticClaimV3::Pass)?,
        production_changed: require_claim_bool(claims, SemanticClaimV3::ProductionChanged)?,
        refs_changed: require_claim_bool(claims, SemanticClaimV3::RefsChanged)?,
        status: require_claim_status(claims)?,
    };
    if observed.status != "PASS"
        || !observed.pass
        || observed.refs_changed
        || observed.production_changed
    {
        return Err(invalid(format!(
            "required prerequisite is not a non-mutating PASS: {id}"
        )));
    }
    Ok(observed)
}

fn validate_gate_shape(gate: &str, observed: &ObservedGateV3) -> Result<(), AcceptanceError> {
    match observed.status.as_str() {
        "PASS"
            if observed.pass
                && observed.qualification
                && observed.candidate_executed
                && !observed.candidate_failure
                && !observed.harness_failure
                && observed.executed_steps > 0 => {}
        "BLOCKED_EXTERNAL"
            if gate == "github-actions"
                && !observed.pass
                && !observed.qualification
                && !observed.candidate_executed
                && !observed.candidate_failure
                && !observed.harness_failure
                && observed.executed_steps == 0 => {}
        "BLOCKED_HARNESS"
            if !observed.pass
                && !observed.qualification
                && !observed.candidate_failure
                && observed.harness_failure => {}
        "FAIL_CANDIDATE"
            if !observed.pass
                && !observed.qualification
                && observed.candidate_executed
                && observed.candidate_failure
                && !observed.harness_failure
                && observed.executed_steps > 0 => {}
        _ => {
            return Err(invalid(format!(
                "platform gate has an invalid fail-closed status shape: {gate}"
            )));
        }
    }
    Ok(())
}

fn validate_candidate_claims(
    claims: &BTreeMap<SemanticClaimV3, Value>,
    require_cutoff: bool,
    candidate: &CandidateBindingV3,
) -> Result<(), AcceptanceError> {
    require_claim_string(claims, SemanticClaimV3::CandidateHead, &candidate.head)?;
    require_claim_string(claims, SemanticClaimV3::CandidateTree, &candidate.tree)?;
    require_claim_string(
        claims,
        SemanticClaimV3::CandidateParent,
        &candidate.parents[0],
    )?;
    if require_cutoff {
        require_claim_string(
            claims,
            SemanticClaimV3::UpstreamCutoff,
            &candidate.upstream_cutoff,
        )?;
    }
    Ok(())
}

fn required_gate_claims() -> BTreeSet<SemanticClaimV3> {
    [
        SemanticClaimV3::CandidateExecuted,
        SemanticClaimV3::CandidateFailure,
        SemanticClaimV3::CandidateHead,
        SemanticClaimV3::CandidateParent,
        SemanticClaimV3::CandidateTree,
        SemanticClaimV3::ExecutedSteps,
        SemanticClaimV3::HarnessFailure,
        SemanticClaimV3::Pass,
        SemanticClaimV3::ProductionChanged,
        SemanticClaimV3::Qualification,
        SemanticClaimV3::RefsChanged,
        SemanticClaimV3::Schema,
        SemanticClaimV3::Status,
        SemanticClaimV3::UpstreamCutoff,
    ]
    .into_iter()
    .collect()
}

fn required_prerequisite_claims(id: &str) -> Result<BTreeSet<SemanticClaimV3>, AcceptanceError> {
    let mut required = [
        SemanticClaimV3::CandidateHead,
        SemanticClaimV3::CandidateParent,
        SemanticClaimV3::CandidateTree,
        SemanticClaimV3::Pass,
        SemanticClaimV3::ProductionChanged,
        SemanticClaimV3::RefsChanged,
        SemanticClaimV3::Schema,
        SemanticClaimV3::Status,
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    match id {
        PORTABLE_INPUTS_ID | "upstream-cutoff-observation" => {
            required.insert(SemanticClaimV3::UpstreamCutoff);
        }
        "canonical-path-trust" => {}
        _ => return Err(invalid("unknown prerequisite id")),
    }
    Ok(required)
}

fn require_claim_set(
    claims: &BTreeMap<SemanticClaimV3, Value>,
    required: &BTreeSet<SemanticClaimV3>,
) -> Result<(), AcceptanceError> {
    let present = claims.keys().copied().collect::<BTreeSet<_>>();
    if !required.is_subset(&present) {
        return Err(invalid("receipt omits a required semantic claim"));
    }
    Ok(())
}

fn require_claim_string(
    claims: &BTreeMap<SemanticClaimV3, Value>,
    claim: SemanticClaimV3,
    expected: &str,
) -> Result<(), AcceptanceError> {
    if claims.get(&claim).and_then(Value::as_str) != Some(expected) {
        return Err(invalid(format!("semantic claim differs: {claim:?}")));
    }
    Ok(())
}

fn require_claim_bool(
    claims: &BTreeMap<SemanticClaimV3, Value>,
    claim: SemanticClaimV3,
) -> Result<bool, AcceptanceError> {
    let value = claims
        .get(&claim)
        .ok_or_else(|| invalid(format!("semantic claim is absent: {claim:?}")))?;
    boolish(value).ok_or_else(|| invalid(format!("semantic claim is not boolean: {claim:?}")))
}

fn require_claim_u64(
    claims: &BTreeMap<SemanticClaimV3, Value>,
    claim: SemanticClaimV3,
) -> Result<u64, AcceptanceError> {
    let value = claims
        .get(&claim)
        .ok_or_else(|| invalid(format!("semantic claim is absent: {claim:?}")))?;
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|raw| raw.parse().ok()))
        .ok_or_else(|| invalid(format!("semantic claim is not u64: {claim:?}")))
}

fn require_claim_status(
    claims: &BTreeMap<SemanticClaimV3, Value>,
) -> Result<String, AcceptanceError> {
    let raw = claims
        .get(&SemanticClaimV3::Status)
        .and_then(Value::as_str)
        .ok_or_else(|| invalid("status semantic claim is not a string"))?;
    match raw.to_ascii_uppercase().replace('-', "_").as_str() {
        "PASS" => Ok("PASS".to_string()),
        "BLOCKED_EXTERNAL" => Ok("BLOCKED_EXTERNAL".to_string()),
        "BLOCKED_HARNESS" => Ok("BLOCKED_HARNESS".to_string()),
        "FAIL_CANDIDATE" => Ok("FAIL_CANDIDATE".to_string()),
        _ => Err(invalid("status semantic claim is not recognized")),
    }
}

fn boolish(value: &Value) -> Option<bool> {
    if let Some(value) = value.as_bool() {
        return Some(value);
    }
    if let Some(value) = value.as_u64() {
        return match value {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        };
    }
    match value.as_str()?.to_ascii_lowercase().as_str() {
        "true" | "yes" | "pass" | "ready" | "1" => Some(true),
        "false" | "no" | "fail" | "blocked" | "not_run" | "0" => Some(false),
        _ => None,
    }
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

fn parse_key_values(bytes: &[u8]) -> Result<BTreeMap<String, String>, AcceptanceError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| invalid("key/value evidence artifact is not UTF-8"))?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(invalid(
            "key/value evidence artifact must be nonempty and newline terminated",
        ));
    }
    let mut values = BTreeMap::new();
    for line in text.lines() {
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
        || path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(invalid("output name is not a safe relative path"));
    }
    Ok(())
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}

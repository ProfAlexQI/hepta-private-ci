use std::collections::BTreeSet;
use std::fs::DirBuilder;
use std::fs::File;
use std::path::Path;

use crate::AcceptanceError;
use crate::ceremony::path_present;
use crate::ceremony::path_string;
use crate::durable::MAX_SMALL_FILE_BYTES;
use crate::durable::canonical_json;
use crate::durable::secure_canonical_file_path;
use crate::durable::secure_read;
use crate::durable::secure_root;
use crate::durable::sha256;
use crate::durable::write_private_new;
use crate::manifest_inventory::VerifiedManifest;
use crate::manifest_inventory::digest_shape;
use crate::model::AuthorityBoundary;

use super::evidence::ValidationPolicy;
use super::evidence::assess_packet;
use super::evidence::exact_candidate;
use super::evidence::validate_output_relative_name;
use super::evidence::validate_spec;
use super::model::AggregateBuildPlanV3;
use super::model::AggregateBuildRecordV3;
use super::model::AggregateBuildSpecV3;
use super::model::AggregateQualificationPacketV3;
use super::model::CandidateBindingV3;
use super::model::SealedAggregateV3;
use super::model::VerifiedAggregateV3;

const BUILD_RECORD_SCHEMA: &str = "hepta_vnext_aggregate_build_record_v3";
const BUILD_PLAN_SCHEMA: &str = "hepta_vnext_aggregate_build_plan_v3";
const SEALED_AGGREGATE_SCHEMA: &str = "hepta_vnext_sealed_aggregate_v3";
const VERIFIED_AGGREGATE_SCHEMA: &str = "hepta_vnext_verified_aggregate_v3";
const AGGREGATE_PREFIX: &str = "vnext-main-52ec4b3868-aggregate-qualification-v3";
const MANIFEST_PATH: &str = "SHA256SUMS";
const MODES_PATH: &str = "MODES.tsv";
const RECORD_PATH: &str = "aggregate-build-record.json";
const SPEC_PATH: &str = "build-spec.json";
const PACKET_PATH: &str = "qualification-packet.json";
const AGGREGATE_MANIFEST_ENTRIES: usize = 4;

const AGGREGATE_FILES: [&str; 5] = [
    MANIFEST_PATH,
    MODES_PATH,
    RECORD_PATH,
    SPEC_PATH,
    PACKET_PATH,
];

struct PreparedBuild {
    plan: AggregateBuildPlanV3,
    packet: AggregateQualificationPacketV3,
    packet_bytes: Vec<u8>,
    record_bytes: Vec<u8>,
    spec_bytes: Vec<u8>,
}

pub(super) fn plan(
    spec_path: &Path,
    expected_spec_sha256: &str,
    output_root: &Path,
    receipts_parent: &Path,
) -> Result<AggregateBuildPlanV3, AcceptanceError> {
    Ok(prepare_with_policy(
        spec_path,
        expected_spec_sha256,
        output_root,
        receipts_parent,
        &exact_candidate(),
        AGGREGATE_PREFIX,
    )?
    .plan)
}

pub(super) fn build(
    spec_path: &Path,
    expected_spec_sha256: &str,
    output_root: &Path,
    receipts_parent: &Path,
) -> Result<SealedAggregateV3, AcceptanceError> {
    let prepared = prepare_with_policy(
        spec_path,
        expected_spec_sha256,
        output_root,
        receipts_parent,
        &exact_candidate(),
        AGGREGATE_PREFIX,
    )?;
    create_output_root(output_root)?;

    let write_result = write_aggregate(output_root, &prepared);
    if let Err(error) = write_result {
        return Err(AcceptanceError::Invalid(format!(
            "aggregate output is incomplete and must never be reused: {error}"
        )));
    }

    let manifest_sha256 = sha256(&secure_read(
        &output_root.join(MANIFEST_PATH),
        MAX_SMALL_FILE_BYTES,
    )?);
    let verified = verify_with_policy(
        output_root,
        &manifest_sha256,
        receipts_parent,
        &exact_candidate(),
        AGGREGATE_PREFIX,
    )?;
    Ok(SealedAggregateV3 {
        aggregate_manifest_entry_count: verified.aggregate_manifest_entry_count,
        aggregate_manifest_sha256: verified.aggregate_manifest_sha256,
        aggregate_root: verified.aggregate_root,
        assessment: verified.assessment,
        build_spec_sha256: verified.build_spec_sha256,
        qualification_packet_sha256: verified.qualification_packet_sha256,
        schema: SEALED_AGGREGATE_SCHEMA.to_string(),
    })
}

pub(super) fn verify(
    aggregate_root: &Path,
    expected_manifest_sha256: &str,
    receipts_parent: &Path,
) -> Result<VerifiedAggregateV3, AcceptanceError> {
    verify_with_policy(
        aggregate_root,
        expected_manifest_sha256,
        receipts_parent,
        &exact_candidate(),
        AGGREGATE_PREFIX,
    )
}

fn verify_with_policy(
    aggregate_root: &Path,
    expected_manifest_sha256: &str,
    receipts_parent: &Path,
    expected_candidate: &CandidateBindingV3,
    aggregate_prefix: &str,
) -> Result<VerifiedAggregateV3, AcceptanceError> {
    if !digest_shape(expected_manifest_sha256) {
        return Err(invalid("aggregate manifest digest is malformed"));
    }
    validate_aggregate_root(aggregate_root, receipts_parent, aggregate_prefix, true)?;
    let aggregate = VerifiedManifest::load(
        aggregate_root,
        expected_manifest_sha256,
        AGGREGATE_MANIFEST_ENTRIES,
    )?;
    let expected_paths = [MODES_PATH, RECORD_PATH, SPEC_PATH, PACKET_PATH]
        .into_iter()
        .collect::<BTreeSet<_>>();
    if aggregate.entry_paths().collect::<BTreeSet<_>>() != expected_paths {
        return Err(invalid(
            "aggregate manifest contains an unexpected file set",
        ));
    }
    verify_modes(aggregate_root, &aggregate.bytes(MODES_PATH)?)?;

    let spec_bytes = aggregate.bytes(SPEC_PATH)?;
    let spec: AggregateBuildSpecV3 = aggregate.json_canonical(SPEC_PATH)?;
    let packet_bytes = aggregate.bytes(PACKET_PATH)?;
    let packet: AggregateQualificationPacketV3 = aggregate.json_canonical(PACKET_PATH)?;
    let record: AggregateBuildRecordV3 = aggregate.json_canonical(RECORD_PATH)?;
    let expected_packet = validate_spec(
        &spec,
        ValidationPolicy {
            expected_candidate,
            receipts_parent,
        },
    )?;
    if packet != expected_packet {
        return Err(invalid(
            "aggregate packet differs from reverified source receipts",
        ));
    }
    let spec_sha256 = sha256(&spec_bytes);
    let packet_sha256 = sha256(&packet_bytes);
    let expected_record = build_record(expected_candidate, &spec_sha256, &packet_sha256);
    if record != expected_record {
        return Err(invalid(
            "aggregate build record differs from its exact source bindings",
        ));
    }
    let assessment = assess_packet(&packet, expected_manifest_sha256);
    Ok(VerifiedAggregateV3 {
        aggregate_manifest_entry_count: aggregate.entry_count(),
        aggregate_manifest_sha256: expected_manifest_sha256.to_string(),
        aggregate_root: path_string(&aggregate.root)?,
        assessment,
        build_spec_sha256: spec_sha256,
        evidence_reverified: true,
        qualification_packet_sha256: packet_sha256,
        schema: VERIFIED_AGGREGATE_SCHEMA.to_string(),
    })
}

fn prepare_with_policy(
    spec_path: &Path,
    expected_spec_sha256: &str,
    output_root: &Path,
    receipts_parent: &Path,
    expected_candidate: &CandidateBindingV3,
    aggregate_prefix: &str,
) -> Result<PreparedBuild, AcceptanceError> {
    if !digest_shape(expected_spec_sha256) {
        return Err(invalid("build-spec digest is malformed"));
    }
    secure_root(receipts_parent, "receipt store")?;
    validate_aggregate_root(output_root, receipts_parent, aggregate_prefix, false)?;
    let spec_path = secure_canonical_file_path(spec_path, "aggregate build spec")?;
    let spec_bytes = secure_read(&spec_path, MAX_SMALL_FILE_BYTES)?;
    if sha256(&spec_bytes) != expected_spec_sha256 {
        return Err(invalid(
            "aggregate build spec differs from its external pin",
        ));
    }
    let spec: AggregateBuildSpecV3 = serde_json::from_slice(&spec_bytes)
        .map_err(|error| invalid(format!("invalid aggregate build spec: {error}")))?;
    if canonical_json(&spec)? != spec_bytes {
        return Err(invalid("aggregate build spec is not canonical JSON"));
    }
    let packet = validate_spec(
        &spec,
        ValidationPolicy {
            expected_candidate,
            receipts_parent,
        },
    )?;
    let packet_bytes = canonical_json(&packet)?;
    let packet_sha256 = sha256(&packet_bytes);
    let record_bytes = canonical_json(&build_record(
        expected_candidate,
        expected_spec_sha256,
        &packet_sha256,
    ))?;
    let assessment = assess_packet(&packet, "");
    let plan = AggregateBuildPlanV3 {
        blockers: assessment.blockers,
        build_spec_sha256: expected_spec_sha256.to_string(),
        candidate_head: expected_candidate.head.clone(),
        candidate_tree: expected_candidate.tree.clone(),
        execute_required: true,
        output_root: path_string(output_root)?,
        qualification_packet_sha256: packet_sha256,
        ready_for_challenge: assessment.ready_for_challenge,
        schema: BUILD_PLAN_SCHEMA.to_string(),
        would_create_files: AGGREGATE_FILES
            .iter()
            .map(|path| (*path).to_string())
            .collect(),
    };
    Ok(PreparedBuild {
        plan,
        packet,
        packet_bytes,
        record_bytes,
        spec_bytes,
    })
}

fn write_aggregate(root: &Path, prepared: &PreparedBuild) -> Result<(), AcceptanceError> {
    let packet_round_trip: AggregateQualificationPacketV3 =
        serde_json::from_slice(&prepared.packet_bytes)
            .map_err(|error| invalid(format!("packet round trip failed: {error}")))?;
    if packet_round_trip != prepared.packet {
        return Err(invalid("canonical packet round trip changed its value"));
    }

    write_private_new(&root.join(RECORD_PATH), &prepared.record_bytes)?;
    write_private_new(&root.join(SPEC_PATH), &prepared.spec_bytes)?;
    write_private_new(&root.join(PACKET_PATH), &prepared.packet_bytes)?;
    write_private_new(&root.join(MODES_PATH), modes_bytes())?;

    let manifest = manifest_bytes(root)?;
    write_private_new(&root.join(MANIFEST_PATH), &manifest)?;
    seal_modes(root)?;
    File::open(root)?.sync_all()?;
    Ok(())
}

fn build_record(
    candidate: &CandidateBindingV3,
    spec_sha256: &str,
    packet_sha256: &str,
) -> AggregateBuildRecordV3 {
    AggregateBuildRecordV3 {
        automatic_transition: false,
        authority: AuthorityBoundary::all_closed(),
        build_spec_sha256: spec_sha256.to_string(),
        candidate_head: candidate.head.clone(),
        candidate_tree: candidate.tree.clone(),
        evidence_reverified: true,
        qualification_packet_sha256: packet_sha256.to_string(),
        schema: BUILD_RECORD_SCHEMA.to_string(),
        schema_version: 3,
    }
}

fn validate_aggregate_root(
    root: &Path,
    receipts_parent: &Path,
    aggregate_prefix: &str,
    must_exist: bool,
) -> Result<(), AcceptanceError> {
    if !root.is_absolute() || root.parent() != Some(receipts_parent) {
        return Err(invalid(
            "aggregate root must be an immediate child of the receipt store",
        ));
    }
    let name = root
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| invalid("aggregate root name is not UTF-8"))?;
    validate_output_relative_name(name)?;
    let suffix = name.strip_prefix(aggregate_prefix).and_then(|value| {
        value
            .strip_prefix('-')
            .filter(|nonce| nonce.len() >= 8 && nonce.len() <= 32)
    });
    if !suffix.is_some_and(|nonce| {
        nonce
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    }) {
        return Err(invalid(
            "aggregate root name is not head-scoped with a lowercase hex nonce",
        ));
    }
    match (must_exist, path_present(root)?) {
        (true, true) | (false, false) => Ok(()),
        (true, false) => Err(invalid("aggregate root does not exist")),
        (false, true) => Err(invalid(
            "aggregate root already exists; one-shot output cannot be reused",
        )),
    }
}

fn create_output_root(root: &Path) -> Result<(), AcceptanceError> {
    let mut builder = DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(root)?;
    secure_root(root, "new aggregate root")?;
    Ok(())
}

fn manifest_bytes(root: &Path) -> Result<Vec<u8>, AcceptanceError> {
    let mut lines = Vec::new();
    let mut paths = [MODES_PATH, RECORD_PATH, SPEC_PATH, PACKET_PATH];
    paths.sort();
    for relative in paths {
        let bytes = secure_read(&root.join(relative), MAX_SMALL_FILE_BYTES)?;
        lines.push(format!("{}  ./{relative}\n", sha256(&bytes)));
    }
    Ok(lines.concat().into_bytes())
}

fn modes_bytes() -> &'static [u8] {
    b"400\t./MODES.tsv\n400\t./SHA256SUMS\n400\t./aggregate-build-record.json\n400\t./build-spec.json\n400\t./qualification-packet.json\n"
}

fn seal_modes(root: &Path) -> Result<(), AcceptanceError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for relative in AGGREGATE_FILES {
            std::fs::set_permissions(root.join(relative), std::fs::Permissions::from_mode(0o400))?;
        }
    }
    Ok(())
}

fn verify_modes(root: &Path, bytes: &[u8]) -> Result<(), AcceptanceError> {
    if bytes != modes_bytes() {
        return Err(invalid(
            "aggregate modes manifest differs from its exact form",
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        for relative in AGGREGATE_FILES {
            let metadata = std::fs::symlink_metadata(root.join(relative))?;
            if !metadata.is_file() || metadata.mode() & 0o777 != 0o400 {
                return Err(invalid(format!("aggregate file mode differs: {relative}")));
            }
        }
    }
    Ok(())
}

pub(super) fn aggregate_prefix() -> &'static str {
    AGGREGATE_PREFIX
}

#[cfg(test)]
pub(super) fn plan_for_test(
    spec_path: &Path,
    expected_spec_sha256: &str,
    output_root: &Path,
    receipts_parent: &Path,
    expected_candidate: &CandidateBindingV3,
    aggregate_prefix: &str,
) -> Result<AggregateBuildPlanV3, AcceptanceError> {
    Ok(prepare_with_policy(
        spec_path,
        expected_spec_sha256,
        output_root,
        receipts_parent,
        expected_candidate,
        aggregate_prefix,
    )?
    .plan)
}

#[cfg(test)]
pub(super) fn build_for_test(
    spec_path: &Path,
    expected_spec_sha256: &str,
    output_root: &Path,
    receipts_parent: &Path,
    expected_candidate: &CandidateBindingV3,
    aggregate_prefix: &str,
) -> Result<SealedAggregateV3, AcceptanceError> {
    let prepared = prepare_with_policy(
        spec_path,
        expected_spec_sha256,
        output_root,
        receipts_parent,
        expected_candidate,
        aggregate_prefix,
    )?;
    create_output_root(output_root)?;
    write_aggregate(output_root, &prepared)?;
    let manifest_sha256 = sha256(&secure_read(
        &output_root.join(MANIFEST_PATH),
        MAX_SMALL_FILE_BYTES,
    )?);
    let verified = verify_with_policy(
        output_root,
        &manifest_sha256,
        receipts_parent,
        expected_candidate,
        aggregate_prefix,
    )?;
    Ok(SealedAggregateV3 {
        aggregate_manifest_entry_count: verified.aggregate_manifest_entry_count,
        aggregate_manifest_sha256: verified.aggregate_manifest_sha256,
        aggregate_root: verified.aggregate_root,
        assessment: verified.assessment,
        build_spec_sha256: verified.build_spec_sha256,
        qualification_packet_sha256: verified.qualification_packet_sha256,
        schema: SEALED_AGGREGATE_SCHEMA.to_string(),
    })
}

#[cfg(test)]
pub(super) fn verify_for_test(
    aggregate_root: &Path,
    expected_manifest_sha256: &str,
    receipts_parent: &Path,
    expected_candidate: &CandidateBindingV3,
    aggregate_prefix: &str,
) -> Result<VerifiedAggregateV3, AcceptanceError> {
    verify_with_policy(
        aggregate_root,
        expected_manifest_sha256,
        receipts_parent,
        expected_candidate,
        aggregate_prefix,
    )
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}

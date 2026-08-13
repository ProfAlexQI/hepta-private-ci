use std::collections::BTreeSet;
use std::fs::DirBuilder;
use std::fs::File;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::path::PathBuf;

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
use super::evidence::verify_mode_manifest;
use super::model::AggregateBuildPlanV3;
use super::model::AggregateBuildRecordV3;
use super::model::AggregateBuildSpecV3;
use super::model::AggregateQualificationPacketV3;
use super::model::CandidateBindingV3;
use super::model::ManifestLayerBindingV3;
use super::model::ManifestLayerIdV3;
use super::model::ManifestRootKindV3;
use super::model::ModeManifestBindingV3;
use super::model::ModeManifestFormatV3;
use super::model::SealedAggregateV3;
use super::model::VerifiedAggregateV3;
use super::profiles;

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

type SpecValidator = for<'a> fn(
    &AggregateBuildSpecV3,
    ValidationPolicy<'a>,
) -> Result<AggregateQualificationPacketV3, AcceptanceError>;

struct PreparedBuild {
    plan: AggregateBuildPlanV3,
    packet: AggregateQualificationPacketV3,
    packet_bytes: Vec<u8>,
    record_bytes: Vec<u8>,
    spec_bytes: Vec<u8>,
    spec_path: PathBuf,
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
        validate_spec,
    )?
    .plan)
}

pub(super) fn build(
    spec_path: &Path,
    expected_spec_sha256: &str,
    output_root: &Path,
    receipts_parent: &Path,
) -> Result<SealedAggregateV3, AcceptanceError> {
    build_with_policy_and_hook(
        spec_path,
        expected_spec_sha256,
        output_root,
        receipts_parent,
        &exact_candidate(),
        AGGREGATE_PREFIX,
        validate_spec,
        || Ok(()),
    )
}

#[allow(clippy::too_many_arguments)]
fn build_with_policy_and_hook(
    spec_path: &Path,
    expected_spec_sha256: &str,
    output_root: &Path,
    receipts_parent: &Path,
    expected_candidate: &CandidateBindingV3,
    aggregate_prefix: &str,
    spec_validator: SpecValidator,
    before_source_reverification: impl FnOnce() -> Result<(), AcceptanceError>,
) -> Result<SealedAggregateV3, AcceptanceError> {
    let prepared = prepare_with_policy(
        spec_path,
        expected_spec_sha256,
        output_root,
        receipts_parent,
        expected_candidate,
        aggregate_prefix,
        spec_validator,
    )?;
    if !prepared.plan.ready_for_challenge || !prepared.plan.blockers.is_empty() {
        return Err(AcceptanceError::Invalid(format!(
            "formal aggregate build is blocked: {}",
            prepared.plan.blockers.join(",")
        )));
    }
    let incoming_root = incoming_root(output_root)?;
    create_output_root(&incoming_root)?;

    let write_result = write_aggregate(&incoming_root, &prepared);
    if let Err(error) = write_result {
        return Err(AcceptanceError::Invalid(format!(
            "aggregate incoming output is incomplete and must never be reused: {error}"
        )));
    }

    let manifest_sha256 = sha256(&secure_read(
        &incoming_root.join(MANIFEST_PATH),
        MAX_SMALL_FILE_BYTES,
    )?);
    verify_contents_with_policy(
        &incoming_root,
        &manifest_sha256,
        receipts_parent,
        expected_candidate,
        spec_validator,
    )?;
    before_source_reverification()?;
    reverify_source_spec(
        &prepared,
        receipts_parent,
        expected_candidate,
        spec_validator,
    )?;
    File::open(&incoming_root)?.sync_all()?;
    File::open(receipts_parent)?.sync_all()?;
    publish_exclusive(&incoming_root, output_root)?;
    File::open(receipts_parent)?.sync_all()?;
    let verified = verify_with_policy(
        output_root,
        &manifest_sha256,
        receipts_parent,
        expected_candidate,
        aggregate_prefix,
        spec_validator,
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
        validate_spec,
    )
}

fn verify_with_policy(
    aggregate_root: &Path,
    expected_manifest_sha256: &str,
    receipts_parent: &Path,
    expected_candidate: &CandidateBindingV3,
    aggregate_prefix: &str,
    spec_validator: SpecValidator,
) -> Result<VerifiedAggregateV3, AcceptanceError> {
    if !digest_shape(expected_manifest_sha256) {
        return Err(invalid("aggregate manifest digest is malformed"));
    }
    validate_aggregate_root(aggregate_root, receipts_parent, aggregate_prefix, true)?;
    verify_contents_with_policy(
        aggregate_root,
        expected_manifest_sha256,
        receipts_parent,
        expected_candidate,
        spec_validator,
    )
}

fn verify_contents_with_policy(
    aggregate_root: &Path,
    expected_manifest_sha256: &str,
    receipts_parent: &Path,
    expected_candidate: &CandidateBindingV3,
    spec_validator: SpecValidator,
) -> Result<VerifiedAggregateV3, AcceptanceError> {
    if !digest_shape(expected_manifest_sha256) {
        return Err(invalid("aggregate manifest digest is malformed"));
    }
    let aggregate = VerifiedManifest::load(
        aggregate_root,
        expected_manifest_sha256,
        AGGREGATE_MANIFEST_ENTRIES,
    )?;
    if aggregate.directory_paths().collect::<BTreeSet<_>>() != [""].into_iter().collect() {
        return Err(invalid(
            "aggregate output must contain no nested or empty directories",
        ));
    }
    let expected_paths = [MODES_PATH, RECORD_PATH, SPEC_PATH, PACKET_PATH]
        .into_iter()
        .collect::<BTreeSet<_>>();
    if aggregate.entry_paths().collect::<BTreeSet<_>>() != expected_paths {
        return Err(invalid(
            "aggregate manifest contains an unexpected file set",
        ));
    }
    verify_modes(aggregate_root, &aggregate, &aggregate.bytes(MODES_PATH)?)?;

    let spec_bytes = aggregate.bytes(SPEC_PATH)?;
    let spec: AggregateBuildSpecV3 = strict_canonical_json(&aggregate, SPEC_PATH)?;
    let packet_bytes = aggregate.bytes(PACKET_PATH)?;
    let packet: AggregateQualificationPacketV3 = strict_canonical_json(&aggregate, PACKET_PATH)?;
    let record: AggregateBuildRecordV3 = strict_canonical_json(&aggregate, RECORD_PATH)?;
    let expected_packet = spec_validator(
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
    aggregate.reverify()?;
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

fn strict_canonical_json<T>(
    manifest: &VerifiedManifest,
    relative: &str,
) -> Result<T, AcceptanceError>
where
    T: for<'de> serde::Deserialize<'de> + serde::Serialize,
{
    let bytes = manifest.bytes(relative)?;
    let value = super::strict_json::parse(&bytes)?;
    let decoded: T = serde_json::from_value(value)
        .map_err(|error| invalid(format!("invalid {relative}: {error}")))?;
    if canonical_json(&decoded)? != bytes {
        return Err(invalid(format!("{relative} is not canonical JSON")));
    }
    Ok(decoded)
}

fn reverify_source_spec(
    prepared: &PreparedBuild,
    receipts_parent: &Path,
    expected_candidate: &CandidateBindingV3,
    spec_validator: SpecValidator,
) -> Result<(), AcceptanceError> {
    let current_spec = secure_read(&prepared.spec_path, MAX_SMALL_FILE_BYTES)?;
    if current_spec != prepared.spec_bytes {
        return Err(invalid(
            "build spec changed after aggregate staging was written",
        ));
    }
    let spec: AggregateBuildSpecV3 =
        serde_json::from_value(super::strict_json::parse(&current_spec)?)
            .map_err(|error| invalid(format!("invalid aggregate build spec: {error}")))?;
    let packet = spec_validator(
        &spec,
        ValidationPolicy {
            expected_candidate,
            receipts_parent,
        },
    )?;
    if canonical_json(&packet)? != prepared.packet_bytes {
        return Err(invalid(
            "source evidence changed after aggregate staging was written",
        ));
    }
    Ok(())
}

fn prepare_with_policy(
    spec_path: &Path,
    expected_spec_sha256: &str,
    output_root: &Path,
    receipts_parent: &Path,
    expected_candidate: &CandidateBindingV3,
    aggregate_prefix: &str,
    spec_validator: SpecValidator,
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
    let spec: AggregateBuildSpecV3 =
        serde_json::from_value(super::strict_json::parse(&spec_bytes)?)
            .map_err(|error| invalid(format!("invalid aggregate build spec: {error}")))?;
    if canonical_json(&spec)? != spec_bytes {
        return Err(invalid("aggregate build spec is not canonical JSON"));
    }
    let packet = spec_validator(
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
        spec_path,
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
    let modes = modes_bytes(root)?;
    write_private_new(&root.join(MODES_PATH), &modes)?;

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
        profile_set: profiles::PROFILE_SET.to_string(),
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

fn incoming_root(output_root: &Path) -> Result<PathBuf, AcceptanceError> {
    let parent = output_root
        .parent()
        .ok_or_else(|| invalid("aggregate output has no parent"))?;
    let name = output_root
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| invalid("aggregate output name is not UTF-8"))?;
    Ok(parent.join(format!(".incoming-{name}")))
}

#[cfg(target_os = "macos")]
pub(super) fn publish_exclusive(source: &Path, destination: &Path) -> Result<(), AcceptanceError> {
    use std::ffi::CString;

    const RENAME_EXCL: libc::c_uint = 0x0000_0004;
    let source = CString::new(source.as_os_str().as_bytes())
        .map_err(|_| invalid("aggregate source path contains NUL"))?;
    let destination = CString::new(destination.as_os_str().as_bytes())
        .map_err(|_| invalid("aggregate destination path contains NUL"))?;
    // SAFETY: both paths are live NUL-terminated strings and the call does not
    // retain their pointers. RENAME_EXCL is the macOS no-replace primitive.
    let rc = unsafe {
        renameatx_np(
            libc::AT_FDCWD,
            source.as_ptr(),
            libc::AT_FDCWD,
            destination.as_ptr(),
            RENAME_EXCL,
        )
    };
    if rc != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(())
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn renameatx_np(
        from_fd: libc::c_int,
        from: *const libc::c_char,
        to_fd: libc::c_int,
        to: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
}

#[cfg(target_os = "linux")]
pub(super) fn publish_exclusive(source: &Path, destination: &Path) -> Result<(), AcceptanceError> {
    use std::ffi::CString;

    const RENAME_NOREPLACE: libc::c_uint = 1;
    let source = CString::new(source.as_os_str().as_bytes())
        .map_err(|_| invalid("aggregate source path contains NUL"))?;
    let destination = CString::new(destination.as_os_str().as_bytes())
        .map_err(|_| invalid("aggregate destination path contains NUL"))?;
    // SAFETY: both paths are live NUL-terminated strings and the syscall does
    // not retain their pointers. renameat2 with RENAME_NOREPLACE is atomic.
    let rc = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            libc::AT_FDCWD,
            source.as_ptr(),
            libc::AT_FDCWD,
            destination.as_ptr(),
            RENAME_NOREPLACE,
        )
    };
    if rc != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(())
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub(super) fn publish_exclusive(
    _source: &Path,
    _destination: &Path,
) -> Result<(), AcceptanceError> {
    Err(invalid(
        "atomic no-replace directory publication is unsupported on this platform",
    ))
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

pub(super) fn modes_bytes(root: &Path) -> Result<Vec<u8>, AcceptanceError> {
    let manifest_size = aggregate_manifest_size();
    let mut mode_size = 0_u64;
    for _ in 0..32 {
        let mut rows = vec![(".".to_string(), "Directory\t700\t-\t.\n".to_string())];
        for relative in AGGREGATE_FILES {
            let size = match relative {
                MODES_PATH => mode_size,
                MANIFEST_PATH => manifest_size,
                _ => std::fs::symlink_metadata(root.join(relative))?.len(),
            };
            rows.push((
                relative.to_string(),
                format!("Regular File\t400\t{size}\t./{relative}\n"),
            ));
        }
        rows.sort_by(|left, right| left.0.cmp(&right.0));
        let bytes = rows
            .into_iter()
            .map(|(_, row)| row)
            .collect::<String>()
            .into_bytes();
        if bytes.len() as u64 == mode_size {
            return Ok(bytes);
        }
        mode_size = bytes.len() as u64;
    }
    Err(invalid(
        "aggregate mode inventory size did not reach a fixed point",
    ))
}

fn aggregate_manifest_size() -> u64 {
    [MODES_PATH, RECORD_PATH, SPEC_PATH, PACKET_PATH]
        .into_iter()
        .map(|relative| 64_u64 + 2 + 2 + relative.len() as u64 + 1)
        .sum()
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

fn verify_modes(
    root: &Path,
    manifest: &VerifiedManifest,
    bytes: &[u8],
) -> Result<(), AcceptanceError> {
    if bytes != modes_bytes(root)? {
        return Err(invalid(
            "aggregate modes manifest differs from its exact form",
        ));
    }
    let mode_manifest = ModeManifestBindingV3 {
        format: ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
        relative_path: MODES_PATH.to_string(),
        sha256: sha256(bytes),
    };
    let binding = ManifestLayerBindingV3 {
        layer_id: ManifestLayerIdV3::Outer,
        manifest_entry_count: manifest.entry_count(),
        manifest_relative_path: MANIFEST_PATH.to_string(),
        manifest_root_kind: ManifestRootKindV3::Sha256ManifestFullInventoryV1,
        manifest_sha256: sha256(&secure_read(
            &root.join(MANIFEST_PATH),
            MAX_SMALL_FILE_BYTES,
        )?),
        mode_manifest,
        root_relative_path: ".".to_string(),
    };
    verify_mode_manifest(
        &binding,
        manifest,
        &[MODES_PATH.to_string()].into_iter().collect(),
    )
}

pub(super) fn aggregate_prefix() -> &'static str {
    AGGREGATE_PREFIX
}

// These hooks compile only into the crate's unit-test harness. They exercise
// the production plan/stage/reverify/publish/verify state machine while letting
// fixtures supply a synthetic evidence validator; the CLI always uses the
// compiled exact validator above.
#[cfg(test)]
pub(super) type SpecValidatorForTest = SpecValidator;

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
pub(super) fn plan_for_test(
    spec_path: &Path,
    expected_spec_sha256: &str,
    output_root: &Path,
    receipts_parent: &Path,
    expected_candidate: &CandidateBindingV3,
    aggregate_prefix: &str,
    spec_validator: SpecValidatorForTest,
) -> Result<AggregateBuildPlanV3, AcceptanceError> {
    Ok(prepare_with_policy(
        spec_path,
        expected_spec_sha256,
        output_root,
        receipts_parent,
        expected_candidate,
        aggregate_prefix,
        spec_validator,
    )?
    .plan)
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
pub(super) fn build_for_test(
    spec_path: &Path,
    expected_spec_sha256: &str,
    output_root: &Path,
    receipts_parent: &Path,
    expected_candidate: &CandidateBindingV3,
    aggregate_prefix: &str,
    spec_validator: SpecValidatorForTest,
    before_source_reverification: impl FnOnce() -> Result<(), AcceptanceError>,
) -> Result<SealedAggregateV3, AcceptanceError> {
    build_with_policy_and_hook(
        spec_path,
        expected_spec_sha256,
        output_root,
        receipts_parent,
        expected_candidate,
        aggregate_prefix,
        spec_validator,
        before_source_reverification,
    )
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
pub(super) fn verify_for_test(
    aggregate_root: &Path,
    expected_manifest_sha256: &str,
    receipts_parent: &Path,
    expected_candidate: &CandidateBindingV3,
    aggregate_prefix: &str,
    spec_validator: SpecValidatorForTest,
) -> Result<VerifiedAggregateV3, AcceptanceError> {
    verify_with_policy(
        aggregate_root,
        expected_manifest_sha256,
        receipts_parent,
        expected_candidate,
        aggregate_prefix,
        spec_validator,
    )
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}

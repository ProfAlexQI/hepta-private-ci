use std::collections::BTreeSet;
use std::ffi::OsString;
use std::path::Path;

use crate::DirectoryAnchorV8;
use crate::FileIdentityV8;
use crate::NativeErrorV8;
use crate::StateRootLockV8;
use crate::TrustedNodeMetadataV8;
use crate::VerifiedFileFdV8;
use crate::invalid;

use super::DurableJournalRecordV8;
use super::JOURNAL_RECORD_DIGITS_V8;
use super::attempt_relative_path_v8;
use super::verify_durable_journal_chain_slices_v8;
use super::verify_durable_journal_chain_v8;

const MAX_DURABLE_RECORD_BYTES_V8: u64 = 64 * 1024 * 1024;
const MAX_DURABLE_JOURNAL_TOTAL_BYTES_V8: u64 = 1024 * 1024 * 1024;
const MAX_DESCRIPTOR_BOUND_RECORDS_V8: usize = 256;
const MAX_DESCRIPTOR_BOUND_RECORD_BYTES_V8: u64 = 8 * 1024 * 1024;
const MAX_DESCRIPTOR_BOUND_TOTAL_BYTES_V8: u64 = 64 * 1024 * 1024;
const DESCRIPTOR_BOUND_RLIMIT_HEADROOM_V8: u64 = 64;

/// Opaque structural inventory obtained by anchored directory enumeration,
/// strict file identity checks, exact canonical decoding, and complete
/// hash-chain replay. It is not effect or release authority by itself.
#[derive(Debug)]
pub struct VerifiedDurableJournalScanV8 {
    attempt_identity_sha256: String,
    incoming_residue_detected: bool,
    last_boot_epoch: u64,
    last_boot_id: String,
    record_count: u64,
    state_root_identity: FileIdentityV8,
    tip_sha256: String,
}

/// Internal replay result retaining the exact decoded records. Public callers
/// receive only the structural scan; typed recovery code uses the retained
/// records to fold effect obligations without reopening pathnames or trusting
/// caller-supplied payloads.
pub(crate) struct VerifiedDurableJournalRecordsV8 {
    pub(crate) scan: VerifiedDurableJournalScanV8,
    pub(crate) records: Vec<DurableJournalRecordV8>,
}

/// Linux-only replay inventory for the consuming descriptor-bound capsule.
/// Unlike the general scanner above, this value intentionally retains every
/// accepted record descriptor and enforces a much smaller closed resource
/// budget. It never crosses the public API or any authority boundary.
pub(crate) struct VerifiedDescriptorBoundDurableJournalRecordsV8 {
    pub(crate) scan: VerifiedDurableJournalScanV8,
    directory_identity: FileIdentityV8,
    directory_metadata: TrustedNodeMetadataV8,
    leaf_names: Vec<OsString>,
    pub(crate) records: Vec<RetainedDurableJournalRecordV8>,
}

/// One accepted journal record with its still-open descriptor. The descriptor
/// is never exposed or cloned; replay code can only re-read and revalidate the
/// exact inode that supplied the canonical record.
pub(crate) struct RetainedDurableJournalRecordV8 {
    canonical_bytes: Vec<u8>,
    descriptor: VerifiedFileFdV8,
    identity: FileIdentityV8,
    metadata: TrustedNodeMetadataV8,
    name: String,
    record: DurableJournalRecordV8,
}

impl RetainedDurableJournalRecordV8 {
    pub(crate) fn record(&self) -> &DurableJournalRecordV8 {
        &self.record
    }

    fn revalidate_named_and_retained_bytes_v8(
        &self,
        journal_directory: &DirectoryAnchorV8,
    ) -> Result<(), NativeErrorV8> {
        self.descriptor.revalidate_identity()?;
        if self.descriptor.identity() != self.identity
            || self.descriptor.trusted_node_metadata()? != self.metadata
        {
            return Err(invalid(
                "retained journal record identity or mount metadata drifted",
            ));
        }
        let retained_before = self
            .descriptor
            .read_all(MAX_DESCRIPTOR_BOUND_RECORD_BYTES_V8)?;
        if retained_before != self.canonical_bytes
            || DurableJournalRecordV8::decode_exact(&retained_before)? != self.record
        {
            return Err(invalid(
                "retained journal descriptor no longer contains its pinned canonical record",
            ));
        }

        let named =
            journal_directory.open_regular_readonly_beneath(Path::new(self.name.as_str()))?;
        if named.identity() != self.identity || named.trusted_node_metadata()? != self.metadata {
            return Err(invalid(
                "journal record pathname no longer names its retained descriptor identity",
            ));
        }
        let named_before = named.read_all(MAX_DESCRIPTOR_BOUND_RECORD_BYTES_V8)?;
        named.revalidate_identity()?;
        if named.trusted_node_metadata()? != self.metadata {
            return Err(invalid(
                "named journal record metadata changed after descriptor read",
            ));
        }

        let retained_after = self
            .descriptor
            .read_all(MAX_DESCRIPTOR_BOUND_RECORD_BYTES_V8)?;
        self.descriptor.revalidate_identity()?;
        if self.descriptor.trusted_node_metadata()? != self.metadata {
            return Err(invalid(
                "retained journal record metadata changed during named replay",
            ));
        }
        let named_after = named.read_all(MAX_DESCRIPTOR_BOUND_RECORD_BYTES_V8)?;
        named.revalidate_identity()?;
        if named.trusted_node_metadata()? != self.metadata
            || retained_before != named_before
            || named_before != retained_after
            || retained_after != named_after
            || named_after != self.canonical_bytes
        {
            return Err(invalid(
                "named and retained journal record bytes changed during descriptor replay",
            ));
        }
        Ok(())
    }
}

impl VerifiedDescriptorBoundDurableJournalRecordsV8 {
    pub(crate) fn revalidate_descriptor_bound_v8(
        &self,
        journal_directory: &DirectoryAnchorV8,
    ) -> Result<(), NativeErrorV8> {
        if journal_directory.current_identity()? != self.directory_identity
            || journal_directory.trusted_node_metadata()? != self.directory_metadata
        {
            return Err(invalid(
                "retained journal directory identity or mount metadata drifted",
            ));
        }
        if journal_directory.list_leaf_names_bounded(super::MAX_DURABLE_JOURNAL_LEAVES_V8)?
            != self.leaf_names
        {
            return Err(invalid(
                "retained journal namespace changed after descriptor replay",
            ));
        }

        for retained in &self.records {
            retained.revalidate_named_and_retained_bytes_v8(journal_directory)?;
        }
        let chain = verify_durable_journal_chain_slices_v8(
            self.records
                .iter()
                .map(|retained| retained.canonical_bytes.as_slice()),
            self.scan.attempt_identity_sha256(),
        )?;
        if chain.record_count() != self.scan.record_count()
            || chain.tip_sha256() != self.scan.tip_sha256()
        {
            return Err(invalid(
                "descriptor-bound journal chain differs from the pinned scan",
            ));
        }

        // Repeat the full descriptor/name/byte pass after chain replay. This
        // closes ordinary in-place cutpoints around the pure hash fold; a
        // hostile same-UID mutate-and-restore entirely between observations
        // remains outside the authority model and is documented as such.
        for retained in &self.records {
            retained.revalidate_named_and_retained_bytes_v8(journal_directory)?;
        }
        let final_chain = verify_durable_journal_chain_slices_v8(
            self.records
                .iter()
                .map(|retained| retained.canonical_bytes.as_slice()),
            self.scan.attempt_identity_sha256(),
        )?;
        if final_chain.record_count() != self.scan.record_count()
            || final_chain.tip_sha256() != self.scan.tip_sha256()
        {
            return Err(invalid(
                "descriptor-bound journal chain differs from the pinned scan",
            ));
        }
        if journal_directory.list_leaf_names_bounded(super::MAX_DURABLE_JOURNAL_LEAVES_V8)?
            != self.leaf_names
            || journal_directory.current_identity()? != self.directory_identity
            || journal_directory.trusted_node_metadata()? != self.directory_metadata
        {
            return Err(invalid(
                "journal directory changed during descriptor-bound chain replay",
            ));
        }
        Ok(())
    }
}

impl VerifiedDurableJournalScanV8 {
    pub fn attempt_identity_sha256(&self) -> &str {
        &self.attempt_identity_sha256
    }

    pub fn incoming_residue_detected(&self) -> bool {
        self.incoming_residue_detected
    }

    pub fn last_boot_epoch(&self) -> u64 {
        self.last_boot_epoch
    }

    pub fn last_boot_id(&self) -> &str {
        &self.last_boot_id
    }

    pub fn record_count(&self) -> u64 {
        self.record_count
    }

    pub fn state_root_identity(&self) -> FileIdentityV8 {
        self.state_root_identity
    }

    pub fn tip_sha256(&self) -> &str {
        &self.tip_sha256
    }
}

pub fn scan_durable_journal_v8(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &StateRootLockV8,
    expected_attempt_identity_sha256: &str,
) -> Result<VerifiedDurableJournalScanV8, NativeErrorV8> {
    if !state_root_lock
        .state_root_identity()
        .matches_stable_directory(state_root.identity())
    {
        return Err(invalid(
            "durable journal scan lock belongs to a different state root",
        ));
    }
    state_root_lock.revalidate_for_root(state_root)?;
    let attempt_directory = attempt_relative_path_v8(expected_attempt_identity_sha256)?;
    let journal_relative = format!("{attempt_directory}/journal");
    let journal_directory = state_root.open_directory_beneath(Path::new(&journal_relative))?;
    let scan = scan_journal_directory_v8(
        &journal_directory,
        expected_attempt_identity_sha256,
        state_root.identity(),
    )?;
    state_root_lock.revalidate_for_root(state_root)?;
    Ok(scan)
}

pub(crate) fn scan_journal_directory_v8(
    journal_directory: &DirectoryAnchorV8,
    expected_attempt_identity_sha256: &str,
    state_root_identity: FileIdentityV8,
) -> Result<VerifiedDurableJournalScanV8, NativeErrorV8> {
    Ok(scan_journal_directory_with_records_v8(
        journal_directory,
        expected_attempt_identity_sha256,
        state_root_identity,
    )?
    .scan)
}

pub(crate) fn scan_journal_directory_with_records_v8(
    journal_directory: &DirectoryAnchorV8,
    expected_attempt_identity_sha256: &str,
    state_root_identity: FileIdentityV8,
) -> Result<VerifiedDurableJournalRecordsV8, NativeErrorV8> {
    let names = journal_directory.list_leaf_names_bounded(super::MAX_DURABLE_JOURNAL_LEAVES_V8)?;
    let mut incoming_residue_detected = false;
    let mut record_names = Vec::new();
    for name in &names {
        let Some(name) = name.to_str() else {
            return Err(invalid("durable journal contains a non-UTF-8 leaf"));
        };
        if name.starts_with('.') && name.ends_with(".incoming") {
            incoming_residue_detected = true;
            continue;
        }
        let sequence = parse_record_name(name)?;
        record_names.push((sequence, name.to_string()));
    }
    if record_names.is_empty() {
        return Err(invalid("durable journal contains no committed records"));
    }
    enforce_record_count_budget(record_names.len())?;
    record_names.sort_by_key(|(sequence, _)| *sequence);

    let directory_identity = journal_directory.identity();
    let mut encoded_records = Vec::with_capacity(record_names.len());
    let mut records = Vec::with_capacity(record_names.len());
    let mut last_boot_epoch = 0;
    let mut last_boot_id = String::new();
    let mut seen_boot_ids = BTreeSet::new();
    let mut total_record_bytes = 0_u64;
    for (index, (name_sequence, name)) in record_names.iter().enumerate() {
        let expected_sequence = u64::try_from(index)
            .map_err(|_| invalid("durable journal inventory index overflow"))?
            .checked_add(1)
            .ok_or_else(|| invalid("durable journal inventory sequence overflow"))?;
        if *name_sequence != expected_sequence {
            return Err(invalid("durable journal filename sequence has a gap"));
        }
        let file = journal_directory.open_regular_readonly_beneath(Path::new(name))?;
        let identity = file.identity();
        if identity.owner_uid() != directory_identity.owner_uid()
            || identity.owner_gid() != directory_identity.owner_gid()
            || identity.mode() != 0o600
            || identity.link_count() != 1
        {
            return Err(invalid(
                "durable journal record ownership, mode, or link count mismatches",
            ));
        }
        total_record_bytes = add_record_to_scan_budget(total_record_bytes, identity.size())?;
        let bytes = file.read_all(MAX_DURABLE_RECORD_BYTES_V8)?;
        let decoded = DurableJournalRecordV8::decode_exact(&bytes)?;
        if decoded.global_sequence() != expected_sequence {
            return Err(invalid(
                "durable journal filename and internal sequence mismatch",
            ));
        }
        if decoded.boot_epoch() < last_boot_epoch {
            return Err(invalid("durable journal boot epoch regressed"));
        }
        if decoded.boot_epoch() == last_boot_epoch
            && !last_boot_id.is_empty()
            && decoded.boot_id() != last_boot_id
        {
            return Err(invalid(
                "durable journal boot id changed without an epoch transition",
            ));
        }
        if decoded.boot_epoch() > last_boot_epoch {
            if decoded.boot_epoch() != last_boot_epoch + 1 {
                return Err(invalid("durable journal boot epoch has a gap"));
            }
            if !last_boot_id.is_empty() && decoded.boot_id() == last_boot_id {
                return Err(invalid(
                    "durable journal boot epoch advanced without a new boot id",
                ));
            }
            if !seen_boot_ids.insert(decoded.boot_id().to_string()) {
                return Err(invalid(
                    "durable journal reused a boot id from an earlier epoch",
                ));
            }
        } else if seen_boot_ids.is_empty() {
            seen_boot_ids.insert(decoded.boot_id().to_string());
        }
        last_boot_epoch = decoded.boot_epoch();
        last_boot_id = decoded.boot_id().to_string();
        records.push(decoded);
        encoded_records.push(bytes);
    }

    let chain =
        verify_durable_journal_chain_v8(&encoded_records, expected_attempt_identity_sha256)?;
    if journal_directory.list_leaf_names_bounded(super::MAX_DURABLE_JOURNAL_LEAVES_V8)? != names {
        return Err(invalid(
            "durable journal namespace changed during anchored scan",
        ));
    }
    Ok(VerifiedDurableJournalRecordsV8 {
        scan: VerifiedDurableJournalScanV8 {
            attempt_identity_sha256: expected_attempt_identity_sha256.to_string(),
            incoming_residue_detected,
            last_boot_epoch,
            last_boot_id,
            record_count: chain.record_count(),
            state_root_identity,
            tip_sha256: chain.tip_sha256().to_string(),
        },
        records,
    })
}

/// Linux-only scanner used exclusively by the consuming descriptor-bound
/// replay capsule. The general scanner above deliberately retains its older,
/// cross-platform resource policy and releases record descriptors normally.
pub(crate) fn scan_journal_directory_descriptor_bound_v8(
    journal_directory: &DirectoryAnchorV8,
    expected_attempt_identity_sha256: &str,
    state_root_identity: FileIdentityV8,
) -> Result<VerifiedDescriptorBoundDurableJournalRecordsV8, NativeErrorV8> {
    let directory_identity = journal_directory.current_identity()?;
    let directory_metadata = journal_directory.trusted_node_metadata()?;
    let names = journal_directory.list_leaf_names_bounded(super::MAX_DURABLE_JOURNAL_LEAVES_V8)?;
    let mut incoming_residue_detected = false;
    let mut record_names = Vec::new();
    for name in &names {
        let Some(name) = name.to_str() else {
            return Err(invalid("durable journal contains a non-UTF-8 leaf"));
        };
        if name.starts_with('.') && name.ends_with(".incoming") {
            incoming_residue_detected = true;
            continue;
        }
        let sequence = parse_record_name(name)?;
        record_names.push((sequence, name.to_string()));
    }
    if record_names.is_empty() {
        return Err(invalid("durable journal contains no committed records"));
    }
    enforce_descriptor_bound_resource_budget_v8(record_names.len())?;
    record_names.sort_by_key(|(sequence, _)| *sequence);

    let mut records = Vec::with_capacity(record_names.len());
    let mut last_boot_epoch = 0;
    let mut last_boot_id = String::new();
    let mut seen_boot_ids = BTreeSet::new();
    let mut total_record_bytes = 0_u64;
    for (index, (name_sequence, name)) in record_names.iter().enumerate() {
        let expected_sequence = u64::try_from(index)
            .map_err(|_| invalid("durable journal inventory index overflow"))?
            .checked_add(1)
            .ok_or_else(|| invalid("durable journal inventory sequence overflow"))?;
        if *name_sequence != expected_sequence {
            return Err(invalid("durable journal filename sequence has a gap"));
        }
        let file = journal_directory.open_regular_readonly_beneath(Path::new(name))?;
        let identity = file.identity();
        if identity.owner_uid() != directory_identity.owner_uid()
            || identity.owner_gid() != directory_identity.owner_gid()
            || identity.mode() != 0o600
            || identity.link_count() != 1
        {
            return Err(invalid(
                "durable journal record ownership, mode, or link count mismatches",
            ));
        }
        let metadata = file.trusted_node_metadata()?;
        if !metadata.matches_filesystem_domain(directory_metadata) {
            return Err(invalid(
                "durable journal record differs from its directory mount domain",
            ));
        }
        if identity.size() > MAX_DESCRIPTOR_BOUND_RECORD_BYTES_V8 {
            return Err(invalid(format!(
                "descriptor-bound journal record exceeds the {MAX_DESCRIPTOR_BOUND_RECORD_BYTES_V8}-byte retained-record budget"
            )));
        }
        total_record_bytes =
            add_descriptor_bound_record_to_scan_budget(total_record_bytes, identity.size())?;
        let bytes = file.read_all(MAX_DESCRIPTOR_BOUND_RECORD_BYTES_V8)?;
        file.revalidate_identity()?;
        if file.trusted_node_metadata()? != metadata {
            return Err(invalid(
                "durable journal record metadata changed during descriptor read",
            ));
        }
        let decoded = DurableJournalRecordV8::decode_exact(&bytes)?;
        if decoded.global_sequence() != expected_sequence {
            return Err(invalid(
                "durable journal filename and internal sequence mismatch",
            ));
        }
        if decoded.boot_epoch() < last_boot_epoch {
            return Err(invalid("durable journal boot epoch regressed"));
        }
        if decoded.boot_epoch() == last_boot_epoch
            && !last_boot_id.is_empty()
            && decoded.boot_id() != last_boot_id
        {
            return Err(invalid(
                "durable journal boot id changed without an epoch transition",
            ));
        }
        if decoded.boot_epoch() > last_boot_epoch {
            if decoded.boot_epoch() != last_boot_epoch + 1 {
                return Err(invalid("durable journal boot epoch has a gap"));
            }
            if !last_boot_id.is_empty() && decoded.boot_id() == last_boot_id {
                return Err(invalid(
                    "durable journal boot epoch advanced without a new boot id",
                ));
            }
            if !seen_boot_ids.insert(decoded.boot_id().to_string()) {
                return Err(invalid(
                    "durable journal reused a boot id from an earlier epoch",
                ));
            }
        } else if seen_boot_ids.is_empty() {
            seen_boot_ids.insert(decoded.boot_id().to_string());
        }
        last_boot_epoch = decoded.boot_epoch();
        last_boot_id = decoded.boot_id().to_string();
        records.push(RetainedDurableJournalRecordV8 {
            canonical_bytes: bytes,
            descriptor: file,
            identity,
            metadata,
            name: name.clone(),
            record: decoded,
        });
    }

    let chain = verify_durable_journal_chain_slices_v8(
        records
            .iter()
            .map(|retained| retained.canonical_bytes.as_slice()),
        expected_attempt_identity_sha256,
    )?;
    if journal_directory.list_leaf_names_bounded(super::MAX_DURABLE_JOURNAL_LEAVES_V8)? != names
        || journal_directory.current_identity()? != directory_identity
        || journal_directory.trusted_node_metadata()? != directory_metadata
    {
        return Err(invalid(
            "descriptor-bound journal namespace or directory identity changed during scan",
        ));
    }
    Ok(VerifiedDescriptorBoundDurableJournalRecordsV8 {
        scan: VerifiedDurableJournalScanV8 {
            attempt_identity_sha256: expected_attempt_identity_sha256.to_string(),
            incoming_residue_detected,
            last_boot_epoch,
            last_boot_id,
            record_count: chain.record_count(),
            state_root_identity,
            tip_sha256: chain.tip_sha256().to_string(),
        },
        directory_identity,
        directory_metadata,
        leaf_names: names,
        records,
    })
}

fn enforce_record_count_budget(record_count: usize) -> Result<(), NativeErrorV8> {
    if record_count > super::MAX_DURABLE_JOURNAL_RECORDS_V8 {
        return Err(invalid(format!(
            "durable journal exceeds the {}-record scan budget",
            super::MAX_DURABLE_JOURNAL_RECORDS_V8
        )));
    }
    Ok(())
}

fn enforce_descriptor_bound_resource_budget_v8(record_count: usize) -> Result<(), NativeErrorV8> {
    if record_count > MAX_DESCRIPTOR_BOUND_RECORDS_V8 {
        return Err(invalid(format!(
            "descriptor-bound replay exceeds the {MAX_DESCRIPTOR_BOUND_RECORDS_V8}-record retained-fd budget"
        )));
    }
    enforce_descriptor_fd_headroom_v8(record_count)
}

fn descriptor_bound_required_fd_capacity_v8(record_count: usize) -> Result<u64, NativeErrorV8> {
    u64::try_from(record_count)
        .map_err(|_| invalid("descriptor-bound record count exceeds u64"))?
        .checked_mul(2)
        .and_then(|retained_and_fresh| {
            retained_and_fresh.checked_add(DESCRIPTOR_BOUND_RLIMIT_HEADROOM_V8)
        })
        .ok_or_else(|| invalid("descriptor-bound duplicate replay fd capacity overflow"))
}

#[cfg(target_os = "linux")]
fn enforce_descriptor_fd_headroom_v8(record_count: usize) -> Result<(), NativeErrorV8> {
    // SAFETY: zero initializes a writable rlimit structure for getrlimit.
    let mut limit: libc::rlimit = unsafe { std::mem::zeroed() };
    // SAFETY: limit remains writable for the call and RLIMIT_NOFILE is a
    // scalar resource selector.
    if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut limit) } != 0 {
        return Err(crate::NativeSysErrorV8::Io {
            operation: "getrlimit descriptor-bound replay",
            source: std::io::Error::last_os_error(),
        }
        .into());
    }
    let required = descriptor_bound_required_fd_capacity_v8(record_count)?;
    let soft_limit = limit.rlim_cur;
    if soft_limit != libc::RLIM_INFINITY && soft_limit < required {
        return Err(invalid(format!(
            "descriptor-bound replay lacks minimum RLIMIT_NOFILE capacity: records={record_count} reserve={DESCRIPTOR_BOUND_RLIMIT_HEADROOM_V8} limit={soft_limit}"
        )));
    }
    // We intentionally do not scan the whole descriptor-number domain here.
    // The compile-time record cap bounds allocation and open attempts; any
    // ambient descriptor pressure fails safely at openat2 with no authority.
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn enforce_descriptor_fd_headroom_v8(_record_count: usize) -> Result<(), NativeErrorV8> {
    Err(crate::NativeSysErrorV8::UnsupportedPlatform(
        "descriptor-bound replay RLIMIT_NOFILE inventory",
    )
    .into())
}

fn add_record_to_scan_budget(current: u64, record_size: u64) -> Result<u64, NativeErrorV8> {
    let total = current
        .checked_add(record_size)
        .ok_or_else(|| invalid("durable journal cumulative size overflow"))?;
    if total > MAX_DURABLE_JOURNAL_TOTAL_BYTES_V8 {
        return Err(invalid(format!(
            "durable journal exceeds the {MAX_DURABLE_JOURNAL_TOTAL_BYTES_V8}-byte scan budget"
        )));
    }
    Ok(total)
}

fn add_descriptor_bound_record_to_scan_budget(
    current: u64,
    record_size: u64,
) -> Result<u64, NativeErrorV8> {
    let total = current
        .checked_add(record_size)
        .ok_or_else(|| invalid("descriptor-bound journal cumulative size overflow"))?;
    if total > MAX_DESCRIPTOR_BOUND_TOTAL_BYTES_V8 {
        return Err(invalid(format!(
            "descriptor-bound journal exceeds the {MAX_DESCRIPTOR_BOUND_TOTAL_BYTES_V8}-byte retained-byte budget"
        )));
    }
    Ok(total)
}

fn parse_record_name(name: &str) -> Result<u64, NativeErrorV8> {
    let expected_length = JOURNAL_RECORD_DIGITS_V8 + ".record".len();
    if name.len() != expected_length || !name.ends_with(".record") {
        return Err(invalid("durable journal contains an unknown leaf"));
    }
    let digits = name
        .get(..JOURNAL_RECORD_DIGITS_V8)
        .ok_or_else(|| invalid("durable journal record name is truncated"))?;
    if !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(invalid("durable journal record name is not canonical"));
    }
    let sequence = digits
        .parse::<u64>()
        .map_err(|_| invalid("durable journal record sequence is invalid"))?;
    if sequence == 0 {
        return Err(invalid("durable journal record sequence must be non-zero"));
    }
    Ok(sequence)
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use std::ffi::OsStr;
    use std::fs;
    use std::os::unix::fs::OpenOptionsExt;
    use std::os::unix::fs::PermissionsExt;

    use super::*;
    use crate::publish_record_noreplace_v8;

    const ZERO_SHA256: &str = "0000000000000000000000000000000000000000000000000000000000000000";

    fn temporary_directory(label: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "hepta-linux-v8-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        path
    }

    fn digest(character: char) -> String {
        character.to_string().repeat(64)
    }

    fn scan_boot_chain(label: &str, boots: &[(u64, &str)]) -> Result<(), NativeErrorV8> {
        let root = temporary_directory(label);
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let attempt = digest('1');
        let mut previous = ZERO_SHA256.to_string();
        for (index, (epoch, boot_id)) in boots.iter().enumerate() {
            let sequence = u64::try_from(index).unwrap() + 1;
            let record = DurableJournalRecordV8::new(
                attempt.clone(),
                *epoch,
                (*boot_id).to_string(),
                sequence,
                previous,
                format!("boot-{sequence}").into_bytes(),
            )
            .unwrap();
            publish_record_noreplace_v8(
                &anchor,
                &format!("{sequence:020}.record"),
                &digest(char::from_digit(u32::try_from(sequence).unwrap() + 1, 10).unwrap()),
                &record.canonical_bytes().unwrap(),
            )
            .unwrap();
            previous = record.record_sha256().unwrap();
        }
        let result = scan_journal_directory_v8(&anchor, &attempt, anchor.identity()).map(|_| ());
        fs::remove_dir_all(root).unwrap();
        result
    }

    #[test]
    fn scan_binds_filesystem_identity_names_bytes_and_chain() {
        let root = temporary_directory("scan");
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let attempt = digest('1');
        let first = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            1,
            ZERO_SHA256.to_string(),
            b"first".to_vec(),
        )
        .unwrap();
        let second = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            2,
            first.record_sha256().unwrap(),
            b"second".to_vec(),
        )
        .unwrap();
        for record in [&first, &second] {
            let name = format!("{:020}.record", record.global_sequence());
            publish_record_noreplace_v8(
                &anchor,
                &name,
                &digest(char::from_digit(record.global_sequence() as u32 + 1, 10).unwrap()),
                &record.canonical_bytes().unwrap(),
            )
            .unwrap();
        }

        let scan = scan_journal_directory_v8(&anchor, &attempt, anchor.identity()).unwrap();
        assert_eq!(scan.record_count(), 2);
        assert_eq!(scan.tip_sha256(), second.record_sha256().unwrap());
        assert!(!scan.incoming_residue_detected());

        fs::write(root.join("unknown"), b"bad").unwrap();
        assert!(scan_journal_directory_v8(&anchor, &attempt, anchor.identity()).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn incoming_residue_is_visible_for_forced_abandonment() {
        let root = temporary_directory("incoming");
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let attempt = digest('1');
        let first = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            1,
            ZERO_SHA256.to_string(),
            b"first".to_vec(),
        )
        .unwrap();
        publish_record_noreplace_v8(
            &anchor,
            "00000000000000000001.record",
            &digest('2'),
            &first.canonical_bytes().unwrap(),
        )
        .unwrap();
        fs::write(
            root.join(format!(
                ".00000000000000000002.record.{}.incoming",
                digest('3')
            )),
            b"partial",
        )
        .unwrap();
        let scan = scan_journal_directory_v8(&anchor, &attempt, anchor.identity()).unwrap();
        assert!(scan.incoming_residue_detected());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn filename_gap_fails_closed() {
        let root = temporary_directory("scan-negative");
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let attempt = digest('1');
        let record = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            2,
            digest('2'),
            b"gap".to_vec(),
        )
        .unwrap();
        fs::write(
            root.join("00000000000000000002.record"),
            record.canonical_bytes().unwrap(),
        )
        .unwrap();
        assert!(scan_journal_directory_v8(&anchor, &attempt, anchor.identity()).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn boot_epochs_require_one_new_never_reused_id_per_transition() {
        const BOOT_A: &str = "01234567-89ab-cdef-0123-456789abcdef";
        const BOOT_B: &str = "11234567-89ab-cdef-0123-456789abcdef";
        assert!(scan_boot_chain("boot-positive", &[(1, BOOT_A), (1, BOOT_A), (2, BOOT_B)]).is_ok());
        assert!(scan_boot_chain("boot-same-epoch-splice", &[(1, BOOT_A), (1, BOOT_B)]).is_err());
        assert!(scan_boot_chain("boot-same-id-next-epoch", &[(1, BOOT_A), (2, BOOT_A)]).is_err());
        assert!(scan_boot_chain("boot-reuse", &[(1, BOOT_A), (2, BOOT_B), (3, BOOT_A)]).is_err());
    }

    #[test]
    fn record_mode_and_external_hardlink_fail_closed() {
        for hardlink in [false, true] {
            let root = temporary_directory(if hardlink { "hardlink" } else { "mode" });
            let anchor = DirectoryAnchorV8::open(&root).unwrap();
            let attempt = digest('1');
            let record = DurableJournalRecordV8::new(
                attempt.clone(),
                1,
                "01234567-89ab-cdef-0123-456789abcdef".to_string(),
                1,
                ZERO_SHA256.to_string(),
                b"identity-negative".to_vec(),
            )
            .unwrap();
            let record_path = root.join("00000000000000000001.record");
            publish_record_noreplace_v8(
                &anchor,
                "00000000000000000001.record",
                &digest('2'),
                &record.canonical_bytes().unwrap(),
            )
            .unwrap();
            let external = root.with_extension("external-hardlink");
            if hardlink {
                fs::hard_link(&record_path, &external).unwrap();
            } else {
                fs::set_permissions(&record_path, fs::Permissions::from_mode(0o640)).unwrap();
            }
            assert!(scan_journal_directory_v8(&anchor, &attempt, anchor.identity()).is_err());
            if hardlink {
                fs::remove_file(external).unwrap();
            }
            fs::remove_dir_all(root).unwrap();
        }
    }

    #[test]
    fn durable_record_real_uid_or_gid_mismatch_fails_closed() {
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt as _;
        use std::os::unix::fs::MetadataExt as _;

        let root = temporary_directory("owner-mismatch");
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let directory_identity = anchor.identity();
        let attempt = digest('1');
        let record = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            1,
            ZERO_SHA256.to_string(),
            b"real-owner-negative".to_vec(),
        )
        .unwrap();
        let record_path = root.join("00000000000000000001.record");
        publish_record_noreplace_v8(
            &anchor,
            "00000000000000000001.record",
            &digest('2'),
            &record.canonical_bytes().unwrap(),
        )
        .unwrap();
        let record_path_c = CString::new(record_path.as_os_str().as_bytes()).unwrap();

        // Exercise a real kernel credential transition. Privileged
        // qualification changes both UID and GID. A rootless run may change
        // only to an actual supplementary GID; it never fabricates metadata.
        // SAFETY: geteuid has no arguments or preconditions.
        let effective_uid = unsafe { libc::geteuid() };
        if effective_uid == 0 {
            let mismatched_uid = if directory_identity.owner_uid() == 65_534 {
                65_533
            } else {
                65_534
            };
            let mismatched_gid = if directory_identity.owner_gid() == 65_534 {
                65_533
            } else {
                65_534
            };
            // SAFETY: record_path_c is a live NUL-terminated path inside this
            // test's private temporary directory.
            let rc = unsafe { libc::chown(record_path_c.as_ptr(), mismatched_uid, mismatched_gid) };
            assert_eq!(
                rc,
                0,
                "privileged fixture must perform a real UID/GID mismatch: {}",
                std::io::Error::last_os_error()
            );
        } else {
            // SAFETY: the first getgroups call requests only the required
            // count; the second receives exactly that many gid_t slots.
            let group_count = unsafe { libc::getgroups(0, std::ptr::null_mut()) };
            if group_count < 0 {
                eprintln!(
                    "SKIP durable real-owner mismatch: rootless getgroups failed: {}",
                    std::io::Error::last_os_error()
                );
                fs::remove_dir_all(root).unwrap();
                return;
            }
            let mut groups = vec![0 as libc::gid_t; usize::try_from(group_count).unwrap()];
            if group_count > 0 {
                // SAFETY: groups has group_count writable gid_t elements.
                let loaded = unsafe { libc::getgroups(group_count, groups.as_mut_ptr()) };
                if loaded != group_count {
                    eprintln!(
                        "SKIP durable real-owner mismatch: rootless supplementary-group inventory changed or failed: {}",
                        std::io::Error::last_os_error()
                    );
                    fs::remove_dir_all(root).unwrap();
                    return;
                }
            }
            let Some(mismatched_gid) = groups
                .into_iter()
                .find(|gid| *gid != directory_identity.owner_gid())
            else {
                eprintln!(
                    "SKIP durable real-owner mismatch: rootless process has no supplementary GID distinct from directory GID {}",
                    directory_identity.owner_gid()
                );
                fs::remove_dir_all(root).unwrap();
                return;
            };
            // SAFETY: record_path_c is a live NUL-terminated path inside this
            // test's private temporary directory. uid_t::MAX means unchanged.
            let rc =
                unsafe { libc::chown(record_path_c.as_ptr(), libc::uid_t::MAX, mismatched_gid) };
            if rc != 0 {
                eprintln!(
                    "SKIP durable real-owner mismatch: rootless chgrp to actual supplementary GID {mismatched_gid} was denied: {}",
                    std::io::Error::last_os_error()
                );
                fs::remove_dir_all(root).unwrap();
                return;
            }
        }

        let changed = fs::metadata(&record_path).unwrap();
        assert!(
            changed.uid() != directory_identity.owner_uid()
                || changed.gid() != directory_identity.owner_gid(),
            "fixture must establish a real leaf UID or GID mismatch"
        );
        let error = scan_journal_directory_v8(&anchor, &attempt, directory_identity).unwrap_err();
        match error {
            NativeErrorV8::Invalid(message) => assert_eq!(
                message,
                "durable journal record ownership, mode, or link count mismatches"
            ),
            other => panic!("unexpected durable owner-mismatch error: {other}"),
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn parser_rejects_noncanonical_or_unknown_names() {
        assert_eq!(parse_record_name("00000000000000000001.record").unwrap(), 1);
        for name in [
            "1.record",
            "00000000000000000000.record",
            "00000000000000000001.record.extra",
            "0000000000000000000a.record",
            OsStr::new("unknown").to_str().unwrap(),
        ] {
            assert!(parse_record_name(name).is_err());
        }
    }

    #[test]
    fn scan_budgets_fail_before_unbounded_allocation_or_reads() {
        assert!(enforce_record_count_budget(super::super::MAX_DURABLE_JOURNAL_RECORDS_V8).is_ok());
        assert!(
            enforce_record_count_budget(super::super::MAX_DURABLE_JOURNAL_RECORDS_V8 + 1).is_err()
        );
        assert_eq!(
            add_record_to_scan_budget(MAX_DURABLE_JOURNAL_TOTAL_BYTES_V8 - 1, 1).unwrap(),
            MAX_DURABLE_JOURNAL_TOTAL_BYTES_V8
        );
        assert!(add_record_to_scan_budget(MAX_DURABLE_JOURNAL_TOTAL_BYTES_V8, 1).is_err());

        assert!(
            enforce_record_count_budget(MAX_DESCRIPTOR_BOUND_RECORDS_V8 + 1).is_ok(),
            "the general scanner must retain its historical 65,536-record budget"
        );
        assert!(
            enforce_descriptor_bound_resource_budget_v8(MAX_DESCRIPTOR_BOUND_RECORDS_V8 + 1)
                .is_err()
        );
        assert_eq!(
            descriptor_bound_required_fd_capacity_v8(MAX_DESCRIPTOR_BOUND_RECORDS_V8).unwrap(),
            2 * MAX_DESCRIPTOR_BOUND_RECORDS_V8 as u64 + DESCRIPTOR_BOUND_RLIMIT_HEADROOM_V8
        );
        assert_eq!(
            add_descriptor_bound_record_to_scan_budget(MAX_DESCRIPTOR_BOUND_TOTAL_BYTES_V8 - 1, 1,)
                .unwrap(),
            MAX_DESCRIPTOR_BOUND_TOTAL_BYTES_V8
        );
        assert!(
            add_descriptor_bound_record_to_scan_budget(MAX_DESCRIPTOR_BOUND_TOTAL_BYTES_V8, 1,)
                .is_err()
        );
    }

    #[test]
    fn descriptor_scanner_rejects_oversize_sparse_record_before_reading_it() {
        let root = temporary_directory("descriptor-oversize");
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let attempt = digest('1');
        let record_path = root.join("00000000000000000001.record");
        let record = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&record_path)
            .unwrap();
        record
            .set_len(MAX_DESCRIPTOR_BOUND_RECORD_BYTES_V8 + 1)
            .unwrap();
        drop(record);
        let error =
            scan_journal_directory_descriptor_bound_v8(&anchor, &attempt, anchor.identity())
                .err()
                .expect("oversize descriptor scan must fail");
        assert!(error.to_string().contains("retained-record budget"));
        fs::remove_dir_all(root).unwrap();
    }
}

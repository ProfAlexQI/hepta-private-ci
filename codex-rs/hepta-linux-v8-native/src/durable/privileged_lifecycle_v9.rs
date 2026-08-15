//! Fail-closed Linux v9 privileged-lifecycle admission boundary.
//!
//! This module deliberately exposes no effect executor or authority token.
//! The concrete read-only admission typestate is added alongside the
//! descriptor-retention and replay invariants it composes.

use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

use serde::Serialize;
use sha2::Digest as _;

use crate::ClosedAuthorityFlagV8;
use crate::DescriptorBoundRuntimeAssessmentV8;
use crate::DirectoryAnchorV8;
use crate::FileIdentityV8;
use crate::NativeErrorV8;
use crate::TrustedNodeMetadataV8;
use crate::VerifiedDescriptorBoundReplayV8;
use crate::VerifiedFileFdV8;
use crate::VerifiedPacketV8;
use crate::VerifiedPeerV8;
use crate::invalid;

use super::validate_leaf_name;

const MAX_CANDIDATE_REQUEST_BYTES_V9: u64 = crate::MAX_SEQPACKET_PAYLOAD_BYTES_V8 as u64;

/// Retained exact bytes for the candidate request named by typed journal
/// evidence. The writer namespace and request descriptor stay open for the
/// token lifetime. This is only an independent hash binding; it does not
/// interpret the request or authorize its execution.
#[derive(Debug)]
pub struct RetainedCandidateRequestV9 {
    file: RetainedCandidateRequestFileV9,
    producer: VerifiedPeerV8,
}

#[derive(Debug)]
struct RetainedCandidateRequestFileV9 {
    bytes: Vec<u8>,
    namespace: DirectoryAnchorV8,
    namespace_identity: FileIdentityV8,
    namespace_metadata: TrustedNodeMetadataV8,
    namespace_path: PathBuf,
    request: VerifiedFileFdV8,
    request_identity: FileIdentityV8,
    request_leaf: OsString,
    request_metadata: TrustedNodeMetadataV8,
    request_sha256: String,
    writer_gid: u32,
    writer_uid: u32,
}

impl RetainedCandidateRequestV9 {
    pub fn request_sha256(&self) -> &str {
        &self.file.request_sha256
    }

    pub fn producer_uid(&self) -> u32 {
        self.producer.uid()
    }

    pub fn writer_uid(&self) -> u32 {
        self.file.writer_uid
    }

    pub fn revalidate(&self) -> Result<(), NativeErrorV8> {
        require_live_separated_producer_v9(&self.producer, self.file.writer_uid)?;
        self.file.revalidate()?;
        require_live_separated_producer_v9(&self.producer, self.file.writer_uid)
    }

    fn matches_writer_root_v9(&self, root: FileIdentityV8) -> bool {
        self.file.matches_writer_root_v9(root)
    }
}

impl RetainedCandidateRequestFileV9 {
    fn revalidate(&self) -> Result<(), NativeErrorV8> {
        let (effective_uid, effective_gid) = effective_ids_v9()?;
        if effective_uid != self.writer_uid || effective_gid != self.writer_gid {
            return Err(invalid(
                "candidate-request writer credential changed after pinning",
            ));
        }

        self.namespace.revalidate_identity()?;
        if self.namespace.current_identity()? != self.namespace_identity
            || self.namespace.trusted_node_metadata()? != self.namespace_metadata
        {
            return Err(invalid(
                "retained candidate-request namespace identity or metadata drifted",
            ));
        }
        require_exact_request_roster_v9(&self.namespace, &self.request_leaf)?;

        self.request.revalidate_identity()?;
        if self.request.identity() != self.request_identity
            || self.request.trusted_node_metadata()? != self.request_metadata
            || self.request.read_all(MAX_CANDIDATE_REQUEST_BYTES_V9)? != self.bytes
        {
            return Err(invalid(
                "retained candidate-request descriptor identity, metadata, or bytes drifted",
            ));
        }

        let named_namespace = DirectoryAnchorV8::open(&self.namespace_path)?;
        if !named_namespace
            .identity()
            .matches_stable_directory(self.namespace.identity())
            || named_namespace.current_identity()? != self.namespace_identity
            || named_namespace.trusted_node_metadata()? != self.namespace_metadata
        {
            return Err(invalid(
                "candidate-request pathname no longer names the retained namespace",
            ));
        }
        require_exact_request_roster_v9(&named_namespace, &self.request_leaf)?;
        let named = named_namespace.open_regular_readonly_beneath(Path::new(&self.request_leaf))?;
        if named.identity() != self.request_identity
            || named.trusted_node_metadata()? != self.request_metadata
            || named.read_all(MAX_CANDIDATE_REQUEST_BYTES_V9)? != self.bytes
        {
            return Err(invalid(
                "candidate-request pathname no longer names the retained exact bytes",
            ));
        }

        self.request.revalidate_identity()?;
        self.namespace.revalidate_identity()?;
        if self.request.read_all(MAX_CANDIDATE_REQUEST_BYTES_V9)? != self.bytes {
            return Err(invalid(
                "candidate-request descriptor changed during final replay",
            ));
        }
        Ok(())
    }

    fn matches_writer_root_v9(&self, root: FileIdentityV8) -> bool {
        root.owner_uid() == self.writer_uid
            && root.owner_gid() == self.writer_gid
            && root.mode() == 0o700
            && root.link_count() > 0
    }
}

/// Opens a candidate request only inside a credential-separated writer
/// namespace and binds its exact bytes to one kernel-authenticated seqpacket
/// request. The function is crate-private: no external caller can mint the
/// retained typestate from a caller-selected UID/pathname and mistake it for
/// an effect token.
pub(crate) fn open_credential_separated_candidate_request_v9(
    namespace_path: &Path,
    request_leaf: &OsStr,
    packet: VerifiedPacketV8,
) -> Result<RetainedCandidateRequestV9, NativeErrorV8> {
    let (payload, producer, file_descriptors) = packet.into_parts();
    if !file_descriptors.is_empty() {
        return Err(invalid(
            "candidate request packet must carry zero ancillary file descriptors",
        ));
    }
    let (writer_uid, _) = effective_ids_v9()?;
    require_live_separated_producer_v9(&producer, writer_uid)?;
    let file = open_retained_candidate_request_file_v9(namespace_path, request_leaf, &payload)?;
    let retained = RetainedCandidateRequestV9 { file, producer };
    retained.revalidate()?;
    Ok(retained)
}

fn open_retained_candidate_request_file_v9(
    namespace_path: &Path,
    request_leaf: &OsStr,
    expected_bytes: &[u8],
) -> Result<RetainedCandidateRequestFileV9, NativeErrorV8> {
    let request_leaf_utf8 = request_leaf
        .to_str()
        .ok_or_else(|| invalid("candidate-request leaf is not UTF-8"))?;
    validate_leaf_name(request_leaf_utf8)?;
    let (writer_uid, writer_gid) = effective_ids_v9()?;
    if expected_bytes.is_empty() || expected_bytes.len() as u64 > MAX_CANDIDATE_REQUEST_BYTES_V9 {
        return Err(invalid(
            "candidate request packet must contain one nonempty bounded exact byte sequence",
        ));
    }

    let namespace = DirectoryAnchorV8::open(namespace_path)?;
    let namespace_identity = namespace.current_identity()?;
    let namespace_metadata = namespace.trusted_node_metadata()?;
    require_writer_namespace_v9(namespace_identity, writer_uid, writer_gid)?;
    require_exact_request_roster_v9(&namespace, request_leaf)?;

    let request = namespace.open_regular_readonly_beneath(Path::new(request_leaf))?;
    let request_identity = request.identity();
    let request_metadata = request.trusted_node_metadata()?;
    require_request_identity_v9(
        request_identity,
        request_metadata,
        namespace_identity,
        namespace_metadata,
        writer_uid,
        writer_gid,
    )?;
    let bytes = request.read_all(MAX_CANDIDATE_REQUEST_BYTES_V9)?;
    if bytes != expected_bytes || request_identity.size() != bytes.len() as u64 {
        return Err(invalid(
            "candidate request file does not equal the kernel-authenticated packet bytes",
        ));
    }
    let request_sha256 = format!("{:x}", sha2::Sha256::digest(&bytes));
    let retained = RetainedCandidateRequestFileV9 {
        bytes,
        namespace,
        namespace_identity,
        namespace_metadata,
        namespace_path: namespace_path.to_path_buf(),
        request,
        request_identity,
        request_leaf: request_leaf.to_os_string(),
        request_metadata,
        request_sha256,
        writer_gid,
        writer_uid,
    };
    retained.revalidate()?;
    Ok(retained)
}

/// Opaque v9 composition of the descriptor-bound durable replay, retained
/// machine-id source, exact `{journal}` attempt roster, and an independently
/// retained candidate request under a different producer UID. It intentionally
/// has no method that yields an executor or authority token.
pub struct PrivilegedLifecycleNoAuthorityV9 {
    candidate_request: RetainedCandidateRequestV9,
    replay: VerifiedDescriptorBoundReplayV8,
}

impl PrivilegedLifecycleNoAuthorityV9 {
    pub fn assess_read_only(&self) -> Result<PrivilegedLifecycleAssessmentV9, NativeErrorV8> {
        self.replay.revalidate()?;
        self.candidate_request.revalidate()?;
        let descriptor = self.replay.assess_read_only()?;
        require_candidate_request_binding_v9(&descriptor, &self.candidate_request)?;
        self.candidate_request.revalidate()?;
        self.replay.revalidate()?;
        Ok(PrivilegedLifecycleAssessmentV9 {
            schema: "hepta-linux-v9-privileged-lifecycle-no-authority-v1".to_string(),
            descriptor_bound_replay: descriptor,
            candidate_request_sha256: self.candidate_request.request_sha256().to_string(),
            writer_uid: self.candidate_request.writer_uid(),
            producer_pid: self.candidate_request.producer.pid(),
            producer_uid: self.candidate_request.producer.uid(),
            producer_gid: self.candidate_request.producer.gid(),
            producer_start_ticks: self.candidate_request.producer.start_ticks(),
            credential_boundary: "kernel-verified-seqpacket-peer-distinct-from-writer".to_string(),
            candidate_request_binding: "verified-packet-to-retained-descriptor-exact-bytes"
                .to_string(),
            machine_id_binding: "retained-descriptor-and-fixed-path".to_string(),
            attempt_roster: "closed-world-journal-only".to_string(),
            activation_allowed: ClosedAuthorityFlagV8::closed_v8(),
            recovery_effect_allowed: ClosedAuthorityFlagV8::closed_v8(),
            barrier_release_allowed: ClosedAuthorityFlagV8::closed_v8(),
            authority: "read-only-admission-no-authority".to_string(),
        })
    }
}

pub(crate) fn bind_privileged_lifecycle_no_authority_v9(
    replay: VerifiedDescriptorBoundReplayV8,
    candidate_request: RetainedCandidateRequestV9,
) -> Result<PrivilegedLifecycleNoAuthorityV9, NativeErrorV8> {
    replay.revalidate()?;
    candidate_request.revalidate()?;
    if !candidate_request.matches_writer_root_v9(replay.retained_root_identity_v8()) {
        return Err(invalid(
            "privileged lifecycle writer differs from the retained state-root owner",
        ));
    }
    let descriptor = replay.assess_read_only()?;
    require_candidate_request_binding_v9(&descriptor, &candidate_request)?;
    candidate_request.revalidate()?;
    replay.revalidate()?;
    Ok(PrivilegedLifecycleNoAuthorityV9 {
        candidate_request,
        replay,
    })
}

/// Serializable evidence only. All fields are private, and each authority
/// field is the unforgeable false-only wire type shared with v1/v2.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PrivilegedLifecycleAssessmentV9 {
    schema: String,
    descriptor_bound_replay: DescriptorBoundRuntimeAssessmentV8,
    candidate_request_sha256: String,
    writer_uid: u32,
    producer_pid: u32,
    producer_uid: u32,
    producer_gid: u32,
    producer_start_ticks: u64,
    credential_boundary: String,
    candidate_request_binding: String,
    machine_id_binding: String,
    attempt_roster: String,
    activation_allowed: ClosedAuthorityFlagV8,
    recovery_effect_allowed: ClosedAuthorityFlagV8,
    barrier_release_allowed: ClosedAuthorityFlagV8,
    authority: String,
}

impl PrivilegedLifecycleAssessmentV9 {
    pub fn descriptor_bound_replay(&self) -> &DescriptorBoundRuntimeAssessmentV8 {
        &self.descriptor_bound_replay
    }

    pub fn candidate_request_sha256(&self) -> &str {
        &self.candidate_request_sha256
    }

    pub fn authority(&self) -> &str {
        &self.authority
    }
}

fn require_candidate_request_binding_v9(
    descriptor: &DescriptorBoundRuntimeAssessmentV8,
    candidate_request: &RetainedCandidateRequestV9,
) -> Result<(), NativeErrorV8> {
    if descriptor.candidate_execution_request_sha256.as_deref()
        != Some(candidate_request.request_sha256())
        || !descriptor.activation_allowed.is_closed()
        || !descriptor.recovery_effect_allowed.is_closed()
        || !descriptor.barrier_release_allowed.is_closed()
    {
        return Err(invalid(
            "descriptor-bound replay does not bind the retained candidate request with all authority closed",
        ));
    }
    Ok(())
}

fn require_exact_request_roster_v9(
    namespace: &DirectoryAnchorV8,
    request_leaf: &OsStr,
) -> Result<(), NativeErrorV8> {
    let names = namespace.list_leaf_names_bounded(2)?;
    if names.len() != 1 || names[0] != request_leaf {
        return Err(invalid(
            "candidate-request namespace must contain exactly the retained request leaf",
        ));
    }
    Ok(())
}

fn require_writer_namespace_v9(
    identity: FileIdentityV8,
    writer_uid: u32,
    writer_gid: u32,
) -> Result<(), NativeErrorV8> {
    if identity.device() == 0
        || identity.inode() == 0
        || identity.link_count() == 0
        || identity.owner_uid() != writer_uid
        || identity.owner_gid() != writer_gid
        || identity.mode() != 0o700
    {
        return Err(invalid(
            "candidate-request namespace is not owned exclusively by the writer credential",
        ));
    }
    Ok(())
}

fn require_request_identity_v9(
    request: FileIdentityV8,
    request_metadata: TrustedNodeMetadataV8,
    namespace: FileIdentityV8,
    namespace_metadata: TrustedNodeMetadataV8,
    writer_uid: u32,
    writer_gid: u32,
) -> Result<(), NativeErrorV8> {
    if request.device() != namespace.device()
        || request.inode() == 0
        || request.link_count() != 1
        || request.owner_uid() != writer_uid
        || request.owner_gid() != writer_gid
        || request.mode() != 0o400
        || request.size() == 0
        || request.size() > MAX_CANDIDATE_REQUEST_BYTES_V9
        || !request_metadata.matches_filesystem_domain(namespace_metadata)
    {
        return Err(invalid(
            "candidate-request leaf identity or filesystem domain is not exact",
        ));
    }
    Ok(())
}

fn require_live_separated_producer_v9(
    producer: &VerifiedPeerV8,
    writer_uid: u32,
) -> Result<(), NativeErrorV8> {
    if producer.uid() == writer_uid {
        return Err(invalid(
            "kernel-authenticated candidate producer and privileged writer must use distinct UIDs",
        ));
    }
    let exited = producer.process_exited().map_err(|error| {
        invalid(format!(
            "kernel-authenticated candidate producer could not be revalidated: {error}"
        ))
    })?;
    if exited {
        return Err(invalid(
            "kernel-authenticated candidate producer exited before admission replay completed",
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn effective_ids_v9() -> Result<(u32, u32), NativeErrorV8> {
    // SAFETY: geteuid/getegid have no arguments or preconditions.
    Ok((unsafe { libc::geteuid() }, unsafe { libc::getegid() }))
}

#[cfg(not(target_os = "linux"))]
fn effective_ids_v9() -> Result<(u32, u32), NativeErrorV8> {
    Err(invalid(
        "privileged lifecycle candidate-request retention requires Linux",
    ))
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use std::fs;
    use std::io::BufRead as _;
    use std::io::BufReader;
    use std::io::Read as _;
    use std::io::Write as _;
    use std::os::unix::fs::OpenOptionsExt as _;
    use std::os::unix::fs::PermissionsExt as _;
    use std::os::unix::process::CommandExt as _;
    use std::process::Command;
    use std::process::Stdio;

    use super::*;

    fn request_fixture(label: &str) -> (tempfile::TempDir, PathBuf, OsString, Vec<u8>) {
        let temporary = tempfile::tempdir().unwrap();
        let namespace = temporary.path().join(format!("request-{label}"));
        fs::create_dir(&namespace).unwrap();
        fs::set_permissions(&namespace, fs::Permissions::from_mode(0o700)).unwrap();
        let leaf = OsString::from("candidate-request.bin");
        let bytes = format!("exact candidate request {label}\n").into_bytes();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o400)
            .open(namespace.join(&leaf))
            .unwrap();
        file.write_all(&bytes).unwrap();
        file.sync_all().unwrap();
        drop(file);
        fs::set_permissions(namespace.join(&leaf), fs::Permissions::from_mode(0o400)).unwrap();
        (temporary, namespace, leaf, bytes)
    }

    fn same_uid_verified_packet(socket_path: &Path, payload: &[u8]) -> VerifiedPacketV8 {
        let listener = crate::SeqpacketListenerV8::bind(socket_path).unwrap();
        let sender_path = socket_path.to_path_buf();
        let sender_payload = payload.to_vec();
        let sender = std::thread::spawn(move || {
            crate::connect_seqpacket_v8(&sender_path)
                .unwrap()
                .send_one_request(&sender_payload, &[])
                .unwrap();
        });
        let packet = listener
            .accept()
            .unwrap()
            .receive_one_request(crate::ExactFileDescriptorCountV8::new(0).unwrap())
            .unwrap();
        sender.join().unwrap();
        packet
    }

    #[test]
    fn retained_request_rejects_same_uid_producer_and_open_namespace() {
        let (temporary, namespace, leaf, bytes) = request_fixture("credentials");
        let packet = same_uid_verified_packet(&temporary.path().join("producer.sock"), &bytes);
        assert!(open_credential_separated_candidate_request_v9(&namespace, &leaf, packet).is_err());
        assert!(
            open_retained_candidate_request_file_v9(&namespace, &leaf, b"different packet bytes",)
                .is_err()
        );

        fs::set_permissions(&namespace, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(open_retained_candidate_request_file_v9(&namespace, &leaf, &bytes).is_err());
    }

    #[test]
    fn privileged_cross_uid_peer_is_kernel_bound_and_retained() {
        const CHILD_SOCKET_ENV: &str = "HEPTA_V9_CROSS_UID_CHILD_SOCKET";
        const CHILD_PAYLOAD_ENV: &str = "HEPTA_V9_CROSS_UID_CHILD_PAYLOAD";
        const PRODUCER_UID: u32 = 65_534;
        const PRODUCER_GID: u32 = 65_534;

        if let (Ok(socket), Ok(payload)) = (
            std::env::var(CHILD_SOCKET_ENV),
            std::env::var(CHILD_PAYLOAD_ENV),
        ) {
            crate::connect_seqpacket_v8(Path::new(&socket))
                .unwrap()
                .send_one_request(payload.as_bytes(), &[])
                .unwrap();
            println!("HEPTA_V9_CROSS_UID_READY");
            std::io::stdout().flush().unwrap();
            let mut keepalive = Vec::new();
            std::io::stdin().read_to_end(&mut keepalive).unwrap();
            return;
        }

        if unsafe { libc::geteuid() } != 0 {
            eprintln!("SKIP: privileged cross-UID positive fixture requires euid 0");
            return;
        }

        let (temporary, namespace, leaf, bytes) = request_fixture("cross-uid");
        fs::set_permissions(temporary.path(), fs::Permissions::from_mode(0o755)).unwrap();
        let socket_path = temporary.path().join("producer.sock");
        let listener = crate::SeqpacketListenerV8::bind(&socket_path).unwrap();
        fs::set_permissions(&socket_path, fs::Permissions::from_mode(0o777)).unwrap();
        let payload = String::from_utf8(bytes.clone()).unwrap();
        let current_exe = std::env::current_exe().unwrap();
        let mut child = Command::new(current_exe)
            .arg("privileged_cross_uid_peer_is_kernel_bound_and_retained")
            .arg("--nocapture")
            .env(CHILD_SOCKET_ENV, &socket_path)
            .env(CHILD_PAYLOAD_ENV, &payload)
            .uid(PRODUCER_UID)
            .gid(PRODUCER_GID)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut child_stdout = BufReader::new(child.stdout.take().unwrap());
        let mut readiness_log = String::new();
        let mut ready = false;
        for _ in 0..32 {
            let mut line = String::new();
            if child_stdout.read_line(&mut line).unwrap() == 0 {
                break;
            }
            readiness_log.push_str(&line);
            if line.contains("HEPTA_V9_CROSS_UID_READY") {
                ready = true;
                break;
            }
        }
        if !ready {
            drop(child.stdin.take());
            child_stdout.read_to_string(&mut readiness_log).unwrap();
            let output = child.wait_with_output().unwrap();
            panic!(
                "cross-UID child did not become ready: stdout={readiness_log:?} stderr={}",
                String::from_utf8_lossy(&output.stderr),
            );
        }
        let packet = listener
            .accept()
            .unwrap()
            .receive_one_request(crate::ExactFileDescriptorCountV8::new(0).unwrap())
            .unwrap();
        let retained =
            open_credential_separated_candidate_request_v9(&namespace, &leaf, packet).unwrap();
        assert_eq!(retained.writer_uid(), 0);
        assert_eq!(retained.producer_uid(), PRODUCER_UID);
        assert_eq!(retained.producer.gid(), PRODUCER_GID);
        assert!(retained.producer.start_ticks() > 0);
        assert!(!retained.producer.process_exited().unwrap());
        retained.revalidate().unwrap();

        drop(child.stdin.take());
        child_stdout.read_to_string(&mut readiness_log).unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "cross-UID child failed: stdout={} stderr={}",
            readiness_log,
            String::from_utf8_lossy(&output.stderr),
        );
    }

    #[test]
    fn retained_request_rejects_siblings_replacement_and_byte_drift() {
        for mutation in ["sibling", "replacement", "bytes"] {
            let (_temporary, namespace, leaf, bytes) = request_fixture(mutation);
            let retained =
                open_retained_candidate_request_file_v9(&namespace, &leaf, &bytes).unwrap();
            retained.revalidate().unwrap();

            match mutation {
                "sibling" => fs::write(namespace.join("shadow"), b"shadow").unwrap(),
                "replacement" => {
                    fs::remove_file(namespace.join(&leaf)).unwrap();
                    let mut replacement = fs::OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .mode(0o400)
                        .open(namespace.join(&leaf))
                        .unwrap();
                    replacement.write_all(&bytes).unwrap();
                    replacement.sync_all().unwrap();
                    drop(replacement);
                    fs::set_permissions(namespace.join(&leaf), fs::Permissions::from_mode(0o400))
                        .unwrap();
                }
                "bytes" => {
                    fs::set_permissions(namespace.join(&leaf), fs::Permissions::from_mode(0o600))
                        .unwrap();
                    let mut changed = bytes;
                    changed[0] ^= 0xff;
                    fs::write(namespace.join(&leaf), changed).unwrap();
                    fs::set_permissions(namespace.join(&leaf), fs::Permissions::from_mode(0o400))
                        .unwrap();
                }
                _ => unreachable!(),
            }
            assert!(retained.revalidate().is_err(), "mutation={mutation}");
        }
    }
}

use std::ffi::OsStr;
use std::fmt;
use std::path::Path;

use sha2::Digest as _;

use crate::DirectoryAnchorV8;
use crate::FileIdentityV8;
use crate::NativeErrorV8;
use crate::StateRootLockV8;
use crate::invalid;

use super::DurablePublicationCheckpointV8;
use super::PublishedRecordV8;
use super::TrustedStateRootV8;
use super::publish_record_noreplace_observed_v8;
use super::validate_digest;

pub const INSTALL_EPOCH_EVENT_DIGITS_V1: usize = 20;
pub const INSTALL_EPOCH_STORE_MAX_EVENTS_V1: usize = 4_096;
const INSTALL_EPOCH_EVENT_SCHEMA_V1: &[u8] = b"hepta_linux_v8_install_epoch_store_event_v1\0";
const MAX_EVENT_PAYLOAD_BYTES_V1: u64 = 16 * 1024 * 1024;
const MAX_EVENT_RECORD_BYTES_V1: u64 = MAX_EVENT_PAYLOAD_BYTES_V1 + 2 * 1024;
const MAX_STORE_BYTES_V1: u64 = 512 * 1024 * 1024;
const ZERO_SHA256: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// Closed structural roster for the local install-epoch/outbox log. Semantic
/// verification of provider requests, fences, receipts, and signatures stays
/// in the future bridge crate; this enum merely prevents untyped log entries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstallEpochStoreEventKindV1 {
    Intent,
    CasOutbox,
    /// Reserved encoding only. It is not admitted by the v1 transition graph;
    /// the durable CasOutbox publication is itself the dispatch fence.
    CasFence,
    CasReceipt,
    CurrentTipOutbox,
    /// Reserved encoding only. It is not admitted by the v1 transition graph.
    CurrentTipFence,
    /// Reserved encoding only. The signed current-tip receipt is carried by a
    /// retry CurrentTipOutbox or the terminal Final payload.
    CurrentTipReceipt,
    /// Reserved encoding only. A retry CurrentTipOutbox atomically carries the
    /// preceding closure and the next exact query identity.
    QueryClosed,
    Final,
    Quarantine,
}

impl InstallEpochStoreEventKindV1 {
    fn encoded(self) -> u8 {
        match self {
            Self::Intent => 1,
            Self::CasOutbox => 2,
            Self::CasFence => 3,
            Self::CasReceipt => 4,
            Self::CurrentTipOutbox => 5,
            Self::CurrentTipFence => 6,
            Self::CurrentTipReceipt => 7,
            Self::QueryClosed => 8,
            Self::Final => 9,
            Self::Quarantine => 10,
        }
    }

    fn decode(value: u8) -> Result<Self, NativeErrorV8> {
        match value {
            1 => Ok(Self::Intent),
            2 => Ok(Self::CasOutbox),
            3 => Ok(Self::CasFence),
            4 => Ok(Self::CasReceipt),
            5 => Ok(Self::CurrentTipOutbox),
            6 => Ok(Self::CurrentTipFence),
            7 => Ok(Self::CurrentTipReceipt),
            8 => Ok(Self::QueryClosed),
            9 => Ok(Self::Final),
            10 => Ok(Self::Quarantine),
            _ => Err(invalid("install-epoch event kind is unknown")),
        }
    }

    fn permits_successor(self, next: Self) -> bool {
        match self {
            Self::Intent => matches!(next, Self::CasOutbox | Self::Quarantine),
            Self::CasOutbox => matches!(next, Self::CasReceipt | Self::Quarantine),
            Self::CasReceipt => matches!(next, Self::CurrentTipOutbox | Self::Quarantine),
            Self::CurrentTipOutbox => matches!(
                next,
                Self::CurrentTipOutbox | Self::Final | Self::Quarantine
            ),
            Self::CasFence
            | Self::CurrentTipFence
            | Self::CurrentTipReceipt
            | Self::QueryClosed
            | Self::Final
            | Self::Quarantine => false,
        }
    }
}

/// Canonical structural event. Construction is crate-private because shape
/// validation alone cannot bind the opaque payload to the qualification-owned
/// durable projection. A future bridge must consume its verified token before
/// exposing any append capability.
#[derive(Eq, PartialEq)]
pub(crate) struct InstallEpochStoreRecordV1 {
    completion_profile_sha256: String,
    completion_slot_sha256: String,
    epoch_sequence: u64,
    event_kind: InstallEpochStoreEventKindV1,
    event_sequence: u64,
    job_id_sha256: String,
    layout_manifest_sha256: String,
    machine_id_sha256: String,
    operation_binding_sha256: String,
    payload: Vec<u8>,
    phase_head_sha256: String,
    phase_predecessor_revision: u64,
    phase_predecessor_state_sha256: String,
    phase_successor_revision: u64,
    phase_successor_state_sha256: String,
    prepared_epoch_binding_sha256: String,
    previous_event_sha256: String,
    provider_profile_sha256: String,
    state_root_profile_sha256: String,
    stream_id_sha256: String,
}

impl InstallEpochStoreRecordV1 {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        completion_profile_sha256: String,
        completion_slot_sha256: String,
        epoch_sequence: u64,
        event_kind: InstallEpochStoreEventKindV1,
        event_sequence: u64,
        job_id_sha256: String,
        layout_manifest_sha256: String,
        machine_id_sha256: String,
        operation_binding_sha256: String,
        payload: Vec<u8>,
        phase_head_sha256: String,
        phase_predecessor_revision: u64,
        phase_predecessor_state_sha256: String,
        phase_successor_revision: u64,
        phase_successor_state_sha256: String,
        prepared_epoch_binding_sha256: String,
        previous_event_sha256: String,
        provider_profile_sha256: String,
        state_root_profile_sha256: String,
        stream_id_sha256: String,
    ) -> Result<Self, NativeErrorV8> {
        let record = Self {
            completion_profile_sha256,
            completion_slot_sha256,
            epoch_sequence,
            event_kind,
            event_sequence,
            job_id_sha256,
            layout_manifest_sha256,
            machine_id_sha256,
            operation_binding_sha256,
            payload,
            phase_head_sha256,
            phase_predecessor_revision,
            phase_predecessor_state_sha256,
            phase_successor_revision,
            phase_successor_state_sha256,
            prepared_epoch_binding_sha256,
            previous_event_sha256,
            provider_profile_sha256,
            state_root_profile_sha256,
            stream_id_sha256,
        };
        record.validate()?;
        Ok(record)
    }

    pub(crate) fn event_kind(&self) -> InstallEpochStoreEventKindV1 {
        self.event_kind
    }

    fn validate(&self) -> Result<(), NativeErrorV8> {
        for (label, digest) in [
            ("completion profile", &self.completion_profile_sha256),
            ("completion slot", &self.completion_slot_sha256),
            ("install job", &self.job_id_sha256),
            ("layout manifest", &self.layout_manifest_sha256),
            ("machine", &self.machine_id_sha256),
            ("operation binding", &self.operation_binding_sha256),
            ("phase head", &self.phase_head_sha256),
            ("phase successor state", &self.phase_successor_state_sha256),
            ("prepared epoch", &self.prepared_epoch_binding_sha256),
            ("provider profile", &self.provider_profile_sha256),
            ("state-root profile", &self.state_root_profile_sha256),
            ("stream", &self.stream_id_sha256),
        ] {
            validate_digest(label, digest)?;
        }
        if self.event_sequence == 0 || self.epoch_sequence == 0 {
            return Err(invalid(
                "install-epoch event and epoch sequences must be non-zero",
            ));
        }
        if self.event_sequence == 1 {
            if self.previous_event_sha256 != ZERO_SHA256 {
                return Err(invalid(
                    "first install-epoch event must bind the zero predecessor",
                ));
            }
        } else {
            validate_digest("previous install-epoch event", &self.previous_event_sha256)?;
        }
        if self.phase_successor_revision
            != self
                .phase_predecessor_revision
                .checked_add(1)
                .ok_or_else(|| invalid("install-epoch phase revision overflows"))?
        {
            return Err(invalid(
                "install-epoch phase transition must advance exactly once",
            ));
        }
        if self.phase_predecessor_revision == 0 {
            if self.phase_predecessor_state_sha256 != ZERO_SHA256 {
                return Err(invalid(
                    "install-epoch genesis phase must use the zero predecessor",
                ));
            }
        } else {
            validate_digest(
                "install-epoch phase predecessor state",
                &self.phase_predecessor_state_sha256,
            )?;
        }
        if self.payload.is_empty()
            || u64::try_from(self.payload.len())
                .map_err(|_| invalid("install-epoch event payload length overflows"))?
                > MAX_EVENT_PAYLOAD_BYTES_V1
        {
            return Err(invalid(
                "install-epoch event payload is empty or exceeds its frozen limit",
            ));
        }
        Ok(())
    }

    pub(crate) fn canonical_bytes(&self) -> Result<Vec<u8>, NativeErrorV8> {
        self.validate()?;
        let mut bytes = INSTALL_EPOCH_EVENT_SCHEMA_V1.to_vec();
        for value in [
            &self.job_id_sha256,
            &self.machine_id_sha256,
            &self.state_root_profile_sha256,
            &self.layout_manifest_sha256,
            &self.provider_profile_sha256,
            &self.completion_profile_sha256,
            &self.stream_id_sha256,
            &self.prepared_epoch_binding_sha256,
            &self.completion_slot_sha256,
        ] {
            bytes.extend_from_slice(value.as_bytes());
        }
        bytes.extend_from_slice(&self.epoch_sequence.to_be_bytes());
        bytes.extend_from_slice(&self.event_sequence.to_be_bytes());
        bytes.extend_from_slice(self.previous_event_sha256.as_bytes());
        bytes.push(self.event_kind.encoded());
        bytes.extend_from_slice(self.phase_head_sha256.as_bytes());
        bytes.extend_from_slice(&self.phase_predecessor_revision.to_be_bytes());
        bytes.extend_from_slice(self.phase_predecessor_state_sha256.as_bytes());
        bytes.extend_from_slice(&self.phase_successor_revision.to_be_bytes());
        bytes.extend_from_slice(self.phase_successor_state_sha256.as_bytes());
        bytes.extend_from_slice(self.operation_binding_sha256.as_bytes());
        bytes.extend_from_slice(
            &u64::try_from(self.payload.len())
                .map_err(|_| invalid("install-epoch payload length overflows"))?
                .to_be_bytes(),
        );
        bytes.extend_from_slice(&self.payload);
        Ok(bytes)
    }

    pub(crate) fn record_sha256(&self) -> Result<String, NativeErrorV8> {
        Ok(format!(
            "{:x}",
            sha2::Sha256::digest(self.canonical_bytes()?)
        ))
    }

    pub(crate) fn decode_exact(bytes: &[u8]) -> Result<Self, NativeErrorV8> {
        if !bytes.starts_with(INSTALL_EPOCH_EVENT_SCHEMA_V1) {
            return Err(invalid("install-epoch event schema is not exact"));
        }
        let mut offset = INSTALL_EPOCH_EVENT_SCHEMA_V1.len();
        let job_id_sha256 = take_digest_v1(bytes, &mut offset, "job")?;
        let machine_id_sha256 = take_digest_v1(bytes, &mut offset, "machine")?;
        let state_root_profile_sha256 = take_digest_v1(bytes, &mut offset, "state root")?;
        let layout_manifest_sha256 = take_digest_v1(bytes, &mut offset, "layout")?;
        let provider_profile_sha256 = take_digest_v1(bytes, &mut offset, "provider")?;
        let completion_profile_sha256 = take_digest_v1(bytes, &mut offset, "completion")?;
        let stream_id_sha256 = take_digest_v1(bytes, &mut offset, "stream")?;
        let prepared_epoch_binding_sha256 = take_digest_v1(bytes, &mut offset, "prepared epoch")?;
        let completion_slot_sha256 = take_digest_v1(bytes, &mut offset, "completion slot")?;
        let epoch_sequence = take_u64_v1(bytes, &mut offset)?;
        let event_sequence = take_u64_v1(bytes, &mut offset)?;
        let previous_event_sha256 = take_ascii_v1(bytes, &mut offset, 64, "previous event")?;
        let event_kind = InstallEpochStoreEventKindV1::decode(take_u8_v1(bytes, &mut offset)?)?;
        let phase_head_sha256 = take_digest_v1(bytes, &mut offset, "phase head")?;
        let phase_predecessor_revision = take_u64_v1(bytes, &mut offset)?;
        let phase_predecessor_state_sha256 =
            take_ascii_v1(bytes, &mut offset, 64, "phase predecessor")?;
        let phase_successor_revision = take_u64_v1(bytes, &mut offset)?;
        let phase_successor_state_sha256 = take_digest_v1(bytes, &mut offset, "phase successor")?;
        let operation_binding_sha256 = take_digest_v1(bytes, &mut offset, "operation")?;
        let payload_length = usize::try_from(take_u64_v1(bytes, &mut offset)?)
            .map_err(|_| invalid("install-epoch payload length does not fit usize"))?;
        let end = offset
            .checked_add(payload_length)
            .ok_or_else(|| invalid("install-epoch payload offset overflows"))?;
        let payload = bytes
            .get(offset..end)
            .ok_or_else(|| invalid("install-epoch event payload is truncated"))?
            .to_vec();
        offset = end;
        if offset != bytes.len() {
            return Err(invalid("install-epoch event has trailing bytes"));
        }
        let record = Self::new(
            completion_profile_sha256,
            completion_slot_sha256,
            epoch_sequence,
            event_kind,
            event_sequence,
            job_id_sha256,
            layout_manifest_sha256,
            machine_id_sha256,
            operation_binding_sha256,
            payload,
            phase_head_sha256,
            phase_predecessor_revision,
            phase_predecessor_state_sha256,
            phase_successor_revision,
            phase_successor_state_sha256,
            prepared_epoch_binding_sha256,
            previous_event_sha256,
            provider_profile_sha256,
            state_root_profile_sha256,
            stream_id_sha256,
        )?;
        if record.canonical_bytes()? != bytes {
            return Err(invalid("install-epoch event encoding is not canonical"));
        }
        Ok(record)
    }
}

impl fmt::Debug for InstallEpochStoreRecordV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let record_sha256 = self
            .record_sha256()
            .unwrap_or_else(|_| "<invalid-record>".to_string());
        formatter
            .debug_struct("InstallEpochStoreRecordV1")
            .field("event_kind", &self.event_kind)
            .field("event_sequence", &self.event_sequence)
            .field("epoch_sequence", &self.epoch_sequence)
            .field("record_sha256", &record_sha256)
            .field("payload_length", &self.payload.len())
            .finish_non_exhaustive()
    }
}

/// Borrowed, read-only terminal record recovered from a complete anchored
/// replay. It cannot be constructed, cloned, deserialized, or converted back
/// into raw append authority by callers.
pub struct RecoveredInstallEpochStoreTipV1<'a> {
    record: &'a InstallEpochStoreRecordV1,
    record_sha256: &'a str,
}

impl fmt::Debug for RecoveredInstallEpochStoreTipV1<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RecoveredInstallEpochStoreTipV1")
            .field("event_kind", &self.event_kind())
            .field("event_sequence", &self.event_sequence())
            .field("epoch_sequence", &self.epoch_sequence())
            .field("record_sha256", &self.record_sha256)
            .field("payload_length", &self.payload().len())
            .finish_non_exhaustive()
    }
}

impl RecoveredInstallEpochStoreTipV1<'_> {
    pub fn completion_profile_sha256(&self) -> &str {
        &self.record.completion_profile_sha256
    }

    pub fn completion_slot_sha256(&self) -> &str {
        &self.record.completion_slot_sha256
    }

    pub fn epoch_sequence(&self) -> u64 {
        self.record.epoch_sequence
    }

    pub fn event_kind(&self) -> InstallEpochStoreEventKindV1 {
        self.record.event_kind
    }

    pub fn event_sequence(&self) -> u64 {
        self.record.event_sequence
    }

    pub fn job_id_sha256(&self) -> &str {
        &self.record.job_id_sha256
    }

    pub fn layout_manifest_sha256(&self) -> &str {
        &self.record.layout_manifest_sha256
    }

    pub fn machine_id_sha256(&self) -> &str {
        &self.record.machine_id_sha256
    }

    pub fn operation_binding_sha256(&self) -> &str {
        &self.record.operation_binding_sha256
    }

    pub fn payload(&self) -> &[u8] {
        &self.record.payload
    }

    pub fn phase_head_sha256(&self) -> &str {
        &self.record.phase_head_sha256
    }

    pub fn phase_predecessor_revision(&self) -> u64 {
        self.record.phase_predecessor_revision
    }

    pub fn phase_predecessor_state_sha256(&self) -> &str {
        &self.record.phase_predecessor_state_sha256
    }

    pub fn phase_successor_revision(&self) -> u64 {
        self.record.phase_successor_revision
    }

    pub fn phase_successor_state_sha256(&self) -> &str {
        &self.record.phase_successor_state_sha256
    }

    pub fn prepared_epoch_binding_sha256(&self) -> &str {
        &self.record.prepared_epoch_binding_sha256
    }

    pub fn previous_event_sha256(&self) -> &str {
        &self.record.previous_event_sha256
    }

    pub fn provider_profile_sha256(&self) -> &str {
        &self.record.provider_profile_sha256
    }

    pub fn record_sha256(&self) -> &str {
        self.record_sha256
    }

    pub fn state_root_profile_sha256(&self) -> &str {
        &self.record.state_root_profile_sha256
    }

    pub fn stream_id_sha256(&self) -> &str {
        &self.record.stream_id_sha256
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, NativeErrorV8> {
        self.record.canonical_bytes()
    }
}

/// Exhaustive replay disposition. Incoming residue is represented in the
/// type, so recovery callers cannot accidentally treat a committed prefix as
/// a clean terminal state.
#[derive(Debug)]
pub enum InstallEpochStoreRecoveryViewV1<'a> {
    Empty,
    Interrupted {
        committed_tip: Option<RecoveredInstallEpochStoreTipV1<'a>>,
    },
    Clean {
        tip: RecoveredInstallEpochStoreTipV1<'a>,
    },
}

/// Exact anchored replay of the complete immutable store. This is structural
/// evidence only and cannot authorize provider dispatch or root installation.
pub struct VerifiedInstallEpochStoreScanV1 {
    event_count: u64,
    incoming_residue_detected: bool,
    last_record: Option<InstallEpochStoreRecordV1>,
    state_root_identity: FileIdentityV8,
    tip_sha256: String,
    total_bytes: u64,
}

impl VerifiedInstallEpochStoreScanV1 {
    pub fn event_count(&self) -> u64 {
        self.event_count
    }

    pub fn incoming_residue_detected(&self) -> bool {
        self.incoming_residue_detected
    }

    pub fn state_root_identity(&self) -> FileIdentityV8 {
        self.state_root_identity
    }

    pub fn tip_sha256(&self) -> &str {
        &self.tip_sha256
    }

    pub fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    pub fn last_event_kind(&self) -> Option<InstallEpochStoreEventKindV1> {
        self.last_record
            .as_ref()
            .map(InstallEpochStoreRecordV1::event_kind)
    }

    pub fn recovery_view(&self) -> InstallEpochStoreRecoveryViewV1<'_> {
        match (self.incoming_residue_detected, self.last_record.as_ref()) {
            (false, None) => InstallEpochStoreRecoveryViewV1::Empty,
            (true, None) => InstallEpochStoreRecoveryViewV1::Interrupted {
                committed_tip: None,
            },
            (true, Some(record)) => InstallEpochStoreRecoveryViewV1::Interrupted {
                committed_tip: Some(RecoveredInstallEpochStoreTipV1 {
                    record,
                    record_sha256: &self.tip_sha256,
                }),
            },
            (false, Some(record)) => InstallEpochStoreRecoveryViewV1::Clean {
                tip: RecoveredInstallEpochStoreTipV1 {
                    record,
                    record_sha256: &self.tip_sha256,
                },
            },
        }
    }
}

impl fmt::Debug for VerifiedInstallEpochStoreScanV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VerifiedInstallEpochStoreScanV1")
            .field("event_count", &self.event_count)
            .field("incoming_residue_detected", &self.incoming_residue_detected)
            .field("state_root_identity", &self.state_root_identity)
            .field("tip_sha256", &self.tip_sha256)
            .field("total_bytes", &self.total_bytes)
            .field("last_event_kind", &self.last_event_kind())
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub struct FreshInstallEpochStoreAppendV1 {
    publication: PublishedRecordV8,
    verified_store: VerifiedInstallEpochStoreScanV1,
}

impl FreshInstallEpochStoreAppendV1 {
    pub fn publication(&self) -> &PublishedRecordV8 {
        &self.publication
    }

    pub fn verified_store(&self) -> &VerifiedInstallEpochStoreScanV1 {
        &self.verified_store
    }
}

#[derive(Debug)]
pub struct ExactExistingInstallEpochStoreEventV1 {
    historical: bool,
    identity: FileIdentityV8,
    record_sha256: String,
    verified_store: VerifiedInstallEpochStoreScanV1,
}

impl ExactExistingInstallEpochStoreEventV1 {
    pub fn historical(&self) -> bool {
        self.historical
    }

    pub fn identity(&self) -> FileIdentityV8 {
        self.identity
    }

    pub fn record_sha256(&self) -> &str {
        &self.record_sha256
    }

    pub fn verified_store(&self) -> &VerifiedInstallEpochStoreScanV1 {
        &self.verified_store
    }
}

#[derive(Debug)]
pub struct InstallEpochStoreRecoveryRequiredV1 {
    verified_store: VerifiedInstallEpochStoreScanV1,
}

impl InstallEpochStoreRecoveryRequiredV1 {
    pub fn verified_store(&self) -> &VerifiedInstallEpochStoreScanV1 {
        &self.verified_store
    }
}

#[derive(Debug)]
pub enum InstallEpochStoreAppendOutcomeV1 {
    Fresh(FreshInstallEpochStoreAppendV1),
    ExactExisting(ExactExistingInstallEpochStoreEventV1),
    RecoveryRequired(InstallEpochStoreRecoveryRequiredV1),
}

#[derive(Debug, thiserror::Error)]
pub enum InstallEpochStoreErrorV1 {
    #[error("invalid install-epoch store request: {0}")]
    InvalidRequest(#[source] NativeErrorV8),
    #[error("install-epoch store conflict requires hold: {0}")]
    ConflictHold(#[source] NativeErrorV8),
    #[error("install-epoch store mutation requires recovery: {0}")]
    RecoveryRequired(#[source] NativeErrorV8),
}

impl InstallEpochStoreErrorV1 {
    pub fn requires_recovery_or_hold(&self) -> bool {
        !matches!(self, Self::InvalidRequest(_))
    }
}

pub fn scan_install_epoch_store_v1(
    trusted_root: &mut TrustedStateRootV8,
) -> Result<VerifiedInstallEpochStoreScanV1, InstallEpochStoreErrorV1> {
    trusted_root
        .revalidate()
        .map_err(InstallEpochStoreErrorV1::RecoveryRequired)?;
    let root_identity = trusted_root.identity();
    let verified = {
        let (root, install_epoch, lock) = trusted_root.split_for_store_v8();
        scan_install_epoch_store_anchored_v1(root, install_epoch, lock, root_identity)
            .map_err(InstallEpochStoreErrorV1::RecoveryRequired)?
    };
    trusted_root
        .revalidate()
        .map_err(InstallEpochStoreErrorV1::RecoveryRequired)?;
    Ok(verified)
}

#[cfg_attr(
    not(all(test, target_os = "linux")),
    allow(
        dead_code,
        reason = "the raw append path stays private until the verified projection bridge is published"
    )
)]
pub(crate) fn append_install_epoch_store_event_v1(
    trusted_root: &mut TrustedStateRootV8,
    record: &InstallEpochStoreRecordV1,
    publication_nonce: &str,
) -> Result<InstallEpochStoreAppendOutcomeV1, InstallEpochStoreErrorV1> {
    append_install_epoch_store_event_observed_v1(trusted_root, record, publication_nonce, |_| {})
}

#[cfg_attr(
    not(all(test, target_os = "linux")),
    allow(
        dead_code,
        reason = "the crash observer stays private with the unpublished raw append path"
    )
)]
fn append_install_epoch_store_event_observed_v1<F>(
    trusted_root: &mut TrustedStateRootV8,
    record: &InstallEpochStoreRecordV1,
    publication_nonce: &str,
    mut observe: F,
) -> Result<InstallEpochStoreAppendOutcomeV1, InstallEpochStoreErrorV1>
where
    F: FnMut(DurablePublicationCheckpointV8),
{
    record
        .validate()
        .map_err(InstallEpochStoreErrorV1::InvalidRequest)?;
    validate_digest("install-epoch publication nonce", publication_nonce)
        .map_err(InstallEpochStoreErrorV1::InvalidRequest)?;
    if record.machine_id_sha256 != trusted_root.machine_id_sha256()
        || record.state_root_profile_sha256 != trusted_root.profile_sha256()
        || record.layout_manifest_sha256 != trusted_root.layout_manifest_sha256()
    {
        return Err(InstallEpochStoreErrorV1::InvalidRequest(invalid(
            "install-epoch event differs from the trusted root binding",
        )));
    }
    trusted_root
        .revalidate()
        .map_err(InstallEpochStoreErrorV1::RecoveryRequired)?;
    let root_identity = trusted_root.identity();
    let (root, install_epoch, lock) = trusted_root.split_for_store_v8();
    let before = scan_install_epoch_store_anchored_v1(root, install_epoch, lock, root_identity)
        .map_err(InstallEpochStoreErrorV1::RecoveryRequired)?;
    if before.incoming_residue_detected {
        let outcome = InstallEpochStoreAppendOutcomeV1::RecoveryRequired(
            InstallEpochStoreRecoveryRequiredV1 {
                verified_store: before,
            },
        );
        trusted_root
            .revalidate()
            .map_err(InstallEpochStoreErrorV1::RecoveryRequired)?;
        return Ok(outcome);
    }
    let store_directory = install_epoch;
    let final_leaf = install_epoch_event_name_v1(record.event_sequence)
        .map_err(InstallEpochStoreErrorV1::InvalidRequest)?;
    let names = store_directory
        .list_leaf_names_bounded(INSTALL_EPOCH_STORE_MAX_EVENTS_V1 * 2)
        .map_err(|error| InstallEpochStoreErrorV1::RecoveryRequired(error.into()))?;
    if names.iter().any(|name| name == OsStr::new(&final_leaf)) {
        let existing = store_directory
            .open_regular_readonly_beneath(Path::new(&final_leaf))
            .map_err(|error| InstallEpochStoreErrorV1::RecoveryRequired(error.into()))?;
        let existing_bytes = existing
            .read_all(MAX_EVENT_RECORD_BYTES_V1)
            .map_err(|error| InstallEpochStoreErrorV1::RecoveryRequired(error.into()))?;
        if existing_bytes
            != record
                .canonical_bytes()
                .map_err(InstallEpochStoreErrorV1::InvalidRequest)?
        {
            return Err(InstallEpochStoreErrorV1::ConflictHold(invalid(
                "install-epoch event sequence is occupied by different bytes",
            )));
        }
        let outcome = InstallEpochStoreAppendOutcomeV1::ExactExisting(
            ExactExistingInstallEpochStoreEventV1 {
                historical: record.event_sequence < before.event_count,
                identity: existing.identity(),
                record_sha256: record
                    .record_sha256()
                    .map_err(InstallEpochStoreErrorV1::InvalidRequest)?,
                verified_store: before,
            },
        );
        trusted_root
            .revalidate()
            .map_err(InstallEpochStoreErrorV1::RecoveryRequired)?;
        return Ok(outcome);
    }
    validate_next_event_v1(&before, record).map_err(InstallEpochStoreErrorV1::ConflictHold)?;
    if before.event_count
        >= u64::try_from(INSTALL_EPOCH_STORE_MAX_EVENTS_V1).map_err(|_| {
            InstallEpochStoreErrorV1::RecoveryRequired(invalid("event limit overflow"))
        })?
    {
        return Err(InstallEpochStoreErrorV1::ConflictHold(invalid(
            "install-epoch store reached its frozen event limit",
        )));
    }

    let canonical = record
        .canonical_bytes()
        .map_err(InstallEpochStoreErrorV1::InvalidRequest)?;
    let canonical_len = u64::try_from(canonical.len()).map_err(|_| {
        InstallEpochStoreErrorV1::InvalidRequest(invalid(
            "install-epoch canonical event size overflows u64",
        ))
    })?;
    let expected_total_bytes = checked_store_total_bytes_v1(before.total_bytes, canonical_len)
        .map_err(InstallEpochStoreErrorV1::ConflictHold)?;
    let mut crossed_mutation_boundary = false;
    let publication = publish_record_noreplace_observed_v8(
        store_directory,
        &final_leaf,
        publication_nonce,
        &canonical,
        |checkpoint| {
            crossed_mutation_boundary = true;
            observe(checkpoint);
        },
    );
    let publication = match publication {
        Ok(value) => value,
        Err(error) if crossed_mutation_boundary => {
            return Err(InstallEpochStoreErrorV1::RecoveryRequired(error));
        }
        Err(error) => return Err(InstallEpochStoreErrorV1::ConflictHold(error)),
    };
    lock.revalidate_for_root(root)
        .map_err(|error| InstallEpochStoreErrorV1::RecoveryRequired(error.into()))?;
    let after = scan_install_epoch_store_anchored_v1(root, install_epoch, lock, root_identity)
        .map_err(InstallEpochStoreErrorV1::RecoveryRequired)?;
    if after.incoming_residue_detected
        || after.event_count != record.event_sequence
        || after.total_bytes != expected_total_bytes
        || after.tip_sha256
            != record
                .record_sha256()
                .map_err(InstallEpochStoreErrorV1::InvalidRequest)?
    {
        return Err(InstallEpochStoreErrorV1::RecoveryRequired(invalid(
            "install-epoch post-publication replay differs from the appended event",
        )));
    }
    let outcome = InstallEpochStoreAppendOutcomeV1::Fresh(FreshInstallEpochStoreAppendV1 {
        publication,
        verified_store: after,
    });
    trusted_root
        .revalidate()
        .map_err(InstallEpochStoreErrorV1::RecoveryRequired)?;
    Ok(outcome)
}

fn scan_install_epoch_store_anchored_v1(
    root: &DirectoryAnchorV8,
    install_epoch: &DirectoryAnchorV8,
    lock: &StateRootLockV8,
    root_identity: FileIdentityV8,
) -> Result<VerifiedInstallEpochStoreScanV1, NativeErrorV8> {
    lock.revalidate_for_root(root)?;
    install_epoch.revalidate_identity()?;
    let directory = install_epoch;
    let before = directory.list_leaf_names_bounded(INSTALL_EPOCH_STORE_MAX_EVENTS_V1 * 2)?;
    let mut incoming_residue_detected = false;
    let mut event_names = Vec::new();
    for name in &before {
        let Some(name) = name.to_str() else {
            return Err(invalid("install-epoch store contains a non-UTF-8 leaf"));
        };
        if name.starts_with('.') && name.ends_with(".incoming") {
            incoming_residue_detected = true;
            continue;
        }
        event_names.push((parse_install_epoch_event_name_v1(name)?, name.to_string()));
    }
    if event_names.len() > INSTALL_EPOCH_STORE_MAX_EVENTS_V1 {
        return Err(invalid("install-epoch store exceeds its event limit"));
    }
    event_names.sort_by_key(|(sequence, _)| *sequence);
    let mut total_bytes = 0_u64;
    let mut last_record: Option<InstallEpochStoreRecordV1> = None;
    let mut tip_sha256 = ZERO_SHA256.to_string();
    for (index, (filename_sequence, name)) in event_names.iter().enumerate() {
        let expected_sequence = u64::try_from(index)
            .map_err(|_| invalid("install-epoch inventory index overflows"))?
            .checked_add(1)
            .ok_or_else(|| invalid("install-epoch sequence overflows"))?;
        if *filename_sequence != expected_sequence {
            return Err(invalid("install-epoch event filenames contain a gap"));
        }
        let file = directory.open_regular_readonly_beneath(Path::new(name))?;
        let identity = file.identity();
        if identity.owner_uid() != root_identity.owner_uid()
            || identity.owner_gid() != root_identity.owner_gid()
            || identity.mode() != 0o600
            || identity.link_count() != 1
            || identity.device() != root_identity.device()
        {
            return Err(invalid("install-epoch event file identity is not exact"));
        }
        total_bytes = total_bytes
            .checked_add(identity.size())
            .ok_or_else(|| invalid("install-epoch cumulative bytes overflow"))?;
        if total_bytes > MAX_STORE_BYTES_V1 {
            return Err(invalid("install-epoch store exceeds its byte budget"));
        }
        let bytes = file.read_all(MAX_EVENT_RECORD_BYTES_V1)?;
        let decoded = InstallEpochStoreRecordV1::decode_exact(&bytes)?;
        if decoded.event_sequence != expected_sequence
            || decoded.previous_event_sha256 != tip_sha256
        {
            return Err(invalid("install-epoch event chain is not contiguous"));
        }
        if let Some(previous) = &last_record {
            validate_record_transition_v1(previous, &decoded)?;
        } else if decoded.event_kind != InstallEpochStoreEventKindV1::Intent
            || decoded.phase_predecessor_revision != 0
        {
            return Err(invalid(
                "install-epoch store must begin with a genesis intent",
            ));
        }
        tip_sha256 = decoded.record_sha256()?;
        last_record = Some(decoded);
    }
    lock.revalidate_for_root(root)?;
    install_epoch.revalidate_identity()?;
    if directory.list_leaf_names_bounded(INSTALL_EPOCH_STORE_MAX_EVENTS_V1 * 2)? != before {
        return Err(invalid("install-epoch namespace changed during scan"));
    }
    Ok(VerifiedInstallEpochStoreScanV1 {
        event_count: u64::try_from(event_names.len())
            .map_err(|_| invalid("install-epoch event count overflows"))?,
        incoming_residue_detected,
        last_record,
        state_root_identity: root_identity,
        tip_sha256,
        total_bytes,
    })
}

fn checked_store_total_bytes_v1(current: u64, additional: u64) -> Result<u64, NativeErrorV8> {
    let prospective = current
        .checked_add(additional)
        .ok_or_else(|| invalid("install-epoch prospective byte budget overflows"))?;
    if prospective > MAX_STORE_BYTES_V1 {
        return Err(invalid(
            "install-epoch append would exceed its frozen byte budget",
        ));
    }
    Ok(prospective)
}

#[cfg_attr(
    not(all(test, target_os = "linux")),
    allow(
        dead_code,
        reason = "the transition validator is reached only by the unpublished raw append path"
    )
)]
fn validate_next_event_v1(
    scan: &VerifiedInstallEpochStoreScanV1,
    next: &InstallEpochStoreRecordV1,
) -> Result<(), NativeErrorV8> {
    if scan.event_count.checked_add(1) != Some(next.event_sequence)
        || scan.tip_sha256 != next.previous_event_sha256
    {
        return Err(invalid(
            "install-epoch append does not extend the exact durable tip",
        ));
    }
    if let Some(previous) = &scan.last_record {
        validate_record_transition_v1(previous, next)
    } else if next.event_kind != InstallEpochStoreEventKindV1::Intent
        || next.phase_predecessor_revision != 0
    {
        Err(invalid(
            "empty install-epoch store requires a genesis intent",
        ))
    } else {
        Ok(())
    }
}

fn validate_record_transition_v1(
    previous: &InstallEpochStoreRecordV1,
    next: &InstallEpochStoreRecordV1,
) -> Result<(), NativeErrorV8> {
    for (label, prior, current) in [
        (
            "machine",
            &previous.machine_id_sha256,
            &next.machine_id_sha256,
        ),
        (
            "state-root profile",
            &previous.state_root_profile_sha256,
            &next.state_root_profile_sha256,
        ),
        (
            "layout manifest",
            &previous.layout_manifest_sha256,
            &next.layout_manifest_sha256,
        ),
        (
            "provider profile",
            &previous.provider_profile_sha256,
            &next.provider_profile_sha256,
        ),
        ("stream", &previous.stream_id_sha256, &next.stream_id_sha256),
    ] {
        if prior != current {
            return Err(invalid(format!(
                "install-epoch {label} binding changed within one store",
            )));
        }
    }
    if previous.job_id_sha256 == next.job_id_sha256 {
        for (label, prior, current) in [
            (
                "completion profile",
                &previous.completion_profile_sha256,
                &next.completion_profile_sha256,
            ),
            (
                "completion slot",
                &previous.completion_slot_sha256,
                &next.completion_slot_sha256,
            ),
            (
                "prepared epoch",
                &previous.prepared_epoch_binding_sha256,
                &next.prepared_epoch_binding_sha256,
            ),
            (
                "phase head",
                &previous.phase_head_sha256,
                &next.phase_head_sha256,
            ),
            (
                "operation binding",
                &previous.operation_binding_sha256,
                &next.operation_binding_sha256,
            ),
        ] {
            if prior != current {
                return Err(invalid(format!(
                    "install-epoch {label} changed within one job",
                )));
            }
        }
        if previous.epoch_sequence != next.epoch_sequence
            || previous.phase_successor_revision != next.phase_predecessor_revision
            || previous.phase_successor_state_sha256 != next.phase_predecessor_state_sha256
            || !previous.event_kind.permits_successor(next.event_kind)
        {
            return Err(invalid(
                "install-epoch event violates its closed job transition",
            ));
        }
        Ok(())
    } else {
        Err(invalid(
            "cross-job install-epoch transition requires the unpublished verified projection bridge",
        ))
    }
}

#[cfg_attr(
    not(all(test, target_os = "linux")),
    allow(
        dead_code,
        reason = "event publication stays private until the verified projection bridge is published"
    )
)]
fn install_epoch_event_name_v1(sequence: u64) -> Result<String, NativeErrorV8> {
    if sequence == 0 {
        return Err(invalid("install-epoch event sequence must be non-zero"));
    }
    Ok(format!("{sequence:0INSTALL_EPOCH_EVENT_DIGITS_V1$}.event"))
}

fn parse_install_epoch_event_name_v1(name: &str) -> Result<u64, NativeErrorV8> {
    if name.len() != INSTALL_EPOCH_EVENT_DIGITS_V1 + ".event".len() || !name.ends_with(".event") {
        return Err(invalid("install-epoch store contains an unknown leaf"));
    }
    let digits = name
        .get(..INSTALL_EPOCH_EVENT_DIGITS_V1)
        .ok_or_else(|| invalid("install-epoch event filename is truncated"))?;
    if !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(invalid("install-epoch event filename is not canonical"));
    }
    let sequence = digits
        .parse::<u64>()
        .map_err(|_| invalid("install-epoch event filename sequence is invalid"))?;
    if sequence == 0 {
        return Err(invalid("install-epoch filename sequence must be non-zero"));
    }
    Ok(sequence)
}

fn take_digest_v1(bytes: &[u8], offset: &mut usize, label: &str) -> Result<String, NativeErrorV8> {
    let value = take_ascii_v1(bytes, offset, 64, label)?;
    validate_digest(label, &value)?;
    Ok(value)
}

fn take_ascii_v1(
    bytes: &[u8],
    offset: &mut usize,
    length: usize,
    label: &str,
) -> Result<String, NativeErrorV8> {
    let end = offset
        .checked_add(length)
        .ok_or_else(|| invalid("install-epoch decode offset overflows"))?;
    let value = bytes
        .get(*offset..end)
        .ok_or_else(|| invalid(format!("install-epoch {label} is truncated")))?;
    *offset = end;
    if !value.is_ascii() {
        return Err(invalid(format!("install-epoch {label} is not ASCII")));
    }
    String::from_utf8(value.to_vec()).map_err(|_| invalid("install-epoch ASCII decode failed"))
}

fn take_u64_v1(bytes: &[u8], offset: &mut usize) -> Result<u64, NativeErrorV8> {
    let end = offset
        .checked_add(8)
        .ok_or_else(|| invalid("install-epoch decode offset overflows"))?;
    let value: [u8; 8] = bytes
        .get(*offset..end)
        .ok_or_else(|| invalid("install-epoch scalar is truncated"))?
        .try_into()
        .map_err(|_| invalid("install-epoch scalar length is invalid"))?;
    *offset = end;
    Ok(u64::from_be_bytes(value))
}

fn take_u8_v1(bytes: &[u8], offset: &mut usize) -> Result<u8, NativeErrorV8> {
    let value = *bytes
        .get(*offset)
        .ok_or_else(|| invalid("install-epoch event kind is truncated"))?;
    *offset = offset
        .checked_add(1)
        .ok_or_else(|| invalid("install-epoch decode offset overflows"))?;
    Ok(value)
}

#[cfg(test)]
#[path = "install_epoch_store_tests.rs"]
mod tests;

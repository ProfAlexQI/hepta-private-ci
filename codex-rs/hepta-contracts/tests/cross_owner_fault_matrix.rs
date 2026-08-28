use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::AuthorityAction;
use codex_hepta_contracts::IdempotencyKey;
use codex_hepta_contracts::OperationBinding;
use codex_hepta_contracts::OperationId;
use codex_hepta_contracts::OperationPhase;
use codex_hepta_contracts::ProductComponentId;
use codex_hepta_contracts::RecoveryDecision;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::recovery_decision;

const MAGIC: &[u8; 8] = b"HPTOPJ01";
const SOURCE_AGENT: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const DESTINATION_AGENT: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c13";
static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(1);

struct TempRoot {
    path: PathBuf,
}

impl TempRoot {
    fn new(label: &str) -> io::Result<Self> {
        let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "hepta-cross-owner-{label}-{}-{id}",
            std::process::id()
        ));
        match std::fs::remove_dir_all(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        std::fs::create_dir(&path)?;
        Ok(Self { path })
    }

    fn join(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct JournalRecord {
    binding_sha256: Sha256Digest,
    phase: OperationPhase,
    sequence: u64,
    boundary_crossed: bool,
}

impl JournalRecord {
    fn new(
        binding: &OperationBinding,
        phase: OperationPhase,
        sequence: u64,
        boundary_crossed: bool,
    ) -> Self {
        Self {
            binding_sha256: binding.digest(),
            phase,
            sequence,
            boundary_crossed,
        }
    }

    fn encode(&self) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(MAGIC);
        payload.extend_from_slice(self.binding_sha256.as_str().as_bytes());
        payload.push(phase_code(self.phase));
        payload.extend_from_slice(&self.sequence.to_be_bytes());
        payload.push(u8::from(self.boundary_crossed));
        let checksum = Sha256Digest::for_bytes(&payload);
        payload.extend_from_slice(checksum.as_str().as_bytes());
        payload
    }

    fn decode(bytes: &[u8]) -> io::Result<Self> {
        const PAYLOAD_BYTES: usize = 8 + 64 + 1 + 8 + 1;
        const RECORD_BYTES: usize = PAYLOAD_BYTES + 64;
        if bytes.len() != RECORD_BYTES || &bytes[..8] != MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid operation journal framing",
            ));
        }
        let expected_checksum = std::str::from_utf8(&bytes[PAYLOAD_BYTES..])
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid checksum UTF-8"))?;
        if Sha256Digest::for_bytes(&bytes[..PAYLOAD_BYTES]).as_str() != expected_checksum {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "operation journal checksum mismatch",
            ));
        }
        let binding = std::str::from_utf8(&bytes[8..72])
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid digest UTF-8"))?;
        let binding_sha256 = Sha256Digest::parse(binding.to_string())
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        let phase = parse_phase(bytes[72])?;
        let sequence_bytes: [u8; 8] = bytes[73..81]
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid sequence"))?;
        let sequence = u64::from_be_bytes(sequence_bytes);
        if sequence == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "zero operation sequence",
            ));
        }
        let boundary_crossed = match bytes[81] {
            0 => false,
            1 => true,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid boundary flag",
                ));
            }
        };
        Ok(Self {
            binding_sha256,
            phase,
            sequence,
            boundary_crossed,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CommitFailpoint {
    None,
    BeforeWrite,
    AfterSyncBeforeRename,
    AfterRenameBeforeReturn,
}

fn commit_record(
    final_path: &Path,
    record: &JournalRecord,
    failpoint: CommitFailpoint,
) -> io::Result<()> {
    if failpoint == CommitFailpoint::BeforeWrite {
        return Err(io::Error::other("failpoint before journal write"));
    }
    let temp_path = final_path.with_extension("tmp");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp_path)?;
    file.write_all(&record.encode())?;
    file.sync_all()?;
    drop(file);
    if failpoint == CommitFailpoint::AfterSyncBeforeRename {
        return Err(io::Error::other("failpoint after sync before rename"));
    }
    std::fs::rename(&temp_path, final_path)?;
    let parent = File::open(
        final_path
            .parent()
            .ok_or_else(|| io::Error::other("journal path has no parent"))?,
    )?;
    parent.sync_all()?;
    if failpoint == CommitFailpoint::AfterRenameBeforeReturn {
        return Err(io::Error::other("acknowledgement lost after durable rename"));
    }
    Ok(())
}

fn reopen_record(path: &Path) -> io::Result<JournalRecord> {
    let mut bytes = Vec::new();
    File::open(path)?.read_to_end(&mut bytes)?;
    JournalRecord::decode(&bytes)
}

struct DiskFullWriter {
    remaining: usize,
    bytes: Vec<u8>,
}

impl DiskFullWriter {
    fn new(remaining: usize) -> Self {
        Self {
            remaining,
            bytes: Vec::new(),
        }
    }
}

impl Write for DiskFullWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if self.remaining == 0 {
            return Err(io::Error::other("simulated disk full"));
        }
        let count = self.remaining.min(buffer.len());
        self.bytes.extend_from_slice(&buffer[..count]);
        self.remaining -= count;
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn binding(generation: u64, command: &[u8]) -> OperationBinding {
    let source = AgentId::parse(SOURCE_AGENT)
        .unwrap_or_else(|error| panic!("source AgentId must parse: {error}"));
    let destination = AgentId::parse(DESTINATION_AGENT)
        .unwrap_or_else(|error| panic!("destination AgentId must parse: {error}"));
    OperationBinding::new(
        OperationId::parse("operation:fault-matrix")
            .unwrap_or_else(|error| panic!("operation id must parse: {error}")),
        IdempotencyKey::parse("idempotency:fault-matrix")
            .unwrap_or_else(|error| panic!("idempotency key must parse: {error}")),
        source,
        ProductComponentId::AutomationRuntime,
        destination,
        ProductComponentId::AppServer,
        AuthorityAction::MutateAutomation,
        1,
        4,
        generation,
        Sha256Digest::for_bytes(b"fencing-token"),
        Sha256Digest::for_bytes(command),
        u64::try_from(command.len())
            .unwrap_or_else(|error| panic!("command length must fit u64: {error}")),
    )
    .unwrap_or_else(|error| panic!("operation binding must be valid: {error}"))
}

#[test]
fn process_death_before_write_leaves_no_durable_record() {
    let root = TempRoot::new("before-write")
        .unwrap_or_else(|error| panic!("temporary root must open: {error}"));
    let final_path = root.join("operation.journal");
    let record = JournalRecord::new(&binding(1, b"command"), OperationPhase::OutboxPending, 1, false);
    assert!(commit_record(&final_path, &record, CommitFailpoint::BeforeWrite).is_err());
    assert!(!final_path.exists());
}

#[test]
fn process_death_after_sync_before_rename_does_not_publish_partial_record() {
    let root = TempRoot::new("before-rename")
        .unwrap_or_else(|error| panic!("temporary root must open: {error}"));
    let final_path = root.join("operation.journal");
    let record = JournalRecord::new(&binding(1, b"command"), OperationPhase::OutboxPending, 1, false);
    assert!(
        commit_record(
            &final_path,
            &record,
            CommitFailpoint::AfterSyncBeforeRename
        )
        .is_err()
    );
    assert!(!final_path.exists());
    assert!(final_path.with_extension("tmp").exists());
}

#[test]
fn acknowledgement_loss_after_rename_reopens_exact_durable_record() {
    let root = TempRoot::new("after-rename")
        .unwrap_or_else(|error| panic!("temporary root must open: {error}"));
    let final_path = root.join("operation.journal");
    let record = JournalRecord::new(
        &binding(1, b"command"),
        OperationPhase::DeliveryClaimed,
        2,
        true,
    );
    assert!(
        commit_record(
            &final_path,
            &record,
            CommitFailpoint::AfterRenameBeforeReturn
        )
        .is_err()
    );
    let reopened = reopen_record(&final_path)
        .unwrap_or_else(|error| panic!("durable record must reopen: {error}"));
    assert_eq!(reopened, record);
    assert_eq!(
        recovery_decision(reopened.phase, reopened.boundary_crossed),
        RecoveryDecision::LookupOnly
    );
}

#[test]
fn disk_full_write_never_yields_a_decodable_record() {
    let record = JournalRecord::new(&binding(1, b"command"), OperationPhase::OutboxPending, 1, false);
    let encoded = record.encode();
    let mut writer = DiskFullWriter::new(encoded.len() / 2);
    assert!(writer.write_all(&encoded).is_err());
    assert!(JournalRecord::decode(&writer.bytes).is_err());
}

#[test]
fn corruption_and_truncation_fail_closed_on_reopen() {
    let record = JournalRecord::new(&binding(1, b"command"), OperationPhase::OutboxPending, 1, false);
    let mut corrupt = record.encode();
    corrupt[16] ^= 0x01;
    assert!(JournalRecord::decode(&corrupt).is_err());
    assert!(JournalRecord::decode(&record.encode()[..40]).is_err());
}

#[test]
fn stale_generation_and_changed_payload_cannot_adopt_old_record() {
    let original_binding = binding(1, b"command");
    let original = JournalRecord::new(
        &original_binding,
        OperationPhase::DestinationCommitted,
        3,
        true,
    );
    let stale_generation = binding(2, b"command");
    let changed_payload = binding(1, b"changed-command");
    assert_ne!(original.binding_sha256, stale_generation.digest());
    assert_ne!(original.binding_sha256, changed_payload.digest());
    assert_eq!(
        recovery_decision(original.phase, original.boundary_crossed),
        RecoveryDecision::AdoptAcknowledgement
    );
}

#[test]
fn terminal_record_never_reopens_for_delivery() {
    for phase in [
        OperationPhase::Acknowledged,
        OperationPhase::ReconciledApplied,
        OperationPhase::ReconciledNotApplied,
        OperationPhase::Quarantined,
    ] {
        assert_eq!(
            recovery_decision(phase, true),
            RecoveryDecision::Terminal
        );
    }
}

fn phase_code(phase: OperationPhase) -> u8 {
    match phase {
        OperationPhase::IntentAppended => 1,
        OperationPhase::SourceCommitted => 2,
        OperationPhase::OutboxPending => 3,
        OperationPhase::DeliveryClaimed => 4,
        OperationPhase::DestinationCommitted => 5,
        OperationPhase::Acknowledged => 6,
        OperationPhase::Indeterminate => 7,
        OperationPhase::ReconciledApplied => 8,
        OperationPhase::ReconciledNotApplied => 9,
        OperationPhase::Quarantined => 10,
    }
}

fn parse_phase(value: u8) -> io::Result<OperationPhase> {
    match value {
        1 => Ok(OperationPhase::IntentAppended),
        2 => Ok(OperationPhase::SourceCommitted),
        3 => Ok(OperationPhase::OutboxPending),
        4 => Ok(OperationPhase::DeliveryClaimed),
        5 => Ok(OperationPhase::DestinationCommitted),
        6 => Ok(OperationPhase::Acknowledged),
        7 => Ok(OperationPhase::Indeterminate),
        8 => Ok(OperationPhase::ReconciledApplied),
        9 => Ok(OperationPhase::ReconciledNotApplied),
        10 => Ok(OperationPhase::Quarantined),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unknown operation phase",
        )),
    }
}

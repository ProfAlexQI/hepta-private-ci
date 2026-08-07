use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_contracts::Sha256Digest;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

use crate::ProofAppendDisposition;
use crate::ProofError;
use crate::ProofStore;
use crate::file_hash::sha256_regular_file;
use crate::file_hash::validate_execution_directory;
use crate::framing::length_delimited_sha256;
use crate::runner::execute;

pub const MAX_PROOF_ARGUMENTS: usize = 256;
pub const MAX_PROOF_ARGUMENT_BYTES: usize = 64 * 1024;
pub const MAX_PROOF_ENVIRONMENT_ENTRIES: usize = 128;
pub const MAX_PROOF_ENVIRONMENT_VALUE_BYTES: usize = 64 * 1024;
pub const MAX_PROOF_PATH_BYTES: usize = 16 * 1024;
pub const MAX_PROOF_TIMEOUT_MS: u64 = 24 * 60 * 60 * 1_000;
pub const MAX_PROOF_CAPTURE_BYTES: u64 = 8 * 1024 * 1024;
pub const MAX_PROOF_HASH_FILE_BYTES: u64 = 8 * 1024 * 1024 * 1024;
pub const PROOF_SCHEMA_VERSION: u32 = 1;

const PROOF_COMMAND_BINDING_DOMAIN: &str = "hepta.proof-command-binding.v1";
const PROOF_INVOCATION_DOMAIN: &str = "hepta.proof-invocation.v1";
const PROOF_RECEIPT_DOMAIN: &str = "hepta.proof-receipt.v1";
const PROOF_RECEIPT_CONTENT_DOMAIN: &str = "hepta.proof-receipt-content.v1";
const PROOF_INVOCATION_ID_PREFIX: &str = "proof-invocation:v1:";
const PROOF_RECEIPT_ID_PREFIX: &str = "proof-receipt:v1:";

pub struct ProofCommandSpec {
    pub(crate) program: PathBuf,
    pub(crate) program_sha256: Sha256Digest,
    pub(crate) arguments: Vec<String>,
    pub(crate) cwd: PathBuf,
    pub(crate) environment: BTreeMap<String, String>,
    pub(crate) timeout_ms: u64,
    pub(crate) max_stdout_bytes: u64,
    pub(crate) max_stderr_bytes: u64,
    binding_sha256: Sha256Digest,
}

#[derive(Serialize)]
struct CommandBinding<'a> {
    domain: &'static str,
    schema_version: u32,
    program: &'a str,
    program_sha256: &'a Sha256Digest,
    arguments: &'a [String],
    cwd: &'a str,
    environment: &'a BTreeMap<String, String>,
    timeout_ms: u64,
    max_stdout_bytes: u64,
    max_stderr_bytes: u64,
}

impl ProofCommandSpec {
    /// Creates a bounded command binding and synchronously hashes the program.
    ///
    /// `timeout_ms` later bounds spawned-command capture. It does not bound
    /// this constructor, execution-path revalidation, filesystem latency, or
    /// total caller wall-clock time.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        program: impl Into<PathBuf>,
        arguments: Vec<String>,
        cwd: impl Into<PathBuf>,
        environment: BTreeMap<String, String>,
        timeout_ms: u64,
        max_stdout_bytes: u64,
        max_stderr_bytes: u64,
    ) -> Result<Self, ProofError> {
        let program = program.into();
        let cwd = cwd.into();
        validate_command_input(
            &program,
            &arguments,
            &cwd,
            &environment,
            timeout_ms,
            max_stdout_bytes,
            max_stderr_bytes,
        )?;
        validate_execution_directory(&cwd)?;
        let program_sha256 = sha256_regular_file(&program, MAX_PROOF_HASH_FILE_BYTES)?;
        let binding = CommandBinding {
            domain: PROOF_COMMAND_BINDING_DOMAIN,
            schema_version: PROOF_SCHEMA_VERSION,
            program: program.to_str().ok_or_else(|| {
                ProofError::InvalidInput("proof program path is not UTF-8".to_string())
            })?,
            program_sha256: &program_sha256,
            arguments: &arguments,
            cwd: cwd
                .to_str()
                .ok_or_else(|| ProofError::InvalidInput("proof cwd is not UTF-8".to_string()))?,
            environment: &environment,
            timeout_ms,
            max_stdout_bytes,
            max_stderr_bytes,
        };
        let binding_bytes = serde_json::to_vec(&binding)
            .map_err(|error| ProofError::InvalidInput(error.to_string()))?;
        let binding_sha256 = Sha256Digest::for_bytes(&binding_bytes);
        Ok(Self {
            program,
            program_sha256,
            arguments,
            cwd,
            environment,
            timeout_ms,
            max_stdout_bytes,
            max_stderr_bytes,
            binding_sha256,
        })
    }

    pub const fn binding_sha256(&self) -> &Sha256Digest {
        &self.binding_sha256
    }
}

/// Metadata naming the subject and context of one proof run.
///
/// These digests are attribution only. A `ProofSubject` does not prove that a
/// candidate or context is authoritative, present, or accepted.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProofSubject {
    candidate_sha256: Sha256Digest,
    context_sha256: Sha256Digest,
    #[serde(
        default,
        skip_serializing_if = "ProofContextOrigin::is_caller_supplied"
    )]
    context_origin: ProofContextOrigin,
}

/// The in-process API path that supplied a proof context.
///
/// This marker distinguishes the public generic constructor from the
/// crate-private historical-store resolver. It is not secret, signed, or an
/// authority against direct replacement of the local proof root.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofContextOrigin {
    #[default]
    CallerSupplied,
    HistoricalStoreResolved,
}

impl ProofContextOrigin {
    pub(crate) const fn as_wire_str(self) -> &'static str {
        match self {
            Self::CallerSupplied => "caller_supplied",
            Self::HistoricalStoreResolved => "historical_store_resolved",
        }
    }

    // Serde's `skip_serializing_if` callback receives a shared reference.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    const fn is_caller_supplied(&self) -> bool {
        matches!(self, Self::CallerSupplied)
    }
}

impl ProofSubject {
    pub fn new(
        candidate_sha256: Sha256Digest,
        context_sha256: Sha256Digest,
    ) -> Result<Self, String> {
        Sha256Digest::parse(candidate_sha256.as_str())?;
        Sha256Digest::parse(context_sha256.as_str())?;
        Ok(Self {
            candidate_sha256,
            context_sha256,
            context_origin: ProofContextOrigin::CallerSupplied,
        })
    }

    pub(crate) fn new_historical_store_resolved(
        candidate_sha256: Sha256Digest,
        context_sha256: Sha256Digest,
    ) -> Result<Self, String> {
        let mut subject = Self::new(candidate_sha256, context_sha256)?;
        subject.context_origin = ProofContextOrigin::HistoricalStoreResolved;
        Ok(subject)
    }

    pub const fn candidate_sha256(&self) -> &Sha256Digest {
        &self.candidate_sha256
    }

    pub const fn context_sha256(&self) -> &Sha256Digest {
        &self.context_sha256
    }

    pub const fn context_origin(&self) -> ProofContextOrigin {
        self.context_origin
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProofSubjectWire {
    candidate_sha256: Sha256Digest,
    context_sha256: Sha256Digest,
    #[serde(default)]
    context_origin: ProofContextOrigin,
}

impl<'de> Deserialize<'de> for ProofSubject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ProofSubjectWire::deserialize(deserializer)?;
        let mut subject = Self::new(wire.candidate_sha256, wire.context_sha256)
            .map_err(serde::de::Error::custom)?;
        subject.context_origin = wire.context_origin;
        Ok(subject)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ProofInvocationId(String);

impl ProofInvocationId {
    pub fn parse(value: impl Into<String>) -> Result<Self, String> {
        parse_prefixed_sha256_id(value.into(), PROOF_INVOCATION_ID_PREFIX)
            .map(Self)
            .map_err(|_| "proof invocation ID is invalid".to_string())
    }

    pub(crate) fn for_intent(
        subject: &ProofSubject,
        command_binding_sha256: &Sha256Digest,
        nonce_sha256: &Sha256Digest,
    ) -> Self {
        let schema_version = PROOF_SCHEMA_VERSION.to_string();
        let digest = match subject.context_origin {
            ProofContextOrigin::CallerSupplied => length_delimited_sha256([
                PROOF_INVOCATION_DOMAIN,
                schema_version.as_str(),
                subject.candidate_sha256.as_str(),
                subject.context_sha256.as_str(),
                command_binding_sha256.as_str(),
                nonce_sha256.as_str(),
            ]),
            ProofContextOrigin::HistoricalStoreResolved => length_delimited_sha256([
                PROOF_INVOCATION_DOMAIN,
                schema_version.as_str(),
                subject.context_origin.as_wire_str(),
                subject.candidate_sha256.as_str(),
                subject.context_sha256.as_str(),
                command_binding_sha256.as_str(),
                nonce_sha256.as_str(),
            ]),
        };
        Self(format!("{PROOF_INVOCATION_ID_PREFIX}{}", digest.as_str()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn digest_suffix(&self) -> &str {
        self.0
            .strip_prefix(PROOF_INVOCATION_ID_PREFIX)
            .unwrap_or(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for ProofInvocationId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ProofReceiptId(String);

impl ProofReceiptId {
    pub fn parse(value: impl Into<String>) -> Result<Self, String> {
        parse_prefixed_sha256_id(value.into(), PROOF_RECEIPT_ID_PREFIX)
            .map(Self)
            .map_err(|_| "proof receipt ID is invalid".to_string())
    }

    pub(crate) fn for_invocation(invocation_id: &ProofInvocationId) -> Self {
        let digest = length_delimited_sha256([
            PROOF_RECEIPT_DOMAIN,
            &PROOF_SCHEMA_VERSION.to_string(),
            invocation_id.as_str(),
        ]);
        Self(format!("{PROOF_RECEIPT_ID_PREFIX}{}", digest.as_str()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn digest_suffix(&self) -> &str {
        self.0
            .strip_prefix(PROOF_RECEIPT_ID_PREFIX)
            .unwrap_or(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for ProofReceiptId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ProofIntent {
    pub schema_version: u32,
    pub invocation_id: ProofInvocationId,
    pub subject: ProofSubject,
    pub command_binding_sha256: Sha256Digest,
    pub nonce_sha256: Sha256Digest,
}

pub struct ProofInvocation {
    pub(crate) intent: ProofIntent,
    pub(crate) command: ProofCommandSpec,
}

impl ProofInvocation {
    pub fn new(mut subject: ProofSubject, nonce: [u8; 16], command: ProofCommandSpec) -> Self {
        subject.context_origin = ProofContextOrigin::CallerSupplied;
        Self::new_with_bound_origin(subject, nonce, command)
    }

    pub(crate) fn new_historical_store_resolved(
        subject: ProofSubject,
        nonce: [u8; 16],
        command: ProofCommandSpec,
    ) -> Self {
        debug_assert_eq!(
            subject.context_origin,
            ProofContextOrigin::HistoricalStoreResolved
        );
        Self::new_with_bound_origin(subject, nonce, command)
    }

    fn new_with_bound_origin(
        subject: ProofSubject,
        nonce: [u8; 16],
        command: ProofCommandSpec,
    ) -> Self {
        let nonce_sha256 = Sha256Digest::for_bytes(&nonce);
        let invocation_id =
            ProofInvocationId::for_intent(&subject, &command.binding_sha256, &nonce_sha256);
        Self {
            intent: ProofIntent {
                schema_version: PROOF_SCHEMA_VERSION,
                invocation_id,
                subject,
                command_binding_sha256: command.binding_sha256.clone(),
                nonce_sha256,
            },
            command,
        }
    }

    pub const fn invocation_id(&self) -> &ProofInvocationId {
        &self.intent.invocation_id
    }

    pub const fn subject(&self) -> &ProofSubject {
        &self.intent.subject
    }

    pub fn receipt_id(&self) -> ProofReceiptId {
        ProofReceiptId::for_invocation(&self.intent.invocation_id)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofStreamKind {
    Stdout,
    Stderr,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProofStreamEvidence {
    complete: bool,
    bytes_observed: u64,
    sha256: Option<Sha256Digest>,
}

impl ProofStreamEvidence {
    pub(crate) fn complete(bytes: &[u8]) -> Self {
        Self {
            complete: true,
            bytes_observed: bytes.len() as u64,
            sha256: Some(Sha256Digest::for_bytes(bytes)),
        }
    }

    pub(crate) fn unavailable() -> Self {
        Self {
            complete: false,
            bytes_observed: 0,
            sha256: None,
        }
    }

    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    pub const fn bytes_observed(&self) -> u64 {
        self.bytes_observed
    }

    pub const fn sha256(&self) -> Option<&Sha256Digest> {
        self.sha256.as_ref()
    }

    pub(crate) fn validate_shape(&self) -> Result<(), String> {
        if self.bytes_observed > MAX_PROOF_CAPTURE_BYTES {
            return Err("proof stream evidence exceeds the hard bound".to_string());
        }
        match (&self.sha256, self.complete) {
            (Some(sha256), true) => Sha256Digest::parse(sha256.as_str()).map(|_| ()),
            (None, false) if self.bytes_observed == 0 => Ok(()),
            _ => Err("proof stream completeness and digest disagree".to_string()),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProofStreamEvidenceWire {
    complete: bool,
    bytes_observed: u64,
    sha256: Option<Sha256Digest>,
}

impl<'de> Deserialize<'de> for ProofStreamEvidence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ProofStreamEvidenceWire::deserialize(deserializer)?;
        let evidence = Self {
            complete: wire.complete,
            bytes_observed: wire.bytes_observed,
            sha256: wire.sha256,
        };
        evidence
            .validate_shape()
            .map_err(serde::de::Error::custom)?;
        Ok(evidence)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ProofTerminal {
    Completed {
        success: bool,
        exit_code: Option<i32>,
    },
    TimedOut,
    OutputLimitExceeded {
        stream: ProofStreamKind,
    },
    NotStarted {
        reason_code: String,
    },
    Indeterminate {
        reason_code: String,
    },
}

impl ProofTerminal {
    pub(crate) fn validate_shape(&self) -> Result<(), String> {
        match self {
            Self::Completed { success, exit_code } if *success != (*exit_code == Some(0)) => {
                Err("proof terminal success and exit code disagree".to_string())
            }
            Self::NotStarted { reason_code } | Self::Indeterminate { reason_code }
                if !valid_reason_code(reason_code) =>
            {
                Err("proof terminal reason code is invalid".to_string())
            }
            _ => Ok(()),
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum ProofTerminalWire {
    Completed {
        success: bool,
        exit_code: Option<i32>,
    },
    TimedOut,
    OutputLimitExceeded {
        stream: ProofStreamKind,
    },
    NotStarted {
        reason_code: String,
    },
    Indeterminate {
        reason_code: String,
    },
}

impl<'de> Deserialize<'de> for ProofTerminal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let terminal = match ProofTerminalWire::deserialize(deserializer)? {
            ProofTerminalWire::Completed { success, exit_code } => {
                Self::Completed { success, exit_code }
            }
            ProofTerminalWire::TimedOut => Self::TimedOut,
            ProofTerminalWire::OutputLimitExceeded { stream } => {
                Self::OutputLimitExceeded { stream }
            }
            ProofTerminalWire::NotStarted { reason_code } => Self::NotStarted { reason_code },
            ProofTerminalWire::Indeterminate { reason_code } => Self::Indeterminate { reason_code },
        };
        terminal
            .validate_shape()
            .map_err(serde::de::Error::custom)?;
        Ok(terminal)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProofReceipt {
    schema_version: u32,
    receipt_id: ProofReceiptId,
    invocation_id: ProofInvocationId,
    subject: ProofSubject,
    command_binding_sha256: Sha256Digest,
    started_at_unix_ms: u64,
    finished_at_unix_ms: u64,
    terminal: ProofTerminal,
    stdout: ProofStreamEvidence,
    stderr: ProofStreamEvidence,
    receipt_sha256: Sha256Digest,
}

impl ProofReceipt {
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub const fn receipt_id(&self) -> &ProofReceiptId {
        &self.receipt_id
    }

    pub const fn invocation_id(&self) -> &ProofInvocationId {
        &self.invocation_id
    }

    pub const fn subject(&self) -> &ProofSubject {
        &self.subject
    }

    pub const fn command_binding_sha256(&self) -> &Sha256Digest {
        &self.command_binding_sha256
    }

    pub const fn started_at_unix_ms(&self) -> u64 {
        self.started_at_unix_ms
    }

    pub const fn finished_at_unix_ms(&self) -> u64 {
        self.finished_at_unix_ms
    }

    pub const fn terminal(&self) -> &ProofTerminal {
        &self.terminal
    }

    pub const fn stdout(&self) -> &ProofStreamEvidence {
        &self.stdout
    }

    pub const fn stderr(&self) -> &ProofStreamEvidence {
        &self.stderr
    }

    /// Returns an unkeyed, recomputable self-consistency digest.
    ///
    /// This digest is not an external anchor or authority against replacement
    /// or rollback of the local proof root.
    pub const fn receipt_sha256(&self) -> &Sha256Digest {
        &self.receipt_sha256
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProofReceiptWire {
    schema_version: u32,
    receipt_id: ProofReceiptId,
    invocation_id: ProofInvocationId,
    subject: ProofSubject,
    command_binding_sha256: Sha256Digest,
    started_at_unix_ms: u64,
    finished_at_unix_ms: u64,
    terminal: ProofTerminal,
    stdout: ProofStreamEvidence,
    stderr: ProofStreamEvidence,
    receipt_sha256: Sha256Digest,
}

impl<'de> Deserialize<'de> for ProofReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ProofReceiptWire::deserialize(deserializer)?;
        let receipt = Self {
            schema_version: wire.schema_version,
            receipt_id: wire.receipt_id,
            invocation_id: wire.invocation_id,
            subject: wire.subject,
            command_binding_sha256: wire.command_binding_sha256,
            started_at_unix_ms: wire.started_at_unix_ms,
            finished_at_unix_ms: wire.finished_at_unix_ms,
            terminal: wire.terminal,
            stdout: wire.stdout,
            stderr: wire.stderr,
            receipt_sha256: wire.receipt_sha256,
        };
        crate::validation::validate_receipt(&receipt)
            .map_err(|error| serde::de::Error::custom(error.to_string()))?;
        Ok(receipt)
    }
}

#[derive(Serialize)]
struct ProofReceiptBinding<'a> {
    domain: &'static str,
    schema_version: u32,
    receipt_id: &'a ProofReceiptId,
    invocation_id: &'a ProofInvocationId,
    subject: &'a ProofSubject,
    command_binding_sha256: &'a Sha256Digest,
    started_at_unix_ms: u64,
    finished_at_unix_ms: u64,
    terminal: &'a ProofTerminal,
    stdout: &'a ProofStreamEvidence,
    stderr: &'a ProofStreamEvidence,
}

pub(crate) fn expected_receipt_sha256(receipt: &ProofReceipt) -> Result<Sha256Digest, ProofError> {
    let binding = ProofReceiptBinding {
        domain: PROOF_RECEIPT_CONTENT_DOMAIN,
        schema_version: receipt.schema_version,
        receipt_id: &receipt.receipt_id,
        invocation_id: &receipt.invocation_id,
        subject: &receipt.subject,
        command_binding_sha256: &receipt.command_binding_sha256,
        started_at_unix_ms: receipt.started_at_unix_ms,
        finished_at_unix_ms: receipt.finished_at_unix_ms,
        terminal: &receipt.terminal,
        stdout: &receipt.stdout,
        stderr: &receipt.stderr,
    };
    serde_json::to_vec(&binding)
        .map(|bytes| Sha256Digest::for_bytes(&bytes))
        .map_err(|error| ProofError::Corrupt(format!("proof receipt binding failed: {error}")))
}

#[derive(Debug)]
pub struct ProofExecutionResult {
    pub receipt: ProofReceipt,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

#[derive(Clone)]
pub struct ProofHarness {
    store: ProofStore,
}

impl ProofHarness {
    pub fn new(store: ProofStore) -> Self {
        Self { store }
    }

    /// Runs one bounded local command observation.
    ///
    /// The create-new intent blocks replay only while the configured local
    /// proof root remains intact. Deleting or rolling back the root can revive
    /// an invocation, so this is not exactly-once execution or an authority
    /// ledger. A pre-existing lock fails closed and has no automatic stale-lock
    /// recovery. Unix cleanup covers only processes that remain in the spawned
    /// process group and is not sandbox containment.
    pub async fn run(
        &self,
        invocation: ProofInvocation,
    ) -> Result<ProofExecutionResult, ProofError> {
        let _lock = self.store.acquire_lock(&invocation.intent.invocation_id)?;
        let receipt_id = invocation.receipt_id();
        if self.store.get_receipt(&receipt_id)?.is_some() {
            return Err(ProofError::ReplayBlocked {
                invocation_id: invocation.intent.invocation_id.as_str().to_string(),
            });
        }
        match self.store.claim_intent(&invocation.intent)? {
            ProofAppendDisposition::Inserted => {}
            ProofAppendDisposition::AlreadyPresent => {
                return Err(ProofError::ReplayBlocked {
                    invocation_id: invocation.intent.invocation_id.as_str().to_string(),
                });
            }
        }
        let started_at_unix_ms = now_millis()?;
        let execution = execute(&invocation.command).await;
        let finished_at_unix_ms = now_millis()?;
        let receipt = ProofReceipt {
            schema_version: PROOF_SCHEMA_VERSION,
            receipt_id,
            invocation_id: invocation.intent.invocation_id,
            subject: invocation.intent.subject,
            command_binding_sha256: invocation.intent.command_binding_sha256,
            started_at_unix_ms,
            finished_at_unix_ms,
            terminal: execution.terminal,
            stdout: execution.stdout_evidence,
            stderr: execution.stderr_evidence,
            receipt_sha256: Sha256Digest::for_bytes(b"pending-proof-receipt"),
        };
        let mut receipt = receipt;
        receipt.receipt_sha256 = expected_receipt_sha256(&receipt)?;
        match self.store.append_receipt(&receipt)? {
            ProofAppendDisposition::Inserted => Ok(ProofExecutionResult {
                receipt,
                stdout: execution.stdout,
                stderr: execution.stderr,
            }),
            ProofAppendDisposition::AlreadyPresent => Err(ProofError::EvidenceConflict {
                record_id: receipt.receipt_id.as_str().to_string(),
            }),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn validate_command_input(
    program: &Path,
    arguments: &[String],
    cwd: &Path,
    environment: &BTreeMap<String, String>,
    timeout_ms: u64,
    max_stdout_bytes: u64,
    max_stderr_bytes: u64,
) -> Result<(), ProofError> {
    if !valid_absolute_path(program) {
        return Err(ProofError::InvalidInput(
            "proof program must be a bounded absolute UTF-8 path".to_string(),
        ));
    }
    if !valid_absolute_path(cwd) {
        return Err(ProofError::InvalidInput(
            "proof cwd must be a bounded absolute UTF-8 path".to_string(),
        ));
    }
    if arguments.len() > MAX_PROOF_ARGUMENTS
        || arguments.iter().any(|argument| argument.contains('\0'))
        || arguments
            .iter()
            .map(String::len)
            .try_fold(0_usize, usize::checked_add)
            .is_none_or(|bytes| bytes > MAX_PROOF_ARGUMENT_BYTES)
    {
        return Err(ProofError::InvalidInput(
            "proof arguments exceed the hard bound".to_string(),
        ));
    }
    if environment.len() > MAX_PROOF_ENVIRONMENT_ENTRIES
        || environment
            .iter()
            .any(|(name, value)| !valid_environment_name(name) || value.contains('\0'))
        || environment
            .iter()
            .map(|(name, value)| name.len().saturating_add(value.len()))
            .try_fold(0_usize, usize::checked_add)
            .is_none_or(|bytes| bytes > MAX_PROOF_ENVIRONMENT_VALUE_BYTES)
    {
        return Err(ProofError::InvalidInput(
            "proof environment exceeds the hard bound".to_string(),
        ));
    }
    if timeout_ms == 0 || timeout_ms > MAX_PROOF_TIMEOUT_MS {
        return Err(ProofError::InvalidInput(
            "proof timeout is outside the hard bound".to_string(),
        ));
    }
    if max_stdout_bytes == 0
        || max_stdout_bytes > MAX_PROOF_CAPTURE_BYTES
        || max_stderr_bytes == 0
        || max_stderr_bytes > MAX_PROOF_CAPTURE_BYTES
    {
        return Err(ProofError::InvalidInput(
            "proof output limit is outside the hard bound".to_string(),
        ));
    }
    Ok(())
}

fn valid_environment_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 256
        && name.bytes().enumerate().all(|(index, byte)| {
            byte == b'_' || byte.is_ascii_alphabetic() || (index > 0 && byte.is_ascii_digit())
        })
}

pub(crate) fn valid_reason_code(reason_code: &str) -> bool {
    !reason_code.is_empty()
        && reason_code.len() <= 128
        && reason_code
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn valid_absolute_path(path: &Path) -> bool {
    path.is_absolute()
        && path
            .to_str()
            .is_some_and(|value| !value.contains('\0') && value.len() <= MAX_PROOF_PATH_BYTES)
}

fn parse_prefixed_sha256_id(value: String, prefix: &str) -> Result<String, String> {
    let Some(digest) = value.strip_prefix(prefix) else {
        return Err("wrong prefix".to_string());
    };
    Sha256Digest::parse(digest.to_string())?;
    Ok(value)
}

fn now_millis() -> Result<u64, ProofError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| ProofError::StoreUnavailable(error.to_string()))?;
    u64::try_from(duration.as_millis())
        .map_err(|_| ProofError::StoreUnavailable("system clock exceeds u64".to_string()))
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

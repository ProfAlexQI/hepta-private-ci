#!/usr/bin/env python3
"""Source definitions for bounded control, inference and adapter closure."""

from __future__ import annotations

RUST_PACKAGES = {
    "hepta-authbus": "codex-hepta-authbus",
    "hepta-authbus-p1-3-qualification": "codex-hepta-authbus-p1-3-qualification",
    "hepta-control-plane": "codex-hepta-control-plane",
    "hepta-infer-core": "codex-hepta-infer-core",
    "hepta-inferd": "codex-hepta-inferd",
    "hepta-intelligence": "codex-hepta-intelligence",
    "hepta-codex-adapter": "codex-hepta-codex-adapter",
    "hepta-bao-adapter": "codex-hepta-bao-adapter",
}

SOURCE_ROOTS = {
    "auth.authbus": (
        "codex-rs/hepta-authbus",
        "codex-rs/hepta-authbus-p1-3-qualification",
    ),
    "control.runtime": ("codex-rs/hepta-control-plane",),
    "inference.control": (
        "codex-rs/hepta-infer-core",
        "codex-rs/hepta-inferd",
    ),
    "intelligence.control": ("codex-rs/hepta-intelligence",),
    "runtime.codex": (
        "codex-rs/codex-app-server",
        "codex-rs/hepta-codex-adapter",
    ),
    "secrets.heptabao": (
        "external/HeptaBao",
        "codex-rs/hepta-bao-adapter",
    ),
}


def manifest(package: str, lib: str, dependencies: tuple[tuple[str, str], ...] = ()) -> str:
    dependency_lines = ['codex-hepta-types = { path = "../hepta-types" }']
    dependency_lines.extend(
        f'{name} = {{ path = "../{path}" }}' for name, path in dependencies
    )
    return f"""[package]
edition.workspace = true
license.workspace = true
name = "{package}"
version.workspace = true
publish = false

[lib]
name = "{lib}"
path = "src/lib.rs"
doctest = false

[lints]
workspace = true

[dependencies]
{chr(10).join(dependency_lines)}

[dev-dependencies]
pretty_assertions = {{ workspace = true }}
"""


def build(root: str) -> str:
    return f"""load("//:defs.bzl", "codex_rust_crate")

codex_rust_crate(
    name = "{root}",
    crate_name = "codex_{root.replace("-", "_")}",
)
"""


FILES = {
    "codex-rs/hepta-authbus/Cargo.toml": manifest(
        "codex-hepta-authbus", "codex_hepta_authbus"
    ),
    "codex-rs/hepta-authbus/BUILD.bazel": build("hepta-authbus"),
    "codex-rs/hepta-authbus/src/lib.rs": r'''//! Authenticated, replay-fenced message verification.
//!
//! This crate verifies pre-existing envelopes. It cannot mint grants, widen
//! scope, dispatch effects, select, promote, merge or release.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt;

use codex_hepta_types::{AuthorityPosture, Digest32, StableId};

const MAX_SUBJECTS: usize = 16_384;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthEnvelope {
    pub message_id: StableId,
    pub subject_id: StableId,
    pub scope_digest: Digest32,
    pub payload_digest: Digest32,
    pub signature_digest: Digest32,
    pub sequence: u64,
    pub expires_at_ms: u64,
    pub revoked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationReceipt {
    pub message_id: StableId,
    pub subject_id: StableId,
    pub sequence: u64,
    pub envelope_digest: Digest32,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptyDigest(&'static str),
    ZeroSequence,
    Revoked,
    Expired,
    ScopeMismatch,
    PayloadMismatch,
    Replay,
    CapacityExceeded,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayWindow {
    highest_sequence: BTreeMap<StableId, u64>,
    maximum_subjects: usize,
}

impl ReplayWindow {
    #[must_use]
    pub fn new(maximum_subjects: usize) -> Self {
        Self {
            highest_sequence: BTreeMap::new(),
            maximum_subjects: maximum_subjects.min(MAX_SUBJECTS),
        }
    }

    pub fn verify(
        &mut self,
        now_ms: u64,
        envelope: AuthEnvelope,
        expected_scope: Digest32,
        expected_payload: Digest32,
    ) -> Result<VerificationReceipt, Error> {
        for (name, digest) in [
            ("scope", envelope.scope_digest),
            ("payload", envelope.payload_digest),
            ("signature", envelope.signature_digest),
        ] {
            if digest.is_zero() {
                return Err(Error::EmptyDigest(name));
            }
        }
        if envelope.sequence == 0 {
            return Err(Error::ZeroSequence);
        }
        if envelope.revoked {
            return Err(Error::Revoked);
        }
        if now_ms >= envelope.expires_at_ms {
            return Err(Error::Expired);
        }
        if envelope.scope_digest != expected_scope {
            return Err(Error::ScopeMismatch);
        }
        if envelope.payload_digest != expected_payload {
            return Err(Error::PayloadMismatch);
        }
        if self
            .highest_sequence
            .get(&envelope.subject_id)
            .is_some_and(|sequence| *sequence >= envelope.sequence)
        {
            return Err(Error::Replay);
        }
        if !self.highest_sequence.contains_key(&envelope.subject_id)
            && self.highest_sequence.len() >= self.maximum_subjects
        {
            return Err(Error::CapacityExceeded);
        }
        self.highest_sequence
            .insert(envelope.subject_id.clone(), envelope.sequence);

        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"hepta.authbus.verification.v1");
        push_id(&mut bytes, &envelope.message_id);
        push_id(&mut bytes, &envelope.subject_id);
        bytes.extend_from_slice(envelope.scope_digest.as_array());
        bytes.extend_from_slice(envelope.payload_digest.as_array());
        bytes.extend_from_slice(envelope.signature_digest.as_array());
        bytes.extend_from_slice(&envelope.sequence.to_be_bytes());
        bytes.extend_from_slice(&envelope.expires_at_ms.to_be_bytes());

        Ok(VerificationReceipt {
            message_id: envelope.message_id,
            subject_id: envelope.subject_id,
            sequence: envelope.sequence,
            envelope_digest: Digest32::of_bytes(&bytes),
            authority: AuthorityPosture::DENY_ALL,
        })
    }
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    bytes.extend_from_slice(&u32::try_from(raw.len()).unwrap_or(u32::MAX).to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
''',
    "codex-rs/hepta-authbus/src/lib_tests.rs": r'''use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn digest(value: &[u8]) -> Digest32 {
    Digest32::of_bytes(value)
}

fn envelope(sequence: u64) -> AuthEnvelope {
    AuthEnvelope {
        message_id: id(&format!("message:{sequence}")),
        subject_id: id("subject:1"),
        scope_digest: digest(b"scope"),
        payload_digest: digest(b"payload"),
        signature_digest: digest(b"signature"),
        sequence,
        expires_at_ms: 2_000,
        revoked: false,
    }
}

#[test]
fn exact_envelope_is_verified_without_authority_grant() {
    let mut window = ReplayWindow::new(8);
    let Ok(receipt) = window.verify(1_000, envelope(1), digest(b"scope"), digest(b"payload")) else {
        panic!("exact envelope must verify");
    };
    assert_eq!(receipt.sequence, 1);
    assert!(!receipt.authority.grants_any());
}

#[test]
fn replay_is_rejected() {
    let mut window = ReplayWindow::new(8);
    assert!(window
        .verify(1_000, envelope(1), digest(b"scope"), digest(b"payload"))
        .is_ok());
    assert_eq!(
        window.verify(1_000, envelope(1), digest(b"scope"), digest(b"payload")),
        Err(Error::Replay)
    );
}

#[test]
fn revoked_envelope_is_rejected() {
    let mut value = envelope(1);
    value.revoked = true;
    assert_eq!(
        ReplayWindow::new(8).verify(1_000, value, digest(b"scope"), digest(b"payload")),
        Err(Error::Revoked)
    );
}

#[test]
fn payload_drift_is_rejected() {
    assert_eq!(
        ReplayWindow::new(8).verify(
            1_000,
            envelope(1),
            digest(b"scope"),
            digest(b"other")
        ),
        Err(Error::PayloadMismatch)
    );
}

#[test]
fn expiration_is_fail_closed() {
    assert_eq!(
        ReplayWindow::new(8).verify(
            2_000,
            envelope(1),
            digest(b"scope"),
            digest(b"payload")
        ),
        Err(Error::Expired)
    );
}
''',
    "codex-rs/hepta-authbus-p1-3-qualification/Cargo.toml": manifest(
        "codex-hepta-authbus-p1-3-qualification",
        "codex_hepta_authbus_p1_3_qualification",
        (("codex-hepta-authbus", "hepta-authbus"),),
    ),
    "codex-rs/hepta-authbus-p1-3-qualification/BUILD.bazel": build(
        "hepta-authbus-p1-3-qualification"
    ),
    "codex-rs/hepta-authbus-p1-3-qualification/src/lib.rs": r'''//! Closed-world negative qualification for the AuthBus verifier.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::error::Error as StdError;
use std::fmt;

use codex_hepta_authbus::VerificationReceipt;
use codex_hepta_types::{AuthorityPosture, Digest32, StableId};

const MAX_CASES: usize = 32;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NegativeCase {
    Expired,
    Revoked,
    Replay,
    PayloadDrift,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaseEvidence {
    pub case: NegativeCase,
    pub case_id: StableId,
    pub rejected: bool,
    pub evidence_digest: Digest32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualificationReceipt {
    pub case_count: usize,
    pub qualification_digest: Digest32,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    CaseLimitExceeded,
    DuplicateCase,
    MissingRequiredCase,
    CaseDidNotReject(String),
    EmptyEvidence(String),
    PositiveReceiptGrantedAuthority,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

pub fn bind_positive_receipt(receipt: &VerificationReceipt) -> Result<Digest32, Error> {
    if receipt.authority.grants_any() {
        return Err(Error::PositiveReceiptGrantedAuthority);
    }
    Ok(receipt.envelope_digest)
}

pub fn qualify(mut cases: Vec<CaseEvidence>) -> Result<QualificationReceipt, Error> {
    if cases.len() > MAX_CASES {
        return Err(Error::CaseLimitExceeded);
    }
    cases.sort_by(|left, right| {
        left.case
            .cmp(&right.case)
            .then_with(|| left.case_id.cmp(&right.case_id))
    });
    let required = BTreeSet::from([
        NegativeCase::Expired,
        NegativeCase::Revoked,
        NegativeCase::Replay,
        NegativeCase::PayloadDrift,
    ]);
    let mut seen = BTreeSet::new();
    for evidence in &cases {
        if !seen.insert(evidence.case) {
            return Err(Error::DuplicateCase);
        }
        if !evidence.rejected {
            return Err(Error::CaseDidNotReject(evidence.case_id.to_string()));
        }
        if evidence.evidence_digest.is_zero() {
            return Err(Error::EmptyEvidence(evidence.case_id.to_string()));
        }
    }
    if seen != required {
        return Err(Error::MissingRequiredCase);
    }

    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.authbus.qualification.v1");
    for evidence in &cases {
        bytes.push(case_code(evidence.case));
        push_id(&mut bytes, &evidence.case_id);
        bytes.extend_from_slice(evidence.evidence_digest.as_array());
    }
    Ok(QualificationReceipt {
        case_count: cases.len(),
        qualification_digest: Digest32::of_bytes(&bytes),
        authority: AuthorityPosture::DENY_ALL,
    })
}

fn case_code(value: NegativeCase) -> u8 {
    match value {
        NegativeCase::Expired => 0,
        NegativeCase::Revoked => 1,
        NegativeCase::Replay => 2,
        NegativeCase::PayloadDrift => 3,
    }
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    bytes.extend_from_slice(&u32::try_from(raw.len()).unwrap_or(u32::MAX).to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
''',
    "codex-rs/hepta-authbus-p1-3-qualification/src/lib_tests.rs": r'''use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn cases() -> Vec<CaseEvidence> {
    [
        NegativeCase::Expired,
        NegativeCase::Revoked,
        NegativeCase::Replay,
        NegativeCase::PayloadDrift,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, case)| CaseEvidence {
        case,
        case_id: id(&format!("case:{index}")),
        rejected: true,
        evidence_digest: Digest32::of_bytes(format!("evidence:{index}").as_bytes()),
    })
    .collect()
}

#[test]
fn complete_negative_matrix_qualifies_without_authority() {
    let Ok(receipt) = qualify(cases()) else {
        panic!("complete matrix must qualify");
    };
    assert_eq!(receipt.case_count, 4);
    assert!(!receipt.authority.grants_any());
}

#[test]
fn missing_case_is_rejected() {
    let mut value = cases();
    value.pop();
    assert_eq!(qualify(value), Err(Error::MissingRequiredCase));
}

#[test]
fn unexpected_success_fails_qualification() {
    let mut value = cases();
    value[0].rejected = false;
    assert_eq!(
        qualify(value),
        Err(Error::CaseDidNotReject("case:0".to_string()))
    );
}
''',
    "codex-rs/hepta-control-plane/Cargo.toml": manifest(
        "codex-hepta-control-plane", "codex_hepta_control_plane"
    ),
    "codex-rs/hepta-control-plane/BUILD.bazel": build("hepta-control-plane"),
    "codex-rs/hepta-control-plane/src/lib.rs": r'''//! Revision- and authority-epoch-fenced runtime control state.
//!
//! Actions update desired control state only. They do not execute effects,
//! operate hardware, deploy, merge, promote or release.

#![forbid(unsafe_code)]

use std::error::Error as StdError;
use std::fmt;

use codex_hepta_types::{AuthorityPosture, Digest32, Revision, StableId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlMode {
    Ready,
    Quarantined,
    Recovering,
    RollbackRequested,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlState {
    pub revision: Revision,
    pub authority_epoch: u64,
    pub mode: ControlMode,
    pub configuration_digest: Digest32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlAction {
    Quarantine,
    BeginRecovery,
    RequestRollback,
    MarkReady,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlIntent {
    pub operation_id: StableId,
    pub expected_revision: Revision,
    pub expected_authority_epoch: u64,
    pub action: ControlAction,
    pub payload_digest: Digest32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlReceipt {
    pub operation_id: StableId,
    pub previous_revision: Revision,
    pub next_revision: Revision,
    pub mode: ControlMode,
    pub state_digest: Digest32,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptyDigest(&'static str),
    ZeroAuthorityEpoch,
    StaleRevision,
    StaleAuthorityEpoch,
    InvalidTransition,
    RevisionOverflow,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

pub fn apply(
    state: &ControlState,
    intent: ControlIntent,
) -> Result<(ControlState, ControlReceipt), Error> {
    if state.configuration_digest.is_zero() {
        return Err(Error::EmptyDigest("configuration"));
    }
    if intent.payload_digest.is_zero() {
        return Err(Error::EmptyDigest("intent payload"));
    }
    if state.authority_epoch == 0 || intent.expected_authority_epoch == 0 {
        return Err(Error::ZeroAuthorityEpoch);
    }
    if intent.expected_revision != state.revision {
        return Err(Error::StaleRevision);
    }
    if intent.expected_authority_epoch != state.authority_epoch {
        return Err(Error::StaleAuthorityEpoch);
    }
    let next_mode = transition(state.mode, intent.action)?;
    let next_revision = state.revision.next().map_err(|_| Error::RevisionOverflow)?;
    let next = ControlState {
        revision: next_revision,
        authority_epoch: state.authority_epoch,
        mode: next_mode,
        configuration_digest: state.configuration_digest,
    };
    let state_digest = digest_state(&next, &intent);
    let receipt = ControlReceipt {
        operation_id: intent.operation_id,
        previous_revision: state.revision,
        next_revision,
        mode: next_mode,
        state_digest,
        authority: AuthorityPosture::DENY_ALL,
    };
    Ok((next, receipt))
}

fn transition(mode: ControlMode, action: ControlAction) -> Result<ControlMode, Error> {
    match (mode, action) {
        (ControlMode::Ready, ControlAction::Quarantine) => Ok(ControlMode::Quarantined),
        (ControlMode::Quarantined, ControlAction::BeginRecovery) => Ok(ControlMode::Recovering),
        (ControlMode::Recovering, ControlAction::MarkReady) => Ok(ControlMode::Ready),
        (ControlMode::Quarantined, ControlAction::RequestRollback)
        | (ControlMode::Recovering, ControlAction::RequestRollback) => {
            Ok(ControlMode::RollbackRequested)
        }
        _ => Err(Error::InvalidTransition),
    }
}

fn digest_state(state: &ControlState, intent: &ControlIntent) -> Digest32 {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.control.state.v1");
    bytes.extend_from_slice(&state.revision.get().to_be_bytes());
    bytes.extend_from_slice(&state.authority_epoch.to_be_bytes());
    bytes.push(match state.mode {
        ControlMode::Ready => 0,
        ControlMode::Quarantined => 1,
        ControlMode::Recovering => 2,
        ControlMode::RollbackRequested => 3,
    });
    bytes.extend_from_slice(state.configuration_digest.as_array());
    bytes.extend_from_slice(intent.payload_digest.as_array());
    Digest32::of_bytes(&bytes)
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
''',
    "codex-rs/hepta-control-plane/src/lib_tests.rs": r'''use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn revision(value: u64) -> Revision {
    let Ok(value) = Revision::new(value) else {
        panic!("test revision must be valid");
    };
    value
}

fn state() -> ControlState {
    ControlState {
        revision: revision(1),
        authority_epoch: 7,
        mode: ControlMode::Ready,
        configuration_digest: Digest32::of_bytes(b"configuration"),
    }
}

fn intent(action: ControlAction) -> ControlIntent {
    ControlIntent {
        operation_id: id("operation:1"),
        expected_revision: revision(1),
        expected_authority_epoch: 7,
        action,
        payload_digest: Digest32::of_bytes(b"payload"),
    }
}

#[test]
fn quarantine_updates_desired_state_without_effect_authority() {
    let Ok((next, receipt)) = apply(&state(), intent(ControlAction::Quarantine)) else {
        panic!("quarantine transition must succeed");
    };
    assert_eq!(next.mode, ControlMode::Quarantined);
    assert!(!receipt.authority.grants_any());
}

#[test]
fn stale_revision_is_rejected() {
    let mut value = intent(ControlAction::Quarantine);
    value.expected_revision = revision(2);
    assert_eq!(apply(&state(), value), Err(Error::StaleRevision));
}

#[test]
fn stale_epoch_is_rejected() {
    let mut value = intent(ControlAction::Quarantine);
    value.expected_authority_epoch = 8;
    assert_eq!(apply(&state(), value), Err(Error::StaleAuthorityEpoch));
}

#[test]
fn invalid_transition_is_rejected() {
    assert_eq!(
        apply(&state(), intent(ControlAction::MarkReady)),
        Err(Error::InvalidTransition)
    );
}
''',
    "codex-rs/hepta-infer-core/Cargo.toml": manifest(
        "codex-hepta-infer-core", "codex_hepta_infer_core"
    ),
    "codex-rs/hepta-infer-core/BUILD.bazel": build("hepta-infer-core"),
    "codex-rs/hepta-infer-core/src/lib.rs": r'''//! Durable-style inference request and reservation state machine.
//!
//! No function in this crate dispatches a provider or executes a model.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt;

use codex_hepta_types::{AuthorityPosture, Digest32, StableId};

const MAX_REQUESTS: usize = 16_384;
const MAX_TOKENS: u32 = 1_000_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestStatus {
    Pending,
    Reserved,
    Completed,
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InferenceRequest {
    pub request_id: StableId,
    pub model_digest: Digest32,
    pub prompt_digest: Digest32,
    pub maximum_tokens: u32,
    pub deadline_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestRecord {
    pub request: InferenceRequest,
    pub request_digest: Digest32,
    pub status: RequestStatus,
    pub reservation_id: Option<StableId>,
    pub terminal_receipt_digest: Option<Digest32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Disposition {
    Inserted,
    Unchanged,
    Transitioned,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerReceipt {
    pub request_id: StableId,
    pub status: RequestStatus,
    pub disposition: Disposition,
    pub record_digest: Digest32,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    ZeroCapacity,
    CapacityExceeded,
    EmptyDigest(&'static str),
    InvalidMaximumTokens,
    RequestConflict(String),
    RequestNotFound(String),
    DigestMismatch,
    InvalidTransition,
    TerminalReceiptMissing,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InferenceLedger {
    records: BTreeMap<StableId, RequestRecord>,
    maximum_requests: usize,
}

impl InferenceLedger {
    pub fn new(maximum_requests: usize) -> Result<Self, Error> {
        if maximum_requests == 0 {
            return Err(Error::ZeroCapacity);
        }
        Ok(Self {
            records: BTreeMap::new(),
            maximum_requests: maximum_requests.min(MAX_REQUESTS),
        })
    }

    pub fn submit(&mut self, request: InferenceRequest) -> Result<LedgerReceipt, Error> {
        validate_request(&request)?;
        let digest = request_digest(&request);
        if let Some(existing) = self.records.get(&request.request_id) {
            if existing.request_digest == digest {
                return Ok(receipt(existing, Disposition::Unchanged));
            }
            return Err(Error::RequestConflict(request.request_id.to_string()));
        }
        if self.records.len() >= self.maximum_requests {
            return Err(Error::CapacityExceeded);
        }
        let record = RequestRecord {
            request,
            request_digest: digest,
            status: RequestStatus::Pending,
            reservation_id: None,
            terminal_receipt_digest: None,
        };
        let result = receipt(&record, Disposition::Inserted);
        self.records.insert(record.request.request_id.clone(), record);
        Ok(result)
    }

    pub fn reserve(
        &mut self,
        request_id: &StableId,
        expected_digest: Digest32,
        reservation_id: StableId,
    ) -> Result<LedgerReceipt, Error> {
        let Some(record) = self.records.get_mut(request_id) else {
            return Err(Error::RequestNotFound(request_id.to_string()));
        };
        if record.request_digest != expected_digest {
            return Err(Error::DigestMismatch);
        }
        if record.status != RequestStatus::Pending {
            return Err(Error::InvalidTransition);
        }
        record.status = RequestStatus::Reserved;
        record.reservation_id = Some(reservation_id);
        Ok(receipt(record, Disposition::Transitioned))
    }

    pub fn complete(
        &mut self,
        request_id: &StableId,
        terminal_receipt_digest: Digest32,
    ) -> Result<LedgerReceipt, Error> {
        if terminal_receipt_digest.is_zero() {
            return Err(Error::TerminalReceiptMissing);
        }
        let Some(record) = self.records.get_mut(request_id) else {
            return Err(Error::RequestNotFound(request_id.to_string()));
        };
        if record.status != RequestStatus::Reserved {
            return Err(Error::InvalidTransition);
        }
        record.status = RequestStatus::Completed;
        record.terminal_receipt_digest = Some(terminal_receipt_digest);
        Ok(receipt(record, Disposition::Transitioned))
    }

    pub fn cancel(&mut self, request_id: &StableId) -> Result<LedgerReceipt, Error> {
        let Some(record) = self.records.get_mut(request_id) else {
            return Err(Error::RequestNotFound(request_id.to_string()));
        };
        if matches!(record.status, RequestStatus::Completed | RequestStatus::Cancelled) {
            return Err(Error::InvalidTransition);
        }
        record.status = RequestStatus::Cancelled;
        Ok(receipt(record, Disposition::Transitioned))
    }

    #[must_use]
    pub fn get(&self, request_id: &StableId) -> Option<&RequestRecord> {
        self.records.get(request_id)
    }
}

#[must_use]
pub fn request_digest(request: &InferenceRequest) -> Digest32 {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.inference.core.request.v1");
    push_id(&mut bytes, &request.request_id);
    bytes.extend_from_slice(request.model_digest.as_array());
    bytes.extend_from_slice(request.prompt_digest.as_array());
    bytes.extend_from_slice(&request.maximum_tokens.to_be_bytes());
    bytes.extend_from_slice(&request.deadline_ms.to_be_bytes());
    Digest32::of_bytes(&bytes)
}

fn validate_request(request: &InferenceRequest) -> Result<(), Error> {
    if request.model_digest.is_zero() {
        return Err(Error::EmptyDigest("model"));
    }
    if request.prompt_digest.is_zero() {
        return Err(Error::EmptyDigest("prompt"));
    }
    if request.maximum_tokens == 0 || request.maximum_tokens > MAX_TOKENS {
        return Err(Error::InvalidMaximumTokens);
    }
    Ok(())
}

fn receipt(record: &RequestRecord, disposition: Disposition) -> LedgerReceipt {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.inference.core.record.v1");
    bytes.extend_from_slice(record.request_digest.as_array());
    bytes.push(match record.status {
        RequestStatus::Pending => 0,
        RequestStatus::Reserved => 1,
        RequestStatus::Completed => 2,
        RequestStatus::Cancelled => 3,
    });
    if let Some(value) = &record.reservation_id {
        push_id(&mut bytes, value);
    }
    if let Some(value) = record.terminal_receipt_digest {
        bytes.extend_from_slice(value.as_array());
    }
    LedgerReceipt {
        request_id: record.request.request_id.clone(),
        status: record.status,
        disposition,
        record_digest: Digest32::of_bytes(&bytes),
        authority: AuthorityPosture::DENY_ALL,
    }
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    bytes.extend_from_slice(&u32::try_from(raw.len()).unwrap_or(u32::MAX).to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
''',
    "codex-rs/hepta-infer-core/src/lib_tests.rs": r'''use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn request() -> InferenceRequest {
    InferenceRequest {
        request_id: id("request:1"),
        model_digest: Digest32::of_bytes(b"model"),
        prompt_digest: Digest32::of_bytes(b"prompt"),
        maximum_tokens: 128,
        deadline_ms: 2_000,
    }
}

fn ledger() -> InferenceLedger {
    let Ok(value) = InferenceLedger::new(8) else {
        panic!("test ledger must initialize");
    };
    value
}

#[test]
fn request_lifecycle_is_fenced_and_authority_free() {
    let mut value = ledger();
    let request = request();
    let digest = request_digest(&request);
    assert!(value.submit(request).is_ok());
    assert!(value
        .reserve(&id("request:1"), digest, id("reservation:1"))
        .is_ok());
    let Ok(receipt) = value.complete(
        &id("request:1"),
        Digest32::of_bytes(b"terminal-receipt"),
    ) else {
        panic!("completion must succeed");
    };
    assert_eq!(receipt.status, RequestStatus::Completed);
    assert!(!receipt.authority.grants_any());
}

#[test]
fn conflicting_identity_is_rejected() {
    let mut value = ledger();
    assert!(value.submit(request()).is_ok());
    let mut drifted = request();
    drifted.prompt_digest = Digest32::of_bytes(b"other");
    assert_eq!(
        value.submit(drifted),
        Err(Error::RequestConflict("request:1".to_string()))
    );
}

#[test]
fn stale_digest_cannot_reserve() {
    let mut value = ledger();
    assert!(value.submit(request()).is_ok());
    assert_eq!(
        value.reserve(
            &id("request:1"),
            Digest32::of_bytes(b"stale"),
            id("reservation:1")
        ),
        Err(Error::DigestMismatch)
    );
}

#[test]
fn completion_requires_reservation() {
    let mut value = ledger();
    assert!(value.submit(request()).is_ok());
    assert_eq!(
        value.complete(
            &id("request:1"),
            Digest32::of_bytes(b"terminal-receipt")
        ),
        Err(Error::InvalidTransition)
    );
}
''',
    "codex-rs/hepta-inferd/Cargo.toml": manifest(
        "codex-hepta-inferd", "codex_hepta_inferd"
    ),
    "codex-rs/hepta-inferd/BUILD.bazel": build("hepta-inferd"),
    "codex-rs/hepta-inferd/src/lib.rs": r'''//! Exact-bound inference dispatch planning.
//!
//! A dispatch plan names a worker and frozen request/reservation/lease digests.
//! It is not provider dispatch authority and does not execute a model.

#![forbid(unsafe_code)]

use std::error::Error as StdError;
use std::fmt;

use codex_hepta_types::{AuthorityPosture, Digest32, StableId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatchRequest {
    pub dispatch_id: StableId,
    pub request_id: StableId,
    pub worker_id: StableId,
    pub request_digest: Digest32,
    pub reservation_digest: Digest32,
    pub lease_digest: Digest32,
    pub model_digest: Digest32,
    pub deadline_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatchPlan {
    pub dispatch_id: StableId,
    pub request_id: StableId,
    pub worker_id: StableId,
    pub plan_digest: Digest32,
    pub provider_dispatch_authority: bool,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptyDigest(&'static str),
    DeadlineExpired,
    BindingMismatch(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

pub fn plan(
    now_ms: u64,
    request: DispatchRequest,
    expected_request: Digest32,
    expected_reservation: Digest32,
    expected_lease: Digest32,
    expected_model: Digest32,
) -> Result<DispatchPlan, Error> {
    for (name, digest) in [
        ("request", request.request_digest),
        ("reservation", request.reservation_digest),
        ("lease", request.lease_digest),
        ("model", request.model_digest),
    ] {
        if digest.is_zero() {
            return Err(Error::EmptyDigest(name));
        }
    }
    if now_ms >= request.deadline_ms {
        return Err(Error::DeadlineExpired);
    }
    for (name, actual, expected) in [
        ("request", request.request_digest, expected_request),
        (
            "reservation",
            request.reservation_digest,
            expected_reservation,
        ),
        ("lease", request.lease_digest, expected_lease),
        ("model", request.model_digest, expected_model),
    ] {
        if actual != expected {
            return Err(Error::BindingMismatch(name));
        }
    }
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.inferd.plan.v1");
    push_id(&mut bytes, &request.dispatch_id);
    push_id(&mut bytes, &request.request_id);
    push_id(&mut bytes, &request.worker_id);
    bytes.extend_from_slice(request.request_digest.as_array());
    bytes.extend_from_slice(request.reservation_digest.as_array());
    bytes.extend_from_slice(request.lease_digest.as_array());
    bytes.extend_from_slice(request.model_digest.as_array());
    Ok(DispatchPlan {
        dispatch_id: request.dispatch_id,
        request_id: request.request_id,
        worker_id: request.worker_id,
        plan_digest: Digest32::of_bytes(&bytes),
        provider_dispatch_authority: false,
        authority: AuthorityPosture::DENY_ALL,
    })
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    bytes.extend_from_slice(&u32::try_from(raw.len()).unwrap_or(u32::MAX).to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
''',
    "codex-rs/hepta-inferd/src/lib_tests.rs": r'''use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn digest(value: &[u8]) -> Digest32 {
    Digest32::of_bytes(value)
}

fn request() -> DispatchRequest {
    DispatchRequest {
        dispatch_id: id("dispatch:1"),
        request_id: id("request:1"),
        worker_id: id("worker:1"),
        request_digest: digest(b"request"),
        reservation_digest: digest(b"reservation"),
        lease_digest: digest(b"lease"),
        model_digest: digest(b"model"),
        deadline_ms: 2_000,
    }
}

#[test]
fn exact_plan_grants_no_provider_authority() {
    let value = request();
    let Ok(plan) = plan(
        1_000,
        value,
        digest(b"request"),
        digest(b"reservation"),
        digest(b"lease"),
        digest(b"model"),
    ) else {
        panic!("exact plan must succeed");
    };
    assert!(!plan.provider_dispatch_authority);
    assert!(!plan.authority.grants_any());
}

#[test]
fn lease_drift_is_rejected() {
    assert_eq!(
        plan(
            1_000,
            request(),
            digest(b"request"),
            digest(b"reservation"),
            digest(b"other-lease"),
            digest(b"model"),
        ),
        Err(Error::BindingMismatch("lease"))
    );
}

#[test]
fn expired_dispatch_is_rejected() {
    assert_eq!(
        plan(
            2_000,
            request(),
            digest(b"request"),
            digest(b"reservation"),
            digest(b"lease"),
            digest(b"model"),
        ),
        Err(Error::DeadlineExpired)
    );
}
''',
    "codex-rs/hepta-intelligence/Cargo.toml": manifest(
        "codex-hepta-intelligence", "codex_hepta_intelligence"
    ),
    "codex-rs/hepta-intelligence/BUILD.bazel": build("hepta-intelligence"),
    "codex-rs/hepta-intelligence/src/lib.rs": r'''//! Bounded intelligence composition and abstention.
//!
//! The output is a plan receipt. It cannot invoke a model, tool or provider,
//! execute an effect, mutate current learning state, promote or release.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::error::Error as StdError;
use std::fmt;

use codex_hepta_types::{AuthorityPosture, Digest32, FixedQ32, StableId};

const MAX_CANDIDATES: usize = 4_096;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanCandidate {
    pub candidate_id: StableId,
    pub legal: bool,
    pub hard_veto: bool,
    pub score: FixedQ32,
    pub support_digest: Digest32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningRequest {
    pub plan_id: StableId,
    pub objective_digest: Digest32,
    pub context_digest: Digest32,
    pub snapshot_digest: Digest32,
    pub candidates: Vec<PlanCandidate>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AbstentionReason {
    NoEligibleCandidate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanDecision {
    Selected(StableId),
    Abstained(AbstentionReason),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntelligencePlanReceipt {
    pub plan_id: StableId,
    pub decision: PlanDecision,
    pub considered_candidates: Vec<StableId>,
    pub plan_digest: Digest32,
    pub effect_authority: bool,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptyDigest(&'static str),
    CandidateLimitExceeded,
    DuplicateCandidate(String),
    EmptySupport(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

pub fn compose(mut request: PlanningRequest) -> Result<IntelligencePlanReceipt, Error> {
    for (name, digest) in [
        ("objective", request.objective_digest),
        ("context", request.context_digest),
        ("snapshot", request.snapshot_digest),
    ] {
        if digest.is_zero() {
            return Err(Error::EmptyDigest(name));
        }
    }
    if request.candidates.len() > MAX_CANDIDATES {
        return Err(Error::CandidateLimitExceeded);
    }
    request
        .candidates
        .sort_by(|left, right| left.candidate_id.cmp(&right.candidate_id));
    let mut seen = BTreeSet::new();
    for candidate in &request.candidates {
        if !seen.insert(candidate.candidate_id.clone()) {
            return Err(Error::DuplicateCandidate(candidate.candidate_id.to_string()));
        }
        if candidate.support_digest.is_zero() {
            return Err(Error::EmptySupport(candidate.candidate_id.to_string()));
        }
    }
    let selected = request
        .candidates
        .iter()
        .filter(|candidate| candidate.legal && !candidate.hard_veto)
        .max_by(|left, right| {
            left.score
                .cmp(&right.score)
                .then_with(|| right.candidate_id.cmp(&left.candidate_id))
        });
    let decision = selected.map_or(
        PlanDecision::Abstained(AbstentionReason::NoEligibleCandidate),
        |candidate| PlanDecision::Selected(candidate.candidate_id.clone()),
    );
    let considered_candidates = request
        .candidates
        .iter()
        .map(|candidate| candidate.candidate_id.clone())
        .collect::<Vec<_>>();
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.intelligence.plan.v1");
    push_id(&mut bytes, &request.plan_id);
    bytes.extend_from_slice(request.objective_digest.as_array());
    bytes.extend_from_slice(request.context_digest.as_array());
    bytes.extend_from_slice(request.snapshot_digest.as_array());
    for candidate in &request.candidates {
        push_id(&mut bytes, &candidate.candidate_id);
        bytes.push(u8::from(candidate.legal));
        bytes.push(u8::from(candidate.hard_veto));
        bytes.extend_from_slice(&candidate.score.raw().to_be_bytes());
        bytes.extend_from_slice(candidate.support_digest.as_array());
    }
    Ok(IntelligencePlanReceipt {
        plan_id: request.plan_id,
        decision,
        considered_candidates,
        plan_digest: Digest32::of_bytes(&bytes),
        effect_authority: false,
        authority: AuthorityPosture::DENY_ALL,
    })
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    bytes.extend_from_slice(&u32::try_from(raw.len()).unwrap_or(u32::MAX).to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
''',
    "codex-rs/hepta-intelligence/src/lib_tests.rs": r'''use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn candidate(name: &str, score: i64) -> PlanCandidate {
    PlanCandidate {
        candidate_id: id(name),
        legal: true,
        hard_veto: false,
        score: FixedQ32::from_raw(score),
        support_digest: Digest32::of_bytes(name.as_bytes()),
    }
}

fn request(candidates: Vec<PlanCandidate>) -> PlanningRequest {
    PlanningRequest {
        plan_id: id("plan:1"),
        objective_digest: Digest32::of_bytes(b"objective"),
        context_digest: Digest32::of_bytes(b"context"),
        snapshot_digest: Digest32::of_bytes(b"snapshot"),
        candidates,
    }
}

#[test]
fn highest_eligible_candidate_is_selected_without_effect_authority() {
    let Ok(receipt) = compose(request(vec![
        candidate("candidate:a", 10),
        candidate("candidate:b", 20),
    ])) else {
        panic!("planning must succeed");
    };
    assert_eq!(
        receipt.decision,
        PlanDecision::Selected(id("candidate:b"))
    );
    assert!(!receipt.effect_authority);
    assert!(!receipt.authority.grants_any());
}

#[test]
fn hard_veto_is_never_overridden() {
    let allowed = candidate("candidate:allowed", 10);
    let mut vetoed = candidate("candidate:vetoed", 100);
    vetoed.hard_veto = true;
    let Ok(receipt) = compose(request(vec![vetoed, allowed])) else {
        panic!("planning must succeed");
    };
    assert_eq!(
        receipt.decision,
        PlanDecision::Selected(id("candidate:allowed"))
    );
}

#[test]
fn no_eligible_candidate_abstains() {
    let mut value = candidate("candidate:a", 10);
    value.legal = false;
    let Ok(receipt) = compose(request(vec![value])) else {
        panic!("planning must succeed");
    };
    assert_eq!(
        receipt.decision,
        PlanDecision::Abstained(AbstentionReason::NoEligibleCandidate)
    );
}

#[test]
fn duplicate_candidate_is_rejected() {
    let value = candidate("candidate:a", 10);
    assert_eq!(
        compose(request(vec![value.clone(), value])),
        Err(Error::DuplicateCandidate("candidate:a".to_string()))
    );
}
''',
    "codex-rs/hepta-codex-adapter/Cargo.toml": manifest(
        "codex-hepta-codex-adapter", "codex_hepta_codex_adapter"
    ),
    "codex-rs/hepta-codex-adapter/BUILD.bazel": build("hepta-codex-adapter"),
    "codex-rs/hepta-codex-adapter/src/lib.rs": r'''//! Exact-bound Codex app-server request adapter.
//!
//! The adapter translates an already-authorized intent and observes a terminal
//! app-server outcome. It does not mint model/provider authority.

#![forbid(unsafe_code)]

use std::error::Error as StdError;
use std::fmt;

use codex_hepta_types::{AuthorityPosture, Digest32, StableId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodexOperationIntent {
    pub operation_id: StableId,
    pub thread_id: StableId,
    pub method_id: StableId,
    pub payload_digest: Digest32,
    pub lease_payload_digest: Digest32,
    pub deadline_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppServerObservation {
    pub terminal_observed: bool,
    pub response_digest: Digest32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterStatus {
    Succeeded,
    Indeterminate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodexAdapterReceipt {
    pub operation_id: StableId,
    pub request_digest: Digest32,
    pub status: AdapterStatus,
    pub response_digest: Option<Digest32>,
    pub model_authority: bool,
    pub provider_authority: bool,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptyDigest(&'static str),
    PayloadBindingMismatch,
    DeadlineExpired,
    MissingTerminalResponse,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

pub fn adapt(
    now_ms: u64,
    intent: CodexOperationIntent,
    observation: Option<AppServerObservation>,
) -> Result<CodexAdapterReceipt, Error> {
    if intent.payload_digest.is_zero() || intent.lease_payload_digest.is_zero() {
        return Err(Error::EmptyDigest("payload"));
    }
    if intent.payload_digest != intent.lease_payload_digest {
        return Err(Error::PayloadBindingMismatch);
    }
    if now_ms >= intent.deadline_ms {
        return Err(Error::DeadlineExpired);
    }
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.codex.adapter.request.v1");
    push_id(&mut bytes, &intent.operation_id);
    push_id(&mut bytes, &intent.thread_id);
    push_id(&mut bytes, &intent.method_id);
    bytes.extend_from_slice(intent.payload_digest.as_array());
    let request_digest = Digest32::of_bytes(&bytes);
    let (status, response_digest) = match observation {
        None => (AdapterStatus::Indeterminate, None),
        Some(value) if !value.terminal_observed => (AdapterStatus::Indeterminate, None),
        Some(value) => {
            if value.response_digest.is_zero() {
                return Err(Error::MissingTerminalResponse);
            }
            (AdapterStatus::Succeeded, Some(value.response_digest))
        }
    };
    Ok(CodexAdapterReceipt {
        operation_id: intent.operation_id,
        request_digest,
        status,
        response_digest,
        model_authority: false,
        provider_authority: false,
        authority: AuthorityPosture::DENY_ALL,
    })
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    bytes.extend_from_slice(&u32::try_from(raw.len()).unwrap_or(u32::MAX).to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
''',
    "codex-rs/hepta-codex-adapter/src/lib_tests.rs": r'''use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn digest(value: &[u8]) -> Digest32 {
    Digest32::of_bytes(value)
}

fn intent() -> CodexOperationIntent {
    CodexOperationIntent {
        operation_id: id("operation:1"),
        thread_id: id("thread:1"),
        method_id: id("method:1"),
        payload_digest: digest(b"payload"),
        lease_payload_digest: digest(b"payload"),
        deadline_ms: 2_000,
    }
}

#[test]
fn exact_terminal_observation_maps_without_authority() {
    let observation = AppServerObservation {
        terminal_observed: true,
        response_digest: digest(b"response"),
    };
    let Ok(receipt) = adapt(1_000, intent(), Some(observation)) else {
        panic!("terminal observation must succeed");
    };
    assert_eq!(receipt.status, AdapterStatus::Succeeded);
    assert!(!receipt.model_authority);
    assert!(!receipt.provider_authority);
    assert!(!receipt.authority.grants_any());
}

#[test]
fn missing_observation_is_indeterminate() {
    let Ok(receipt) = adapt(1_000, intent(), None) else {
        panic!("unknown outcome must be represented");
    };
    assert_eq!(receipt.status, AdapterStatus::Indeterminate);
    assert_eq!(receipt.response_digest, None);
}

#[test]
fn payload_drift_is_rejected() {
    let mut value = intent();
    value.lease_payload_digest = digest(b"other");
    assert_eq!(
        adapt(1_000, value, None),
        Err(Error::PayloadBindingMismatch)
    );
}
''',
    "codex-rs/hepta-bao-adapter/Cargo.toml": manifest(
        "codex-hepta-bao-adapter", "codex_hepta_bao_adapter"
    ),
    "codex-rs/hepta-bao-adapter/BUILD.bazel": build("hepta-bao-adapter"),
    "codex-rs/hepta-bao-adapter/src/lib.rs": r'''//! Opaque HeptaBao secret-reference adapter.
//!
//! Raw secret bytes never enter this API. The adapter verifies a scoped lease
//! and returns only a deterministic opaque handle digest.

#![forbid(unsafe_code)]

use std::error::Error as StdError;
use std::fmt;

use codex_hepta_types::{AuthorityPosture, Digest32, StableId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecretReference {
    pub secret_id: StableId,
    pub version: u64,
    pub secret_digest: Digest32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecretLease {
    pub lease_id: StableId,
    pub secret_id: StableId,
    pub version: u64,
    pub secret_digest: Digest32,
    pub scope_digest: Digest32,
    pub expires_at_ms: u64,
    pub revoked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecretRequest {
    pub request_id: StableId,
    pub reference: SecretReference,
    pub scope_digest: Digest32,
    pub deadline_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpaqueSecretReceipt {
    pub request_id: StableId,
    pub lease_id: StableId,
    pub opaque_handle_digest: Digest32,
    pub contains_raw_secret: bool,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptyDigest(&'static str),
    ZeroVersion,
    DeadlineExpired,
    LeaseExpired,
    LeaseRevoked,
    IdentityMismatch,
    VersionMismatch,
    SecretDigestMismatch,
    ScopeMismatch,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

pub fn resolve(
    now_ms: u64,
    request: SecretRequest,
    lease: SecretLease,
) -> Result<OpaqueSecretReceipt, Error> {
    if request.reference.version == 0 || lease.version == 0 {
        return Err(Error::ZeroVersion);
    }
    for (name, digest) in [
        ("secret", request.reference.secret_digest),
        ("scope", request.scope_digest),
        ("lease secret", lease.secret_digest),
        ("lease scope", lease.scope_digest),
    ] {
        if digest.is_zero() {
            return Err(Error::EmptyDigest(name));
        }
    }
    if now_ms >= request.deadline_ms {
        return Err(Error::DeadlineExpired);
    }
    if lease.revoked {
        return Err(Error::LeaseRevoked);
    }
    if now_ms >= lease.expires_at_ms {
        return Err(Error::LeaseExpired);
    }
    if lease.secret_id != request.reference.secret_id {
        return Err(Error::IdentityMismatch);
    }
    if lease.version != request.reference.version {
        return Err(Error::VersionMismatch);
    }
    if lease.secret_digest != request.reference.secret_digest {
        return Err(Error::SecretDigestMismatch);
    }
    if lease.scope_digest != request.scope_digest {
        return Err(Error::ScopeMismatch);
    }
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.bao.opaque-handle.v1");
    push_id(&mut bytes, &request.request_id);
    push_id(&mut bytes, &lease.lease_id);
    push_id(&mut bytes, &request.reference.secret_id);
    bytes.extend_from_slice(&request.reference.version.to_be_bytes());
    bytes.extend_from_slice(request.reference.secret_digest.as_array());
    bytes.extend_from_slice(request.scope_digest.as_array());
    Ok(OpaqueSecretReceipt {
        request_id: request.request_id,
        lease_id: lease.lease_id,
        opaque_handle_digest: Digest32::of_bytes(&bytes),
        contains_raw_secret: false,
        authority: AuthorityPosture::DENY_ALL,
    })
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    bytes.extend_from_slice(&u32::try_from(raw.len()).unwrap_or(u32::MAX).to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
''',
    "codex-rs/hepta-bao-adapter/src/lib_tests.rs": r'''use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn digest(value: &[u8]) -> Digest32 {
    Digest32::of_bytes(value)
}

fn fixture() -> (SecretRequest, SecretLease) {
    let reference = SecretReference {
        secret_id: id("secret:1"),
        version: 3,
        secret_digest: digest(b"secret-digest"),
    };
    let request = SecretRequest {
        request_id: id("request:1"),
        reference: reference.clone(),
        scope_digest: digest(b"scope"),
        deadline_ms: 2_000,
    };
    let lease = SecretLease {
        lease_id: id("lease:1"),
        secret_id: reference.secret_id,
        version: reference.version,
        secret_digest: reference.secret_digest,
        scope_digest: request.scope_digest,
        expires_at_ms: 1_500,
        revoked: false,
    };
    (request, lease)
}

#[test]
fn exact_reference_returns_only_opaque_digest() {
    let (request, lease) = fixture();
    let Ok(receipt) = resolve(1_000, request, lease) else {
        panic!("exact secret reference must resolve");
    };
    assert!(!receipt.contains_raw_secret);
    assert!(!receipt.authority.grants_any());
}

#[test]
fn revoked_lease_is_rejected() {
    let (request, mut lease) = fixture();
    lease.revoked = true;
    assert_eq!(resolve(1_000, request, lease), Err(Error::LeaseRevoked));
}

#[test]
fn scope_drift_is_rejected() {
    let (request, mut lease) = fixture();
    lease.scope_digest = digest(b"other");
    assert_eq!(resolve(1_000, request, lease), Err(Error::ScopeMismatch));
}

#[test]
fn version_drift_is_rejected() {
    let (request, mut lease) = fixture();
    lease.version = 4;
    assert_eq!(resolve(1_000, request, lease), Err(Error::VersionMismatch));
}
''',
}

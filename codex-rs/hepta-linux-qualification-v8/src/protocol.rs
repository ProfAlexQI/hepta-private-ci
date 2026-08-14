use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::AttemptIdentityV8;
use crate::AuthoritySignatureAlgorithmV8;
use crate::AuthoritySignerBindingV8;
use crate::CryptographicSignatureObservation;
use crate::JournalAssessmentV8;
use crate::QualificationError;
use crate::RecoveryStateBindingV8;
use crate::VerifiedAuthorityV8;
use crate::invalid;

pub const COPY_ACK_NAMESPACE_V8: &str = "hepta-linux-v8-copy-ack";
pub const MAX_COPY_ACK_LIFETIME_SECONDS_V8: u64 = 15 * 60;
const MAX_COPY_ACK_SIGNATURE_BYTES_V8: usize = 16 * 1024;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CandidateOutcomeV8 {
    Crash,
    Fail,
    Pass,
    Timeout,
}

impl CandidateOutcomeV8 {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Crash => "CRASH",
            Self::Fail => "FAIL",
            Self::Pass => "PASS",
            Self::Timeout => "TIMEOUT",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NoReplacePublicationMethodV8 {
    RenameAt2NoReplaceFileAndDirectoryFsync,
}

impl NoReplacePublicationMethodV8 {
    const fn as_str(self) -> &'static str {
        match self {
            Self::RenameAt2NoReplaceFileAndDirectoryFsync => {
                "renameat2_noreplace_file_and_directory_fsync"
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublishedFileIdentityV8 {
    pub device: u64,
    pub inode: u64,
    pub mode: u32,
    pub nlink: u64,
    pub sha256: String,
    pub size_bytes: u64,
}

impl PublishedFileIdentityV8 {
    pub fn validate_private_regular_file(&self) -> Result<(), QualificationError> {
        if self.device == 0
            || self.inode == 0
            || self.mode != 0o600
            || self.nlink != 1
            || self.size_bytes == 0
            || !hex64(&self.sha256)
        {
            return Err(invalid("published file identity is not exact and private"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateContainmentEvidenceV8 {
    pub attempt_identity_sha256: String,
    pub cgroup_path: String,
    pub delegated_controller_count: u32,
    pub observed_process_count: u32,
    pub owner_gid: u32,
    pub owner_uid: u32,
    pub populated_value: u32,
    pub root_observation_sha256: String,
}

impl CandidateContainmentEvidenceV8 {
    pub fn validate(&self, expected_attempt: &str) -> Result<(), QualificationError> {
        if self.attempt_identity_sha256 != expected_attempt
            || !self.cgroup_path.starts_with("/hepta-vnext/linux-v8/")
            || self.cgroup_path.contains("..")
            || self.owner_uid != 0
            || self.owner_gid != 0
            || self.delegated_controller_count != 0
            || self.observed_process_count != 0
            || self.populated_value != 0
            || !hex64(&self.root_observation_sha256)
        {
            return Err(invalid("candidate containment is not root-owned and empty"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateResultBundleV8 {
    pub attempt: AttemptIdentityV8,
    pub containment: CandidateContainmentEvidenceV8,
    pub manifest_sha256: String,
    pub outcome: CandidateOutcomeV8,
    pub publication_method: NoReplacePublicationMethodV8,
    pub source: PublishedFileIdentityV8,
}

impl CandidateResultBundleV8 {
    pub fn validate(&self) -> Result<(), QualificationError> {
        self.attempt.validate()?;
        let attempt_sha256 = self.attempt.sha256()?;
        self.containment.validate(&attempt_sha256)?;
        self.source.validate_private_regular_file()?;
        if !hex64(&self.manifest_sha256) {
            return Err(invalid("candidate result manifest digest is malformed"));
        }
        Ok(())
    }

    pub fn sha256(&self) -> Result<String, QualificationError> {
        self.validate()?;
        let attempt = self.attempt.sha256()?;
        let mut bytes = Vec::new();
        push(&mut bytes, b"hepta_linux_v8_candidate_result_bundle_v1");
        push(&mut bytes, attempt.as_bytes());
        push(&mut bytes, self.outcome.as_str().as_bytes());
        push(&mut bytes, self.publication_method.as_str().as_bytes());
        push(&mut bytes, self.manifest_sha256.as_bytes());
        push(&mut bytes, self.source.sha256.as_bytes());
        push_u64(&mut bytes, self.source.device);
        push_u64(&mut bytes, self.source.inode);
        push_u64(&mut bytes, u64::from(self.source.mode));
        push_u64(&mut bytes, self.source.nlink);
        push_u64(&mut bytes, self.source.size_bytes);
        push(&mut bytes, self.containment.cgroup_path.as_bytes());
        push_u64(
            &mut bytes,
            u64::from(self.containment.delegated_controller_count),
        );
        push_u64(
            &mut bytes,
            u64::from(self.containment.observed_process_count),
        );
        push_u64(&mut bytes, u64::from(self.containment.owner_uid));
        push_u64(&mut bytes, u64::from(self.containment.owner_gid));
        push_u64(&mut bytes, u64::from(self.containment.populated_value));
        push(
            &mut bytes,
            self.containment.root_observation_sha256.as_bytes(),
        );
        Ok(sha256(&bytes))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MacCopyAckV8 {
    pub allowed_signers_sha256: String,
    pub attempt_identity_sha256: String,
    pub candidate_result_bundle_sha256: String,
    pub challenge_nonce: String,
    pub copied_manifest_sha256: String,
    pub copied_publication: PublishedFileIdentityV8,
    pub issued_unix_seconds: u64,
    pub linux_source_device: u64,
    pub linux_source_inode: u64,
    pub principal: String,
    pub signature_bytes: Vec<u8>,
    pub signer_fingerprint: String,
    pub valid_before_unix_seconds: u64,
}

impl MacCopyAckV8 {
    pub fn canonical_statement(&self) -> Result<Vec<u8>, QualificationError> {
        self.validate_shape()?;
        let mut bytes = Vec::new();
        push(&mut bytes, b"hepta_linux_v8_mac_copy_ack_statement_v1");
        push(&mut bytes, COPY_ACK_NAMESPACE_V8.as_bytes());
        push(&mut bytes, self.challenge_nonce.as_bytes());
        push(&mut bytes, self.attempt_identity_sha256.as_bytes());
        push(&mut bytes, self.candidate_result_bundle_sha256.as_bytes());
        push_u64(&mut bytes, self.linux_source_device);
        push_u64(&mut bytes, self.linux_source_inode);
        push(&mut bytes, self.copied_manifest_sha256.as_bytes());
        push(&mut bytes, self.copied_publication.sha256.as_bytes());
        push_u64(&mut bytes, self.copied_publication.device);
        push_u64(&mut bytes, self.copied_publication.inode);
        push_u64(&mut bytes, u64::from(self.copied_publication.mode));
        push_u64(&mut bytes, self.copied_publication.nlink);
        push_u64(&mut bytes, self.copied_publication.size_bytes);
        push(&mut bytes, self.principal.as_bytes());
        push(&mut bytes, self.signer_fingerprint.as_bytes());
        push(&mut bytes, self.allowed_signers_sha256.as_bytes());
        push_u64(&mut bytes, self.issued_unix_seconds);
        push_u64(&mut bytes, self.valid_before_unix_seconds);
        Ok(bytes)
    }

    fn validate_shape(&self) -> Result<(), QualificationError> {
        self.copied_publication.validate_private_regular_file()?;
        self.signer_binding().validate()?;
        if !hex64(&self.allowed_signers_sha256)
            || !hex64(&self.attempt_identity_sha256)
            || !hex64(&self.candidate_result_bundle_sha256)
            || !hex64(&self.challenge_nonce)
            || !hex64(&self.copied_manifest_sha256)
            || self.linux_source_device == 0
            || self.linux_source_inode == 0
            || self.signature_bytes.is_empty()
            || self.signature_bytes.len() > MAX_COPY_ACK_SIGNATURE_BYTES_V8
            || self.issued_unix_seconds >= self.valid_before_unix_seconds
            || self.valid_before_unix_seconds - self.issued_unix_seconds
                > MAX_COPY_ACK_LIFETIME_SECONDS_V8
        {
            return Err(invalid("Mac copy acknowledgement shape is invalid"));
        }
        Ok(())
    }

    fn signer_binding(&self) -> AuthoritySignerBindingV8 {
        AuthoritySignerBindingV8 {
            allowed_signers_sha256: self.allowed_signers_sha256.clone(),
            key_fingerprint: self.signer_fingerprint.clone(),
            principal: self.principal.clone(),
            signature_algorithm: AuthoritySignatureAlgorithmV8::OpenSshSshsigEd25519,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedCopyAckV8 {
    attempt_identity_sha256: String,
    candidate_result_bundle_sha256: String,
    challenge_nonce: String,
    copied_manifest_sha256: String,
    signature_sha256: String,
    statement_sha256: String,
}

impl VerifiedCopyAckV8 {
    pub fn attempt_identity_sha256(&self) -> &str {
        &self.attempt_identity_sha256
    }

    pub fn candidate_result_bundle_sha256(&self) -> &str {
        &self.candidate_result_bundle_sha256
    }

    pub fn challenge_nonce(&self) -> &str {
        &self.challenge_nonce
    }

    pub fn copied_manifest_sha256(&self) -> &str {
        &self.copied_manifest_sha256
    }

    pub fn signature_sha256(&self) -> &str {
        &self.signature_sha256
    }

    pub fn statement_sha256(&self) -> &str {
        &self.statement_sha256
    }
}

#[derive(Debug, Default)]
pub struct CopyAckReplayGuardV8 {
    consumed_nonces: BTreeSet<String>,
}

impl CopyAckReplayGuardV8 {
    pub fn from_consumed_nonces(
        consumed_nonces: impl IntoIterator<Item = String>,
    ) -> Result<Self, QualificationError> {
        let consumed_nonces = consumed_nonces.into_iter().collect::<BTreeSet<_>>();
        if consumed_nonces.iter().any(|nonce| !hex64(nonce)) {
            return Err(invalid("persisted copy-ack nonce claim is malformed"));
        }
        Ok(Self { consumed_nonces })
    }

    pub fn verify_and_consume(
        &mut self,
        ack: &MacCopyAckV8,
        candidate: &CandidateResultBundleV8,
        observation: &CryptographicSignatureObservation,
        now_unix_seconds: u64,
    ) -> Result<VerifiedCopyAckV8, QualificationError> {
        candidate.validate()?;
        let attempt_sha256 = candidate.attempt.sha256()?;
        let candidate_sha256 = candidate.sha256()?;
        let statement_sha256 = sha256(&ack.canonical_statement()?);
        let signature_sha256 = sha256(&ack.signature_bytes);
        let signer = ack.signer_binding();
        if ack.attempt_identity_sha256 != attempt_sha256
            || ack.candidate_result_bundle_sha256 != candidate_sha256
            || ack.linux_source_device != candidate.source.device
            || ack.linux_source_inode != candidate.source.inode
            || now_unix_seconds < ack.issued_unix_seconds
            || now_unix_seconds >= ack.valid_before_unix_seconds
            || !observation.exactly_matches(
                &signature_sha256,
                &statement_sha256,
                COPY_ACK_NAMESPACE_V8,
                &signer,
            )
        {
            return Err(invalid(
                "copy acknowledgement is stale or cryptographically misbound",
            ));
        }
        if !self.consumed_nonces.insert(ack.challenge_nonce.clone()) {
            return Err(invalid("copy acknowledgement challenge was replayed"));
        }
        Ok(VerifiedCopyAckV8 {
            attempt_identity_sha256: attempt_sha256,
            candidate_result_bundle_sha256: candidate_sha256,
            challenge_nonce: ack.challenge_nonce.clone(),
            copied_manifest_sha256: ack.copied_manifest_sha256.clone(),
            signature_sha256,
            statement_sha256,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdmissionPhaseV8 {
    Unarmed,
    Armed,
    Claimed,
    RunnerStopped,
    CandidatePublished,
    CopyAcknowledged,
    RunnerRestored,
    PostSnapshot,
    Released,
    Abandoned,
}

#[derive(Clone, Debug)]
pub struct AdmissionStateV8 {
    attempt: AttemptIdentityV8,
    barrier_armed: bool,
    permanent_quarantine: bool,
    phase: AdmissionPhaseV8,
    qualification_abandoned: bool,
    qualification_pass: bool,
    active_claim_sha256: Option<String>,
    candidate_result: Option<CandidateResultBundleV8>,
    copy_ack_statement_sha256: Option<String>,
    post_snapshot_sha256: Option<String>,
    restore_evidence_sha256: Option<String>,
    runner_stop_evidence_sha256: Option<String>,
    verified_journal_tip_sha256: Option<String>,
}

impl AdmissionStateV8 {
    pub fn new(attempt: AttemptIdentityV8) -> Result<Self, QualificationError> {
        attempt.validate()?;
        Ok(Self {
            attempt,
            barrier_armed: false,
            permanent_quarantine: false,
            phase: AdmissionPhaseV8::Unarmed,
            qualification_abandoned: false,
            qualification_pass: false,
            active_claim_sha256: None,
            candidate_result: None,
            copy_ack_statement_sha256: None,
            post_snapshot_sha256: None,
            restore_evidence_sha256: None,
            runner_stop_evidence_sha256: None,
            verified_journal_tip_sha256: None,
        })
    }

    pub fn phase(&self) -> AdmissionPhaseV8 {
        self.phase
    }

    pub fn barrier_armed(&self) -> bool {
        self.barrier_armed
    }

    pub fn permanent_quarantine(&self) -> bool {
        self.permanent_quarantine
    }

    pub fn qualification_abandoned(&self) -> bool {
        self.qualification_abandoned
    }

    pub fn qualification_pass(&self) -> bool {
        self.qualification_pass
    }

    pub(crate) fn apply(&mut self, event: AdmissionEventV8) -> Result<(), QualificationError> {
        match event {
            AdmissionEventV8::Arm { authority } => {
                self.require_phase(AdmissionPhaseV8::Unarmed)?;
                if !authority.authorizes_one_shot(&self.attempt) {
                    return Err(invalid(
                        "admission arm lacks the exact verified one-shot authority",
                    ));
                }
                self.barrier_armed = true;
                self.phase = AdmissionPhaseV8::Armed;
            }
            AdmissionEventV8::Claim { capability_sha256 } => {
                self.require_phase(AdmissionPhaseV8::Armed)?;
                require_hex(&capability_sha256, "worker capability")?;
                self.active_claim_sha256 = Some(capability_sha256);
                self.phase = AdmissionPhaseV8::Claimed;
            }
            AdmissionEventV8::RunnerStopped { evidence_sha256 } => {
                self.require_phase(AdmissionPhaseV8::Claimed)?;
                require_hex(&evidence_sha256, "runner stop evidence")?;
                self.runner_stop_evidence_sha256 = Some(evidence_sha256);
                self.phase = AdmissionPhaseV8::RunnerStopped;
            }
            AdmissionEventV8::CandidatePublished { result } => {
                self.require_phase(AdmissionPhaseV8::RunnerStopped)?;
                if result.attempt != self.attempt {
                    return Err(invalid("candidate result selects a different attempt"));
                }
                result.validate()?;
                self.active_claim_sha256 = None;
                self.candidate_result = Some(result);
                self.phase = AdmissionPhaseV8::CandidatePublished;
            }
            AdmissionEventV8::CopyAcknowledged { verified } => {
                self.require_phase(AdmissionPhaseV8::CandidatePublished)?;
                let candidate = self
                    .candidate_result
                    .as_ref()
                    .ok_or_else(|| invalid("copy acknowledgement lacks candidate result"))?;
                if verified.attempt_identity_sha256() != self.attempt.sha256()?
                    || verified.candidate_result_bundle_sha256() != candidate.sha256()?
                {
                    return Err(invalid(
                        "verified copy acknowledgement selects different evidence",
                    ));
                }
                self.copy_ack_statement_sha256 = Some(verified.statement_sha256().to_string());
                self.phase = AdmissionPhaseV8::CopyAcknowledged;
            }
            AdmissionEventV8::RunnerRestored { evidence_sha256 } => {
                self.require_phase(AdmissionPhaseV8::CopyAcknowledged)?;
                require_hex(&evidence_sha256, "runner restore evidence")?;
                self.restore_evidence_sha256 = Some(evidence_sha256);
                self.phase = AdmissionPhaseV8::RunnerRestored;
            }
            AdmissionEventV8::PostSnapshot { snapshot_sha256 } => {
                self.require_phase(AdmissionPhaseV8::RunnerRestored)?;
                require_hex(&snapshot_sha256, "post-restore snapshot")?;
                self.post_snapshot_sha256 = Some(snapshot_sha256);
                self.phase = AdmissionPhaseV8::PostSnapshot;
            }
            AdmissionEventV8::Release { journal } => {
                self.require_phase(AdmissionPhaseV8::PostSnapshot)?;
                if journal.attempt_sha256() != self.attempt.sha256()?
                    || !journal.release_complete()
                    || journal.reboot_observed()
                    || journal.qualification_abandoned()
                {
                    return Err(invalid(
                        "release journal does not prove this exact uninterrupted attempt",
                    ));
                }
                let candidate = self
                    .candidate_result
                    .as_ref()
                    .ok_or_else(|| invalid("release state omits candidate result"))?;
                if journal.runner_stop_observation_sha256()
                    != self.runner_stop_evidence_sha256.as_deref()
                    || journal.candidate_result_sha256().map(str::to_owned)
                        != Some(candidate.sha256()?)
                    || journal.candidate_relay_observation_sha256()
                        != self.copy_ack_statement_sha256.as_deref()
                    || journal.runner_restore_observation_sha256()
                        != self.restore_evidence_sha256.as_deref()
                    || journal.barrier_release_observation_sha256()
                        != self.post_snapshot_sha256.as_deref()
                {
                    return Err(invalid(
                        "release journal observations do not bind the exact durable facts",
                    ));
                }
                if self.active_claim_sha256.is_some()
                    || self.runner_stop_evidence_sha256.is_none()
                    || self.candidate_result.is_none()
                    || self.copy_ack_statement_sha256.is_none()
                    || self.restore_evidence_sha256.is_none()
                    || self.post_snapshot_sha256.is_none()
                {
                    return Err(invalid("durable release prerequisites are incomplete"));
                }
                self.verified_journal_tip_sha256 = Some(journal.tip_sha256().to_string());
                self.barrier_armed = false;
                self.qualification_pass = journal.qualification_may_pass()
                    && self
                        .candidate_result
                        .as_ref()
                        .is_some_and(|result| result.outcome == CandidateOutcomeV8::Pass);
                self.phase = AdmissionPhaseV8::Released;
            }
            AdmissionEventV8::RecoverAfterCrash {
                recovery_evidence_sha256,
            } => {
                if !self.barrier_armed || self.phase == AdmissionPhaseV8::Released {
                    return Err(invalid("recovery requires an armed unreleased barrier"));
                }
                require_hex(&recovery_evidence_sha256, "recovery evidence")?;
                self.abandon();
            }
            AdmissionEventV8::BreakGlassRestore {
                authority,
                current_state,
                restore_evidence_sha256,
            } => {
                if !self.barrier_armed || self.phase == AdmissionPhaseV8::Released {
                    return Err(invalid("recovery requires an armed unreleased barrier"));
                }
                if !authority.authorizes_break_glass(&self.attempt, &current_state) {
                    return Err(invalid(
                        "break-glass restore lacks exact verified authority",
                    ));
                }
                require_hex(&restore_evidence_sha256, "break-glass restore evidence")?;
                self.abandon();
            }
        }
        Ok(())
    }

    fn abandon(&mut self) {
        self.active_claim_sha256 = None;
        self.permanent_quarantine = true;
        self.qualification_abandoned = true;
        self.qualification_pass = false;
        self.phase = AdmissionPhaseV8::Abandoned;
    }

    pub fn final_receipt(&self) -> Result<FinalReceiptBindingV8, QualificationError> {
        self.require_phase(AdmissionPhaseV8::Released)?;
        let journal_tip_sha256 = self
            .verified_journal_tip_sha256
            .clone()
            .ok_or_else(|| invalid("released state omits verified journal tip"))?;
        let candidate_result = self
            .candidate_result
            .as_ref()
            .ok_or_else(|| invalid("released state omits candidate result"))?;
        let receipt = FinalReceiptBindingV8 {
            attempt_identity_sha256: self.attempt.sha256()?,
            candidate_result_bundle_sha256: candidate_result.sha256()?,
            copy_ack_statement_sha256: self
                .copy_ack_statement_sha256
                .clone()
                .ok_or_else(|| invalid("released state omits copy acknowledgement"))?,
            journal_tip_sha256,
            outcome: candidate_result.outcome,
            post_snapshot_sha256: self
                .post_snapshot_sha256
                .clone()
                .ok_or_else(|| invalid("released state omits post snapshot"))?,
            qualification_pass: self.qualification_pass,
            restore_evidence_sha256: self
                .restore_evidence_sha256
                .clone()
                .ok_or_else(|| invalid("released state omits restore evidence"))?,
        };
        receipt.validate()?;
        Ok(receipt)
    }

    fn require_phase(&self, expected: AdmissionPhaseV8) -> Result<(), QualificationError> {
        if self.phase != expected || self.qualification_abandoned || self.permanent_quarantine {
            return Err(invalid(
                "admission transition is out of order or quarantined",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(crate) enum AdmissionEventV8 {
    Arm {
        authority: VerifiedAuthorityV8,
    },
    Claim {
        capability_sha256: String,
    },
    RunnerStopped {
        evidence_sha256: String,
    },
    CandidatePublished {
        result: CandidateResultBundleV8,
    },
    CopyAcknowledged {
        verified: VerifiedCopyAckV8,
    },
    RunnerRestored {
        evidence_sha256: String,
    },
    PostSnapshot {
        snapshot_sha256: String,
    },
    Release {
        journal: JournalAssessmentV8,
    },
    RecoverAfterCrash {
        recovery_evidence_sha256: String,
    },
    BreakGlassRestore {
        authority: VerifiedAuthorityV8,
        current_state: RecoveryStateBindingV8,
        restore_evidence_sha256: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FinalReceiptBindingV8 {
    pub attempt_identity_sha256: String,
    pub candidate_result_bundle_sha256: String,
    pub copy_ack_statement_sha256: String,
    pub journal_tip_sha256: String,
    pub outcome: CandidateOutcomeV8,
    pub post_snapshot_sha256: String,
    pub qualification_pass: bool,
    pub restore_evidence_sha256: String,
}

impl FinalReceiptBindingV8 {
    pub fn validate(&self) -> Result<(), QualificationError> {
        for (label, digest) in [
            ("final attempt identity", &self.attempt_identity_sha256),
            (
                "final candidate result bundle",
                &self.candidate_result_bundle_sha256,
            ),
            (
                "final copy acknowledgement statement",
                &self.copy_ack_statement_sha256,
            ),
            ("final journal tip", &self.journal_tip_sha256),
            ("final post snapshot", &self.post_snapshot_sha256),
            ("final restore evidence", &self.restore_evidence_sha256),
        ] {
            require_hex(digest, label)?;
        }
        if self.qualification_pass != (self.outcome == CandidateOutcomeV8::Pass) {
            return Err(invalid(
                "final qualification verdict disagrees with the candidate outcome",
            ));
        }
        Ok(())
    }

    pub fn sha256(&self) -> Result<String, QualificationError> {
        self.validate()?;
        let mut bytes = Vec::new();
        push(&mut bytes, b"hepta_linux_v8_final_receipt_binding_v1");
        push(&mut bytes, self.attempt_identity_sha256.as_bytes());
        push(&mut bytes, self.candidate_result_bundle_sha256.as_bytes());
        push(&mut bytes, self.copy_ack_statement_sha256.as_bytes());
        push(&mut bytes, self.journal_tip_sha256.as_bytes());
        push(&mut bytes, self.outcome.as_str().as_bytes());
        push(&mut bytes, self.post_snapshot_sha256.as_bytes());
        push(
            &mut bytes,
            if self.qualification_pass {
                b"PASS"
            } else {
                b"FAIL"
            },
        );
        push(&mut bytes, self.restore_evidence_sha256.as_bytes());
        Ok(sha256(&bytes))
    }
}

fn require_hex(value: &str, label: &str) -> Result<(), QualificationError> {
    if !hex64(value) {
        return Err(invalid(format!("{label} digest is malformed")));
    }
    Ok(())
}

fn hex64(value: &str) -> bool {
    value.len() == 64
        && !value.bytes().all(|byte| byte == b'0')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn push(bytes: &mut Vec<u8>, value: &[u8]) {
    push_u64(bytes, value.len() as u64);
    bytes.extend_from_slice(value);
}

fn push_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
#[path = "protocol_tests.rs"]
mod tests;

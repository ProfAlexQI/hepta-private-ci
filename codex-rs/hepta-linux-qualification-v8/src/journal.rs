use crate::AttemptIdentityV8;
use crate::QualificationError;
use crate::append_bytes;
use crate::append_text;
use crate::append_u64;
use crate::invalid;
use crate::sha256_hex;
use crate::validate_lower_hex;
use crate::validate_sha256;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;

const JOURNAL_RECORD_SCHEMA: &str = "hepta_linux_exact_v8_journal_record_v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BootStampV8 {
    pub boot_epoch: u64,
    pub boot_id: String,
    pub boot_seq: u64,
    pub monotonic_ns: u64,
}

impl BootStampV8 {
    pub fn validate(&self) -> Result<(), QualificationError> {
        if self.boot_epoch == 0 {
            return Err(invalid("boot_epoch must be non-zero"));
        }
        validate_boot_id(&self.boot_id)?;
        if self.boot_seq == 0 {
            return Err(invalid("boot_seq must be non-zero"));
        }
        if self.monotonic_ns == 0 {
            return Err(invalid("monotonic_ns must be non-zero"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JournalEffectV8 {
    RunnerStop,
    CandidateExecution,
    CandidateRelay,
    RunnerRestore,
    PostRestoreSnapshot,
    BarrierRelease,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "event",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
pub enum JournalEventV8 {
    AttemptOpened {
        authority_manifest_sha256: String,
    },
    EffectIntent {
        effect: JournalEffectV8,
        effect_manifest_sha256: String,
    },
    EffectObserved {
        effect: JournalEffectV8,
        intent_record_sha256: String,
        observation_sha256: String,
    },
    CandidateCompleted {
        candidate_result_sha256: String,
    },
    BootRecovery {
        previous_boot_id: String,
        previous_journal_tip_sha256: String,
        recovery_observation_sha256: String,
    },
    QualificationAbandoned {
        abandonment_evidence_sha256: String,
    },
}

impl JournalEventV8 {
    /// Validates one event independently of journal ordering. Native durable
    /// storage uses this boundary before admitting event bytes to its exact
    /// on-disk envelope; whole-journal ordering remains the responsibility of
    /// [`validate_journal_v8`].
    pub fn validate(&self) -> Result<(), QualificationError> {
        match self {
            Self::AttemptOpened {
                authority_manifest_sha256,
            } => validate_sha256("authority_manifest_sha256", authority_manifest_sha256),
            Self::EffectIntent {
                effect: _,
                effect_manifest_sha256,
            } => validate_sha256("effect_manifest_sha256", effect_manifest_sha256),
            Self::EffectObserved {
                effect: _,
                intent_record_sha256,
                observation_sha256,
            } => {
                validate_sha256("intent_record_sha256", intent_record_sha256)?;
                validate_sha256("observation_sha256", observation_sha256)
            }
            Self::CandidateCompleted {
                candidate_result_sha256,
            } => validate_sha256("candidate_result_sha256", candidate_result_sha256),
            Self::BootRecovery {
                previous_boot_id,
                previous_journal_tip_sha256,
                recovery_observation_sha256,
            } => {
                validate_boot_id(previous_boot_id)?;
                validate_sha256("previous_journal_tip_sha256", previous_journal_tip_sha256)?;
                validate_sha256("recovery_observation_sha256", recovery_observation_sha256)
            }
            Self::QualificationAbandoned {
                abandonment_evidence_sha256,
            } => validate_sha256("abandonment_evidence_sha256", abandonment_evidence_sha256),
        }
    }

    /// Canonical semantic bytes shared by model and native durable replay.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Self::AttemptOpened {
                authority_manifest_sha256,
            } => {
                append_text(&mut bytes, "event", "ATTEMPT_OPENED");
                append_text(
                    &mut bytes,
                    "authority_manifest_sha256",
                    authority_manifest_sha256,
                );
            }
            Self::EffectIntent {
                effect,
                effect_manifest_sha256,
            } => {
                append_text(&mut bytes, "event", "EFFECT_INTENT");
                append_text(&mut bytes, "effect", effect.canonical_name());
                append_text(&mut bytes, "effect_manifest_sha256", effect_manifest_sha256);
            }
            Self::EffectObserved {
                effect,
                intent_record_sha256,
                observation_sha256,
            } => {
                append_text(&mut bytes, "event", "EFFECT_OBSERVED");
                append_text(&mut bytes, "effect", effect.canonical_name());
                append_text(&mut bytes, "intent_record_sha256", intent_record_sha256);
                append_text(&mut bytes, "observation_sha256", observation_sha256);
            }
            Self::CandidateCompleted {
                candidate_result_sha256,
            } => {
                append_text(&mut bytes, "event", "CANDIDATE_COMPLETED");
                append_text(
                    &mut bytes,
                    "candidate_result_sha256",
                    candidate_result_sha256,
                );
            }
            Self::BootRecovery {
                previous_boot_id,
                previous_journal_tip_sha256,
                recovery_observation_sha256,
            } => {
                append_text(&mut bytes, "event", "BOOT_RECOVERY");
                append_text(&mut bytes, "previous_boot_id", previous_boot_id);
                append_text(
                    &mut bytes,
                    "previous_journal_tip_sha256",
                    previous_journal_tip_sha256,
                );
                append_text(
                    &mut bytes,
                    "recovery_observation_sha256",
                    recovery_observation_sha256,
                );
            }
            Self::QualificationAbandoned {
                abandonment_evidence_sha256,
            } => {
                append_text(&mut bytes, "event", "QUALIFICATION_ABANDONED");
                append_text(
                    &mut bytes,
                    "abandonment_evidence_sha256",
                    abandonment_evidence_sha256,
                );
            }
        }
        bytes
    }
}

impl JournalEffectV8 {
    pub fn canonical_name(self) -> &'static str {
        match self {
            Self::RunnerStop => "runner_stop",
            Self::CandidateExecution => "candidate_execution",
            Self::CandidateRelay => "candidate_relay",
            Self::RunnerRestore => "runner_restore",
            Self::PostRestoreSnapshot => "post_restore_snapshot",
            Self::BarrierRelease => "barrier_release",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JournalRecordV8 {
    pub attempt: AttemptIdentityV8,
    pub global_seq: u64,
    pub boot: BootStampV8,
    pub previous_record_sha256: Option<String>,
    pub event: JournalEventV8,
    pub record_sha256: String,
}

impl JournalRecordV8 {
    pub fn new(
        attempt: AttemptIdentityV8,
        global_seq: u64,
        boot: BootStampV8,
        previous_record_sha256: Option<String>,
        event: JournalEventV8,
    ) -> Result<Self, QualificationError> {
        let mut record = Self {
            attempt,
            global_seq,
            boot,
            previous_record_sha256,
            event,
            record_sha256: String::new(),
        };
        record.validate_content()?;
        record.record_sha256 = sha256_hex(&record.canonical_bytes()?);
        Ok(record)
    }

    /// Returns the record representation hashed by `record_sha256`.
    ///
    /// The sequence is schema, complete attempt identity, global sequence, boot
    /// epoch/id/sequence/monotonic time, an explicit previous-record presence
    /// marker and digest, then the tagged event and its fields.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, QualificationError> {
        self.validate_content()?;
        let mut bytes = Vec::new();
        append_text(&mut bytes, "schema", JOURNAL_RECORD_SCHEMA);
        append_bytes(
            &mut bytes,
            "attempt_identity",
            &self.attempt.canonical_bytes()?,
        );
        append_u64(&mut bytes, "global_seq", self.global_seq);
        append_u64(&mut bytes, "boot_epoch", self.boot.boot_epoch);
        append_text(&mut bytes, "boot_id", &self.boot.boot_id);
        append_u64(&mut bytes, "boot_seq", self.boot.boot_seq);
        append_u64(&mut bytes, "monotonic_ns", self.boot.monotonic_ns);
        match &self.previous_record_sha256 {
            Some(previous) => {
                append_bytes(&mut bytes, "has_previous_record", &[1]);
                append_text(&mut bytes, "previous_record_sha256", previous);
            }
            None => append_bytes(&mut bytes, "has_previous_record", &[0]),
        }
        append_bytes(&mut bytes, "event", &self.event.canonical_bytes());
        Ok(bytes)
    }

    pub fn computed_sha256(&self) -> Result<String, QualificationError> {
        Ok(sha256_hex(&self.canonical_bytes()?))
    }

    pub fn validate(&self) -> Result<(), QualificationError> {
        validate_sha256("record_sha256", &self.record_sha256)?;
        let computed = self.computed_sha256()?;
        if self.record_sha256 != computed {
            return Err(invalid(
                "record_sha256 does not match canonical record bytes",
            ));
        }
        Ok(())
    }

    fn validate_content(&self) -> Result<(), QualificationError> {
        self.attempt.validate()?;
        if self.global_seq == 0 {
            return Err(invalid("global_seq must be non-zero"));
        }
        self.boot.validate()?;
        if let Some(previous) = &self.previous_record_sha256 {
            validate_sha256("previous_record_sha256", previous)?;
        }
        self.event.validate()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalAssessmentV8 {
    attempt_sha256: String,
    tip_sha256: String,
    record_count: u64,
    boot_count: u64,
    reboot_observed: bool,
    qualification_abandoned: bool,
    ready_for_release_authorization: bool,
    pre_release_tip_sha256: Option<String>,
    release_complete: bool,
    qualification_may_pass: bool,
    runner_stop_observation_sha256: Option<String>,
    candidate_result_sha256: Option<String>,
    candidate_relay_observation_sha256: Option<String>,
    runner_restore_observation_sha256: Option<String>,
    post_snapshot_observation_sha256: Option<String>,
    barrier_release_manifest_sha256: Option<String>,
    barrier_release_observation_sha256: Option<String>,
}

impl JournalAssessmentV8 {
    pub fn attempt_sha256(&self) -> &str {
        &self.attempt_sha256
    }

    pub fn tip_sha256(&self) -> &str {
        &self.tip_sha256
    }

    pub fn record_count(&self) -> u64 {
        self.record_count
    }

    pub fn boot_count(&self) -> u64 {
        self.boot_count
    }

    pub fn reboot_observed(&self) -> bool {
        self.reboot_observed
    }

    pub fn qualification_abandoned(&self) -> bool {
        self.qualification_abandoned
    }

    pub fn release_complete(&self) -> bool {
        self.release_complete
    }

    pub fn ready_for_release_authorization(&self) -> bool {
        self.ready_for_release_authorization
    }

    pub fn pre_release_tip_sha256(&self) -> Option<&str> {
        self.pre_release_tip_sha256.as_deref()
    }

    pub fn qualification_may_pass(&self) -> bool {
        self.qualification_may_pass
    }

    pub fn runner_stop_observation_sha256(&self) -> Option<&str> {
        self.runner_stop_observation_sha256.as_deref()
    }

    pub fn candidate_result_sha256(&self) -> Option<&str> {
        self.candidate_result_sha256.as_deref()
    }

    pub fn candidate_relay_observation_sha256(&self) -> Option<&str> {
        self.candidate_relay_observation_sha256.as_deref()
    }

    pub fn runner_restore_observation_sha256(&self) -> Option<&str> {
        self.runner_restore_observation_sha256.as_deref()
    }

    pub fn post_snapshot_observation_sha256(&self) -> Option<&str> {
        self.post_snapshot_observation_sha256.as_deref()
    }

    pub fn barrier_release_manifest_sha256(&self) -> Option<&str> {
        self.barrier_release_manifest_sha256.as_deref()
    }

    pub fn barrier_release_observation_sha256(&self) -> Option<&str> {
        self.barrier_release_observation_sha256.as_deref()
    }
}

pub fn validate_journal_v8(
    records: &[JournalRecordV8],
) -> Result<JournalAssessmentV8, QualificationError> {
    let Some(first) = records.first() else {
        return Err(invalid("journal must contain at least one record"));
    };
    first.validate()?;
    if first.global_seq != 1
        || first.boot.boot_epoch != 1
        || first.boot.boot_seq != 1
        || first.previous_record_sha256.is_some()
        || !matches!(&first.event, JournalEventV8::AttemptOpened { .. })
    {
        return Err(invalid(
            "journal must start at global_seq=1 and boot epoch/seq=1 with ATTEMPT_OPENED and no predecessor",
        ));
    }

    let attempt = &first.attempt;
    let mut reboot_observed = false;
    let mut seen_boot_ids = HashSet::from([first.boot.boot_id.as_str()]);
    for pair in records.windows(2) {
        let previous = &pair[0];
        let current = &pair[1];
        current.validate()?;
        if &current.attempt != attempt {
            return Err(invalid("journal contains a spliced attempt identity"));
        }
        if current.global_seq
            != previous
                .global_seq
                .checked_add(1)
                .ok_or_else(|| invalid("global_seq overflow prevents a contiguous journal"))?
        {
            return Err(invalid("journal global_seq is not contiguous"));
        }
        if current.previous_record_sha256.as_deref() != Some(previous.record_sha256.as_str()) {
            return Err(invalid("journal previous-record hash chain is broken"));
        }

        if current.boot.boot_epoch == previous.boot.boot_epoch {
            validate_same_boot(previous, current)?;
        } else {
            validate_boot_recovery(previous, current)?;
            if !seen_boot_ids.insert(current.boot.boot_id.as_str()) {
                return Err(invalid("boot_id was reused by a later boot epoch"));
            }
            reboot_observed = true;
        }
    }

    let tip = &records[records.len() - 1];
    let semantics = validate_journal_semantics(records, reboot_observed)?;
    Ok(JournalAssessmentV8 {
        attempt_sha256: attempt.sha256()?,
        tip_sha256: tip.record_sha256.clone(),
        record_count: u64::try_from(records.len())
            .map_err(|_| invalid("journal record count exceeds u64"))?,
        boot_count: tip.boot.boot_epoch,
        reboot_observed,
        qualification_abandoned: reboot_observed || semantics.abandoned,
        ready_for_release_authorization: semantics.ready_for_release_authorization,
        pre_release_tip_sha256: semantics.pre_release_tip_sha256,
        release_complete: semantics.release_complete,
        qualification_may_pass: !reboot_observed
            && !semantics.abandoned
            && semantics.release_complete,
        runner_stop_observation_sha256: semantics.runner_stop_observation_sha256,
        candidate_result_sha256: semantics.candidate_result_sha256,
        candidate_relay_observation_sha256: semantics.candidate_relay_observation_sha256,
        runner_restore_observation_sha256: semantics.runner_restore_observation_sha256,
        post_snapshot_observation_sha256: semantics.post_snapshot_observation_sha256,
        barrier_release_manifest_sha256: semantics.barrier_release_manifest_sha256,
        barrier_release_observation_sha256: semantics.barrier_release_observation_sha256,
    })
}

/// Shared fail-closed phase fold for the exact 13-step qualification effect
/// lifecycle. This type carries ordering facts only. It is not runner,
/// candidate, relay, snapshot, barrier, recovery, or release authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QualificationJournalPhaseV8 {
    AwaitRunnerStopIntent,
    AwaitRunnerStopObservation,
    AwaitCandidateExecutionIntent,
    AwaitCandidateExecutionObservation,
    AwaitCandidateCompleted,
    AwaitCandidateRelayIntent,
    AwaitCandidateRelayObservation,
    AwaitRunnerRestoreIntent,
    AwaitRunnerRestoreObservation,
    AwaitPostRestoreSnapshotIntent,
    AwaitPostRestoreSnapshotObservation,
    AwaitBarrierReleaseIntent,
    AwaitBarrierReleaseObservation,
    Complete,
}

impl QualificationJournalPhaseV8 {
    pub const fn initial() -> Self {
        Self::AwaitRunnerStopIntent
    }

    pub const fn ready_for_release_authorization(self) -> bool {
        matches!(self, Self::AwaitBarrierReleaseIntent)
    }

    pub const fn release_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    pub fn advance(self, event: &JournalEventV8) -> Result<Self, QualificationError> {
        let next = match (self, event) {
            (
                Self::AwaitRunnerStopIntent,
                JournalEventV8::EffectIntent {
                    effect: JournalEffectV8::RunnerStop,
                    ..
                },
            ) => Self::AwaitRunnerStopObservation,
            (
                Self::AwaitRunnerStopObservation,
                JournalEventV8::EffectObserved {
                    effect: JournalEffectV8::RunnerStop,
                    ..
                },
            ) => Self::AwaitCandidateExecutionIntent,
            (
                Self::AwaitCandidateExecutionIntent,
                JournalEventV8::EffectIntent {
                    effect: JournalEffectV8::CandidateExecution,
                    ..
                },
            ) => Self::AwaitCandidateExecutionObservation,
            (
                Self::AwaitCandidateExecutionObservation,
                JournalEventV8::EffectObserved {
                    effect: JournalEffectV8::CandidateExecution,
                    ..
                },
            ) => Self::AwaitCandidateCompleted,
            (Self::AwaitCandidateCompleted, JournalEventV8::CandidateCompleted { .. }) => {
                Self::AwaitCandidateRelayIntent
            }
            (
                Self::AwaitCandidateRelayIntent,
                JournalEventV8::EffectIntent {
                    effect: JournalEffectV8::CandidateRelay,
                    ..
                },
            ) => Self::AwaitCandidateRelayObservation,
            (
                Self::AwaitCandidateRelayObservation,
                JournalEventV8::EffectObserved {
                    effect: JournalEffectV8::CandidateRelay,
                    ..
                },
            ) => Self::AwaitRunnerRestoreIntent,
            (
                Self::AwaitRunnerRestoreIntent,
                JournalEventV8::EffectIntent {
                    effect: JournalEffectV8::RunnerRestore,
                    ..
                },
            ) => Self::AwaitRunnerRestoreObservation,
            (
                Self::AwaitRunnerRestoreObservation,
                JournalEventV8::EffectObserved {
                    effect: JournalEffectV8::RunnerRestore,
                    ..
                },
            ) => Self::AwaitPostRestoreSnapshotIntent,
            (
                Self::AwaitPostRestoreSnapshotIntent,
                JournalEventV8::EffectIntent {
                    effect: JournalEffectV8::PostRestoreSnapshot,
                    ..
                },
            ) => Self::AwaitPostRestoreSnapshotObservation,
            (
                Self::AwaitPostRestoreSnapshotObservation,
                JournalEventV8::EffectObserved {
                    effect: JournalEffectV8::PostRestoreSnapshot,
                    ..
                },
            ) => Self::AwaitBarrierReleaseIntent,
            (
                Self::AwaitBarrierReleaseIntent,
                JournalEventV8::EffectIntent {
                    effect: JournalEffectV8::BarrierRelease,
                    ..
                },
            ) => Self::AwaitBarrierReleaseObservation,
            (
                Self::AwaitBarrierReleaseObservation,
                JournalEventV8::EffectObserved {
                    effect: JournalEffectV8::BarrierRelease,
                    ..
                },
            ) => Self::Complete,
            (Self::Complete, _) => {
                return Err(invalid(
                    "qualification phase fold rejects records after terminal barrier release",
                ));
            }
            _ => {
                return Err(invalid(
                    "qualification event is duplicated, skipped, or out of exact phase order",
                ));
            }
        };
        Ok(next)
    }
}

struct JournalSemanticsV8 {
    abandoned: bool,
    ready_for_release_authorization: bool,
    pre_release_tip_sha256: Option<String>,
    release_complete: bool,
    runner_stop_observation_sha256: Option<String>,
    candidate_result_sha256: Option<String>,
    candidate_relay_observation_sha256: Option<String>,
    runner_restore_observation_sha256: Option<String>,
    post_snapshot_observation_sha256: Option<String>,
    barrier_release_manifest_sha256: Option<String>,
    barrier_release_observation_sha256: Option<String>,
}

fn validate_journal_semantics(
    records: &[JournalRecordV8],
    reboot_observed: bool,
) -> Result<JournalSemanticsV8, QualificationError> {
    let mut phase = QualificationJournalPhaseV8::initial();
    let mut pending_intent: Option<(JournalEffectV8, &str)> = None;
    let mut recovery_seen = false;
    let mut abandoned = false;
    let mut runner_stop_observation_sha256 = None;
    let mut candidate_result_sha256 = None;
    let mut candidate_relay_observation_sha256 = None;
    let mut runner_restore_observation_sha256 = None;
    let mut post_snapshot_observation_sha256 = None;
    let mut pre_release_tip_sha256 = None;
    let mut barrier_release_manifest_sha256 = None;
    let mut barrier_release_observation_sha256 = None;

    for record in records.iter().skip(1) {
        if abandoned {
            return Err(invalid(
                "journal contains records after terminal abandonment",
            ));
        }
        if phase.release_complete() {
            return Err(invalid(
                "journal contains records after terminal barrier release",
            ));
        }
        match &record.event {
            JournalEventV8::BootRecovery { .. } => {
                if recovery_seen {
                    return Err(invalid("journal contains more than one boot recovery"));
                }
                recovery_seen = true;
                pending_intent = None;
            }
            JournalEventV8::QualificationAbandoned { .. } => {
                abandoned = true;
                pending_intent = None;
            }
            JournalEventV8::AttemptOpened { .. } => {
                return Err(invalid("journal reopens an existing attempt"));
            }
            event if recovery_seen => {
                let _ = event;
                return Err(invalid(
                    "a rebooted qualification may only record recovery and abandonment",
                ));
            }
            JournalEventV8::EffectIntent {
                effect,
                effect_manifest_sha256,
            } => {
                if pending_intent.is_some() {
                    return Err(invalid("effect intent is duplicated or out of order"));
                }
                phase = phase.advance(&record.event)?;
                if *effect == JournalEffectV8::BarrierRelease {
                    barrier_release_manifest_sha256 = Some(effect_manifest_sha256.clone());
                }
                pending_intent = Some((*effect, record.record_sha256.as_str()));
            }
            JournalEventV8::EffectObserved {
                effect,
                intent_record_sha256,
                observation_sha256,
                ..
            } => {
                if pending_intent != Some((*effect, intent_record_sha256.as_str())) {
                    return Err(invalid(
                        "effect observation does not close the exact pending intent",
                    ));
                }
                phase = phase.advance(&record.event)?;
                match effect {
                    JournalEffectV8::RunnerStop => {
                        runner_stop_observation_sha256 = Some(observation_sha256.clone());
                    }
                    JournalEffectV8::CandidateExecution => {}
                    JournalEffectV8::CandidateRelay => {
                        candidate_relay_observation_sha256 = Some(observation_sha256.clone());
                    }
                    JournalEffectV8::RunnerRestore => {
                        runner_restore_observation_sha256 = Some(observation_sha256.clone());
                    }
                    JournalEffectV8::PostRestoreSnapshot => {
                        post_snapshot_observation_sha256 = Some(observation_sha256.clone());
                        pre_release_tip_sha256 = Some(record.record_sha256.clone());
                    }
                    JournalEffectV8::BarrierRelease => {
                        barrier_release_observation_sha256 = Some(observation_sha256.clone());
                    }
                }
                pending_intent = None;
            }
            JournalEventV8::CandidateCompleted {
                candidate_result_sha256: observed,
            } => {
                if pending_intent.is_some() {
                    return Err(invalid("candidate completion is out of order"));
                }
                phase = phase.advance(&record.event)?;
                candidate_result_sha256 = Some(observed.clone());
            }
        }
    }

    if reboot_observed && !recovery_seen {
        return Err(invalid("rebooted journal omits recovery semantics"));
    }
    Ok(JournalSemanticsV8 {
        abandoned,
        ready_for_release_authorization: phase.ready_for_release_authorization()
            && pending_intent.is_none()
            && !recovery_seen
            && !abandoned,
        pre_release_tip_sha256,
        release_complete: phase.release_complete()
            && pending_intent.is_none()
            && !recovery_seen
            && !abandoned,
        runner_stop_observation_sha256,
        candidate_result_sha256,
        candidate_relay_observation_sha256,
        runner_restore_observation_sha256,
        post_snapshot_observation_sha256,
        barrier_release_manifest_sha256,
        barrier_release_observation_sha256,
    })
}

fn validate_same_boot(
    previous: &JournalRecordV8,
    current: &JournalRecordV8,
) -> Result<(), QualificationError> {
    if current.boot.boot_id != previous.boot.boot_id {
        return Err(invalid("boot_id changed without a new boot epoch"));
    }
    if current.boot.boot_seq
        != previous
            .boot
            .boot_seq
            .checked_add(1)
            .ok_or_else(|| invalid("boot_seq overflow prevents a contiguous boot journal"))?
    {
        return Err(invalid("boot_seq is not contiguous within one boot"));
    }
    if current.boot.monotonic_ns <= previous.boot.monotonic_ns {
        return Err(invalid(
            "monotonic_ns must increase strictly within one boot",
        ));
    }
    if matches!(&current.event, JournalEventV8::BootRecovery { .. }) {
        return Err(invalid(
            "BOOT_RECOVERY is only valid at a new boot boundary",
        ));
    }
    Ok(())
}

fn validate_boot_recovery(
    previous: &JournalRecordV8,
    current: &JournalRecordV8,
) -> Result<(), QualificationError> {
    if current.boot.boot_epoch
        != previous
            .boot
            .boot_epoch
            .checked_add(1)
            .ok_or_else(|| invalid("boot_epoch overflow prevents recovery"))?
        || current.boot.boot_id == previous.boot.boot_id
        || current.boot.boot_seq != 1
    {
        return Err(invalid(
            "new boot must increment epoch, change boot_id, and restart boot_seq at one",
        ));
    }
    match &current.event {
        JournalEventV8::BootRecovery {
            previous_boot_id,
            previous_journal_tip_sha256,
            recovery_observation_sha256: _,
        } if previous_boot_id == &previous.boot.boot_id
            && previous_journal_tip_sha256 == &previous.record_sha256 =>
        {
            Ok(())
        }
        JournalEventV8::BootRecovery { .. } => Err(invalid(
            "BOOT_RECOVERY does not bind the exact prior boot and journal tip",
        )),
        JournalEventV8::AttemptOpened { .. }
        | JournalEventV8::EffectIntent { .. }
        | JournalEventV8::EffectObserved { .. }
        | JournalEventV8::CandidateCompleted { .. }
        | JournalEventV8::QualificationAbandoned { .. } => {
            Err(invalid("new boot must begin with BOOT_RECOVERY"))
        }
    }
}

fn validate_boot_id(value: &str) -> Result<(), QualificationError> {
    if value.len() != 36
        || value.as_bytes().get(8) != Some(&b'-')
        || value.as_bytes().get(13) != Some(&b'-')
        || value.as_bytes().get(18) != Some(&b'-')
        || value.as_bytes().get(23) != Some(&b'-')
    {
        return Err(invalid("boot_id must be a lowercase hyphenated UUID"));
    }
    let compact = value.replace('-', "");
    validate_lower_hex("boot_id", &compact, 32)
}

#[cfg(test)]
#[path = "journal_tests.rs"]
mod tests;

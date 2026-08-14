use codex_hepta_linux_qualification_v8::JournalEffectV8;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest as _;

use crate::FileIdentityV8;
use crate::NativeErrorV8;
use crate::invalid;

use super::DurableJournalRecordV8;
use super::validate_boot_id_v8;
use super::validate_digest;

const MAX_FROZEN_TRANSITION_EVIDENCE_BYTES_V8: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FrozenTransitionEvidencePhaseV8 {
    Intent,
    Observation,
}

/// Exact caller-independent context from which one typed transition intent is
/// frozen. Constructing evidence is not effect or recovery authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FrozenTransitionIntentContextV8 {
    pub(crate) machine_id_sha256: String,
    pub(crate) machine_id_source_identity: FileIdentityV8,
    pub(crate) state_root_binding_sha256: String,
    pub(crate) state_root_identity: FileIdentityV8,
    pub(crate) state_root_mount_id: u64,
    pub(crate) state_root_lock_identity: FileIdentityV8,
    pub(crate) attempt_identity_sha256: String,
    pub(crate) active_attempt_record_sha256: String,
    pub(crate) active_attempt_file_identity: FileIdentityV8,
    pub(crate) barrier_generation: u64,
    pub(crate) restore_plan_sha256: String,
    pub(crate) boot_id: String,
    pub(crate) boot_epoch: u64,
    pub(crate) global_sequence: u64,
    pub(crate) journal_tip_sha256: String,
    pub(crate) predecessor_record_sha256: String,
    pub(crate) candidate_execution_request_sha256: String,
}

/// Live, descriptor-retained origin against which candidate-execution
/// evidence is replayed by the read-only capsule. The candidate request hash
/// is intentionally absent: this slice has no retained external source for
/// that value, so it remains canonical journal evidence only and can never be
/// treated as independently authenticated execution authority.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DescriptorReplayOriginV8<'a> {
    pub(crate) machine_id_sha256: &'a str,
    pub(crate) machine_id_source_identity: FileIdentityV8,
    pub(crate) state_root_binding_sha256: &'a str,
    pub(crate) state_root_identity: FileIdentityV8,
    pub(crate) state_root_mount_id: u64,
    pub(crate) state_root_lock_identity: FileIdentityV8,
    pub(crate) attempt_identity_sha256: &'a str,
    pub(crate) active_attempt_record_sha256: &'a str,
    pub(crate) active_attempt_file_identity: FileIdentityV8,
    pub(crate) barrier_generation: u64,
    pub(crate) restore_plan_sha256: &'a str,
    pub(crate) boot_id: &'a str,
}

/// Canonical fact retained once a future executor has crossed an effect-call
/// boundary. `effect_call_issued_or_uncertain` is deliberately one-way: it is
/// never accepted as a fresh retry permit.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DurableEffectIssuedOrUncertainFactV8 {
    schema: String,
    effect: JournalEffectV8,
    machine_id_sha256: String,
    machine_id_source_device: u64,
    machine_id_source_inode: u64,
    state_root_binding_sha256: String,
    state_root_device: u64,
    state_root_inode: u64,
    state_root_mode: u32,
    state_root_owner_uid: u32,
    state_root_owner_gid: u32,
    state_root_mount_id: u64,
    state_root_lock_device: u64,
    state_root_lock_inode: u64,
    attempt_identity_sha256: String,
    active_attempt_record_sha256: String,
    active_attempt_file_device: u64,
    active_attempt_file_inode: u64,
    barrier_generation: u64,
    restore_plan_sha256: String,
    boot_id: String,
    boot_epoch: u64,
    intent_global_sequence: u64,
    pre_intent_journal_tip_sha256: String,
    predecessor_record_sha256: String,
    candidate_execution_request_sha256: String,
    intent_manifest_sha256: String,
    intent_record_sha256: String,
    issuance_nonce_sha256: String,
    issue_started_monotonic_ns: u64,
    issue_completed_monotonic_ns: u64,
    effect_call_issued_or_uncertain: bool,
}

impl DurableEffectIssuedOrUncertainFactV8 {
    fn from_intent(
        effect: JournalEffectV8,
        intent: &FrozenTransitionEffectEvidenceFieldsV8,
        intent_manifest_sha256: String,
        intent_record_sha256: String,
        issuance_nonce_sha256: String,
        issue_started_monotonic_ns: u64,
        issue_completed_monotonic_ns: u64,
    ) -> Result<Self, NativeErrorV8> {
        let fact = Self {
            schema: issued_schema_v8(effect)?.to_string(),
            effect,
            machine_id_sha256: intent.machine_id_sha256.clone(),
            machine_id_source_device: intent.machine_id_source_device,
            machine_id_source_inode: intent.machine_id_source_inode,
            state_root_binding_sha256: intent.state_root_binding_sha256.clone(),
            state_root_device: intent.state_root_device,
            state_root_inode: intent.state_root_inode,
            state_root_mode: intent.state_root_mode,
            state_root_owner_uid: intent.state_root_owner_uid,
            state_root_owner_gid: intent.state_root_owner_gid,
            state_root_mount_id: intent.state_root_mount_id,
            state_root_lock_device: intent.state_root_lock_device,
            state_root_lock_inode: intent.state_root_lock_inode,
            attempt_identity_sha256: intent.attempt_identity_sha256.clone(),
            active_attempt_record_sha256: intent.active_attempt_record_sha256.clone(),
            active_attempt_file_device: intent.active_attempt_file_device,
            active_attempt_file_inode: intent.active_attempt_file_inode,
            barrier_generation: intent.barrier_generation,
            restore_plan_sha256: intent.restore_plan_sha256.clone(),
            boot_id: intent.boot_id.clone(),
            boot_epoch: intent.boot_epoch,
            intent_global_sequence: intent.global_sequence,
            pre_intent_journal_tip_sha256: intent.journal_tip_sha256.clone(),
            predecessor_record_sha256: intent.predecessor_record_sha256.clone(),
            candidate_execution_request_sha256: intent.candidate_execution_request_sha256.clone(),
            intent_manifest_sha256,
            intent_record_sha256,
            issuance_nonce_sha256,
            issue_started_monotonic_ns,
            issue_completed_monotonic_ns,
            effect_call_issued_or_uncertain: true,
        };
        fact.validate(effect)?;
        Ok(fact)
    }

    pub(crate) fn canonical_bytes(&self) -> Result<Vec<u8>, NativeErrorV8> {
        self.validate(self.effect)?;
        serde_json::to_vec(self)
            .map_err(|error| invalid(format!("encode issued-or-uncertain effect fact: {error}")))
    }

    pub(crate) fn decode_exact(
        effect: JournalEffectV8,
        bytes: &[u8],
    ) -> Result<Self, NativeErrorV8> {
        if bytes.is_empty() || bytes.len() > MAX_FROZEN_TRANSITION_EVIDENCE_BYTES_V8 {
            return Err(invalid("issued-or-uncertain effect fact size is invalid"));
        }
        let fact: Self = serde_json::from_slice(bytes)
            .map_err(|error| invalid(format!("decode issued-or-uncertain effect fact: {error}")))?;
        fact.validate(effect)?;
        if fact.canonical_bytes()? != bytes {
            return Err(invalid(
                "issued-or-uncertain effect fact bytes are not canonical",
            ));
        }
        Ok(fact)
    }

    fn validate(&self, effect: JournalEffectV8) -> Result<(), NativeErrorV8> {
        if self.schema != issued_schema_v8(effect)?
            || self.effect != effect
            || self.state_root_device == 0
            || self.state_root_inode == 0
            || self.state_root_mode != 0o700
            || self.state_root_mount_id == 0
            || self.state_root_lock_device == 0
            || self.state_root_lock_inode == 0
            || self.machine_id_source_device == 0
            || self.machine_id_source_inode == 0
            || self.active_attempt_file_device == 0
            || self.active_attempt_file_inode == 0
            || self.barrier_generation == 0
            || self.boot_epoch == 0
            || self.intent_global_sequence < 2
            || self.issue_started_monotonic_ns == 0
            || self.issue_completed_monotonic_ns < self.issue_started_monotonic_ns
            || !self.effect_call_issued_or_uncertain
        {
            return Err(invalid("issued-or-uncertain effect fact is malformed"));
        }
        validate_boot_id_v8(&self.boot_id)?;
        for (label, digest) in [
            ("issued machine", self.machine_id_sha256.as_str()),
            (
                "issued state-root binding",
                self.state_root_binding_sha256.as_str(),
            ),
            ("issued attempt", self.attempt_identity_sha256.as_str()),
            (
                "issued active attempt",
                self.active_attempt_record_sha256.as_str(),
            ),
            ("issued restore plan", self.restore_plan_sha256.as_str()),
            (
                "issued pre-intent journal tip",
                self.pre_intent_journal_tip_sha256.as_str(),
            ),
            (
                "issued predecessor",
                self.predecessor_record_sha256.as_str(),
            ),
            (
                "issued candidate execution request",
                self.candidate_execution_request_sha256.as_str(),
            ),
            (
                "issued intent manifest",
                self.intent_manifest_sha256.as_str(),
            ),
            ("issued intent record", self.intent_record_sha256.as_str()),
            ("issued nonce", self.issuance_nonce_sha256.as_str()),
        ] {
            validate_digest(label, digest)?;
        }
        Ok(())
    }

    fn matches_intent(
        &self,
        effect: JournalEffectV8,
        intent: &FrozenTransitionEffectEvidenceFieldsV8,
        intent_manifest_sha256: &str,
        intent_record_sha256: &str,
    ) -> bool {
        self.validate(effect).is_ok()
            && self.machine_id_sha256 == intent.machine_id_sha256
            && self.machine_id_source_device == intent.machine_id_source_device
            && self.machine_id_source_inode == intent.machine_id_source_inode
            && self.state_root_binding_sha256 == intent.state_root_binding_sha256
            && self.state_root_device == intent.state_root_device
            && self.state_root_inode == intent.state_root_inode
            && self.state_root_mode == intent.state_root_mode
            && self.state_root_owner_uid == intent.state_root_owner_uid
            && self.state_root_owner_gid == intent.state_root_owner_gid
            && self.state_root_mount_id == intent.state_root_mount_id
            && self.state_root_lock_device == intent.state_root_lock_device
            && self.state_root_lock_inode == intent.state_root_lock_inode
            && self.attempt_identity_sha256 == intent.attempt_identity_sha256
            && self.active_attempt_record_sha256 == intent.active_attempt_record_sha256
            && self.active_attempt_file_device == intent.active_attempt_file_device
            && self.active_attempt_file_inode == intent.active_attempt_file_inode
            && self.barrier_generation == intent.barrier_generation
            && self.restore_plan_sha256 == intent.restore_plan_sha256
            && self.boot_id == intent.boot_id
            && self.boot_epoch == intent.boot_epoch
            && self.intent_global_sequence == intent.global_sequence
            && self.pre_intent_journal_tip_sha256 == intent.journal_tip_sha256
            && self.predecessor_record_sha256 == intent.predecessor_record_sha256
            && self.candidate_execution_request_sha256 == intent.candidate_execution_request_sha256
            && self.intent_manifest_sha256 == intent_manifest_sha256
            && self.intent_record_sha256 == intent_record_sha256
    }
}

fn issued_schema_v8(effect: JournalEffectV8) -> Result<&'static str, NativeErrorV8> {
    match effect {
        JournalEffectV8::CandidateExecution => {
            Ok("hepta-linux-v8-candidate-execution-issued-or-uncertain-v1")
        }
        JournalEffectV8::RunnerStop
        | JournalEffectV8::RunnerRestore
        | JournalEffectV8::CandidateRelay
        | JournalEffectV8::PostRestoreSnapshot
        | JournalEffectV8::BarrierRelease => Err(invalid(
            "effect has no frozen issued-or-uncertain evidence schema",
        )),
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct FrozenTransitionEffectEvidenceFieldsV8 {
    schema: String,
    phase: FrozenTransitionEvidencePhaseV8,
    effect: JournalEffectV8,
    machine_id_sha256: String,
    machine_id_source_device: u64,
    machine_id_source_inode: u64,
    state_root_binding_sha256: String,
    state_root_device: u64,
    state_root_inode: u64,
    state_root_mode: u32,
    state_root_owner_uid: u32,
    state_root_owner_gid: u32,
    state_root_mount_id: u64,
    state_root_lock_device: u64,
    state_root_lock_inode: u64,
    attempt_identity_sha256: String,
    active_attempt_record_sha256: String,
    active_attempt_file_device: u64,
    active_attempt_file_inode: u64,
    barrier_generation: u64,
    restore_plan_sha256: String,
    boot_id: String,
    boot_epoch: u64,
    global_sequence: u64,
    journal_tip_sha256: String,
    predecessor_record_sha256: String,
    candidate_execution_request_sha256: String,
    intent_manifest_sha256: Option<String>,
    intent_record_sha256: Option<String>,
    issued_or_uncertain: Option<DurableEffectIssuedOrUncertainFactV8>,
    effect_result_sha256: Option<String>,
    observation_started_monotonic_ns: Option<u64>,
    observation_completed_monotonic_ns: Option<u64>,
}

impl FrozenTransitionEffectEvidenceFieldsV8 {
    fn intent(
        schema: &'static str,
        effect: JournalEffectV8,
        context: FrozenTransitionIntentContextV8,
    ) -> Result<Self, NativeErrorV8> {
        let root = context.state_root_identity;
        let lock = context.state_root_lock_identity;
        let machine_source = context.machine_id_source_identity;
        let active_file = context.active_attempt_file_identity;
        let evidence = Self {
            schema: schema.to_string(),
            phase: FrozenTransitionEvidencePhaseV8::Intent,
            effect,
            machine_id_sha256: context.machine_id_sha256,
            machine_id_source_device: machine_source.device(),
            machine_id_source_inode: machine_source.inode(),
            state_root_binding_sha256: context.state_root_binding_sha256,
            state_root_device: root.device(),
            state_root_inode: root.inode(),
            state_root_mode: root.mode(),
            state_root_owner_uid: root.owner_uid(),
            state_root_owner_gid: root.owner_gid(),
            state_root_mount_id: context.state_root_mount_id,
            state_root_lock_device: lock.device(),
            state_root_lock_inode: lock.inode(),
            attempt_identity_sha256: context.attempt_identity_sha256,
            active_attempt_record_sha256: context.active_attempt_record_sha256,
            active_attempt_file_device: active_file.device(),
            active_attempt_file_inode: active_file.inode(),
            barrier_generation: context.barrier_generation,
            restore_plan_sha256: context.restore_plan_sha256,
            boot_id: context.boot_id,
            boot_epoch: context.boot_epoch,
            global_sequence: context.global_sequence,
            journal_tip_sha256: context.journal_tip_sha256,
            predecessor_record_sha256: context.predecessor_record_sha256,
            candidate_execution_request_sha256: context.candidate_execution_request_sha256,
            intent_manifest_sha256: None,
            intent_record_sha256: None,
            issued_or_uncertain: None,
            effect_result_sha256: None,
            observation_started_monotonic_ns: None,
            observation_completed_monotonic_ns: None,
        };
        evidence.validate(schema, effect)?;
        Ok(evidence)
    }

    #[allow(clippy::too_many_arguments)]
    fn observation(
        schema: &'static str,
        effect: JournalEffectV8,
        intent: &Self,
        intent_manifest_sha256: String,
        intent_record_sha256: String,
        issuance_nonce_sha256: String,
        issue_started_monotonic_ns: u64,
        issue_completed_monotonic_ns: u64,
        effect_result_sha256: String,
        observation_started_monotonic_ns: u64,
        observation_completed_monotonic_ns: u64,
    ) -> Result<Self, NativeErrorV8> {
        intent.validate(schema, effect)?;
        if intent.phase != FrozenTransitionEvidencePhaseV8::Intent {
            return Err(invalid("transition observation source is not an intent"));
        }
        let issued_or_uncertain = DurableEffectIssuedOrUncertainFactV8::from_intent(
            effect,
            intent,
            intent_manifest_sha256.clone(),
            intent_record_sha256.clone(),
            issuance_nonce_sha256,
            issue_started_monotonic_ns,
            issue_completed_monotonic_ns,
        )?;
        let global_sequence = intent
            .global_sequence
            .checked_add(1)
            .ok_or_else(|| invalid("transition observation global sequence overflows"))?;
        let evidence = Self {
            schema: schema.to_string(),
            phase: FrozenTransitionEvidencePhaseV8::Observation,
            effect,
            machine_id_sha256: intent.machine_id_sha256.clone(),
            machine_id_source_device: intent.machine_id_source_device,
            machine_id_source_inode: intent.machine_id_source_inode,
            state_root_binding_sha256: intent.state_root_binding_sha256.clone(),
            state_root_device: intent.state_root_device,
            state_root_inode: intent.state_root_inode,
            state_root_mode: intent.state_root_mode,
            state_root_owner_uid: intent.state_root_owner_uid,
            state_root_owner_gid: intent.state_root_owner_gid,
            state_root_mount_id: intent.state_root_mount_id,
            state_root_lock_device: intent.state_root_lock_device,
            state_root_lock_inode: intent.state_root_lock_inode,
            attempt_identity_sha256: intent.attempt_identity_sha256.clone(),
            active_attempt_record_sha256: intent.active_attempt_record_sha256.clone(),
            active_attempt_file_device: intent.active_attempt_file_device,
            active_attempt_file_inode: intent.active_attempt_file_inode,
            barrier_generation: intent.barrier_generation,
            restore_plan_sha256: intent.restore_plan_sha256.clone(),
            boot_id: intent.boot_id.clone(),
            boot_epoch: intent.boot_epoch,
            global_sequence,
            journal_tip_sha256: intent_record_sha256.clone(),
            predecessor_record_sha256: intent.predecessor_record_sha256.clone(),
            candidate_execution_request_sha256: intent.candidate_execution_request_sha256.clone(),
            intent_manifest_sha256: Some(intent_manifest_sha256),
            intent_record_sha256: Some(intent_record_sha256),
            issued_or_uncertain: Some(issued_or_uncertain),
            effect_result_sha256: Some(effect_result_sha256),
            observation_started_monotonic_ns: Some(observation_started_monotonic_ns),
            observation_completed_monotonic_ns: Some(observation_completed_monotonic_ns),
        };
        evidence.validate(schema, effect)?;
        Ok(evidence)
    }

    fn validate(&self, schema: &'static str, effect: JournalEffectV8) -> Result<(), NativeErrorV8> {
        if self.schema != schema
            || self.effect != effect
            || self.state_root_device == 0
            || self.state_root_inode == 0
            || self.state_root_mode != 0o700
            || self.state_root_mount_id == 0
            || self.state_root_lock_device == 0
            || self.state_root_lock_inode == 0
            || self.machine_id_source_device == 0
            || self.machine_id_source_inode == 0
            || self.active_attempt_file_device == 0
            || self.active_attempt_file_inode == 0
            || self.barrier_generation == 0
            || self.boot_epoch == 0
            || self.global_sequence < 2
        {
            return Err(invalid("typed transition evidence header is malformed"));
        }
        validate_boot_id_v8(&self.boot_id)?;
        for (label, digest) in [
            ("transition machine", self.machine_id_sha256.as_str()),
            (
                "transition state-root binding",
                self.state_root_binding_sha256.as_str(),
            ),
            ("transition attempt", self.attempt_identity_sha256.as_str()),
            (
                "transition active attempt",
                self.active_attempt_record_sha256.as_str(),
            ),
            ("transition restore plan", self.restore_plan_sha256.as_str()),
            ("transition journal tip", self.journal_tip_sha256.as_str()),
            (
                "transition predecessor",
                self.predecessor_record_sha256.as_str(),
            ),
            (
                "transition candidate execution request",
                self.candidate_execution_request_sha256.as_str(),
            ),
        ] {
            validate_digest(label, digest)?;
        }
        match self.phase {
            FrozenTransitionEvidencePhaseV8::Intent => {
                if self.journal_tip_sha256 != self.predecessor_record_sha256
                    || self.intent_manifest_sha256.is_some()
                    || self.intent_record_sha256.is_some()
                    || self.issued_or_uncertain.is_some()
                    || self.effect_result_sha256.is_some()
                    || self.observation_started_monotonic_ns.is_some()
                    || self.observation_completed_monotonic_ns.is_some()
                {
                    return Err(invalid("transition intent contains post-effect facts"));
                }
            }
            FrozenTransitionEvidencePhaseV8::Observation => {
                let intent_manifest_sha256 = self
                    .intent_manifest_sha256
                    .as_deref()
                    .ok_or_else(|| invalid("transition observation omits intent manifest"))?;
                let intent_record_sha256 = self
                    .intent_record_sha256
                    .as_deref()
                    .ok_or_else(|| invalid("transition observation omits intent record"))?;
                let effect_result_sha256 = self
                    .effect_result_sha256
                    .as_deref()
                    .ok_or_else(|| invalid("transition observation omits effect result"))?;
                validate_digest(
                    "transition observation intent manifest",
                    intent_manifest_sha256,
                )?;
                validate_digest("transition observation intent record", intent_record_sha256)?;
                validate_digest("transition observation effect result", effect_result_sha256)?;
                let started = self
                    .observation_started_monotonic_ns
                    .ok_or_else(|| invalid("transition observation omits start time"))?;
                let completed = self
                    .observation_completed_monotonic_ns
                    .ok_or_else(|| invalid("transition observation omits completion time"))?;
                let issued = self
                    .issued_or_uncertain
                    .as_ref()
                    .ok_or_else(|| invalid("transition observation omits issued fact"))?;
                issued.validate(effect)?;
                let expected_observation_sequence = issued
                    .intent_global_sequence
                    .checked_add(1)
                    .ok_or_else(|| invalid("transition observation global sequence overflows"))?;
                if self.journal_tip_sha256 != intent_record_sha256
                    || self.global_sequence != expected_observation_sequence
                    || started == 0
                    || completed < started
                    || started < issued.issue_completed_monotonic_ns
                {
                    return Err(invalid(
                        "transition observation chronology or journal binding is malformed",
                    ));
                }
            }
        }
        Ok(())
    }

    fn canonical_bytes(
        &self,
        schema: &'static str,
        effect: JournalEffectV8,
    ) -> Result<Vec<u8>, NativeErrorV8> {
        self.validate(schema, effect)?;
        serde_json::to_vec(self)
            .map_err(|error| invalid(format!("encode typed transition evidence: {error}")))
    }

    fn decode_exact(
        schema: &'static str,
        effect: JournalEffectV8,
        bytes: &[u8],
    ) -> Result<Self, NativeErrorV8> {
        if bytes.is_empty() || bytes.len() > MAX_FROZEN_TRANSITION_EVIDENCE_BYTES_V8 {
            return Err(invalid("typed transition evidence size is invalid"));
        }
        let evidence: Self = serde_json::from_slice(bytes)
            .map_err(|error| invalid(format!("decode typed transition evidence: {error}")))?;
        evidence.validate(schema, effect)?;
        if evidence.canonical_bytes(schema, effect)? != bytes {
            return Err(invalid("typed transition evidence bytes are not canonical"));
        }
        Ok(evidence)
    }

    fn sha256(
        &self,
        schema: &'static str,
        effect: JournalEffectV8,
    ) -> Result<String, NativeErrorV8> {
        Ok(format!(
            "{:x}",
            sha2::Sha256::digest(self.canonical_bytes(schema, effect)?)
        ))
    }

    fn validate_record_context(
        &self,
        schema: &'static str,
        effect: JournalEffectV8,
        record: &DurableJournalRecordV8,
        state_root: FileIdentityV8,
        state_root_mount_id: u64,
        state_root_lock: FileIdentityV8,
    ) -> Result<(), NativeErrorV8> {
        self.validate(schema, effect)?;
        if self.attempt_identity_sha256 != record.attempt_identity_sha256()
            || self.boot_id != record.boot_id()
            || self.boot_epoch != record.boot_epoch()
            || self.global_sequence != record.global_sequence()
            || self.journal_tip_sha256 != record.previous_record_sha256()
            || self.state_root_device != state_root.device()
            || self.state_root_inode != state_root.inode()
            || self.state_root_mode != state_root.mode()
            || self.state_root_owner_uid != state_root.owner_uid()
            || self.state_root_owner_gid != state_root.owner_gid()
            || self.state_root_mount_id != state_root_mount_id
            || self.state_root_lock_device != state_root_lock.device()
            || self.state_root_lock_inode != state_root_lock.inode()
        {
            return Err(invalid(
                "typed transition evidence differs from its journal/root context",
            ));
        }
        if self.phase == FrozenTransitionEvidencePhaseV8::Intent
            && self.predecessor_record_sha256 != record.previous_record_sha256()
        {
            return Err(invalid(
                "typed transition intent does not bind its exact predecessor",
            ));
        }
        if self.phase == FrozenTransitionEvidencePhaseV8::Observation
            && self.intent_record_sha256.as_deref() != Some(record.previous_record_sha256())
        {
            return Err(invalid(
                "typed transition observation does not follow its exact intent",
            ));
        }
        Ok(())
    }

    fn validate_descriptor_origin(
        &self,
        schema: &'static str,
        effect: JournalEffectV8,
        record: &DurableJournalRecordV8,
        previous_record: &DurableJournalRecordV8,
        origin: &DescriptorReplayOriginV8<'_>,
    ) -> Result<(), NativeErrorV8> {
        self.validate(schema, effect)?;
        let previous_record_sha256 = previous_record.record_sha256()?;
        let expected_sequence = previous_record
            .global_sequence()
            .checked_add(1)
            .ok_or_else(|| invalid("descriptor-bound journal sequence overflows"))?;
        if previous_record_sha256 != record.previous_record_sha256()
            || expected_sequence != record.global_sequence()
            || self.machine_id_sha256 != origin.machine_id_sha256
            || self.machine_id_source_device != origin.machine_id_source_identity.device()
            || self.machine_id_source_inode != origin.machine_id_source_identity.inode()
            || self.state_root_binding_sha256 != origin.state_root_binding_sha256
            || self.state_root_device != origin.state_root_identity.device()
            || self.state_root_inode != origin.state_root_identity.inode()
            || self.state_root_mode != origin.state_root_identity.mode()
            || self.state_root_owner_uid != origin.state_root_identity.owner_uid()
            || self.state_root_owner_gid != origin.state_root_identity.owner_gid()
            || self.state_root_mount_id != origin.state_root_mount_id
            || self.state_root_lock_device != origin.state_root_lock_identity.device()
            || self.state_root_lock_inode != origin.state_root_lock_identity.inode()
            || self.attempt_identity_sha256 != origin.attempt_identity_sha256
            || self.active_attempt_record_sha256 != origin.active_attempt_record_sha256
            || self.active_attempt_file_device != origin.active_attempt_file_identity.device()
            || self.active_attempt_file_inode != origin.active_attempt_file_identity.inode()
            || self.barrier_generation != origin.barrier_generation
            || self.restore_plan_sha256 != origin.restore_plan_sha256
            || self.boot_id != origin.boot_id
            || self.attempt_identity_sha256 != record.attempt_identity_sha256()
            || self.boot_id != record.boot_id()
            || self.boot_epoch != record.boot_epoch()
            || self.global_sequence != record.global_sequence()
            || self.journal_tip_sha256 != previous_record_sha256
        {
            return Err(invalid(
                "typed candidate evidence differs from its retained live origin or record chain",
            ));
        }
        match self.phase {
            FrozenTransitionEvidencePhaseV8::Intent
                if self.predecessor_record_sha256 != previous_record_sha256 =>
            {
                Err(invalid(
                    "typed candidate intent does not bind its immediately preceding retained record",
                ))
            }
            FrozenTransitionEvidencePhaseV8::Observation
                if self.intent_record_sha256.as_deref()
                    != Some(previous_record_sha256.as_str()) =>
            {
                Err(invalid(
                    "typed candidate observation does not bind its immediately preceding retained intent record",
                ))
            }
            FrozenTransitionEvidencePhaseV8::Intent
            | FrozenTransitionEvidencePhaseV8::Observation => Ok(()),
        }
    }

    fn closes_exact_manifest(
        &self,
        schema: &'static str,
        effect: JournalEffectV8,
        intent: &Self,
        intent_manifest_sha256: &str,
        intent_record_sha256: &str,
    ) -> bool {
        self.validate(schema, effect).is_ok()
            && intent.validate(schema, effect).is_ok()
            && self.phase == FrozenTransitionEvidencePhaseV8::Observation
            && intent.phase == FrozenTransitionEvidencePhaseV8::Intent
            && self.machine_id_sha256 == intent.machine_id_sha256
            && self.machine_id_source_device == intent.machine_id_source_device
            && self.machine_id_source_inode == intent.machine_id_source_inode
            && self.state_root_binding_sha256 == intent.state_root_binding_sha256
            && self.state_root_device == intent.state_root_device
            && self.state_root_inode == intent.state_root_inode
            && self.state_root_mode == intent.state_root_mode
            && self.state_root_owner_uid == intent.state_root_owner_uid
            && self.state_root_owner_gid == intent.state_root_owner_gid
            && self.state_root_mount_id == intent.state_root_mount_id
            && self.state_root_lock_device == intent.state_root_lock_device
            && self.state_root_lock_inode == intent.state_root_lock_inode
            && self.attempt_identity_sha256 == intent.attempt_identity_sha256
            && self.active_attempt_record_sha256 == intent.active_attempt_record_sha256
            && self.active_attempt_file_device == intent.active_attempt_file_device
            && self.active_attempt_file_inode == intent.active_attempt_file_inode
            && self.barrier_generation == intent.barrier_generation
            && self.restore_plan_sha256 == intent.restore_plan_sha256
            && self.boot_id == intent.boot_id
            && self.boot_epoch == intent.boot_epoch
            && intent
                .global_sequence
                .checked_add(1)
                .is_some_and(|expected| self.global_sequence == expected)
            && self.predecessor_record_sha256 == intent.predecessor_record_sha256
            && self.candidate_execution_request_sha256 == intent.candidate_execution_request_sha256
            && self.intent_manifest_sha256.as_deref() == Some(intent_manifest_sha256)
            && self.intent_record_sha256.as_deref() == Some(intent_record_sha256)
            && self.journal_tip_sha256 == intent_record_sha256
            && self.issued_or_uncertain.as_ref().is_some_and(|issued| {
                issued.matches_intent(effect, intent, intent_manifest_sha256, intent_record_sha256)
            })
    }
}

macro_rules! define_frozen_transition_evidence {
    ($name:ident, $schema:literal, $effect:path) => {
        #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
        #[serde(transparent)]
        pub(crate) struct $name(FrozenTransitionEffectEvidenceFieldsV8);

        impl $name {
            pub(crate) fn intent(
                context: FrozenTransitionIntentContextV8,
            ) -> Result<Self, NativeErrorV8> {
                Ok(Self(FrozenTransitionEffectEvidenceFieldsV8::intent(
                    $schema, $effect, context,
                )?))
            }

            #[allow(clippy::too_many_arguments)]
            pub(crate) fn observation(
                intent: &Self,
                intent_manifest_sha256: String,
                intent_record_sha256: String,
                issuance_nonce_sha256: String,
                issue_started_monotonic_ns: u64,
                issue_completed_monotonic_ns: u64,
                effect_result_sha256: String,
                observation_started_monotonic_ns: u64,
                observation_completed_monotonic_ns: u64,
            ) -> Result<Self, NativeErrorV8> {
                Ok(Self(FrozenTransitionEffectEvidenceFieldsV8::observation(
                    $schema,
                    $effect,
                    &intent.0,
                    intent_manifest_sha256,
                    intent_record_sha256,
                    issuance_nonce_sha256,
                    issue_started_monotonic_ns,
                    issue_completed_monotonic_ns,
                    effect_result_sha256,
                    observation_started_monotonic_ns,
                    observation_completed_monotonic_ns,
                )?))
            }

            pub(crate) fn canonical_bytes(&self) -> Result<Vec<u8>, NativeErrorV8> {
                self.0.canonical_bytes($schema, $effect)
            }

            pub(crate) fn sha256(&self) -> Result<String, NativeErrorV8> {
                self.0.sha256($schema, $effect)
            }

            pub(crate) fn decode_exact(bytes: &[u8]) -> Result<Self, NativeErrorV8> {
                Ok(Self(FrozenTransitionEffectEvidenceFieldsV8::decode_exact(
                    $schema, $effect, bytes,
                )?))
            }
        }
    };
}

define_frozen_transition_evidence!(
    CandidateExecutionEffectEvidenceV8,
    "hepta-linux-v8-candidate-execution-evidence-v1",
    JournalEffectV8::CandidateExecution
);

impl CandidateExecutionEffectEvidenceV8 {
    pub(crate) fn phase(&self) -> FrozenTransitionEvidencePhaseV8 {
        self.0.phase
    }

    pub(crate) fn predecessor_record_sha256(&self) -> &str {
        &self.0.predecessor_record_sha256
    }

    pub(crate) fn effect_result_sha256(&self) -> Option<&str> {
        self.0.effect_result_sha256.as_deref()
    }

    pub(crate) fn intent_record_sha256(&self) -> Option<&str> {
        self.0.intent_record_sha256.as_deref()
    }

    pub(crate) fn validate_record_context(
        &self,
        record: &DurableJournalRecordV8,
        state_root: FileIdentityV8,
        state_root_mount_id: u64,
        state_root_lock: FileIdentityV8,
    ) -> Result<(), NativeErrorV8> {
        self.0.validate_record_context(
            "hepta-linux-v8-candidate-execution-evidence-v1",
            JournalEffectV8::CandidateExecution,
            record,
            state_root,
            state_root_mount_id,
            state_root_lock,
        )
    }

    pub(crate) fn validate_descriptor_origin(
        &self,
        record: &DurableJournalRecordV8,
        previous_record: &DurableJournalRecordV8,
        origin: &DescriptorReplayOriginV8<'_>,
    ) -> Result<(), NativeErrorV8> {
        self.0.validate_descriptor_origin(
            "hepta-linux-v8-candidate-execution-evidence-v1",
            JournalEffectV8::CandidateExecution,
            record,
            previous_record,
            origin,
        )
    }

    pub(crate) fn closes_exact_manifest(
        &self,
        intent: &Self,
        intent_manifest_sha256: &str,
        intent_record_sha256: &str,
    ) -> bool {
        self.0.closes_exact_manifest(
            "hepta-linux-v8-candidate-execution-evidence-v1",
            JournalEffectV8::CandidateExecution,
            &intent.0,
            intent_manifest_sha256,
            intent_record_sha256,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(character: char) -> String {
        character.to_string().repeat(64)
    }

    fn identity() -> FileIdentityV8 {
        FileIdentityV8::for_test_only(7, 8, 0, 0, 0o700, 1, 0)
    }

    fn leaf_identity(device: u64, inode: u64, mode: u32) -> FileIdentityV8 {
        FileIdentityV8::for_test_only(device, inode, 0, 0, mode, 1, 32)
    }

    fn context() -> FrozenTransitionIntentContextV8 {
        FrozenTransitionIntentContextV8 {
            machine_id_sha256: digest('1'),
            machine_id_source_identity: leaf_identity(9, 10, 0o444),
            state_root_binding_sha256: digest('2'),
            state_root_identity: identity(),
            state_root_mount_id: 11,
            state_root_lock_identity: leaf_identity(7, 12, 0o600),
            attempt_identity_sha256: digest('3'),
            active_attempt_record_sha256: digest('4'),
            active_attempt_file_identity: leaf_identity(7, 13, 0o600),
            barrier_generation: 9,
            restore_plan_sha256: digest('5'),
            boot_id: "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            boot_epoch: 1,
            global_sequence: 2,
            journal_tip_sha256: digest('6'),
            predecessor_record_sha256: digest('6'),
            candidate_execution_request_sha256: digest('7'),
        }
    }

    macro_rules! assert_lifecycle {
        ($type:ty, $effect:path) => {{
            let intent = <$type>::intent(context()).unwrap();
            let intent_bytes = intent.canonical_bytes().unwrap();
            let intent_sha256 = intent.sha256().unwrap();
            assert_eq!(<$type>::decode_exact(&intent_bytes).unwrap(), intent);

            let observation = <$type>::observation(
                &intent,
                intent_sha256.clone(),
                digest('8'),
                digest('9'),
                10,
                20,
                digest('a'),
                20,
                30,
            )
            .unwrap();
            let observation_bytes = observation.canonical_bytes().unwrap();
            assert_eq!(
                <$type>::decode_exact(&observation_bytes).unwrap(),
                observation
            );
            assert!(observation.closes_exact_manifest(&intent, &intent_sha256, &digest('8')));
            let issued_bytes = observation
                .0
                .issued_or_uncertain
                .as_ref()
                .unwrap()
                .canonical_bytes()
                .unwrap();
            assert_eq!(
                DurableEffectIssuedOrUncertainFactV8::decode_exact($effect, &issued_bytes).unwrap(),
                *observation.0.issued_or_uncertain.as_ref().unwrap()
            );
        }};
    }

    #[test]
    fn candidate_execution_lifecycle_is_exact_canonical_and_closed() {
        assert_lifecycle!(
            CandidateExecutionEffectEvidenceV8,
            JournalEffectV8::CandidateExecution
        );
    }

    #[test]
    fn descriptor_origin_binds_live_fields_and_adjacent_retained_records() {
        const ZERO_SHA256: &str =
            "0000000000000000000000000000000000000000000000000000000000000000";

        let previous = DurableJournalRecordV8::new(
            digest('3'),
            1,
            "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            1,
            ZERO_SHA256.to_string(),
            b"retained predecessor".to_vec(),
        )
        .unwrap();
        let previous_sha256 = previous.record_sha256().unwrap();
        let mut context = context();
        context.journal_tip_sha256 = previous_sha256.clone();
        context.predecessor_record_sha256 = previous_sha256.clone();
        let intent = CandidateExecutionEffectEvidenceV8::intent(context.clone()).unwrap();
        let intent_record = DurableJournalRecordV8::new(
            context.attempt_identity_sha256.clone(),
            context.boot_epoch,
            context.boot_id.clone(),
            context.global_sequence,
            previous_sha256,
            b"candidate intent envelope".to_vec(),
        )
        .unwrap();
        let origin = DescriptorReplayOriginV8 {
            machine_id_sha256: &context.machine_id_sha256,
            machine_id_source_identity: context.machine_id_source_identity,
            state_root_binding_sha256: &context.state_root_binding_sha256,
            state_root_identity: context.state_root_identity,
            state_root_mount_id: context.state_root_mount_id,
            state_root_lock_identity: context.state_root_lock_identity,
            attempt_identity_sha256: &context.attempt_identity_sha256,
            active_attempt_record_sha256: &context.active_attempt_record_sha256,
            active_attempt_file_identity: context.active_attempt_file_identity,
            barrier_generation: context.barrier_generation,
            restore_plan_sha256: &context.restore_plan_sha256,
            boot_id: &context.boot_id,
        };
        intent
            .validate_descriptor_origin(&intent_record, &previous, &origin)
            .unwrap();

        let bad_digest = digest('f');
        let bad_boot = "11111111-1111-1111-1111-111111111111".to_string();
        let mut bad = origin;
        bad.machine_id_sha256 = &bad_digest;
        assert!(
            intent
                .validate_descriptor_origin(&intent_record, &previous, &bad)
                .is_err()
        );
        let mut bad = origin;
        bad.machine_id_source_identity = leaf_identity(99, 10, 0o444);
        assert!(
            intent
                .validate_descriptor_origin(&intent_record, &previous, &bad)
                .is_err()
        );
        let mut bad = origin;
        bad.state_root_binding_sha256 = &bad_digest;
        assert!(
            intent
                .validate_descriptor_origin(&intent_record, &previous, &bad)
                .is_err()
        );
        let mut bad = origin;
        bad.state_root_identity = FileIdentityV8::for_test_only(7, 99, 0, 0, 0o700, 1, 0);
        assert!(
            intent
                .validate_descriptor_origin(&intent_record, &previous, &bad)
                .is_err()
        );
        let mut bad = origin;
        bad.state_root_mount_id += 1;
        assert!(
            intent
                .validate_descriptor_origin(&intent_record, &previous, &bad)
                .is_err()
        );
        let mut bad = origin;
        bad.state_root_lock_identity = leaf_identity(7, 99, 0o600);
        assert!(
            intent
                .validate_descriptor_origin(&intent_record, &previous, &bad)
                .is_err()
        );
        let mut bad = origin;
        bad.attempt_identity_sha256 = &bad_digest;
        assert!(
            intent
                .validate_descriptor_origin(&intent_record, &previous, &bad)
                .is_err()
        );
        let mut bad = origin;
        bad.active_attempt_record_sha256 = &bad_digest;
        assert!(
            intent
                .validate_descriptor_origin(&intent_record, &previous, &bad)
                .is_err()
        );
        let mut bad = origin;
        bad.active_attempt_file_identity = leaf_identity(7, 99, 0o600);
        assert!(
            intent
                .validate_descriptor_origin(&intent_record, &previous, &bad)
                .is_err()
        );
        let mut bad = origin;
        bad.barrier_generation += 1;
        assert!(
            intent
                .validate_descriptor_origin(&intent_record, &previous, &bad)
                .is_err()
        );
        let mut bad = origin;
        bad.restore_plan_sha256 = &bad_digest;
        assert!(
            intent
                .validate_descriptor_origin(&intent_record, &previous, &bad)
                .is_err()
        );
        let mut bad = origin;
        bad.boot_id = &bad_boot;
        assert!(
            intent
                .validate_descriptor_origin(&intent_record, &previous, &bad)
                .is_err()
        );

        let different_previous = DurableJournalRecordV8::new(
            context.attempt_identity_sha256.clone(),
            1,
            context.boot_id.clone(),
            1,
            ZERO_SHA256.to_string(),
            b"different predecessor".to_vec(),
        )
        .unwrap();
        assert!(
            intent
                .validate_descriptor_origin(&intent_record, &different_previous, &origin)
                .is_err()
        );
        let wrong_sequence_record = DurableJournalRecordV8::new(
            context.attempt_identity_sha256.clone(),
            1,
            context.boot_id.clone(),
            3,
            previous.record_sha256().unwrap(),
            b"wrong containing sequence".to_vec(),
        )
        .unwrap();
        assert!(
            intent
                .validate_descriptor_origin(&wrong_sequence_record, &previous, &origin)
                .is_err()
        );

        let intent_manifest_sha256 = intent.sha256().unwrap();
        let intent_record_sha256 = intent_record.record_sha256().unwrap();
        let observation = CandidateExecutionEffectEvidenceV8::observation(
            &intent,
            intent_manifest_sha256,
            intent_record_sha256.clone(),
            digest('8'),
            10,
            20,
            digest('9'),
            20,
            30,
        )
        .unwrap();
        let observation_record = DurableJournalRecordV8::new(
            context.attempt_identity_sha256.clone(),
            1,
            context.boot_id.clone(),
            3,
            intent_record_sha256,
            b"candidate observation envelope".to_vec(),
        )
        .unwrap();
        observation
            .validate_descriptor_origin(&observation_record, &intent_record, &origin)
            .unwrap();
    }

    #[test]
    fn cross_effect_and_every_context_splice_fail_closed() {
        let intent = CandidateExecutionEffectEvidenceV8::intent(context()).unwrap();
        let intent_bytes = intent.canonical_bytes().unwrap();
        let mut wrong_effect: serde_json::Value = serde_json::from_slice(&intent_bytes).unwrap();
        wrong_effect["effect"] = serde_json::json!("candidate_relay");
        assert!(
            CandidateExecutionEffectEvidenceV8::decode_exact(
                &serde_json::to_vec(&wrong_effect).unwrap()
            )
            .is_err()
        );

        let intent_sha256 = intent.sha256().unwrap();
        let observation = CandidateExecutionEffectEvidenceV8::observation(
            &intent,
            intent_sha256.clone(),
            digest('8'),
            digest('9'),
            10,
            20,
            digest('a'),
            20,
            30,
        )
        .unwrap();
        for mutate in [
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.machine_id_sha256 = digest('b')
            },
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.machine_id_source_device += 1
            },
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.machine_id_source_inode += 1
            },
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.state_root_binding_sha256 = digest('c')
            },
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| fields.state_root_device += 1,
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| fields.state_root_inode += 1,
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| fields.state_root_mode += 1,
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| fields.state_root_owner_uid += 1,
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| fields.state_root_owner_gid += 1,
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| fields.state_root_mount_id += 1,
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.state_root_lock_device += 1
            },
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| fields.state_root_lock_inode += 1,
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.attempt_identity_sha256 = digest('d')
            },
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.active_attempt_record_sha256 = digest('e')
            },
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.active_attempt_file_device += 1
            },
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.active_attempt_file_inode += 1
            },
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| fields.barrier_generation += 1,
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.restore_plan_sha256 = digest('f')
            },
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.boot_id = "11111111-1111-1111-1111-111111111111".to_string()
            },
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| fields.boot_epoch += 1,
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| fields.global_sequence += 1,
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.journal_tip_sha256 = digest('0')
            },
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.predecessor_record_sha256 = digest('1')
            },
            |fields: &mut FrozenTransitionEffectEvidenceFieldsV8| {
                fields.candidate_execution_request_sha256 = digest('2')
            },
        ] as [fn(&mut FrozenTransitionEffectEvidenceFieldsV8); 24]
        {
            let mut spliced = observation.clone();
            mutate(&mut spliced.0);
            assert!(!spliced.closes_exact_manifest(&intent, &intent_sha256, &digest('8')));
        }

        let mut noncanonical = intent_bytes;
        noncanonical.push(b'\n');
        assert!(CandidateExecutionEffectEvidenceV8::decode_exact(&noncanonical).is_err());
    }
}

//! Durable one-shot state for operator-authenticated native mutations.
//!
//! The journal is a bounded, authenticated state file published with the
//! descriptor-relative atomic writer shared by Telegram durability. A
//! mutation identifier can be reserved only once. Commit consumes the plan
//! before RuntimeKernel approval, so crashes and ambiguous execution become
//! fail-closed `in_doubt` states rather than replayable authority.

use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use hepta_authority::AuthenticationFraming;
use hepta_authority::OPERATOR_MUTATION_JOURNAL_ENGINE;
use hepta_authority::OPERATOR_MUTATION_JOURNAL_POLICY;
use hepta_authority::decode_sha256_hex;
use hepta_runtime::RuntimeExecutionReceipt;
use serde::Deserialize;
use serde::Serialize;

use crate::telegram_durable_files::read_private_state;
use crate::telegram_durable_files::update_private_state_atomically;

const JOURNAL_SCHEMA: &str = OPERATOR_MUTATION_JOURNAL_POLICY.schema;
const LEGACY_JOURNAL_SCHEMA: &str = "hepta.native.operator-mutation-journal.v1";
const JOURNAL_MAC_DOMAIN: &[u8] = b"hepta.native.operator-mutation-journal.hmac-sha256.v1";
const JOURNAL_STATE_HASH_DOMAIN: &[u8] = b"hepta.native.operator-mutation-journal.state-sha256.v1";
const MAX_JOURNAL_BYTES: u64 = OPERATOR_MUTATION_JOURNAL_POLICY.max_journal_bytes;
const MAX_JOURNAL_RECORDS: usize = OPERATOR_MUTATION_JOURNAL_POLICY.max_active_records;
const MAX_CHECKPOINTED_AUTHORITIES: usize =
    OPERATOR_MUTATION_JOURNAL_POLICY.max_checkpointed_authorities;
const RETAIN_SUCCEEDED_RECORDS: usize = 512;
const CHECKPOINT_GENESIS_HASH: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000000";
const CHECKPOINT_HASH_DOMAIN: &[u8] = b"hepta.native.operator-mutation-checkpoint.sha256.v1";
const STAGING_PREFIX: &str = ".hepta-operator-mutation-journal";

#[derive(Debug, Clone)]
pub(crate) struct OperatorMutationJournal {
    path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OperatorMutationMonotonicState {
    pub(crate) schema: &'static str,
    pub(crate) journal_revision: u64,
    pub(crate) state_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct OperatorMutationJournalInspection {
    pub(crate) plan_hash: String,
    pub(crate) phase: &'static str,
    pub(crate) plan_request_binding_hash: String,
    pub(crate) session_binding_hash: String,
    pub(crate) candidate_binding_hash: Option<String>,
    pub(crate) commit_request_binding_hash: Option<String>,
    pub(crate) attempt_id: Option<String>,
    pub(crate) effect_plan_hash: Option<String>,
    pub(crate) provider_effect_ack_hash: Option<String>,
    pub(crate) terminal_receipt_id: Option<String>,
    pub(crate) terminal_receipt_hash: Option<String>,
    pub(crate) terminal_outcome_hash: Option<String>,
    pub(crate) terminal_evidence_hash: Option<String>,
    pub(crate) terminal_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct JournalState {
    schema: String,
    revision: u64,
    #[serde(default)]
    checkpoint: JournalCheckpoint,
    records: Vec<JournalRecord>,
    mac: String,
}

#[derive(Debug, Clone, Serialize)]
struct UnsignedJournalState<'a> {
    schema: &'a str,
    revision: u64,
    checkpoint: &'a JournalCheckpoint,
    records: &'a [JournalRecord],
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct JournalCheckpoint {
    compacted_records: u64,
    #[serde(default)]
    consumed_authorities: Vec<ConsumedAuthority>,
    #[serde(default)]
    history_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConsumedAuthority {
    mutation_id_hash: String,
    plan_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct JournalRecord {
    mutation_id_hash: String,
    plan_hash: String,
    plan_request_binding_hash: String,
    session_binding_hash: String,
    candidate_binding_hash: Option<String>,
    commit_request_binding_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    attempt_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    effect_plan_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    provider_effect_ack_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    terminal_receipt_id: Option<String>,
    terminal_receipt_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    terminal_outcome_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    terminal_evidence_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    terminal_status: Option<String>,
    phase: JournalPhase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum JournalPhase {
    Reserved,
    Planned,
    Committing,
    Succeeded,
    InDoubt,
}

impl OperatorMutationJournal {
    pub(crate) fn for_key_file(key_file: &Path) -> Result<Self> {
        let file_name = key_file
            .file_name()
            .context("operator mutation key path must name a file")?
            .to_str()
            .context("operator mutation key file name must be UTF-8")?;
        if file_name.is_empty() || file_name.contains('/') {
            anyhow::bail!("operator mutation key file name is invalid");
        }
        Ok(Self {
            path: key_file.with_file_name(format!(".{file_name}.operator-mutations.v1.json")),
        })
    }

    #[cfg(test)]
    pub(crate) fn for_test_path(path: PathBuf) -> Self {
        Self { path }
    }

    pub(crate) fn reserve_plan(
        &self,
        key: &[u8],
        mutation_id_hash: &str,
        plan_hash: &str,
        plan_request_binding_hash: &str,
        session_binding_hash: &str,
    ) -> Result<()> {
        validate_hex(mutation_id_hash, "mutation id hash")?;
        validate_hex(plan_hash, "plan hash")?;
        validate_hex(plan_request_binding_hash, "plan request binding")?;
        validate_hex(session_binding_hash, "session binding")?;
        self.update(key, |state| {
            compact_succeeded_records(state, RETAIN_SUCCEEDED_RECORDS)?;
            if state.records.len() >= MAX_JOURNAL_RECORDS {
                anyhow::bail!("operator mutation journal record limit reached");
            }
            if authority_consumed(state, mutation_id_hash, plan_hash) {
                anyhow::bail!("operator mutation identifier or plan was already consumed");
            }
            state.records.push(JournalRecord {
                mutation_id_hash: mutation_id_hash.to_string(),
                plan_hash: plan_hash.to_string(),
                plan_request_binding_hash: plan_request_binding_hash.to_string(),
                session_binding_hash: session_binding_hash.to_string(),
                candidate_binding_hash: None,
                commit_request_binding_hash: None,
                attempt_id: None,
                effect_plan_hash: None,
                provider_effect_ack_hash: None,
                terminal_receipt_id: None,
                terminal_receipt_hash: None,
                terminal_outcome_hash: None,
                terminal_evidence_hash: None,
                terminal_status: None,
                phase: JournalPhase::Reserved,
            });
            Ok(())
        })
    }

    pub(crate) fn validate_reservable(
        &self,
        key: &[u8],
        mutation_id_hash: &str,
        plan_hash: &str,
    ) -> Result<()> {
        validate_hex(mutation_id_hash, "mutation id hash")?;
        validate_hex(plan_hash, "plan hash")?;
        let state = self.read(key)?;
        if state.records.len() >= MAX_JOURNAL_RECORDS && removable_succeeded_records(&state) == 0 {
            anyhow::bail!("operator mutation journal record limit reached");
        }
        if authority_consumed(&state, mutation_id_hash, plan_hash) {
            anyhow::bail!("operator mutation identifier or plan was already consumed");
        }
        Ok(())
    }

    pub(crate) fn publish_candidate(
        &self,
        key: &[u8],
        plan_hash: &str,
        candidate_binding_hash: &str,
    ) -> Result<()> {
        validate_hex(plan_hash, "plan hash")?;
        validate_content_hash(candidate_binding_hash, "candidate binding")?;
        self.update(key, |state| {
            let record = exact_record_mut(state, plan_hash)?;
            if record.phase != JournalPhase::Reserved
                || record.candidate_binding_hash.is_some()
                || record.commit_request_binding_hash.is_some()
            {
                anyhow::bail!("operator mutation plan is not reservable");
            }
            record.candidate_binding_hash = Some(candidate_binding_hash.to_string());
            record.phase = JournalPhase::Planned;
            Ok(())
        })
    }

    pub(crate) fn begin_commit(
        &self,
        key: &[u8],
        mutation_id_hash: &str,
        plan_hash: &str,
        plan_request_binding_hash: &str,
        session_binding_hash: &str,
        candidate_binding_hash: &str,
        commit_request_binding_hash: &str,
    ) -> Result<()> {
        validate_hex(mutation_id_hash, "mutation id hash")?;
        validate_hex(plan_hash, "plan hash")?;
        validate_hex(plan_request_binding_hash, "plan request binding")?;
        validate_hex(session_binding_hash, "session binding")?;
        validate_content_hash(candidate_binding_hash, "candidate binding")?;
        validate_hex(commit_request_binding_hash, "commit request binding")?;
        self.update(key, |state| {
            let record = exact_record_mut(state, plan_hash)?;
            if record.phase != JournalPhase::Planned
                || record.mutation_id_hash != mutation_id_hash
                || record.plan_request_binding_hash != plan_request_binding_hash
                || record.session_binding_hash != session_binding_hash
                || record.candidate_binding_hash.as_deref() != Some(candidate_binding_hash)
                || record.commit_request_binding_hash.is_some()
            {
                anyhow::bail!("operator mutation journal does not match the exact pending plan");
            }
            record.commit_request_binding_hash = Some(commit_request_binding_hash.to_string());
            record.phase = JournalPhase::Committing;
            Ok(())
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn validate_committable(
        &self,
        key: &[u8],
        mutation_id_hash: &str,
        plan_hash: &str,
        plan_request_binding_hash: &str,
        session_binding_hash: &str,
        candidate_binding_hash: &str,
    ) -> Result<()> {
        validate_hex(mutation_id_hash, "mutation id hash")?;
        validate_hex(plan_hash, "plan hash")?;
        validate_hex(plan_request_binding_hash, "plan request binding")?;
        validate_hex(session_binding_hash, "session binding")?;
        validate_content_hash(candidate_binding_hash, "candidate binding")?;
        let state = self.read(key)?;
        let matches = state
            .records
            .iter()
            .filter(|record| record.plan_hash == plan_hash)
            .collect::<Vec<_>>();
        let [record] = matches.as_slice() else {
            anyhow::bail!("operator mutation journal plan is missing or ambiguous");
        };
        if record.phase != JournalPhase::Planned
            || record.mutation_id_hash != mutation_id_hash
            || record.plan_request_binding_hash != plan_request_binding_hash
            || record.session_binding_hash != session_binding_hash
            || record.candidate_binding_hash.as_deref() != Some(candidate_binding_hash)
            || record.commit_request_binding_hash.is_some()
        {
            anyhow::bail!("operator mutation journal does not match the exact pending plan");
        }
        Ok(())
    }

    pub(crate) fn record_runtime_linkage(
        &self,
        key: &[u8],
        plan_hash: &str,
        receipt: &RuntimeExecutionReceipt,
    ) -> Result<()> {
        validate_hex(plan_hash, "plan hash")?;
        validate_runtime_receipt(receipt)?;
        self.update(key, |state| {
            let record = exact_record_mut(state, plan_hash)?;
            if record.phase != JournalPhase::Committing
                || record.commit_request_binding_hash.is_none()
                || runtime_linkage_present(record)
            {
                anyhow::bail!("operator mutation journal is not committing");
            }
            record.attempt_id = Some(receipt.attempt_id.clone());
            record.effect_plan_hash = receipt.effect_plan_hash.clone();
            record.provider_effect_ack_hash = receipt.provider_effect_ack_hash.clone();
            record.terminal_receipt_id = Some(receipt.terminal_receipt_id.clone());
            record.terminal_receipt_hash = Some(receipt.terminal_receipt_hash.clone());
            record.terminal_outcome_hash = Some(receipt.terminal_outcome_hash.clone());
            record.terminal_evidence_hash = Some(receipt.terminal_evidence_hash.clone());
            record.terminal_status = Some(receipt.terminal_status.clone());
            record.phase = JournalPhase::InDoubt;
            Ok(())
        })
    }

    pub(crate) fn finalize_linked_success(
        &self,
        key: &[u8],
        plan_hash: &str,
        receipt: &RuntimeExecutionReceipt,
    ) -> Result<()> {
        validate_hex(plan_hash, "plan hash")?;
        validate_runtime_receipt(receipt)?;
        self.update(key, |state| {
            let record = exact_record_mut(state, plan_hash)?;
            if record.phase != JournalPhase::InDoubt || !runtime_linkage_matches(record, receipt) {
                anyhow::bail!("operator mutation runtime linkage is not exactly reconcilable");
            }
            record.phase = JournalPhase::Succeeded;
            Ok(())
        })
    }

    pub(crate) fn inspect(
        &self,
        key: &[u8],
        plan_hash: &str,
    ) -> Result<OperatorMutationJournalInspection> {
        validate_hex(plan_hash, "plan hash")?;
        let state = self.read(key)?;
        let matches = state
            .records
            .iter()
            .filter(|record| record.plan_hash == plan_hash)
            .collect::<Vec<_>>();
        let [record] = matches.as_slice() else {
            anyhow::bail!("operator mutation journal plan is missing or ambiguous");
        };
        Ok(OperatorMutationJournalInspection {
            plan_hash: record.plan_hash.clone(),
            phase: record.phase.as_str(),
            plan_request_binding_hash: record.plan_request_binding_hash.clone(),
            session_binding_hash: record.session_binding_hash.clone(),
            candidate_binding_hash: record.candidate_binding_hash.clone(),
            commit_request_binding_hash: record.commit_request_binding_hash.clone(),
            attempt_id: record.attempt_id.clone(),
            effect_plan_hash: record.effect_plan_hash.clone(),
            provider_effect_ack_hash: record.provider_effect_ack_hash.clone(),
            terminal_receipt_id: record.terminal_receipt_id.clone(),
            terminal_receipt_hash: record.terminal_receipt_hash.clone(),
            terminal_outcome_hash: record.terminal_outcome_hash.clone(),
            terminal_evidence_hash: record.terminal_evidence_hash.clone(),
            terminal_status: record.terminal_status.clone(),
        })
    }

    pub(crate) fn mark_in_doubt(&self, key: &[u8], plan_hash: &str) -> Result<()> {
        validate_hex(plan_hash, "plan hash")?;
        self.update(key, |state| {
            let record = exact_record_mut(state, plan_hash)?;
            if matches!(
                record.phase,
                JournalPhase::Succeeded | JournalPhase::InDoubt
            ) {
                anyhow::bail!("operator mutation journal is already terminal");
            }
            record.phase = JournalPhase::InDoubt;
            Ok(())
        })
    }

    pub(crate) fn monotonic_state(&self, key: &[u8]) -> Result<OperatorMutationMonotonicState> {
        let state = self.read(key)?;
        let encoded =
            serde_json::to_vec(&state).context("encode operator mutation monotonic state")?;
        Ok(OperatorMutationMonotonicState {
            schema: JOURNAL_SCHEMA,
            journal_revision: state.revision,
            state_hash: OPERATOR_MUTATION_JOURNAL_ENGINE.content_hash(
                AuthenticationFraming::FramedDomain,
                JOURNAL_STATE_HASH_DOMAIN,
                &[&encoded],
            ),
        })
    }

    fn read(&self, key: &[u8]) -> Result<JournalState> {
        match read_private_state(&self.path, MAX_JOURNAL_BYTES)? {
            Some(bytes) => decode_and_verify(&bytes, key),
            None => empty_state(key),
        }
    }

    fn update(
        &self,
        key: &[u8],
        mutate: impl FnOnce(&mut JournalState) -> Result<()>,
    ) -> Result<()> {
        update_private_state_atomically(&self.path, MAX_JOURNAL_BYTES, STAGING_PREFIX, |current| {
            let mut state = match current {
                Some(bytes) => decode_and_verify(bytes, key)?,
                None => empty_state(key)?,
            };
            mutate(&mut state)?;
            state.schema = JOURNAL_SCHEMA.to_string();
            state.revision = state
                .revision
                .checked_add(1)
                .context("operator mutation journal revision exhausted")?;
            state.mac = sign_state(&state, key)?;
            let mut bytes =
                serde_json::to_vec(&state).context("encode operator mutation journal")?;
            bytes.push(b'\n');
            if bytes.len() as u64 > MAX_JOURNAL_BYTES {
                anyhow::bail!("operator mutation journal exceeds its bounded size");
            }
            Ok((bytes, ()))
        })
    }

    #[cfg(test)]
    pub(crate) fn compact_succeeded_for_test(&self, key: &[u8], retain: usize) -> Result<()> {
        self.update(key, |state| compact_succeeded_records(state, retain))
    }
}

impl JournalPhase {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Reserved => "reserved",
            Self::Planned => "planned",
            Self::Committing => "committing",
            Self::Succeeded => "succeeded",
            Self::InDoubt => "in_doubt",
        }
    }
}

fn empty_state(key: &[u8]) -> Result<JournalState> {
    let mut state = JournalState {
        schema: JOURNAL_SCHEMA.to_string(),
        revision: 0,
        checkpoint: JournalCheckpoint {
            compacted_records: 0,
            consumed_authorities: Vec::new(),
            history_hash: CHECKPOINT_GENESIS_HASH.to_string(),
        },
        records: Vec::new(),
        mac: String::new(),
    };
    state.mac = sign_state(&state, key)?;
    Ok(state)
}

fn decode_and_verify(bytes: &[u8], key: &[u8]) -> Result<JournalState> {
    let mut state: JournalState =
        serde_json::from_slice(bytes).context("decode operator mutation journal")?;
    if !matches!(
        state.schema.as_str(),
        JOURNAL_SCHEMA | LEGACY_JOURNAL_SCHEMA
    ) || OPERATOR_MUTATION_JOURNAL_ENGINE
        .validate_counts(
            state.records.len(),
            state.checkpoint.consumed_authorities.len(),
        )
        .is_err()
        || state.mac.len() != 64
    {
        anyhow::bail!("operator mutation journal schema or bounds are invalid");
    }
    let expected = if state.schema == LEGACY_JOURNAL_SCHEMA {
        sign_legacy_state(&state, key)?
    } else {
        sign_state(&state, key)?
    };
    decode_sha256_hex(&state.mac).context("decode operator mutation journal MAC")?;
    if !OPERATOR_MUTATION_JOURNAL_ENGINE.constant_time_equal(&state.mac, &expected) {
        anyhow::bail!("operator mutation journal MAC is invalid");
    }
    if state.schema == LEGACY_JOURNAL_SCHEMA {
        state.checkpoint = JournalCheckpoint {
            compacted_records: 0,
            consumed_authorities: Vec::new(),
            history_hash: CHECKPOINT_GENESIS_HASH.to_string(),
        };
    }
    for (index, record) in state.records.iter().enumerate() {
        validate_record(record)?;
        if state.records[..index].iter().any(|existing| {
            existing.mutation_id_hash == record.mutation_id_hash
                || existing.plan_hash == record.plan_hash
        }) {
            anyhow::bail!("operator mutation journal contains duplicate authority");
        }
    }
    validate_checkpoint(&state.checkpoint)?;
    for authority in &state.checkpoint.consumed_authorities {
        if state.records.iter().any(|record| {
            record.mutation_id_hash == authority.mutation_id_hash
                || record.plan_hash == authority.plan_hash
        }) {
            anyhow::bail!("operator mutation checkpoint overlaps live authority");
        }
    }
    Ok(state)
}

fn validate_checkpoint(checkpoint: &JournalCheckpoint) -> Result<()> {
    if checkpoint.compacted_records != checkpoint.consumed_authorities.len() as u64 {
        anyhow::bail!("operator mutation checkpoint count is inconsistent");
    }
    validate_content_hash(&checkpoint.history_hash, "checkpoint history hash")?;
    for (index, authority) in checkpoint.consumed_authorities.iter().enumerate() {
        validate_hex(&authority.mutation_id_hash, "checkpoint mutation id hash")?;
        validate_hex(&authority.plan_hash, "checkpoint plan hash")?;
        if checkpoint.consumed_authorities[..index]
            .iter()
            .any(|existing| {
                existing.mutation_id_hash == authority.mutation_id_hash
                    || existing.plan_hash == authority.plan_hash
            })
        {
            anyhow::bail!("operator mutation checkpoint contains duplicate authority");
        }
    }
    Ok(())
}

fn authority_consumed(state: &JournalState, mutation_id_hash: &str, plan_hash: &str) -> bool {
    state
        .records
        .iter()
        .any(|record| record.mutation_id_hash == mutation_id_hash || record.plan_hash == plan_hash)
        || state
            .checkpoint
            .consumed_authorities
            .iter()
            .any(|authority| {
                authority.mutation_id_hash == mutation_id_hash || authority.plan_hash == plan_hash
            })
}

fn removable_succeeded_records(state: &JournalState) -> usize {
    state
        .records
        .iter()
        .filter(|record| record.phase == JournalPhase::Succeeded)
        .count()
        .saturating_sub(RETAIN_SUCCEEDED_RECORDS)
}

fn compact_succeeded_records(state: &mut JournalState, retain: usize) -> Result<()> {
    let succeeded = state
        .records
        .iter()
        .filter(|record| record.phase == JournalPhase::Succeeded)
        .count();
    let mut remaining = succeeded.saturating_sub(retain);
    if remaining == 0 {
        return Ok(());
    }
    if state
        .checkpoint
        .consumed_authorities
        .len()
        .saturating_add(remaining)
        > MAX_CHECKPOINTED_AUTHORITIES
    {
        anyhow::bail!("operator mutation checkpoint authority limit reached");
    }
    let mut retained = Vec::with_capacity(state.records.len() - remaining);
    for record in state.records.drain(..) {
        if remaining > 0 && record.phase == JournalPhase::Succeeded {
            state.checkpoint.history_hash =
                checkpoint_history_hash(&state.checkpoint.history_hash, &record)?;
            state
                .checkpoint
                .consumed_authorities
                .push(ConsumedAuthority {
                    mutation_id_hash: record.mutation_id_hash,
                    plan_hash: record.plan_hash,
                });
            state.checkpoint.compacted_records = state
                .checkpoint
                .compacted_records
                .checked_add(1)
                .context("operator mutation checkpoint count exhausted")?;
            remaining -= 1;
        } else {
            retained.push(record);
        }
    }
    state.records = retained;
    Ok(())
}

fn checkpoint_history_hash(previous: &str, record: &JournalRecord) -> Result<String> {
    let encoded =
        serde_json::to_vec(record).context("encode compacted operator mutation record")?;
    Ok(OPERATOR_MUTATION_JOURNAL_ENGINE.content_hash(
        AuthenticationFraming::FramedDomain,
        CHECKPOINT_HASH_DOMAIN,
        &[previous.as_bytes(), &encoded],
    ))
}

fn validate_record(record: &JournalRecord) -> Result<()> {
    validate_hex(&record.mutation_id_hash, "mutation id hash")?;
    validate_hex(&record.plan_hash, "plan hash")?;
    validate_hex(&record.plan_request_binding_hash, "plan request binding")?;
    validate_hex(&record.session_binding_hash, "session binding")?;
    if let Some(binding) = &record.candidate_binding_hash {
        validate_content_hash(binding, "candidate binding")?;
    }
    if let Some(binding) = &record.commit_request_binding_hash {
        validate_hex(binding, "commit request binding")?;
    }
    if let Some(attempt_id) = &record.attempt_id {
        validate_binding(attempt_id, "attempt id")?;
    }
    if let Some(hash) = &record.effect_plan_hash {
        validate_content_hash(hash, "effect plan hash")?;
    }
    if let Some(hash) = &record.provider_effect_ack_hash {
        validate_content_hash(hash, "provider effect ACK hash")?;
    }
    if let Some(receipt_id) = &record.terminal_receipt_id {
        validate_binding(receipt_id, "terminal receipt id")?;
    }
    if let Some(hash) = &record.terminal_receipt_hash {
        validate_content_hash(hash, "terminal receipt hash")?;
    }
    if let Some(hash) = &record.terminal_outcome_hash {
        validate_content_hash(hash, "terminal outcome hash")?;
    }
    if let Some(hash) = &record.terminal_evidence_hash {
        validate_content_hash(hash, "terminal evidence hash")?;
    }
    if record
        .terminal_status
        .as_deref()
        .is_some_and(|status| status != "succeeded")
    {
        anyhow::bail!("operator mutation terminal status is invalid");
    }
    let linkage_complete = runtime_linkage_complete(record);
    let extended_linkage_present = extended_runtime_linkage_present(record);
    let linkage_present = runtime_linkage_present(record);
    if extended_linkage_present && !linkage_complete {
        anyhow::bail!("operator mutation runtime linkage is partial");
    }
    match record.phase {
        JournalPhase::Reserved => {
            if record.candidate_binding_hash.is_some()
                || record.commit_request_binding_hash.is_some()
                || linkage_present
            {
                anyhow::bail!("reserved operator mutation journal record is inconsistent");
            }
        }
        JournalPhase::Planned => {
            if record.candidate_binding_hash.is_none()
                || record.commit_request_binding_hash.is_some()
                || linkage_present
            {
                anyhow::bail!("planned operator mutation journal record is inconsistent");
            }
        }
        JournalPhase::Committing => {
            if record.candidate_binding_hash.is_none()
                || record.commit_request_binding_hash.is_none()
                || linkage_present
            {
                anyhow::bail!("committing operator mutation journal record is inconsistent");
            }
        }
        JournalPhase::Succeeded => {
            if record.candidate_binding_hash.is_none()
                || record.commit_request_binding_hash.is_none()
                || record.terminal_receipt_hash.is_none()
            {
                anyhow::bail!("succeeded operator mutation journal record is inconsistent");
            }
            if extended_linkage_present && !linkage_complete {
                anyhow::bail!("succeeded operator mutation runtime linkage is incomplete");
            }
        }
        JournalPhase::InDoubt => {
            if extended_linkage_present && !linkage_complete {
                anyhow::bail!("in-doubt operator mutation runtime linkage is incomplete");
            }
        }
    }
    Ok(())
}

fn validate_runtime_receipt(receipt: &RuntimeExecutionReceipt) -> Result<()> {
    validate_binding(&receipt.attempt_id, "attempt id")?;
    if !receipt.durable_intent_recorded || !receipt.effect_plan_recorded {
        anyhow::bail!("operator mutation runtime receipt lacks durable intent or effect plan");
    }
    validate_content_hash(
        receipt
            .effect_plan_hash
            .as_deref()
            .context("operator mutation runtime receipt lacks effect plan hash")?,
        "effect plan hash",
    )?;
    validate_content_hash(
        receipt
            .provider_effect_ack_hash
            .as_deref()
            .context("operator mutation runtime receipt lacks provider effect ACK hash")?,
        "provider effect ACK hash",
    )?;
    validate_binding(&receipt.terminal_receipt_id, "terminal receipt id")?;
    validate_content_hash(&receipt.terminal_receipt_hash, "terminal receipt hash")?;
    validate_content_hash(&receipt.terminal_outcome_hash, "terminal outcome hash")?;
    validate_content_hash(&receipt.terminal_evidence_hash, "terminal evidence hash")?;
    if receipt.terminal_status != "succeeded" {
        anyhow::bail!("operator mutation runtime receipt is not successful");
    }
    Ok(())
}

fn runtime_linkage_present(record: &JournalRecord) -> bool {
    [
        record.attempt_id.as_ref(),
        record.effect_plan_hash.as_ref(),
        record.provider_effect_ack_hash.as_ref(),
        record.terminal_receipt_id.as_ref(),
        record.terminal_receipt_hash.as_ref(),
        record.terminal_outcome_hash.as_ref(),
        record.terminal_evidence_hash.as_ref(),
        record.terminal_status.as_ref(),
    ]
    .into_iter()
    .any(|value| value.is_some())
}

fn extended_runtime_linkage_present(record: &JournalRecord) -> bool {
    [
        record.attempt_id.as_ref(),
        record.effect_plan_hash.as_ref(),
        record.provider_effect_ack_hash.as_ref(),
        record.terminal_receipt_id.as_ref(),
        record.terminal_outcome_hash.as_ref(),
        record.terminal_evidence_hash.as_ref(),
        record.terminal_status.as_ref(),
    ]
    .into_iter()
    .any(|value| value.is_some())
}

fn runtime_linkage_complete(record: &JournalRecord) -> bool {
    [
        record.attempt_id.as_ref(),
        record.effect_plan_hash.as_ref(),
        record.provider_effect_ack_hash.as_ref(),
        record.terminal_receipt_id.as_ref(),
        record.terminal_receipt_hash.as_ref(),
        record.terminal_outcome_hash.as_ref(),
        record.terminal_evidence_hash.as_ref(),
        record.terminal_status.as_ref(),
    ]
    .into_iter()
    .all(|value| value.is_some())
}

fn runtime_linkage_matches(record: &JournalRecord, receipt: &RuntimeExecutionReceipt) -> bool {
    record.attempt_id.as_deref() == Some(receipt.attempt_id.as_str())
        && record.effect_plan_hash == receipt.effect_plan_hash
        && record.provider_effect_ack_hash == receipt.provider_effect_ack_hash
        && record.terminal_receipt_id.as_deref() == Some(receipt.terminal_receipt_id.as_str())
        && record.terminal_receipt_hash.as_deref() == Some(receipt.terminal_receipt_hash.as_str())
        && record.terminal_outcome_hash.as_deref() == Some(receipt.terminal_outcome_hash.as_str())
        && record.terminal_evidence_hash.as_deref() == Some(receipt.terminal_evidence_hash.as_str())
        && record.terminal_status.as_deref() == Some(receipt.terminal_status.as_str())
}

fn exact_record_mut<'a>(
    state: &'a mut JournalState,
    plan_hash: &str,
) -> Result<&'a mut JournalRecord> {
    let matches = state
        .records
        .iter()
        .enumerate()
        .filter_map(|(index, record)| (record.plan_hash == plan_hash).then_some(index))
        .collect::<Vec<_>>();
    let [index] = matches.as_slice() else {
        anyhow::bail!("operator mutation journal plan is missing or ambiguous");
    };
    Ok(&mut state.records[*index])
}

fn sign_state(state: &JournalState, key: &[u8]) -> Result<String> {
    let unsigned = UnsignedJournalState {
        schema: &state.schema,
        revision: state.revision,
        checkpoint: &state.checkpoint,
        records: &state.records,
    };
    let encoded =
        serde_json::to_vec(&unsigned).context("encode operator mutation journal MAC payload")?;
    OPERATOR_MUTATION_JOURNAL_ENGINE
        .mac_hex(
            key,
            AuthenticationFraming::FramedDomain,
            JOURNAL_MAC_DOMAIN,
            &[&encoded],
        )
        .context("initialize operator mutation journal HMAC")
}

fn sign_legacy_state(state: &JournalState, key: &[u8]) -> Result<String> {
    #[derive(Serialize)]
    struct LegacyUnsignedJournalState<'a> {
        schema: &'a str,
        revision: u64,
        records: &'a [JournalRecord],
    }
    let encoded = serde_json::to_vec(&LegacyUnsignedJournalState {
        schema: &state.schema,
        revision: state.revision,
        records: &state.records,
    })
    .context("encode legacy operator mutation journal MAC payload")?;
    OPERATOR_MUTATION_JOURNAL_ENGINE
        .mac_hex(
            key,
            AuthenticationFraming::FramedDomain,
            JOURNAL_MAC_DOMAIN,
            &[&encoded],
        )
        .context("initialize legacy operator mutation journal HMAC")
}

fn validate_hex(value: &str, name: &str) -> Result<()> {
    if decode_sha256_hex(value).is_ok() {
        return Ok(());
    }
    anyhow::bail!("{name} must be canonical lowercase SHA-256 hex")
}

fn validate_binding(value: &str, name: &str) -> Result<()> {
    if !value.is_empty()
        && value.len() <= 256
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':' | b'.'))
    {
        return Ok(());
    }
    anyhow::bail!("{name} is not a canonical binding")
}

fn validate_content_hash(value: &str, name: &str) -> Result<()> {
    let digest = value
        .strip_prefix("sha256:")
        .with_context(|| format!("{name} must use the sha256 content-hash domain"))?;
    validate_hex(digest, name)
}

#[cfg(all(test, unix))]
#[path = "../tests/unit/operator_mutation_journal.rs"]
mod tests;

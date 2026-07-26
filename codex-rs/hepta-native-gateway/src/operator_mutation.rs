//! Operator-authenticated product mutation for bounded local notes.
//!
//! Planning creates only an exact RuntimeKernel approval candidate. A separate
//! HMAC domain authorizes commit of that exact candidate; only then may the
//! RuntimeKernel record durable intent, dispatch the fixed-path write, persist
//! the provider effect ACK, and publish a terminal receipt.

use std::env;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use hepta_core::ApprovalRequirement;
use hepta_runtime::EffectBroker;
use hepta_runtime::EffectPlan;
use hepta_runtime::ExactExecutionAuthority;
use hepta_runtime::ExecutionAdmission;
use hepta_runtime::ExecutionIngress;
use hepta_runtime::ProviderEffectAck;
use hepta_runtime::RuntimeExecutionReceipt;
use hepta_runtime::RuntimeKernel;
use hepta_runtime::TerminalEffectReceipt;
use hmac::Hmac;
use hmac::Mac;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use zeroize::Zeroizing;

use crate::operator_mutation_journal::OperatorMutationJournal;
use crate::operator_mutation_journal::OperatorMutationMonotonicState;
use crate::secure_key_file::read_private_key;

pub(crate) const OPERATOR_MUTATION_PLAN_ENDPOINT: &str = "/api/v2/operator-mutations/note/plan";
pub(crate) const OPERATOR_MUTATION_COMMIT_ENDPOINT: &str = "/api/v2/operator-mutations/note/commit";
pub(crate) const OPERATOR_MUTATION_ENABLED_ENV: &str = "HEPTA_OPERATOR_MUTATION_ENABLED";
const OPERATOR_MUTATION_KEY_FILE_ENV: &str = "HEPTA_OPERATOR_MUTATION_AUTH_KEY_FILE";
const MAX_NOTE_BYTES: usize = 8 * 1024;
const PLAN_PROOF_DOMAIN: &[u8] = b"hepta.native.operator-note.plan.v1";
const COMMIT_PROOF_DOMAIN: &[u8] = b"hepta.native.operator-note.commit.v1";
const PLAN_HASH_DOMAIN: &[u8] = b"hepta.native.operator-note.plan-hash.v1";

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct OperatorMutationPlanRequest {
    mutation_id: String,
    note: String,
    proof: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct OperatorMutationCommitRequest {
    mutation_id: String,
    note: String,
    plan_hash: String,
    candidate_binding_hash: String,
    plan_request_binding_hash: String,
    session_binding_hash: String,
    proof: String,
}

#[derive(Serialize)]
struct OperatorNoteArtifact<'a> {
    schema: &'static str,
    mutation_id: &'a str,
    note: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct OperatorMutationPlanReceipt {
    schema: &'static str,
    status: &'static str,
    action: &'static str,
    request_binding_hash: String,
    session_binding_hash: String,
    plan_hash: String,
    candidate_binding_hash: String,
    target_class: &'static str,
    authority: &'static str,
    mutation_authorized: bool,
    durable_intent_recorded: bool,
    provider_effect_ack_recorded: bool,
    terminal_receipt_recorded: bool,
    filesystem_mutated: bool,
    arbitrary_path_accepted: bool,
    arbitrary_content_accepted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct OperatorMutationCommitReceipt {
    schema: &'static str,
    status: &'static str,
    action: &'static str,
    plan_request_binding_hash: String,
    commit_request_binding_hash: String,
    session_binding_hash: String,
    plan_hash: String,
    candidate_binding_hash: String,
    mutation_id_hash: String,
    target_class: &'static str,
    invoked_tool: String,
    authority: &'static str,
    execution_receipt: RuntimeExecutionReceipt,
    external_network_requested: bool,
    arbitrary_path_accepted: bool,
    arbitrary_content_accepted: bool,
}

pub(crate) fn enabled() -> bool {
    env::var(OPERATOR_MUTATION_ENABLED_ENV)
        .ok()
        .is_some_and(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
}

pub(crate) fn monotonic_state() -> Result<Option<OperatorMutationMonotonicState>> {
    if !enabled() {
        return Ok(None);
    }
    let authority = load_authority()?;
    authority
        .journal
        .monotonic_state(authority.key.as_ref())
        .map(Some)
}

pub(crate) fn prevalidate_plan(
    body: Option<&str>,
    request_binding_hash: &str,
    session_binding_hash: &str,
) -> Result<()> {
    let authority = load_authority()?;
    let (mutation_id_hash, plan_hash) = prevalidate_plan_with_key(
        body,
        request_binding_hash,
        session_binding_hash,
        authority.key.as_ref(),
    )?;
    authority
        .journal
        .validate_reservable(authority.key.as_ref(), &mutation_id_hash, &plan_hash)
}

pub(crate) fn prevalidate_commit(
    body: Option<&str>,
    commit_request_binding_hash: &str,
    current_session_binding_hash: &str,
) -> Result<()> {
    let authority = load_authority()?;
    let request = prevalidate_commit_with_key(
        body,
        commit_request_binding_hash,
        current_session_binding_hash,
        authority.key.as_ref(),
    )?;
    authority.journal.validate_committable(
        authority.key.as_ref(),
        &mutation_id_hash(&request.mutation_id),
        &request.plan_hash,
        &request.plan_request_binding_hash,
        &request.session_binding_hash,
        &request.candidate_binding_hash,
    )
}

pub(crate) fn plan(
    kernel: &RuntimeKernel,
    body: Option<&str>,
    request_binding_hash: &str,
    session_binding_hash: &str,
) -> Result<OperatorMutationPlanReceipt> {
    let authority = load_authority()?;
    plan_with_key(
        kernel,
        body,
        request_binding_hash,
        session_binding_hash,
        authority.key.as_ref(),
        &authority.journal,
    )
}

pub(crate) fn commit(
    kernel: &RuntimeKernel,
    body: Option<&str>,
    commit_request_binding_hash: &str,
    current_session_binding_hash: &str,
) -> Result<OperatorMutationCommitReceipt> {
    let authority = load_authority()?;
    commit_with_key(
        kernel,
        body,
        commit_request_binding_hash,
        current_session_binding_hash,
        authority.key.as_ref(),
        &authority.journal,
    )
}

fn plan_with_key(
    kernel: &RuntimeKernel,
    body: Option<&str>,
    request_binding_hash: &str,
    session_binding_hash: &str,
    key: &[u8],
    journal: &OperatorMutationJournal,
) -> Result<OperatorMutationPlanReceipt> {
    canonical_binding(request_binding_hash, "request binding")?;
    canonical_binding(session_binding_hash, "session binding")?;
    let request: OperatorMutationPlanRequest =
        serde_json::from_str(body.context("operator mutation plan body is required")?)
            .context("decode operator mutation plan body")?;
    validate_mutation(&request.mutation_id, &request.note)?;
    verify_proof(
        key,
        PLAN_PROOF_DOMAIN,
        &[
            &request.mutation_id,
            &request.note,
            request_binding_hash,
            session_binding_hash,
        ],
        &request.proof,
    )?;
    let prepared = PreparedOperatorMutation::new(
        &request.mutation_id,
        &request.note,
        request_binding_hash,
        session_binding_hash,
    )?;
    let mutation_id_hash = mutation_id_hash(&request.mutation_id);
    journal.reserve_plan(
        key,
        &mutation_id_hash,
        &prepared.plan_hash,
        request_binding_hash,
        session_binding_hash,
    )?;
    let candidate_binding_hash = match prepare_and_select_exact_candidate(kernel, &prepared) {
        Ok(candidate_binding_hash) => candidate_binding_hash,
        Err(error) => {
            return Err(fail_plan_closed(journal, key, &prepared.plan_hash, error));
        }
    };
    if let Err(error) = journal.publish_candidate(key, &prepared.plan_hash, &candidate_binding_hash)
    {
        return Err(fail_plan_closed(journal, key, &prepared.plan_hash, error));
    }
    Ok(OperatorMutationPlanReceipt {
        schema: "hepta.native.operator-note-mutation.v1",
        status: "approval_required",
        action: "operator-note-write-plan",
        request_binding_hash: request_binding_hash.to_string(),
        session_binding_hash: session_binding_hash.to_string(),
        plan_hash: prepared.plan_hash,
        candidate_binding_hash,
        target_class: "fixed_local_operator_note",
        authority: "operator_hmac_plan_and_runtime_exact_candidate",
        mutation_authorized: false,
        durable_intent_recorded: false,
        provider_effect_ack_recorded: false,
        terminal_receipt_recorded: false,
        filesystem_mutated: false,
        arbitrary_path_accepted: false,
        arbitrary_content_accepted: true,
    })
}

fn commit_with_key(
    kernel: &RuntimeKernel,
    body: Option<&str>,
    commit_request_binding_hash: &str,
    current_session_binding_hash: &str,
    key: &[u8],
    journal: &OperatorMutationJournal,
) -> Result<OperatorMutationCommitReceipt> {
    canonical_binding(commit_request_binding_hash, "commit request binding")?;
    canonical_binding(current_session_binding_hash, "current session binding")?;
    let request: OperatorMutationCommitRequest =
        serde_json::from_str(body.context("operator mutation commit body is required")?)
            .context("decode operator mutation commit body")?;
    validate_mutation(&request.mutation_id, &request.note)?;
    canonical_binding(&request.plan_hash, "plan binding")?;
    canonical_content_hash(&request.candidate_binding_hash, "candidate binding")?;
    canonical_binding(&request.plan_request_binding_hash, "plan request binding")?;
    canonical_binding(&request.session_binding_hash, "session binding")?;
    if request.session_binding_hash != current_session_binding_hash {
        anyhow::bail!("operator mutation plan became stale before commit");
    }
    verify_proof(
        key,
        COMMIT_PROOF_DOMAIN,
        &[
            &request.mutation_id,
            &request.note,
            &request.plan_hash,
            &request.candidate_binding_hash,
            &request.plan_request_binding_hash,
            &request.session_binding_hash,
            commit_request_binding_hash,
        ],
        &request.proof,
    )?;
    let prepared = PreparedOperatorMutation::new(
        &request.mutation_id,
        &request.note,
        &request.plan_request_binding_hash,
        &request.session_binding_hash,
    )?;
    if prepared.plan_hash != request.plan_hash {
        anyhow::bail!("operator mutation commit does not match its canonical plan");
    }
    let mutation_id_hash = mutation_id_hash(&request.mutation_id);
    journal.begin_commit(
        key,
        &mutation_id_hash,
        &request.plan_hash,
        &request.plan_request_binding_hash,
        &request.session_binding_hash,
        &request.candidate_binding_hash,
        commit_request_binding_hash,
    )?;
    let executor = tokio::runtime::Builder::new_current_thread()
        .build()
        .context("build operator mutation executor")?;
    let execution =
        (|| -> Result<(hepta_runtime::VerticalSliceResult, RuntimeExecutionReceipt)> {
            let (mut effect_broker, effect_plan_hash) = effect_broker_for_commit(
                &request,
                &prepared,
                commit_request_binding_hash,
                &mutation_id_hash,
            )?;
            let result = executor
                .block_on(kernel.approve_candidate_and_run_demo_turn_in_session(
                    &prepared.session_id,
                    &request.candidate_binding_hash,
                    &prepared.runtime_instruction,
                ))
                .map_err(|error| {
                    anyhow::anyhow!("execute operator mutation through RuntimeKernel: {error}")
                })?;
            let receipt = result
                .execution_receipt
                .clone()
                .context("operator mutation completed without an execution receipt")?;
            if result.invoked_tool.as_deref() != Some("write_file")
                || !receipt.durable_intent_recorded
                || !receipt.effect_plan_recorded
                || receipt.effect_plan_hash.is_none()
                || receipt.provider_effect_ack_hash.is_none()
                || receipt.terminal_receipt_id.is_empty()
                || receipt.terminal_receipt_hash.is_empty()
                || receipt.terminal_outcome_hash.is_empty()
                || receipt.terminal_evidence_hash.is_empty()
                || receipt.terminal_status != "succeeded"
            {
                anyhow::bail!("operator mutation lifecycle evidence failed closed");
            }
            complete_effect_broker(&mut effect_broker, &effect_plan_hash, &receipt)?;
            Ok((result, receipt))
        })();
    let (result, receipt) = match execution {
        Ok(execution) => execution,
        Err(error) => {
            return Err(fail_plan_closed(journal, key, &prepared.plan_hash, error));
        }
    };
    debug_assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
    if let Err(error) = journal.record_runtime_linkage(key, &prepared.plan_hash, &receipt) {
        return Err(fail_plan_closed(
            journal,
            key,
            &prepared.plan_hash,
            error.context("persist operator mutation RuntimeKernel linkage"),
        ));
    }
    journal
        .finalize_linked_success(key, &prepared.plan_hash, &receipt)
        .context("durably finalize one-shot operator mutation")?;
    Ok(OperatorMutationCommitReceipt {
        schema: "hepta.native.operator-note-mutation.v1",
        status: "succeeded",
        action: "operator-note-write-commit",
        plan_request_binding_hash: request.plan_request_binding_hash,
        commit_request_binding_hash: commit_request_binding_hash.to_string(),
        session_binding_hash: request.session_binding_hash,
        plan_hash: request.plan_hash,
        candidate_binding_hash: request.candidate_binding_hash,
        mutation_id_hash: format!("sha256:{mutation_id_hash}"),
        target_class: "fixed_local_operator_note",
        invoked_tool: "write_file".to_string(),
        authority: "operator_hmac_commit_and_runtime_one_shot_exact_candidate",
        execution_receipt: receipt,
        external_network_requested: false,
        arbitrary_path_accepted: false,
        arbitrary_content_accepted: true,
    })
}

struct PreparedOperatorMutation {
    plan_hash: String,
    session_id: String,
    runtime_instruction: String,
    target: String,
    payload_digest: String,
}

impl PreparedOperatorMutation {
    fn new(
        mutation_id: &str,
        note: &str,
        request_binding_hash: &str,
        session_binding_hash: &str,
    ) -> Result<Self> {
        canonical_binding(request_binding_hash, "request binding")?;
        canonical_binding(session_binding_hash, "session binding")?;
        let artifact = serde_json::to_string(&OperatorNoteArtifact {
            schema: "hepta.operator-note.v1",
            mutation_id,
            note,
        })
        .context("encode canonical operator note")?;
        let target = format!("artifacts/.hepta-operator-note-{mutation_id}.json");
        let payload_digest = format!("sha256:{:x}", Sha256::digest(artifact.as_bytes()));
        let plan_hash = digest(
            PLAN_HASH_DOMAIN,
            &[
                request_binding_hash,
                session_binding_hash,
                mutation_id,
                &target,
                &artifact,
            ],
        );
        Ok(Self {
            session_id: format!("native-gateway:operator-note:{plan_hash}"),
            runtime_instruction: format!("overwrite:{target} => {artifact}"),
            plan_hash,
            target,
            payload_digest,
        })
    }
}

fn effect_broker_for_commit(
    request: &OperatorMutationCommitRequest,
    prepared: &PreparedOperatorMutation,
    commit_request_binding_hash: &str,
    mutation_id_hash: &str,
) -> Result<(EffectBroker, String)> {
    let authority = ExactExecutionAuthority::new(
        commit_request_binding_hash,
        &request.plan_request_binding_hash,
        &request.session_binding_hash,
    )
    .context("bind exact operator mutation execution authority")?;
    let admission = ExecutionAdmission::new(
        ExecutionIngress::NativeGateway,
        "operator-note-write",
        authority,
        &request.plan_hash,
        &request.candidate_binding_hash,
    )
    .context("admit operator mutation execution")?;
    let effect_plan = EffectPlan::new(
        admission.admission_hash(),
        "write_file",
        &prepared.target,
        &prepared.payload_digest,
        mutation_id_hash,
    )
    .context("record operator mutation effect plan")?;
    let effect_plan_hash = effect_plan.effect_plan_hash().to_string();
    let mut broker = EffectBroker::admit(admission);
    broker
        .record_effect_plan(effect_plan)
        .context("publish operator mutation effect plan before dispatch")?;
    Ok((broker, effect_plan_hash))
}

fn complete_effect_broker(
    broker: &mut EffectBroker,
    effect_plan_hash: &str,
    receipt: &RuntimeExecutionReceipt,
) -> Result<()> {
    let provider_receipt_hash = receipt
        .provider_effect_ack_hash
        .as_deref()
        .context("operator mutation provider ACK hash is missing")?;
    let provider_ack = ProviderEffectAck::new(
        effect_plan_hash,
        "runtime-kernel:write_file",
        provider_receipt_hash,
    )
    .context("bind operator mutation provider ACK")?;
    let provider_ack_hash = provider_ack.ack_hash().to_string();
    broker
        .record_provider_ack(provider_ack)
        .context("record operator mutation provider ACK")?;
    let terminal_receipt = TerminalEffectReceipt::terminal(
        provider_ack_hash,
        "succeeded",
        &receipt.terminal_receipt_hash,
        &receipt.terminal_evidence_hash,
    )
    .context("bind operator mutation terminal receipt")?;
    broker
        .record_terminal_receipt(terminal_receipt)
        .context("record operator mutation terminal receipt")?;
    broker
        .completed_receipt_hash()
        .context("operator mutation effect broker did not reach terminal state")?;
    Ok(())
}

fn prepare_exact_candidate(
    kernel: &RuntimeKernel,
    prepared: &PreparedOperatorMutation,
) -> Result<()> {
    kernel
        .switch_model_in_session(&prepared.session_id, "demo/demo-chat")
        .map_err(|error| anyhow::anyhow!("select isolated operator mutation model: {error}"))?;
    kernel
        .add_policy_rule(
            Some(&prepared.session_id),
            Some("demo"),
            Some("write_file"),
            None,
            ApprovalRequirement::Ask,
            Some("operator-authenticated bounded product mutation"),
        )
        .map_err(|error| {
            anyhow::anyhow!("install operator mutation approval requirement: {error}")
        })?;
    let executor = tokio::runtime::Builder::new_current_thread()
        .build()
        .context("build operator mutation planning executor")?;
    let result = executor
        .block_on(
            kernel.run_demo_turn_in_session(&prepared.session_id, &prepared.runtime_instruction),
        )
        .map_err(|error| {
            anyhow::anyhow!("plan operator mutation through RuntimeKernel: {error}")
        })?;
    if result.invoked_tool.is_some()
        || result.approval_required.as_deref() != Some("write_file")
        || result.execution_receipt.is_some()
    {
        anyhow::bail!("operator mutation planning failed to stop before provider dispatch");
    }
    Ok(())
}

fn prepare_and_select_exact_candidate(
    kernel: &RuntimeKernel,
    prepared: &PreparedOperatorMutation,
) -> Result<String> {
    prepare_exact_candidate(kernel, prepared)?;
    let pending = kernel
        .approval_snapshot_for_session(&prepared.session_id)
        .map_err(|error| anyhow::anyhow!("inspect operator mutation candidate: {error}"))?
        .pending
        .into_iter()
        .filter(|candidate| candidate.tool_name == "write_file")
        .filter_map(|candidate| candidate.candidate_binding_hash)
        .collect::<Vec<_>>();
    let [candidate_binding_hash] = pending.as_slice() else {
        anyhow::bail!("operator mutation plan did not produce exactly one write candidate");
    };
    canonical_content_hash(candidate_binding_hash, "candidate binding")?;
    Ok(candidate_binding_hash.clone())
}

pub(crate) struct LoadedOperatorAuthority {
    pub(crate) key: Zeroizing<[u8; 32]>,
    pub(crate) journal: OperatorMutationJournal,
}

pub(crate) fn load_authority() -> Result<LoadedOperatorAuthority> {
    let key_file = env::var_os(OPERATOR_MUTATION_KEY_FILE_ENV)
        .map(PathBuf::from)
        .context("HEPTA_OPERATOR_MUTATION_AUTH_KEY_FILE is required when mutation is enabled")?;
    let key = read_private_key(
        &key_file,
        OPERATOR_MUTATION_KEY_FILE_ENV,
        "operator mutation authentication",
    )?;
    let journal = OperatorMutationJournal::for_key_file(&key_file)?;
    Ok(LoadedOperatorAuthority { key, journal })
}

fn mutation_id_hash(mutation_id: &str) -> String {
    format!("{:x}", Sha256::digest(mutation_id.as_bytes()))
}

fn fail_plan_closed(
    journal: &OperatorMutationJournal,
    key: &[u8],
    plan_hash: &str,
    original: anyhow::Error,
) -> anyhow::Error {
    match journal.mark_in_doubt(key, plan_hash) {
        Ok(()) => original,
        Err(journal_error) => anyhow::anyhow!(
            "{original:#}; additionally failed to persist in-doubt operator mutation: {journal_error:#}"
        ),
    }
}

fn validate_mutation(mutation_id: &str, note: &str) -> Result<()> {
    canonical_binding(mutation_id, "mutation id")?;
    if note.is_empty() || note.len() > MAX_NOTE_BYTES {
        anyhow::bail!("operator note must contain 1..={MAX_NOTE_BYTES} UTF-8 bytes");
    }
    Ok(())
}

fn prevalidate_plan_with_key(
    body: Option<&str>,
    request_binding_hash: &str,
    session_binding_hash: &str,
    key: &[u8],
) -> Result<(String, String)> {
    canonical_binding(request_binding_hash, "request binding")?;
    canonical_binding(session_binding_hash, "session binding")?;
    let request: OperatorMutationPlanRequest =
        serde_json::from_str(body.context("operator mutation plan body is required")?)
            .context("decode operator mutation plan body")?;
    validate_mutation(&request.mutation_id, &request.note)?;
    verify_proof(
        key,
        PLAN_PROOF_DOMAIN,
        &[
            &request.mutation_id,
            &request.note,
            request_binding_hash,
            session_binding_hash,
        ],
        &request.proof,
    )?;
    let prepared = PreparedOperatorMutation::new(
        &request.mutation_id,
        &request.note,
        request_binding_hash,
        session_binding_hash,
    )?;
    Ok((mutation_id_hash(&request.mutation_id), prepared.plan_hash))
}

fn prevalidate_commit_with_key(
    body: Option<&str>,
    commit_request_binding_hash: &str,
    current_session_binding_hash: &str,
    key: &[u8],
) -> Result<OperatorMutationCommitRequest> {
    canonical_binding(commit_request_binding_hash, "commit request binding")?;
    canonical_binding(current_session_binding_hash, "current session binding")?;
    let request: OperatorMutationCommitRequest =
        serde_json::from_str(body.context("operator mutation commit body is required")?)
            .context("decode operator mutation commit body")?;
    validate_mutation(&request.mutation_id, &request.note)?;
    canonical_binding(&request.plan_hash, "plan binding")?;
    canonical_content_hash(&request.candidate_binding_hash, "candidate binding")?;
    canonical_binding(&request.plan_request_binding_hash, "plan request binding")?;
    canonical_binding(&request.session_binding_hash, "session binding")?;
    if request.session_binding_hash != current_session_binding_hash {
        anyhow::bail!("operator mutation plan became stale before commit");
    }
    verify_proof(
        key,
        COMMIT_PROOF_DOMAIN,
        &[
            &request.mutation_id,
            &request.note,
            &request.plan_hash,
            &request.candidate_binding_hash,
            &request.plan_request_binding_hash,
            &request.session_binding_hash,
            commit_request_binding_hash,
        ],
        &request.proof,
    )?;
    let prepared = PreparedOperatorMutation::new(
        &request.mutation_id,
        &request.note,
        &request.plan_request_binding_hash,
        &request.session_binding_hash,
    )?;
    if prepared.plan_hash != request.plan_hash {
        anyhow::bail!("operator mutation commit does not match its canonical plan");
    }
    Ok(request)
}

fn canonical_binding(value: &str, name: &str) -> Result<()> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Ok(());
    }
    anyhow::bail!("{name} must be canonical lowercase SHA-256 hex")
}

fn canonical_content_hash(value: &str, name: &str) -> Result<()> {
    let digest = value
        .strip_prefix("sha256:")
        .with_context(|| format!("{name} must use the sha256 content-hash domain"))?;
    canonical_binding(digest, name)
}

fn verify_proof(key: &[u8], domain: &[u8], fields: &[&str], proof: &str) -> Result<()> {
    let proof = decode_hex(proof).context("operator mutation proof encoding is invalid")?;
    let mut mac = HmacSha256::new_from_slice(key).context("initialize operator mutation HMAC")?;
    update_field(&mut mac, domain);
    for field in fields {
        update_field(&mut mac, field.as_bytes());
    }
    mac.verify_slice(&proof)
        .map_err(|_| anyhow::anyhow!("operator mutation proof is invalid"))
}

fn digest(domain: &[u8], fields: &[&str]) -> String {
    let mut hasher = Sha256::new();
    hasher.update((domain.len() as u64).to_be_bytes());
    hasher.update(domain);
    for field in fields {
        hasher.update((field.len() as u64).to_be_bytes());
        hasher.update(field.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

fn update_field(mac: &mut HmacSha256, field: &[u8]) {
    mac.update(&(field.len() as u64).to_be_bytes());
    mac.update(field);
}

fn decode_hex(value: &str) -> Result<[u8; 32]> {
    canonical_binding(value, "proof")?;
    let mut decoded = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        decoded[index] = (hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?;
    }
    Ok(decoded)
}

fn hex_nibble(value: u8) -> Result<u8> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => anyhow::bail!("non-canonical hexadecimal value"),
    }
}

#[cfg(test)]
fn proof(key: &[u8], domain: &[u8], fields: &[&str]) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("test HMAC key");
    update_field(&mut mac, domain);
    for field in fields {
        update_field(&mut mac, field.as_bytes());
    }
    format!("{:x}", mac.finalize().into_bytes())
}

#[cfg(test)]
#[path = "../tests/unit/operator_mutation.rs"]
mod tests;

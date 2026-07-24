use super::*;

use crate::DurableOutcomeStore;
use crate::ExecutionEffectAck;
use crate::ExecutionIntent;
use crate::ExecutionIntentParts;
use crate::ExecutionIntentResolveResult;
use crate::ExecutionIntentStageResult;
use crate::OutcomeIntentState;
use crate::SyncDurableOutcomeWriter;
use crate::candidate_reference_hash;
use hepta_contracts::AuthorizationRef;
use hepta_contracts::CandidateRef;
use hepta_contracts::OutcomeReceiptParts;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;

#[tokio::test]
async fn pre_dispatch_intent_survives_restart_and_blocks_another_attempt() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-execution-intent.sqlite3");
    let first = execution_intent("attempt-execution-a", "correlation-a")?;

    {
        let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
        assert_eq!(
            store.stage_execution_intent(first.clone()).await?,
            ExecutionIntentStageResult::Staged
        );
        assert_eq!(
            store.stage_execution_intent(first.clone()).await?,
            ExecutionIntentStageResult::AlreadyStaged
        );
    }

    let reopened = DurableOutcomeStore::open_existing(&database_path).await?;
    assert_eq!(
        reopened
            .pending_execution_intent(first.attempt_id())
            .await?
            .as_ref(),
        Some(&first)
    );
    let second = execution_intent("attempt-execution-b", "correlation-b")?;
    assert_eq!(
        reopened
            .stage_execution_intent(second)
            .await
            .expect_err("one unresolved intent must block another provider attempt"),
        OutcomeStoreError::ExecutionIntentOutstanding {
            existing_attempt: first.attempt_id().to_owned(),
            attempted_attempt: "attempt-execution-b".into(),
        }
    );
    Ok(())
}

#[tokio::test]
async fn terminal_commit_before_resolution_recovers_as_in_doubt_then_resolves() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-execution-terminal.sqlite3");
    let intent = execution_intent("attempt-execution-terminal", "correlation-terminal")?;
    let (receipt, envelope, evidence) = canonical_execution_outcome(&intent)?;

    {
        let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
        store.stage_execution_intent(intent.clone()).await?;
        assert_eq!(
            store
                .record(
                    intent.attempt_id(),
                    receipt.clone(),
                    envelope.clone(),
                    evidence.clone(),
                )
                .await?,
            OutcomeRecordResult::Recorded
        );
    }

    let reopened = DurableOutcomeStore::open_existing(&database_path).await?;
    assert_eq!(
        reopened
            .pending_execution_intent(intent.attempt_id())
            .await?
            .as_ref(),
        Some(&intent)
    );
    assert!(
        reopened
            .read_by_attempt(intent.attempt_id())
            .await?
            .is_some()
    );
    assert_eq!(
        reopened
            .resolve_execution_intent(intent.attempt_id(), intent.idempotency_key())
            .await?,
        ExecutionIntentResolveResult::Resolved
    );
    assert!(reopened.pending_execution_intents().await?.is_empty());
    Ok(())
}

#[tokio::test]
async fn execution_intent_cannot_resolve_before_terminal_outcome() -> TestResult {
    let directory = tempfile::tempdir()?;
    let store = DurableOutcomeStore::bootstrap_new(
        directory.path().join("v2-execution-no-terminal.sqlite3"),
    )
    .await?;
    let intent = execution_intent("attempt-execution-no-terminal", "correlation-no-terminal")?;
    store.stage_execution_intent(intent.clone()).await?;

    assert_eq!(
        store
            .resolve_execution_intent(intent.attempt_id(), intent.idempotency_key())
            .await
            .expect_err("planned execution cannot be cleared without terminal evidence"),
        OutcomeStoreError::ExecutionIntentOutcomeMissing {
            attempt_id: intent.attempt_id().to_owned(),
        }
    );
    assert_eq!(store.pending_execution_intents().await?, vec![intent],);
    Ok(())
}

#[tokio::test]
async fn terminal_evidence_drift_cannot_clear_execution_intent() -> TestResult {
    let directory = tempfile::tempdir()?;
    let store = DurableOutcomeStore::bootstrap_new(
        directory.path().join("v2-execution-evidence-drift.sqlite3"),
    )
    .await?;
    let intent = execution_intent("attempt-execution-drift", "correlation-drift")?;
    let (receipt, canonical_evidence, evidence_hash) = canonical_execution_outcome(&intent)?;
    let evidence = canonical_evidence.replace("correlation-drift", "correlation-substituted");
    store.stage_execution_intent(intent.clone()).await?;
    store
        .stage_intent(
            intent.attempt_id(),
            receipt.clone(),
            evidence.clone(),
            evidence_hash.clone(),
        )
        .await?;

    assert!(matches!(
        store
            .commit_staged_intent_and_resolve_execution(
                intent.attempt_id().to_owned(),
                receipt,
                evidence,
                evidence_hash,
                intent.idempotency_key().to_owned(),
            )
            .await,
        Err(OutcomeStoreError::Corrupt { detail })
            if detail.contains("canonical evidence hash")
    ));
    assert!(
        store
            .pending_execution_intent(intent.attempt_id())
            .await?
            .is_some()
    );
    assert!(store.read_by_attempt(intent.attempt_id()).await?.is_none());
    assert_eq!(
        store
            .pending_intent(intent.attempt_id())
            .await?
            .expect("exact terminal material remains staged")
            .state(),
        OutcomeIntentState::Pending
    );
    Ok(())
}

#[tokio::test]
async fn terminal_evidence_schema_is_strict_and_byte_canonical() -> TestResult {
    let intent = execution_intent("attempt-execution-schema", "correlation-schema")?;
    let (receipt, canonical, evidence_hash) = canonical_execution_outcome(&intent)?;
    let parsed: Value = serde_json::from_str(&canonical)?;
    let mut cases = Vec::new();

    let mut reordered = parsed.clone();
    reordered
        .get_mut("fields")
        .and_then(Value::as_array_mut)
        .expect("fields")
        .swap(0, 1);
    cases.push(("reordered", serde_json::to_string(&reordered)?));

    let mut extra = parsed.clone();
    extra
        .get_mut("fields")
        .and_then(Value::as_array_mut)
        .expect("fields")
        .push(serde_json::json!(["unexpected", "value"]));
    cases.push(("extra-field", serde_json::to_string(&extra)?));

    let mut missing = parsed.clone();
    missing
        .get_mut("fields")
        .and_then(Value::as_array_mut)
        .expect("fields")
        .pop();
    cases.push(("missing-field", serde_json::to_string(&missing)?));

    let mut wrong_type = parsed.clone();
    wrong_type
        .get_mut("fields")
        .and_then(Value::as_array_mut)
        .expect("fields")[6][1] = Value::String("1".into());
    cases.push(("wrong-type", serde_json::to_string(&wrong_type)?));

    let mut top_level_extra = parsed;
    top_level_extra
        .as_object_mut()
        .expect("object")
        .insert("unexpected".into(), Value::Bool(true));
    cases.push(("top-level-extra", serde_json::to_string(&top_level_extra)?));
    cases.push(("non-canonical-whitespace", format!(" {canonical}")));

    for (label, envelope) in cases {
        let error =
            commit_terminal_fixture(&intent, receipt.clone(), envelope, evidence_hash.clone())
                .await
                .expect_err(label);
        assert!(
            matches!(error, OutcomeStoreError::Corrupt { .. }),
            "{label}: {error}"
        );
    }
    Ok(())
}

#[tokio::test]
async fn receipt_identity_hash_outcome_and_status_drift_are_rejected() -> TestResult {
    let intent = execution_intent("attempt-execution-receipt", "correlation-receipt")?;
    let (receipt, canonical, evidence_hash) = canonical_execution_outcome(&intent)?;
    let parts = receipt.rehydration_parts();
    let cases = [
        OutcomeReceipt::try_rehydrate(OutcomeReceiptParts::new(
            ReceiptId::new("receipt:sha256:substituted"),
            parts.receipt_hash().clone(),
            parts.candidate().clone(),
            parts.authorization().clone(),
            parts.payload_set_hash().clone(),
            parts.executed_by().clone(),
            parts.outcome_hash().clone(),
            parts.status().clone(),
        ))?,
        OutcomeReceipt::try_rehydrate(OutcomeReceiptParts::new(
            parts.id().clone(),
            ContentHash::new("sha256:substituted"),
            parts.candidate().clone(),
            parts.authorization().clone(),
            parts.payload_set_hash().clone(),
            parts.executed_by().clone(),
            parts.outcome_hash().clone(),
            parts.status().clone(),
        ))?,
        OutcomeReceipt::try_rehydrate(OutcomeReceiptParts::new(
            parts.id().clone(),
            parts.receipt_hash().clone(),
            parts.candidate().clone(),
            parts.authorization().clone(),
            parts.payload_set_hash().clone(),
            parts.executed_by().clone(),
            ContentHash::new("sha256:substituted"),
            parts.status().clone(),
        ))?,
        OutcomeReceipt::try_rehydrate(OutcomeReceiptParts::new(
            parts.id().clone(),
            parts.receipt_hash().clone(),
            parts.candidate().clone(),
            parts.authorization().clone(),
            parts.payload_set_hash().clone(),
            parts.executed_by().clone(),
            parts.outcome_hash().clone(),
            OutcomeStatus::Failed {
                error_code: "tool.invoke_error".into(),
            },
        ))?,
    ];

    for drifted in cases {
        assert!(matches!(
            commit_terminal_fixture(&intent, drifted, canonical.clone(), evidence_hash.clone(),)
                .await,
            Err(OutcomeStoreError::Corrupt { .. })
        ));
    }
    Ok(())
}

#[tokio::test]
async fn self_consistent_terminal_substitution_cannot_clear_exact_intent() -> TestResult {
    let intent = execution_intent("attempt-execution-substitution", "correlation-substitution")?;
    let (receipt, canonical, _) = canonical_execution_outcome(&intent)?;
    let mut envelope: Value = serde_json::from_str(&canonical)?;
    let fields = envelope
        .get_mut("fields")
        .and_then(Value::as_array_mut)
        .expect("fields");
    fields[2][1] = Value::String("correlation-other".into());
    let typed_fields = fields
        .iter()
        .map(|field| {
            let pair = field.as_array().expect("pair");
            (pair[0].as_str().expect("name").to_owned(), pair[1].clone())
        })
        .collect::<Vec<_>>();
    let substituted_hash = hash_evidence_fields(&typed_fields);
    let parts = receipt.rehydration_parts();
    let substituted_id = test_receipt_id(&intent, parts.authorization(), &substituted_hash);
    let context = test_context();
    let executor_binding = test_executor_binding_hash(&intent, &context);
    let substituted_receipt_hash = test_receipt_hash(
        &intent,
        &substituted_id,
        parts.authorization(),
        parts.candidate(),
        &context,
        &executor_binding,
        &substituted_hash,
    );
    let substituted_receipt = OutcomeReceipt::try_rehydrate(OutcomeReceiptParts::new(
        ReceiptId::new(substituted_id),
        substituted_receipt_hash,
        parts.candidate().clone(),
        parts.authorization().clone(),
        parts.payload_set_hash().clone(),
        parts.executed_by().clone(),
        substituted_hash.clone(),
        parts.status().clone(),
    ))?;
    assert!(matches!(
        commit_terminal_fixture(
            &intent,
            substituted_receipt,
            serde_json::to_string(&envelope)?,
            substituted_hash,
        )
        .await,
        Err(OutcomeStoreError::Corrupt { detail }) if detail.contains("correlation.id")
    ));
    Ok(())
}

#[tokio::test]
async fn self_consistent_candidate_reference_substitution_cannot_clear_exact_intent() -> TestResult
{
    let intent = execution_intent(
        "attempt-execution-candidate-substitution",
        "correlation-candidate-substitution",
    )?;
    let (receipt, canonical, _) = canonical_execution_outcome(&intent)?;
    let parts = receipt.rehydration_parts();

    for substitution in [
        CandidateSubstitution::Id,
        CandidateSubstitution::Revision,
        CandidateSubstitution::Action,
        CandidateSubstitution::Metacontrol,
    ] {
        let candidate = substituted_candidate(parts.candidate(), substitution);
        let mut envelope: Value = serde_json::from_str(&canonical)?;
        let fields = envelope
            .get_mut("fields")
            .and_then(Value::as_array_mut)
            .expect("fields");
        replace_candidate_evidence(fields, &candidate, substitution);
        let typed_fields = fields
            .iter()
            .map(|field| {
                let pair = field.as_array().expect("pair");
                (pair[0].as_str().expect("name").to_owned(), pair[1].clone())
            })
            .collect::<Vec<_>>();
        let substituted_hash = hash_evidence_fields(&typed_fields);
        let substituted_id = test_receipt_id(&intent, parts.authorization(), &substituted_hash);
        let context = test_context();
        let executor_binding = test_executor_binding_hash(&intent, &context);
        let substituted_receipt_hash = test_receipt_hash(
            &intent,
            &substituted_id,
            parts.authorization(),
            &candidate,
            &context,
            &executor_binding,
            &substituted_hash,
        );
        let substituted_receipt = OutcomeReceipt::try_rehydrate(OutcomeReceiptParts::new(
            ReceiptId::new(substituted_id),
            substituted_receipt_hash,
            candidate,
            parts.authorization().clone(),
            parts.payload_set_hash().clone(),
            parts.executed_by().clone(),
            substituted_hash.clone(),
            parts.status().clone(),
        ))?;
        assert!(matches!(
            commit_terminal_fixture(
                &intent,
                substituted_receipt,
                serde_json::to_string(&envelope)?,
                substituted_hash,
            )
            .await,
            Err(OutcomeStoreError::Corrupt { detail })
                if detail.contains("candidate.reference_hash")
        ));
    }
    Ok(())
}

#[test]
fn synchronous_terminal_record_resolves_exact_execution_intent() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-execution-sync.sqlite3");
    let intent = execution_intent("attempt-execution-sync", "correlation-sync")?;
    let (receipt, envelope, evidence) = canonical_execution_outcome(&intent)?;

    {
        let writer = SyncDurableOutcomeWriter::bootstrap_new(&database_path)?;
        writer.stage_execution_intent(intent.clone())?;
    }
    let reopened = SyncDurableOutcomeWriter::open_existing(&database_path)?;
    assert_eq!(reopened.pending_execution_intents()?, vec![intent.clone()]);
    assert_eq!(
        reopened.record_and_resolve_execution(
            intent.attempt_id(),
            receipt,
            envelope,
            evidence,
            &intent,
        )?,
        OutcomeRecordResult::Recorded
    );
    assert!(reopened.pending_execution_intents()?.is_empty());
    Ok(())
}

async fn commit_terminal_fixture(
    intent: &ExecutionIntent,
    receipt: OutcomeReceipt,
    evidence: String,
    evidence_hash: ContentHash,
) -> Result<OutcomeRecordResult, OutcomeStoreError> {
    let directory = tempfile::tempdir().map_err(|error| OutcomeStoreError::Persistence {
        operation: "create execution-intent test directory",
        detail: error.to_string(),
    })?;
    let store = DurableOutcomeStore::bootstrap_new(
        directory.path().join("execution-terminal-fixture.sqlite3"),
    )
    .await?;
    store.stage_execution_intent(intent.clone()).await?;
    store
        .stage_intent(
            intent.attempt_id(),
            receipt.clone(),
            evidence.clone(),
            evidence_hash.clone(),
        )
        .await?;
    store
        .commit_staged_intent_and_resolve_execution(
            intent.attempt_id().to_owned(),
            receipt,
            evidence,
            evidence_hash,
            intent.idempotency_key().to_owned(),
        )
        .await
}

fn canonical_execution_outcome(
    intent: &ExecutionIntent,
) -> Result<(OutcomeReceipt, String, ContentHash), Box<dyn std::error::Error>> {
    canonical_execution_outcome_with_ack(intent, None)
}

pub(super) fn canonical_execution_outcome_with_ack(
    intent: &ExecutionIntent,
    effect_ack: Option<&ExecutionEffectAck>,
) -> Result<(OutcomeReceipt, String, ContentHash), Box<dyn std::error::Error>> {
    let context = test_context();
    let candidate = test_candidate(&context);
    let authorization = AuthorizationRef::new(
        AuthorizationId::new("authorization-outcome-store"),
        Revision::new(1),
        intent.authorization_digest().clone(),
    );
    let executor_binding = test_executor_binding_hash(intent, &context);

    let mut fields = Vec::new();
    push_text(&mut fields, "attempt.id", intent.attempt_id());
    push_text(&mut fields, "session.id", intent.session_id());
    push_text(&mut fields, "correlation.id", intent.correlation_id());
    push_text(&mut fields, "tool.name", intent.tool_name());
    push_text(&mut fields, "tool.operation", intent.capability_operation());
    push_capability(&mut fields, intent, &context);
    push_text(&mut fields, "executor", intent.executor_principal());
    push_text(
        &mut fields,
        "executor.provider",
        intent.capability_provider(),
    );
    push_text(
        &mut fields,
        "executor.manifest_hash",
        intent.capability_manifest_hash().as_str(),
    );
    push_text(
        &mut fields,
        "executor.binding_hash",
        executor_binding.as_str(),
    );
    push_text(&mut fields, "payload.hash", intent.payload_hash().as_str());
    push_text(
        &mut fields,
        "execution.idempotency_key",
        intent.idempotency_key(),
    );
    push_text(
        &mut fields,
        "execution.resource_summary_hash",
        intent.resource_summary_hash().as_str(),
    );
    push_text(
        &mut fields,
        "execution.effect_plan_hash",
        intent
            .effect_plan_hash()
            .map(ContentHash::as_str)
            .unwrap_or(""),
    );
    push_text(
        &mut fields,
        "execution.effect_ack_hash",
        effect_ack
            .map(ExecutionEffectAck::ack_hash)
            .map(ContentHash::as_str)
            .unwrap_or(""),
    );
    push_number(&mut fields, "time.started_unix_ms", 10);
    push_number(&mut fields, "time.finished_unix_ms", 20);
    push_text(&mut fields, "authorization.id", authorization.id().as_str());
    push_number(
        &mut fields,
        "authorization.revision",
        authorization.revision().get(),
    );
    push_text(
        &mut fields,
        "authorization.content_hash",
        authorization.content_hash().as_str(),
    );
    push_text(&mut fields, "admission.id", intent.admission_id());
    push_number(
        &mut fields,
        "admission.revision",
        intent.admission_revision(),
    );
    push_text(
        &mut fields,
        "admission.content_hash",
        intent.admission_digest().as_str(),
    );
    push_candidate(&mut fields, &candidate);
    push_context(&mut fields, "authorization.context", &context);
    push_stamp(&mut fields, "authorization.policy", context.policy());
    push_text(&mut fields, "authorization.decided_by", "safety-kernel");
    let scope_hash = test_authorization_scope_hash(intent, &context);
    push_text(&mut fields, "authorization.scope_hash", scope_hash.as_str());
    push_text(&mut fields, "authorization.decision", "authorized");
    push_text(
        &mut fields,
        "kernel.candidate_binding",
        intent.kernel_candidate_hash().as_str(),
    );
    push_text(
        &mut fields,
        "kernel.payload_set_hash",
        intent.payload_set_hash().as_str(),
    );
    push_text(&mut fields, "terminal.status", "succeeded");
    push_text(&mut fields, "terminal.code", "ok");
    push_number(&mut fields, "terminal.timeout_ms", 0);
    push_text(&mut fields, "terminal.error_hash", "");
    push_text(&mut fields, "effect.disposition", "none");
    push_text(&mut fields, "validation.status", "not_required");
    push_text(&mut fields, "validation.error_hash", "");
    for prefix in ["content", "provider_output", "final_output"] {
        push_text(&mut fields, &format!("{prefix}.presence"), "absent");
        push_text(&mut fields, &format!("{prefix}.hash"), "");
    }
    push_text(&mut fields, "transaction.status", "not_applicable");
    push_text(&mut fields, "transaction.id", "");
    push_text(&mut fields, "transaction.group_id", "");
    push_text(&mut fields, "transaction.entry_hash", "");
    push_text(&mut fields, "transaction.error_hash", "");

    let evidence_hash = hash_evidence_fields(&fields);
    let receipt_id = test_receipt_id(intent, &authorization, &evidence_hash);
    let receipt_hash = test_receipt_hash(
        intent,
        &receipt_id,
        &authorization,
        &candidate,
        &context,
        &executor_binding,
        &evidence_hash,
    );
    let receipt = OutcomeReceipt::try_rehydrate(OutcomeReceiptParts::new(
        ReceiptId::new(receipt_id),
        receipt_hash,
        candidate,
        authorization,
        intent.payload_set_hash().clone(),
        PrincipalId::new(intent.executor_principal()),
        evidence_hash.clone(),
        OutcomeStatus::Succeeded,
    ))?;
    let envelope = serde_json::to_string(&serde_json::json!({
        "domain": "hepta.runtime.tool-outcome.v1",
        "fields": fields,
    }))?;
    Ok((receipt, envelope, evidence_hash))
}

fn push_text(fields: &mut Vec<(String, Value)>, name: &str, value: &str) {
    fields.push((name.to_owned(), Value::String(value.to_owned())));
}

fn push_number(fields: &mut Vec<(String, Value)>, name: &str, value: u64) {
    fields.push((name.to_owned(), Value::from(value)));
}

fn push_capability(
    fields: &mut Vec<(String, Value)>,
    intent: &ExecutionIntent,
    context: &FrozenTurnContext,
) {
    push_text(fields, "capability.id", intent.capability_id());
    push_number(fields, "capability.revision", intent.capability_revision());
    push_text(
        fields,
        "capability.manifest_hash",
        intent.capability_manifest_hash().as_str(),
    );
    push_stamp(fields, "capability.catalog", context.capability_catalog());
}

fn push_candidate(fields: &mut Vec<(String, Value)>, candidate: &CandidateRef) {
    push_text(fields, "candidate.id", candidate.id().as_str());
    push_number(fields, "candidate.revision", candidate.revision().get());
    push_text(
        fields,
        "candidate.content_hash",
        candidate.content_hash().as_str(),
    );
    push_context(fields, "candidate.context", candidate.context());
    push_text(
        fields,
        "candidate.action_hash",
        candidate.action_hash().as_str(),
    );
    push_text(
        fields,
        "candidate.metacontrol_hash",
        candidate.metacontrol_hash().as_str(),
    );
    push_text(
        fields,
        "candidate.payload_set_hash",
        candidate.payload_set_hash().as_str(),
    );
}

fn push_context(fields: &mut Vec<(String, Value)>, prefix: &str, context: &FrozenTurnContext) {
    push_text(
        fields,
        &format!("{prefix}.observation.id"),
        context.observation().id().as_str(),
    );
    push_number(
        fields,
        &format!("{prefix}.observation.revision"),
        context.observation().revision().get(),
    );
    push_text(
        fields,
        &format!("{prefix}.observation.content_hash"),
        context.observation().content_hash().as_str(),
    );
    push_stamp(fields, &format!("{prefix}.state"), context.state());
    push_stamp(fields, &format!("{prefix}.policy"), context.policy());
    push_stamp(
        fields,
        &format!("{prefix}.capability_catalog"),
        context.capability_catalog(),
    );
    push_stamp(
        fields,
        &format!("{prefix}.preference"),
        context.preference(),
    );
}

fn push_stamp(fields: &mut Vec<(String, Value)>, prefix: &str, stamp: &RevisionStamp) {
    push_number(
        fields,
        &format!("{prefix}.revision"),
        stamp.revision().get(),
    );
    push_text(
        fields,
        &format!("{prefix}.content_hash"),
        stamp.content_hash().as_str(),
    );
}

fn hash_evidence_fields(fields: &[(String, Value)]) -> ContentHash {
    let mut hash = TestFrameHasher::new("hepta.runtime.tool-outcome.v1");
    for (name, value) in fields {
        if let Some(text) = value.as_str() {
            hash.text(name, text);
        } else {
            hash.number(name, value.as_u64().expect("canonical u64 test field"));
        }
    }
    hash.finish()
}

fn test_receipt_id(
    intent: &ExecutionIntent,
    authorization: &AuthorizationRef,
    outcome_hash: &ContentHash,
) -> String {
    let mut hash = TestFrameHasher::new("hepta.runtime.outcome-receipt-id.v1");
    hash.text("attempt.id", intent.attempt_id());
    hash.text("authorization.id", authorization.id().as_str());
    hash.number("authorization.revision", authorization.revision().get());
    hash.text(
        "authorization.content_hash",
        authorization.content_hash().as_str(),
    );
    hash.text("outcome.hash", outcome_hash.as_str());
    format!("receipt:{}", hash.finish().as_str())
}

#[allow(clippy::too_many_arguments)]
fn test_receipt_hash(
    intent: &ExecutionIntent,
    receipt_id: &str,
    authorization: &AuthorizationRef,
    candidate: &CandidateRef,
    context: &FrozenTurnContext,
    executor_binding: &ContentHash,
    outcome_hash: &ContentHash,
) -> ContentHash {
    let mut hash = TestFrameHasher::new("hepta.runtime.outcome-receipt.v1");
    hash.text("receipt.id", receipt_id);
    hash_candidate(&mut hash, candidate);
    hash.text("authorization.id", authorization.id().as_str());
    hash.number("authorization.revision", authorization.revision().get());
    hash.text(
        "authorization.content_hash",
        authorization.content_hash().as_str(),
    );
    hash_capability(&mut hash, intent, context);
    hash.text("payload_set_hash", intent.payload_hash().as_str());
    hash.text("executor", intent.executor_principal());
    hash.text("executor.provider", intent.capability_provider());
    hash.text(
        "executor.manifest_hash",
        intent.capability_manifest_hash().as_str(),
    );
    hash.text("executor.binding_hash", executor_binding.as_str());
    hash.text("status.tag", "succeeded");
    hash.text("status.code", "");
    hash.text("outcome.hash", outcome_hash.as_str());
    hash.finish()
}

fn test_executor_binding_hash(
    intent: &ExecutionIntent,
    context: &FrozenTurnContext,
) -> ContentHash {
    let mut hash = TestFrameHasher::new("hepta.runtime.tool-executor-binding.v1");
    hash_capability(&mut hash, intent, context);
    hash.text("executor", intent.executor_principal());
    hash.text("executor.provider", intent.capability_provider());
    hash.text(
        "executor.manifest_hash",
        intent.capability_manifest_hash().as_str(),
    );
    hash.finish()
}

fn test_executor_principal() -> String {
    let mut hash = TestFrameHasher::new("hepta.runtime.executor-principal.v1");
    hash.text("provider", "hepta-runtime-test");
    hash.text("operation", "test.outcome-store");
    hash.text("manifest_hash", "sha256:capability");
    format!("executor:{}", hash.finish().as_str())
}

fn hash_capability(
    hash: &mut TestFrameHasher,
    intent: &ExecutionIntent,
    context: &FrozenTurnContext,
) {
    hash.text("capability.id", intent.capability_id());
    hash.number("capability.revision", intent.capability_revision());
    hash.text(
        "capability.manifest_hash",
        intent.capability_manifest_hash().as_str(),
    );
    hash.number(
        "capability.catalog.revision",
        context.capability_catalog().revision().get(),
    );
    hash.text(
        "capability.catalog.content_hash",
        context.capability_catalog().content_hash().as_str(),
    );
}

fn hash_candidate(hash: &mut TestFrameHasher, candidate: &CandidateRef) {
    hash.text("candidate.id", candidate.id().as_str());
    hash.number("candidate.revision", candidate.revision().get());
    hash.text("candidate.content_hash", candidate.content_hash().as_str());
    hash_context(hash, "candidate.context", candidate.context());
    hash.text("candidate.action_hash", candidate.action_hash().as_str());
    hash.text(
        "candidate.metacontrol_hash",
        candidate.metacontrol_hash().as_str(),
    );
    hash.text(
        "candidate.payload_set_hash",
        candidate.payload_set_hash().as_str(),
    );
}

fn hash_context(hash: &mut TestFrameHasher, prefix: &str, context: &FrozenTurnContext) {
    hash.text(
        &format!("{prefix}.observation.id"),
        context.observation().id().as_str(),
    );
    hash.number(
        &format!("{prefix}.observation.revision"),
        context.observation().revision().get(),
    );
    hash.text(
        &format!("{prefix}.observation.content_hash"),
        context.observation().content_hash().as_str(),
    );
    hash_stamp(hash, &format!("{prefix}.state"), context.state());
    hash_stamp(hash, &format!("{prefix}.policy"), context.policy());
    hash_stamp(
        hash,
        &format!("{prefix}.capability_catalog"),
        context.capability_catalog(),
    );
    hash_stamp(hash, &format!("{prefix}.preference"), context.preference());
}

fn hash_stamp(hash: &mut TestFrameHasher, prefix: &str, stamp: &RevisionStamp) {
    hash.number(&format!("{prefix}.revision"), stamp.revision().get());
    hash.text(
        &format!("{prefix}.content_hash"),
        stamp.content_hash().as_str(),
    );
}

struct TestFrameHasher(Sha256);

impl TestFrameHasher {
    fn new(domain: &str) -> Self {
        let mut value = Self(Sha256::new());
        value.bytes("domain", domain.as_bytes());
        value
    }

    fn bytes(&mut self, name: &str, value: &[u8]) {
        self.0.update((name.len() as u64).to_be_bytes());
        self.0.update(name.as_bytes());
        self.0.update((value.len() as u64).to_be_bytes());
        self.0.update(value);
    }

    fn text(&mut self, name: &str, value: &str) {
        self.bytes(name, value.as_bytes());
    }

    fn number(&mut self, name: &str, value: u64) {
        self.bytes(name, &value.to_be_bytes());
    }

    fn finish(self) -> ContentHash {
        ContentHash::new(format!("sha256:{:x}", self.0.finalize()))
    }
}

fn test_context() -> FrozenTurnContext {
    let observation = ObservationSnapshot::new(
        ObservationId::new("observation-outcome-store"),
        Revision::new(1),
        ContentHash::new("sha256:observation"),
        PrincipalId::new("observer"),
        Vec::new(),
    );
    FrozenTurnContext::new(
        observation.reference(),
        revision_stamp("state"),
        revision_stamp("policy"),
        revision_stamp("catalog"),
        revision_stamp("preference"),
    )
}

fn test_authorization_scope_hash(
    intent: &ExecutionIntent,
    context: &FrozenTurnContext,
) -> ContentHash {
    test_authorization_scope_hash_for(intent.attempt_id(), intent.executor_principal(), context)
}

fn test_authorization_scope_hash_for(
    attempt_id: &str,
    executor_principal: &str,
    context: &FrozenTurnContext,
) -> ContentHash {
    let mut hash = TestFrameHasher::new("hepta.runtime.authorization-scope.v2");
    hash.text("attempt_id", attempt_id);
    hash.text("tool_name", "test_tool");
    hash.text("payload_hash", "sha256:payload");
    hash.text("capability_id", "capability-outcome-store");
    hash.text("capability_revision", "1");
    hash.text("capability_manifest_hash", "sha256:capability");
    hash.text("catalog_revision", "1");
    hash.text("executor_principal", executor_principal);
    hash.text("executor_provider", "hepta-runtime-test");
    hash.text("executor_operation", "test.outcome-store");
    hash.text("executor_manifest_hash", "sha256:capability");
    hash.text("state", context.state().content_hash().as_str());
    hash.text("policy", context.policy().content_hash().as_str());
    hash.text(
        "catalog",
        context.capability_catalog().content_hash().as_str(),
    );
    hash.finish()
}

fn test_authorization_digest(
    attempt_id: &str,
    executor_principal: &str,
    context: &FrozenTurnContext,
) -> ContentHash {
    let scope_hash = test_authorization_scope_hash_for(attempt_id, executor_principal, context);
    let mut hash = TestFrameHasher::new("hepta.kernel.safety-gate.authorization-record.v1");
    hash.text("authorization.id", "authorization-outcome-store");
    hash.number("authorization.revision", 1);
    hash.text("authorization.admission.content_hash", "sha256:admission");
    hash.text("authorization.candidate_binding", "sha256:kernel-candidate");
    hash.text("authorization.payload_set_hash", "sha256:payload");
    hash_context(&mut hash, "authorization.current_context", context);
    hash.text("authorization.decided_by", "safety-kernel");
    hash.text("authorization.scope_hash", scope_hash.as_str());
    hash.finish()
}

fn execution_intent(
    attempt_id: &str,
    correlation_id: &str,
) -> Result<ExecutionIntent, Box<dyn std::error::Error>> {
    execution_intent_with_effect_plan(attempt_id, correlation_id, None)
}

pub(super) fn execution_intent_with_effect_plan(
    attempt_id: &str,
    correlation_id: &str,
    canonical_effect_plan: Option<String>,
) -> Result<ExecutionIntent, Box<dyn std::error::Error>> {
    let context = test_context();
    let candidate = test_candidate(&context);
    let executor_principal = test_executor_principal();
    let authorization_digest = test_authorization_digest(attempt_id, &executor_principal, &context);
    Ok(ExecutionIntent::try_new(ExecutionIntentParts {
        attempt_id: attempt_id.into(),
        session_id: "session-execution-intent".into(),
        correlation_id: correlation_id.into(),
        tool_name: "test_tool".into(),
        payload_hash: ContentHash::new("sha256:payload"),
        candidate_hash: ContentHash::new("sha256:candidate"),
        candidate_reference_hash: candidate_reference_hash(&candidate),
        kernel_candidate_hash: ContentHash::new("sha256:kernel-candidate"),
        payload_set_hash: ContentHash::new("sha256:payload"),
        capability_id: "capability-outcome-store".into(),
        capability_revision: 1,
        capability_provider: "hepta-runtime-test".into(),
        capability_operation: "test.outcome-store".into(),
        capability_manifest_hash: ContentHash::new("sha256:capability"),
        executor_principal,
        authorization_digest,
        admission_id: "admission-outcome-store".into(),
        admission_revision: 1,
        admission_digest: ContentHash::new("sha256:admission"),
        canonical_resource_summary: r#"{"read":null,"schema_version":1,"writes":[]}"#.into(),
        canonical_effect_plan,
    })?)
}

fn test_candidate(context: &FrozenTurnContext) -> CandidateRef {
    CandidateRef::new(
        CandidateId::new("candidate-outcome-store"),
        Revision::new(1),
        ContentHash::new("sha256:candidate"),
        context.clone(),
        ContentHash::new("sha256:action"),
        ContentHash::new("sha256:metacontrol"),
        ContentHash::new("sha256:payload"),
    )
}

#[derive(Clone, Copy)]
enum CandidateSubstitution {
    Id,
    Revision,
    Action,
    Metacontrol,
}

fn substituted_candidate(
    candidate: &CandidateRef,
    substitution: CandidateSubstitution,
) -> CandidateRef {
    CandidateRef::new(
        if matches!(substitution, CandidateSubstitution::Id) {
            CandidateId::new("candidate-substituted")
        } else {
            candidate.id().clone()
        },
        if matches!(substitution, CandidateSubstitution::Revision) {
            Revision::new(candidate.revision().get() + 1)
        } else {
            candidate.revision()
        },
        candidate.content_hash().clone(),
        candidate.context().clone(),
        if matches!(substitution, CandidateSubstitution::Action) {
            ContentHash::new("sha256:action-substituted")
        } else {
            candidate.action_hash().clone()
        },
        if matches!(substitution, CandidateSubstitution::Metacontrol) {
            ContentHash::new("sha256:metacontrol-substituted")
        } else {
            candidate.metacontrol_hash().clone()
        },
        candidate.payload_set_hash().clone(),
    )
}

fn replace_candidate_evidence(
    fields: &mut [Value],
    candidate: &CandidateRef,
    substitution: CandidateSubstitution,
) {
    let (name, value) = match substitution {
        CandidateSubstitution::Id => (
            "candidate.id",
            Value::String(candidate.id().as_str().to_owned()),
        ),
        CandidateSubstitution::Revision => (
            "candidate.revision",
            Value::from(candidate.revision().get()),
        ),
        CandidateSubstitution::Action => (
            "candidate.action_hash",
            Value::String(candidate.action_hash().as_str().to_owned()),
        ),
        CandidateSubstitution::Metacontrol => (
            "candidate.metacontrol_hash",
            Value::String(candidate.metacontrol_hash().as_str().to_owned()),
        ),
    };
    let field = fields
        .iter_mut()
        .find(|field| field.get(0).and_then(Value::as_str) == Some(name))
        .expect("candidate evidence field");
    field[1] = value;
}

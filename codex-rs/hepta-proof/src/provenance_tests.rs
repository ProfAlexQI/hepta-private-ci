use std::collections::BTreeMap;

use codex_hepta_contracts::ActionId;
use codex_hepta_contracts::ChannelAdapterId;
use codex_hepta_contracts::ChannelIngressEvent;
use codex_hepta_contracts::ChannelIngressReceipt;
use codex_hepta_contracts::ChannelIngressTerminal;
use codex_hepta_contracts::ChannelScope;
use codex_hepta_contracts::GOVERNANCE_SCHEMA_VERSION;
use codex_hepta_contracts::GovernanceDecision;
use codex_hepta_contracts::GovernanceDecisionRecord;
use codex_hepta_contracts::GovernanceMode;
use codex_hepta_contracts::GovernanceReceipt;
use codex_hepta_contracts::HandlerOutcome;
use codex_hepta_contracts::MigrationDisposition;
use codex_hepta_contracts::MigrationFamilyId;
use codex_hepta_contracts::MigrationFamilySnapshot;
use codex_hepta_contracts::PROVIDER_EVIDENCE_SCHEMA_VERSION;
use codex_hepta_contracts::PolicyPhase;
use codex_hepta_contracts::PolicyStamp;
use codex_hepta_contracts::ProviderInvocationIntent;
use codex_hepta_contracts::ProviderInvocationReceipt;
use codex_hepta_contracts::ProviderRequestBinding;
use codex_hepta_contracts::ProviderRequestKind;
use codex_hepta_contracts::ProviderTerminal;
use codex_hepta_contracts::ProviderTransport;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::ToolAction;
use codex_hepta_contracts::ToolActionSource;
use codex_hepta_contracts::channel_target_thread_sha256;
use codex_hepta_evidence::HeptaEvidenceStore;
use codex_hepta_evidence::HistoricalEvidenceFamily;
use codex_hepta_evidence::HistoricalEvidenceSelector;
use codex_hepta_evidence::HistoricalEvidenceState;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use serde_json::Value;
use serde_json::json;
use tempfile::TempDir;

use crate::LocalProofProvenanceLineage;
use crate::ProofCommandSpec;
use crate::ProofContextOrigin;
use crate::ProofHarness;
use crate::ProofInvocation;
use crate::ProofInvocationId;
use crate::ProofProvenanceContext;
use crate::ProofReceiptId;
use crate::ProofStore;
use crate::ProofStreamKind;
use crate::ProofSubject;
use crate::ProofTerminal;
use crate::ProvenanceEvidenceKind;
use crate::build_local_proof_provenance_lineage;
use crate::run_historical_observation;

fn digest(label: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(label.as_bytes())
}

#[test]
fn historical_store_resolved_invocation_id_has_a_fixed_oracle() {
    let subject = ProofSubject::new_historical_store_resolved(
        digest("canonical-candidate"),
        digest("canonical-historical-context"),
    )
    .expect("historical-store-resolved subject");
    let invocation_id = ProofInvocationId::for_intent(
        &subject,
        &digest("canonical-command-binding"),
        &digest("canonical-nonce"),
    );

    assert_eq!(
        invocation_id.as_str(),
        "proof-invocation:v1:966059f2053fd0172912e2b81aa4b77a448f258dbdd7778c488feaa91b70b6b3"
    );
}

fn sqlite_config(temp: &TempDir) -> SqliteConfig {
    SqliteConfig::new_for_testing(
        AbsolutePathBuf::try_from(temp.path().to_path_buf()).expect("absolute temp path"),
    )
}

fn snapshot(candidate_sha256: Sha256Digest) -> MigrationFamilySnapshot {
    MigrationFamilySnapshot::new(
        MigrationFamilyId::new("proof/exact-lineage").expect("family"),
        MigrationDisposition::Rewrite,
        digest("old-proof-implementation"),
        digest("vnext-proof-implementation"),
        candidate_sha256,
    )
}

fn ingress_event(scope_label: &str, source_label: &str) -> ChannelIngressEvent {
    ChannelIngressEvent::new(
        ChannelScope {
            adapter_id: ChannelAdapterId::new("historical-clean").expect("adapter"),
            installation_sha256: digest("installation"),
            account_sha256: digest(scope_label),
            conversation_sha256: digest(scope_label),
            principal_sha256: digest("principal"),
        },
        digest(source_label),
        digest("event-payload"),
        channel_target_thread_sha256("thread-1").expect("target thread"),
        None,
        digest("next-cursor"),
        10_000,
    )
    .expect("ingress event")
}

fn governance_decision(phase: PolicyPhase) -> GovernanceDecisionRecord {
    GovernanceDecisionRecord::new(
        ToolAction {
            schema_version: GOVERNANCE_SCHEMA_VERSION,
            action_id: ActionId::for_tool_call("thread-1", "turn-1", "proof-call-1"),
            thread_id: "thread-1".to_string(),
            turn_id: "turn-1".to_string(),
            call_id: "proof-call-1".to_string(),
            tool_name: "exec_command".to_string(),
            source: ToolActionSource::Direct,
            payload_sha256: digest("governance-payload"),
        },
        phase,
        GovernanceMode::Enforce,
        PolicyStamp::new("hepta.proof-context.v1", 1, b"allow"),
        GovernanceDecision::Allow,
    )
}

fn provider_intent() -> ProviderInvocationIntent {
    ProviderInvocationIntent::new(
        [41; 16],
        ProviderRequestBinding {
            schema_version: PROVIDER_EVIDENCE_SCHEMA_VERSION,
            thread_id: "thread-1".to_string(),
            turn_id: "turn-1".to_string(),
            host_request_binding_id_sha256: digest("host-binding"),
            request_kind: ProviderRequestKind::Turn,
            provider_id: "fixture-provider".to_string(),
            provider_config_sha256: digest("provider-config"),
            model: "fixture-model".to_string(),
            transport: ProviderTransport::Http,
            endpoint_sha256: digest("/responses"),
            logical_request_sha256: digest("logical-request"),
            ephemeral_input_sha256: None,
            ephemeral_input_witness_sha256: None,
            wire_semantic_sha256: digest("wire-semantic"),
            previous_response_id_sha256: None,
            generate: true,
        },
    )
}

fn command(temp: &TempDir) -> ProofCommandSpec {
    #[cfg(unix)]
    let (program, arguments) = (
        std::path::PathBuf::from("/usr/bin/printf"),
        vec!["proof-provenance\n".to_string()],
    );
    #[cfg(not(unix))]
    let (program, arguments) = (
        std::env::current_exe().expect("current test executable"),
        vec![
            "--list".to_string(),
            "--format".to_string(),
            "terse".to_string(),
        ],
    );
    ProofCommandSpec::new(
        program,
        arguments,
        temp.path(),
        BTreeMap::new(),
        30_000,
        1024 * 1024,
        1024 * 1024,
    )
    .expect("proof command")
}

#[cfg(unix)]
fn failing_command(temp: &TempDir) -> ProofCommandSpec {
    let program = ["/usr/bin/false", "/bin/false"]
        .into_iter()
        .find(|path| std::path::Path::new(path).is_file())
        .expect("false executable");
    ProofCommandSpec::new(
        program,
        Vec::new(),
        temp.path(),
        BTreeMap::new(),
        30_000,
        1024 * 1024,
        1024 * 1024,
    )
    .expect("failing proof command")
}

#[tokio::test]
async fn governance_and_provider_positive_records_have_fixed_context_oracles() {
    let temp = TempDir::new().expect("temp dir");
    let evidence_store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence store");
    let proof_store = ProofStore::open(temp.path()).expect("open proof store");
    let migration_snapshot = snapshot(digest("candidate"));

    let admission = governance_decision(PolicyPhase::Admission);
    let authorization = governance_decision(PolicyPhase::Authorization);
    evidence_store
        .append_decision(&admission)
        .await
        .expect("append admission");
    evidence_store
        .append_decision(&authorization)
        .await
        .expect("append authorization");
    evidence_store
        .append_receipt(&GovernanceReceipt::new(
            admission.clone(),
            Some(authorization),
            true,
            HandlerOutcome::HandlerCompleted {
                reported_success: true,
            },
        ))
        .await
        .expect("append governance receipt");
    let governance_selector = HistoricalEvidenceSelector::new(
        HistoricalEvidenceFamily::GovernanceAction,
        admission.action.action_id.as_str(),
    )
    .expect("governance selector");
    let governance_record = evidence_store
        .historical_record(&governance_selector)
        .await
        .expect("governance read")
        .expect("governance record");
    let governance_context =
        ProofProvenanceContext::from_record(&migration_snapshot, &governance_record)
            .expect("governance context");
    assert_eq!(
        governance_context.evidence_kind(),
        ProvenanceEvidenceKind::GovernanceHandlerCompletedSuccess
    );
    assert_eq!(
        governance_context.context_sha256().as_str(),
        "e9ab5bf734dd31c7e8408e1a18c31c03f4c38c97de5d9601a35295ad9dbe180c"
    );
    let governance_result = run_historical_observation(
        &ProofHarness::new(proof_store.clone()),
        &evidence_store,
        &migration_snapshot,
        &governance_selector,
        [51; 16],
        command(&temp),
    )
    .await
    .expect("governance proof observation");
    assert_eq!(
        build_local_proof_provenance_lineage(
            &proof_store,
            &evidence_store,
            &migration_snapshot,
            &governance_selector,
            governance_result.receipt.receipt_id(),
        )
        .await
        .expect("governance lineage")
        .context(),
        &governance_context
    );

    let provider = provider_intent();
    evidence_store
        .append_provider_intent(&provider)
        .await
        .expect("append provider intent");
    evidence_store
        .append_provider_receipt(&ProviderInvocationReceipt::new(
            provider.clone(),
            ProviderTerminal::Completed {
                response_id_sha256: digest("response-id"),
                response_items_sha256: digest("response-items"),
                token_usage_sha256: digest("token-usage"),
                end_turn: Some(true),
            },
        ))
        .await
        .expect("append provider receipt");
    let provider_selector = HistoricalEvidenceSelector::new(
        HistoricalEvidenceFamily::ProviderAttempt,
        provider.attempt_id.as_str(),
    )
    .expect("provider selector");
    let provider_record = evidence_store
        .historical_record(&provider_selector)
        .await
        .expect("provider read")
        .expect("provider record");
    let provider_context =
        ProofProvenanceContext::from_record(&migration_snapshot, &provider_record)
            .expect("provider context");
    assert_eq!(
        provider_context.evidence_kind(),
        ProvenanceEvidenceKind::ProviderCompleted
    );
    assert_eq!(
        provider_context.context_sha256().as_str(),
        "86e96424b33f457c051db14eb049c304c879df88169886b8738c63154650bdbb"
    );
    let provider_result = run_historical_observation(
        &ProofHarness::new(proof_store.clone()),
        &evidence_store,
        &migration_snapshot,
        &provider_selector,
        [52; 16],
        command(&temp),
    )
    .await
    .expect("provider proof observation");
    assert_eq!(
        build_local_proof_provenance_lineage(
            &proof_store,
            &evidence_store,
            &migration_snapshot,
            &provider_selector,
            provider_result.receipt.receipt_id(),
        )
        .await
        .expect("provider lineage")
        .context(),
        &provider_context
    );
}

#[tokio::test]
async fn exact_positive_historical_record_binds_proof_invocation_context() {
    let temp = TempDir::new().expect("temp dir");
    let evidence_store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence store");
    let event = ingress_event("account", "source-event");
    evidence_store
        .claim_channel_ingress_event(&event)
        .await
        .expect("claim event");
    let selector = HistoricalEvidenceSelector::new(
        HistoricalEvidenceFamily::ChannelIngress,
        event.event_id.as_str(),
    )
    .expect("selector");
    let pending = evidence_store
        .historical_record(&selector)
        .await
        .expect("pending read")
        .expect("pending record");
    assert!(ProofProvenanceContext::from_record(&snapshot(digest("candidate")), &pending).is_err());
    let proof_store = ProofStore::open(temp.path()).expect("open proof store");
    assert!(
        run_historical_observation(
            &ProofHarness::new(proof_store.clone()),
            &evidence_store,
            &snapshot(digest("candidate")),
            &selector,
            [30; 16],
            command(&temp),
        )
        .await
        .is_err()
    );
    assert_eq!(
        std::fs::read_dir(temp.path().join("intents"))
            .expect("intent directory")
            .count(),
        0
    );

    evidence_store
        .append_channel_ingress_receipt(&ChannelIngressReceipt::new(
            event,
            ChannelIngressTerminal::Accepted {
                thread_id: "thread-1".to_string(),
                turn_id: "turn-1".to_string(),
            },
        ))
        .await
        .expect("append accepted receipt");
    let accepted = evidence_store
        .historical_record(&selector)
        .await
        .expect("accepted read")
        .expect("accepted record");
    assert_eq!(accepted.state(), HistoricalEvidenceState::Accepted);

    let migration_snapshot = snapshot(digest("candidate"));
    let context = ProofProvenanceContext::from_record(&migration_snapshot, &accepted)
        .expect("provenance context");
    assert_eq!(
        context.context_sha256().as_str(),
        "56df005879954a2f504870b885ec1f0aa97336848ebed8fc25da65a71f7d6bb4"
    );
    let invocation = ProofInvocation::new_with_provenance(&context, [31; 16], command(&temp))
        .expect("bound invocation");
    assert_eq!(
        invocation.subject().candidate_sha256(),
        migration_snapshot.candidate_sha256()
    );
    assert_eq!(
        invocation.subject().context_sha256(),
        context.context_sha256()
    );
    let generic_same_binding = ProofInvocation::new(
        ProofSubject::new(
            migration_snapshot.candidate_sha256().clone(),
            context.context_sha256().clone(),
        )
        .expect("generic same subject"),
        [31; 16],
        command(&temp),
    );
    assert_ne!(
        invocation.invocation_id(),
        generic_same_binding.invocation_id(),
        "typed origin must domain-separate otherwise identical invocations"
    );

    let other_snapshot = snapshot(digest("other-candidate"));
    let other_context =
        ProofProvenanceContext::from_record(&other_snapshot, &accepted).expect("other context");
    assert_ne!(context.context_sha256(), other_context.context_sha256());
    assert_ne!(
        invocation.invocation_id(),
        ProofInvocation::new_with_provenance(&other_context, [31; 16], command(&temp))
            .expect("other invocation")
            .invocation_id()
    );

    let result = run_historical_observation(
        &ProofHarness::new(proof_store.clone()),
        &evidence_store,
        &migration_snapshot,
        &selector,
        [32; 16],
        command(&temp),
    )
    .await
    .expect("store-resolved proof observation");
    assert_eq!(
        result.receipt.subject().context_sha256(),
        context.context_sha256()
    );
    assert_eq!(
        result.receipt.subject().context_origin(),
        ProofContextOrigin::HistoricalStoreResolved
    );

    let lineage = build_local_proof_provenance_lineage(
        &proof_store,
        &evidence_store,
        &migration_snapshot,
        &selector,
        result.receipt.receipt_id(),
    )
    .await
    .expect("dual-store lineage");
    assert_eq!(lineage.context(), &context);
    assert_eq!(
        lineage.proof_context_origin(),
        ProofContextOrigin::HistoricalStoreResolved
    );
    assert_eq!(lineage.proof_receipt_id(), result.receipt.receipt_id());

    let reopened_evidence = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("reopen evidence store");
    let reopened_proof = ProofStore::open(temp.path()).expect("reopen proof store");
    let restarted = build_local_proof_provenance_lineage(
        &reopened_proof,
        &reopened_evidence,
        &migration_snapshot,
        &selector,
        result.receipt.receipt_id(),
    )
    .await
    .expect("restart lineage");
    assert_eq!(lineage, restarted);

    let generic_subject = ProofSubject::new(
        migration_snapshot.candidate_sha256().clone(),
        context.context_sha256().clone(),
    )
    .expect("generic exact subject");
    let generic_result = ProofHarness::new(proof_store.clone())
        .run(ProofInvocation::new(
            generic_subject,
            [33; 16],
            command(&temp),
        ))
        .await
        .expect("generic exact proof run");
    assert_eq!(
        generic_result.receipt.subject().context_sha256(),
        context.context_sha256()
    );
    assert_eq!(
        generic_result.receipt.subject().context_origin(),
        ProofContextOrigin::CallerSupplied
    );
    assert!(
        build_local_proof_provenance_lineage(
            &proof_store,
            &evidence_store,
            &migration_snapshot,
            &selector,
            generic_result.receipt.receipt_id(),
        )
        .await
        .is_err(),
        "generic caller must remain lineage-ineligible even with the exact context digest"
    );

    assert!(
        build_local_proof_provenance_lineage(
            &proof_store,
            &evidence_store,
            &other_snapshot,
            &selector,
            result.receipt.receipt_id(),
        )
        .await
        .is_err(),
        "snapshot substitution must fail"
    );

    let other_event = ingress_event("other-account", "other-source");
    let other_selector = HistoricalEvidenceSelector::new(
        HistoricalEvidenceFamily::ChannelIngress,
        other_event.event_id.as_str(),
    )
    .expect("other selector");
    evidence_store
        .claim_channel_ingress_event(&other_event)
        .await
        .expect("claim other event");
    evidence_store
        .append_channel_ingress_receipt(&ChannelIngressReceipt::new(
            other_event,
            ChannelIngressTerminal::Accepted {
                thread_id: "thread-1".to_string(),
                turn_id: "turn-2".to_string(),
            },
        ))
        .await
        .expect("append other accepted receipt");
    assert!(
        build_local_proof_provenance_lineage(
            &proof_store,
            &evidence_store,
            &migration_snapshot,
            &other_selector,
            result.receipt.receipt_id(),
        )
        .await
        .is_err(),
        "same-family historical record graft must fail"
    );

    let missing_receipt = ProofReceiptId::parse(format!(
        "proof-receipt:v1:{}",
        digest("missing-receipt").as_str()
    ))
    .expect("missing receipt ID");
    assert!(
        build_local_proof_provenance_lineage(
            &proof_store,
            &evidence_store,
            &migration_snapshot,
            &selector,
            &missing_receipt,
        )
        .await
        .is_err()
    );
    let missing_selector = HistoricalEvidenceSelector::new(
        HistoricalEvidenceFamily::ChannelIngress,
        format!("channel-ingress:v1:{}", digest("missing-history").as_str()),
    )
    .expect("missing history selector");
    assert!(
        build_local_proof_provenance_lineage(
            &proof_store,
            &evidence_store,
            &migration_snapshot,
            &missing_selector,
            result.receipt.receipt_id(),
        )
        .await
        .is_err()
    );
}

#[tokio::test]
async fn rejected_terminal_is_not_eligible_provenance_context() {
    let temp = TempDir::new().expect("temp dir");
    let evidence_store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence store");
    let event = ingress_event("rejected-account", "rejected-source");
    let selector = HistoricalEvidenceSelector::new(
        HistoricalEvidenceFamily::ChannelIngress,
        event.event_id.as_str(),
    )
    .expect("selector");
    evidence_store
        .claim_channel_ingress_event(&event)
        .await
        .expect("claim event");
    evidence_store
        .append_channel_ingress_receipt(&ChannelIngressReceipt::new(
            event,
            ChannelIngressTerminal::Rejected {
                reason_code: "test_rejected".to_string(),
            },
        ))
        .await
        .expect("append rejected receipt");
    let rejected = evidence_store
        .historical_record(&selector)
        .await
        .expect("rejected read")
        .expect("rejected record");
    assert_eq!(rejected.state(), HistoricalEvidenceState::Rejected);
    assert!(
        ProofProvenanceContext::from_record(&snapshot(digest("candidate")), &rejected).is_err()
    );
}

#[tokio::test]
async fn context_deserialization_rejects_every_bound_field_substitution() {
    let temp = TempDir::new().expect("temp dir");
    let evidence_store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence store");
    let event = ingress_event("binding-account", "binding-source");
    let selector = HistoricalEvidenceSelector::new(
        HistoricalEvidenceFamily::ChannelIngress,
        event.event_id.as_str(),
    )
    .expect("selector");
    evidence_store
        .claim_channel_ingress_event(&event)
        .await
        .expect("claim event");
    evidence_store
        .append_channel_ingress_receipt(&ChannelIngressReceipt::new(
            event,
            ChannelIngressTerminal::Accepted {
                thread_id: "thread-1".to_string(),
                turn_id: "turn-1".to_string(),
            },
        ))
        .await
        .expect("append accepted receipt");
    let record = evidence_store
        .historical_record(&selector)
        .await
        .expect("record read")
        .expect("record");
    let context = ProofProvenanceContext::from_record(&snapshot(digest("candidate")), &record)
        .expect("context");
    let canonical = serde_json::to_value(&context).expect("context JSON");
    let substitutions: [(&str, Value); 9] = [
        ("schema_version", json!(2)),
        (
            "snapshot_id",
            json!(format!(
                "migration-snapshot:v1:{}",
                digest("other-snapshot").as_str()
            )),
        ),
        (
            "candidate_sha256",
            json!(digest("other-candidate").as_str()),
        ),
        ("historical_schema_version", json!(2)),
        ("evidence_kind", json!("provider_completed")),
        (
            "historical_record_id",
            json!(format!(
                "channel-ingress:v1:{}",
                digest("other-record").as_str()
            )),
        ),
        (
            "historical_evidence_sha256",
            json!(digest("other-evidence").as_str()),
        ),
        (
            "historical_record_sha256",
            json!(digest("other-record-digest").as_str()),
        ),
        ("context_sha256", json!(digest("other-context").as_str())),
    ];
    for (field, replacement) in substitutions {
        let mut substituted = canonical.clone();
        substituted[field] = replacement;
        assert!(
            serde_json::from_value::<ProofProvenanceContext>(substituted).is_err(),
            "{field} substitution must fail closed"
        );
    }
    let mut unknown = canonical;
    unknown["unknown"] = json!(true);
    assert!(serde_json::from_value::<ProofProvenanceContext>(unknown).is_err());
}

#[tokio::test]
async fn lineage_has_a_fixed_digest_and_rejects_every_top_level_substitution() {
    let temp = TempDir::new().expect("temp dir");
    let evidence_store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence store");
    let event = ingress_event("lineage-account", "lineage-source");
    let selector = HistoricalEvidenceSelector::new(
        HistoricalEvidenceFamily::ChannelIngress,
        event.event_id.as_str(),
    )
    .expect("selector");
    evidence_store
        .claim_channel_ingress_event(&event)
        .await
        .expect("claim event");
    evidence_store
        .append_channel_ingress_receipt(&ChannelIngressReceipt::new(
            event,
            ChannelIngressTerminal::Accepted {
                thread_id: "thread-1".to_string(),
                turn_id: "turn-1".to_string(),
            },
        ))
        .await
        .expect("append accepted receipt");
    let record = evidence_store
        .historical_record(&selector)
        .await
        .expect("record read")
        .expect("record");
    let context = ProofProvenanceContext::from_record(&snapshot(digest("candidate")), &record)
        .expect("context");
    let other_context =
        ProofProvenanceContext::from_record(&snapshot(digest("other-candidate")), &record)
            .expect("other context");
    let invocation_id = ProofInvocationId::parse(format!(
        "proof-invocation:v1:{}",
        digest("fixed-lineage-invocation").as_str()
    ))
    .expect("invocation ID");
    let receipt_id = ProofReceiptId::for_invocation(&invocation_id);
    let mut lineage = LocalProofProvenanceLineage {
        schema_version: super::LOCAL_PROOF_PROVENANCE_LINEAGE_SCHEMA_VERSION,
        context,
        proof_schema_version: crate::PROOF_SCHEMA_VERSION,
        proof_context_origin: ProofContextOrigin::HistoricalStoreResolved,
        proof_invocation_id: invocation_id,
        proof_receipt_id: receipt_id,
        command_binding_sha256: digest("fixed-command-binding"),
        proof_receipt_sha256: digest("fixed-proof-receipt"),
        proof_terminal: ProofTerminal::Completed {
            success: true,
            exit_code: Some(0),
        },
        lineage_sha256: digest("pending-lineage"),
    };
    lineage.lineage_sha256 = lineage.expected_lineage_sha256();
    lineage.validate().expect("valid synthetic lineage");
    assert_eq!(
        lineage.lineage_sha256().as_str(),
        "3fc00e9422974f4e45b676695a8fd594009a21906df4e6d83edd046e041a6a60"
    );

    let canonical = serde_json::to_value(&lineage).expect("lineage JSON");
    assert_eq!(
        serde_json::from_value::<LocalProofProvenanceLineage>(canonical.clone())
            .expect("lineage roundtrip"),
        lineage
    );
    let other_invocation_id = format!(
        "proof-invocation:v1:{}",
        digest("other-invocation").as_str()
    );
    let other_receipt_id = format!("proof-receipt:v1:{}", digest("other-receipt").as_str());
    let substitutions: [(&str, Value); 10] = [
        ("schema_version", json!(2)),
        (
            "context",
            serde_json::to_value(other_context).expect("other context JSON"),
        ),
        ("proof_schema_version", json!(2)),
        ("proof_context_origin", json!("caller_supplied")),
        ("proof_invocation_id", json!(other_invocation_id)),
        ("proof_receipt_id", json!(other_receipt_id)),
        (
            "command_binding_sha256",
            json!(digest("other-command-binding").as_str()),
        ),
        (
            "proof_receipt_sha256",
            json!(digest("other-proof-receipt").as_str()),
        ),
        (
            "proof_terminal",
            json!({"kind":"completed","success":false,"exit_code":1}),
        ),
        ("lineage_sha256", json!(digest("other-lineage").as_str())),
    ];
    for (field, replacement) in substitutions {
        let mut substituted = canonical.clone();
        substituted[field] = replacement;
        assert!(
            serde_json::from_value::<LocalProofProvenanceLineage>(substituted).is_err(),
            "{field} substitution must fail closed"
        );
    }
    let mut unknown = canonical;
    unknown["unknown"] = json!(true);
    assert!(serde_json::from_value::<LocalProofProvenanceLineage>(unknown).is_err());
}

#[cfg(unix)]
#[tokio::test]
async fn failed_proof_receipt_is_not_lineage_eligible() {
    let temp = TempDir::new().expect("temp dir");
    let evidence_store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence store");
    let event = ingress_event("failed-account", "failed-source");
    let selector = HistoricalEvidenceSelector::new(
        HistoricalEvidenceFamily::ChannelIngress,
        event.event_id.as_str(),
    )
    .expect("selector");
    evidence_store
        .claim_channel_ingress_event(&event)
        .await
        .expect("claim event");
    evidence_store
        .append_channel_ingress_receipt(&ChannelIngressReceipt::new(
            event,
            ChannelIngressTerminal::Accepted {
                thread_id: "thread-1".to_string(),
                turn_id: "turn-1".to_string(),
            },
        ))
        .await
        .expect("append accepted receipt");
    let proof_store = ProofStore::open(temp.path()).expect("proof store");
    let result = run_historical_observation(
        &ProofHarness::new(proof_store.clone()),
        &evidence_store,
        &snapshot(digest("candidate")),
        &selector,
        [44; 16],
        failing_command(&temp),
    )
    .await
    .expect("failed command still records a terminal receipt");
    assert!(matches!(
        result.receipt.terminal(),
        ProofTerminal::Completed {
            success: false,
            exit_code: Some(_)
        }
    ));
    assert!(
        build_local_proof_provenance_lineage(
            &proof_store,
            &evidence_store,
            &snapshot(digest("candidate")),
            &selector,
            result.receipt.receipt_id(),
        )
        .await
        .is_err()
    );
}

#[test]
fn successful_terminal_eligibility_is_exhaustively_fixed() {
    assert!(super::successful_terminal(&ProofTerminal::Completed {
        success: true,
        exit_code: Some(0),
    }));
    let ineligible = [
        ProofTerminal::Completed {
            success: false,
            exit_code: Some(1),
        },
        ProofTerminal::Completed {
            success: false,
            exit_code: None,
        },
        ProofTerminal::TimedOut,
        ProofTerminal::OutputLimitExceeded {
            stream: ProofStreamKind::Stdout,
        },
        ProofTerminal::NotStarted {
            reason_code: "not_started".to_string(),
        },
        ProofTerminal::Indeterminate {
            reason_code: "indeterminate".to_string(),
        },
    ];
    for terminal in ineligible {
        assert!(!super::successful_terminal(&terminal));
    }
}

#[test]
fn public_generic_constructor_forces_caller_supplied_origin() {
    let serialized = json!({
        "candidate_sha256": digest("candidate"),
        "context_sha256": digest("context"),
        "context_origin": "historical_store_resolved",
    });
    let forged =
        serde_json::from_value::<ProofSubject>(serialized).expect("self-consistent forged subject");
    assert_eq!(
        forged.context_origin(),
        ProofContextOrigin::HistoricalStoreResolved
    );
    let temp = TempDir::new().expect("temp dir");
    let invocation = ProofInvocation::new(forged, [45; 16], command(&temp));
    assert_eq!(
        invocation.subject().context_origin(),
        ProofContextOrigin::CallerSupplied
    );
}

#[test]
fn evidence_kind_mapping_is_exhaustively_fixed() {
    let fixtures = [
        (
            ProvenanceEvidenceKind::GovernanceHandlerCompletedSuccess,
            HistoricalEvidenceFamily::GovernanceAction,
            HistoricalEvidenceState::HandlerCompletedSuccess,
        ),
        (
            ProvenanceEvidenceKind::ProviderCompleted,
            HistoricalEvidenceFamily::ProviderAttempt,
            HistoricalEvidenceState::Completed,
        ),
        (
            ProvenanceEvidenceKind::ChannelIngressAccepted,
            HistoricalEvidenceFamily::ChannelIngress,
            HistoricalEvidenceState::Accepted,
        ),
    ];
    for (kind, family, state) in fixtures {
        assert_eq!(kind.historical_family(), family);
        assert_eq!(kind.historical_state(), state);
    }
}

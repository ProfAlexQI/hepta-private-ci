use hepta_contracts::Admission;
use hepta_contracts::AdmissionDecision;
use hepta_contracts::AdmissionId;
use hepta_contracts::Authorization;
use hepta_contracts::AuthorizationDecision;
use hepta_contracts::AuthorizationId;
use hepta_contracts::CandidateId;
use hepta_contracts::CandidateRef;
use hepta_contracts::CapabilityDescriptor;
use hepta_contracts::CapabilityId;
use hepta_contracts::CapabilityManifestRef;
use hepta_contracts::CapabilityRequest;
use hepta_contracts::CapabilityRequestId;
use hepta_contracts::CapabilityRequestRef;
use hepta_contracts::ContentHash;
use hepta_contracts::ContractError;
use hepta_contracts::FrozenTurnContext;
use hepta_contracts::JointCandidate;
use hepta_contracts::ObservationFact;
use hepta_contracts::ObservationId;
use hepta_contracts::ObservationSnapshot;
use hepta_contracts::OutcomeReceipt;
use hepta_contracts::OutcomeReceiptParts;
use hepta_contracts::OutcomeStatus;
use hepta_contracts::PreferenceEvidenceId;
use hepta_contracts::PreferenceEvidenceRef;
use hepta_contracts::PreferenceEvidenceSignal;
use hepta_contracts::PreferenceId;
use hepta_contracts::PreferenceState;
use hepta_contracts::PreferenceTransition;
use hepta_contracts::PreferenceTransitionId;
use hepta_contracts::PrincipalId;
use hepta_contracts::ReceiptId;
use hepta_contracts::ReceiptRef;
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;
use hepta_contracts::ToolSchema;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CompleteContractFlow {
    observation: ObservationSnapshot,
    frozen_context: FrozenTurnContext,
    capability: CapabilityDescriptor,
    request: CapabilityRequest,
    candidate: JointCandidate,
    admission: Admission,
    authorization: Authorization,
    outcome: OutcomeReceipt,
    preference_evidence: PreferenceEvidenceRef,
    preference_transition: PreferenceTransition,
}

#[test]
fn complete_architecture_flow_compares_as_whole_objects() -> Result<(), ContractError> {
    let actual = contract_flow()?;
    let expected = contract_flow()?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn tool_schema_preserves_exact_provider_contract_material() {
    let schema = ToolSchema {
        name: "workspace.read".into(),
        description: "Read one workspace file".into(),
        input_schema_json: r#"{"type":"object"}"#.into(),
        output_schema_json: r#"{"type":"string"}"#.into(),
    };
    assert_eq!(
        schema,
        ToolSchema {
            name: "workspace.read".into(),
            description: "Read one workspace file".into(),
            input_schema_json: r#"{"type":"object"}"#.into(),
            output_schema_json: r#"{"type":"string"}"#.into(),
        }
    );
}

#[test]
fn observation_reference_preserves_the_complete_frozen_identity() {
    let snapshot = observation_snapshot();

    assert_eq!(
        snapshot.reference(),
        hepta_contracts::ObservationRef::new(
            ObservationId::new("observation-7"),
            Revision::new(7),
            ContentHash::new("sha256:observation"),
        )
    );
}

#[test]
fn reused_id_with_different_exact_content_is_not_equal() {
    let context = frozen_context(observation_snapshot().reference());
    let original_candidate = CandidateRef::new(
        CandidateId::new("candidate-reused"),
        Revision::new(4),
        ContentHash::new("sha256:candidate-v4"),
        context.clone(),
        ContentHash::new("sha256:action"),
        ContentHash::new("sha256:metacontrol"),
        ContentHash::new("sha256:payload-set"),
    );
    let rebound_candidate = CandidateRef::new(
        CandidateId::new("candidate-reused"),
        Revision::new(5),
        ContentHash::new("sha256:candidate-v5"),
        context,
        ContentHash::new("sha256:action"),
        ContentHash::new("sha256:metacontrol"),
        ContentHash::new("sha256:payload-set"),
    );
    let original_manifest = CapabilityManifestRef::new(
        CapabilityId::new("capability-reused"),
        Revision::new(2),
        ContentHash::new("sha256:manifest-v2"),
        RevisionStamp::new(Revision::new(8), ContentHash::new("sha256:catalog-v8")),
    );
    let rebound_manifest = CapabilityManifestRef::new(
        CapabilityId::new("capability-reused"),
        Revision::new(2),
        ContentHash::new("sha256:manifest-v2"),
        RevisionStamp::new(Revision::new(9), ContentHash::new("sha256:catalog-v9")),
    );
    let original_receipt = ReceiptRef::new(
        ReceiptId::new("receipt-reused"),
        ContentHash::new("sha256:receipt-envelope-a"),
    );
    let rebound_receipt = ReceiptRef::new(
        ReceiptId::new("receipt-reused"),
        ContentHash::new("sha256:receipt-envelope-b"),
    );

    assert_ne!(original_candidate, rebound_candidate);
    assert_ne!(original_manifest, rebound_manifest);
    assert_ne!(original_receipt, rebound_receipt);
}

#[test]
fn commit_time_reauthorization_makes_context_drift_visible() -> Result<(), ContractError> {
    let flow = contract_flow()?;
    let drifted_context = FrozenTurnContext::new(
        flow.frozen_context.observation().clone(),
        flow.frozen_context.state().clone(),
        RevisionStamp::new(
            Revision::new(6),
            ContentHash::new("sha256:policy-v6-after-approval"),
        ),
        flow.frozen_context.capability_catalog().clone(),
        flow.frozen_context.preference().clone(),
    );
    let reauthorization = Authorization::try_new_commit_time(
        AuthorizationId::new("authorization-after-wait"),
        Revision::new(2),
        ContentHash::new("sha256:authorization-after-wait"),
        &flow.admission,
        drifted_context,
        PrincipalId::new("safety-kernel"),
        AuthorizationDecision::Denied {
            reason_code: "policy_revision_drift".into(),
        },
    )?;

    assert_eq!(reauthorization.candidate(), flow.admission.candidate());
    assert_eq!(
        reauthorization.payload_set_hash(),
        flow.admission.payload_set_hash()
    );
    assert_ne!(reauthorization.current_context(), flow.admission.context());
    assert_ne!(reauthorization.policy(), flow.admission.policy());
    Ok(())
}

#[test]
fn outcome_and_preference_transition_bind_exact_receipt_envelope() -> Result<(), ContractError> {
    let flow = contract_flow()?;
    let rebound = ReceiptRef::new(
        flow.outcome.id().clone(),
        ContentHash::new("sha256:different-receipt-envelope"),
    );

    assert_eq!(
        flow.preference_transition.caused_by(),
        &flow.outcome.reference()
    );
    assert_eq!(
        flow.preference_transition.evidence(),
        &flow.preference_evidence
    );
    assert_ne!(flow.preference_transition.caused_by(), &rebound);
    Ok(())
}

#[test]
fn outcome_receipt_rehydrates_without_recreating_execution_authority() -> Result<(), ContractError>
{
    let flow = contract_flow()?;
    let parts = flow.outcome.rehydration_parts();

    assert_eq!(parts.authorization(), flow.outcome.authorization());
    assert_eq!(
        OutcomeReceipt::try_rehydrate(parts)?,
        flow.outcome,
        "rehydration must preserve every observable receipt field"
    );
    Ok(())
}

#[test]
fn outcome_receipt_rehydration_rejects_payload_and_terminal_drift() -> Result<(), ContractError> {
    let flow = contract_flow()?;
    let parts = flow.outcome.rehydration_parts();
    let mismatched_payload = OutcomeReceiptParts::new(
        parts.id().clone(),
        parts.receipt_hash().clone(),
        parts.candidate().clone(),
        parts.authorization().clone(),
        ContentHash::new("sha256:different-payload-set"),
        parts.executed_by().clone(),
        parts.outcome_hash().clone(),
        parts.status().clone(),
    );
    assert_eq!(
        assert_contract_error(
            OutcomeReceipt::try_rehydrate(mismatched_payload),
            "persisted payload drift must fail receipt rehydration",
        ),
        ContractError::OutcomePayloadSetMismatch
    );

    for (status, expected) in [
        (
            OutcomeStatus::Failed {
                error_code: "  ".into(),
            },
            ContractError::OutcomeFailureCodeEmpty,
        ),
        (
            OutcomeStatus::Cancelled {
                reason_code: String::new(),
            },
            ContractError::OutcomeCancellationCodeEmpty,
        ),
    ] {
        let invalid_terminal = OutcomeReceiptParts::new(
            parts.id().clone(),
            parts.receipt_hash().clone(),
            parts.candidate().clone(),
            parts.authorization().clone(),
            parts.payload_set_hash().clone(),
            parts.executed_by().clone(),
            parts.outcome_hash().clone(),
            status,
        );
        assert_eq!(
            assert_contract_error(
                OutcomeReceipt::try_rehydrate(invalid_terminal),
                "invalid persisted terminal status must fail receipt rehydration",
            ),
            expected
        );
    }
    Ok(())
}

#[test]
fn preference_transition_cannot_independently_rebind_evidence_dimensions()
-> Result<(), ContractError> {
    let flow = contract_flow()?;
    let evidence = flow.preference_transition.evidence();

    assert_eq!(flow.preference_transition.subject(), evidence.subject());
    assert_eq!(
        flow.preference_transition.preference(),
        evidence.preference()
    );
    assert_eq!(flow.preference_transition.caused_by(), evidence.receipt());
    assert_eq!(
        flow.preference_transition.evidence().signal(),
        evidence.signal()
    );

    for rebound in [
        preference_evidence(
            evidence.receipt().clone(),
            PrincipalId::new("different-subject"),
            evidence.preference().clone(),
            evidence.signal(),
        ),
        preference_evidence(
            evidence.receipt().clone(),
            evidence.subject().clone(),
            PreferenceId::new("different-preference"),
            evidence.signal(),
        ),
        preference_evidence(
            ReceiptRef::new(
                ReceiptId::new("different-receipt"),
                ContentHash::new("sha256:different-receipt"),
            ),
            evidence.subject().clone(),
            evidence.preference().clone(),
            evidence.signal(),
        ),
        preference_evidence(
            evidence.receipt().clone(),
            evidence.subject().clone(),
            evidence.preference().clone(),
            PreferenceEvidenceSignal::Rejected,
        ),
    ] {
        assert_ne!(evidence, &rebound);
        let transition = PreferenceTransition::try_new(
            PreferenceTransitionId::new("rebound-transition"),
            &rebound,
            PreferenceState::new(
                Revision::new(10),
                ContentHash::new("sha256:preference-before"),
            ),
            PreferenceState::new(
                Revision::new(11),
                ContentHash::new("sha256:preference-after"),
            ),
        )?;
        assert_eq!(transition.subject(), rebound.subject());
        assert_eq!(transition.preference(), rebound.preference());
        assert_eq!(transition.caused_by(), rebound.receipt());
        assert_eq!(transition.evidence().signal(), rebound.signal());
    }
    Ok(())
}

#[test]
fn rejected_admission_and_denied_authorization_cannot_reach_execution() -> Result<(), ContractError>
{
    let flow = contract_flow()?;
    let rejected_admission = Admission::new(
        AdmissionId::new("admission-rejected"),
        Revision::new(2),
        ContentHash::new("sha256:admission-rejected"),
        &flow.candidate,
        PrincipalId::new("safety-kernel"),
        AdmissionDecision::Rejected {
            reason_code: "unsafe_payload".into(),
        },
    );

    let authorization_error = assert_contract_error(
        Authorization::try_new_commit_time(
            AuthorizationId::new("authorization-invalid"),
            Revision::new(2),
            ContentHash::new("sha256:authorization-invalid"),
            &rejected_admission,
            flow.frozen_context.clone(),
            PrincipalId::new("safety-kernel"),
            AuthorizationDecision::Denied {
                reason_code: "not_admitted".into(),
            },
        ),
        "a rejected admission cannot reach commit-time authorization",
    );
    assert_eq!(authorization_error, ContractError::AdmissionNotAdmitted);

    let denied_authorization = Authorization::try_new_commit_time(
        AuthorizationId::new("authorization-denied"),
        Revision::new(2),
        ContentHash::new("sha256:authorization-denied"),
        &flow.admission,
        flow.frozen_context,
        PrincipalId::new("safety-kernel"),
        AuthorizationDecision::Denied {
            reason_code: "approval_expired".into(),
        },
    )?;
    let outcome_error = assert_contract_error(
        OutcomeReceipt::try_new(
            ReceiptId::new("receipt-invalid"),
            ContentHash::new("sha256:receipt-invalid"),
            &denied_authorization,
            PrincipalId::new("builder"),
            ContentHash::new("sha256:outcome-invalid"),
            OutcomeStatus::Succeeded,
        ),
        "a denied authorization cannot produce an execution receipt",
    );
    assert_eq!(outcome_error, ContractError::AuthorizationNotAuthorized);
    Ok(())
}

#[test]
fn catalog_and_context_mismatches_are_rejected_during_assembly() -> Result<(), ContractError> {
    let flow = contract_flow()?;
    let outside_manifest = CapabilityManifestRef::new(
        CapabilityId::new("capability-outside-catalog"),
        Revision::new(3),
        ContentHash::new("sha256:outside-manifest"),
        RevisionStamp::new(
            Revision::new(99),
            ContentHash::new("sha256:outside-catalog"),
        ),
    );
    let request_error = assert_contract_error(
        CapabilityRequest::try_new(
            CapabilityRequestId::new("request-outside-catalog"),
            ContentHash::new("sha256:request-outside-catalog"),
            outside_manifest.clone(),
            PrincipalId::new("planner"),
            flow.frozen_context.clone(),
            ContentHash::new("sha256:payload-outside-catalog"),
        ),
        "a request manifest must belong to the frozen catalog",
    );
    assert_eq!(request_error, ContractError::CapabilityCatalogMismatch);

    let drifted_context = FrozenTurnContext::new(
        flow.frozen_context.observation().clone(),
        RevisionStamp::new(
            Revision::new(4),
            ContentHash::new("sha256:state-v4-after-request"),
        ),
        flow.frozen_context.policy().clone(),
        flow.frozen_context.capability_catalog().clone(),
        flow.frozen_context.preference().clone(),
    );
    let context_error = assert_contract_error(
        JointCandidate::try_new(
            CandidateId::new("candidate-context-mismatch"),
            Revision::new(5),
            ContentHash::new("sha256:candidate-context-mismatch"),
            drifted_context,
            ContentHash::new("sha256:action"),
            ContentHash::new("sha256:metacontrol"),
            ContentHash::new("sha256:payload-set"),
            vec![PrincipalId::new("planner")],
            vec![flow.request.reference()],
        ),
        "candidate requests must share the candidate context",
    );
    assert_eq!(
        context_error,
        ContractError::CandidateRequestContextMismatch { request_index: 0 }
    );

    let fabricated_request = CapabilityRequestRef::new(
        CapabilityRequestId::new("request-fabricated"),
        ContentHash::new("sha256:request-fabricated"),
        outside_manifest,
        PrincipalId::new("planner"),
        flow.frozen_context.clone(),
        ContentHash::new("sha256:payload-fabricated"),
    );
    let catalog_error = assert_contract_error(
        JointCandidate::try_new(
            CandidateId::new("candidate-catalog-mismatch"),
            Revision::new(5),
            ContentHash::new("sha256:candidate-catalog-mismatch"),
            flow.frozen_context,
            ContentHash::new("sha256:action"),
            ContentHash::new("sha256:metacontrol"),
            ContentHash::new("sha256:payload-set"),
            vec![PrincipalId::new("planner")],
            vec![fabricated_request],
        ),
        "candidate request manifests must belong to the frozen catalog",
    );
    assert_eq!(
        catalog_error,
        ContractError::CandidateRequestCatalogMismatch { request_index: 0 }
    );

    let requester_error = assert_contract_error(
        JointCandidate::try_new(
            CandidateId::new("candidate-requester-mismatch"),
            Revision::new(5),
            ContentHash::new("sha256:candidate-requester-mismatch"),
            flow.request.context().clone(),
            ContentHash::new("sha256:action"),
            ContentHash::new("sha256:metacontrol"),
            flow.request.payload_hash().clone(),
            vec![PrincipalId::new("different-planner")],
            vec![flow.request.reference()],
        ),
        "candidate contributors must include every request principal",
    );
    assert_eq!(
        requester_error,
        ContractError::CandidateRequestRequesterMissing { request_index: 0 }
    );
    Ok(())
}

#[test]
fn preference_cas_requires_exact_successor_and_reports_overflow() {
    let expected = Revision::new(10);
    for (transition_id, committed) in [
        ("preference-transition-same", Revision::new(10)),
        ("preference-transition-older", Revision::new(9)),
        ("preference-transition-skipped", Revision::new(12)),
    ] {
        let evidence = preference_evidence(
            ReceiptRef::new(
                ReceiptId::new("receipt-preference-evidence"),
                ContentHash::new("sha256:receipt-preference-evidence"),
            ),
            PrincipalId::new("operator"),
            PreferenceId::new("prefer-local-builds"),
            PreferenceEvidenceSignal::Accepted,
        );
        let error = assert_contract_error(
            PreferenceTransition::try_new(
                PreferenceTransitionId::new(transition_id),
                &evidence,
                PreferenceState::new(expected, ContentHash::new("sha256:preference-before")),
                PreferenceState::new(committed, ContentHash::new("sha256:preference-invalid")),
            ),
            "committed preference revision must be the exact successor",
        );

        assert_eq!(
            error,
            ContractError::PreferenceRevisionNotAdvanced {
                expected,
                committed,
            }
        );
    }

    let evidence = preference_evidence(
        ReceiptRef::new(
            ReceiptId::new("receipt-preference-overflow"),
            ContentHash::new("sha256:receipt-preference-overflow"),
        ),
        PrincipalId::new("operator"),
        PreferenceId::new("prefer-local-builds"),
        PreferenceEvidenceSignal::Accepted,
    );
    let overflow = assert_contract_error(
        PreferenceTransition::try_new(
            PreferenceTransitionId::new("preference-transition-overflow"),
            &evidence,
            PreferenceState::new(
                Revision::new(u64::MAX),
                ContentHash::new("sha256:preference-max"),
            ),
            PreferenceState::new(
                Revision::new(u64::MAX),
                ContentHash::new("sha256:preference-overflow"),
            ),
        ),
        "maximum preference revision must fail with typed overflow",
    );
    assert_eq!(
        overflow,
        ContractError::PreferenceRevisionOverflow {
            expected: Revision::new(u64::MAX)
        }
    );
}

fn contract_flow() -> Result<CompleteContractFlow, ContractError> {
    let observation = observation_snapshot();
    let frozen_context = frozen_context(observation.reference());
    let capability = CapabilityDescriptor::new(
        CapabilityId::new("capability-build"),
        Revision::new(2),
        ContentHash::new("sha256:capability-manifest"),
        frozen_context.capability_catalog().clone(),
        PrincipalId::new("builder"),
        "workspace.build",
    );
    let request = CapabilityRequest::try_new(
        CapabilityRequestId::new("request-9"),
        ContentHash::new("sha256:request-envelope"),
        capability.reference(),
        PrincipalId::new("planner"),
        frozen_context.clone(),
        ContentHash::new("sha256:request-payload"),
    )?;
    let candidate = JointCandidate::try_new(
        CandidateId::new("candidate-4"),
        Revision::new(4),
        ContentHash::new("sha256:candidate"),
        frozen_context.clone(),
        ContentHash::new("sha256:action-a-t"),
        ContentHash::new("sha256:metacontrol-theta-t"),
        ContentHash::new("sha256:ordered-payload-set"),
        vec![PrincipalId::new("planner"), PrincipalId::new("builder")],
        vec![request.reference()],
    )?;
    let admission = Admission::new(
        AdmissionId::new("admission-4"),
        Revision::new(1),
        ContentHash::new("sha256:admission-record"),
        &candidate,
        PrincipalId::new("safety-kernel"),
        AdmissionDecision::Admitted,
    );
    let authorization = Authorization::try_new_commit_time(
        AuthorizationId::new("authorization-4"),
        Revision::new(1),
        ContentHash::new("sha256:commit-authorization-record"),
        &admission,
        frozen_context.clone(),
        PrincipalId::new("safety-kernel"),
        AuthorizationDecision::Authorized {
            scope_hash: ContentHash::new("sha256:execution-scope"),
        },
    )?;
    let outcome = OutcomeReceipt::try_new(
        ReceiptId::new("receipt-4"),
        ContentHash::new("sha256:complete-receipt-envelope"),
        &authorization,
        PrincipalId::new("builder"),
        ContentHash::new("sha256:outcome"),
        OutcomeStatus::Succeeded,
    )?;
    let preference_evidence = preference_evidence(
        outcome.reference(),
        PrincipalId::new("operator"),
        PreferenceId::new("prefer-local-builds"),
        PreferenceEvidenceSignal::Accepted,
    );
    let preference_transition = PreferenceTransition::try_new(
        PreferenceTransitionId::new("preference-transition-4"),
        &preference_evidence,
        PreferenceState::new(
            Revision::new(10),
            ContentHash::new("sha256:preference-before"),
        ),
        PreferenceState::new(
            Revision::new(11),
            ContentHash::new("sha256:preference-after"),
        ),
    )?;

    Ok(CompleteContractFlow {
        observation,
        frozen_context,
        capability,
        request,
        candidate,
        admission,
        authorization,
        outcome,
        preference_evidence,
        preference_transition,
    })
}

fn preference_evidence(
    receipt: ReceiptRef,
    subject: PrincipalId,
    preference: PreferenceId,
    signal: PreferenceEvidenceSignal,
) -> PreferenceEvidenceRef {
    PreferenceEvidenceRef::new(
        PreferenceEvidenceId::new("preference-evidence-4"),
        ContentHash::new("sha256:preference-evidence"),
        signal,
        receipt,
        ContentHash::new("sha256:feedback-session-binding"),
        subject,
        preference,
        ContentHash::new("sha256:preference-target-binding"),
    )
}

fn assert_contract_error<T: std::fmt::Debug>(
    result: Result<T, ContractError>,
    message: &str,
) -> ContractError {
    match result {
        Ok(value) => panic!("{message}: unexpectedly constructed {value:?}"),
        Err(error) => error,
    }
}

fn observation_snapshot() -> ObservationSnapshot {
    ObservationSnapshot::new(
        ObservationId::new("observation-7"),
        Revision::new(7),
        ContentHash::new("sha256:observation"),
        PrincipalId::new("observer"),
        vec![
            ObservationFact::new("queue.depth", "3"),
            ObservationFact::new("workspace.clean", "true"),
        ],
    )
}

fn frozen_context(observation: hepta_contracts::ObservationRef) -> FrozenTurnContext {
    FrozenTurnContext::new(
        observation,
        RevisionStamp::new(Revision::new(3), ContentHash::new("sha256:state-v3")),
        RevisionStamp::new(Revision::new(5), ContentHash::new("sha256:policy-v5")),
        RevisionStamp::new(
            Revision::new(8),
            ContentHash::new("sha256:capability-catalog-v8"),
        ),
        RevisionStamp::new(Revision::new(10), ContentHash::new("sha256:preference-v10")),
    )
}

use crate::RuntimeKernel;
use crate::SafetyGateClient;
use crate::runtime_kernel::approval_state::ApprovalState;
use crate::runtime_kernel::approval_state::CandidateApproval;
use crate::runtime_kernel::approval_state::ExactApprovalMaterial;
use hepta_contracts::CapabilityDescriptor;
use hepta_contracts::ContentHash;
use hepta_contracts::PrincipalId;
use hepta_core::CorrelationId;
use hepta_core::PolicyEvaluationContext;
use hepta_core::SessionId;

fn material(runtime: &RuntimeKernel, tool_name: &str) -> ExactApprovalMaterial {
    let arguments = match tool_name {
        "echo" => r#"{"text":"descriptor"}"#,
        other => panic!("unsupported descriptor test tool: {other}"),
    };
    let active_model = runtime.model_selection().expect("model").active;
    let decision = runtime
        .policy
        .evaluate_with_match(PolicyEvaluationContext {
            session_id: Some(SessionId("session-main".into())),
            model: Some(active_model.clone()),
            tool_name: tool_name.into(),
            risk_tier: runtime.tools.risk_tier(tool_name).expect("risk"),
        })
        .expect("exact policy decision");
    SafetyGateClient::prepare_candidate(
        runtime,
        "session-main",
        &active_model,
        tool_name,
        arguments,
        &decision,
    )
    .expect("exact material")
}

fn descriptor_with(
    material: &ExactApprovalMaterial,
    provider: &str,
    operation: &str,
    manifest_hash: ContentHash,
) -> CapabilityDescriptor {
    let descriptor = &material.capability_descriptor;
    CapabilityDescriptor::new(
        descriptor.id().clone(),
        descriptor.revision(),
        manifest_hash,
        descriptor.catalog().clone(),
        PrincipalId::new(provider),
        operation,
    )
}

fn authorization_error(
    runtime: &RuntimeKernel,
    approved: &ExactApprovalMaterial,
    presented: &ExactApprovalMaterial,
) -> crate::HeptaError {
    let epoch = runtime
        .capture_execution_epoch("session-main")
        .expect("execution epoch");
    let lease = runtime.begin_execution_lease(epoch).expect("lease");
    let lease = lease
        .bind_tool_resources(
            runtime,
            "session-main",
            &presented.tool_name,
            &presented.canonical_arguments,
        )
        .expect("resource-bound lease");
    SafetyGateClient::authorize_execution_without_grant(
        runtime,
        &SessionId("session-main".into()),
        &CorrelationId("capability-descriptor".into()),
        approved,
        presented,
        lease,
    )
    .expect_err("descriptor substitution must fail")
}

#[test]
fn architecture_v2_capability_descriptor_is_production_registry_manifest_data() {
    let runtime = RuntimeKernel::new();
    let descriptors = runtime.tool_descriptors();
    let echo = descriptors
        .iter()
        .find(|descriptor| descriptor.name == "echo")
        .expect("echo descriptor");
    assert_eq!(echo.executor_provider, "hepta-runtime-builtin");
    assert_eq!(echo.operation, "echo");
    let native = descriptors
        .iter()
        .find(|descriptor| descriptor.name == "sessions_list")
        .expect("native descriptor");
    assert_eq!(native.executor_provider, "openclaw-native");
    assert_eq!(native.operation, "sessions_list");

    let exact = material(&runtime, "echo");
    let [request] = exact.candidate.capability_requests() else {
        panic!("one capability request");
    };
    assert_eq!(
        request.capability(),
        &exact.capability_descriptor.reference()
    );
    assert_eq!(
        exact.capability_descriptor.provider().as_str(),
        echo.executor_provider
    );
    assert_eq!(exact.capability_descriptor.operation(), echo.operation);
}

#[test]
fn architecture_v2_capability_descriptor_provider_and_operation_drift_fail_closed() {
    for (provider, operation) in [
        ("substituted-provider", "echo"),
        ("hepta-runtime-builtin", "substituted-operation"),
    ] {
        let mut runtime = RuntimeKernel::new();
        let approved = material(&runtime, "echo");
        let presented = approved.clone();
        runtime
            .tools
            .override_executor_binding("echo", provider, operation);
        let error = authorization_error(&runtime, &approved, &presented);
        assert!(
            error
                .0
                .contains("capability catalog changed after candidate freeze"),
            "unexpected drift error: {}",
            error.0
        );
        assert_eq!(runtime.outcome_record_count().expect("outcome count"), 0);
    }
}

#[test]
fn architecture_v2_capability_descriptor_candidate_manifest_mismatch_fails_closed() {
    let runtime = RuntimeKernel::new();
    let mut approved = material(&runtime, "echo");
    approved.capability_descriptor = descriptor_with(
        &approved,
        approved.capability_descriptor.provider().as_str(),
        approved.capability_descriptor.operation(),
        ContentHash::new("sha256:substituted-manifest"),
    );
    let presented = approved.clone();
    let error = authorization_error(&runtime, &approved, &presented);
    assert!(
        error.0.contains("approved_manifest_reference_mismatch"),
        "unexpected manifest error: {}",
        error.0
    );
}

#[test]
fn architecture_v2_capability_descriptor_is_part_of_exact_approval_binding() {
    let runtime = RuntimeKernel::new();
    let approved = material(&runtime, "echo");
    let mut substituted = approved.clone();
    substituted.capability_descriptor = descriptor_with(
        &substituted,
        "substituted-provider",
        substituted.capability_descriptor.operation(),
        substituted.capability_descriptor.manifest_hash().clone(),
    );

    let direct_error = SafetyGateClient::authorize_without_grant(&approved, &substituted)
        .expect_err("descriptor substitution must not reuse candidate admission");
    assert!(direct_error.0.contains("approval_descriptor_substitution"));

    let mut approvals = ApprovalState::default();
    let binding = approved.binding_hash().as_str().to_string();
    approvals.remember_pending_exact("session-main", approved);
    approvals
        .approve_candidate("session-main", &binding)
        .expect("exact candidate approval");
    assert!(matches!(
        approvals.candidate_approval("session-main", &substituted),
        CandidateApproval::Missing
    ));
}

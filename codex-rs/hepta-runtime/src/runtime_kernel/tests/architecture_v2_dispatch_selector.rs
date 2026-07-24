use crate::ExecutionBus;
use crate::OutcomeRecorder;
use crate::RuntimeKernel;
use crate::SafetyGateClient;
use crate::runtime_kernel::execution_attempt::AuthorizedToolExecution;
use crate::runtime_kernel::execution_attempt::ExecutorBinding;
use crate::runtime_kernel::execution_bus::CapturedDispatchTerminal;
use crate::runtime_kernel::execution_bus::CapturedToolExecution;
use hepta_core::CorrelationId;
use hepta_core::PolicyEvaluationContext;
use hepta_core::SessionId;
use hepta_memory::OutcomeRecord;
use serde_json::Value;

fn authorize_echo(
    runtime: &RuntimeKernel,
    correlation: &str,
) -> (AuthorizedToolExecution, String, ExecutorBinding) {
    let session = SessionId("session-main".into());
    let model = runtime.model_selection().expect("model").active;
    let decision = runtime
        .policy
        .evaluate_with_match(PolicyEvaluationContext {
            session_id: Some(session.clone()),
            model: Some(model.clone()),
            tool_name: "echo".into(),
            risk_tier: runtime.tools.risk_tier("echo").expect("risk"),
        })
        .expect("exact policy");
    let material = SafetyGateClient::prepare_candidate(
        runtime,
        &session.0,
        &model,
        "echo",
        r#"{"text":"dispatch-selector"}"#,
        &decision,
    )
    .expect("candidate");
    let epoch = runtime.capture_execution_epoch(&session.0).expect("epoch");
    let lease = runtime.begin_execution_lease(epoch).expect("lease");
    let lease = lease
        .bind_tool_resources(runtime, &session.0, "echo", &material.canonical_arguments)
        .expect("resource binding");
    let execution = SafetyGateClient::authorize_execution_without_grant(
        runtime,
        &session,
        &CorrelationId(correlation.into()),
        &material,
        &material,
        lease,
    )
    .expect("authorization");
    let attempt_id = execution.attempt_id().to_string();
    let executor = execution.executor().clone();
    (execution, attempt_id, executor)
}

fn finalize(
    runtime: &RuntimeKernel,
    captured: &mut CapturedToolExecution<'_>,
    attempt_id: &str,
) -> OutcomeRecord {
    captured.capture_write_transaction();
    OutcomeRecorder::new(runtime)
        .finalize_tool_dispatch(captured)
        .expect("receipt");
    runtime
        .outcome_record_by_attempt(attempt_id)
        .expect("outcome read")
        .expect("outcome record")
}

fn evidence_field(record: &OutcomeRecord, name: &str) -> Value {
    let envelope: Value =
        serde_json::from_str(record.canonical_evidence()).expect("canonical evidence");
    envelope["fields"]
        .as_array()
        .expect("fields")
        .iter()
        .find_map(|field| {
            let pair = field.as_array()?;
            (pair.first()?.as_str()? == name)
                .then(|| pair.get(1))
                .flatten()
        })
        .cloned()
        .expect("named evidence")
}

fn assert_exact_executor(record: &OutcomeRecord, expected: &ExecutorBinding) {
    assert_eq!(record.receipt().executed_by(), expected.principal());
    assert_eq!(
        evidence_field(record, "executor.provider").as_str(),
        Some(expected.provider())
    );
    assert_eq!(
        evidence_field(record, "tool.operation").as_str(),
        Some(expected.operation())
    );
    assert_eq!(
        evidence_field(record, "executor.manifest_hash").as_str(),
        Some(expected.manifest_hash().as_str())
    );
}

#[tokio::test]
async fn architecture_v2_dispatch_selector_provider_and_operation_drift_never_invoke() {
    for (provider, operation) in [
        ("replaced-provider", "echo"),
        ("hepta-runtime-builtin", "replaced-operation"),
    ] {
        let mut runtime = RuntimeKernel::new();
        let (execution, attempt_id, expected) =
            authorize_echo(&runtime, "selector-provider-operation");
        runtime
            .tools
            .override_executor_binding("echo", provider, operation);

        let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
        match captured.terminal() {
            CapturedDispatchTerminal::DispatchBlocked(reason) => {
                assert!(reason.contains("dispatch selector denied"));
            }
            terminal => panic!("registry drift must block dispatch: {terminal:?}"),
        }
        assert_eq!(runtime.tools.provider_invocation_count("echo"), 0);
        let record = finalize(&runtime, &mut captured, &attempt_id);
        assert_exact_executor(&record, &expected);
    }
}

#[tokio::test]
async fn architecture_v2_dispatch_selector_manifest_drift_never_invokes_provider() {
    let mut runtime = RuntimeKernel::new();
    let (execution, attempt_id, expected) = authorize_echo(&runtime, "selector-manifest");
    runtime
        .tools
        .override_manifest_description("echo", "replacement manifest");

    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    match captured.terminal() {
        CapturedDispatchTerminal::DispatchBlocked(reason) => {
            assert!(reason.contains("registry_descriptor_drift"));
        }
        terminal => panic!("manifest drift must block dispatch: {terminal:?}"),
    }
    assert_eq!(runtime.tools.provider_invocation_count("echo"), 0);
    let record = finalize(&runtime, &mut captured, &attempt_id);
    assert_exact_executor(&record, &expected);
}

#[tokio::test]
async fn architecture_v2_dispatch_selector_receipt_preserves_exact_executor_binding() {
    let runtime = RuntimeKernel::new();
    let (execution, attempt_id, expected) = authorize_echo(&runtime, "selector-success");
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    assert!(matches!(
        captured.terminal(),
        CapturedDispatchTerminal::Succeeded
    ));
    assert_eq!(runtime.tools.provider_invocation_count("echo"), 1);
    let record = finalize(&runtime, &mut captured, &attempt_id);
    assert_exact_executor(&record, &expected);
}

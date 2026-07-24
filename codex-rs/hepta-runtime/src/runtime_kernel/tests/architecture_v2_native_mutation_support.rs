use std::fs;
use std::path::Path;

use crate::ExecutionBus;
use crate::HeptaError;
use crate::OutcomeRecorder;
use crate::RuntimeKernel;
use crate::SafetyGateClient;
use crate::runtime_kernel::approval_state::ExactApprovalMaterial;
use crate::runtime_kernel::execution_attempt::AuthorizedToolExecution;
use crate::runtime_kernel::execution_bus::CapturedDispatchTerminal;
use crate::runtime_kernel::execution_bus::CapturedTransaction;
use hepta_core::ApprovalRequirement;
use hepta_core::CorrelationId;
use hepta_core::FilesystemScope;
use hepta_core::PolicyEvaluationContext;
use hepta_core::SessionId;
use hepta_core::ToolResult;
use hepta_core::WritePathScope;
use serde_json::json;

fn allow_and_material(
    runtime: &RuntimeKernel,
    session: &str,
    tool: &str,
    arguments: &str,
) -> ExactApprovalMaterial {
    let active_model = runtime.model_selection().expect("active model").active;
    let context = || PolicyEvaluationContext {
        session_id: Some(SessionId(session.into())),
        model: Some(active_model.clone()),
        tool_name: tool.into(),
        risk_tier: runtime.tools.risk_tier(tool).expect("risk tier"),
    };
    if runtime
        .policy
        .evaluate_with_match(context())
        .expect("current policy")
        .requirement
        != ApprovalRequirement::None
    {
        runtime
            .add_policy_rule(
                None,
                None,
                Some(tool),
                None,
                ApprovalRequirement::None,
                Some("native mutation reservation regression"),
            )
            .expect("allow native mutation");
    }
    let decision = runtime
        .policy
        .evaluate_with_match(context())
        .expect("exact policy");
    SafetyGateClient::prepare_candidate(runtime, session, &active_model, tool, arguments, &decision)
        .expect("candidate")
}

fn authorize(
    runtime: &RuntimeKernel,
    session: &str,
    tool: &str,
    arguments: &str,
    correlation: &str,
) -> Result<AuthorizedToolExecution, HeptaError> {
    let material = allow_and_material(runtime, session, tool, arguments);
    let epoch = runtime.capture_execution_epoch(session)?;
    let lease = runtime.begin_execution_lease(epoch)?;
    let lease = lease.bind_tool_resources(runtime, session, tool, &material.canonical_arguments)?;
    SafetyGateClient::authorize_execution_without_grant(
        runtime,
        &SessionId(session.into()),
        &CorrelationId(correlation.into()),
        &material,
        &material,
        lease,
    )
}

fn mutation_arguments(tool: &str, path: &str) -> String {
    match tool {
        "write" => json!({"path":path,"content":"alpha\n"}).to_string(),
        "edit" => {
            json!({"path":path,"edits":[{"oldText":"alpha\n","newText":"beta\n"}]}).to_string()
        }
        "apply_patch" => json!({
            "input":format!(
                "*** Begin Patch\n*** Update File: {path}\n@@\n-alpha\n+beta\n*** End Patch"
            )
        })
        .to_string(),
        "tts" => json!({"text":"sealed speech","path":path,"dryRun":true}).to_string(),
        _ => panic!("unsupported mutation tool {tool}"),
    }
}

fn active_reservations(runtime: &RuntimeKernel) -> usize {
    runtime
        .write_lock_state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .active_target_reservations
        .len()
}

fn remove_recorded_checkpoints(runtime: &RuntimeKernel, target: &Path) {
    let target_path = target.display().to_string();
    for transaction in runtime
        .write_transactions(Some(&target_path))
        .expect("write transactions")
        .transactions
    {
        if transaction.target_path == target_path
            && let Some(checkpoint) = transaction.rollback_checkpoint_path
        {
            fs::remove_file(checkpoint).expect("remove test rollback checkpoint");
        }
    }
}

async fn execute_success(
    runtime: &RuntimeKernel,
    execution: AuthorizedToolExecution,
) -> ToolResult {
    let mut captured = ExecutionBus::new(runtime).dispatch(execution).await;
    assert!(matches!(
        captured.terminal(),
        CapturedDispatchTerminal::Succeeded
    ));
    captured.capture_write_transaction();
    assert!(
        matches!(
            captured.transaction(),
            CapturedTransaction::Recorded { .. } | CapturedTransaction::Preview
        ),
        "native mutation must produce exact transaction evidence: {:?}",
        captured.transaction()
    );
    assert!(
        matches!(captured.terminal(), CapturedDispatchTerminal::Succeeded),
        "transaction capture must preserve successful terminal state: {:?}",
        captured.terminal()
    );
    let result = captured.tool_result().expect("tool result").clone();
    OutcomeRecorder::new(runtime)
        .finalize_tool_dispatch(&mut captured)
        .expect("terminal receipt");
    result
}

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;

use hepta_core::ApprovalRequirement;
use hepta_memory::DurableIntegrityKey;

use super::*;

struct RestoreDirectoryPermissions(PathBuf);

impl Drop for RestoreDirectoryPermissions {
    fn drop(&mut self) {
        let _ = fs::set_permissions(&self.0, fs::Permissions::from_mode(0o700));
    }
}

#[test]
fn reconciliation_finalizes_exact_linkage_without_provider_or_terminal_replay() -> Result<()> {
    let root = tempfile::tempdir()?;
    let kernel = RuntimeKernel::bootstrap_with_durable_outcomes(
        root.path().join("outcomes.sqlite3"),
        DurableIntegrityKey::from_bytes([19; 32]),
    )?;
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let artifact_root = tempfile::Builder::new()
        .prefix(".hepta-operator-reconciliation-")
        .tempdir_in(workspace.join("artifacts"))?;
    let artifact_path = artifact_root.path().join("note.txt");
    let relative_artifact_path = artifact_path.strip_prefix(&workspace)?;
    let (candidate_binding_hash, runtime_receipt) =
        execute_one_runtime_mutation(&kernel, relative_artifact_path)?;
    let artifact_before = fs::read(&artifact_path)?;
    let receipt_before = kernel
        .execution_receipt_by_attempt(&runtime_receipt.attempt_id)?
        .context("runtime receipt must survive execution")?;

    let key = [23_u8; 32];
    let journal =
        OperatorMutationJournal::for_test_path(root.path().join("operator-mutations.json"));
    let mutation_id_hash = "1".repeat(64);
    let plan_hash = "2".repeat(64);
    let plan_request_binding_hash = "3".repeat(64);
    let session_binding_hash = "4".repeat(64);
    let commit_request_binding_hash = "5".repeat(64);
    let request_binding_hash = "6".repeat(64);
    journal.reserve_plan(
        &key,
        &mutation_id_hash,
        &plan_hash,
        &plan_request_binding_hash,
        &session_binding_hash,
    )?;
    journal.publish_candidate(&key, &plan_hash, &candidate_binding_hash)?;
    journal.begin_commit(
        &key,
        &mutation_id_hash,
        &plan_hash,
        &plan_request_binding_hash,
        &session_binding_hash,
        &candidate_binding_hash,
        &commit_request_binding_hash,
    )?;
    journal.record_runtime_linkage(&key, &plan_hash, &runtime_receipt)?;

    fs::set_permissions(artifact_root.path(), fs::Permissions::from_mode(0o500))?;
    let _permission_guard = RestoreDirectoryPermissions(artifact_root.path().to_path_buf());

    let effect_plan_hash = runtime_receipt
        .effect_plan_hash
        .as_deref()
        .context("runtime receipt effect plan")?;
    let inspect_proof = proof(
        &key,
        "POST",
        OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT,
        &session_binding_hash,
        &plan_hash,
        &runtime_receipt.attempt_id,
        effect_plan_hash,
        &request_binding_hash,
        None,
    )?;
    let inspect_body = serde_json::json!({
        "plan_hash": plan_hash,
        "attempt_id": runtime_receipt.attempt_id,
        "effect_plan_hash": effect_plan_hash,
        "session_binding_hash": session_binding_hash,
        "proof": inspect_proof,
    })
    .to_string();
    let inspect = route_http_with_authority(
        &kernel,
        "POST",
        OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT,
        Some(&inspect_body),
        &request_binding_hash,
        &session_binding_hash,
        &key,
        &journal,
    );
    assert_eq!(inspect.status, "200 OK");
    assert!(!inspect.journal_state_changed);
    let inspect_json: serde_json::Value = serde_json::from_str(&inspect.body)?;
    assert_eq!(inspect_json["operation"], "inspect_only");
    assert_eq!(inspect_json["inspection"]["phase"], "in_doubt");
    assert_eq!(
        inspect_json["inspection"]["session_binding_hash"],
        session_binding_hash
    );
    assert_eq!(inspect_json["provider_replayed"], false);
    assert_eq!(inspect_json["terminal_receipt_retried"], false);

    let wrong_session_binding_hash = "7".repeat(64);
    let wrong_session_proof = proof(
        &key,
        "POST",
        OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT,
        &wrong_session_binding_hash,
        &plan_hash,
        &runtime_receipt.attempt_id,
        effect_plan_hash,
        &request_binding_hash,
        None,
    )?;
    let wrong_session_body = serde_json::json!({
        "plan_hash": plan_hash,
        "attempt_id": runtime_receipt.attempt_id,
        "effect_plan_hash": effect_plan_hash,
        "session_binding_hash": wrong_session_binding_hash,
        "proof": wrong_session_proof,
    })
    .to_string();
    let wrong_session = route_http_with_authority(
        &kernel,
        "POST",
        OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT,
        Some(&wrong_session_body),
        &request_binding_hash,
        &wrong_session_binding_hash,
        &key,
        &journal,
    );
    assert_eq!(wrong_session.status, "409 Conflict");
    assert_eq!(
        wrong_session.body,
        r#"{"error":"operator_mutation_reconciliation.runtime_linkage_mismatch"}"#
    );

    let resolve_proof = proof(
        &key,
        "POST",
        OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT,
        &session_binding_hash,
        &plan_hash,
        &runtime_receipt.attempt_id,
        effect_plan_hash,
        &request_binding_hash,
        Some(ReconciliationDecision::FinalizeProductJournalOnly),
    )?;
    assert_ne!(resolve_proof, inspect_proof);
    let resolve_body = serde_json::json!({
        "plan_hash": plan_hash,
        "attempt_id": runtime_receipt.attempt_id,
        "effect_plan_hash": effect_plan_hash,
        "session_binding_hash": session_binding_hash,
        "decision": "finalize_product_journal_only",
        "proof": resolve_proof,
    })
    .to_string();
    let resolved = route_http_with_authority(
        &kernel,
        "POST",
        OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT,
        Some(&resolve_body),
        &request_binding_hash,
        &session_binding_hash,
        &key,
        &journal,
    );
    assert_eq!(resolved.status, "200 OK");
    assert!(resolved.journal_state_changed);
    let resolved_json: serde_json::Value = serde_json::from_str(&resolved.body)?;
    assert_eq!(resolved_json["operation"], "finalize_product_journal_only");
    assert_eq!(resolved_json["inspection"]["phase"], "succeeded");
    assert_eq!(resolved_json["provider_replayed"], false);
    assert_eq!(resolved_json["terminal_receipt_retried"], false);
    assert_eq!(fs::read(&artifact_path)?, artifact_before);
    assert_eq!(
        kernel
            .execution_receipt_by_attempt(&runtime_receipt.attempt_id)?
            .context("runtime receipt must remain readable")?,
        receipt_before
    );

    let repeated = route_http_with_authority(
        &kernel,
        "POST",
        OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT,
        Some(&resolve_body),
        &request_binding_hash,
        &session_binding_hash,
        &key,
        &journal,
    );
    assert_eq!(repeated.status, "409 Conflict");
    assert!(!repeated.journal_state_changed);
    assert_eq!(
        repeated.body,
        r#"{"error":"operator_mutation_reconciliation.journal_not_in_doubt"}"#
    );
    Ok(())
}

fn execute_one_runtime_mutation(
    kernel: &RuntimeKernel,
    relative_artifact_path: &Path,
) -> Result<(String, RuntimeExecutionReceipt)> {
    let session_id = "native-gateway:operator-reconciliation:test";
    let instruction = format!(
        "overwrite:{} => terminal-only reconciliation",
        relative_artifact_path.display()
    );
    kernel.switch_model_in_session(session_id, "demo/demo-chat")?;
    kernel.add_policy_rule(
        Some(session_id),
        Some("demo"),
        Some("write_file"),
        /*risk_tier*/ None,
        ApprovalRequirement::Ask,
        Some("operator mutation reconciliation test"),
    )?;
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    let planned = executor.block_on(kernel.run_demo_turn_in_session(session_id, &instruction))?;
    assert_eq!(planned.approval_required.as_deref(), Some("write_file"));
    assert!(planned.execution_receipt.is_none());
    let pending = kernel
        .approval_snapshot_for_session(session_id)?
        .pending
        .into_iter()
        .filter(|candidate| candidate.tool_name == "write_file")
        .filter_map(|candidate| candidate.candidate_binding_hash)
        .collect::<Vec<_>>();
    let [candidate_binding_hash] = pending.as_slice() else {
        anyhow::bail!("test mutation did not produce exactly one write candidate");
    };
    let executed = executor.block_on(kernel.approve_candidate_and_run_demo_turn_in_session(
        session_id,
        candidate_binding_hash,
        &instruction,
    ))?;
    let receipt = executed
        .execution_receipt
        .context("test mutation must return an execution receipt")?;
    Ok((candidate_binding_hash.clone(), receipt))
}

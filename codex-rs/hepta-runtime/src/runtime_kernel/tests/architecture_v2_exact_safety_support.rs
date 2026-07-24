fn existing_read_intent() -> String {
    let path = std::fs::canonicalize(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/decisions/ADR-0001-architecture-foundation.md"
    ))
    .expect("existing exact-read fixture");
    format!("read:{}", path.display())
}

fn changed_read_intent() -> String {
    format!(
        "read:{}",
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/decisions/ADR-0002-interface-first-contracts.md"
        )
    )
}

fn exact_pending_binding(runtime: &RuntimeKernel) -> String {
    runtime
        .approval_snapshot()
        .expect("approval snapshot")
        .pending
        .first()
        .and_then(|pending| pending.candidate_binding_hash.clone())
        .expect("exact candidate binding")
}

fn exact_policy_decision(
    runtime: &RuntimeKernel,
    requirement: ApprovalRequirement,
    reason: &str,
) -> hepta_core::PolicyDecision {
    let rule = runtime
        .add_policy_rule(
            Some("session-main"),
            None,
            Some("read_file"),
            None,
            requirement,
            Some(reason),
        )
        .expect("test policy rule");
    hepta_core::PolicyDecision {
        requirement,
        reason: rule.reason,
        matched_rule_id: Some(rule.id),
    }
}
